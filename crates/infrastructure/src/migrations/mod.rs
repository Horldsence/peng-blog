//! Database Migrations
//!
//! Modular migration files for better maintainability and compile times.
//! Each table/schema change has its own file.

mod m20250101_000000_create_user;
mod m20250101_000001_create_post;
mod m20250101_000002_create_session;
mod m20250101_000004_create_file;
mod m20250101_000006_create_comment;
mod m20250101_000008_create_visit_stats;
mod m20250101_000010_create_post_stats;
mod m20250101_000011_add_views_to_post;
mod m20250101_000012_create_category;
mod m20250101_000013_create_tag;
mod m20250101_000014_create_post_tag;
mod m20250101_000015_add_category_to_post;
mod m20250101_000016_add_indexnow_to_post;

use sea_orm_migration::prelude::*;

pub use m20250101_000000_create_user::CreateUser;
pub use m20250101_000001_create_post::CreatePost;
pub use m20250101_000002_create_session::CreateSession;
pub use m20250101_000004_create_file::CreateFile;
pub use m20250101_000006_create_comment::CreateComment;
pub use m20250101_000008_create_visit_stats::CreateVisitStats;
pub use m20250101_000010_create_post_stats::CreatePostStats;
pub use m20250101_000011_add_views_to_post::AddViewsToPost;
pub use m20250101_000012_create_category::CreateCategory;
pub use m20250101_000013_create_tag::CreateTag;
pub use m20250101_000014_create_post_tag::CreatePostTag;
pub use m20250101_000015_add_category_to_post::AddCategoryToPost;
pub use m20250101_000016_add_indexnow_to_post::AddIndexNowToPost;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(CreateUser),
            Box::new(CreatePost),
            Box::new(CreateSession),
            Box::new(CreateFile),
            Box::new(CreateComment),
            Box::new(CreateVisitStats),
            Box::new(CreatePostStats),
            Box::new(AddViewsToPost),
            Box::new(CreateCategory),
            Box::new(CreateTag),
            Box::new(CreatePostTag),
            Box::new(AddCategoryToPost),
            Box::new(AddIndexNowToPost),
        ]
    }
}
