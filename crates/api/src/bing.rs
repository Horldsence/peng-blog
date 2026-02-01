use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

use crate::{ApiError, ApiResponse, AppState};

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

#[derive(Debug, Serialize)]
pub struct BingDailyImageResponse {
    pub url: String,
    pub copyright: String,
    pub copyright_link: Option<String>,
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new().route("/daily-image", axum::routing::get(get_bing_daily_image))
}

pub async fn get_bing_daily_image(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
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

    let result = BingDailyImageResponse {
        url: full_url,
        copyright: bing_image.copyright.clone(),
        copyright_link: bing_image.copyright_link.clone(),
    };

    Ok((StatusCode::OK, Json(ApiResponse::success(result))).into_response())
}
