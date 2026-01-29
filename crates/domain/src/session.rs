use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a user session for cookie-based authentication
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Session {
    pub id: String,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl Session {
    /// Create a new session with default expiration (24 hours)
    pub fn new(user_id: Uuid) -> Self {
        Self {
            id: Self::generate_token(),
            user_id,
            expires_at: Utc::now() + Duration::hours(24),
            created_at: Utc::now(),
        }
    }

    /// Create a long-lived session (30 days)
    pub fn with_remember(user_id: Uuid) -> Self {
        Self {
            id: Self::generate_token(),
            user_id,
            expires_at: Utc::now() + Duration::days(30),
            created_at: Utc::now(),
        }
    }

    /// Check if session is expired
    pub fn is_expired(&self) -> bool {
        self.expires_at < Utc::now()
    }

    /// Generate a cryptographically random session token
    fn generate_token() -> String {
        Uuid::new_v4().to_string()
    }
}

/// Request to create a session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSession {
    pub user_id: Uuid,
    pub remember_me: bool,
}