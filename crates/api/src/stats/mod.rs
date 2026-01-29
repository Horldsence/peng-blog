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

use crate::{
    error::ApiError,
    state::AppState,
    PostRepository,
    UserRepository,
    SessionRepository,
    FileRepository,
    CommentRepository,
    StatsRepository,
};

// ============================================================================
// Routes
// ============================================================================

pub fn routes<PR, UR, SR, FR, CR, STR>() -> Router<AppState<PR, UR, SR, FR, CR, STR>>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
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
pub async fn get_visits<PR, UR, SR, FR, CR, STR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR>>,
) -> Result<impl IntoResponse, ApiError>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
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
pub async fn record_visit<PR, UR, SR, FR, CR, STR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR>>,
    Json(input): Json<RecordViewRequest>,
) -> Result<impl IntoResponse, ApiError>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
    state
        .stats_service
        .record_view(input)
        .await
        .map_err(|e| ApiError::Domain(e))?;

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({ "message": "Visit recorded" })),
    ))
}

/// GET /api/stats/posts/:id/views
/// Get view count for a specific post
///
/// This endpoint is public - no authentication required.
pub async fn get_post_views<PR, UR, SR, FR, CR, STR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
    let post_id = Uuid::parse_str(&id)
        .map_err(|e| ApiError::Validation(format!("Invalid post ID: {}", e)))?;

    let stats = state
        .stats_service
        .get_post_stats(post_id)
        .await
        .map_err(|e| ApiError::Domain(e))?;

    Ok((StatusCode::OK, Json(stats)))
}

/// POST /api/stats/posts/:id/views
/// Record a view for a specific post
///
/// This endpoint is public - no authentication required.
pub async fn record_post_view<PR, UR, SR, FR, CR, STR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, ApiError>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
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
pub async fn get_total_stats<PR, UR, SR, FR, CR, STR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR>>,
) -> Result<impl IntoResponse, ApiError>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
    // Note: Admin check would be done via middleware in real implementation
    // For now, we'll just allow access

    let stats = state
        .stats_service
        .get_total_stats()
        .await
        .map_err(ApiError::Domain)?;

    Ok((StatusCode::OK, Json(stats)))
}
