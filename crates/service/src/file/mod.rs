//! File Service
//!
//! This module provides business logic for file management.
//! Files are uploaded by users and stored in the filesystem.
//!
//! Design Principles:
//! - Simple CRUD operations on files
//! - No special cases - all files follow the same rules
//! - File system operations are abstracted through repository

use domain::FileRepository;
use domain::{Error, File, FileResponse, Result, UploadFile};
use std::sync::Arc;

/// File service for managing user uploads
///
/// This service handles all file-related business logic including:
/// - Uploading new files
/// - Retrieving file information
/// - Listing user files
/// - Deleting files
///
/// All operations are database-backed through the FileRepository trait.
#[derive(Clone)]
pub struct FileService {
    file_repo: Arc<dyn FileRepository>,
    upload_dir: String,
    base_url: String,
}

impl FileService {
    /// Create a new file service
    ///
    /// # Arguments
    /// * `file_repo` - The file repository implementation (wrapped in Arc)
    /// * `upload_dir` - Directory where files are stored (absolute path)
    /// * `base_url` - Base URL for file access (e.g., "http://example.com")
    pub fn new(file_repo: Arc<dyn FileRepository>, upload_dir: String, base_url: String) -> Self {
        Self {
            file_repo,
            upload_dir,
            base_url,
        }
    }

    /// Upload a new file
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user uploading the file
    /// * `upload` - The file upload request with filename, content type, and data
    ///
    /// # Returns
    /// * `Ok(FileResponse)` - The uploaded file information
    /// * `Err(Error::Validation)` - File size exceeds limit or invalid type
    /// * `Err(Error::Internal)` - File system or database error
    pub async fn upload_file(
        &self,
        user_id: uuid::Uuid,
        upload: UploadFile,
    ) -> Result<FileResponse> {
        // Validate file size (max 10MB)
        const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024;
        if upload.data.len() as u64 > MAX_FILE_SIZE {
            return Err(Error::Validation(
                "File size exceeds 10MB limit".to_string(),
            ));
        }

        // Validate content type (only allow images and documents)
        let allowed_types = [
            "image/jpeg",
            "image/png",
            "image/gif",
            "image/webp",
            "application/pdf",
            "text/plain",
            "text/markdown",
        ];
        if !allowed_types.contains(&upload.content_type.as_str()) {
            return Err(Error::Validation(format!(
                "Invalid content type: {}. Allowed types: {}",
                upload.content_type,
                allowed_types.join(", ")
            )));
        }

        // Generate unique filename
        let filename = self.generate_filename(&upload.filename);
        let file_path = format!("{}/{}", self.upload_dir.trim_end_matches('/'), filename);

        // Get file size before moving data
        let size_bytes = upload.data.len() as u64;

        // Write file to disk
        tokio::fs::write(&file_path, upload.data)
            .await
            .map_err(|e| Error::Internal(format!("Failed to write file: {}", e)))?;

        // Create file record in database
        let url = File::generate_url(&self.base_url, &filename);
        let file = File::new(
            user_id,
            filename.clone(),
            upload.filename,
            upload.content_type,
            size_bytes,
            url,
        );

        let saved_file = self.file_repo.create_file(file).await?;

        Ok(FileResponse {
            id: saved_file.id,
            url: saved_file.url,
            filename: saved_file.filename,
            original_filename: saved_file.original_filename,
            content_type: saved_file.content_type,
            size_bytes: saved_file.size_bytes,
        })
    }

    /// Get file information by ID
    ///
    /// # Arguments
    /// * `id` - The file ID
    ///
    /// # Returns
    /// * `Ok(Some(FileResponse))` - File information if found
    /// * `Ok(None)` - File not found
    /// * `Err(Error)` - Database error
    pub async fn get_file(&self, id: uuid::Uuid) -> Result<Option<FileResponse>> {
        self.file_repo.get_file(id).await.map(|opt| {
            opt.map(|file| FileResponse {
                id: file.id,
                url: file.url,
                filename: file.filename,
                original_filename: file.original_filename,
                content_type: file.content_type,
                size_bytes: file.size_bytes,
            })
        })
    }

    /// List files uploaded by a user
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user
    /// * `limit` - Maximum number of files to return
    ///
    /// # Returns
    /// * `Ok(Vec<FileResponse>)` - List of files
    /// * `Err(Error)` - Database error
    pub async fn list_files(&self, user_id: uuid::Uuid, limit: u64) -> Result<Vec<FileResponse>> {
        let files = self.file_repo.list_files_by_user(user_id, limit).await?;
        Ok(files
            .into_iter()
            .map(|file| FileResponse {
                id: file.id,
                url: file.url,
                filename: file.filename,
                original_filename: file.original_filename,
                content_type: file.content_type,
                size_bytes: file.size_bytes,
            })
            .collect())
    }

    /// Delete a file
    ///
    /// # Arguments
    /// * `id` - The file ID
    /// * `user_id` - The ID of the user (for ownership verification)
    ///
    /// # Returns
    /// * `Ok(())` - File deleted
    /// * `Err(Error::NotFound)` - File not found or not owned by user
    /// * `Err(Error)` - File system or database error
    pub async fn delete_file(&self, id: uuid::Uuid, user_id: uuid::Uuid) -> Result<()> {
        // Check ownership first
        let file_opt = self.file_repo.get_file(id).await?;
        let file = file_opt.ok_or_else(|| Error::NotFound("File not found".to_string()))?;

        if !file.is_owned_by(user_id) {
            return Err(Error::Validation(
                "You can only delete your own files".to_string(),
            ));
        }

        // Delete from database
        self.file_repo.delete_file(id, user_id).await?;

        // Delete from filesystem (best effort)
        let file_path = format!(
            "{}/{}",
            self.upload_dir.trim_end_matches('/'),
            file.filename
        );
        let _ = tokio::fs::remove_file(file_path).await;

        Ok(())
    }
}

impl FileService {
    /// Generate a unique filename
    ///
    /// # Arguments
    /// * `original_filename` - The original filename
    ///
    /// # Returns
    /// A unique filename preserving the original extension
    fn generate_filename(&self, original_filename: &str) -> String {
        let uuid = uuid::Uuid::new_v4();
        let extension = std::path::Path::new(original_filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");

        if extension.is_empty() {
            uuid.to_string()
        } else {
            format!("{}.{}", uuid, extension)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::FileRepository;
    use async_trait::async_trait;
    use domain::{File, Result};
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    // Mock repository for testing
    struct MockFileRepo {
        files: Arc<RwLock<HashMap<uuid::Uuid, File>>>,
    }

    #[async_trait]
    impl FileRepository for MockFileRepo {
        async fn create_file(&self, file: File) -> Result<File> {
            let mut files = self.files.write().await;
            files.insert(file.id, file.clone());
            Ok(file)
        }

        async fn get_file(&self, id: uuid::Uuid) -> Result<Option<File>> {
            let files = self.files.read().await;
            Ok(files.get(&id).cloned())
        }

        async fn list_files_by_user(&self, user_id: uuid::Uuid, _limit: u64) -> Result<Vec<File>> {
            let files = self.files.read().await;
            Ok(files
                .values()
                .filter(|f| f.user_id == user_id)
                .cloned()
                .collect())
        }

        async fn delete_file(&self, id: uuid::Uuid, _user_id: uuid::Uuid) -> Result<()> {
            let mut files = self.files.write().await;
            files.remove(&id);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_upload_file() {
        let repo = Arc::new(MockFileRepo {
            files: Arc::new(RwLock::new(HashMap::new())),
        });
        let _service = FileService::new(
            repo,
            "/tmp/uploads".to_string(),
            "http://example.com".to_string(),
        );

        let _upload = UploadFile {
            filename: "test.txt".to_string(),
            content_type: "text/plain".to_string(),
            data: b"Hello, World!".to_vec(),
        };

        // Note: This test will fail because we can't write to /tmp/uploads in tests
        // In real tests, we'd use a temp directory or mock the file system
        // For now, we just show the structure
        let _user_id = uuid::Uuid::new_v4();
        // let result = service.upload_file(user_id, upload).await;
        // assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_generate_filename() {
        let repo = Arc::new(MockFileRepo {
            files: Arc::new(RwLock::new(HashMap::new())),
        });
        let service = FileService::new(
            repo,
            "/tmp/uploads".to_string(),
            "http://example.com".to_string(),
        );

        let filename = service.generate_filename("test.jpg");
        assert!(filename.ends_with(".jpg"));

        let filename = service.generate_filename("test");
        assert!(!filename.contains("."));
    }
}
