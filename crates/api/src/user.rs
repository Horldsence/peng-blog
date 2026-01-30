//! User Management API Routes
//!
//! This module provides HTTP handlers for user management operations.
//! All routes that require authentication use the `Claims` extractor.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
    Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    error::ApiError,
    middleware::auth::Claims,
    state::AppState,
    PostRepository,
    UserRepository,
    SessionRepository,
    FileRepository,
    CommentRepository,
    StatsRepository,
    CategoryRepository,
    TagRepository,
};
use domain::{USER_MANAGE, check_permission, check_ownership_or_admin};

#[derive(Debug, Deserialize)]
pub struct ListUsersQuery {
    #[serde(default = "default_limit")]
    limit: u64,
}

fn default_limit() -> u64 {
    50
}

#[derive(Debug, Deserialize)]
pub struct ListPostsQuery {
    #[serde(default = "default_post_limit")]
    limit: u64,
}

fn default_post_limit() -> u64 {
    20
}

#[derive(Debug, Deserialize)]
pub struct UpdatePermissionsRequest {
    permissions: u64,
}

// ============================================================================
// Routes
// ============================================================================

pub fn routes<PR, UR, SR, FR, CR, STR, CTR, TR>() -> Router<AppState<PR, UR, SR, FR, CR, STR, CTR, TR>>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
    CTR: CategoryRepository + Send + Sync + 'static + Clone,
    TR: TagRepository + Send + Sync + 'static + Clone,
{
    Router::new()
        // GET /api/users - List all users (admin only)
        .route("/", axum::routing::get(list_users))
        // GET /api/users/{id} - Get user info (self or admin)
        .route("/{id}", axum::routing::get(get_user))
        // GET /api/users/{id}/posts - Get user's posts (public)
        .route("/{id}/posts", axum::routing::get(list_user_posts))
        // PATCH /api/users/{id}/permissions - Update user permissions (admin only)
        .route(
            "/{id}/permissions",
            axum::routing::patch(update_permissions),
        )
}

// ============================================================================
// Handlers
// ============================================================================

/// GET /api/users
/// List all users (admin only)
async fn list_users<PR, UR, SR, FR, CR, STR, CTR, TR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR, CTR, TR>>,
    user: Claims,
    Query(params): Query<ListUsersQuery>,
) -> Result<impl IntoResponse, ApiError>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
    CTR: CategoryRepository + Send + Sync + 'static + Clone,
    TR: TagRepository + Send + Sync + 'static + Clone,
{
    check_permission(user.permissions, USER_MANAGE)?;
    
    let users = state
        .user_service
        .list(user.permissions, Some(params.limit))
        .await
        .map_err(ApiError::Domain)?;
    
    Ok((StatusCode::OK, Json(users)))
}

/// GET /api/users/:id
/// Get user info (self or admin)
async fn get_user<PR, UR, SR, FR, CR, STR, CTR, TR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR, CTR, TR>>,
    user: Claims,
    Path(user_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
    CTR: CategoryRepository + Send + Sync + 'static + Clone,
    TR: TagRepository + Send + Sync + 'static + Clone,
{
    let requester_id = uuid::Uuid::parse_str(&user.sub)
        .map_err(|e| ApiError::Internal(format!("Invalid user ID: {}", e)))?;
    
    check_ownership_or_admin(user_id, requester_id, user.permissions, USER_MANAGE)?;
    
    let target_user = state
        .user_service
        .get(user_id)
        .await
        .map_err(ApiError::Domain)?;
    
    Ok((StatusCode::OK, Json(target_user)))
}

/// PATCH /api/users/:id/permissions
/// Update user permissions (admin only)
async fn update_permissions<PR, UR, SR, FR, CR, STR, CTR, TR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR, CTR, TR>>,
    user: Claims,
    Path(user_id): Path<Uuid>,
    Json(input): Json<UpdatePermissionsRequest>,
) -> Result<impl IntoResponse, ApiError>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
    CTR: CategoryRepository + Send + Sync + 'static + Clone,
    TR: TagRepository + Send + Sync + 'static + Clone,
{
    check_permission(user.permissions, USER_MANAGE)?;
    
    let requester_id = uuid::Uuid::parse_str(&user.sub)
        .map_err(|e| ApiError::Internal(format!("Invalid user ID: {}", e)))?;
    
    let updated_user = state
        .user_service
        .update_permissions(
            requester_id,
            user.permissions,
            user_id,
            input.permissions,
        )
        .await
        .map_err(ApiError::Domain)?;
    
    Ok((StatusCode::OK, Json(updated_user)))
}

/// GET /api/users/:id/posts
/// Get public posts by a specific user (public endpoint)
async fn list_user_posts<PR, UR, SR, FR, CR, STR, CTR, TR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR, CTR, TR>>,
    Path(user_id): Path<Uuid>,
    Query(params): Query<ListPostsQuery>,
) -> Result<impl IntoResponse, ApiError>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
    CTR: CategoryRepository + Send + Sync + 'static + Clone,
    TR: TagRepository + Send + Sync + 'static + Clone,
{
    let posts = state
        .post_service
        .list_by_user(user_id, Some(params.limit))
        .await
        .map_err(ApiError::Domain)?;
    
    Ok((StatusCode::OK, Json(posts)))
}