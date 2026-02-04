use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents an uploaded file
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct File {
    pub id: Uuid,
    pub user_id: Uuid,
    pub filename: String,
    pub original_filename: String,
    pub content_type: String,
    pub size_bytes: u64,
    pub url: String,
    pub created_at: DateTime<Utc>,
}

impl File {
    /// Create a new file record
    pub fn new(
        user_id: Uuid,
        filename: String,
        original_filename: String,
        content_type: String,
        size_bytes: u64,
        url: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            filename,
            original_filename,
            content_type,
            size_bytes,
            url,
            created_at: Utc::now(),
        }
    }

    /// Check if file is owned by a specific user
    pub fn is_owned_by(&self, user_id: Uuid) -> bool {
        self.user_id == user_id
    }

    /// Generate URL for the file download endpoint
    pub fn generate_url(base_url: &str, filename: &str) -> String {
        format!(
            "{}/api/files/{}/download",
            base_url.trim_end_matches('/'),
            filename
        )
    }
}

/// Request to upload a file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFile {
    pub filename: String,
    pub content_type: String,
    pub data: Vec<u8>,
}

/// Response for file upload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileResponse {
    pub id: Uuid,
    pub url: String,
    pub filename: String,
    pub original_filename: String,
    pub content_type: String,
    pub size_bytes: u64,
}
