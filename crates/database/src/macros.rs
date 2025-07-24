//! SeaORM Repository 向けの共通 CRUD マクロ
//! 現状は find_by_id / find_all の 2 つだけを生成します。
#![allow(unused_macros)]

#[macro_export]
macro_rules! impl_basic_fetch {
    ($entity:ident) => {
        /// id 1 件取得
        pub async fn find_by_id(
            db: &::sea_orm::DatabaseConnection,
            id: i32,
        ) -> Result<Option<$entity::Model>, ::sea_orm::DbErr> {
            $entity::Entity::find_by_id(id).one(db).await
        }

        /// 全件取得
        pub async fn find_all(
            db: &::sea_orm::DatabaseConnection,
        ) -> Result<Vec<$entity::Model>, ::sea_orm::DbErr> {
            $entity::Entity::find().all(db).await
        }
    };
}
