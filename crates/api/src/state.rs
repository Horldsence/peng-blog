//! Application State
//!
//! This module defines the shared application state that is passed to all route handlers.
//! The state is generic over repository implementations, allowing for easy testing
//! and swapping of different database backends.

use service::{
    PostService, UserService, SessionService, FileService, CommentService, StatsService,
    PostRepository, UserRepository, SessionRepository, FileRepository, CommentRepository, StatsRepository,
};
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
/// * `SR` - Session repository implementation
/// * `FR` - File repository implementation
/// * `CR` - Comment repository implementation
/// * `STR` - Stats repository implementation
#[derive(Clone)]
pub struct AppState<PR, UR, SR, FR, CR, STR>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
    /// Post service with business logic for post operations
    pub post_service: Arc<PostService<PR>>,

    /// User service with business logic for user operations
    pub user_service: Arc<UserService<UR>>,

    /// Session service with business logic for session operations
    pub session_service: Arc<SessionService<SR>>,

    /// File service with business logic for file operations
    pub file_service: Arc<FileService<FR>>,

    /// Comment service with business logic for comment operations
    pub comment_service: Arc<CommentService<CR, UR>>,

    /// Stats service with business logic for statistics operations
    pub stats_service: Arc<StatsService<STR>>,

    /// Authentication state for JWT token operations
    pub auth_state: AuthState,

    /// Upload directory for file storage
    pub upload_dir: String,
}

impl<PR, UR, SR, FR, CR, STR> AppState<PR, UR, SR, FR, CR, STR>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
    /// Create a new application state
    ///
    /// # Arguments
    /// * `post_service` - Post service instance
    /// * `user_service` - User service instance
    /// * `session_service` - Session service instance
    /// * `file_service` - File service instance
    /// * `comment_service` - Comment service instance
    /// * `stats_service` - Stats service instance
    /// * `auth_state` - Authentication state for JWT operations
    /// * `upload_dir` - Directory for file uploads
    ///
    /// # Example
    ///
    /// ```ignore
    /// use api::{AppState, AuthState};
    /// use service::{PostService, UserService, SessionService, FileService, CommentService, StatsService};
    /// use infrastructure::{PostRepositoryImpl, UserRepositoryImpl, SessionRepositoryImpl, FileRepositoryImpl, CommentRepositoryImpl, StatsRepositoryImpl};
    /// use std::sync::Arc;
    ///
    /// // Initialize database connection
    /// let db = establish_connection(&database_url).await?;
    /// let post_repo = Arc::new(PostRepositoryImpl::new(db.clone()));
    /// let user_repo = Arc::new(UserRepositoryImpl::new(db.clone()));
    /// let session_repo = Arc::new(SessionRepositoryImpl::new(db.clone()));
    /// let file_repo = Arc::new(FileRepositoryImpl::new(db.clone()));
    /// let comment_repo = Arc::new(CommentRepositoryImpl::new(db.clone()));
    /// let stats_repo = Arc::new(StatsRepositoryImpl::new(db));
    ///
    /// let post_service = PostService::new(post_repo);
    /// let user_service = UserService::new(user_repo.clone());
    /// let session_service = SessionService::new(session_repo);
    /// let file_service = FileService::new(file_repo, "/uploads".to_string(), "http://example.com".to_string());
    /// let comment_service = CommentService::new(comment_repo, user_repo, "client_id".to_string(), "client_secret".to_string());
    /// let stats_service = StatsService::new(stats_repo);
    /// let auth_state = AuthState::new("your-secret-key");
    ///
    /// let state = AppState::new(post_service, user_service, session_service, file_service, comment_service, stats_service, auth_state, "/uploads".to_string());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(
        post_service: PostService<PR>,
        user_service: UserService<UR>,
        session_service: SessionService<SR>,
        file_service: FileService<FR>,
        comment_service: CommentService<CR, UR>,
        stats_service: StatsService<STR>,
        auth_state: AuthState,
        upload_dir: String,
    ) -> Self {
        Self {
            post_service: Arc::new(post_service),
            user_service: Arc::new(user_service),
            session_service: Arc::new(session_service),
            file_service: Arc::new(file_service),
            comment_service: Arc::new(comment_service),
            stats_service: Arc::new(stats_service),
            auth_state,
            upload_dir,
        }
    }

    /// Get a reference to the authentication state
    pub fn auth_state(&self) -> &AuthState {
        &self.auth_state
    }
}
