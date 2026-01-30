//! File Repository Implementation
//!
//! This module provides a concrete implementation of FileRepository
//! using SeaORM for database operations.
//!
//! Design Principles:
//! - Simple CRUD operations
//! - Clear error mapping
//! - No special cases

use crate::entity::file;
use crate::entity::prelude::*;
use async_trait::async_trait;
use domain::{Error, File, Result};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, Set,
};
use std::sync::Arc;

/// Concrete implementation of FileRepository
///
/// This implementation uses SeaORM to interact with file table
/// in the database.
pub struct FileRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

impl Clone for FileRepositoryImpl {
    fn clone(&self) -> Self {
        Self {
            db: Arc::clone(&self.db),
        }
    }
}

impl FileRepositoryImpl {
    /// Create a new file repository
    ///
    /// # Arguments
    /// * `db` - Database connection
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl service::FileRepository for FileRepositoryImpl {
    /// Create a new file record
    async fn create_file(&self, file: File) -> Result<File> {
        let active_model = file::ActiveModel {
            id: Set(file.id.to_string()),
            user_id: Set(file.user_id.to_string()),
            filename: Set(file.filename.clone()),
            original_filename: Set(file.original_filename.clone()),
            content_type: Set(file.content_type.clone()),
            size_bytes: Set(file.size_bytes as i64),
            url: Set(file.url.clone()),
            created_at: Set(file.created_at.to_rfc3339()),
        };

        active_model
            .insert(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to create file: {}", e)))?;

        Ok(file)
    }

    /// Get a file by ID
    async fn get_file(&self, id: uuid::Uuid) -> Result<Option<File>> {
        let model = FileEntity::find_by_id(id.to_string())
            .one(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to get file: {}", e)))?;

        Ok(model.map(|m| File {
            id: uuid::Uuid::parse_str(&m.id).unwrap_or_else(|_| uuid::Uuid::new_v4()),
            user_id: uuid::Uuid::parse_str(&m.user_id).unwrap_or_else(|_| uuid::Uuid::new_v4()),
            filename: m.filename,
            original_filename: m.original_filename,
            content_type: m.content_type,
            size_bytes: m.size_bytes as u64,
            url: m.url,
            created_at: m.created_at.parse().unwrap_or_else(|_| chrono::Utc::now()),
        }))
    }

    /// List files by user ID
    async fn list_files_by_user(&self, user_id: uuid::Uuid, limit: u64) -> Result<Vec<File>> {
        let models = FileEntity::find()
            .filter(file::Column::UserId.eq(user_id.to_string()))
            .limit(limit)
            .all(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to list files: {}", e)))?;

        models
            .into_iter()
            .map(|m| {
                Ok(File {
                    id: uuid::Uuid::parse_str(&m.id).map_err(|e| {
                        Error::Internal(format!("Invalid file ID in database: {}", e))
                    })?,
                    user_id: uuid::Uuid::parse_str(&m.user_id).map_err(|e| {
                        Error::Internal(format!("Invalid user ID in database: {}", e))
                    })?,
                    filename: m.filename,
                    original_filename: m.original_filename,
                    content_type: m.content_type,
                    size_bytes: m.size_bytes as u64,
                    url: m.url,
                    created_at: m.created_at.parse().map_err(|e| {
                        Error::Internal(format!("Invalid created_at in database: {}", e))
                    })?,
                })
            })
            .collect()
    }

    /// Delete a file by ID
    async fn delete_file(&self, id: uuid::Uuid, user_id: uuid::Uuid) -> Result<()> {
        // Verify ownership first
        // Check ownership first
        let model = FileEntity::find_by_id(id.to_string())
            .one(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to find file: {}", e)))?
            .ok_or_else(|| Error::NotFound("File not found".to_string()))?;

        if model.user_id != user_id.to_string() {
            return Err(Error::Validation(
                "You can only delete your own files".to_string(),
            ));
        }

        FileEntity::delete_by_id(id.to_string())
            .exec(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to delete file: {}", e)))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_repository_structure() {
        // Note: This is a placeholder test
        // Real tests would use a test database or mock

        let user_id = uuid::Uuid::new_v4();
        let file = File::new(
            user_id,
            "test.txt".to_string(),
            "original.txt".to_string(),
            "text/plain".to_string(),
            100,
            "http://example.com/test.txt".to_string(),
        );

        assert_eq!(file.user_id, user_id);
        assert_eq!(file.filename, "test.txt");
        assert_eq!(file.size_bytes, 100);
        assert!(file.is_owned_by(user_id));
        assert!(!file.is_owned_by(uuid::Uuid::new_v4()));
    }
}
