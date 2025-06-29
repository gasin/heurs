use crate::compiler::{Compiler, CppCompiler};
use async_trait::async_trait;
use heurs_database::TestCaseModel;
use regex::Regex;
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

impl From<&heurs_database::ExecutionResultModel> for ExecutionResult {
    fn from(model: &heurs_database::ExecutionResultModel) -> Self {
        ExecutionResult {
            test_case_id: model.test_case_id as u32,
            success: model.success,
            stdout: model.stdout.clone(),
            stderr: model.stderr.clone(),
            execution_time_ms: model.execution_time_ms as u32,
            score: model.score,
        }
    }
}

/// コマンド実行器のトレイト
#[async_trait]
pub trait Runner {
    async fn execute(
        &self,
        source_path: &Path,
        parallel: u32,
        test_cases: Vec<TestCaseModel>,
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

#[async_trait]
impl Runner for LocalRunner {
    async fn execute(
        &self,
        source_path: &Path,
        parallel: u32,
        test_cases: Vec<TestCaseModel>,
        _timeout: u32,
    ) -> Result<Vec<ExecutionResult>, Box<dyn std::error::Error + Send + Sync>> {
        let compiler = CppCompiler::new();
        let binary_path = PathBuf::from("./a.out");
        compiler.compile(source_path, &binary_path)?;

        let pool = ThreadPool::new(parallel as usize);
        let (tx, rx) = mpsc::channel();

        for test_case in test_cases {
            let tx = tx.clone();
            let binary_path = binary_path.clone();

            pool.execute(move || {
                let input = test_case.input;

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
                            test_case_id: test_case.id as u32,
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

                child
                    .stdin
                    .as_mut()
                    .unwrap()
                    .write_all(input.as_bytes())
                    .unwrap();

                let output = child.wait_with_output().unwrap();

                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();

                let success = output.status.success();
                let mut score = 0;
                let mut execution_time_ms = 0;

                let re = Regex::new(r"^@@HEURS_(\w+)=(\d+)$").unwrap();
                for line in stderr.lines() {
                    if let Some(cap) = re.captures(line.trim()) {
                        match &cap[1] {
                            "SCORE" => score = cap[2].parse::<i64>().unwrap(),
                            "TIME_MS" => execution_time_ms = cap[2].parse::<u32>().unwrap(),
                            _ => {}
                        }
                    }
                }

                let result = ExecutionResult {
                    test_case_id: test_case.id as u32,
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
