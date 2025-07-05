use crate::runner::{ExecutionResult, Runner};
use async_trait::async_trait;
use aws_config::{BehaviorVersion, defaults};
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Region;
use heurs_database::TestCaseModel;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

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
        _parallel: u32,
        _test_cases: Vec<TestCaseModel>,
        _timeout: u32,
    ) -> Result<Vec<ExecutionResult>, Box<dyn std::error::Error + Send + Sync>> {
        // ---- AWS SDK 初期化 ----
        // リージョンは環境変数 AWS_REGION / AWS_DEFAULT_REGION から取得。
        // 未設定の場合は "us-east-1" をデフォルトとする。
        let config = defaults(BehaviorVersion::latest())
            .region(Region::new("ap-northeast-1"))
            .load()
            .await;

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
        use aws_sdk_s3::primitives::ByteStream;
        let body = ByteStream::from_path(source_path).await?;

        // ---- アップロード実行 ----
        client
            .put_object()
            .bucket(&bucket_name)
            .key(&key)
            .body(body)
            .send()
            .await?;

        // 今はアップロードのみなので実行結果は空ベクタを返す
        Ok(Vec::new())
    }
}
