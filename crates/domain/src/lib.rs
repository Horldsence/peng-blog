//! # Domain Crate
//!
//! This crate contains all domain types, error definitions, and constants
//! used throughout the blog application.
//!
//! The domain layer defines the core business entities and rules that are
//! independent of any infrastructure or API concerns.

pub mod post;
pub mod user;
pub mod error;

// Re-export commonly used types for convenience
pub use error::{Error, Result};
pub use post::{Post, CreatePost, UpdatePost};
pub use user::{User, UserInfo, RegisterRequest, LoginRequest, LoginResponse};

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
pub const ADMIN_PERMISSIONS: u64 = POST_CREATE | POST_UPDATE | POST_DELETE | POST_PUBLISH | USER_MANAGE;