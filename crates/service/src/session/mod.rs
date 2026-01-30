//! Session Service
//!
//! This module provides business logic for session management.
//! Sessions are used for cookie-based authentication.
//!
//! Design Principles:
//! - Simple CRUD operations on sessions
//! - Automatic expiration handling
//! - No special cases - all sessions follow the same rules

use crate::repository::SessionRepository;
use domain::{Result, Session};
use std::sync::Arc;

/// Session service for managing user sessions
///
/// This service handles all session-related business logic including:
/// - Creating new sessions (with optional "remember me" functionality)
/// - Validating existing sessions
/// - Destroying sessions
/// - Cleaning up expired sessions
///
/// All operations are database-backed through the SessionRepository trait.
#[derive(Clone)]
pub struct SessionService {
    session_repo: Arc<dyn SessionRepository>,
}

impl SessionService {
    /// Create a new session service
    pub fn new(session_repo: Arc<dyn SessionRepository>) -> Self {
        Self { session_repo }
    }

    /// Create a new session for a user
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user
    /// * `remember_me` - If true, session expires in 30 days; otherwise 24 hours
    ///
    /// # Returns
    /// * `Ok(Session)` - The created session
    /// * `Err(Error)` - Database error
    pub async fn create_session(&self, user_id: uuid::Uuid, remember_me: bool) -> Result<Session> {
        let _session = if remember_me {
            Session::with_remember(user_id)
        } else {
            Session::new(user_id)
        };
        self.session_repo.create_session(user_id, remember_me).await
    }

    /// Validate a session token
    ///
    /// # Arguments
    /// * `token` - The session token to validate
    ///
    /// # Returns
    /// * `Ok(Some(Session))` - Valid session
    /// * `Ok(None)` - Session not found or expired
    /// * `Err(Error)` - Database error
    pub async fn validate_session(&self, token: &str) -> Result<Option<Session>> {
        let session = self.session_repo.get_session(token).await?;

        if let Some(session) = session {
            if session.is_expired() {
                // Delete expired session on access
                let _ = self.session_repo.delete_session(token).await;
                return Ok(None);
            }
            return Ok(Some(session));
        }

        Ok(None)
    }

    /// Destroy a session
    ///
    /// # Arguments
    /// * `token` - The session token to destroy
    ///
    /// # Returns
    /// * `Ok(())` - Session destroyed
    /// * `Err(Error)` - Database error
    pub async fn destroy_session(&self, token: &str) -> Result<()> {
        self.session_repo.delete_session(token).await
    }

    /// Destroy all sessions for a user
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user
    ///
    /// # Returns
    /// * `Ok(())` - All sessions destroyed
    /// * `Err(Error)` - Database error
    pub async fn destroy_user_sessions(&self, user_id: uuid::Uuid) -> Result<()> {
        self.session_repo.delete_user_sessions(user_id).await
    }

    /// Clean up all expired sessions
    ///
    /// This should be called periodically (e.g., daily) to remove expired sessions
    /// from the database.
    ///
    /// # Returns
    /// * `Ok(count)` - Number of sessions cleaned up
    /// * `Err(Error)` - Database error
    pub async fn cleanup_expired(&self) -> Result<u64> {
        self.session_repo.cleanup_expired_sessions().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::SessionRepository;
    use async_trait::async_trait;
    use domain::{Result, Session};
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    // Mock repository for testing
    struct MockSessionRepo {
        sessions: Arc<RwLock<HashMap<String, Session>>>,
    }

    #[async_trait]
    impl SessionRepository for MockSessionRepo {
        async fn create_session(&self, user_id: uuid::Uuid, remember_me: bool) -> Result<Session> {
            let session = if remember_me {
                Session::with_remember(user_id)
            } else {
                Session::new(user_id)
            };
            let mut sessions = self.sessions.write().await;
            sessions.insert(session.id.clone(), session.clone());
            Ok(session)
        }

        async fn get_session(&self, token: &str) -> Result<Option<Session>> {
            let sessions = self.sessions.read().await;
            Ok(sessions.get(token).cloned())
        }

        async fn delete_session(&self, token: &str) -> Result<()> {
            let mut sessions = self.sessions.write().await;
            sessions.remove(token);
            Ok(())
        }

        async fn delete_user_sessions(&self, user_id: uuid::Uuid) -> Result<()> {
            let mut sessions = self.sessions.write().await;
            sessions.retain(|_, session| session.user_id != user_id);
            Ok(())
        }

        async fn cleanup_expired_sessions(&self) -> Result<u64> {
            let mut sessions = self.sessions.write().await;
            let before = sessions.len();
            sessions.retain(|_, session| !session.is_expired());
            Ok((before - sessions.len()) as u64)
        }
    }

    #[tokio::test]
    async fn test_create_session() {
        let repo = Arc::new(MockSessionRepo {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        });
        let service = SessionService::new(repo);

        let user_id = uuid::Uuid::new_v4();
        let session = service.create_session(user_id, false).await.unwrap();

        assert_eq!(session.user_id, user_id);
        assert!(!session.id.is_empty());
        assert!(!session.is_expired());
    }

    #[tokio::test]
    async fn test_validate_session() {
        let repo = Arc::new(MockSessionRepo {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        });
        let service = SessionService::new(repo);

        let user_id = uuid::Uuid::new_v4();
        let session = service.create_session(user_id, false).await.unwrap();

        // Valid session
        let validated = service.validate_session(&session.id).await.unwrap();
        assert!(validated.is_some());
        assert_eq!(validated.unwrap().user_id, user_id);

        // Invalid session
        let invalid = service.validate_session("invalid").await.unwrap();
        assert!(invalid.is_none());
    }

    #[tokio::test]
    async fn test_destroy_session() {
        let repo = Arc::new(MockSessionRepo {
            sessions: Arc::new(RwLock::new(HashMap::new())),
        });
        let service = SessionService::new(repo);

        let user_id = uuid::Uuid::new_v4();
        let session = service.create_session(user_id, false).await.unwrap();

        service.destroy_session(&session.id).await.unwrap();

        let validated = service.validate_session(&session.id).await.unwrap();
        assert!(validated.is_none());
    }
}
