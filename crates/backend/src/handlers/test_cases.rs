use crate::models::test_cases::{TestCase, TestCaseMeta, TestCaseResponse, TestCasesResponse};
use axum::{
    Json, Router,
    extract::{Path, Query},
    http::StatusCode,
    routing::get,
};
use heurs_database::{DatabaseManager, TestCaseModel, TestCaseRepository};

#[derive(Debug, serde::Deserialize)]
struct ListParams {
    #[serde(default)]
    offset: Option<u64>,
    #[serde(default)]
    limit: Option<u64>,
}

pub fn test_case_routes() -> Router {
    Router::new()
        .route("/api/test_cases", get(get_test_cases))
        .route("/api/test_cases/{id}", get(get_test_case))
}

async fn get_test_cases(Query(params): Query<ListParams>) -> (StatusCode, Json<TestCasesResponse>) {
    // DB 接続
    let db = match DatabaseManager::connect("sqlite://heurs.db").await {
        Ok(db) => db,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TestCasesResponse { test_cases: vec![] }),
            );
        }
    };

    // 取得
    let mut cases: Vec<TestCaseModel> = TestCaseRepository::find_all(&db).await.unwrap_or_default();

    // offset / limit
    if let Some(off) = params.offset {
        cases = cases.into_iter().skip(off as usize).collect();
    }
    if let Some(lim) = params.limit {
        cases.truncate(lim as usize);
    }

    (
        StatusCode::OK,
        Json(TestCasesResponse {
            test_cases: cases
                .into_iter()
                .map(|c| TestCaseMeta {
                    id: c.id,
                    filename: c.filename,
                    created_at: c.created_at,
                })
                .collect(),
        }),
    )
}

async fn get_test_case(Path(id): Path<i32>) -> (StatusCode, Json<TestCaseResponse>) {
    // DB 接続
    let db = match DatabaseManager::connect("sqlite://heurs.db").await {
        Ok(db) => db,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TestCaseResponse {
                    test_case: TestCase {
                        id: 0,
                        filename: String::new(),
                        content: String::new(),
                        created_at: chrono::Utc::now(),
                    },
                }),
            );
        }
    };

    match TestCaseRepository::find_by_id(&db, id).await {
        Ok(Some(c)) => (
            StatusCode::OK,
            Json(TestCaseResponse {
                test_case: TestCase {
                    id: c.id,
                    filename: c.filename,
                    content: c.input,
                    created_at: c.created_at,
                },
            }),
        ),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(TestCaseResponse {
                test_case: TestCase {
                    id: 0,
                    filename: String::new(),
                    content: String::new(),
                    created_at: chrono::Utc::now(),
                },
            }),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(TestCaseResponse {
                test_case: TestCase {
                    id: 0,
                    filename: String::new(),
                    content: String::new(),
                    created_at: chrono::Utc::now(),
                },
            }),
        ),
    }
}
