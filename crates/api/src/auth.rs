//! Authentication API Routes
//!
//! This module provides HTTP handlers for authentication operations.
//! All auth routes are public (no authentication required).

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json, Router};
use domain::{LoginRequest, LoginResponse, RegisterRequest, UserInfo};

use crate::{error::ApiError, middleware::auth::Claims, state::AppState};

// ============================================================================
// Routes
// ============================================================================

pub fn routes() -> Router<AppState> {
    Router::new()
        // POST /api/auth/register - Register a new user (public)
        .route("/register", axum::routing::post(register))
        // POST /api/auth/login - Login with username/password (public)
        .route("/login", axum::routing::post(login))
        // GET /api/auth/me - Get current user info (requires authentication)
        .route("/me", axum::routing::get(me))
}

// ============================================================================
// Handlers
// ============================================================================

/// POST /api/auth/register
/// Register a new user (public endpoint)
pub async fn register(
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

    Ok((
        StatusCode::CREATED,
        Json(LoginResponse {
            token,
            user: UserInfo::from(&user),
        }),
    ))
}

/// POST /api/auth/login
/// Login with username/password (public endpoint)
pub async fn login(
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

    Ok((
        StatusCode::OK,
        Json(LoginResponse {
            token,
            user: UserInfo::from(&user),
        }),
    ))
}

/// GET /api/auth/me
/// Get current user info (requires authentication)
pub async fn me(
    user: Claims,
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = uuid::Uuid::parse_str(&user.sub)
        .map_err(|e| ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    let user_info = domain::UserInfo {
        id: user_id,
        username: user.username,
        permissions: user.permissions,
    };

    Ok((StatusCode::OK, Json(user_info)))
}

// ============================================================================
// Private Validation Helpers
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
