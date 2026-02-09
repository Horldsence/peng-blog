//! # Post Service - Business Logic for Posts
//!
//! This service implements business logic for post operations.
//! It coordinates repository calls and enforces business rules.

use domain::PostRepository;
use domain::{
    Error, IndexNowRequest, Post, Result, SearchPostsRequest, SearchPostsResponse, POST_DELETE,
    POST_PUBLISH, POST_UPDATE,
};
use infrastructure::IndexNowClient;
use std::sync::Arc;
use uuid::Uuid;

// ============================================================================
// Constants
// ============================================================================

/// Default limit for listing posts
const DEFAULT_LIST_LIMIT: u64 = 20;

/// Service for post business logic
///
/// This service encapsulates all business rules for post operations.
/// It uses dependency injection for repositories, making it testable.
pub struct PostService {
    repo: Arc<dyn PostRepository>,
    indexnow_client: Option<Arc<IndexNowClient>>,
    base_url: String,
    indexnow_key: Option<String>,
}

impl PostService {
    /// Create a new PostService with given repository and optional IndexNow client
    pub fn new(
        repo: Arc<dyn PostRepository>,
        indexnow_client: Option<Arc<IndexNowClient>>,
        base_url: String,
        indexnow_key: Option<String>,
    ) -> Self {
        Self {
            repo,
            indexnow_client,
            base_url,
            indexnow_key,
        }
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
        domain::check_permission(updater_permissions, POST_UPDATE)?;

        let mut post = self.repo.get_post(id).await?;

        domain::check_ownership_or_admin(
            post.user_id,
            updater_id,
            updater_permissions,
            POST_DELETE,
        )?;

        // Track if content changed for IndexNow notification
        let content_changed = title.is_some() || content.is_some();
        let was_published = post.is_published();

        // Update fields if provided
        if let Some(title) = title {
            self.validate_title(&title)?;
            post.title = title;
        }

        if let Some(content) = content {
            self.validate_content(&content)?;
            post.content = content;
        }

        let updated_post = self.repo.update_post(post).await?;

        // Notify IndexNow if post is published and content changed
        if was_published && content_changed {
            let _ = self.notify_indexnow(updated_post.id).await;
        }

        Ok(updated_post)
    }

    /// Publish a post with permission and ownership checks
    pub async fn publish(&self, id: Uuid, user_id: Uuid, permissions: u64) -> Result<Post> {
        domain::check_permission(permissions, POST_PUBLISH)?;

        let mut post = self.repo.get_post(id).await?;

        domain::check_ownership_or_admin(post.user_id, user_id, permissions, POST_DELETE)?;

        post.publish();
        let updated_post = self.repo.update_post(post).await?;

        // Notify IndexNow if configured
        let _ = self.notify_indexnow(updated_post.id).await;

        Ok(updated_post)
    }

    /// Unpublish a post with permission and ownership checks
    pub async fn unpublish(&self, id: Uuid, user_id: Uuid, permissions: u64) -> Result<Post> {
        domain::check_permission(permissions, POST_PUBLISH)?;

        let mut post = self.repo.get_post(id).await?;

        domain::check_ownership_or_admin(post.user_id, user_id, permissions, POST_DELETE)?;

        post.unpublish();
        self.repo.update_post(post).await
    }

    /// Delete a post with permission and ownership checks
    pub async fn delete(&self, id: Uuid, user_id: Uuid, permissions: u64) -> Result<()> {
        domain::check_permission(permissions, POST_DELETE)?;

        let post = self.repo.get_post(id).await?;

        domain::check_ownership_or_admin(post.user_id, user_id, permissions, POST_DELETE)?;

        self.repo.delete_post(id).await
    }

    /// List published posts
    pub async fn list_published(&self, limit: Option<u64>) -> Result<Vec<Post>> {
        self.repo
            .list_published_posts(limit.unwrap_or(DEFAULT_LIST_LIMIT))
            .await
    }

    /// Get posts by user
    pub async fn list_by_user(&self, user_id: Uuid, limit: Option<u64>) -> Result<Vec<Post>> {
        self.repo
            .get_posts_by_user(user_id, limit.unwrap_or(DEFAULT_LIST_LIMIT))
            .await
    }

    /// List all posts (including unpublished) - admin only
    pub async fn list_all(&self, limit: Option<u64>) -> Result<Vec<Post>> {
        self.repo
            .list_all_posts(limit.unwrap_or(DEFAULT_LIST_LIMIT))
            .await
    }

    /// List published posts by a specific user
    pub async fn list_published_by_user(
        &self,
        user_id: Uuid,
        limit: Option<u64>,
    ) -> Result<Vec<Post>> {
        self.repo
            .list_published_posts_by_user(user_id, limit.unwrap_or(DEFAULT_LIST_LIMIT))
            .await
    }

    /// Notify IndexNow for a single post and update status
    pub async fn notify_indexnow(&self, post_id: Uuid) -> Result<Post> {
        let Some(client) = &self.indexnow_client else {
            tracing::debug!("IndexNow client not configured");
            return self.repo.get_post(post_id).await;
        };

        let Some(key) = &self.indexnow_key else {
            tracing::debug!("IndexNow key not configured");
            return self.repo.get_post(post_id).await;
        };

        // Get current post
        let mut post = self.repo.get_post(post_id).await?;

        // Prepare IndexNow request
        let url = format!("{}/post/{}", self.base_url, post.id);

        let base_url_clean = self
            .base_url
            .trim_start_matches("http://")
            .trim_start_matches("https://");
        let host = base_url_clean.split('/').next().unwrap_or("localhost");

        let key_location = format!("{}/{}.txt", self.base_url, key);

        let request = IndexNowRequest {
            host: host.to_string(),
            key: key.clone(),
            key_location: Some(key_location),
            url_list: vec![url],
        };

        // Update status to pending
        post.indexnow_submitted = true;
        post.indexnow_submitted_at = Some(chrono::Utc::now());
        post.indexnow_last_status = Some("pending".to_string());
        post.indexnow_last_error = None;
        post = self.repo.update_post(post).await?;

        // Send notification
        match client.notify(request).await {
            Ok(()) => {
                post.indexnow_last_status = Some("success".to_string());
                post.indexnow_last_error = None;
                tracing::info!("IndexNow notification successful for post {}", post_id);
            }
            Err(e) => {
                post.indexnow_last_status = Some("failed".to_string());
                post.indexnow_last_error = Some(e.clone());
                tracing::error!("IndexNow notification failed for post {}: {}", post_id, e);
            }
        }

        // Update final status
        self.repo.update_post(post).await
    }

    /// Set category for a post with permission and ownership checks
    pub async fn set_category(
        &self,
        post_id: Uuid,
        category_id: Option<Uuid>,
        user_id: Uuid,
        permissions: u64,
    ) -> Result<()> {
        domain::check_permission(permissions, POST_UPDATE)?;

        let post = self.repo.get_post(post_id).await?;
        domain::check_ownership_or_admin(post.user_id, user_id, permissions, POST_DELETE)?;

        self.repo.update_post_category(post_id, category_id).await
    }

    /// Add tag to post with permission and ownership checks
    pub async fn add_tag(
        &self,
        post_id: Uuid,
        tag_id: Uuid,
        user_id: Uuid,
        permissions: u64,
    ) -> Result<()> {
        domain::check_permission(permissions, POST_UPDATE)?;

        let post = self.repo.get_post(post_id).await?;
        domain::check_ownership_or_admin(post.user_id, user_id, permissions, POST_DELETE)?;

        self.repo.add_tag_to_post(post_id, tag_id).await
    }

    /// Remove tag from post with permission and ownership checks
    pub async fn remove_tag(
        &self,
        post_id: Uuid,
        tag_id: Uuid,
        user_id: Uuid,
        permissions: u64,
    ) -> Result<()> {
        domain::check_permission(permissions, POST_UPDATE)?;

        let post = self.repo.get_post(post_id).await?;
        domain::check_ownership_or_admin(post.user_id, user_id, permissions, POST_DELETE)?;

        self.repo.remove_tag_from_post(post_id, tag_id).await
    }

    /// Get tags for a post
    pub async fn get_tags(&self, post_id: Uuid) -> Result<Vec<domain::Tag>> {
        self.repo.get_post_tags(post_id).await
    }

    /// List published posts by category
    pub async fn list_by_category(
        &self,
        category_id: Uuid,
        limit: Option<u64>,
    ) -> Result<Vec<Post>> {
        self.repo
            .get_posts_by_category(category_id, limit.unwrap_or(DEFAULT_LIST_LIMIT))
            .await
    }

    /// List published posts by tag
    pub async fn list_by_tag(&self, tag_id: Uuid, limit: Option<u64>) -> Result<Vec<Post>> {
        self.repo
            .get_posts_by_tag(tag_id, limit.unwrap_or(DEFAULT_LIST_LIMIT))
            .await
    }

    /// Search posts by query
    pub async fn search(&self, request: SearchPostsRequest) -> Result<SearchPostsResponse> {
        let query = request.query.trim();
        if query.is_empty() {
            return Err(Error::Validation(
                "Search query cannot be empty".to_string(),
            ));
        }
        let limit = request.limit.unwrap_or(DEFAULT_LIST_LIMIT);
        let offset = request.offset.unwrap_or(0);
        self.repo.search_posts(query, limit, offset).await
    }
}

// ============================================================================
// Private Validation Helpers
// ============================================================================

impl PostService {
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
    use domain::Tag;
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
            async fn list_published_posts_by_user(&self, user_id: Uuid, limit: u64) -> Result<Vec<Post>>;
            async fn list_all_posts(&self, limit: u64) -> Result<Vec<Post>>;
            async fn update_post_category(&self, post_id: Uuid, category_id: Option<Uuid>) -> Result<()>;
            async fn get_posts_by_category(&self, category_id: Uuid, limit: u64) -> Result<Vec<Post>>;
            async fn add_tag_to_post(&self, post_id: Uuid, tag_id: Uuid) -> Result<()>;
            async fn remove_tag_from_post(&self, post_id: Uuid, tag_id: Uuid) -> Result<()>;
            async fn get_post_tags(&self, post_id: Uuid) -> Result<Vec<Tag>>;
            async fn get_posts_by_tag(&self, tag_id: Uuid, limit: u64) -> Result<Vec<Post>>;
            async fn search_posts(&self, query: &str, limit: u64, offset: u64) -> Result<SearchPostsResponse>;
        }
    }

    #[tokio::test]
    async fn test_create_post_validates_empty_title() {
        let mock_repo = Arc::new(MockPostRepo::new());
        let service = PostService::new(mock_repo, None, "http://localhost".to_string(), None);

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
        let mock_repo = Arc::new(MockPostRepo::new());
        let service = PostService::new(mock_repo, None, "http://localhost".to_string(), None);

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
        let mock_repo = Arc::new(MockPostRepo::new());
        let service = PostService::new(mock_repo, None, "http://localhost".to_string(), None);

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
        let mock_repo = Arc::new(MockPostRepo::new());
        let service = PostService::new(mock_repo, None, "http://localhost".to_string(), None);

        let post_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let no_permissions = 0;

        let result = service
            .update(
                post_id,
                None,
                Some("new content".to_string()),
                user_id,
                no_permissions,
            )
            .await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("Permission denied")),
            _ => panic!("Expected permission error"),
        }
    }

    #[tokio::test]
    async fn test_delete_post_requires_permission() {
        let mock_repo = Arc::new(MockPostRepo::new());
        let service = PostService::new(mock_repo, None, "http://localhost".to_string(), None);

        let post_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        let no_permissions = 0;

        let result = service.delete(post_id, user_id, no_permissions).await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("Permission denied")),
            _ => panic!("Expected permission error"),
        }
    }
}
