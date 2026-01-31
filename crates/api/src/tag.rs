//! Tag API Routes
//!
//! This module provides HTTP handlers for tag management.
//! Tags are managed by admins and can be associated with posts.
//!
//! ## Endpoints
//!
//! | Method | Endpoint | Description |
//! |--------|----------|-------------|
//! | GET | /tags | List all tags |
//! | POST | /tags | Create tag (admin) |
//! | GET | /tags/{id} | Get tag details |
//! | GET | /tags/{id}/posts | Get posts with tag |
//! | DELETE | /tags/{id} | Delete tag (admin) |

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json, Router,
};
use domain::{CreateTag, USER_MANAGE};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    error::ApiError,
    middleware::auth::Claims,
    response::{helpers as resp, Pagination},
    state::AppState,
};

/// Query parameters for listing tags
#[derive(Debug, Deserialize)]
pub struct ListTagsQuery {
    /// Page number (1-based)
    #[serde(default = "default_page")]
    page: u64,
    /// Items per page
    #[serde(default = "default_per_page")]
    per_page: u64,
}

/// Query parameters for listing posts with a tag
#[derive(Debug, Deserialize)]
pub struct ListPostsQuery {
    #[serde(default = "default_page")]
    page: u64,
    #[serde(default = "default_per_page")]
    per_page: u64,
}

fn default_page() -> u64 {
    1
}

fn default_per_page() -> u64 {
    50
}

/// Create tag routes
pub fn routes() -> Router<AppState> {
    Router::new()
        // Public routes
        .route("/", axum::routing::get(list_tags))
        .route("/{id}", axum::routing::get(get_tag))
        .route("/{id}/posts", axum::routing::get(list_tag_posts))
        // Admin routes
        .route("/", axum::routing::post(create_tag))
        .route("/{id}", axum::routing::delete(delete_tag))
}

/// GET /tags
/// List all tags
async fn list_tags(
    State(state): State<AppState>,
    Query(params): Query<ListTagsQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let tags = state
        .tag_service
        .list()
        .await
        .map_err(ApiError::Domain)?;

    // TODO: Implement proper pagination in service layer
    let total = tags.len() as u64;
    let pagination = Pagination::new(params.page, params.per_page, total);

    Ok(resp::list(tags, pagination))
}

/// POST /tags
/// Create a new tag (admin only)
async fn create_tag(
    State(state): State<AppState>,
    user: Claims,
    Json(input): Json<CreateTag>,
) -> Result<impl IntoResponse, ApiError> {
    domain::check_permission(user.permissions, USER_MANAGE)
        .map_err(|e| ApiError::Unauthorized(e.to_string()))?;

    let tag = state
        .tag_service
        .create(input)
        .await
        .map_err(ApiError::Domain)?;

    Ok(resp::created(tag))
}

/// GET /tags/{id}
/// Get tag details
async fn get_tag(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let tag = state
        .tag_service
        .get(id)
        .await
        .map_err(ApiError::Domain)?;

    Ok(resp::ok(tag))
}

/// GET /tags/{id}/posts
/// Get posts with a specific tag
async fn list_tag_posts(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(params): Query<ListPostsQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let posts = state
        .post_service
        .list_by_tag(id, Some(params.per_page))
        .await
        .map_err(ApiError::Domain)?;

    // TODO: Get total count for pagination
    let total = posts.len() as u64;
    let pagination = Pagination::new(params.page, params.per_page, total);

    Ok(resp::list(posts, pagination))
}

/// DELETE /tags/{id}
/// Delete a tag (admin only)
async fn delete_tag(
    State(state): State<AppState>,
    user: Claims,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    domain::check_permission(user.permissions, USER_MANAGE)
        .map_err(|e| ApiError::Unauthorized(e.to_string()))?;

    state
        .tag_service
        .delete(id)
        .await
        .map_err(ApiError::Domain)?;

    Ok(resp::no_content())
}
