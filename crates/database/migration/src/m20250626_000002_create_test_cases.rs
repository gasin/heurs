use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TestCases::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TestCases::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TestCases::Input).text().not_null())
                    .col(ColumnDef::new(TestCases::Filename).string().not_null())
                    .col(ColumnDef::new(TestCases::CreatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TestCases::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TestCases {
    Table,
    Id,
    Input,
    Filename,
    CreatedAt,
}
