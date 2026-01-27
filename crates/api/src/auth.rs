use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use domain::{
    LoginRequest, LoginResponse, RegisterRequest, UserInfo, ADMIN_PERMISSIONS,
    DEFAULT_USER_PERMISSIONS,
};
use infrastructure::UserRepository;


use crate::{
    error::ApiError,
    middleware::auth::Claims,
    state::AppState,
};

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

/// POST /api/auth/register
/// Register a new user (public endpoint)
pub async fn register(
    State(state): State<AppState>,
    Json(input): Json<RegisterRequest>,
) -> Result<impl IntoResponse, ApiError> {
    validate_username(&input.username)?;
    validate_password(&input.password)?;

    let existing_users = state.user_repo.find_by_username(&input.username).await?;
    if existing_users.is_some() {
        return Err(ApiError::Validation("Username already exists".to_string()));
    }

    let is_first_user = true;
    let permissions = if is_first_user {
        ADMIN_PERMISSIONS
    } else {
        DEFAULT_USER_PERMISSIONS
    };

    let user = state
        .user_repo
        .create_user(input.username.clone(), input.password, permissions)
        .await?;

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
        return Err(ApiError::Validation("Username and password required".to_string()));
    }

    let user = state
        .user_repo
        .verify_credentials(&input.username, &input.password)
        .await?
        .ok_or_else(|| ApiError::Unauthorized("Invalid credentials".to_string()))?;

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