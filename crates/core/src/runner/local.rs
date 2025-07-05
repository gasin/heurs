use crate::runner::{ExecutionResult, Runner};
use async_trait::async_trait;
use heurs_database::TestCaseModel;
use regex::Regex;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::mpsc;
use threadpool::ThreadPool;

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
        compile_cmd: &str,
        exec_cmd: &str,
        parallel: u32,
        test_cases: Vec<TestCaseModel>,
        _timeout: u32,
    ) -> Result<Vec<ExecutionResult>, Box<dyn std::error::Error + Send + Sync>> {
        // プレースホルダ置換
        let compile_cmd = compile_cmd.replace("{{src}}", &source_path.display().to_string());

        // コンパイルを実行
        let status = Command::new("sh").arg("-c").arg(compile_cmd).status()?;
        if !status.success() {
            return Err("Compilation failed".into());
        }

        let pool = ThreadPool::new(parallel as usize);
        let (tx, rx) = mpsc::channel();

        for test_case in test_cases {
            let tx = tx.clone();
            let exec_cmd = exec_cmd.to_string();

            pool.execute(move || {
                let input = test_case.input;

                let mut child = match Command::new("sh")
                    .arg("-c")
                    .arg(&exec_cmd)
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
