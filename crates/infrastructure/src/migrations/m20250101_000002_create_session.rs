use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct CreateSession;

impl MigrationName for CreateSession {
    fn name(&self) -> &str {
        "m20250101_000002_create_session"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for CreateSession {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE session (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                expires_at TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
            );
            CREATE INDEX idx_session_user_id ON session(user_id);
            CREATE INDEX idx_session_expires_at ON session(expires_at);
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
        let sql = "DROP TABLE session";
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
