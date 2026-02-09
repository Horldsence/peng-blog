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

    pub category_id: Option<String>,

    /// Optional ISO 8601 datetime string when post was published
    pub published_at: Option<String>,

    /// ISO 8601 datetime string when post was created
    pub created_at: String,

    /// Number of times this post has been viewed
    pub views: i64,

    /// IndexNow submission status (0 = not submitted, 1 = submitted)
    pub indexnow_submitted: i64,

    /// ISO 8601 datetime string when IndexNow was last submitted
    pub indexnow_submitted_at: Option<String>,

    /// Last IndexNow submission status (pending, success, failed)
    pub indexnow_last_status: Option<String>,

    /// Last IndexNow submission error message
    pub indexnow_last_error: Option<String>,
}

/// Relations for Post entity
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryId",
        to = "super::category::Column::Id",
        on_update = "Cascade",
        on_delete = "SetNull"
    )]
    Category,
    #[sea_orm(has_many = "super::post_tag::Entity")]
    PostTags,
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl Related<super::post_tag::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostTags.def()
    }
}

impl Related<super::tag::Entity> for Entity {
    fn to() -> RelationDef {
        // SeaORM 1.1 doesn't support through() in the same way
        // We use find_also_related in queries instead
        super::post_tag::Entity::has_many(super::tag::Entity).into()
    }
}

impl ActiveModelBehavior for ActiveModel {
    /// Hook that runs before inserting a new record
    fn new() -> Self {
        Self {
            created_at: Set(chrono::Utc::now().to_rfc3339()),
            views: Set(0),
            indexnow_submitted: Set(0),
            indexnow_last_status: Set(Some("pending".to_string())),
            ..ActiveModelTrait::default()
        }
    }
}
