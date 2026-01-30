use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use domain::{CreateTag, USER_MANAGE};

use crate::error::ApiResult;
use crate::middleware::auth::Claims;
use uuid::Uuid;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", axum::routing::get(list_tags))
        .route("/", axum::routing::post(create_tag))
        .route("/:id", axum::routing::get(get_tag))
        .route("/:id", axum::routing::delete(delete_tag))
}

async fn list_tags(State(state): State<AppState>) -> ApiResult<impl IntoResponse> {
    let tags = state
        .tag_service
        .list()
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::OK, Json(tags)))
}

async fn create_tag(
    State(state): State<AppState>,
    user: Claims,
    Json(input): Json<CreateTag>,
) -> ApiResult<impl IntoResponse> {
    domain::check_permission(user.permissions, USER_MANAGE)?;

    let tag = state
        .tag_service
        .create(input)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::CREATED, Json(tag)))
}

async fn get_tag(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse> {
    let tag = state
        .tag_service
        .get(id)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::OK, Json(tag)))
}

async fn delete_tag(
    State(state): State<AppState>,
    user: Claims,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse> {
    domain::check_permission(user.permissions, USER_MANAGE)?;

    state
        .tag_service
        .delete(id)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::NO_CONTENT, Json(serde_json::json!({}))))
}
