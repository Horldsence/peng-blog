//! Session Repository Implementation
//!
//! This module provides the concrete implementation of SessionRepository
//! using SeaORM for database operations.
//!
//! Design Principles:
//! - Simple CRUD operations
//! - Clear error mapping
//! - No special cases

use crate::entity::session;
use crate::entity::prelude::*;
use async_trait::async_trait;
use chrono::Utc;
use domain::{Error, Result, Session};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use std::sync::Arc;

/// Concrete implementation of SessionRepository
///
/// This implementation uses SeaORM to interact with the session table
/// in the database.
pub struct SessionRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

impl Clone for SessionRepositoryImpl {
    fn clone(&self) -> Self {
        Self {
            db: Arc::clone(&self.db),
        }
    }
}

impl SessionRepositoryImpl {
    /// Create a new session repository
    ///
    /// # Arguments
    /// * `db` - Database connection
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl service::SessionRepository for SessionRepositoryImpl {
    /// Create a new session
    async fn create_session(&self, user_id: uuid::Uuid, remember_me: bool) -> Result<Session> {
        let session = if remember_me {
            Session::with_remember(user_id)
        } else {
            Session::new(user_id)
        };

        let active_model = session::ActiveModel {
            id: Set(session.id.clone()),
            user_id: Set(session.user_id.to_string()),
            expires_at: Set(session.expires_at.to_rfc3339()),
            created_at: Set(session.created_at.to_rfc3339()),
        };

        active_model
            .insert(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to create session: {}", e)))?;

        Ok(session)
    }

    /// Get a session by token
    async fn get_session(&self, token: &str) -> Result<Option<Session>> {
        let model = SessionEntity::find_by_id(token.to_string())
            .one(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to get session: {}", e)))?;

        Ok(model.map(|m| {
            Session {
                id: m.id,
                user_id: uuid::Uuid::parse_str(&m.user_id).unwrap_or_else(|_| uuid::Uuid::new_v4()),
                expires_at: m.expires_at.parse().unwrap_or_else(|_| chrono::Utc::now()),
                created_at: m.created_at.parse().unwrap_or_else(|_| chrono::Utc::now()),
            }
        }))
    }

    /// Delete a session by token
    async fn delete_session(&self, token: &str) -> Result<()> {
        SessionEntity::delete_by_id(token.to_string())
            .exec(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to delete session: {}", e)))?;

        Ok(())
    }

    /// Delete all sessions for a user
    async fn delete_user_sessions(&self, user_id: uuid::Uuid) -> Result<()> {
        SessionEntity::delete_many()
            .filter(session::Column::UserId.eq(user_id.to_string()))
            .exec(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to delete user sessions: {}", e)))?;

        Ok(())
    }

    /// Clean up expired sessions
    async fn cleanup_expired_sessions(&self) -> Result<u64> {
        let now = Utc::now().to_rfc3339();
        let result = SessionEntity::delete_many()
            .filter(session::Column::ExpiresAt.lt(now))
            .exec(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to cleanup expired sessions: {}", e)))?;

        Ok(result.rows_affected)
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    #[tokio::test]
    async fn test_session_repository() {
        // Note: This is a placeholder test
        // Real tests would use a test database or mock
        // For now, we just show the structure
        
        let user_id = uuid::Uuid::new_v4();
        
        // Create session
        let session = Session::new(user_id);
        assert_eq!(session.user_id, user_id);
        assert!(!session.is_expired());
        
        // Create session with remember me
        let long_session = Session::with_remember(user_id);
        assert_eq!(long_session.user_id, user_id);
        assert!(!long_session.is_expired());
    }
}