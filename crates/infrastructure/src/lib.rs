//! Infrastructure Crate
//!
//! Concrete implementations of repository interfaces defined in service layer.
//! Handles all database interactions using SeaORM.
//!
//! Architecture:
//! - Entity models: Database table definitions
//! - Repository implementations: Concrete implementations
//! - Migrations: Database schema evolution
//!
//! Design Principles:
//! - Simple, direct database access
//! - Clear error mapping from database to domain errors
//! - No special cases

pub mod entity;

// Repository implementations
pub mod category;
pub mod comment;
pub mod file;
pub mod post;
pub mod session;
pub mod stats;
pub mod tag;
pub mod user;

// Database migrations
pub mod migrations;

// Re-exports for convenience
pub use category::*;
pub use comment::*;
pub use file::*;
pub use post::*;
pub use session::*;
pub use stats::*;
pub use tag::*;
pub use user::*;

use sea_orm::{Database, DatabaseConnection, DbErr};
use std::sync::Arc;

/// Establish database connection
///
/// # Arguments
/// * `database_url` - SQLite database URL (e.g., "sqlite://blog.db")
///
/// # Returns
/// * `Ok(Arc<DatabaseConnection>)` - Database connection
/// * `Err(DbErr)` - Connection error
pub async fn establish_connection(database_url: &str) -> Result<Arc<DatabaseConnection>, DbErr> {
    let db = Database::connect(database_url).await?;
    Ok(Arc::new(db))
}

/// Re-export migrator for use in main binary
pub use migrations::Migrator;

/// Re-export MigratorTrait for running migrations
pub use sea_orm_migration::MigratorTrait;
