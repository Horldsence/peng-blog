use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use domain::post::Error as DomainError;
use serde_json::json;
use thiserror::Error;
use crate::middleware::auth::AuthError;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error("Internal server error: {0}")]
    Internal(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Auth error: {0}")]
    Auth(#[from] AuthError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ApiError::Domain(err) => match err {
                DomainError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
                DomainError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
                DomainError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            },
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Auth(err) => (StatusCode::UNAUTHORIZED, format!("{:?}", err)),
        };

        let body = json!({
            "error": message,
        });

        (status, Json(body)).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
