use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct CreateFile;

impl MigrationName for CreateFile {
    fn name(&self) -> &str {
        "m20250101_000004_create_file"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for CreateFile {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let create_table = r#"
            CREATE TABLE file (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                filename TEXT NOT NULL,
                original_filename TEXT NOT NULL,
                content_type TEXT NOT NULL,
                size_bytes BIGINT NOT NULL,
                url TEXT NOT NULL,
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

        let idx_user_id = "CREATE INDEX idx_file_user_id ON file(user_id)";
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            idx_user_id.to_owned(),
        ))
        .await
        .map(|_| ())?;

        let idx_created_at = "CREATE INDEX idx_file_created_at ON file(created_at)";
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            idx_created_at.to_owned(),
        ))
        .await
        .map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE file";
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
