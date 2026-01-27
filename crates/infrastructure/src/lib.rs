mod entity;
mod post;
mod user;

pub use post::*;
pub use user::*;

use sea_orm::{Database, DatabaseConnection, DbErr};
use std::sync::Arc;

pub async fn establish_connection(database_url: &str) -> Result<Arc<DatabaseConnection>, DbErr> {
    let db = Database::connect(database_url).await?;
    Ok(Arc::new(db))
}

pub mod migrations {
    use sea_orm_migration::prelude::*;

    pub struct Migrator;

    #[async_trait::async_trait]
    impl MigratorTrait for Migrator {
        fn migrations() -> Vec<Box<dyn MigrationTrait>> {
            vec![]
        }
    }
}

