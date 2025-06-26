use crate::entities::submission::Model as Submission;
use anyhow::Error;
use async_trait::async_trait;

#[async_trait]
pub trait SubmissionRepository: Send + Sync {
    async fn create(&self, submission: Submission) -> Result<Submission, Error>;
    async fn get_by_id(&self, id: i32) -> Result<Submission, Error>;
    async fn list(&self) -> Result<Vec<Submission>, Error>;
}
