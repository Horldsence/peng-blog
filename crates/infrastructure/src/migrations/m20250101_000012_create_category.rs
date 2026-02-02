use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct CreateCategory;

impl MigrationName for CreateCategory {
    fn name(&self) -> &str {
        "m20250101_000012_create_category"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for CreateCategory {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let create_table = r#"
            CREATE TABLE category (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                slug TEXT NOT NULL UNIQUE,
                parent_id TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (parent_id) REFERENCES category(id) ON DELETE SET NULL
            )
        "#;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            create_table.to_owned(),
        ))
        .await
        .map(|_| ())?;

        let idx_parent_id = "CREATE INDEX idx_category_parent_id ON category(parent_id)";
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            idx_parent_id.to_owned(),
        ))
        .await
        .map(|_| ())?;

        let idx_slug = "CREATE INDEX idx_category_slug ON category(slug)";
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            idx_slug.to_owned(),
        ))
        .await
        .map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE category";
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
