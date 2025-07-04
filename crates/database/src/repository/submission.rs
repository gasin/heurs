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

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<submissions::Model>, DbErr> {
        submissions::Entity::find_by_id(id).one(db).await
    }

    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<submissions::Model>, DbErr> {
        submissions::Entity::find().all(db).await
    }
}
