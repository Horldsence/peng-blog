use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
    Router,
};
use domain::{CreateTag, USER_MANAGE};

use crate::error::ApiResult;
use crate::middleware::auth::Claims;
use uuid::Uuid;

use crate::{state::AppState, PostRepository, UserRepository, SessionRepository, FileRepository, CommentRepository, StatsRepository, CategoryRepository, TagRepository};

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
        .route("/", axum::routing::get(list_tags))
        .route("/", axum::routing::post(create_tag))
        .route("/:id", axum::routing::get(get_tag))
        .route("/:id", axum::routing::delete(delete_tag))
}

async fn list_tags<PR, UR, SR, FR, CR, STR, CTR, TR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR, CTR, TR>>,
) -> ApiResult<impl IntoResponse>
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
    let tags = state
        .tag_service
        .list()
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::OK, Json(tags)))
}

async fn create_tag<PR, UR, SR, FR, CR, STR, CTR, TR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR, CTR, TR>>,
    user: Claims,
    Json(input): Json<CreateTag>,
) -> ApiResult<impl IntoResponse>
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
    domain::check_permission(user.permissions, USER_MANAGE)?;

    let tag = state
        .tag_service
        .create(input)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::CREATED, Json(tag)))
}

async fn get_tag<PR, UR, SR, FR, CR, STR, CTR, TR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR, CTR, TR>>,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse>
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
    let tag = state
        .tag_service
        .get(id)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::OK, Json(tag)))
}

async fn delete_tag<PR, UR, SR, FR, CR, STR, CTR, TR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR, CTR, TR>>,
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
    CTR: CategoryRepository + Send + Sync + 'static + Clone,
    TR: TagRepository + Send + Sync + 'static + Clone,
{
    domain::check_permission(user.permissions, USER_MANAGE)?;

    state
        .tag_service
        .delete(id)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::NO_CONTENT, Json(serde_json::json!({}))))
}
