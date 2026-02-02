use sea_orm::Statement;
use sea_orm_migration::prelude::*;

pub struct CreateVisitStats;

impl MigrationName for CreateVisitStats {
    fn name(&self) -> &str {
        "m20250101_000008_create_visit_stats"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for CreateVisitStats {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let create_table = r#"
            CREATE TABLE visit_stats (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                total_visits BIGINT NOT NULL DEFAULT 0,
                today_visits BIGINT NOT NULL DEFAULT 0,
                last_updated TEXT NOT NULL
            )
        "#;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            create_table.to_owned(),
        ))
        .await
        .map(|_| ())?;

        let insert_data = r#"
            INSERT INTO visit_stats (id, total_visits, today_visits, last_updated)
            VALUES (1, 0, 0, '1970-01-01T00:00:00+00:00')
        "#;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            insert_data.to_owned(),
        ))
        .await
        .map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE visit_stats";
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
