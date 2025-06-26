pub use sea_orm_migration::prelude::*;

mod m20250626_000001_create_submissions;
mod m20250626_000002_create_test_cases;
mod m20250626_000003_create_execution_results;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250626_000001_create_submissions::Migration),
            Box::new(m20250626_000002_create_test_cases::Migration),
            Box::new(m20250626_000003_create_execution_results::Migration),
        ]
    }
}
