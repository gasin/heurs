use crate::models::{RunRequest, RunResponse};
use axum::{Json, Router, http::StatusCode, routing::post};
use heurs_core::{LocalRunner, Runner};
use heurs_database::{DatabaseManager, ExecutionResultRepository, SubmissionRepository};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn run_routes() -> Router {
    Router::new().route("/api/run", post(run_code))
}

#[axum::debug_handler]
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
        Ok(execution_results) => {
            // 実行結果をデータベースに保存
            let mut saved_count = 0;
            for result in execution_results {
                match ExecutionResultRepository::create(
                    &db,
                    submission.id as i64,
                    result.test_case_id as i64,
                    result.success,
                    result.stdout,
                    result.stderr,
                    result.score,
                    result.execution_time_ms,
                )
                .await
                {
                    Ok(_) => {
                        saved_count += 1;
                    }
                    Err(e) => {
                        eprintln!(
                            "Failed to save test case {} result: {}",
                            result.test_case_id, e
                        );
                    }
                }
            }

            (
                StatusCode::OK,
                Json(RunResponse {
                    success: true,
                    result: format!(
                        "実行に成功しました。{}個のテストケース結果を保存しました。",
                        saved_count
                    ),
                    error: None,
                    submission_id: Some(submission.id),
                }),
            )
        }
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
