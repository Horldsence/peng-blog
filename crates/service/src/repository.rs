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