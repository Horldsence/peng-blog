//! Application State
//!
//! This module defines the shared application state that is passed to all route handlers.
//! The state is generic over repository implementations, allowing for easy testing
//! and swapping of different database backends.

use blog_service::{PostService, UserService, PostRepository, UserRepository};
use std::sync::Arc;

use crate::middleware::auth::AuthState;

/// Shared application state
///
/// This struct holds all the services and shared data needed by route handlers.
/// It is generic over repository types to support different implementations
/// (e.g., real database vs. mock for testing).
///
/// # Type Parameters
/// * `PR` - Post repository implementation
/// * `UR` - User repository implementation
#[derive(Clone)]
pub struct AppState<PR, UR>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
{
    /// Post service with business logic for post operations
    pub post_service: Arc<PostService<PR>>,
    
    /// User service with business logic for user operations
    pub user_service: Arc<UserService<UR>>,
    
    /// Authentication state for JWT token operations
    pub auth_state: AuthState,
}

impl<PR, UR> AppState<PR, UR>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
{
    /// Create a new application state
    ///
    /// # Arguments
    /// * `post_service` - Post service instance
    /// * `user_service` - User service instance
    /// * `auth_state` - Authentication state for JWT operations
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use api::{AppState, AuthState};
    /// use blog_service::{PostService, UserService};
    /// use infrastructure::{PostRepositoryImpl, UserRepositoryImpl};
    /// use std::sync::Arc;
    ///
    /// let post_repo = Arc::new(PostRepositoryImpl::new(db.clone()));
    /// let user_repo = Arc::new(UserRepositoryImpl::new(db));
    /// let post_service = PostService::new(post_repo);
    /// let user_service = UserService::new(user_repo);
    /// let auth_state = AuthState::new("secret");
    /// 
    /// let state = AppState::new(post_service, user_service, auth_state);
    /// ```
    pub fn new(
        post_service: PostService<PR>,
        user_service: UserService<UR>,
        auth_state: AuthState,
    ) -> Self {
        Self {
            post_service: Arc::new(post_service),
            user_service: Arc::new(user_service),
            auth_state,
        }
    }

    /// Get a reference to the authentication state
    pub fn auth_state(&self) -> &AuthState {
        &self.auth_state
    }
}