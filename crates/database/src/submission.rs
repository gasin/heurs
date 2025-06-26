use crate::entities::submission::{self, ActiveModel, Model};
use crate::repository::SubmissionRepository;
use anyhow::Error;
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

pub struct SeaOrmSubmissionRepository {
    db: DatabaseConnection,
}

impl SeaOrmSubmissionRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl SubmissionRepository for SeaOrmSubmissionRepository {
    async fn create(&self, submission: Model) -> Result<Model, Error> {
        let mut active_model: ActiveModel = submission.into();
        active_model.id = Set(0); // 自動採番の場合
        let result = active_model.insert(&self.db).await?;
        Ok(result)
    }

    async fn get_by_id(&self, id: i32) -> Result<Model, Error> {
        let submission = submission::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Submission not found"))?;
        Ok(submission)
    }

    async fn list(&self) -> Result<Vec<Model>, Error> {
        let submissions = submission::Entity::find().all(&self.db).await?;
        Ok(submissions)
    }
}
