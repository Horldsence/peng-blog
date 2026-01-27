use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a user with full information including password hash
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub permissions: u64,
    pub created_at: DateTime<Utc>,
}

impl User {
    /// Create a new user
    pub fn new(id: Uuid, username: String, password_hash: String, permissions: u64) -> Self {
        Self {
            id,
            username,
            password_hash,
            permissions,
            created_at: Utc::now(),
        }
    }

    /// Check if user has specific permission
    pub fn has_permission(&self, permission: u64) -> bool {
        (self.permissions & permission) != 0
    }

    /// Check if user is admin (has all permissions)
    pub fn is_admin(&self) -> bool {
        self.permissions == crate::ADMIN_PERMISSIONS
    }
}

/// Public user information (without password hash)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
    pub permissions: u64,
}

impl From<&User> for UserInfo {
    fn from(user: &User) -> Self {
        UserInfo {
            id: user.id,
            username: user.username.clone(),
            permissions: user.permissions,
        }
    }
}

/// Request to register a new user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

/// Request to login
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// Response for successful login/register
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: UserInfo,
}