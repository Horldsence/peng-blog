use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct CreateUser;

impl MigrationName for CreateUser {
    fn name(&self) -> &str {
        "m20250101_000000_create_user"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for CreateUser {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let create_table = r#"
            CREATE TABLE "user" (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL,
                permissions BIGINT NOT NULL DEFAULT 15,
                created_at TEXT NOT NULL
            )
        "#;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            create_table.to_owned(),
        ))
        .await
        .map(|_| ())?;

        let idx_username = "CREATE INDEX idx_user_username ON \"user\"(username)";
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            idx_username.to_owned(),
        ))
        .await
        .map(|_| ())?;

        let idx_created_at = "CREATE INDEX idx_user_created_at ON \"user\"(created_at)";
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            idx_created_at.to_owned(),
        ))
        .await
        .map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"DROP TABLE "user""#;
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
