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
        let db = manager.get_connection();

        let add_column = "ALTER TABLE post ADD COLUMN category_id TEXT";
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            add_column.to_owned(),
        ))
        .await
        .map(|_| ())?;

        let create_index = "CREATE INDEX idx_post_category_id ON post(category_id)";
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            create_index.to_owned(),
        ))
        .await
        .map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let create_new = r#"
            CREATE TABLE post_new (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                published_at TEXT,
                created_at TEXT NOT NULL,
                views BIGINT NOT NULL DEFAULT 0,
                FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE
            )
        "#;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            create_new.to_owned(),
        ))
        .await
        .map(|_| ())?;

        let copy_data = r#"
            INSERT INTO post_new (id, user_id, title, content, published_at, created_at, views)
            SELECT id, user_id, title, content, published_at, created_at, views FROM post
        "#;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            copy_data.to_owned(),
        ))
        .await
        .map(|_| ())?;

        let drop_old = "DROP TABLE post";
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            drop_old.to_owned(),
        ))
        .await
        .map(|_| ())?;

        let rename = "ALTER TABLE post_new RENAME TO post";
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            rename.to_owned(),
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
}
