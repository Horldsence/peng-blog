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
use domain::POST_CREATE;
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
        let user_id = Uuid::parse_str(&user_id_str)
            .map_err(|e| crate::error::ApiError::Validation(format!("Invalid user ID: {}", e)))?;
        state
            .post_service
            .list_by_user(user_id, Some(params.limit))
            .await
            .map_err(crate::error::ApiError::Domain)?
    } else {
        state
            .post_service
            .list_published(Some(params.limit))
            .await
            .map_err(crate::error::ApiError::Domain)?
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
    Ok((StatusCode::OK, Json(post)))
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