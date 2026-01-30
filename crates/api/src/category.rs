use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use domain::{CreateCategory, UpdateCategory, USER_MANAGE};

use crate::error::ApiResult;
use crate::middleware::auth::Claims;
use uuid::Uuid;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", axum::routing::get(list_categories))
        .route("/", axum::routing::post(create_category))
        .route("/:id", axum::routing::get(get_category))
        .route("/:id", axum::routing::put(update_category))
        .route("/:id", axum::routing::delete(delete_category))
        .route("/:id/children", axum::routing::get(get_children))
}

async fn list_categories(State(state): State<AppState>) -> ApiResult<impl IntoResponse> {
    let categories = state
        .category_service
        .list()
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::OK, Json(categories)))
}

async fn create_category(
    State(state): State<AppState>,
    user: Claims,
    Json(input): Json<CreateCategory>,
) -> ApiResult<impl IntoResponse> {
    domain::check_permission(user.permissions, USER_MANAGE)?;

    let category = state
        .category_service
        .create(input)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::CREATED, Json(category)))
}

async fn get_category(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse> {
    let category = state
        .category_service
        .get(id)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::OK, Json(category)))
}

async fn update_category(
    State(state): State<AppState>,
    user: Claims,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateCategory>,
) -> ApiResult<impl IntoResponse> {
    domain::check_permission(user.permissions, USER_MANAGE)?;

    let category = state
        .category_service
        .update(id, input)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::OK, Json(category)))
}

async fn delete_category(
    State(state): State<AppState>,
    user: Claims,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse> {
    domain::check_permission(user.permissions, USER_MANAGE)?;

    state
        .category_service
        .delete(id)
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::NO_CONTENT, Json(serde_json::json!({}))))
}

async fn get_children(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<impl IntoResponse> {
    let categories = state
        .category_service
        .get_children(Some(id))
        .await
        .map_err(crate::error::ApiError::Domain)?;

    Ok((StatusCode::OK, Json(categories)))
}
