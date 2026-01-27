//! # Post Service - Business Logic for Posts
//!
//! This service implements business logic for post operations.
//! It coordinates repository calls and enforces business rules.

use crate::repository::PostRepository;
use domain::{Error, Post, Result, POST_DELETE, POST_PUBLISH, POST_UPDATE};
use uuid::Uuid;
use std::sync::Arc;

// ============================================================================
// Constants
// ============================================================================

/// Default limit for listing posts
const DEFAULT_LIST_LIMIT: u64 = 20;

/// Service for post business logic
///
/// This service encapsulates all business rules for post operations.
/// It uses dependency injection for repositories, making it testable.
pub struct PostService<R: PostRepository> {
    repo: Arc<R>,
}

impl<R: PostRepository> PostService<R> {
    /// Create a new PostService with given repository
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    /// Create a new post with validation
    pub async fn create(&self, user_id: Uuid, title: String, content: String) -> Result<Post> {
        self.validate_title(&title)?;
        self.validate_content(&content)?;
        self.repo.create_post(user_id, title, content).await
    }

    /// Get a post by ID
    pub async fn get(&self, id: Uuid) -> Result<Post> {
        self.repo.get_post(id).await
    }

    /// Update an existing post with ownership and permission checks
    pub async fn update(
        &self,
        id: Uuid,
        title: Option<String>,
        content: Option<String>,
        updater_id: Uuid,
        updater_permissions: u64,
    ) -> Result<Post> {
        // Check update permission
        if (updater_permissions & POST_UPDATE) == 0 {
            return Err(Error::Validation(
                "Insufficient permissions to update post".to_string(),
            ));
        }

        let mut post = self.repo.get_post(id).await?;

        // Check ownership or admin permission
        if post.user_id != updater_id && (updater_permissions & POST_DELETE) == 0 {
            return Err(Error::Validation(
                "Cannot update posts owned by others".to_string(),
            ));
        }

        // Update fields if provided
        if let Some(title) = title {
            self.validate_title(&title)?;
            post.title = title;
        }

        if let Some(content) = content {
            self.validate_content(&content)?;
            post.content = content;
        }

        self.repo.update_post(post).await
    }

    /// Publish a post with permission and ownership checks
    pub async fn publish(&self, id: Uuid, user_id: Uuid, permissions: u64) -> Result<Post> {
        // Check publish permission
        if (permissions & POST_PUBLISH) == 0 {
            return Err(Error::Validation(
                "Insufficient permissions to publish post".to_string(),
            ));
        }

        let mut post = self.repo.get_post(id).await?;

        // Check ownership or admin permission
        if post.user_id != user_id && (permissions & POST_DELETE) == 0 {
            return Err(Error::Validation(
                "Cannot publish posts owned by others".to_string(),
            ));
        }

        post.publish();
        self.repo.update_post(post).await
    }

    /// Unpublish a post with permission and ownership checks
    pub async fn unpublish(&self, id: Uuid, user_id: Uuid, permissions: u64) -> Result<Post> {
        // Check publish permission
        if (permissions & POST_PUBLISH) == 0 {
            return Err(Error::Validation(
                "Insufficient permissions to unpublish post".to_string(),
            ));
        }

        let mut post = self.repo.get_post(id).await?;

        // Check ownership or admin permission
        if post.user_id != user_id && (permissions & POST_DELETE) == 0 {
            return Err(Error::Validation(
                "Cannot unpublish posts owned by others".to_string(),
            ));
        }

        post.unpublish();
        self.repo.update_post(post).await
    }

    /// Delete a post with permission and ownership checks
    pub async fn delete(&self, id: Uuid, user_id: Uuid, permissions: u64) -> Result<()> {
        // Check delete permission
        if (permissions & POST_DELETE) == 0 {
            return Err(Error::Validation(
                "Insufficient permissions to delete post".to_string(),
            ));
        }

        let post = self.repo.get_post(id).await?;

        // Check ownership or admin permission
        if post.user_id != user_id && (permissions & POST_DELETE) == 0 {
            return Err(Error::Validation(
                "Cannot delete posts owned by others".to_string(),
            ));
        }

        self.repo.delete_post(id).await
    }

    /// List published posts
    pub async fn list_published(&self, limit: Option<u64>) -> Result<Vec<Post>> {
        self.repo.list_published_posts(limit.unwrap_or(DEFAULT_LIST_LIMIT)).await
    }

    /// Get posts by user
    pub async fn list_by_user(
        &self,
        user_id: Uuid,
        limit: Option<u64>,
    ) -> Result<Vec<Post>> {
        self.repo.get_posts_by_user(user_id, limit.unwrap_or(DEFAULT_LIST_LIMIT)).await
    }
}

// ============================================================================
// Private Validation Helpers
// ============================================================================

impl<R: PostRepository> PostService<R> {
    fn validate_title(&self, title: &str) -> Result<()> {
        if title.trim().is_empty() {
            return Err(Error::Validation("Title cannot be empty".to_string()));
        }
        if title.len() > 200 {
            return Err(Error::Validation(
                "Title too long (max 200 characters)".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_content(&self, content: &str) -> Result<()> {
        if content.trim().is_empty() {
            return Err(Error::Validation("Content cannot be empty".to_string()));
        }
        Ok(())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use mockall::mock;
    use std::sync::Arc;

    // Mock repository for testing
    mock! {
        PostRepo {}

        #[async_trait]
        impl PostRepository for PostRepo {
            async fn create_post(&self, user_id: Uuid, title: String, content: String) -> Result<Post>;
            async fn get_post(&self, id: Uuid) -> Result<Post>;
            async fn update_post(&self, post: Post) -> Result<Post>;
            async fn list_published_posts(&self, limit: u64) -> Result<Vec<Post>>;
            async fn delete_post(&self, id: Uuid) -> Result<()>;
            async fn get_posts_by_user(&self, user_id: Uuid, limit: u64) -> Result<Vec<Post>>;
        }
    }

    #[tokio::test]
    async fn test_create_post_validates_empty_title() {
        let mock_repo = MockPostRepo::new();
        let service = PostService::new(Arc::new(mock_repo));

        let user_id = Uuid::new_v4();

        let result = service
            .create(user_id, "".to_string(), "content".to_string())
            .await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("cannot be empty")),
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_create_post_validates_title_too_long() {
        let mock_repo = MockPostRepo::new();
        let service = PostService::new(Arc::new(mock_repo));

        let user_id = Uuid::new_v4();
        let long_title = "a".repeat(201);

        let result = service
            .create(user_id, long_title, "content".to_string())
            .await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("too long")),
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_create_post_validates_long_content() {
        let mock_repo = MockPostRepo::new();
        let service = PostService::new(Arc::new(mock_repo));

        let _long_content = "a".repeat(10001);
        let user_id = Uuid::new_v4();

        let result = service
            .create(user_id, "title".to_string(), "".to_string())
            .await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("cannot be empty")),
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_update_requires_permission() {
        let mock_repo = MockPostRepo::new();
        let service = PostService::new(Arc::new(mock_repo));

        let post_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let no_permissions = 0;

        let result = service
            .update(post_id, None, Some("new content".to_string()), user_id, no_permissions)
            .await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("Insufficient permissions")),
            _ => panic!("Expected permission error"),
        }
    }

    #[tokio::test]
    async fn test_delete_post_requires_permission() {
        let mock_repo = MockPostRepo::new();
        let service = PostService::new(Arc::new(mock_repo));

        let post_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let no_permissions = 0;

        let result = service.delete(post_id, user_id, no_permissions).await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("Insufficient permissions")),
            _ => panic!("Expected permission error"),
        }
    }
}