use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct AddCategoryToPost;

impl MigrationName for AddCategoryToPost {
    fn name(&self) -> &str {
        "m20250101_000015_add_category_to_post"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for AddCategoryToPost {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            ALTER TABLE post ADD COLUMN category_id TEXT;
            CREATE INDEX idx_post_category_id ON post(category_id);
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
        let sql = r#"
            CREATE TABLE post_new (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                published_at TEXT,
                created_at TEXT NOT NULL,
                views INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
            );
            INSERT INTO post_new (id, user_id, title, content, published_at, created_at, views)
            SELECT id, user_id, title, content, published_at, created_at, views FROM post;
            DROP TABLE post;
            ALTER TABLE post_new RENAME TO post;
            CREATE INDEX idx_post_user_id ON post(user_id);
            CREATE INDEX idx_post_published_at ON post(published_at);
            CREATE INDEX idx_post_created_at ON post(created_at);
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
