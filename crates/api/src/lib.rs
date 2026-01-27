//! # API Crate
//!
//! This crate provides HTTP handlers and routing for the blog API.
//! It uses Axum as the web framework and handles authentication,
//! request validation, and response formatting.

pub mod auth;
pub mod error;
pub mod middleware;
pub mod post;
pub mod state;

// Re-export commonly used types for convenience
pub use state::AppState;
pub use post::routes;
pub use error::ApiResult;

// Re-export repository traits for generic type parameters
pub use service::{PostRepository, UserRepository};
pub use middleware::auth::{AuthState, Claims};