use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
    Router,
};
use domain::{CreateCategory, UpdateCategory, USER_MANAGE};

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
        .route("/", axum::routing::get(list_categories))
        .route("/", axum::routing::post(create_category))
        .route("/:id", axum::routing::get(get_category))
        .route("/:id", axum::routing::put(update_category))
        .route("/:id", axum::routing::delete(delete_category))
        .route("/:id/children", axum::routing::get(get_children))
}

async fn list_categories<PR, UR, SR, FR, CR, STR, CTR, TR>(
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
    let categories = state
        .category_service
        .list()
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::OK, Json(categories)))
}

async fn create_category<PR, UR, SR, FR, CR, STR, CTR, TR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR, CTR, TR>>,
    user: Claims,
    Json(input): Json<CreateCategory>,
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

    let category = state
        .category_service
        .create(input)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::CREATED, Json(category)))
}

async fn get_category<PR, UR, SR, FR, CR, STR, CTR, TR>(
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
    let category = state
        .category_service
        .get(id)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::OK, Json(category)))
}

async fn update_category<PR, UR, SR, FR, CR, STR, CTR, TR>(
    State(state): State<AppState<PR, UR, SR, FR, CR, STR, CTR, TR>>,
    user: Claims,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateCategory>,
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

    let category = state
        .category_service
        .update(id, input)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::OK, Json(category)))
}

async fn delete_category<PR, UR, SR, FR, CR, STR, CTR, TR>(
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
        .category_service
        .delete(id)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::NO_CONTENT, Json(serde_json::json!({}))))
}

async fn get_children<PR, UR, SR, FR, CR, STR, CTR, TR>(
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
    let categories = state
        .category_service
        .get_children(Some(id))
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::OK, Json(categories)))
}
