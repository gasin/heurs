use crate::models::{RunRequest, RunResponse};
use axum::{Json, Router, http::StatusCode, routing::post};
use heurs_core::{LocalRunner, Runner};
use heurs_database::{DatabaseManager, SubmissionRepository};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn run_routes() -> Router {
    Router::new().route("/api/run", post(run_code))
}

async fn run_code(Json(req): Json<RunRequest>) -> (StatusCode, Json<RunResponse>) {
    // データベース接続を確立
    let db = match DatabaseManager::connect("sqlite://heurs.db").await {
        Ok(db) => db,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(RunResponse {
                    success: false,
                    result: String::new(),
                    error: Some(format!("データベース接続エラー: {}", e)),
                    submission_id: None,
                }),
            );
        }
    };

    // submissionをデータベースに保存
    let submission = match SubmissionRepository::create(
        &db,
        req.user_id,
        req.problem_id,
        req.source_code.clone(),
    )
    .await
    {
        Ok(submission) => submission,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(RunResponse {
                    success: false,
                    result: String::new(),
                    error: Some(format!("Submission保存エラー: {}", e)),
                    submission_id: None,
                }),
            );
        }
    };

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
                    submission_id: Some(submission.id),
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
                submission_id: Some(submission.id),
            }),
        ),
        Err(e) => (
            StatusCode::OK,
            Json(RunResponse {
                success: false,
                result: String::new(),
                error: Some(format!("実行エラー: {}", e)),
                submission_id: Some(submission.id),
            }),
        ),
    }
}
