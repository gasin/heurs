use crate::components::item_list_panel::ListItem;
use serde::Deserialize;

#[derive(Clone, Deserialize, PartialEq)]
pub struct TestCaseMeta {
    pub id: i32,
    pub filename: String,
    pub created_at: String,
}

impl ListItem for TestCaseMeta {
    fn id(&self) -> i32 {
        self.id
    }
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct TestCasesResponse {
    pub test_cases: Vec<TestCaseMeta>,
}

#[derive(Clone, Deserialize, PartialEq)]
pub struct TestCase {
    pub id: i32,
    pub filename: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct TestCaseResponse {
    pub test_case: TestCase,
}

// For the submissions list: /api/submissions
#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct SubmissionMeta {
    pub id: i32,
    pub number_of_test_cases: i32,
    pub average_score: f64,
    pub average_execution_time_ms: f64,
    pub created_at: String, // Assuming DateTime<Utc> serializes to a string
}

impl ListItem for SubmissionMeta {
    fn id(&self) -> i32 {
        self.id
    }
}

// For the submission detail: /api/submissions/{id}
#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct ExecutionResultMeta {
    pub test_case_id: i32,
    pub score: i64,
    pub execution_time_ms: i32,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct SubmissionDetail {
    pub id: i32,
    pub source_code: String,
    pub number_of_test_cases: i32,
    pub average_score: f64,
    pub average_execution_time_ms: f64,
    pub created_at: String,
    pub execution_results: Vec<ExecutionResultMeta>,
}
