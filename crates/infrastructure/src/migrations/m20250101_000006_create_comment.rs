use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct CreateComment;

impl MigrationName for CreateComment {
    fn name(&self) -> &str {
        "m20250101_000006_create_comment"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for CreateComment {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE comment (
                id TEXT PRIMARY KEY,
                post_id TEXT NOT NULL,
                user_id TEXT,
                github_username TEXT,
                github_avatar_url TEXT,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE,
                FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE CASCADE
            );
            CREATE INDEX idx_comment_post_id ON comment(post_id);
            CREATE INDEX idx_comment_user_id ON comment(user_id);
            CREATE INDEX idx_comment_created_at ON comment(created_at);
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
        let sql = "DROP TABLE comment";
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
