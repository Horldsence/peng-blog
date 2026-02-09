use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct AddIndexNowToPost;

impl MigrationName for AddIndexNowToPost {
    fn name(&self) -> &str {
        "m20250101_000016_add_indexnow_to_post"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for AddIndexNowToPost {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let add_submitted = "ALTER TABLE post ADD COLUMN indexnow_submitted BIGINT DEFAULT 0";
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            add_submitted.to_owned(),
        ))
        .await
        .map(|_| ())?;

        let add_submitted_at = "ALTER TABLE post ADD COLUMN indexnow_submitted_at TEXT";
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            add_submitted_at.to_owned(),
        ))
        .await
        .map(|_| ())?;

        let add_last_status = "ALTER TABLE post ADD COLUMN indexnow_last_status TEXT";
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            add_last_status.to_owned(),
        ))
        .await
        .map(|_| ())?;

        let add_last_error = "ALTER TABLE post ADD COLUMN indexnow_last_error TEXT";
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            add_last_error.to_owned(),
        ))
        .await
        .map(|_| ())?;

        let create_index = "CREATE INDEX idx_post_indexnow_submitted ON post(indexnow_submitted)";
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            create_index.to_owned(),
        ))
        .await
        .map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let drop_submitted = "ALTER TABLE post DROP COLUMN IF EXISTS indexnow_submitted";
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            drop_submitted.to_owned(),
        ))
        .await
        .map(|_| ())?;

        let drop_submitted_at = "ALTER TABLE post DROP COLUMN IF EXISTS indexnow_submitted_at";
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            drop_submitted_at.to_owned(),
        ))
        .await
        .map(|_| ())?;

        let drop_last_status = "ALTER TABLE post DROP COLUMN IF EXISTS indexnow_last_status";
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            drop_last_status.to_owned(),
        ))
        .await
        .map(|_| ())?;

        let drop_last_error = "ALTER TABLE post DROP COLUMN IF EXISTS indexnow_last_error";
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            drop_last_error.to_owned(),
        ))
        .await
        .map(|_| ())
    }
}
