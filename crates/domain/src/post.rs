use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Error {
    NotFound(String),
    Validation(String),
    Internal(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::NotFound(msg) => write!(f, "Not found: {}", msg),
            Error::Validation(msg) => write!(f, "Validation error: {}", msg),
            Error::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

// ============================================================================
// User and Permission System
// ============================================================================

// Bit mask based permissions - simple, fast, no complex RBAC
pub const POST_CREATE: u64   = 1 << 0;
pub const POST_UPDATE: u64   = 1 << 1;
pub const POST_DELETE: u64   = 1 << 2;
pub const POST_PUBLISH: u64  = 1 << 3;
pub const USER_MANAGE: u64   = 1 << 4;  // Admin only

// Default permissions for regular users
pub const DEFAULT_USER_PERMISSIONS: u64 = POST_CREATE | POST_UPDATE | POST_PUBLISH;

// Admin permissions - all of them
pub const ADMIN_PERMISSIONS: u64 = POST_CREATE | POST_UPDATE | POST_DELETE | POST_PUBLISH | USER_MANAGE;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,  // Argon2 hashed
    pub permissions: u64,       // Bit mask permissions
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(username: String, password_hash: String, permissions: u64) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            password_hash,
            permissions,
            created_at: Utc::now(),
        }
    }

    pub fn has_permission(&self, permission: u64) -> bool {
        (self.permissions & permission) != 0
    }

    pub fn is_admin(&self) -> bool {
        self.has_permission(USER_MANAGE)
    }

    pub fn can_create_post(&self) -> bool {
        self.has_permission(POST_CREATE)
    }

    pub fn can_update_post(&self) -> bool {
        self.has_permission(POST_UPDATE)
    }

    pub fn can_delete_post(&self) -> bool {
        self.has_permission(POST_DELETE)
    }

    pub fn can_publish_post(&self) -> bool {
        self.has_permission(POST_PUBLISH)
    }
}

// ============================================================================
// Auth DTOs (Data Transfer Objects)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
    pub permissions: u64,
}

impl From<&User> for UserInfo {
    fn from(user: &User) -> Self {
        Self {
            id: user.id,
            username: user.username.clone(),
            permissions: user.permissions,
        }
    }
}




#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,  // Owner of the post
    pub title: String,
    pub content: String,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl Post {
    pub fn new(user_id: Uuid, title: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            title,
            content,
            published_at: None,
            created_at: Utc::now(),
        }
    }

    pub fn is_owned_by(&self, user_id: Uuid) -> bool {
        self.user_id == user_id
    }

    pub fn publish(&mut self) {
        self.published_at = Some(Utc::now());
    }

    pub fn unpublish(&mut self) {
        self.published_at = None;
    }

    pub fn is_published(&self) -> bool {
        self.published_at.is_some()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CreatePost {
    pub title: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub content: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
}
