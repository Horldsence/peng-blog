//! Configuration Management API Routes
//!
//! ## Endpoints
//!
//! | Method | Endpoint | Description |
//! |--------|----------|-------------|
//! | GET | /config | Get current configuration (admin) |
//! | PATCH | /config | Update configuration (admin) |

use axum::{extract::State, response::IntoResponse, Json, Router};
use config::{default_config_path, save_config};
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

    let config_response = domain::ConfigResponse::from(state.config.clone());
    Ok(resp::ok(config_response))
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

    let mut updated_config = state.config.clone();

    if let Some(database) = input.database {
        if let Some(url) = database.url {
            updated_config.database.url = url;
        }
    }

    if let Some(server) = input.server {
        if let Some(host) = server.host {
            updated_config.server.host = host;
        }
        if let Some(port) = server.port {
            updated_config.server.port = port;
        }
    }

    if let Some(auth) = input.auth {
        if let Some(jwt_secret) = auth.jwt_secret {
            updated_config.auth.jwt_secret = jwt_secret;
        }
    }

    if let Some(storage) = input.storage {
        if let Some(upload_dir) = storage.upload_dir {
            updated_config.storage.upload_dir = upload_dir;
        }
        if let Some(cache_dir) = storage.cache_dir {
            updated_config.storage.cache_dir = cache_dir;
        }
    }

    if let Some(github) = input.github {
        if let Some(client_id) = github.client_id {
            updated_config.github.client_id = client_id;
        }
        if let Some(client_secret) = github.client_secret {
            updated_config.github.client_secret = client_secret;
        }
    }

    if let Some(site) = input.site {
        if let Some(allow_registration) = site.allow_registration {
            updated_config.site.allow_registration = allow_registration;
        }
    }

    updated_config
        .validate()
        .map_err(|e| ApiError::Validation(format!("Invalid configuration: {}", e)))?;

    save_config(&updated_config, default_config_path())
        .map_err(|e| ApiError::Internal(format!("Failed to save configuration: {}", e)))?;

    let config_response = domain::ConfigResponse::from(updated_config);

    Ok(resp::ok(config_response))
}
