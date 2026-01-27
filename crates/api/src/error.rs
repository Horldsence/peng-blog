use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use domain::Error as DomainError;
use serde_json::json;
use thiserror::Error;
use crate::middleware::auth::AuthError;

/// API error types
#[derive(Debug, Error)]
pub enum ApiError {
    /// Domain errors from business logic layer
    #[error("Domain error: {0}")]
    Domain(DomainError),

    /// Validation errors from input validation
    #[error("Validation error: {0}")]
    Validation(String),

    /// Authentication/authorization errors
    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    /// Internal server errors
    #[error("Internal error: {0}")]
    Internal(String),

    /// Authentication errors from middleware
    #[error("Authentication failed")]
    Auth(#[from] AuthError),
}

impl ApiError {
    /// Create a validation error
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }

    /// Create an unauthorized error
    pub fn unauthorized(msg: impl Into<String>) -> Self {
        Self::Unauthorized(msg.into())
    }

    /// Create an internal error
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match self {
            ApiError::Domain(DomainError::NotFound(msg)) => {
                (StatusCode::NOT_FOUND, "not_found", msg)
            }
            ApiError::Domain(DomainError::Validation(msg)) => {
                (StatusCode::BAD_REQUEST, "validation", msg)
            }
            ApiError::Domain(DomainError::Unauthorized(msg)) => {
                (StatusCode::UNAUTHORIZED, "unauthorized", msg)
            }
            ApiError::Domain(DomainError::Conflict(msg)) => {
                (StatusCode::CONFLICT, "conflict", msg)
            }
            ApiError::Domain(DomainError::Internal(msg)) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal", msg)
            }
            ApiError::Validation(msg) => {
                (StatusCode::BAD_REQUEST, "validation", msg)
            }
            ApiError::Unauthorized(msg) => {
                (StatusCode::UNAUTHORIZED, "unauthorized", msg)
            }
            ApiError::Internal(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal", msg)
            }
            ApiError::Auth(auth_err) => {
                // Let AuthError handle its own response
                return auth_err.into_response();
            }
        };

        let body = json!({
            "error": {
                "type": error_type,
                "message": message
            }
        });

        (status, Json(body)).into_response()
    }
}

/// Result type alias for API handlers
pub type ApiResult<T> = Result<T, ApiError>;

impl From<DomainError> for ApiError {
    fn from(err: DomainError) -> Self {
        Self::Domain(err)
    }
}

impl From<jsonwebtoken::errors::Error> for ApiError {
    fn from(err: jsonwebtoken::errors::Error) -> Self {
        Self::Internal(format!("JWT error: {}", err))
    }
}