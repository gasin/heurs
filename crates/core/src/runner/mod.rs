pub mod aws;
pub mod local;

use async_trait::async_trait;
use heurs_database::TestCaseModel;
use std::path::Path;

pub use aws::AWSRunner;
pub use local::LocalRunner;

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
        compile_cmd: &str,
        exec_cmd: &str,
        parallel: u32,
        test_cases: Vec<TestCaseModel>,
        timeout: u32,
    ) -> Result<Vec<ExecutionResult>, Box<dyn std::error::Error + Send + Sync>>;
}
