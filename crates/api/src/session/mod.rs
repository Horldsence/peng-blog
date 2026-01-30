//! Session API Routes
//!
//! This module provides HTTP handlers for session management.
//! Sessions are used for cookie-based authentication.
//!
//! Design Principles:
//! - Simple RESTful endpoints
//! - Cookie-based authentication using Set-Cookie header
//! - No special cases - all sessions follow the same rules

use axum::{
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
    Json, Router,
};
use domain::UserInfo;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{error::ApiError, middleware::auth::Claims, state::AppState};

// ============================================================================
// Routes
// ============================================================================

pub fn routes() -> Router<AppState> {
    Router::new()
        // POST /api/sessions - Create session (login with cookie)
        .route("/", axum::routing::post(create_session))
        // DELETE /api/sessions - Delete session (logout)
        .route("/", axum::routing::delete(delete_session))
        // GET /api/sessions/info - Get current session info
        .route("/info", axum::routing::get(get_session_info))
        // POST /api/sessions/github - Create session via GitHub OAuth
        .route("/github", axum::routing::post(github_callback))
}

// ============================================================================
// Request/Response Types
// ============================================================================

/// Request to create a session
#[derive(Debug, Deserialize)]
pub struct CreateSessionRequest {
    pub username: String,
    pub password: String,
    pub remember_me: bool,
}

/// Response with session cookie
#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub token: String,
    pub user: UserInfo,
}

// ============================================================================
// Handlers
// ============================================================================

/// POST /api/sessions
/// Create a new session and set cookie
pub async fn create_session(
    State(state): State<AppState>,
    Json(input): Json<CreateSessionRequest>,
) -> Result<impl IntoResponse, ApiError> {
    // Validate input
    if input.username.trim().is_empty() || input.password.trim().is_empty() {
        return Err(ApiError::Validation(
            "Username and password required".to_string(),
        ));
    }

    // Login user
    let user = state
        .user_service
        .login(input.username, input.password)
        .await
        .map_err(|e| match e {
            domain::Error::NotFound(msg) => ApiError::Unauthorized(msg),
            _ => ApiError::Domain(e),
        })?;

    // Create session
    let session = state
        .session_service
        .create_session(user.id, input.remember_me)
        .await?;

    // Set cookie
    let cookie_value = format!(
        "session_token={}; Path=/; HttpOnly; SameSite=Lax; Max-Age={}",
        session.id,
        if input.remember_me {
            30 * 24 * 60 * 60
        } else {
            24 * 60 * 60
        }
    );

    let response = SessionResponse {
        token: session.id,
        user: UserInfo::from(&user),
    };

    Ok((
        StatusCode::CREATED,
        [(header::SET_COOKIE, cookie_value)],
        Json(response),
    ))
}

/// DELETE /api/sessions
/// Delete current session (logout)
pub async fn delete_session(State(_state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    // Delete session (we'll get the token from cookie in a real implementation)
    // For now, just return success - the cookie will be cleared on client side

    let cookie_value = "session_token=; Path=/; HttpOnly; SameSite=Lax; Max-Age=0";

    Ok((
        StatusCode::OK,
        [(header::SET_COOKIE, cookie_value)],
        Json(serde_json::json!({ "message": "Logged out successfully" })),
    ))
}

/// GET /api/sessions/me
/// Get current session info
pub async fn get_session_info(
    user: Claims,
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let user_info = domain::UserInfo {
        id: Uuid::parse_str(&user.sub)
            .map_err(|e| ApiError::Internal(format!("Invalid user ID: {}", e)))?,
        username: user.username,
        permissions: user.permissions,
    };

    Ok((StatusCode::OK, Json(user_info)))
}

/// POST /api/sessions/github
/// Handle GitHub OAuth callback
pub async fn github_callback(
    State(_state): State<AppState>,
    Json(_input): Json<serde_json::Value>,
) -> Result<impl IntoResponse, ApiError> {
    // This will be implemented to handle GitHub OAuth callback
    // For now, return placeholder
    Ok((
        StatusCode::OK,
        Json(serde_json::json!({ "message": "GitHub OAuth not implemented yet" })),
    ))
}
