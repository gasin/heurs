use crate::runner::{ExecutionResult, Runner};
use async_trait::async_trait;
use aws_config::{self, BehaviorVersion};
use aws_sdk_batch::types::JobStatus;
use aws_sdk_batch::{
    Client as BatchClient,
    types::{ContainerOverrides, KeyValuePair},
};
use aws_sdk_s3::Client;
use aws_sdk_s3::primitives::ByteStream;
use heurs_database::TestCaseModel;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::time::{Duration, sleep};

/// AWS実行器の実装
///
/// このRunnerは、コンパイルと実行をAWS上で行います。
/// AWSのサービス（例: S3, AWS Batch）と連携することを想定しています。
pub struct AWSRunner {
    // 現時点では状態を保持しない。必要になったらクライアントやバケット名を追加予定。
}

impl AWSRunner {
    /// 新しい `AWSRunner` を生成する。
    ///
    /// 現在は特別な初期化は不要なので即座に構造体を返すだけ。
    pub fn new() -> Self {
        AWSRunner {}
    }
}

#[async_trait]
impl Runner for AWSRunner {
    async fn execute(
        &self,
        source_path: &Path,
        _compile_cmd: &str,
        _exec_cmd: &str,
        parallel: u32,
        test_cases: Vec<TestCaseModel>,
        _timeout: u32,
    ) -> Result<Vec<ExecutionResult>, Box<dyn std::error::Error + Send + Sync>> {
        // ---- AWS SDK 初期化 ----
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = Client::new(&config);

        // ---- バケット名決定 ----
        let bucket_name =
            std::env::var("HEURS_S3_BUCKET").unwrap_or_else(|_| "heurs-fs".to_string());

        // ---- オブジェクトキー決定 ----
        let filename = source_path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("source")
            .to_string();
        // 秒単位の UNIX タイムスタンプを付与して衝突を回避
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let key = format!("{}-{}", ts, filename);

        // ---- ファイルを ByteStream に変換 ----
        let body = ByteStream::from_path(source_path).await?;

        // ---- アップロード実行 ----
        client
            .put_object()
            .bucket(&bucket_name)
            .key(&key)
            .body(body)
            .send()
            .await?;

        // ---- Batch ジョブ送信 ----
        let job_queue =
            std::env::var("HEURS_BATCH_QUEUE").unwrap_or_else(|_| "aws-runner-queue".to_string());
        let job_definition =
            std::env::var("HEURS_JOB_DEFINITION").unwrap_or_else(|_| "aws-runner".to_string());

        let batch_client = BatchClient::new(&config);

        let mut results: Vec<ExecutionResult> = Vec::new();

        let total = test_cases.len();

        if total == 0 {
            return Ok(results);
        }

        // ----- チャンク分割 -----
        let chunk_count = std::cmp::min(parallel as usize, total);
        let chunk_size = (total + chunk_count - 1) / chunk_count; // ceiling div

        // job_id -> (start_idx, end_idx)
        let mut pending: std::collections::HashMap<String, (usize, usize)> =
            std::collections::HashMap::new();

        for chunk_idx in 0..chunk_count {
            let start_idx = chunk_idx * chunk_size;
            if start_idx >= total {
                break;
            }
            let end_idx = std::cmp::min(start_idx + chunk_size - 1, total - 1);

            let seed_start = start_idx as u32; // enumeration indexをseedとする
            let seed_end = end_idx as u32;

            let env_vars = vec![
                KeyValuePair::builder()
                    .name("CODE_BUCKET")
                    .value(bucket_name.clone())
                    .build(),
                KeyValuePair::builder()
                    .name("CODE_KEY")
                    .value(key.clone())
                    .build(),
                KeyValuePair::builder()
                    .name("IO_BUCKET")
                    .value(bucket_name.clone())
                    .build(),
                KeyValuePair::builder()
                    .name("SEED_START")
                    .value(seed_start.to_string())
                    .build(),
                KeyValuePair::builder()
                    .name("SEED_END")
                    .value(seed_end.to_string())
                    .build(),
            ];

            let container_overrides = ContainerOverrides::builder()
                .set_environment(Some(env_vars))
                .build();

            let submit_out = batch_client
                .submit_job()
                .job_name(format!("aws-runner-chunk-{}", chunk_idx))
                .job_queue(job_queue.clone())
                .job_definition(job_definition.clone())
                .set_container_overrides(Some(container_overrides))
                .send()
                .await?;

            if let Some(job_id) = submit_out.job_id() {
                pending.insert(job_id.to_string(), (start_idx, end_idx));
            } else {
                return Err("Failed to retrieve job_id on submission".into());
            }
        }

        // ----- ジョブ完了待ち & 結果収集 -----
        while !pending.is_empty() {
            let job_ids: Vec<String> = pending.keys().cloned().collect();

            let describe_out = batch_client
                .describe_jobs()
                .set_jobs(Some(job_ids.clone()))
                .send()
                .await?;

            for job in describe_out.jobs() {
                if let Some(status_ref) = job.status() {
                    let status = status_ref.clone();
                    if status == JobStatus::Succeeded || status == JobStatus::Failed {
                        let job_id = job.job_id().unwrap_or_default();
                        let (start_idx, end_idx) = match pending.remove(job_id) {
                            Some(v) => v,
                            None => continue,
                        };

                        // 各 seed / test case を処理
                        for idx in start_idx..=end_idx {
                            let tc = &test_cases[idx];
                            let seed_num = idx as u32;
                            let output_key = format!("outputs/output_{}.txt", seed_num);
                            let error_key = format!("errors/error_{}.txt", seed_num);

                            let mut stdout_data = String::new();
                            let mut success = status == JobStatus::Succeeded;
                            let mut stderr_data = String::new();

                            if success {
                                // fetch stdout
                                match client
                                    .get_object()
                                    .bucket(&bucket_name)
                                    .key(&output_key)
                                    .send()
                                    .await
                                {
                                    Ok(obj) => {
                                        let bytes = obj.body.collect().await?;
                                        stdout_data = String::from_utf8_lossy(&bytes.into_bytes())
                                            .to_string();
                                    }
                                    Err(e) => {
                                        success = false;
                                        stderr_data =
                                            format!("Failed to fetch output from S3: {}", e);
                                    }
                                }

                                // fetch stderr file if exists
                                if let Ok(err_obj) = client
                                    .get_object()
                                    .bucket(&bucket_name)
                                    .key(&error_key)
                                    .send()
                                    .await
                                {
                                    let err_bytes = err_obj.body.collect().await?;
                                    stderr_data = String::from_utf8_lossy(&err_bytes.into_bytes())
                                        .to_string();
                                }
                            } else {
                                // even on failure try to fetch stderr file
                                if let Ok(err_obj) = client
                                    .get_object()
                                    .bucket(&bucket_name)
                                    .key(&error_key)
                                    .send()
                                    .await
                                {
                                    let err_bytes = err_obj.body.collect().await?;
                                    stderr_data = String::from_utf8_lossy(&err_bytes.into_bytes())
                                        .to_string();
                                } else {
                                    stderr_data =
                                        job.status_reason().unwrap_or("Job failed").to_string();
                                }
                            }

                            // スコアと実行時間をパース
                            let (score, execution_time_ms) =
                                crate::extract_heurs_markers!(&stderr_data);

                            results.push(ExecutionResult {
                                test_case_id: tc.id as u32,
                                success,
                                stdout: stdout_data,
                                stderr: stderr_data,
                                execution_time_ms,
                                score,
                            });
                        }
                    }
                }
            }

            // まだ未完了ジョブが残っていればスリープして次の describe へ
            if !pending.is_empty() {
                sleep(Duration::from_secs(5)).await;
            }
        }

        Ok(results)
    }
}
