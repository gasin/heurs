use crate::entity::submissions;
use sea_orm::*;

pub struct SubmissionRepository;

impl SubmissionRepository {
    pub async fn create(
        db: &DatabaseConnection,
        source_code: String,
    ) -> Result<submissions::Model, DbErr> {
        let submission = submissions::ActiveModel {
            source_code: Set(source_code),
            timestamp: Set(chrono::Utc::now()),
            ..Default::default()
        };

        submission.insert(db).await
    }

    crate::impl_basic_fetch!(submissions);
}
