use crate::entity::execution_results;
use sea_orm::*;

pub struct ExecutionResultRepository;

impl ExecutionResultRepository {
    pub async fn create(
        db: &DatabaseConnection,
        submission_id: i64,
        test_case_id: i64,
        success: bool,
        stdout: String,
        stderr: String,
        score: i64,
        execution_time_ms: u32,
    ) -> Result<execution_results::Model, DbErr> {
        let result = execution_results::ActiveModel {
            submission_id: Set(submission_id),
            test_case_id: Set(test_case_id),
            success: Set(success),
            stdout: Set(stdout),
            stderr: Set(stderr),
            score: Set(score),
            execution_time_ms: Set(execution_time_ms as i32),
            created_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        result.insert(db).await
    }

    pub async fn find_by_submission_id(
        db: &DatabaseConnection,
        submission_id: i64,
    ) -> Result<Vec<execution_results::Model>, DbErr> {
        execution_results::Entity::find()
            .filter(execution_results::Column::SubmissionId.eq(submission_id))
            .all(db)
            .await
    }

    pub async fn find_by_test_case_id(
        db: &DatabaseConnection,
        test_case_id: i64,
    ) -> Result<Vec<execution_results::Model>, DbErr> {
        execution_results::Entity::find()
            .filter(execution_results::Column::TestCaseId.eq(test_case_id))
            .all(db)
            .await
    }

    pub async fn find_by_submission_and_test_case(
        db: &DatabaseConnection,
        submission_id: i64,
        test_case_id: i64,
    ) -> Result<Option<execution_results::Model>, DbErr> {
        execution_results::Entity::find()
            .filter(execution_results::Column::SubmissionId.eq(submission_id))
            .filter(execution_results::Column::TestCaseId.eq(test_case_id))
            .one(db)
            .await
    }

    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<execution_results::Model>, DbErr> {
        execution_results::Entity::find().all(db).await
    }
}
