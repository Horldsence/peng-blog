//! Post Entity - Database model for blog posts
//!
//! This file defines the Sea-ORM entity for the posts table.
//! The entity maps to the database schema and provides
//! type-safe database operations.

use sea_orm::entity::prelude::*;
use sea_orm::Set;

/// Post entity model
#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "post")]
pub struct Model {
    /// Primary key - stored as string (UUID as string for SQLite compatibility)
    #[sea_orm(primary_key)]
    pub id: String,
    
    /// ID of the user who owns this post
    pub user_id: String,
    
    /// Post title
    pub title: String,
    
    /// Post content (markdown)
    pub content: String,
    
    /// Optional ISO 8601 datetime string when post was published
    pub published_at: Option<String>,
    
    /// ISO 8601 datetime string when post was created
    pub created_at: String,
    
    /// Number of times this post has been viewed
    pub views: i64,
}

/// Relations for Post entity
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    /// Hook that runs before inserting a new record
    fn new() -> Self {
        Self {
            created_at: Set(chrono::Utc::now().to_rfc3339()),
            views: Set(0),
            ..ActiveModelTrait::default()
        }
    }
}