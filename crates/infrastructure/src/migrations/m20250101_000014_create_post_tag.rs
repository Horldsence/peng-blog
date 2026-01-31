use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct CreatePostTag;

impl MigrationName for CreatePostTag {
    fn name(&self) -> &str {
        "m20250101_000014_create_post_tag"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for CreatePostTag {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE post_tag (
                post_id TEXT NOT NULL,
                tag_id TEXT NOT NULL,
                PRIMARY KEY (post_id, tag_id),
                FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE CASCADE,
                FOREIGN KEY (tag_id) REFERENCES tag(id) ON DELETE CASCADE
            );
            CREATE INDEX idx_post_tag_tag_id ON post_tag(tag_id);
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
        let sql = "DROP TABLE post_tag;";
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
