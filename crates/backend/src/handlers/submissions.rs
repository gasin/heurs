use chrono::Utc;

use crate::models::submissions::{
    ExecutionResultMeta, Submission, SubmissionMeta, SubmissionResponse, SubmissionsResponse,
};
use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    routing::get,
};
use heurs_database::{
    DatabaseManager, ExecutionResultModel, ExecutionResultRepository, SubmissionModel,
    SubmissionRepository,
};

pub fn submission_routes() -> Router {
    Router::new()
        .route("/api/submissions", get(get_submissions))
        .route("/api/submissions/{id}", get(get_submission))
}

#[derive(Debug, serde::Deserialize)]
struct ListParams {
    #[serde(default)]
    limit: Option<u64>,
}

async fn get_submissions(
    Query(params): Query<ListParams>,
) -> (StatusCode, Json<SubmissionsResponse>) {
    // DB 接続
    let db = match DatabaseManager::connect("sqlite://heurs.db").await {
        Ok(db) => db,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SubmissionsResponse {
                    submissions: vec![],
                }),
            );
        }
    };

    // 取得
    let mut submissions: Vec<SubmissionModel> = SubmissionRepository::find_all(&db)
        .await
        .unwrap_or_default();

    // offset / limit
    if let Some(lim) = params.limit {
        submissions.truncate(lim as usize);
    }

    let mut submission_metas: Vec<SubmissionMeta> = vec![];

    for submission in submissions {
        let execution_results: Vec<ExecutionResultModel> =
            ExecutionResultRepository::find_by_submission_id(&db, submission.id as i64)
                .await
                .unwrap_or_default();

        let number_of_test_cases = execution_results.len() as i32;
        let average_score = execution_results.iter().map(|r| r.score).sum::<i64>() as f64
            / number_of_test_cases as f64;
        let average_execution_time_ms = execution_results
            .iter()
            .map(|r| r.execution_time_ms)
            .sum::<i32>() as f64
            / number_of_test_cases as f64;

        submission_metas.push(SubmissionMeta {
            id: submission.id,
            number_of_test_cases,
            average_score,
            average_execution_time_ms,
            created_at: submission.timestamp,
        });
    }

    (
        StatusCode::OK,
        Json(SubmissionsResponse {
            submissions: submission_metas,
        }),
    )
}

async fn get_submission(Path(id): Path<i32>) -> (StatusCode, Json<SubmissionResponse>) {
    // DB 接続
    let db = match DatabaseManager::connect("sqlite://heurs.db").await {
        Ok(db) => db,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(SubmissionResponse {
                    submission: Submission {
                        id: 0,
                        problem_id: 0,
                        source_code: String::new(),
                        number_of_test_cases: 0,
                        average_score: 0.0,
                        average_execution_time_ms: 0.0,
                        created_at: Utc::now(),
                        execution_results: vec![],
                    },
                }),
            );
        }
    };

    let submission = SubmissionRepository::find_by_id(&db, id)
        .await
        .unwrap_or_default();

    if submission.is_none() {
        return (
            StatusCode::NOT_FOUND,
            Json(SubmissionResponse {
                submission: Submission {
                    id: 0,
                    problem_id: 0,
                    source_code: String::new(),
                    number_of_test_cases: 0,
                    average_score: 0.0,
                    average_execution_time_ms: 0.0,
                    created_at: Utc::now(),
                    execution_results: vec![],
                },
            }),
        );
    }

    let submission = submission.unwrap();

    let execution_results: Vec<ExecutionResultModel> =
        ExecutionResultRepository::find_by_submission_id(&db, submission.id as i64)
            .await
            .unwrap_or_default();

    let result = SubmissionResponse {
        submission: Submission {
            id: submission.id,
            problem_id: submission.problem_id,
            source_code: submission.source_code,
            number_of_test_cases: execution_results.len() as i32,
            average_score: execution_results.iter().map(|r| r.score).sum::<i64>() as f64
                / execution_results.len() as f64,
            average_execution_time_ms: execution_results
                .iter()
                .map(|r| r.execution_time_ms)
                .sum::<i32>() as f64
                / execution_results.len() as f64,
            created_at: submission.timestamp,
            execution_results: execution_results
                .iter()
                .map(|r| ExecutionResultMeta {
                    test_case_id: r.test_case_id as i32,
                    score: r.score,
                    execution_time_ms: r.execution_time_ms,
                })
                .collect(),
        },
    };

    (StatusCode::OK, Json(result))
}
