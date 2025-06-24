use clap::{Parser, Subcommand};
use heurs_core::{LocalRunner, Runner};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        command: String,
        #[arg(trailing_var_arg = true)]
        args: Vec<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Run { command, args } => {
            let runner = LocalRunner::new();

            // Vec<String>を&[&str]に変換
            let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

            match runner.execute(command, &args_refs) {
                Ok(result) => {
                    if result.success {
                        print!("{}", result.stdout);
                        if !result.stderr.is_empty() {
                            eprint!("{}", result.stderr);
                        }
                    } else {
                        eprint!("{}", result.stderr);
                        std::process::exit(result.exit_code.unwrap_or(1));
                    }
                }
                Err(e) => {
                    eprintln!("Error executing command: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}
