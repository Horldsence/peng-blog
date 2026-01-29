//! # API Crate
//!
//! This crate provides HTTP handlers and routing for the blog API.
//! It uses Axum as the web framework and handles authentication,
//! request validation, and response formatting.

pub mod auth;
pub mod error;
pub mod middleware;
pub mod post;
pub mod user;
pub mod state;
pub mod session;
pub mod file;
pub mod comment;
pub mod stats;

// Re-export commonly used types for convenience
pub use state::AppState;
pub use error::ApiResult;

// Re-export repository traits for generic type parameters
pub use service::{PostRepository, UserRepository, SessionRepository, FileRepository, CommentRepository, StatsRepository};
pub use middleware::auth::{AuthState, Claims};

// ============================================================================
// Unified Routes Entry Point
// ============================================================================

/// Create complete API router by merging all route modules
///
/// This function encapsulates routing structure, keeping main.rs clean.
/// Routes are organized by resource type: auth, posts, users, sessions, files, comments, and stats.
///
/// Returns a router that requires application state to be provided via `with_state()`.
///
/// # Example
///
/// ```ignore
/// let app = axum::Router::new()
///     .nest("/api", api::routes())
///     .with_state(app_state);
/// ```
pub fn routes<PR, UR, SR, FR, CR, STR>() -> axum::Router<
    AppState<PR, UR, SR, FR, CR, STR>
>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
    axum::Router::new()
        .nest("/auth", auth::routes::<PR, UR, SR, FR, CR, STR>())
        .nest("/posts", post::routes::<PR, UR, SR, FR, CR, STR>())
        .nest("/users", user::routes::<PR, UR, SR, FR, CR, STR>())
        .nest("/sessions", session::routes::<PR, UR, SR, FR, CR, STR>())
        .nest("/files", file::routes::<PR, UR, SR, FR, CR, STR>())
        .nest("/comments", comment::routes::<PR, UR, SR, FR, CR, STR>())
        .nest("/stats", stats::routes::<PR, UR, SR, FR, CR, STR>())
}