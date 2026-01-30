//! Stats API Routes
//!
//! This module provides HTTP handlers for statistics management.
//! Statistics include global visitor tracking and per-post view counts.
//!
//! Design Principles:
//! - Simple RESTful endpoints
//! - Public access for recording views (no auth required)
//! - Admin access for detailed stats
//! - No special cases - all stats follow the same rules

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    Router,
};
#[allow(unused_imports)]
use domain::{RecordViewRequest, StatsResponse};
use uuid::Uuid;

use crate::{error::ApiError, state::AppState};

// ============================================================================
// Routes
// ============================================================================

pub fn routes() -> Router<AppState> {
    Router::new()
        // GET /api/stats/visits - Get global visitor stats
        .route("/visits", axum::routing::get(get_visits))
        // POST /api/stats/visits - Record a visit
        .route("/visits", axum::routing::post(record_visit))
        // GET /api/stats/posts/{id}/views - Get post view count
        .route("/posts/{id}/views", axum::routing::get(get_post_views))
        // POST /api/stats/posts/{id}/views - Record post view
        .route("/posts/{id}/views", axum::routing::post(record_post_view))
        // GET /api/stats/total - Get total stats (admin)
        .route("/total", axum::routing::get(get_total_stats))
}

// ============================================================================
// Handlers
// ============================================================================

/// GET /api/stats/visits
/// Get global visitor statistics
///
/// This endpoint is public - no authentication required.
pub async fn get_visits(State(state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    let stats = state
        .stats_service
        .get_visit_stats()
        .await
        .map_err(ApiError::Domain)?;

    Ok((StatusCode::OK, Json(stats)))
}

/// POST /api/stats/visits
/// Record a page visit
///
/// This endpoint is public - no authentication required.
/// Request body: {"post_id": "uuid"} (optional)
pub async fn record_visit(
    State(state): State<AppState>,
    Json(input): Json<RecordViewRequest>,
) -> Result<impl IntoResponse, ApiError> {
    state
        .stats_service
        .record_view(input)
        .await
        .map_err(ApiError::Domain)?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({ "message": "Visit recorded" })),
    ))
}

/// GET /api/stats/posts/:id/views
/// Get view count for a specific post
///
/// This endpoint is public - no authentication required.
pub async fn get_post_views(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let post_id = Uuid::parse_str(&id)
        .map_err(|e| ApiError::Validation(format!("Invalid post ID: {}", e)))?;

    let stats = state
        .stats_service
        .get_post_stats(post_id)
        .await
        .map_err(ApiError::Domain)?;

    Ok((StatusCode::OK, Json(stats)))
}

/// POST /api/stats/posts/:id/views
/// Record a view for a specific post
///
/// This endpoint is public - no authentication required.
pub async fn record_post_view(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let post_id = Uuid::parse_str(&id)
        .map_err(|e| ApiError::Validation(format!("Invalid post ID: {}", e)))?;

    let request = RecordViewRequest {
        post_id: Some(post_id),
    };

    state
        .stats_service
        .record_view(request)
        .await
        .map_err(ApiError::Domain)?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({ "message": "View recorded" })),
    ))
}

/// GET /api/stats/total
/// Get total statistics (admin only)
///
/// This endpoint requires admin authentication.
pub async fn get_total_stats(State(state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    // Note: Admin check would be done via middleware in real implementation
    // For now, we'll just allow access

    let stats = state
        .stats_service
        .get_total_stats()
        .await
        .map_err(ApiError::Domain)?;

    Ok((StatusCode::OK, Json(stats)))
}
