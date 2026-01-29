use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
    Router,
};
use domain::post::{CreatePost, UpdatePost};

use crate::error::ApiResult;
use crate::middleware::auth::Claims;
use domain::{POST_CREATE, USER_MANAGE};
use serde::Deserialize;
use uuid::Uuid;

use crate::{state::AppState, PostRepository, UserRepository, SessionRepository, FileRepository, CommentRepository, StatsRepository};

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    #[serde(default = "default_limit")]
    limit: u64,
    user_id: Option<String>,
}

fn default_limit() -> u64 {
    20
}


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
        // Public routes - no authentication required
        .route("/", axum::routing::get(list_posts))
        .route("/{id}", axum::routing::get(get_post))
        // Protected routes - require authentication via Claims extractor
        .route("/", axum::routing::post(create_post))
        .route("/{id}", axum::routing::put(update_post))
        .route("/{id}", axum::routing::delete(delete_post))
        .route("/{id}/publish", axum::routing::post(publish_post))
        .route("/{id}/unpublish", axum::routing::post(unpublish_post))
}

async fn list_posts<PR, UR, SR, FR, CR, STR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR>>,
    user: Option<Claims>,
    Query(params): Query<ListQuery>,
) -> ApiResult<impl IntoResponse>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
    let posts = if let Some(user_id_str) = params.user_id {
        let target_user_id = Uuid::parse_str(&user_id_str)
            .map_err(|e| crate::error::ApiError::Validation(format!("Invalid user ID: {}", e)))?;
        
        // Check if current user is the owner or admin
        if let Some(current_user) = user {
            let current_user_id = uuid::Uuid::parse_str(&current_user.sub)
                .map_err(|e| crate::error::ApiError::Internal(format!("Invalid user ID: {}", e)))?;
            
            // Owner or admin can see all posts (including unpublished)
            if current_user_id == target_user_id 
                || (current_user.permissions & USER_MANAGE) != 0 {
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

async fn create_post<PR, UR, SR, FR, CR, STR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR>>,
    user: Claims,
    Json(input): Json<CreatePost>,
) -> ApiResult<impl IntoResponse>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
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

async fn get_post<PR, UR, SR, FR, CR, STR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR>>,
    user: Option<Claims>,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
    let post = state
        .post_service
        .get(id)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    // Check if post is published
    if post.is_published() {
        Ok((StatusCode::OK, Json(post)))
    } else {
        // Post is unpublished - check access permissions
        match user {
            Some(current_user) => {
                let current_user_id = uuid::Uuid::parse_str(&current_user.sub)
                    .map_err(|e| crate::error::ApiError::Internal(format!("Invalid user ID: {}", e)))?;
                
                // Allow if user is the owner or has admin privileges
                if current_user_id == post.user_id 
                    || (current_user.permissions & USER_MANAGE) != 0 {
                    Ok((StatusCode::OK, Json(post)))
                } else {
                    Err(crate::error::ApiError::NotFound("Post not found".to_string()))
                }
            }
            None => {
                // No user logged in - cannot access unpublished posts
                Err(crate::error::ApiError::NotFound("Post not found".to_string()))
            }
        }
    }
}

async fn update_post<PR, UR, SR, FR, CR, STR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR>>,
    user: Claims,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdatePost>,
) -> ApiResult<impl IntoResponse>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
    let user_id = uuid::Uuid::parse_str(&user.sub)
        .map_err(|e| crate::error::ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    let post = state
        .post_service
        .update(
            id,
            input.title,
            input.content,
            user_id,
            user.permissions,
        )
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::OK, Json(post)))
}

async fn delete_post<PR, UR, SR, FR, CR, STR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR>>,
    user: Claims,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
    let user_id = uuid::Uuid::parse_str(&user.sub)
        .map_err(|e| crate::error::ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    state
        .post_service
        .delete(id, user_id, user.permissions)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::NO_CONTENT, Json(serde_json::json!({}))))
}

async fn publish_post<PR, UR, SR, FR, CR, STR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR>>,
    user: Claims,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
    let user_id = uuid::Uuid::parse_str(&user.sub)
        .map_err(|e| crate::error::ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    let post = state
        .post_service
        .publish(id, user_id, user.permissions)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::OK, Json(post)))
}

async fn unpublish_post<PR, UR, SR, FR, CR, STR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR>>,
    user: Claims,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
    SR: SessionRepository + Send + Sync + 'static + Clone,
    FR: FileRepository + Send + Sync + 'static + Clone,
    CR: CommentRepository + Send + Sync + 'static + Clone,
    STR: StatsRepository + Send + Sync + 'static + Clone,
{
    let user_id = uuid::Uuid::parse_str(&user.sub)
        .map_err(|e| crate::error::ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    let post = state
        .post_service
        .unpublish(id, user_id, user.permissions)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::OK, Json(post)))
}