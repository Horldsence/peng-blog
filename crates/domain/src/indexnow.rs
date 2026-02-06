//! # IndexNow Domain Types
//!
//! IndexNow is a protocol for search engines to be notified of website content changes.
//! See: https://www.indexnow.org/

use serde::{Deserialize, Serialize};

/// IndexNow notification request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexNowRequest {
    /// The URL of the content that was created or updated
    pub url: String,

    /// The API key provided by the search engine
    pub key: String,
}

/// IndexNow notification response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexNowResponse {
    /// Whether the notification was successful
    pub success: bool,

    /// Optional message from the search engine
    pub message: Option<String>,
}
