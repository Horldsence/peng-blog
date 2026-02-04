use axum::{
    extract::State,
    http::{header::CONTENT_TYPE, HeaderMap},
    response::{IntoResponse, Response},
};

use crate::{error::ApiError, state::AppState};

/// GET /rss
/// Get RSS feed of published posts
pub async fn get_rss_feed(State(state): State<AppState>) -> Result<Response, ApiError> {
    let feed = state
        .rss_service
        .generate_rss()
        .await
        .map_err(ApiError::Domain)?;

    let mut headers = HeaderMap::new();
    headers.insert(
        CONTENT_TYPE,
        "application/rss+xml; charset=utf-8".parse().unwrap(),
    );

    Ok((headers, feed).into_response())
}
