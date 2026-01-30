//! Database Migrations
//!
//! All database schema migrations in one place.
//! Applied in version order.
//!
//! Design Principles:
//! - Raw SQL for clarity and control
//! - Clear up/down paths
//! - No special cases

use sea_orm_migration::prelude::*;
use sea_orm::Statement;

/// Main migrator
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
        ]
    }
}

struct CreateUser;

impl MigrationName for CreateUser {
    fn name(&self) -> &str {
        "m20250101_000000_create_user"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for CreateUser {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE user (
                id TEXT PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                password_hash TEXT NOT NULL,
                permissions INTEGER NOT NULL DEFAULT 15,
                created_at TEXT NOT NULL
            );
            CREATE INDEX idx_user_username ON user(username);
            CREATE INDEX idx_user_created_at ON user(created_at);
        "#;
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE user";
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }
}

struct CreatePost;

impl MigrationName for CreatePost {
    fn name(&self) -> &str {
        "m20250101_0000001_create_post"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for CreatePost {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE post (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                published_at TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
            );
            CREATE INDEX idx_post_user_id ON post(user_id);
            CREATE INDEX idx_post_published_at ON post(published_at);
            CREATE INDEX idx_post_created_at ON post(created_at);
        "#;
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE post";
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }
}

struct CreateSession;

impl MigrationName for CreateSession {
    fn name(&self) -> &str {
        "m20250101_000001_create_session"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for CreateSession {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE session (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                expires_at TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
            );
            CREATE INDEX idx_session_user_id ON session(user_id);
            CREATE INDEX idx_session_expires_at ON session(expires_at);
        "#;
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE session";
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }
}

struct CreateFile;

impl MigrationName for CreateFile {
    fn name(&self) -> &str {
        "m20250101_000002_create_file"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for CreateFile {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE file (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                filename TEXT NOT NULL,
                original_filename TEXT NOT NULL,
                content_type TEXT NOT NULL,
                size_bytes INTEGER NOT NULL,
                url TEXT NOT NULL,
                created_at TEXT NOT NULL,
                FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
            );
            CREATE INDEX idx_file_user_id ON file(user_id);
            CREATE INDEX idx_file_created_at ON file(created_at);
        "#;
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE file";
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }
}

struct CreateComment;

impl MigrationName for CreateComment {
    fn name(&self) -> &str {
        "m20250101_000003_create_comment"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for CreateComment {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE comment (
                id TEXT PRIMARY KEY,
                post_id TEXT NOT NULL,
                user_id TEXT,
                github_username TEXT,
                github_avatar_url TEXT,
                content TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE,
                FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE CASCADE
            );
            CREATE INDEX idx_comment_post_id ON comment(post_id);
            CREATE INDEX idx_comment_user_id ON comment(user_id);
            CREATE INDEX idx_comment_created_at ON comment(created_at);
        "#;
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE comment";
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }
}

struct CreateVisitStats;

impl MigrationName for CreateVisitStats {
    fn name(&self) -> &str {
        "m20250101_000004_create_visit_stats"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for CreateVisitStats {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE visit_stats (
                id INTEGER PRIMARY KEY CHECK (id = 1),
                total_visits INTEGER NOT NULL DEFAULT 0,
                today_visits INTEGER NOT NULL DEFAULT 0,
                last_updated TEXT NOT NULL
            );
            INSERT INTO visit_stats (id, total_visits, today_visits, last_updated)
            VALUES (1, 0, 0, '1970-01-01T00:00:00+00:00');
        "#;
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE visit_stats";
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }
}

struct CreatePostStats;

impl MigrationName for CreatePostStats {
    fn name(&self) -> &str {
        "m20250101_000005_create_post_stats"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for CreatePostStats {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE post_stats (
                id TEXT PRIMARY KEY,
                post_id TEXT NOT NULL UNIQUE,
                views INTEGER NOT NULL DEFAULT 0,
                last_viewed_at TEXT NOT NULL,
                FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE CASCADE
            );
            CREATE INDEX idx_post_stats_post_id ON post_stats(post_id);
        "#;
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE post_stats";
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }
}

struct AddViewsToPost;

impl MigrationName for AddViewsToPost {
    fn name(&self) -> &str {
        "m20250101_000006_add_views_to_post"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for AddViewsToPost {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "ALTER TABLE post ADD COLUMN views INTEGER NOT NULL DEFAULT 0;";
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE post_new (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                published_at TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
            );
            INSERT INTO post_new (id, user_id, title, content, published_at, created_at)
            SELECT id, user_id, title, content, published_at, created_at FROM post;
            DROP TABLE post;
            ALTER TABLE post_new RENAME TO post;
            CREATE INDEX idx_post_user_id ON post(user_id);
            CREATE INDEX idx_post_published_at ON post(published_at);
        "#;
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }
}

struct CreateCategory;

impl MigrationName for CreateCategory {
    fn name(&self) -> &str {
        "m20250101_000007_create_category"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for CreateCategory {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE category (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                slug TEXT NOT NULL UNIQUE,
                parent_id TEXT,
                created_at TEXT NOT NULL,
                FOREIGN KEY (parent_id) REFERENCES category(id) ON DELETE SET NULL
            );
            CREATE INDEX idx_category_parent_id ON category(parent_id);
            CREATE INDEX idx_category_slug ON category(slug);
        "#;
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE category;";
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }
}

struct CreateTag;

impl MigrationName for CreateTag {
    fn name(&self) -> &str {
        "m20250101_000008_create_tag"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for CreateTag {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE tag (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                slug TEXT NOT NULL UNIQUE,
                created_at TEXT NOT NULL
            );
            CREATE INDEX idx_tag_slug ON tag(slug);
        "#;
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE tag;";
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }
}

struct CreatePostTag;

impl MigrationName for CreatePostTag {
    fn name(&self) -> &str {
        "m20250101_000009_create_post_tag"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for CreatePostTag {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            CREATE TABLE post_tag (
                post_id TEXT NOT NULL,
                tag_id TEXT NOT NULL,
                PRIMARY KEY (post_id, tag_id),
                FOREIGN KEY (post_id) REFERENCES post(id) ON DELETE CASCADE,
                FOREIGN KEY (tag_id) REFERENCES tag(id) ON DELETE CASCADE
            );
            CREATE INDEX idx_post_tag_tag_id ON post_tag(tag_id);
        "#;
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = "DROP TABLE post_tag;";
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }
}

struct AddCategoryToPost;

impl MigrationName for AddCategoryToPost {
    fn name(&self) -> &str {
        "m20250101_000010_add_category_to_post"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for AddCategoryToPost {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let sql = r#"
            ALTER TABLE post ADD COLUMN category_id TEXT;
            CREATE INDEX idx_post_category_id ON post(category_id);
        "#;
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // SQLite doesn't support DROP COLUMN directly, need to recreate table
        let sql = r#"
            CREATE TABLE post_new (
                id TEXT PRIMARY KEY,
                user_id TEXT NOT NULL,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                published_at TEXT,
                created_at TEXT NOT NULL,
                views INTEGER NOT NULL DEFAULT 0,
                FOREIGN KEY (user_id) REFERENCES user(id) ON DELETE CASCADE
            );
            INSERT INTO post_new (id, user_id, title, content, published_at, created_at, views)
            SELECT id, user_id, title, content, published_at, created_at, views FROM post;
            DROP TABLE post;
            ALTER TABLE post_new RENAME TO post;
            CREATE INDEX idx_post_user_id ON post(user_id);
            CREATE INDEX idx_post_published_at ON post(published_at);
            CREATE INDEX idx_post_created_at ON post(created_at);
        "#;
        manager.get_connection().execute(
            Statement::from_string(manager.get_database_backend(), sql.to_owned())
        ).await.map(|_| ())
    }
}