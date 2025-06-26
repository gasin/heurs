use crate::models::{RunRequest, RunResponse};
use axum::{Json, Router, http::StatusCode, routing::post};
use heurs_core::{LocalRunner, Runner};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn run_routes() -> Router {
    Router::new().route("/api/run", post(run_code))
}

async fn run_code(Json(req): Json<RunRequest>) -> (StatusCode, Json<RunResponse>) {
    let tmp_path = PathBuf::from("/tmp/source.cpp");
    match File::create(&tmp_path).and_then(|mut f| f.write_all(req.source_code.as_bytes())) {
        Ok(_) => {}
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(RunResponse {
                    success: false,
                    result: String::new(),
                    error: Some(format!("ファイル作成エラー: {}", e)),
                }),
            );
        }
    }

    let runner = LocalRunner::new();
    let result = runner.execute(&tmp_path, req.cases, req.parallel, req.timeout);
    match result {
        Ok(_) => (
            StatusCode::OK,
            Json(RunResponse {
                success: true,
                result: "実行に成功しました".to_string(),
                error: None,
            }),
        ),
        Err(e) => (
            StatusCode::OK,
            Json(RunResponse {
                success: false,
                result: String::new(),
                error: Some(format!("実行エラー: {}", e)),
            }),
        ),
    }
}
