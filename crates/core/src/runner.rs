use crate::compiler::{Compiler, CppCompiler};
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use threadpool::ThreadPool;

/// コマンド実行器のトレイト
pub trait Runner {
    fn execute(
        &self,
        source_path: &PathBuf,
        cases: u32,
        parallel: u32,
        timeout: u32,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

/// ローカル実行器の実装
pub struct LocalRunner;

impl LocalRunner {
    pub fn new() -> Self {
        LocalRunner
    }
}

impl Runner for LocalRunner {
    fn execute(
        &self,
        source_path: &PathBuf,
        cases: u32,
        parallel: u32,
        timeout: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let compiler: CppCompiler = CppCompiler::default();
        let binary_path = PathBuf::from("./a.out");
        compiler.compile(source_path, &binary_path)?;

        let pool = ThreadPool::new(parallel as usize);
        let (tx, rx) = mpsc::channel();

        for case_index in 0..cases {
            let tx = tx.clone();
            let binary_path = binary_path.clone();
            let case_input_path = PathBuf::from(format!("workspace/inputs/case_{}.in", case_index));
            let case_output_path =
                PathBuf::from(format!("workspace/outputs/case_{}.out", case_index));

            pool.execute(move || {
                // 入力ファイルを開いて標準入力に流し込む
                let input = match std::fs::read(&case_input_path) {
                    Ok(input) => input,
                    Err(e) => {
                        eprintln!("Error reading input file: {}", e);
                        return;
                    }
                };

                let mut child = match Command::new(&binary_path)
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                {
                    Ok(child) => child,
                    Err(e) => {
                        eprintln!("Error spawning process: {}", e);
                        return;
                    }
                };

                child.stdin.as_mut().unwrap().write_all(&input).unwrap();

                let output = child.wait_with_output().unwrap();
                match std::fs::write(&case_output_path, &output.stdout) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error writing output file: {}", e);
                        return;
                    }
                }

                let success = output.status.success();
                tx.send(success).unwrap();
            });
        }

        drop(tx); // チャネルを閉じる

        // 全ケースの結果を回収
        for success in rx.iter() {
            if !success {
                return Err("Some case failed".into());
            }
        }

        Ok(())
    }
}

impl Default for LocalRunner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_runner() {
        let runner = LocalRunner::new();
        let source_path = PathBuf::from("test.cpp");
        let result = runner.execute(&source_path, 1, 1, 30).unwrap();
        assert!(result);
    }

    #[test]
    fn test_local_runner_default() {
        let runner = LocalRunner::default();
        let source_path = PathBuf::from("test.cpp");
        let result = runner.execute(&source_path, 1, 1, 30).unwrap();
        assert!(result);
    }
}
