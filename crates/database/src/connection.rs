use anyhow::Result;
use sea_orm::Database;

pub struct DatabaseManager;

impl DatabaseManager {
    /// データベース接続を確立します
    pub async fn connect(database_url: &str) -> Result<DatabaseConnection> {
        let db = Database::connect(database_url).await?;
        Ok(db)
    }
}

// DatabaseConnection型を再エクスポート
pub use sea_orm::DatabaseConnection;
