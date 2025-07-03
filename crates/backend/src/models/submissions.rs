use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SubmissionMeta {
    pub id: i32,
    pub number_of_test_cases: i32,
    pub average_score: f64,
    pub average_execution_time_ms: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct SubmissionsResponse {
    pub submissions: Vec<SubmissionMeta>,
}

#[derive(Serialize, Deserialize)]
pub struct ExecutionResultMeta {
    pub test_case_id: i32,
    pub score: i64,
    pub execution_time_ms: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Submission {
    pub id: i32,
    pub problem_id: i32,
    pub source_code: String,
    pub number_of_test_cases: i32,
    pub average_score: f64,
    pub average_execution_time_ms: f64,
    pub created_at: DateTime<Utc>,
    pub execution_results: Vec<ExecutionResultMeta>,
}

#[derive(Serialize, Deserialize)]
pub struct SubmissionResponse {
    pub submission: Submission,
}
