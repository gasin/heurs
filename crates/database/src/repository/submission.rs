use crate::entity::submissions;
use sea_orm::*;

pub struct SubmissionRepository;

impl SubmissionRepository {
    pub async fn create(
        db: &DatabaseConnection,
        user_id: i32,
        problem_id: i32,
        source_code: String,
    ) -> Result<submissions::Model, DbErr> {
        let submission = submissions::ActiveModel {
            user_id: Set(user_id),
            problem_id: Set(problem_id),
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

    pub async fn find_by_user_id(
        db: &DatabaseConnection,
        user_id: i32,
    ) -> Result<Vec<submissions::Model>, DbErr> {
        submissions::Entity::find()
            .filter(submissions::Column::UserId.eq(user_id))
            .all(db)
            .await
    }

    pub async fn find_by_problem_id(
        db: &DatabaseConnection,
        problem_id: i32,
    ) -> Result<Vec<submissions::Model>, DbErr> {
        submissions::Entity::find()
            .filter(submissions::Column::ProblemId.eq(problem_id))
            .all(db)
            .await
    }
}
