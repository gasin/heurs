use clap::{Parser, Subcommand};
use heurs_core::{LocalRunner, Runner};
use heurs_database::{
    DatabaseManager, ExecutionResultRepository, SubmissionRepository, TestCaseRepository,
};
use sea_orm;
use std::error::Error as StdError;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

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

        // ユーザーID
        #[arg(short, long, default_value = "0")]
        user_id: i32,

        // 問題ID
        #[arg(short, long, default_value = "0")]
        problem_id: i32,

        // データベースURL
        #[arg(short, long, default_value = "sqlite://heurs.db")]
        database_url: String,
    },
    TestCase(TestCaseArgs),
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
            user_id,
            problem_id,
            database_url,
        } => {
            // データベース接続を確立
            let db = DatabaseManager::connect(&database_url).await?;

            // ソースコードを読み込み
            let source_code = fs::read_to_string(&source_path)?;

            // submissionをデータベースに保存
            let submission =
                SubmissionRepository::create(&db, user_id, problem_id, source_code.clone()).await?;
            println!("Submission saved with ID: {}", submission.id);

            let runner = LocalRunner::new();

            let execution_results = runner
                .execute(&source_path, cases, parallel, timeout)
                .map_err(CliError::Execution)?;

            println!("実行に成功しました");

            // 実行結果をデータベースに保存
            for result in execution_results {
                dbg!(&result);
                match ExecutionResultRepository::create(
                    &db,
                    submission.id as i64,
                    result.test_case_id as i64,
                    result.success,
                    result.stdout,
                    result.stderr,
                    result.score,
                    result.execution_time_ms,
                )
                .await
                {
                    Ok(_) => {
                        println!("Test case {} result saved", result.test_case_id);
                    }
                    Err(e) => {
                        eprintln!(
                            "Failed to save test case {} result: {}",
                            result.test_case_id, e
                        );
                    }
                }
            }
        }
    }

    Ok(())
}
