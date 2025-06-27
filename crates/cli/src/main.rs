use anyhow::Result;
use clap::{Parser, Subcommand};
use heurs_core::{LocalRunner, Runner};
use heurs_database::{DatabaseManager, ExecutionResultRepository, SubmissionRepository};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
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
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
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
            let db = DatabaseManager::connect(database_url).await?;

            // ソースコードを読み込み
            let source_code = fs::read_to_string(source_path)?;

            // submissionをデータベースに保存
            let submission =
                SubmissionRepository::create(&db, *user_id, *problem_id, source_code).await?;
            println!("Submission saved with ID: {}", submission.id);

            let runner = LocalRunner::new();

            match runner.execute(source_path, *cases, *parallel, *timeout) {
                Ok(execution_results) => {
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
                Err(e) => {
                    eprintln!("実行に失敗しました: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    Ok(())
}
