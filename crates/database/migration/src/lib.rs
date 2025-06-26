pub use sea_orm_migration::prelude::*;

mod m20250626_000001_create_submission;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20250626_000001_create_submission::Migration)]
    }
}
