//! Comment Repository Implementation
//!
//! This module provides a concrete implementation of CommentRepository
//! using SeaORM for database operations.
//!
//! Design Principles:
//! - Simple CRUD operations
//! - Clear error mapping
//! - No special cases

use crate::entity::comment;
use crate::entity::prelude::*;
use async_trait::async_trait;
use domain::{Comment, Error, Result};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QuerySelect, Set,
};
use std::sync::Arc;

/// Concrete implementation of CommentRepository
///
/// This implementation uses SeaORM to interact with comment table
/// in the database.
pub struct CommentRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

impl Clone for CommentRepositoryImpl {
    fn clone(&self) -> Self {
        Self {
            db: Arc::clone(&self.db),
        }
    }
}

impl CommentRepositoryImpl {
    /// Create a new comment repository
    ///
    /// # Arguments
    /// * `db` - Database connection
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl domain::CommentRepository for CommentRepositoryImpl {
    /// Create a new comment
    async fn create_comment(&self, comment: Comment) -> Result<Comment> {
        let active_model = comment::ActiveModel {
            id: Set(comment.id.to_string()),
            post_id: Set(comment.post_id.to_string()),
            user_id: Set(comment.user_id.map(|id| id.to_string())),
            github_username: Set(comment.github_username.clone()),
            github_avatar_url: Set(comment.github_avatar_url.clone()),
            content: Set(comment.content.clone()),
            created_at: Set(comment.created_at.to_rfc3339()),
            updated_at: Set(comment.updated_at.to_rfc3339()),
        };

        active_model
            .insert(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to create comment: {}", e)))?;

        Ok(comment)
    }

    /// Get a comment by ID
    async fn get_comment(&self, id: uuid::Uuid) -> Result<Option<Comment>> {
        let model = CommentEntity::find_by_id(id.to_string())
            .one(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to get comment: {}", e)))?;

        Ok(model.map(|m| self.model_to_domain(m)))
    }

    /// List comments for a post
    async fn list_post_comments(&self, post_id: uuid::Uuid, limit: u64) -> Result<Vec<Comment>> {
        let models = CommentEntity::find()
            .filter(comment::Column::PostId.eq(post_id.to_string()))
            .limit(limit)
            .all(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to list comments: {}", e)))?;

        Ok(models
            .into_iter()
            .map(|m| self.model_to_domain(m))
            .collect())
    }

    /// Update a comment
    async fn update_comment(&self, comment: Comment) -> Result<Comment> {
        let active_model = comment::ActiveModel {
            id: Set(comment.id.to_string()),
            post_id: Set(comment.post_id.to_string()),
            user_id: Set(comment.user_id.map(|id| id.to_string())),
            github_username: Set(comment.github_username.clone()),
            github_avatar_url: Set(comment.github_avatar_url.clone()),
            content: Set(comment.content.clone()),
            created_at: Set(comment.created_at.to_rfc3339()),
            updated_at: Set(comment.updated_at.to_rfc3339()),
        };

        active_model
            .update(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to update comment: {}", e)))?;

        Ok(comment)
    }

    /// Delete a comment by ID
    async fn delete_comment(
        &self,
        id: uuid::Uuid,
        user_id: Option<uuid::Uuid>,
        is_github_user: bool,
    ) -> Result<()> {
        // Verify ownership before deleting
        let model = CommentEntity::find_by_id(id.to_string())
            .one(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to find comment: {}", e)))?
            .ok_or_else(|| Error::NotFound("Comment not found".to_string()))?;

        // Check ownership
        if is_github_user {
            if model.github_username.is_none() {
                return Err(Error::Validation("Not a GitHub user comment".to_string()));
            }
        } else if let Some(uid) = user_id {
            if model.user_id != Some(uid.to_string()) {
                return Err(Error::Validation(
                    "You can only delete your own comments".to_string(),
                ));
            }
        } else {
            return Err(Error::Validation("Invalid user".to_string()));
        }

        CommentEntity::delete_by_id(id.to_string())
            .exec(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to delete comment: {}", e)))?;

        Ok(())
    }

    /// Get comment count for a post
    async fn get_post_comment_count(&self, post_id: uuid::Uuid) -> Result<u64> {
        let count = CommentEntity::find()
            .filter(comment::Column::PostId.eq(post_id.to_string()))
            .count(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to count comments: {}", e)))?;

        Ok(count)
    }
}

impl CommentRepositoryImpl {
    /// Convert database model to domain type
    fn model_to_domain(&self, model: comment::Model) -> Comment {
        Comment {
            id: uuid::Uuid::parse_str(&model.id).unwrap_or_else(|_| uuid::Uuid::new_v4()),
            post_id: uuid::Uuid::parse_str(&model.post_id).unwrap_or_else(|_| uuid::Uuid::new_v4()),
            user_id: model.user_id.and_then(|id| uuid::Uuid::parse_str(&id).ok()),
            github_username: model.github_username,
            github_avatar_url: model.github_avatar_url,
            content: model.content,
            created_at: model
                .created_at
                .parse()
                .unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: model
                .updated_at
                .parse()
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_comment_repository_structure() {
        // Note: This is a placeholder test
        // Real tests would use a test database or mock

        let post_id = uuid::Uuid::new_v4();
        let user_id = uuid::Uuid::new_v4();

        // Test registered user comment
        let comment = Comment::from_user(post_id, user_id, "Test comment".to_string());
        assert_eq!(comment.post_id, post_id);
        assert_eq!(comment.user_id, Some(user_id));
        assert!(comment.github_username.is_none());
        assert!(comment.is_owned_by(user_id));
        assert!(!comment.is_from_github());

        // Test GitHub user comment
        let github_user = domain::GitHubUser {
            id: 12345,
            login: "testuser".to_string(),
            avatar_url: "https://github.com/avatar.png".to_string(),
            name: Some("Test User".to_string()),
            email: Some("test@example.com".to_string()),
        };

        let gh_comment = Comment::from_github(post_id, &github_user, "GitHub comment".to_string());
        assert_eq!(gh_comment.post_id, post_id);
        assert!(gh_comment.user_id.is_none());
        assert_eq!(gh_comment.github_username, Some("testuser".to_string()));
        assert!(gh_comment.is_from_github());
        assert!(!gh_comment.is_owned_by(user_id));
    }

    #[tokio::test]
    async fn test_comment_update() {
        let post_id = uuid::Uuid::new_v4();
        let user_id = uuid::Uuid::new_v4();

        let mut comment = Comment::from_user(post_id, user_id, "Original content".to_string());
        let original_updated = comment.updated_at;

        // Wait a bit to ensure timestamp changes
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        comment.update_content("Updated content".to_string());

        assert_eq!(comment.content, "Updated content".to_string());
        assert!(comment.updated_at > original_updated);
    }
}
