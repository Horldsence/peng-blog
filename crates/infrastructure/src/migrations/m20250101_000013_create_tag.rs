use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct CreateTag;

impl MigrationName for CreateTag {
    fn name(&self) -> &str {
        "m20250101_000013_create_tag"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for CreateTag {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE tag (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                slug TEXT NOT NULL UNIQUE,
                created_at TEXT NOT NULL
            );
            CREATE INDEX idx_tag_slug ON tag(slug);
        "#;
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                sql.to_owned(),
            ))
            .await
            .map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE tag;";
        manager
            .get_connection()
            .execute(Statement::from_string(
                manager.get_database_backend(),
                sql.to_owned(),
            ))
            .await
            .map(|_| ())
    }
}
