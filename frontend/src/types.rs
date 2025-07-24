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

// 日時文字列 (RFC3339想定) を「YYYY-MM-DD HH:MM」の形に整形するヘルパ
// 例: "2024-07-24T10:39:12Z" -> "2024-07-24 10:39"
pub fn format_datetime_minute(datetime: &str) -> String {
    if let Some(t_pos) = datetime.find('T') {
        let date = &datetime[..t_pos];
        let mut time_part = &datetime[t_pos + 1..];

        // タイムゾーンや小数秒を取り除く
        if let Some(idx) = time_part.find(|c| c == 'Z' || c == '+' || c == '.') {
            time_part = &time_part[..idx];
        }

        // HH:MM:SS の前 5 文字が HH:MM
        if time_part.len() >= 5 {
            let hm = &time_part[..5];
            format!("{} {}", date, hm)
        } else {
            datetime.to_string()
        }
    } else {
        datetime.to_string()
    }
}
