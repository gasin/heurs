use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TestCaseMeta {
    pub id: i32,
    pub filename: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct TestCasesResponse {
    pub test_cases: Vec<TestCaseMeta>,
}

#[derive(Serialize, Deserialize)]
pub struct TestCase {
    pub id: i32,
    pub filename: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct TestCaseResponse {
    pub test_case: TestCase,
}
