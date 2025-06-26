use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RunRequest {
    pub source_code: String,
    pub cases: u32,
    pub parallel: u32,
    pub timeout: u32,
}

#[derive(Serialize, Deserialize)]
pub struct RunResponse {
    pub success: bool,
    pub result: String,
    pub error: Option<String>,
}
