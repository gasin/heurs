use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ExecutionResults::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ExecutionResults::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ExecutionResults::SubmissionId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExecutionResults::TestCaseId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExecutionResults::Success)
                            .boolean()
                            .not_null(),
                    )
                    .col(ColumnDef::new(ExecutionResults::Stdout).text().not_null())
                    .col(ColumnDef::new(ExecutionResults::Stderr).text().not_null())
                    .col(
                        ColumnDef::new(ExecutionResults::Score)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExecutionResults::ExecutionTimeMs)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ExecutionResults::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ExecutionResults::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ExecutionResults {
    Table,
    Id,
    SubmissionId,
    TestCaseId,
    Success,
    Stdout,
    Stderr,
    Score,
    ExecutionTimeMs,
    CreatedAt,
}
