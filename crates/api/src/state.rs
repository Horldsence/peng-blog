//! Application State
//!
//! This module defines the shared application state that is passed to all route handlers.
//! The state uses trait objects for services, allowing for easy testing
//! and swapping of different database backends.

use service::{
    CategoryService, CommentService, FileService, PostService, SessionService, StatsService,
    TagService, UserService,
};
use std::sync::Arc;

use crate::middleware::auth::AuthState;

/// Shared application state
///
/// This struct holds all the services and shared data needed by route handlers.
/// It uses trait objects for services to support different implementations
/// (e.g., real database vs. mock for testing) without generic type parameters.
#[derive(Clone)]
pub struct AppState {
    /// Post service with business logic for post operations
    pub post_service: Arc<PostService>,

    /// User service with business logic for user operations
    pub user_service: Arc<UserService>,

    /// Session service with business logic for session operations
    pub session_service: Arc<SessionService>,

    /// File service with business logic for file operations
    pub file_service: Arc<FileService>,

    /// Comment service with business logic for comment operations
    pub comment_service: Arc<CommentService>,

    /// Stats service with business logic for statistics operations
    pub stats_service: Arc<StatsService>,

    /// Category service with business logic for category operations
    pub category_service: Arc<CategoryService>,

    /// Tag service with business logic for tag operations
    pub tag_service: Arc<TagService>,

    /// Authentication state for JWT token operations
    pub auth_state: AuthState,

    /// Upload directory for file storage
    pub upload_dir: String,
}

impl AppState {
    /// Create a new builder for AppState
    pub fn builder() -> AppStateBuilder {
        AppStateBuilder::default()
    }
}

/// Builder for creating AppState instances
///
/// This builder follows the builder pattern to provide a fluent API
/// for constructing AppState with multiple parameters.
///
/// # Example
///
/// ```ignore
/// use api::{AppState, AuthState};
/// use service::{PostService, UserService, SessionService, FileService, CommentService, StatsService, CategoryService, TagService};
/// use infrastructure::{PostRepositoryImpl, UserRepositoryImpl, SessionRepositoryImpl, FileRepositoryImpl, CommentRepositoryImpl, StatsRepositoryImpl, CategoryRepositoryImpl, TagRepositoryImpl};
/// use std::sync::Arc;
///
/// // Initialize database connection
/// let db = establish_connection(&database_url).await?;
/// let post_repo = Arc::new(PostRepositoryImpl::new(db.clone()));
/// let user_repo = Arc::new(UserRepositoryImpl::new(db.clone()));
/// let session_repo = Arc::new(SessionRepositoryImpl::new(db.clone()));
/// let file_repo = Arc::new(FileRepositoryImpl::new(db.clone()));
/// let comment_repo = Arc::new(CommentRepositoryImpl::new(db.clone()));
/// let stats_repo = Arc::new(StatsRepositoryImpl::new(db.clone()));
/// let category_repo = Arc::new(CategoryRepositoryImpl::new(db.clone()));
/// let tag_repo = Arc::new(TagRepositoryImpl::new(db));
///
/// let post_service = PostService::new(post_repo);
/// let user_service = UserService::new(user_repo.clone());
/// let session_service = SessionService::new(session_repo);
/// let file_service = FileService::new(file_repo, "/uploads".to_string(), "http://example.com".to_string());
/// let comment_service = CommentService::new(comment_repo, user_repo, "client_id".to_string(), "client_secret".to_string());
/// let stats_service = StatsService::new(stats_repo);
/// let category_service = CategoryService::new(category_repo);
/// let tag_service = TagService::new(tag_repo);
/// let auth_state = AuthState::new("your-secret-key");
///
/// let state = AppState::builder()
///     .post_service(post_service)
///     .user_service(user_service)
///     .session_service(session_service)
///     .file_service(file_service)
///     .comment_service(comment_service)
///     .stats_service(stats_service)
///     .category_service(category_service)
///     .tag_service(tag_service)
///     .auth_state(auth_state)
///     .upload_dir("/uploads".to_string())
///     .build();
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Default)]
pub struct AppStateBuilder {
    post_service: Option<PostService>,
    user_service: Option<UserService>,
    session_service: Option<SessionService>,
    file_service: Option<FileService>,
    comment_service: Option<CommentService>,
    stats_service: Option<StatsService>,
    category_service: Option<CategoryService>,
    tag_service: Option<TagService>,
    auth_state: Option<AuthState>,
    upload_dir: Option<String>,
}

impl AppStateBuilder {
    pub fn post_service(mut self, service: PostService) -> Self {
        self.post_service = Some(service);
        self
    }

    pub fn user_service(mut self, service: UserService) -> Self {
        self.user_service = Some(service);
        self
    }

    pub fn session_service(mut self, service: SessionService) -> Self {
        self.session_service = Some(service);
        self
    }

    pub fn file_service(mut self, service: FileService) -> Self {
        self.file_service = Some(service);
        self
    }

    pub fn comment_service(mut self, service: CommentService) -> Self {
        self.comment_service = Some(service);
        self
    }

    pub fn stats_service(mut self, service: StatsService) -> Self {
        self.stats_service = Some(service);
        self
    }

    pub fn category_service(mut self, service: CategoryService) -> Self {
        self.category_service = Some(service);
        self
    }

    pub fn tag_service(mut self, service: TagService) -> Self {
        self.tag_service = Some(service);
        self
    }

    pub fn auth_state(mut self, state: AuthState) -> Self {
        self.auth_state = Some(state);
        self
    }

    pub fn upload_dir(mut self, dir: String) -> Self {
        self.upload_dir = Some(dir);
        self
    }

    /// Build the AppState
    ///
    /// # Panics
    ///
    /// Panics if any required field is not set.
    pub fn build(self) -> AppState {
        AppState {
            post_service: Arc::new(self.post_service.expect("post_service must be set")),
            user_service: Arc::new(self.user_service.expect("user_service must be set")),
            session_service: Arc::new(self.session_service.expect("session_service must be set")),
            file_service: Arc::new(self.file_service.expect("file_service must be set")),
            comment_service: Arc::new(self.comment_service.expect("comment_service must be set")),
            stats_service: Arc::new(self.stats_service.expect("stats_service must be set")),
            category_service: Arc::new(
                self.category_service.expect("category_service must be set"),
            ),
            tag_service: Arc::new(self.tag_service.expect("tag_service must be set")),
            auth_state: self.auth_state.expect("auth_state must be set"),
            upload_dir: self.upload_dir.expect("upload_dir must be set"),
        }
    }
}
