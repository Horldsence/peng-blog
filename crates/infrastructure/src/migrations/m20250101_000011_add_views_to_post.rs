use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct AddViewsToPost;

impl MigrationName for AddViewsToPost {
    fn name(&self) -> &str {
        "m20250101_000011_add_views_to_post"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for AddViewsToPost {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "ALTER TABLE post ADD COLUMN views INTEGER NOT NULL DEFAULT 0;";
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
        let sql = r#"
            CREATE TABLE post_new (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                published_at TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
            );
            INSERT INTO post_new (id, user_id, title, content, published_at, created_at)
            SELECT id, user_id, title, content, published_at, created_at FROM post;
            DROP TABLE post;
            ALTER TABLE post_new RENAME TO post;
            CREATE INDEX idx_post_user_id ON post(user_id);
            CREATE INDEX idx_post_published_at ON post(published_at);
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
}
