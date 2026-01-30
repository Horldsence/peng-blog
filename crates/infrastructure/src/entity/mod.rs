//! Database entity definitions
//!
//! This module contains Sea-ORM entity definitions for the database tables.

pub mod category;
pub mod comment;
pub mod file;
pub mod post;
pub mod post_stats;
pub mod post_tag;
pub mod session;
pub mod stats;
pub mod tag;
pub mod user;

// Prelude module for convenient imports
pub mod prelude;

pub use category::Entity as CategoryEntity;
pub use comment::Entity as CommentEntity;
pub use file::Entity as FileEntity;
pub use post::Entity as PostEntity;
pub use post_stats::Entity as PostStatsEntity;
pub use post_tag::Entity as PostTagEntity;
pub use session::Entity as SessionEntity;
pub use stats::Entity as VisitStatsEntity;
pub use tag::Entity as TagEntity;
pub use user::Entity as UserEntity;
