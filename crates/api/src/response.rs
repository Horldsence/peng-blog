//! Unified API Response Format
//!
//! This module provides standardized response structures for all API endpoints,
//! ensuring consistent response format across the entire API.
//!
//! # Response Format
//!
//! ## Success Response (Single Resource)
//! ```json
//! {
//!   "code": 200,
//!   "message": "success",
//!   "data": { ... }
//! }
//! ```
//!
//! ## Success Response (List)
//! ```json
//! {
//!   "code": 200,
//!   "message": "success",
//!   "data": [ ... ],
//!   "pagination": {
//!     "page": 1,
//!     "per_page": 20,
//!     "total": 100,
//!     "total_pages": 5
//!   }
//! }
//! ```
//!
//! ## Error Response
//! ```json
//! {
//!   "code": 400,
//!   "message": "Validation failed",
//!   "errors": {
//!     "field": ["error message"]
//!   }
//! }
//! ```

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

/// Pagination information for list responses
#[derive(Debug, Serialize, Clone)]
pub struct Pagination {
    pub page: u64,
    pub per_page: u64,
    pub total: u64,
    pub total_pages: u64,
}

impl Pagination {
    /// Create pagination info from page, per_page and total count
    pub fn new(page: u64, per_page: u64, total: u64) -> Self {
        let total_pages = if total == 0 {
            1
        } else {
            total.div_ceil(per_page)
        };
        Self {
            page,
            per_page,
            total,
            total_pages,
        }
    }

    /// Create pagination from limit/offset style parameters
    pub fn from_limit_offset(limit: u64, offset: u64, total: u64) -> Self {
        let page = if limit == 0 { 1 } else { offset / limit + 1 };
        Self::new(page, limit, total)
    }
}

/// Standard API response wrapper
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

impl<T: Serialize> ApiResponse<T> {
    /// Create a successful response with data
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            message: "success".to_string(),
            data: Some(data),
            pagination: None,
        }
    }

    /// Create a created response (201)
    pub fn created(data: T) -> Self {
        Self {
            code: 201,
            message: "created".to_string(),
            data: Some(data),
            pagination: None,
        }
    }

    /// Create a successful response with pagination
    pub fn list(data: T, pagination: Pagination) -> Self {
        Self {
            code: 200,
            message: "success".to_string(),
            data: Some(data),
            pagination: Some(pagination),
        }
    }

    /// Create a custom response
    pub fn custom(code: u16, message: impl Into<String>, data: T) -> Self {
        Self {
            code,
            message: message.into(),
            data: Some(data),
            pagination: None,
        }
    }

    /// Convert to HTTP response with appropriate status code
    pub fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.code).unwrap_or(StatusCode::OK);
        (status, Json(self)).into_response()
    }
}

/// Simple success response without data
#[derive(Debug, Serialize)]
pub struct SuccessResponse {
    pub code: u16,
    pub message: String,
}

impl SuccessResponse {
    pub fn ok() -> Self {
        Self {
            code: 200,
            message: "success".to_string(),
        }
    }

    pub fn created() -> Self {
        Self {
            code: 201,
            message: "created".to_string(),
        }
    }

    pub fn deleted() -> Self {
        Self {
            code: 204,
            message: "deleted".to_string(),
        }
    }

    pub fn custom(code: u16, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    pub fn into_response(self) -> Response {
        if self.code == 204 {
            StatusCode::NO_CONTENT.into_response()
        } else {
            let status = StatusCode::from_u16(self.code).unwrap_or(StatusCode::OK);
            (status, Json(self)).into_response()
        }
    }
}

/// Error response structure
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<serde_json::Value>,
}

impl ErrorResponse {
    pub fn new(code: u16, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            errors: None,
        }
    }

    pub fn with_errors(code: u16, message: impl Into<String>, errors: serde_json::Value) -> Self {
        Self {
            code,
            message: message.into(),
            errors: Some(errors),
        }
    }

    pub fn validation(errors: serde_json::Value) -> Self {
        Self {
            code: 400,
            message: "validation failed".to_string(),
            errors: Some(errors),
        }
    }

    pub fn not_found(resource: impl Into<String>) -> Self {
        Self {
            code: 404,
            message: format!("{} not found", resource.into()),
            errors: None,
        }
    }

    pub fn unauthorized(message: impl Into<String>) -> Self {
        Self {
            code: 401,
            message: message.into(),
            errors: None,
        }
    }

    pub fn forbidden(message: impl Into<String>) -> Self {
        Self {
            code: 403,
            message: message.into(),
            errors: None,
        }
    }

    pub fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, Json(self)).into_response()
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        self.into_response()
    }
}

/// Helper functions for common responses
pub mod helpers {
    use super::*;
    use axum::response::Response;

    /// 200 OK with data
    pub fn ok<T: Serialize>(data: T) -> Response {
        ApiResponse::success(data).into_response()
    }

    /// 201 Created with data
    pub fn created<T: Serialize>(data: T) -> Response {
        ApiResponse::created(data).into_response()
    }

    /// 200 OK with list and pagination
    pub fn list<T: Serialize>(data: T, pagination: Pagination) -> Response {
        ApiResponse::list(data, pagination).into_response()
    }

    /// 204 No Content
    pub fn no_content() -> Response {
        StatusCode::NO_CONTENT.into_response()
    }

    /// 400 Bad Request
    pub fn bad_request(message: impl Into<String>) -> Response {
        ErrorResponse::new(400, message).into_response()
    }

    /// 401 Unauthorized
    pub fn unauthorized(message: impl Into<String>) -> Response {
        ErrorResponse::unauthorized(message).into_response()
    }

    /// 403 Forbidden
    pub fn forbidden(message: impl Into<String>) -> Response {
        ErrorResponse::forbidden(message).into_response()
    }

    /// 404 Not Found
    pub fn not_found(resource: impl Into<String>) -> Response {
        ErrorResponse::not_found(resource).into_response()
    }

    /// 422 Validation Error
    pub fn validation_error(errors: serde_json::Value) -> Response {
        ErrorResponse::validation(errors).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_calculation() {
        let p = Pagination::new(1, 20, 100);
        assert_eq!(p.total_pages, 5);

        let p = Pagination::new(1, 20, 95);
        assert_eq!(p.total_pages, 5);

        let p = Pagination::new(1, 20, 0);
        assert_eq!(p.total_pages, 1);
    }

    #[test]
    fn test_pagination_from_limit_offset() {
        let p = Pagination::from_limit_offset(20, 0, 100);
        assert_eq!(p.page, 1);
        assert_eq!(p.per_page, 20);

        let p = Pagination::from_limit_offset(20, 40, 100);
        assert_eq!(p.page, 3);
    }
}
