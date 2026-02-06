//! # IndexNow Domain Types
//!
//! IndexNow is a protocol for search engines to be notified of website content changes.
//! See: https://www.indexnow.org/

use serde::{Deserialize, Serialize};

/// IndexNow notification request (supports batch URL submission)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexNowRequest {
    /// The host name of the website (e.g., "www.example.org")
    pub host: String,

    /// The API key provided by the search engine
    pub key: String,

    /// Optional: URL location of the key file for verification
    /// If not provided, search engines will look for https://host/key.txt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_location: Option<String>,

    /// List of URLs to submit (can be one or multiple)
    pub url_list: Vec<String>,
}

/// IndexNow notification response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexNowResponse {
    /// Whether the notification was successful
    pub success: bool,

    /// Optional message from the search engine
    pub message: Option<String>,
}
