use async_trait::async_trait;
use heurs_database::{DatabaseConnection, TestCaseModel, TestCaseRepository};

/// テストケースのトレイト
#[async_trait]
pub trait TestCaseProvider: Send + Sync {
    async fn get_test_cases(
        &self,
    ) -> Result<Vec<TestCaseModel>, Box<dyn std::error::Error + Send + Sync>>;
}

pub struct SQLiteTestCaseProvider {
    db: DatabaseConnection,
}

impl SQLiteTestCaseProvider {
    pub fn new(db: DatabaseConnection) -> Self {
        SQLiteTestCaseProvider { db }
    }
}

#[async_trait]
impl TestCaseProvider for SQLiteTestCaseProvider {
    async fn get_test_cases(
        &self,
    ) -> Result<Vec<TestCaseModel>, Box<dyn std::error::Error + Send + Sync>> {
        let test_cases = TestCaseRepository::find_all(&self.db).await?;

        Ok(test_cases)
    }
}
