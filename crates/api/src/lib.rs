//! # API Crate
//!
//! This crate provides HTTP handlers and routing for the blog API.
//! It uses Axum as the web framework and handles authentication,
//! request validation, and response formatting.

pub mod auth;
pub mod bing;
pub mod category;
pub mod comment;
pub mod error;
pub mod file;
pub mod middleware;
pub mod post;
pub mod response;
pub mod session;
pub mod state;
pub mod stats;
pub mod tag;
pub mod user;

// Re-export commonly used types for convenience
pub use error::{ApiError, ApiResult};
pub use response::{helpers as resp, ApiResponse, ErrorResponse, Pagination, SuccessResponse};
pub use state::AppState;

// Re-export middleware types
pub use middleware::auth::{AuthState, Claims};

// ============================================================================
// Unified Routes Entry Point
// ============================================================================

/// Create complete API router by merging all route modules
///
/// This function encapsulates routing structure, keeping main.rs clean.
/// Routes are organized by resource type: auth, posts, users, sessions, files, comments, stats, categories, and tags.
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
pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .nest("/auth", auth::routes())
        .nest("/posts", post::routes())
        .nest("/users", user::routes())
        .nest("/sessions", session::routes())
        .nest("/files", file::routes())
        .nest("/comments", comment::routes())
        .nest("/stats", stats::routes())
        .nest("/categories", category::routes())
        .nest("/tags", tag::routes())
        .nest("/bing", bing::routes())
}
