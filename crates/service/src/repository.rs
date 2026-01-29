//! # Repository Traits
//!
//! This module defines the repository interfaces that the business logic depends on.
//! These traits are implemented by the infrastructure layer to abstract away
//! database operations.
//!
//! ## Design Principles
//! - Define only the operations needed by business logic
//! - Use async traits for database operations
//! - Return domain types or domain errors
//! - No concrete database types in traits

use async_trait::async_trait;
use domain::{Post, Result, User};
use uuid::Uuid;

// ============================================================================
// Post Repository Trait
// ============================================================================

/// Repository interface for Post operations
///
/// This trait defines the contract for post data access.
/// Implementations can use any database (SQLite, PostgreSQL, etc.)
#[async_trait]
pub trait PostRepository: Send + Sync {
    /// Create a new post
    async fn create_post(&self, user_id: Uuid, title: String, content: String) -> Result<Post>;

    /// Get a post by ID
    async fn get_post(&self, id: Uuid) -> Result<Post>;

    /// Update an existing post
    async fn update_post(&self, post: Post) -> Result<Post>;

    /// List published posts with a limit
    async fn list_published_posts(&self, limit: u64) -> Result<Vec<Post>>;

    /// Delete a post by ID
    async fn delete_post(&self, id: Uuid) -> Result<()>;

    /// Get posts by user ID
    async fn get_posts_by_user(&self, user_id: Uuid, limit: u64) -> Result<Vec<Post>>;
}

// ============================================================================
// User Repository Trait
// ============================================================================

/// Repository interface for User operations
///
/// This trait defines the contract for user data access.
/// Implementations can use any database (SQLite, PostgreSQL, etc.)
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Create a new user with password hashing
    async fn create_user(&self, username: String, password: String, permissions: u64) -> Result<User>;

    /// Find a user by username
    async fn find_by_username(&self, username: &str) -> Result<Option<User>>;

    /// Find a user by ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;

    /// Verify user credentials (username and password)
    async fn verify_credentials(&self, username: &str, password: &str) -> Result<Option<User>>;

    /// Update user permissions
    async fn update_permissions(&self, user_id: Uuid, permissions: u64) -> Result<User>;

    /// List all users (admin only)
    async fn list_users(&self, limit: u64) -> Result<Vec<User>>;
}

// ============================================================================
// Session Repository Trait
// ============================================================================

/// Repository interface for Session operations
///
/// This trait defines the contract for session data access.
/// Sessions are used for cookie-based authentication.
#[async_trait]
pub trait SessionRepository: Send + Sync {
    /// Create a new session
    async fn create_session(&self, user_id: Uuid, remember_me: bool) -> Result<domain::Session>;

    /// Get a session by token
    async fn get_session(&self, token: &str) -> Result<Option<domain::Session>>;

    /// Delete a session by token
    async fn delete_session(&self, token: &str) -> Result<()>;

    /// Delete all sessions for a user
    async fn delete_user_sessions(&self, user_id: Uuid) -> Result<()>;

    /// Clean up expired sessions
    async fn cleanup_expired_sessions(&self) -> Result<u64>;
}

// ============================================================================
// File Repository Trait
// ============================================================================

/// Repository interface for File operations
///
/// This trait defines the contract for file data access.
/// Files are user uploads stored in the system.
#[async_trait]
pub trait FileRepository: Send + Sync {
    /// Create a new file record
    async fn create_file(&self, file: domain::File) -> Result<domain::File>;

    /// Get a file by ID
    async fn get_file(&self, id: Uuid) -> Result<Option<domain::File>>;

    /// List files by user ID
    async fn list_files_by_user(&self, user_id: Uuid, limit: u64) -> Result<Vec<domain::File>>;

    /// Delete a file by ID
    async fn delete_file(&self, id: Uuid, user_id: Uuid) -> Result<()>;
}

// ============================================================================
// Comment Repository Trait
// ============================================================================

/// Repository interface for Comment operations
///
/// This trait defines the contract for comment data access.
/// Comments support both registered users and GitHub users.
#[async_trait]
pub trait CommentRepository: Send + Sync {
    /// Create a new comment
    async fn create_comment(&self, comment: domain::Comment) -> Result<domain::Comment>;

    /// Get a comment by ID
    async fn get_comment(&self, id: Uuid) -> Result<Option<domain::Comment>>;

    /// List comments for a post
    async fn list_post_comments(&self, post_id: Uuid, limit: u64) -> Result<Vec<domain::Comment>>;

    /// Update a comment
    async fn update_comment(&self, comment: domain::Comment) -> Result<domain::Comment>;

    /// Delete a comment by ID
    async fn delete_comment(&self, id: Uuid, user_id: Option<Uuid>, is_github_user: bool) -> Result<()>;

    /// Get comment count for a post
    async fn get_post_comment_count(&self, post_id: Uuid) -> Result<u64>;
}

// ============================================================================
// Stats Repository Trait
// ============================================================================

/// Repository interface for Statistics operations
///
/// This trait defines the contract for statistics data access.
/// Includes global visitor stats and per-post view stats.
#[async_trait]
pub trait StatsRepository: Send + Sync {
    /// Get global visitor statistics
    async fn get_visit_stats(&self) -> Result<domain::VisitStats>;

    /// Increment visitor count
    async fn increment_visit(&self, is_today: bool) -> Result<()>;

    /// Reset today's visit count (called at midnight)
    async fn reset_today_visits(&self) -> Result<()>;

    /// Get or create post statistics
    async fn get_or_create_post_stats(&self, post_id: Uuid) -> Result<domain::PostStats>;

    /// Increment post view count
    async fn increment_post_view(&self, post_id: Uuid) -> Result<()>;

    /// Get total statistics (admin only)
    async fn get_total_stats(&self) -> Result<domain::stats::StatsResponse>;
}
