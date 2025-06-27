use crate::entity::test_cases;
use sea_orm::*;

pub struct TestCaseRepository;

impl TestCaseRepository {
    pub async fn create(
        db: &DatabaseConnection,
        input: String,
        filename: String,
    ) -> Result<test_cases::Model, DbErr> {
        let test_case = test_cases::ActiveModel {
            input: Set(input),
            filename: Set(filename),
            created_at: Set(chrono::Utc::now()),
            ..Default::default()
        };

        test_case.insert(db).await
    }

    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: i32,
    ) -> Result<Option<test_cases::Model>, DbErr> {
        test_cases::Entity::find_by_id(id).one(db).await
    }

    pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<test_cases::Model>, DbErr> {
        test_cases::Entity::find().all(db).await
    }

    pub async fn find_by_ids(
        db: &DatabaseConnection,
        ids: Vec<i32>,
    ) -> Result<Vec<test_cases::Model>, DbErr> {
        test_cases::Entity::find()
            .filter(test_cases::Column::Id.is_in(ids))
            .all(db)
            .await
    }

    pub async fn clear(db: &DatabaseConnection) -> Result<DeleteResult, DbErr> {
        test_cases::Entity::delete_many().exec(db).await
    }
}
