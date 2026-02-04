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
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json, Redirect},
    Router,
};
use domain::{CreateComment, CreateCommentGitHub};
use serde::Deserialize;
use uuid::Uuid;

use crate::{error::ApiError, middleware::auth::Claims, state::AppState};

// ============================================================================
// Routes
// ============================================================================

pub fn routes() -> Router<AppState> {
    Router::new()
        // GET /api/comments/github/auth - Get GitHub OAuth URL (most specific)
        .route("/github/auth", axum::routing::get(github_auth_url))
        // GET /api/comments/github/callback - GitHub OAuth callback
        .route("/github/callback", axum::routing::get(github_callback))
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
pub async fn github_auth_url(State(state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    // Generate random state for CSRF protection
    let state_param = uuid::Uuid::new_v4().to_string();

    // Build OAuth callback URL
    let redirect_uri = format!("{}/api/comments/github/callback", state.base_url);

    // Generate GitHub OAuth URL
    let auth_url = state
        .comment_service
        .github_auth_url(&state_param, &redirect_uri);

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "auth_url": auth_url,
            "state": state_param
        })),
    ))
}

#[derive(Deserialize)]
pub struct GitHubCallbackQuery {
    code: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
    #[allow(dead_code)]
    state: String,
}

/// GET /api/comments/github/callback
/// Handle GitHub OAuth callback
///
/// Query parameters:
/// - code: Authorization code from GitHub (on success)
/// - error: Error code from GitHub (on failure)
/// - error_description: Human-readable error description
/// - state: CSRF protection token
///
/// This endpoint:
/// 1. Exchanges code for GitHub access token (on success)
/// 2. Fetches GitHub user information
/// 3. Creates a 6-hour JWT token for the GitHub user
/// 4. Redirects to frontend with token or error
pub async fn github_callback(
    Query(query): Query<GitHubCallbackQuery>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    use domain::comment::{GitHubTokenResponse, GitHubUser};
    use jsonwebtoken::{encode, EncodingKey, Header};
    use reqwest::Client;
    use serde_json::json;
    use std::time::{SystemTime, UNIX_EPOCH};

    // Handle OAuth errors
    if let Some(error) = query.error {
        let error_description = query
            .error_description
            .unwrap_or_else(|| "Unknown error".to_string());
        let redirect_url = format!(
            "{}/github-auth?error={}&description={}",
            state.base_url,
            urlencoding::encode(&error),
            urlencoding::encode(&error_description)
        );
        return Ok(Redirect::to(&redirect_url));
    }

    // Get authorization code
    let code = query
        .code
        .ok_or_else(|| ApiError::Validation("Missing authorization code".to_string()))?;

    // Exchange code for access token
    let client = Client::new();
    let token_response: GitHubTokenResponse = client
        .post("https://github.com/login/oauth/access_token")
        .form(&[
            ("client_id", &state.config.github.client_id),
            ("client_secret", &state.config.github.client_secret),
            ("code", &code),
        ])
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| ApiError::Internal(format!("GitHub API error: {}", e)))?
        .json()
        .await
        .map_err(|e| ApiError::Internal(format!("GitHub API error: {}", e)))?;

    // Get GitHub user information
    let github_user: GitHubUser = client
        .get("https://api.github.com/user")
        .header(
            "Authorization",
            format!("Bearer {}", token_response.access_token),
        )
        .header("User-Agent", "peng-blog")
        .send()
        .await
        .map_err(|e| ApiError::Internal(format!("GitHub API error: {}", e)))?
        .json()
        .await
        .map_err(|e| ApiError::Internal(format!("GitHub API error: {}", e)))?;

    // Create 6-hour JWT token for GitHub user
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0) as usize;

    let expiration = now + 60 * 60 * 6; // 6 hours

    let claims = json!({
        "sub": github_user.login.clone(), // Use GitHub username as user ID
        "username": github_user.login.clone(),
        "avatar_url": github_user.avatar_url.clone(),
        "exp": expiration,
        "iat": now,
        "permissions": 0u64, // GitHub users have no special permissions
    });

    let secret = state.auth_state.get_secret();
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| ApiError::Internal(format!("Failed to create token: {}", e)))?;

    // Redirect to frontend page with token
    let redirect_url = format!("{}/github-auth?token={}", state.base_url, token);
    Ok(Redirect::to(&redirect_url))
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
