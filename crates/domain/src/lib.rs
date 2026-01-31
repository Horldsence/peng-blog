//! # Domain Crate
//!
//! This crate contains all domain types, error definitions, and constants
//! used throughout the blog application.
//!
//! The domain layer defines the core business entities and rules that are
//! independent of any infrastructure or API concerns.

pub mod category;
pub mod comment;
pub mod error;
pub mod file;
pub mod post;
pub mod repository;
pub mod session;
pub mod stats;
pub mod tag;
pub mod user;

// Re-export commonly used types for convenience
pub use category::{Category, CreateCategory, UpdateCategory};
pub use comment::{
    Comment, CommentResponse, CreateComment, CreateCommentGitHub, GitHubAuthRequest, GitHubUser,
};
pub use error::{Error, Result};
pub use file::{File, FileResponse, UploadFile};
pub use post::{CreatePost, Post, UpdatePost};
pub use repository::{
    CategoryRepository, CommentRepository, FileRepository, PostRepository, SessionRepository,
    StatsRepository, TagRepository, UserRepository,
};
pub use session::{CreateSession, Session};
pub use stats::{DailyStats, PostStats, RecordViewRequest, StatsResponse, VisitStats};
pub use tag::{CreateTag, Tag};
pub use user::{LoginRequest, LoginResponse, RegisterRequest, User, UserInfo};

// ============================================================================
// Permission Constants (Bit Flags)
// ============================================================================
//
// Permissions are implemented as bit flags for efficiency and simplicity.
// Each permission is a power of 2, allowing for easy combination using bitwise OR.

/// Permission to create posts
pub const POST_CREATE: u64 = 1 << 0;

/// Permission to update posts
pub const POST_UPDATE: u64 = 1 << 1;

/// Permission to delete posts
pub const POST_DELETE: u64 = 1 << 2;

/// Permission to publish posts
pub const POST_PUBLISH: u64 = 1 << 3;

/// Permission to manage users (admin only)
pub const USER_MANAGE: u64 = 1 << 4;

/// Default permissions for regular users
/// Can create, update, and publish their own posts
pub const DEFAULT_USER_PERMISSIONS: u64 = POST_CREATE | POST_UPDATE | POST_PUBLISH;

/// Admin permissions - all permissions combined
pub const ADMIN_PERMISSIONS: u64 =
    POST_CREATE | POST_UPDATE | POST_DELETE | POST_PUBLISH | USER_MANAGE;

// ============================================================================
// Permission Checking Helpers
// ============================================================================

/// Check if user can perform action on resource owned by another user
///
/// This is the core permission check helper that eliminates repeated
/// "owner or admin" logic throughout the codebase.
///
/// # Arguments
/// * `resource_owner_id` - The ID of the user who owns the resource
/// * `requester_id` - The ID of the user making the request
/// * `requester_permissions` - The permissions bit flags of the requester
/// * `admin_permission` - The admin permission required to bypass ownership check
///
/// # Returns
/// * `Ok(())` if the requester is the owner OR has admin permission
/// * `Err(Error::Validation)` if neither condition is met
pub fn check_ownership_or_admin(
    resource_owner_id: uuid::Uuid,
    requester_id: uuid::Uuid,
    requester_permissions: u64,
    admin_permission: u64,
) -> Result<()> {
    if resource_owner_id == requester_id {
        return Ok(());
    }

    if (requester_permissions & admin_permission) != 0 {
        return Ok(());
    }

    Err(Error::Validation(
        "Permission denied: you must be the resource owner or have admin privileges".to_string(),
    ))
}

/// Check if user has a specific permission
///
/// # Arguments
/// * `user_permissions` - The user's permission bit flags
/// * `required_permission` - The required permission bit flag
///
/// # Returns
/// * `Ok(())` if the user has the required permission
/// * `Err(Error::Validation)` if the user lacks the permission
pub fn check_permission(user_permissions: u64, required_permission: u64) -> Result<()> {
    if (user_permissions & required_permission) != 0 {
        Ok(())
    } else {
        Err(Error::Validation(
            format!(
                "Permission denied: requires permission flag {:#x}",
                required_permission
            )
            .to_string(),
        ))
    }
}
