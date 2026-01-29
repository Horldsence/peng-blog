//! Entity Prelude
//!
//! This module re-exports all common entity types for convenient imports.
//! Use `use crate::entity::prelude::*;` to import all entity-related types.

// Re-export all entities
pub use super::post::Entity as PostEntity;
pub use super::user::Entity as UserEntity;
pub use super::session::Entity as SessionEntity;
pub use super::file::Entity as FileEntity;
pub use super::comment::Entity as CommentEntity;
pub use super::stats::Entity as VisitStatsEntity;
pub use super::post_stats::Entity as PostStatsEntity;

// Re-export common traits and types from sea_orm
pub use sea_orm::entity::prelude::*;