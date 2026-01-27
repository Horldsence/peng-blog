use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
    Router,
};
use domain::post::{CreatePost, UpdatePost};

use crate::error::ApiResult;
use crate::middleware::auth::{Claims, require_permission};
use domain::POST_CREATE;
use serde::Deserialize;
use uuid::Uuid;

use crate::{state::AppState, auth, PostRepository, UserRepository};

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    #[serde(default = "default_limit")]
    limit: u64,
}

fn default_limit() -> u64 {
    20
}

// ============================================================================
// Routes
// ============================================================================

pub fn routes<PR, UR>() -> Router<AppState<PR, UR>>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
{
    Router::new()
        // Public routes - no authentication required
        .route("/auth/register", axum::routing::post(auth::register))
        .route("/auth/login", axum::routing::post(auth::login))
        .route("/posts", axum::routing::get(list_posts))
        .route("/posts/:id", axum::routing::get(get_post))
        // Protected routes - require authentication via Claims extractor
        .route("/auth/me", axum::routing::get(auth::me))
        .route("/posts", axum::routing::post(create_post))
        .route("/posts/:id", axum::routing::put(update_post))
        .route("/posts/:id", axum::routing::delete(delete_post))
        .route("/posts/:id/publish", axum::routing::post(publish_post))
        .route("/posts/:id/unpublish", axum::routing::post(unpublish_post))
}

async fn list_posts<PR, UR>(
    State(state): State<AppState<PR, UR>>,
    Query(params): Query<ListQuery>,
) -> ApiResult<impl IntoResponse>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
{
    let posts = state
        .post_service
        .list_published(Some(params.limit))
        .await
        .map_err(|e| crate::error::ApiError::Domain(e))?;
    Ok((StatusCode::OK, Json(posts)))
}

async fn create_post<PR, UR>(
    State(state): State<AppState<PR, UR>>,
    user: Claims,
    Json(input): Json<CreatePost>,
) -> ApiResult<impl IntoResponse>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
{
    require_permission(&user, POST_CREATE)?;
    let user_id = uuid::Uuid::parse_str(&user.sub)
        .map_err(|e| crate::error::ApiError::Internal(format!("Invalid user ID: {}", e)))?;
    let post = state
        .post_service
        .create(user_id, input.title, input.content)
        .await
        .map_err(|e| crate::error::ApiError::Domain(e))?;

    Ok((StatusCode::CREATED, Json(post)))
}

async fn get_post<PR, UR>(
    State(state): State<AppState<PR, UR>>,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
{
    let post = state
        .post_service
        .get(id)
        .await
        .map_err(|e| crate::error::ApiError::Domain(e))?;
    Ok((StatusCode::OK, Json(post)))
}

async fn update_post<PR, UR>(
    State(state): State<AppState<PR, UR>>,
    user: Claims,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdatePost>,
) -> ApiResult<impl IntoResponse>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
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
        .map_err(|e| crate::error::ApiError::Domain(e))?;

    Ok((StatusCode::OK, Json(post)))
}

async fn delete_post<PR, UR>(
    State(state): State<AppState<PR, UR>>,
    user: Claims,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
{
    let user_id = uuid::Uuid::parse_str(&user.sub)
        .map_err(|e| crate::error::ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    state
        .post_service
        .delete(id, user_id, user.permissions)
        .await
        .map_err(|e| crate::error::ApiError::Domain(e))?;

    Ok((StatusCode::NO_CONTENT, Json(serde_json::json!({}))))
}

async fn publish_post<PR, UR>(
    State(state): State<AppState<PR, UR>>,
    user: Claims,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
{
    let user_id = uuid::Uuid::parse_str(&user.sub)
        .map_err(|e| crate::error::ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    let post = state
        .post_service
        .publish(id, user_id, user.permissions)
        .await
        .map_err(|e| crate::error::ApiError::Domain(e))?;

    Ok((StatusCode::OK, Json(post)))
}

async fn unpublish_post<PR, UR>(
    State(state): State<AppState<PR, UR>>,
    user: Claims,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse>
where
    PR: PostRepository + Send + Sync + 'static + Clone,
    UR: UserRepository + Send + Sync + 'static + Clone,
{
    let user_id = uuid::Uuid::parse_str(&user.sub)
        .map_err(|e| crate::error::ApiError::Internal(format!("Invalid user ID: {}", e)))?;

    let post = state
        .post_service
        .unpublish(id, user_id, user.permissions)
        .await
        .map_err(|e| crate::error::ApiError::Domain(e))?;

    Ok((StatusCode::OK, Json(post)))
}