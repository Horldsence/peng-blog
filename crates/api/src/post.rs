use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
    Router,
};
use domain::post::{CreatePost, UpdatePost};
use infrastructure::PostRepository;

use crate::error::ApiResult;
use crate::middleware::auth::{Claims, require_ownership, require_permission};
use domain::{POST_CREATE, POST_DELETE, POST_PUBLISH, POST_UPDATE};
use serde::Deserialize;
use uuid::Uuid;

use crate::{state::AppState, auth};

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    #[serde(default = "default_limit")]
    limit: u64,
}

fn default_limit() -> u64 {
    20
}

fn validate_post_title(title: &str) -> Result<(), domain::Error> {
    if title.trim().is_empty() {
        return Err(domain::Error::Validation("Title cannot be empty".to_string()));
    }
    if title.len() > 200 {
        return Err(domain::Error::Validation("Title too long (max 200 characters)".to_string()));
    }
    Ok(())
}

fn validate_post_content(content: &str) -> Result<(), domain::Error> {
    if content.trim().is_empty() {
        return Err(domain::Error::Validation("Content cannot be empty".to_string()));
    }
    Ok(())
}

// ============================================================================



pub fn routes() -> Router<AppState> {
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

async fn list_posts(
    State(state): State<AppState>,
    Query(params): Query<ListQuery>,
) -> ApiResult<impl IntoResponse> {
    let posts = state.db
        .list_published_posts(params.limit)
        .await
        .map_err(|e| crate::error::ApiError::Internal(e.to_string()))?;
    Ok((StatusCode::OK, Json(posts)))
}

async fn create_post(
    State(state): State<AppState>,
    user: Claims,
    Json(input): Json<CreatePost>,
) -> ApiResult<impl IntoResponse> {
    require_permission(&user, POST_CREATE)?;
    validate_post_title(&input.title)?;
    validate_post_content(&input.content)?;

    let user_id = uuid::Uuid::parse_str(&user.sub)
        .map_err(|e| crate::error::ApiError::Internal(format!("Invalid user ID: {}", e)))?;
    let post = state.db
        .create_post(user_id, input.title, input.content)
        .await
        .map_err(|e| crate::error::ApiError::Internal(e.to_string()))?;

    Ok((StatusCode::CREATED, Json(post)))
}

async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse> {
    let post = state.db
        .get_post(id)
        .await
        .map_err(|e| crate::error::ApiError::Internal(e.to_string()))?;
    Ok((StatusCode::OK, Json(post)))
}

async fn update_post(
    State(state): State<AppState>,
    user: Claims,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdatePost>,
) -> ApiResult<impl IntoResponse> {
    require_permission(&user, POST_UPDATE)?;

    let post = state.db.get_post(id).await?;
    require_ownership(&user, &post)?;

    let mut post = post.clone();
    
    if let Some(title) = input.title {
        validate_post_title(&title)?;
        post.title = title;
    }

    if let Some(content) = input.content {
        validate_post_content(&content)?;
        post.content = content;
    }

    if let Some(published_at) = input.published_at {
        post.published_at = Some(published_at);
    }

    let post = state.db
        .update_post(post)
        .await
        .map_err(|e| crate::error::ApiError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(post)))
}

async fn delete_post(
    State(state): State<AppState>,
    user: Claims,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse> {
    require_permission(&user, POST_DELETE)?;

    let post = state.db.get_post(id).await?;
    require_ownership(&user, &post)?;

    // TODO: implement actual deletion in repository
    Ok((StatusCode::NO_CONTENT, Json(serde_json::json!({}))))
}

async fn publish_post(
    State(state): State<AppState>,
    user: Claims,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse> {
    require_permission(&user, POST_PUBLISH)?;

    let mut post = state.db.get_post(id).await?;
    require_ownership(&user, &post)?;

    post.publish();
    let post = state.db
        .update_post(post)
        .await
        .map_err(|e| crate::error::ApiError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(post)))
}

async fn unpublish_post(
    State(state): State<AppState>,
    user: Claims,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse> {
    require_permission(&user, POST_PUBLISH)?;

    let mut post = state.db.get_post(id).await?;
    require_ownership(&user, &post)?;

    post.unpublish();
    let post = state.db
        .update_post(post)
        .await
        .map_err(|e| crate::error::ApiError::Internal(e.to_string()))?;

    Ok((StatusCode::OK, Json(post)))
}
