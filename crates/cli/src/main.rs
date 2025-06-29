use clap::{Parser, Subcommand};
use heurs_core::{LocalRunner, Runner};
use heurs_database::{
    DatabaseManager, ExecutionResultRepository, SubmissionRepository, TestCaseRepository,
};
use sea_orm;
use std::cmp::Ordering;
use std::error::Error as StdError;
use std::fs;
use std::path::PathBuf;
use tabled::{Table, Tabled};
use thiserror::Error;

#[derive(Tabled)]
struct LeaderBoardRow {
    #[tabled(rename = "Case ID")]
    case_id: u32,
    #[tabled(rename = "File Name")]
    file_name: String,
    #[tabled(rename = "Score")]
    score: i64,
    #[tabled(rename = "Time(ms)")]
    time: u32,
}

#[derive(Tabled)]
struct SubmissionRow {
    #[tabled(rename = "Submission ID")]
    submission_id: i32,
    #[tabled(rename = "Avg Score")]
    avg_score: f64,
    #[tabled(rename = "Cases")]
    cases: usize,
}

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
                .execute(&source_path, parallel, runner_test_cases, timeout)
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

            let mut rows: Vec<LeaderBoardRow> = execution_results
                .iter()
                .map(|r| {
                    let file_name = test_cases
                        .iter()
                        .find(|t| t.id == r.test_case_id as i32)
                        .map(|t| t.filename.clone())
                        .unwrap_or_else(|| "".to_string());

                    LeaderBoardRow {
                        case_id: r.test_case_id,
                        file_name,
                        score: r.score,
                        time: r.execution_time_ms,
                    }
                })
                .collect();

            // filename でソート
            rows.sort_by(|a, b| a.file_name.cmp(&b.file_name));

            println!("\n{}", Table::new(rows));

            let total: i64 = execution_results.iter().map(|r| r.score).sum();
            println!("Total score: {}", total);
        }
        Commands::LeaderBoard {
            problem_id,
            database_url,
            limit,
        } => {
            let db = DatabaseManager::connect(&database_url).await?;

            // 1. 提出を取得
            let submissions = SubmissionRepository::find_by_problem_id(&db, problem_id).await?;

            // 2. 各 submission の平均スコアを計算
            let mut rows: Vec<SubmissionRow> = Vec::new();

            for sub in &submissions {
                let results =
                    ExecutionResultRepository::find_by_submission_id(&db, sub.id as i64).await?;

                let (sum, count) = results
                    .iter()
                    .fold((0i64, 0usize), |(s, c), r| (s + r.score, c + 1));
                let avg = if count > 0 {
                    sum as f64 / count as f64
                } else {
                    0.0
                };

                rows.push(SubmissionRow {
                    submission_id: sub.id,
                    avg_score: avg,
                    cases: count,
                });
            }

            // 3. 平均スコア降順でソート
            rows.sort_by(|a, b| {
                b.avg_score
                    .partial_cmp(&a.avg_score)
                    .unwrap_or(Ordering::Equal)
            });

            rows = rows.into_iter().take(limit as usize).collect();

            println!("\n{}", Table::new(rows));
        }
    }

    Ok(())
}
