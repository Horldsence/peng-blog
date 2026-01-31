//! User Management API Routes
//!
//! This module provides HTTP handlers for user management operations.
//!
//! ## Endpoints
//!
//! | Method | Endpoint | Description |
//! |--------|----------|-------------|
//! | GET | /users | List all users (admin) |
//! | GET | /users/{id} | Get user info |
//! | PATCH | /users/{id} | Update user (self/admin) |
//! | DELETE | /users/{id} | Delete user (self/admin) |
//! | GET | /users/{id}/posts | Get user's posts |

use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json, Router,
};
use domain::{check_ownership_or_admin, USER_MANAGE};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    error::ApiError,
    middleware::auth::Claims,
    response::{helpers as resp, Pagination},
    state::AppState,
};

/// Query parameters for listing users
#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    #[serde(default = "default_page")]
    page: u64,
    #[serde(default = "default_per_page")]
    per_page: u64,
}

/// Query parameters for listing user's posts
#[derive(Debug, Deserialize)]
pub struct ListPostsQuery {
    #[serde(default = "default_page")]
    page: u64,
    #[serde(default = "default_per_page")]
    per_page: u64,
    /// Include draft posts (only for self or admin)
    #[serde(default)]
    include: Option<String>,
}

/// Request body for updating user permissions
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    permissions: Option<u64>,
}

fn default_page() -> u64 {
    1
}

fn default_per_page() -> u64 {
    50
}

/// Create user routes
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", axum::routing::get(list_users))
        .route("/{id}", axum::routing::get(get_user))
        .route("/{id}", axum::routing::patch(update_user))
        .route("/{id}", axum::routing::delete(delete_user))
        .route("/{id}/posts", axum::routing::get(list_user_posts))
}

/// GET /users
/// List all users (admin only)
async fn list_users(
    State(state): State<AppState>,
    user: Claims,
    Query(params): Query<ListUsersQuery>,
) -> Result<impl IntoResponse, ApiError> {
    domain::check_permission(user.permissions, USER_MANAGE)
        .map_err(|e| ApiError::Unauthorized(e.to_string()))?;

    let users = state
        .user_service
        .list(user.permissions, Some(params.per_page))
        .await
        .map_err(ApiError::Domain)?;

    // TODO: Get total count for pagination
    let total = users.len() as u64;
    let pagination = Pagination::new(params.page, params.per_page, total);

    Ok(resp::list(users, pagination))
}

/// GET /users/{id}
/// Get user info (self or admin)
async fn get_user(
    State(state): State<AppState>,
    user: Claims,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let requester_id = Uuid::parse_str(&user.sub)
        .map_err(|e| ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    check_ownership_or_admin(user_id, requester_id, user.permissions, USER_MANAGE)
        .map_err(|e| ApiError::Unauthorized(e.to_string()))?;

    let target_user = state
        .user_service
        .get(user_id)
        .await
        .map_err(ApiError::Domain)?;

    Ok(resp::ok(target_user))
}

/// PATCH /users/{id}
/// Update user (permissions - admin only)
async fn update_user(
    State(state): State<AppState>,
    user: Claims,
    Path(user_id): Path<Uuid>,
    Json(input): Json<UpdateUserRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let requester_id = Uuid::parse_str(&user.sub)
        .map_err(|e| ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    // Only admin can update permissions
    if let Some(permissions) = input.permissions {
        domain::check_permission(user.permissions, USER_MANAGE)
            .map_err(|e| ApiError::Unauthorized(e.to_string()))?;

        let updated_user = state
            .user_service
            .update_permissions(requester_id, user.permissions, user_id, permissions)
            .await
            .map_err(ApiError::Domain)?;

        return Ok(resp::ok(updated_user));
    }

    // For now, only permissions updates are supported via PATCH
    // Other user profile updates can be added here later

    Err(ApiError::Validation("No valid fields to update".to_string()))
}

/// DELETE /users/{id}
/// Delete a user (self or admin)
async fn delete_user(
    State(state): State<AppState>,
    user: Claims,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let requester_id = Uuid::parse_str(&user.sub)
        .map_err(|e| ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    state
        .user_service
        .delete(user_id, requester_id, user.permissions)
        .await
        .map_err(ApiError::Domain)?;

    Ok(resp::no_content())
}

/// GET /users/{id}/posts
/// Get posts by a specific user
async fn list_user_posts(
    State(state): State<AppState>,
    user: Option<Claims>,
    Path(user_id): Path<Uuid>,
    Query(params): Query<ListPostsQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let include_drafts = params.include.as_ref().map_or(false, |v| v == "drafts");

    // Check permissions for viewing drafts
    let can_view_drafts = if include_drafts {
        match &user {
            Some(current_user) => {
                let current_user_id = Uuid::parse_str(&current_user.sub)
                    .map_err(|e| ApiError::Internal(format!("Invalid user ID: {}", e)))?;
                let is_admin = (current_user.permissions & USER_MANAGE) != 0;
                current_user_id == user_id || is_admin
            }
            None => false,
        }
    } else {
        false
    };

    let posts = if can_view_drafts {
        state
            .post_service
            .list_by_user(user_id, Some(params.per_page))
            .await
            .map_err(ApiError::Domain)?
    } else {
        state
            .post_service
            .list_published_by_user(user_id, Some(params.per_page))
            .await
            .map_err(ApiError::Domain)?
    };

    // TODO: Get total count for pagination
    let total = posts.len() as u64;
    let pagination = Pagination::new(params.page, params.per_page, total);

    Ok(resp::list(posts, pagination))
}
