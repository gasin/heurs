use serde::Deserialize;

#[derive(Clone, Deserialize, PartialEq)]
pub struct TestCaseMeta {
    pub id: i32,
    pub filename: String,
    pub created_at: String,
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
