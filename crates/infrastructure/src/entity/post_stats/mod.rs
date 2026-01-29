//! Post Stats Entity - Database model for post statistics
//!
//! This file defines the Sea-ORM entity for the post_stats table.
//! It stores view counts and last viewed timestamps for each post.
//! This separates statistics from the main post entity for better performance
//! and easier aggregation.

use sea_orm::entity::prelude::*;

/// Post statistics entity model
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "post_stats")]
pub struct Model {
    /// Primary key - stored as string (UUID as string for SQLite compatibility)
    #[sea_orm(primary_key)]
    pub id: String,

    /// ID of the post these stats belong to
    pub post_id: String,

    /// Total number of views for this post
    pub views: i64,

    /// ISO 8601 datetime string of last view
    pub last_viewed_at: String,
}

/// Relations for PostStats entity
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::post::Entity",
        from = "Column::PostId",
        to = "super::post::Column::Id"
    )]
    Post,
}

impl Related<super::post::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
