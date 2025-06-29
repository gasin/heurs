use clap::{Parser, Subcommand};
use heurs_core::ExecutionResult;
use heurs_core::{LocalRunner, Runner};
use heurs_database::{
    DatabaseManager, ExecutionResultRepository, SubmissionRepository, TestCaseRepository,
};
use sea_orm;
use std::error::Error as StdError;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

mod view;

#[derive(Debug, Error)]
enum CliError {
    #[error("Database error: {0}")]
    Db(#[from] sea_orm::DbErr),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to read directory entries")]
    ReadDir,
    #[error("Execution error: {0}")]
    Execution(Box<dyn StdError + Send + Sync>),
}

#[derive(Parser)]
#[command(
    name = "heurs",
    version = "1.0",
    about = "Heuristics contest helper tool"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        // ソースコードのパス
        source_path: PathBuf,

        // テストケースの数
        #[arg(short, long, default_value = "10")]
        cases: u32,

        // 並列実行数
        #[arg(short, long, default_value = "1")]
        parallel: u32,

        // タイムアウト時間(s)
        #[arg(short, long, default_value = "10")]
        timeout: u32,

        // 設定ファイルパス
        #[arg(long, default_value = "heurs.toml")]
        config: PathBuf,

        // 問題ID
        #[arg(short, long, default_value = "0")]
        problem_id: i32,

        // データベースURL
        #[arg(short, long, default_value = "sqlite://heurs.db")]
        database_url: String,
    },
    TestCase(TestCaseArgs),
    LeaderBoard {
        #[arg(short, long, default_value = "0")]
        problem_id: i32,

        // データベースURL
        #[arg(short, long, default_value = "sqlite://heurs.db")]
        database_url: String,

        // 何件表示するか
        #[arg(short, long, default_value = "10")]
        limit: u32,
    },
    Submission(SubmissionArgs),
}

#[derive(Parser, Debug)]
struct SubmissionArgs {
    #[command(subcommand)]
    command: SubmissionCommands,
}

#[derive(Subcommand, Debug)]
enum SubmissionCommands {
    Describe {
        // データベースURL
        #[arg(short, long, default_value = "sqlite://heurs.db")]
        database_url: String,

        #[arg(short, long)]
        submission_id: i32,
    },
}

#[derive(Parser, Debug)]
struct TestCaseArgs {
    #[command(subcommand)]
    command: TestCaseCommands,
}

#[derive(Subcommand, Debug)]
enum TestCaseCommands {
    Add(AddArgs),
    Clear {},
}

#[derive(Parser, Debug)]
struct AddArgs {
    #[arg(long, default_value_t = 0)]
    problem_id: i64,
    #[arg(long)]
    input_path: PathBuf,
}

#[tokio::main]
async fn main() -> std::result::Result<(), CliError> {
    let cli = Cli::parse();

    match cli.command {
        Commands::TestCase(args) => match args.command {
            TestCaseCommands::Add(add_args) => {
                println!(
                    "Adding test cases for problem_id {} from path: {}",
                    add_args.problem_id,
                    add_args.input_path.display()
                );

                let db = DatabaseManager::connect("sqlite://heurs.db").await?;
                let entries =
                    std::fs::read_dir(&add_args.input_path).map_err(|_| CliError::ReadDir)?;

                let mut count = 0;
                for entry in entries {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                            if ext == "txt" || ext == "in" {
                                let input_data = std::fs::read_to_string(&path)?;
                                let filename = path
                                    .file_name()
                                    .and_then(|s| s.to_str())
                                    .unwrap_or_default()
                                    .to_string();
                                TestCaseRepository::create(&db, input_data, filename).await?;
                                println!("Registered test case: {}", path.display());
                                count += 1;
                            }
                        }
                    }
                }
                println!("\nSuccessfully added {} test cases.", count);
            }
            TestCaseCommands::Clear {} => {
                println!("Clearing all test cases...");
                let db = DatabaseManager::connect("sqlite://heurs.db").await?;
                let result = TestCaseRepository::clear(&db).await?;
                println!("Successfully deleted {} test cases.", result.rows_affected);
            }
        },
        Commands::Run {
            source_path,
            cases,
            parallel,
            timeout,
            config,
            problem_id,
            database_url,
        } => {
            // データベース接続を確立
            let db = DatabaseManager::connect(&database_url).await?;

            // ソースコードを読み込み
            let source_code = fs::read_to_string(&source_path)?;

            // submissionをデータベースに保存
            let submission =
                SubmissionRepository::create(&db, problem_id, source_code.clone()).await?;
            println!("Submission saved with ID: {}", submission.id);

            let test_cases = TestCaseRepository::find_limit(&db, cases as u64).await?;

            // Runner 用にクローンを渡し、元の test_cases は後続の表示に再利用する
            let runner_test_cases = test_cases.clone();

            let runner = LocalRunner::new();
            let execution_results = runner
                .execute(&source_path, &config, parallel, runner_test_cases, timeout)
                .await
                .map_err(CliError::Execution)?;

            println!("実行に成功しました");

            // 実行結果をデータベースに保存
            for result in &execution_results {
                match ExecutionResultRepository::create(
                    &db,
                    submission.id as i64,
                    result.test_case_id as i64,
                    result.success,
                    result.stdout.clone(),
                    result.stderr.clone(),
                    result.score,
                    result.execution_time_ms,
                )
                .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!(
                            "Failed to save test case {} result: {}",
                            result.test_case_id, e
                        );
                    }
                }
            }

            view::render_execution_results(&execution_results, &test_cases);

            view::render_submission_summary(&submission, &execution_results);
        }
        Commands::LeaderBoard {
            problem_id,
            database_url,
            limit,
        } => {
            let db = DatabaseManager::connect(&database_url).await?;

            let submissions = SubmissionRepository::find_by_problem_id(&db, problem_id).await?;
            let execution_results = ExecutionResultRepository::find_all(&db).await?;

            view::render_leaderboard(&submissions, &execution_results, limit);
        }
        Commands::Submission(args) => match args.command {
            SubmissionCommands::Describe {
                database_url,
                submission_id,
            } => {
                let db = DatabaseManager::connect(&database_url).await?;

                let execution_results =
                    ExecutionResultRepository::find_by_submission_id(&db, submission_id as i64)
                        .await?;
                let execution_results = execution_results
                    .iter()
                    .map(|r| r.into())
                    .collect::<Vec<ExecutionResult>>();

                let submission =
                    SubmissionRepository::find_by_id(&db, submission_id as i32).await?;
                let test_cases = TestCaseRepository::find_all(&db).await?;

                view::render_execution_results(&execution_results, &test_cases);
                view::render_submission_summary(&submission.unwrap(), &execution_results);
            }
        },
    }

    Ok(())
}
