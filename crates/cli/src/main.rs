use clap::{Parser, Subcommand};
use heurs_core::{LocalRunner, Runner};
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
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run {
            source_path,
            cases,
            parallel,
            timeout,
        } => {
            let runner = LocalRunner::new();

            match runner.execute(source_path, *cases, *parallel, *timeout) {
                Ok(_) => {
                    println!("実行に成功しました");
                }
                Err(e) => {
                    eprintln!("実行に失敗しました: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
