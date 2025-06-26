use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Submissions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Submissions::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Submissions::UserId).integer().not_null())
                    .col(ColumnDef::new(Submissions::ProblemId).integer().not_null())
                    .col(ColumnDef::new(Submissions::SourceCode).text().not_null())
                    .col(
                        ColumnDef::new(Submissions::Timestamp)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Submissions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Submissions {
    Table,
    Id,
    UserId,
    ProblemId,
    SourceCode,
    Timestamp,
}
