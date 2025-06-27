use sea_orm::{Database, DbErr};

pub struct DatabaseManager;

impl DatabaseManager {
    /// データベース接続を確立します
    pub async fn connect(database_url: &str) -> Result<DatabaseConnection, DbErr> {
        let db = Database::connect(database_url).await?;
        Ok(db)
    }
}

// DatabaseConnection型を再エクスポート
pub use sea_orm::DatabaseConnection;
