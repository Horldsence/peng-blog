use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use domain::post::{CreatePost, UpdatePost};

use crate::error::ApiResult;
use crate::middleware::auth::Claims;
use domain::{POST_CREATE, USER_MANAGE};
use serde::Deserialize;
use uuid::Uuid;

use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    #[serde(default = "default_limit")]
    limit: u64,
    user_id: Option<String>,
    category_id: Option<String>,
    tag_id: Option<String>,
}

fn default_limit() -> u64 {
    20
}

// ============================================================================
// Routes
// ============================================================================

pub fn routes() -> Router<AppState> {
    Router::new()
        // Public routes - no authentication required
        .route("/", axum::routing::get(list_posts))
        .route("/{id}", axum::routing::get(get_post))
        .route("/{id}/tags", axum::routing::get(get_post_tags))
        // Protected routes - require authentication via Claims extractor
        .route("/", axum::routing::post(create_post))
        .route("/{id}", axum::routing::put(update_post))
        .route("/{id}", axum::routing::delete(delete_post))
        .route("/{id}/publish", axum::routing::post(publish_post))
        .route("/{id}/unpublish", axum::routing::post(unpublish_post))
        .route("/{id}/category", axum::routing::put(set_post_category))
        .route("/{id}/tags/:tag_id", axum::routing::post(add_post_tag))
        .route("/{id}/tags/:tag_id", axum::routing::delete(remove_post_tag))
}

async fn list_posts(
    State(state): State<AppState>,
    user: Option<Claims>,
    Query(params): Query<ListQuery>,
) -> ApiResult<impl IntoResponse> {
    let posts = if let Some(user_id_str) = params.user_id {
        let target_user_id = Uuid::parse_str(&user_id_str)
            .map_err(|e| crate::error::ApiError::Validation(format!("Invalid user ID: {}", e)))?;

        // Check if current user is the owner or admin
        if let Some(current_user) = user {
            let current_user_id = uuid::Uuid::parse_str(&current_user.sub)
                .map_err(|e| crate::error::ApiError::Internal(format!("Invalid user ID: {}", e)))?;

            // Owner or admin can see all posts (including unpublished)
            if current_user_id == target_user_id || (current_user.permissions & USER_MANAGE) != 0 {
                state
                    .post_service
                    .list_by_user(target_user_id, Some(params.limit))
                    .await
                    .map_err(crate::error::ApiError::Domain)?
            } else {
                // Non-owner, non-admin can only see published posts
                state
                    .post_service
                    .list_published_by_user(target_user_id, Some(params.limit))
                    .await
                    .map_err(crate::error::ApiError::Domain)?
            }
        } else {
            // No user logged in - only show published posts
            state
                .post_service
                .list_published_by_user(target_user_id, Some(params.limit))
                .await
                .map_err(crate::error::ApiError::Domain)?
        }
    } else if let Some(category_id_str) = params.category_id {
        let category_id = Uuid::parse_str(&category_id_str).map_err(|e| {
            crate::error::ApiError::Validation(format!("Invalid category ID: {}", e))
        })?;

        if let Some(_tag_id_str) = params.tag_id {
            // Both category and tag specified - AND logic
            let tag_id = Uuid::parse_str(&_tag_id_str).map_err(|e| {
                crate::error::ApiError::Validation(format!("Invalid tag ID: {}", e))
            })?;

            // Get posts by category first
            let category_posts = state
                .post_service
                .list_by_category(category_id, None)
                .await
                .map_err(crate::error::ApiError::Domain)?;

            // Get posts by tag
            let tag_posts = state
                .post_service
                .list_by_tag(tag_id, None)
                .await
                .map_err(crate::error::ApiError::Domain)?;

            // Filter posts that are in both lists (AND logic)
            let category_post_ids: std::collections::HashSet<Uuid> =
                category_posts.iter().map(|p| p.id).collect();
            let mut and_posts: Vec<_> = tag_posts
                .into_iter()
                .filter(|p| category_post_ids.contains(&p.id))
                .collect();

            // Apply limit
            and_posts.truncate(params.limit as usize);
            and_posts
        } else {
            // Only category specified
            state
                .post_service
                .list_by_category(category_id, Some(params.limit))
                .await
                .map_err(crate::error::ApiError::Domain)?
        }
    } else if let Some(tag_id_str) = params.tag_id {
        let tag_id = Uuid::parse_str(&tag_id_str)
            .map_err(|e| crate::error::ApiError::Validation(format!("Invalid tag ID: {}", e)))?;

        // Only tag specified
        state
            .post_service
            .list_by_tag(tag_id, Some(params.limit))
            .await
            .map_err(crate::error::ApiError::Domain)?
    } else {
        // No user_id specified - only show published posts (unless admin)
        if let Some(current_user) = user {
            if (current_user.permissions & domain::USER_MANAGE) != 0 {
                // Admin can see all posts
                state
                    .post_service
                    .list_all(Some(params.limit))
                    .await
                    .map_err(crate::error::ApiError::Domain)?
            } else {
                // Non-admin can only see published posts
                state
                    .post_service
                    .list_published(Some(params.limit))
                    .await
                    .map_err(crate::error::ApiError::Domain)?
            }
        } else {
            // No user logged in - only show published posts
            state
                .post_service
                .list_published(Some(params.limit))
                .await
                .map_err(crate::error::ApiError::Domain)?
        }
    };
    Ok((StatusCode::OK, Json(posts)))
}

async fn create_post(
    State(state): State<AppState>,
    user: Claims,
    Json(input): Json<CreatePost>,
) -> ApiResult<impl IntoResponse> {
    domain::check_permission(user.permissions, POST_CREATE)?;
    let user_id = uuid::Uuid::parse_str(&user.sub)
        .map_err(|e| crate::error::ApiError::Internal(format!("Invalid user ID: {}", e)))?;
    let post = state
        .post_service
        .create(user_id, input.title, input.content)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::CREATED, Json(post)))
}

async fn get_post(
    State(state): State<AppState>,
    user: Option<Claims>,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse> {
    let post = state
        .post_service
        .get(id)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    if post.is_published() {
        Ok((StatusCode::OK, Json(post)))
    } else {
        match user {
            Some(current_user) => {
                let current_user_id = uuid::Uuid::parse_str(&current_user.sub).map_err(|e| {
                    crate::error::ApiError::Internal(format!("Invalid user ID: {}", e))
                })?;

                if current_user_id == post.user_id || (current_user.permissions & USER_MANAGE) != 0
                {
                    Ok((StatusCode::OK, Json(post)))
                } else {
                    Err(crate::error::ApiError::NotFound(
                        "Post not found".to_string(),
                    ))
                }
            }
            None => Err(crate::error::ApiError::NotFound(
                "Post not found".to_string(),
            )),
        }
    }
}

async fn get_post_tags(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse> {
    let tags = state
        .post_service
        .get_tags(id)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::OK, Json(tags)))
}

async fn set_post_category(
    State(state): State<AppState>,
    user: Claims,
    Path(id): Path<Uuid>,
    Json(input): Json<serde_json::Value>,
) -> ApiResult<impl IntoResponse> {
    let user_id = uuid::Uuid::parse_str(&user.sub)
        .map_err(|e| crate::error::ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    let category_id: Option<Uuid> = input
        .get("category_id")
        .and_then(|v| v.as_str())
        .map(|s| Uuid::parse_str(s))
        .transpose()
        .map_err(|e| crate::error::ApiError::Validation(format!("Invalid category_id: {}", e)))?;

    state
        .post_service
        .set_category(id, category_id, user_id, user.permissions)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::OK, Json(serde_json::json!({"success": true}))))
}

async fn add_post_tag(
    State(state): State<AppState>,
    user: Claims,
    Path((id, tag_id)): Path<(Uuid, Uuid)>,
) -> ApiResult<impl IntoResponse> {
    let user_id = uuid::Uuid::parse_str(&user.sub)
        .map_err(|e| crate::error::ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    state
        .post_service
        .add_tag(id, tag_id, user_id, user.permissions)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((
        StatusCode::CREATED,
        Json(serde_json::json!({"success": true})),
    ))
}

async fn remove_post_tag(
    State(state): State<AppState>,
    user: Claims,
    Path((id, tag_id)): Path<(Uuid, Uuid)>,
) -> ApiResult<impl IntoResponse> {
    let user_id = uuid::Uuid::parse_str(&user.sub)
        .map_err(|e| crate::error::ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    state
        .post_service
        .remove_tag(id, tag_id, user_id, user.permissions)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::OK, Json(serde_json::json!({"success": true}))))
}

async fn update_post(
    State(state): State<AppState>,
    user: Claims,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdatePost>,
) -> ApiResult<impl IntoResponse> {
    let user_id = uuid::Uuid::parse_str(&user.sub)
        .map_err(|e| crate::error::ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    let post = state
        .post_service
        .update(id, input.title, input.content, user_id, user.permissions)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::OK, Json(post)))
}

async fn delete_post(
    State(state): State<AppState>,
    user: Claims,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse> {
    let user_id = uuid::Uuid::parse_str(&user.sub)
        .map_err(|e| crate::error::ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    state
        .post_service
        .delete(id, user_id, user.permissions)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::NO_CONTENT, Json(serde_json::json!({}))))
}

async fn publish_post(
    State(state): State<AppState>,
    user: Claims,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse> {
    let user_id = uuid::Uuid::parse_str(&user.sub)
        .map_err(|e| crate::error::ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    let post = state
        .post_service
        .publish(id, user_id, user.permissions)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::OK, Json(post)))
}

async fn unpublish_post(
    State(state): State<AppState>,
    user: Claims,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse> {
    let user_id = uuid::Uuid::parse_str(&user.sub)
        .map_err(|e| crate::error::ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    let post = state
        .post_service
        .unpublish(id, user_id, user.permissions)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::OK, Json(post)))
}
