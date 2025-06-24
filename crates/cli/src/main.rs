use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        pattern: String,
        path: std::path::PathBuf,
    },
}

fn main() {
    let args = Cli::parse();

    match &args.command {
        Commands::Run { pattern, path } => {
            let content =
                std::fs::read_to_string(path).expect("Should have been able to read the file");
            for line in content.lines() {
                if line.contains(pattern) {
                    println!("{}", line);
                }
            }
        }
    }
}
