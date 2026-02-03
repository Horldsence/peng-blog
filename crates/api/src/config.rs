//! Configuration Management API Routes
//!
//! ## Endpoints
//!
//! | Method | Endpoint | Description |
//! |--------|----------|-------------|
//! | GET | /config | Get current configuration (admin) |
//! | PATCH | /config | Update configuration (admin) |

use axum::{extract::State, response::IntoResponse, Json, Router};
use domain::USER_MANAGE;

use crate::{
    error::ApiError, middleware::auth::Claims, response::helpers as resp, state::AppState,
};

/// Create config routes
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", axum::routing::get(get_config))
        .route("/", axum::routing::patch(update_config))
}

/// GET /config
/// Get current configuration (admin only)
async fn get_config(
    State(state): State<AppState>,
    user: Claims,
) -> Result<impl IntoResponse, ApiError> {
    domain::check_permission(user.permissions, USER_MANAGE)
        .map_err(|e| ApiError::Unauthorized(e.to_string()))?;

    let config = state.config_service.get().await.map_err(ApiError::Domain)?;

    Ok(resp::ok(config))
}

/// PATCH /config
/// Update configuration (admin only)
async fn update_config(
    State(state): State<AppState>,
    user: Claims,
    Json(input): Json<domain::UpdateConfigRequest>,
) -> Result<impl IntoResponse, ApiError> {
    domain::check_permission(user.permissions, USER_MANAGE)
        .map_err(|e| ApiError::Unauthorized(e.to_string()))?;

    let config = state
        .config_service
        .update(input)
        .await
        .map_err(ApiError::Domain)?;

    Ok(resp::ok(config))
}
