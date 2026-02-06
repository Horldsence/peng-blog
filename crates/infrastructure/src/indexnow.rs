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
        self.client
            .post(&self.endpoint)
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to send IndexNow notification: {}", e))?;

        Ok(())
    }
}
