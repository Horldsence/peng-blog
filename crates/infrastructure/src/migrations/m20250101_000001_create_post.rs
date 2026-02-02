use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct CreatePost;

impl MigrationName for CreatePost {
    fn name(&self) -> &str {
        "m20250101_000001_create_post"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for CreatePost {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let create_table = r#"
            CREATE TABLE post (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                published_at TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE
            )
        "#;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            create_table.to_owned(),
        ))
        .await
        .map(|_| ())?;

        let idx_user_id = "CREATE INDEX idx_post_user_id ON post(user_id)";
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            idx_user_id.to_owned(),
        ))
        .await
        .map(|_| ())?;

        let idx_published_at = "CREATE INDEX idx_post_published_at ON post(published_at)";
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            idx_published_at.to_owned(),
        ))
        .await
        .map(|_| ())?;

        let idx_created_at = "CREATE INDEX idx_post_created_at ON post(created_at)";
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            idx_created_at.to_owned(),
        ))
        .await
        .map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE post";
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
