//! Category API Routes
//!
//! This module provides HTTP handlers for category management.
//! Categories are managed by admins and used to organize posts.
//!
//! ## Endpoints
//!
//! | Method | Endpoint | Description |
//! |--------|----------|-------------|
//! | GET | /categories | List all categories |
//! | POST | /categories | Create category (admin) |
//! | GET | /categories/{id} | Get category details |
//! | GET | /categories/{id}/posts | Get posts in category |
//! | PATCH | /categories/{id} | Update category (admin) |
//! | DELETE | /categories/{id} | Delete category (admin) |

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json, Router,
};
use domain::{CreateCategory, UpdateCategory, USER_MANAGE};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    error::ApiError,
    middleware::auth::Claims,
    response::{helpers as resp, Pagination},
    state::AppState,
};

/// Query parameters for listing categories
#[derive(Debug, Deserialize)]
pub struct ListCategoriesQuery {
    /// Page number (1-based)
    #[serde(default = "default_page")]
    page: u64,
    /// Items per page
    #[serde(default = "default_per_page")]
    per_page: u64,
}

/// Query parameters for listing posts in a category
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

/// Create category routes
pub fn routes() -> Router<AppState> {
    Router::new()
        // Public routes
        .route("/", axum::routing::get(list_categories))
        .route("/{id}", axum::routing::get(get_category))
        .route("/{id}/posts", axum::routing::get(list_category_posts))
        // Admin routes
        .route("/", axum::routing::post(create_category))
        .route("/{id}", axum::routing::patch(update_category))
        .route("/{id}", axum::routing::delete(delete_category))
}

/// GET /categories
/// List all categories
async fn list_categories(
    State(state): State<AppState>,
    Query(params): Query<ListCategoriesQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let categories = state
        .category_service
        .list()
        .await
        .map_err(ApiError::Domain)?;

    // TODO: Implement proper pagination in service layer
    let total = categories.len() as u64;
    let pagination = Pagination::new(params.page, params.per_page, total);

    Ok(resp::list(categories, pagination))
}

/// POST /categories
/// Create a new category (admin only)
async fn create_category(
    State(state): State<AppState>,
    user: Claims,
    Json(input): Json<CreateCategory>,
) -> Result<impl IntoResponse, ApiError> {
    domain::check_permission(user.permissions, USER_MANAGE)
        .map_err(|e| ApiError::Unauthorized(e.to_string()))?;

    let category = state
        .category_service
        .create(input)
        .await
        .map_err(ApiError::Domain)?;

    Ok(resp::created(category))
}

/// GET /categories/{id}
/// Get category details
async fn get_category(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let category = state
        .category_service
        .get(id)
        .await
        .map_err(ApiError::Domain)?;

    Ok(resp::ok(category))
}

/// GET /categories/{id}/posts
/// Get posts in a category
async fn list_category_posts(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Query(params): Query<ListPostsQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let posts = state
        .post_service
        .list_by_category(id, Some(params.per_page))
        .await
        .map_err(ApiError::Domain)?;

    // TODO: Get total count for pagination
    let total = posts.len() as u64;
    let pagination = Pagination::new(params.page, params.per_page, total);

    Ok(resp::list(posts, pagination))
}

/// PATCH /categories/{id}
/// Update a category (admin only)
async fn update_category(
    State(state): State<AppState>,
    user: Claims,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateCategory>,
) -> Result<impl IntoResponse, ApiError> {
    domain::check_permission(user.permissions, USER_MANAGE)
        .map_err(|e| ApiError::Unauthorized(e.to_string()))?;

    let category = state
        .category_service
        .update(id, input)
        .await
        .map_err(ApiError::Domain)?;

    Ok(resp::ok(category))
}

/// DELETE /categories/{id}
/// Delete a category (admin only)
async fn delete_category(
    State(state): State<AppState>,
    user: Claims,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    domain::check_permission(user.permissions, USER_MANAGE)
        .map_err(|e| ApiError::Unauthorized(e.to_string()))?;

    state
        .category_service
        .delete(id)
        .await
        .map_err(ApiError::Domain)?;

    Ok(resp::no_content())
}
