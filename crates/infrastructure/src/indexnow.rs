//! IndexNow HTTP Client Implementation
//!
//! This module provides the HTTP client for sending IndexNow notifications
//! to search engines.

use domain::IndexNowRequest;

pub struct IndexNowClient {
    client: reqwest::Client,
    endpoint: String,
}

impl IndexNowClient {
    pub fn new(endpoint: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            endpoint,
        }
    }

    pub async fn notify(&self, request: IndexNowRequest) -> Result<(), String> {
        let response = self
            .client
            .post(&self.endpoint)
            .header("Content-Type", "application/json; charset=utf-8")
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        let status = response.status();

        if status.is_success() {
            tracing::info!("IndexNow notification successful: {}", status);
            Ok(())
        } else {
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read error response".to_string());

            tracing::error!("IndexNow notification failed: {} - {}", status, error_text);

            Err(format!("IndexNow API error: {} - {}", status, error_text))
        }
    }
}
