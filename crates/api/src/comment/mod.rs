//! Comment API Routes
//!
//! This module provides HTTP handlers for comment management.
//! Comments support both registered users and GitHub OAuth users.
//!
//! Design Principles:
//! - Simple RESTful endpoints
//! - Unified error handling
//! - No special cases - all comments follow the same rules

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    Router,
};
use domain::{CreateComment, CreateCommentGitHub};
use uuid::Uuid;

use crate::{error::ApiError, middleware::auth::Claims, state::AppState};

// ============================================================================
// Routes
// ============================================================================

pub fn routes() -> Router<AppState> {
    Router::new()
        // GET /api/comments/github/auth - Get GitHub OAuth URL (most specific)
        .route("/github/auth", axum::routing::get(github_auth_url))
        // POST /api/comments/github - Create comment (GitHub user)
        .route("/github", axum::routing::post(create_comment_github))
        // GET /api/comments/posts/{id} - Get comments for a post
        .route("/posts/{id}", axum::routing::get(list_post_comments))
        // POST /api/comments - Create comment (registered user)
        .route("/", axum::routing::post(create_comment))
        // GET /api/comments/{id} - Get single comment
        .route("/{id}", axum::routing::get(get_comment))
        // PUT /api/comments/{id} - Update comment
        .route("/{id}", axum::routing::put(update_comment))
        // DELETE /api/comments/{id} - Delete comment
        .route("/{id}", axum::routing::delete(delete_comment))
}

// ============================================================================
// Handlers
// ============================================================================

/// GET /api/comments/github/auth
/// Get GitHub OAuth authorization URL
///
/// Query parameters:
/// - state: Random string for CSRF protection
/// - redirect_uri: OAuth callback URL
///
/// This endpoint is public - no authentication required.
pub async fn github_auth_url(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    // Note: In a real implementation, we would:
    // 1. Generate a random state
    // 2. Store it in a cache with expiry
    // 3. Return the GitHub OAuth URL

    // For now, return a placeholder response
    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "message": "GitHub OAuth URL generation not implemented yet",
            "note": "In production, this would generate and return a GitHub OAuth URL with state parameter"
        })),
    ))
}

/// POST /api/comments
/// Create a new comment (registered user only)
///
/// Request body:
/// - post_id: UUID of the post
/// - content: Comment content
///
/// Requires JWT authentication.
pub async fn create_comment(
    user: Claims,
    State(state): State<AppState>,
    Json(input): Json<CreateComment>,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = Uuid::parse_str(&user.sub)
        .map_err(|e| ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    let response = state
        .comment_service
        .create_comment(user_id, input)
        .await
        .map_err(ApiError::Domain)?;

    Ok((StatusCode::CREATED, Json(response)))
}

/// POST /api/comments/github
/// Create a new comment (GitHub OAuth user)
///
/// Request body:
/// - post_id: UUID of the post
/// - github_code: Authorization code from GitHub OAuth callback
/// - content: Comment content
///
/// This endpoint is public - no authentication required.
/// The service will perform the full GitHub OAuth flow.
pub async fn create_comment_github(
    State(state): State<AppState>,
    Json(input): Json<CreateCommentGitHub>,
) -> Result<impl IntoResponse, ApiError> {
    let response = state
        .comment_service
        .create_comment_github(input)
        .await
        .map_err(|e| match e {
            domain::Error::Validation(msg) => ApiError::Validation(msg),
            domain::Error::Internal(msg) => ApiError::Internal(msg),
            _ => ApiError::Domain(e),
        })?;

    Ok((StatusCode::CREATED, Json(response)))
}

/// GET /api/comments/posts/:id?limit=50
/// Get comments for a specific post
///
/// Query parameters:
/// - limit: Maximum number of comments to return (default: 50)
///
/// This endpoint is public - no authentication required.
pub async fn list_post_comments(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let post_id = Uuid::parse_str(&id)
        .map_err(|e| ApiError::Validation(format!("Invalid post ID: {}", e)))?;

    let limit = 50; // Default limit

    let comments = state
        .comment_service
        .list_post_comments(post_id, limit)
        .await
        .map_err(ApiError::Domain)?;

    Ok((StatusCode::OK, Json(comments)))
}

/// GET /api/comments/:id
/// Get a single comment by ID
///
/// This endpoint is public - no authentication required.
pub async fn get_comment(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let comment_id = Uuid::parse_str(&id)
        .map_err(|e| ApiError::Validation(format!("Invalid comment ID: {}", e)))?;

    let response = state
        .comment_service
        .get_comment(comment_id)
        .await
        .map_err(ApiError::Domain)?;

    match response {
        Some(comment) => Ok((StatusCode::OK, Json(comment))),
        None => Err(ApiError::validation("Comment not found".to_string())),
    }
}

/// PUT /api/comments/:id
/// Update a comment
///
/// Request body:
/// - content: New comment content
///
/// Only the comment author can update their own comment.
/// Requires JWT authentication for registered users.
pub async fn update_comment(
    user: Claims,
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(input): Json<serde_json::Value>,
) -> Result<impl IntoResponse, ApiError> {
    let comment_id = Uuid::parse_str(&id)
        .map_err(|e| ApiError::Validation(format!("Invalid comment ID: {}", e)))?;

    let user_id = Some(
        Uuid::parse_str(&user.sub)
            .map_err(|e| ApiError::Internal(format!("Invalid user ID: {}", e)))?,
    );

    let content = input
        .get("content")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ApiError::Validation("Content is required".to_string()))?
        .to_string();

    let response = state
        .comment_service
        .update_comment(comment_id, user_id, false, content)
        .await
        .map_err(|e| match e {
            domain::Error::NotFound(msg) => ApiError::validation(msg),
            domain::Error::Validation(msg) => ApiError::validation(msg),
            _ => ApiError::Domain(e),
        })?;

    Ok((StatusCode::OK, Json(response)))
}

/// DELETE /api/comments/:id
/// Delete a comment
///
/// Only the comment author can delete their own comment.
/// Requires JWT authentication for registered users.
pub async fn delete_comment(
    user: Claims,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let comment_id = Uuid::parse_str(&id)
        .map_err(|e| ApiError::Validation(format!("Invalid comment ID: {}", e)))?;

    let user_id = Some(
        Uuid::parse_str(&user.sub)
            .map_err(|e| ApiError::Internal(format!("Invalid user ID: {}", e)))?,
    );

    state
        .comment_service
        .delete_comment(comment_id, user_id, false)
        .await
        .map_err(|e| match e {
            domain::Error::NotFound(msg) => ApiError::validation(msg),
            domain::Error::Validation(msg) => ApiError::validation(msg),
            _ => ApiError::Domain(e),
        })?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({ "message": "Comment deleted successfully" })),
    ))
}
