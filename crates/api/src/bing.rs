use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{ApiError, ApiResponse, AppState};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BingDailyImageResponse {
    pub url: String,
    pub copyright: String,
    pub copyright_link: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BingImageResponse {
    images: Vec<BingImage>,
}

#[derive(Debug, Deserialize)]
struct BingImage {
    url: String,
    copyright: String,
    copyright_link: Option<String>,
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new().route("/daily-image", axum::routing::get(get_bing_daily_image))
}

pub async fn get_bing_daily_image(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let cache_key = "bing_daily_image";

    if state.bing_cache.is_valid(cache_key, 12).await {
        tracing::debug!("Returning cached Bing image from file");

        if let Ok(Some(cached)) = state
            .bing_cache
            .get::<BingDailyImageResponse>(cache_key)
            .await
        {
            return Ok((StatusCode::OK, Json(ApiResponse::success(cached))).into_response());
        }
    }

    tracing::info!("Cache expired or empty, fetching from Bing API");
    let data = fetch_bing_image_from_api().await?;

    state.bing_cache.set(cache_key, &data).await?;

    Ok((StatusCode::OK, Json(ApiResponse::success(data))).into_response())
}

async fn fetch_bing_image_from_api() -> Result<BingDailyImageResponse, ApiError> {
    let client = reqwest::Client::builder()
        .build()
        .map_err(|e| ApiError::internal(format!("Failed to create HTTP client: {}", e)))?;

    let url = "https://www.bing.com/HPImageArchive.aspx?format=js&idx=0&n=1&mkt=zh-CN";

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| ApiError::internal(format!("Failed to fetch from Bing API: {}", e)))?;

    if !response.status().is_success() {
        return Err(ApiError::internal(format!(
            "Bing API returned status: {}",
            response.status()
        )));
    }

    let bing_response: BingImageResponse = response
        .json()
        .await
        .map_err(|e| ApiError::internal(format!("Failed to parse Bing API response: {}", e)))?;

    if bing_response.images.is_empty() {
        return Err(ApiError::internal("No images in Bing response".to_string()));
    }

    let bing_image = &bing_response.images[0];
    let full_url = if bing_image.url.starts_with("http") {
        bing_image.url.clone()
    } else {
        format!("https://www.bing.com{}", bing_image.url)
    };

    Ok(BingDailyImageResponse {
        url: full_url,
        copyright: bing_image.copyright.clone(),
        copyright_link: bing_image.copyright_link.clone(),
    })
}

pub async fn start_bing_cache_refresh_task(state: AppState) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(12 * 60 * 60));
        loop {
            interval.tick().await;
            tracing::info!("Refreshing Bing daily image cache");

            match fetch_bing_image_from_api().await {
                Ok(data) => {
                    let cache_key = "bing_daily_image";
                    if let Err(e) = state.bing_cache.set(cache_key, &data).await {
                        tracing::error!("Failed to save Bing cache: {}", e);
                    } else {
                        tracing::info!("Bing daily image cache refreshed successfully");
                    }
                }
                Err(e) => {
                    tracing::error!("Failed to refresh Bing cache: {}", e);
                }
            }
        }
    });
}
