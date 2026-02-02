use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct CreatePostStats;

impl MigrationName for CreatePostStats {
    fn name(&self) -> &str {
        "m20250101_000010_create_post_stats"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for CreatePostStats {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let create_table = r#"
            CREATE TABLE post_stats (
                id TEXT PRIMARY KEY,
                post_id TEXT NOT NULL UNIQUE,
                views BIGINT NOT NULL DEFAULT 0,
                last_viewed_at TEXT NOT NULL,
                FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE CASCADE
            )
        "#;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            create_table.to_owned(),
        ))
        .await
        .map(|_| ())?;

        let idx_post_id = "CREATE INDEX idx_post_stats_post_id ON post_stats(post_id)";
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            idx_post_id.to_owned(),
        ))
        .await
        .map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE post_stats";
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
