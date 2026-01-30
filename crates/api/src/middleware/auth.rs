//! JWT Authentication Middleware
//!
//! This module provides JWT-based authentication for the API.
//! It includes:
//! - Claims: JWT token structure with user information
//! - AuthState: JWT token creation and validation
//! - AuthError: Authentication error types
//! - FromRequestParts implementation for automatic token extraction

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use domain::USER_MANAGE;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// JWT Claims structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject - user ID
    pub sub: String,
    /// Username
    pub username: String,
    /// Expiration time as Unix timestamp
    pub exp: usize,
    /// Issued at time as Unix timestamp
    pub iat: usize,
    /// User permissions as bit flags
    pub permissions: u64,
}

/// Authentication state for JWT operations
#[derive(Clone)]
pub struct AuthState {
    /// Secret key for signing tokens
    secret: String,
}

impl AuthState {
    /// Create a new AuthState with the given secret
    pub fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
        }
    }

    /// Create a JWT token for a user
    pub fn create_token(
        &self,
        user_id: impl Into<String>,
        username: String,
        permissions: u64,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0) as usize;

        let expiration = now + 60 * 60 * 24; // 24 hours

        let claims = Claims {
            sub: user_id.into(),
            username,
            exp: expiration,
            iat: now,
            permissions,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
    }

    /// Decode and validate a JWT token
    pub fn decode_token(&self, token: &str) -> Result<Claims, AuthError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map(|data| data.claims)
        .map_err(|_| AuthError::InvalidToken)
    }
}

/// Authentication errors
#[derive(Debug)]
pub enum AuthError {
    MissingToken,
    InvalidToken,
    ExpiredToken,
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::MissingToken => write!(f, "Missing authentication token"),
            AuthError::InvalidToken => write!(f, "Invalid authentication token"),
            AuthError::ExpiredToken => write!(f, "Token has expired"),
        }
    }
}

impl std::error::Error for AuthError {}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authentication token"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid authentication token"),
            AuthError::ExpiredToken => (StatusCode::UNAUTHORIZED, "Token has expired"),
        };

        let body = Json(serde_json::json!({
            "error": message
        }));

        (status, body).into_response()
    }
}

/// Extractor for authenticated user from JWT token
///
/// This automatically extracts and validates JWT tokens from the Authorization header.
/// The header should be in the format: "Bearer <token>"
///
/// # Example
///
/// ```ignore
/// use axum::{routing::get, Router};
/// use api::middleware::auth::Claims;
///
/// async fn protected_route(claims: Claims) -> String {
///     format!("Hello, {}!", claims.username)
/// }
///
/// let app = Router::new()
///     .route("/protected", get(protected_route));
/// ```
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract Authorization header
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(AuthError::MissingToken)?;

        // Parse "Bearer <token>" format
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AuthError::InvalidToken)?;

        // Decode token with a default secret key
        // Note: In production, this should use the secret from app state
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(b"change-this-secret-in-production"),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        // Check expiration
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;

        if token_data.claims.exp < now {
            return Err(AuthError::ExpiredToken);
        }

        Ok(token_data.claims)
    }
}

// Implement OptionalFromRequestParts to allow Option<Claims> as an extractor
// This enables routes that work with or without authentication
impl<S> axum::extract::OptionalFromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        Ok(
            <Self as FromRequestParts<S>>::from_request_parts(parts, state)
                .await
                .ok(),
        )
    }
}

/// Check if user has the required permission
///
/// Returns Ok(()) if the user has the permission, otherwise returns an error
pub fn require_permission(user: &Claims, permission: u64) -> Result<(), AuthError> {
    if (user.permissions & permission) == 0 {
        return Err(AuthError::InvalidToken);
    }
    Ok(())
}

/// Check if user can manage other users (admin only)
pub fn require_admin(user: &Claims) -> Result<(), AuthError> {
    require_permission(user, USER_MANAGE)
}
