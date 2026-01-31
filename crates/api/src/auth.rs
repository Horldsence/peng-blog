//! Authentication API Routes
//!
//! This module provides HTTP handlers for authentication operations.
//! All auth routes are public except `/auth/me`.
//!
//! ## Endpoints
//!
//! | Method | Endpoint | Description |
//! |--------|----------|-------------|
//! | POST | /auth/register | Register new user |
//! | POST | /auth/login | Login with credentials |
//! | POST | /auth/logout | Logout (client-side token removal) |
//! | GET | /auth/me | Get current user info |

use axum::{extract::State, response::IntoResponse, Json, Router};
use domain::{LoginRequest, LoginResponse, RegisterRequest, UserInfo};

use crate::{
    error::ApiError, middleware::auth::Claims, response::helpers as resp, state::AppState,
};

/// Create auth routes
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/register", axum::routing::post(register))
        .route("/login", axum::routing::post(login))
        .route("/logout", axum::routing::post(logout))
        .route("/me", axum::routing::get(me))
}

/// POST /auth/register
/// Register a new user
async fn register(
    State(state): State<AppState>,
    Json(input): Json<RegisterRequest>,
) -> Result<impl IntoResponse, ApiError> {
    validate_username(&input.username)?;
    validate_password(&input.password)?;

    let user = state
        .user_service
        .register(input.username, input.password)
        .await
        .map_err(ApiError::Domain)?;

    let token = state.auth_state.create_token(
        user.id.to_string(),
        user.username.clone(),
        user.permissions,
    )?;

    let response = LoginResponse {
        token,
        user: UserInfo::from(&user),
    };

    Ok(resp::created(response))
}

/// POST /auth/login
/// Login with username and password
async fn login(
    State(state): State<AppState>,
    Json(input): Json<LoginRequest>,
) -> Result<impl IntoResponse, ApiError> {
    if input.username.trim().is_empty() || input.password.trim().is_empty() {
        return Err(ApiError::Validation(
            "Username and password required".to_string(),
        ));
    }

    let user = state
        .user_service
        .login(input.username, input.password)
        .await
        .map_err(|e| match e {
            domain::Error::NotFound(msg) => ApiError::Unauthorized(msg),
            _ => ApiError::Domain(e),
        })?;

    let token = state.auth_state.create_token(
        user.id.to_string(),
        user.username.clone(),
        user.permissions,
    )?;

    let response = LoginResponse {
        token,
        user: UserInfo::from(&user),
    };

    Ok(resp::ok(response))
}

/// POST /auth/logout
/// Logout (informative endpoint - actual logout is client-side)
async fn logout() -> impl IntoResponse {
    // JWT tokens are stateless, so actual logout happens client-side
    // by removing the token from storage
    resp::ok(serde_json::json!({
        "message": "Logout successful. Please remove the token from client storage."
    }))
}

/// GET /auth/me
/// Get current user info (requires authentication)
async fn me(user: Claims, State(_state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    let user_id = uuid::Uuid::parse_str(&user.sub)
        .map_err(|e| ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    let user_info = domain::UserInfo {
        id: user_id,
        username: user.username,
        permissions: user.permissions,
    };

    Ok(resp::ok(user_info))
}

// ============================================================================
// Validation Helpers
// ============================================================================

fn validate_username(username: &str) -> Result<(), ApiError> {
    if username.trim().is_empty() {
        return Err(ApiError::Validation("Username cannot be empty".to_string()));
    }
    if username.len() < 3 {
        return Err(ApiError::Validation(
            "Username must be at least 3 characters".to_string(),
        ));
    }
    if username.len() > 30 {
        return Err(ApiError::Validation(
            "Username too long (max 30 characters)".to_string(),
        ));
    }
    Ok(())
}

fn validate_password(password: &str) -> Result<(), ApiError> {
    if password.len() < 8 {
        return Err(ApiError::Validation(
            "Password must be at least 8 characters".to_string(),
        ));
    }
    Ok(())
}
