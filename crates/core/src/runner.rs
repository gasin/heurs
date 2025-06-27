use crate::compiler::{Compiler, CppCompiler};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc;
use threadpool::ThreadPool;

/// 実行結果を表す構造体
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub test_case_id: u32,
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub execution_time_ms: u32,
    pub score: i64,
}

/// コマンド実行器のトレイト
pub trait Runner {
    fn execute(
        &self,
        source_path: &Path,
        cases: u32,
        parallel: u32,
        timeout: u32,
    ) -> Result<Vec<ExecutionResult>, Box<dyn std::error::Error + Send + Sync>>;
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
        source_path: &Path,
        cases: u32,
        parallel: u32,
        _timeout: u32,
    ) -> Result<Vec<ExecutionResult>, Box<dyn std::error::Error + Send + Sync>> {
        let compiler = CppCompiler::new();
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
                let start_time = std::time::Instant::now();

                // 入力ファイルを開いて標準入力に流し込む
                let input = match std::fs::read(&case_input_path) {
                    Ok(input) => input,
                    Err(e) => {
                        eprintln!("Error reading input file: {}", e);
                        let result = ExecutionResult {
                            test_case_id: case_index,
                            success: false,
                            stdout: String::new(),
                            stderr: format!("Error reading input file: {}", e),
                            execution_time_ms: 0,
                            score: 0,
                        };
                        tx.send(result).unwrap();
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
                        let result = ExecutionResult {
                            test_case_id: case_index,
                            success: false,
                            stdout: String::new(),
                            stderr: format!("Error spawning process: {}", e),
                            execution_time_ms: 0,
                            score: 0,
                        };
                        tx.send(result).unwrap();
                        return;
                    }
                };

                child.stdin.as_mut().unwrap().write_all(&input).unwrap();

                let output = child.wait_with_output().unwrap();
                let execution_time = start_time.elapsed();
                let execution_time_ms = execution_time.as_millis() as u32;

                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();

                match std::fs::write(&case_output_path, &output.stdout) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error writing output file: {}", e);
                    }
                }

                let success = output.status.success();
                let score = if success { 100 } else { 0 };

                let result = ExecutionResult {
                    test_case_id: case_index,
                    success,
                    stdout,
                    stderr,
                    execution_time_ms,
                    score,
                };

                tx.send(result).unwrap();
            });
        }

        drop(tx); // チャネルを閉じる

        // 全ケースの結果を回収
        let mut results = Vec::new();
        for result in rx.iter() {
            results.push(result);
        }

        Ok(results)
    }
}

impl Default for LocalRunner {
    fn default() -> Self {
        Self::new()
    }
}
