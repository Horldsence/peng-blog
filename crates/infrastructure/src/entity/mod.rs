//! Database entity definitions
//!
//! This module contains Sea-ORM entity definitions for the database tables.

pub mod post;
pub mod user;
pub mod session;
pub mod file;
pub mod comment;
pub mod stats;
pub mod post_stats;

// Prelude module for convenient imports
pub mod prelude;

pub use post::Entity as PostEntity;
pub use user::Entity as UserEntity;
pub use session::Entity as SessionEntity;
pub use file::Entity as FileEntity;
pub use comment::Entity as CommentEntity;
pub use stats::Entity as VisitStatsEntity;
pub use post_stats::Entity as PostStatsEntity;