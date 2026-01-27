use axum::{
   extract::FromRequestParts,
   http::{request::Parts, StatusCode},
   response::{IntoResponse, Response},
   Json,
 };
use crate::state::AppState;
use domain::USER_MANAGE;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Arc;

// ============================================================================
// JWT Claims - The Axum Extractor for Authenticated Users
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,       // user_id
    pub username: String,
    pub permissions: u64,
    pub exp: usize,        // expiration time
}

impl Claims {
    pub fn new(user_id: String, username: String, permissions: u64, exp_hours: u64) -> Self {
        let exp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize
            + (exp_hours * 3600) as usize;

        Self {
            sub: user_id,
            username,
            permissions,
            exp,
        }
    }
}

// ============================================================================
// Auth Rejection Type
// ============================================================================

#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    TokenExpired,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::MissingToken => write!(f, "Missing authorization token"),
            AuthError::InvalidToken => write!(f, "Invalid authorization token"),
            AuthError::TokenExpired => write!(f, "Token expired"),
        }
    }
}

impl std::error::Error for AuthError {}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authorization token"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid authorization token"),
            AuthError::TokenExpired => (StatusCode::UNAUTHORIZED, "Token expired"),
        };

        let body = serde_json::json!({ "error": message });
        (status, Json(body)).into_response()
    }
}

// ============================================================================
// Implement FromRequestParts for Claims - Using Axum 0.8 State Pattern
// ============================================================================

impl FromRequestParts<AppState> for Claims {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let auth_header = parts
            .headers
            .get("authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(AuthError::MissingToken)?;

        // Parse "Bearer <token>" format
        let bearer_prefix = "Bearer ";
        if !auth_header.starts_with(bearer_prefix) {
            return Err(AuthError::InvalidToken);
        }
        let token = &auth_header[bearer_prefix.len()..];

        // Get AuthState from AppState
        let auth_state = &state.auth_state;

        // Decode and validate JWT
        let token_data = decode::<Claims>(
            token,
            &auth_state.decoding_key,
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        // Check expiration
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        if token_data.claims.exp < now {
            return Err(AuthError::TokenExpired);
        }

        Ok(token_data.claims)
    }
}

// ============================================================================
// Auth State - JWT Keys
// ============================================================================

#[derive(Clone)]
pub struct AuthState {
    pub encoding_key: Arc<EncodingKey>,
    pub decoding_key: Arc<DecodingKey>,
}

impl AuthState {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: Arc::new(EncodingKey::from_secret(secret.as_ref())),
            decoding_key: Arc::new(DecodingKey::from_secret(secret.as_ref())),
        }
    }

    pub fn create_token(&self, user_id: String, username: String, permissions: u64) -> Result<String, AuthError> {
        let claims = Claims::new(user_id, username, permissions, 24); // 24 hour expiry
        encode(&Header::default(), &claims, &self.encoding_key)
            .map_err(|_| AuthError::InvalidToken)
    }
}

// ============================================================================
// Permission Check Helper Functions (eliminate special cases)
// ============================================================================

pub fn check_permission(user: &Claims, permission: u64) -> Result<(), AuthError> {
    if (user.permissions & permission) == 0 {
        return Err(AuthError::InvalidToken); // Reuse token error for simplicity
    }
    Ok(())
}

pub fn require_permission(user: &Claims, permission: u64) -> Result<(), AuthError> {
    check_permission(user, permission)
}

// ============================================================================
// Permission Constants Helper Functions
// ============================================================================

impl Claims {
    pub fn is_admin(&self) -> bool {
        (self.permissions & USER_MANAGE) != 0
    }
}

// ============================================================================
// Ownership Check Helper
// ============================================================================

pub fn require_ownership(user: &Claims, post: &domain::Post) -> Result<(), AuthError> {
    let user_id = uuid::Uuid::parse_str(&user.sub)
        .map_err(|_| AuthError::InvalidToken)?;

    if !post.is_owned_by(user_id) && !user.is_admin() {
        return Err(AuthError::InvalidToken);
    }

    Ok(())
}
