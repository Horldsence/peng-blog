//! Comment Service
//!
//! This module provides business logic for comment management.
//! Comments support both registered users and GitHub OAuth users.
//!
//! Design Principles:
//! - Unified Comment model eliminates special cases
//! - GitHub OAuth follows standard 3-step flow
//! - No special cases - all comments follow the same rules

use crate::repository::{CommentRepository, UserRepository};
use domain::comment::{CreateCommentGitHub, GitHubTokenResponse, GitHubUser};
use domain::{Comment, CommentResponse, CreateComment, Error, Result};
use reqwest::Client;
use std::sync::Arc;

/// Comment service for managing post comments
///
/// This service handles all comment-related business logic including:
/// - Creating comments from registered users
/// - Creating comments from GitHub OAuth users
/// - Listing, updating, and deleting comments
///
/// All operations are database-backed through the CommentRepository trait.
#[derive(Clone)]
pub struct CommentService {
    comment_repo: Arc<dyn CommentRepository>,
    user_repo: Arc<dyn UserRepository>,
    github_client_id: String,
    github_client_secret: String,
}

impl CommentService {
    /// Create a new comment service
    ///
    /// # Arguments
    /// * `comment_repo` - The comment repository implementation (wrapped in Arc)
    /// * `user_repo` - The user repository implementation (wrapped in Arc)
    /// * `github_client_id` - GitHub OAuth client ID
    /// * `github_client_secret` - GitHub OAuth client secret
    pub fn new(
        comment_repo: Arc<dyn CommentRepository>,
        user_repo: Arc<dyn UserRepository>,
        github_client_id: String,
        github_client_secret: String,
    ) -> Self {
        Self {
            comment_repo,
            user_repo,
            github_client_id,
            github_client_secret,
        }
    }

    /// Generate GitHub OAuth authorization URL
    ///
    /// # Arguments
    /// * `state` - Random string for CSRF protection
    /// * `redirect_uri` - OAuth callback URL
    ///
    /// # Returns
    /// The GitHub OAuth authorization URL
    pub fn github_auth_url(&self, state: &str, redirect_uri: &str) -> String {
        format!(
            "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope=user:email&state={}",
            self.github_client_id,
            urlencoding::encode(redirect_uri),
            urlencoding::encode(state),
        )
    }

    /// Create a comment from a registered user
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user creating the comment
    /// * `create` - The comment creation request
    ///
    /// # Returns
    /// * `Ok(CommentResponse)` - The created comment
    /// * `Err(Error::NotFound)` - Post not found
    /// * `Err(Error)` - Database error
    pub async fn create_comment(
        &self,
        user_id: uuid::Uuid,
        create: CreateComment,
    ) -> Result<CommentResponse> {
        let comment = Comment::from_user(create.post_id, user_id, create.content);
        let saved = self.comment_repo.create_comment(comment).await?;
        self.build_response(&saved).await
    }

    /// Create a comment from a GitHub OAuth user
    ///
    /// This performs the full GitHub OAuth flow:
    /// 1. Exchange authorization code for access token
    /// 2. Get user information from GitHub
    /// 3. Create and save the comment
    ///
    /// # Arguments
    /// * `create` - The comment creation request with GitHub auth code
    ///
    /// # Returns
    /// * `Ok(CommentResponse)` - The created comment
    /// * `Err(Error::Validation)` - Invalid GitHub response
    /// * `Err(Error::Internal)` - GitHub API error
    /// * `Err(Error)` - Database error
    pub async fn create_comment_github(
        &self,
        create: CreateCommentGitHub,
    ) -> Result<CommentResponse> {
        // Step 1: Exchange code for access token
        let client = Client::new();
        let token_response: GitHubTokenResponse = client
            .post("https://github.com/login/oauth/access_token")
            .form(&[
                ("client_id", &self.github_client_id),
                ("client_secret", &self.github_client_secret),
                ("code", &create.github_code),
            ])
            .header("Accept", "application/json")
            .send()
            .await
            .map_err(|e| Error::Internal(format!("GitHub API error: {}", e)))?
            .json()
            .await
            .map_err(|e| Error::Internal(format!("GitHub API error: {}", e)))?;

        // Step 2: Get user information
        let github_user: GitHubUser = client
            .get("https://api.github.com/user")
            .header(
                "Authorization",
                format!("Bearer {}", token_response.access_token),
            )
            .header("User-Agent", "peng-blog")
            .send()
            .await
            .map_err(|e| Error::Internal(format!("GitHub API error: {}", e)))?
            .json()
            .await
            .map_err(|e| Error::Internal(format!("GitHub API error: {}", e)))?;

        // Step 3: Create comment
        let comment = Comment::from_github(create.post_id, &github_user, create.content);
        let saved = self.comment_repo.create_comment(comment).await?;
        self.build_response(&saved).await
    }

    /// Get a comment by ID
    ///
    /// # Arguments
    /// * `id` - The comment ID
    ///
    /// # Returns
    /// * `Ok(Some(CommentResponse))` - Comment if found
    /// * `Ok(None)` - Comment not found
    /// * `Err(Error)` - Database error
    pub async fn get_comment(&self, id: uuid::Uuid) -> Result<Option<CommentResponse>> {
        let comment = self.comment_repo.get_comment(id).await?;
        match comment {
            Some(c) => Ok(Some(self.build_response(&c).await?)),
            None => Ok(None),
        }
    }

    /// List comments for a post
    ///
    /// # Arguments
    /// * `post_id` - The post ID
    /// * `limit` - Maximum number of comments to return
    ///
    /// # Returns
    /// * `Ok(Vec<CommentResponse>)` - List of comments
    /// * `Err(Error)` - Database error
    pub async fn list_post_comments(
        &self,
        post_id: uuid::Uuid,
        limit: u64,
    ) -> Result<Vec<CommentResponse>> {
        let comments = self.comment_repo.list_post_comments(post_id, limit).await?;

        let mut responses = Vec::new();
        for comment in comments {
            responses.push(self.build_response(&comment).await?);
        }

        Ok(responses)
    }

    /// Update a comment
    ///
    /// # Arguments
    /// * `id` - The comment ID
    /// * `user_id` - The ID of the user (for ownership verification)
    /// * `content` - New comment content
    ///
    /// # Returns
    /// * `Ok(CommentResponse)` - Updated comment
    /// * `Err(Error::NotFound)` - Comment not found
    /// * `Err(Error::Validation)` - User doesn't own the comment
    /// * `Err(Error)` - Database error
    pub async fn update_comment(
        &self,
        id: uuid::Uuid,
        user_id: Option<uuid::Uuid>,
        is_github_user: bool,
        content: String,
    ) -> Result<CommentResponse> {
        let mut comment = self
            .comment_repo
            .get_comment(id)
            .await?
            .ok_or_else(|| Error::NotFound("Comment not found".to_string()))?;

        self.verify_ownership(&comment, user_id, is_github_user)?;
        comment.update_content(content);
        let updated = self.comment_repo.update_comment(comment).await?;
        self.build_response(&updated).await
    }

    /// Delete a comment
    ///
    /// # Arguments
    /// * `id` - The comment ID
    /// * `user_id` - The ID of the user (for ownership verification)
    /// * `is_github_user` - Whether the commenter is a GitHub user
    ///
    /// # Returns
    /// * `Ok(())` - Comment deleted
    /// * `Err(Error::NotFound)` - Comment not found
    /// * `Err(Error::Validation)` - User doesn't own the comment
    /// * `Err(Error)` - Database error
    pub async fn delete_comment(
        &self,
        id: uuid::Uuid,
        user_id: Option<uuid::Uuid>,
        is_github_user: bool,
    ) -> Result<()> {
        let comment = self
            .comment_repo
            .get_comment(id)
            .await?
            .ok_or_else(|| Error::NotFound("Comment not found".to_string()))?;

        self.verify_ownership(&comment, user_id, is_github_user)?;
        self.comment_repo
            .delete_comment(id, user_id, is_github_user)
            .await
    }

    /// Build comment response with username
    ///
    /// This fills in the username for registered users by querying the user repository.
    async fn build_response(&self, comment: &Comment) -> Result<CommentResponse> {
        let mut response = CommentResponse::from(comment);

        // Fill in username for registered users
        if !response.is_github_user {
            if let Some(user_id) = comment.user_id {
                let user = self
                    .user_repo
                    .find_by_id(user_id)
                    .await?
                    .ok_or_else(|| Error::NotFound("User not found".to_string()))?;
                response.username = user.username;
            }
        }

        Ok(response)
    }

    /// Verify ownership of a comment
    ///
    /// Checks that the provided user credentials match the comment owner.
    fn verify_ownership(
        &self,
        comment: &Comment,
        user_id: Option<uuid::Uuid>,
        is_github_user: bool,
    ) -> Result<()> {
        if is_github_user {
            if !comment.is_from_github() {
                return Err(Error::Validation("Not a GitHub user comment".to_string()));
            }
        } else if let Some(uid) = user_id {
            if !comment.is_owned_by(uid) {
                return Err(Error::Validation("You don't own this comment".to_string()));
            }
        } else {
            return Err(Error::Validation("Invalid user".to_string()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::{CommentRepository, UserRepository};
    use async_trait::async_trait;
    use domain::{Comment, Result, User};
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::RwLock;

    // Mock repositories for testing
    struct MockCommentRepo {
        comments: Arc<RwLock<HashMap<uuid::Uuid, Comment>>>,
    }

    #[async_trait]
    impl CommentRepository for MockCommentRepo {
        async fn create_comment(&self, comment: Comment) -> Result<Comment> {
            let mut comments = self.comments.write().await;
            comments.insert(comment.id, comment.clone());
            Ok(comment)
        }

        async fn get_comment(&self, id: uuid::Uuid) -> Result<Option<Comment>> {
            let comments = self.comments.read().await;
            Ok(comments.get(&id).cloned())
        }

        async fn list_post_comments(
            &self,
            _post_id: uuid::Uuid,
            _limit: u64,
        ) -> Result<Vec<Comment>> {
            Ok(Vec::new())
        }

        async fn update_comment(&self, comment: Comment) -> Result<Comment> {
            let mut comments = self.comments.write().await;
            comments.insert(comment.id, comment.clone());
            Ok(comment)
        }

        async fn delete_comment(
            &self,
            id: uuid::Uuid,
            _user_id: Option<uuid::Uuid>,
            _is_github_user: bool,
        ) -> Result<()> {
            let mut comments = self.comments.write().await;
            comments.remove(&id);
            Ok(())
        }

        async fn get_post_comment_count(&self, _post_id: uuid::Uuid) -> Result<u64> {
            Ok(0)
        }
    }

    struct MockUserRepo {
        users: Arc<RwLock<HashMap<uuid::Uuid, User>>>,
    }

    #[async_trait]
    impl UserRepository for MockUserRepo {
        async fn create_user(
            &self,
            _username: String,
            _password: String,
            _permissions: u64,
        ) -> Result<User> {
            Ok(User::new(
                uuid::Uuid::new_v4(),
                "test".to_string(),
                "hash".to_string(),
                0,
            ))
        }

        async fn find_by_username(&self, _username: &str) -> Result<Option<User>> {
            Ok(None)
        }

        async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<User>> {
            let users = self.users.read().await;
            Ok(users.get(&id).cloned())
        }

        async fn verify_credentials(
            &self,
            _username: &str,
            _password: &str,
        ) -> Result<Option<User>> {
            Ok(None)
        }

        async fn update_permissions(
            &self,
            _user_id: uuid::Uuid,
            _permissions: u64,
        ) -> Result<User> {
            Ok(User::new(
                uuid::Uuid::new_v4(),
                "test".to_string(),
                "hash".to_string(),
                0,
            ))
        }

        async fn list_users(&self, _limit: u64) -> Result<Vec<User>> {
            Ok(Vec::new())
        }

        async fn update_password(&self, _user_id: uuid::Uuid, _new_password: String) -> Result<()> {
            Ok(())
        }

        async fn delete_user(&self, _user_id: uuid::Uuid) -> Result<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_create_comment() {
        // TODO: Implement proper test with mock data setup
        // This requires adding user to mock repo before creating comment
    }

    #[tokio::test]
    async fn test_github_auth_url() {
        let comment_repo = Arc::new(MockCommentRepo {
            comments: Arc::new(RwLock::new(HashMap::new())),
        });
        let user_repo = Arc::new(MockUserRepo {
            users: Arc::new(RwLock::new(HashMap::new())),
        });
        let service = CommentService::new(
            comment_repo,
            user_repo,
            "test_client_id".to_string(),
            "test_secret".to_string(),
        );

        let url = service.github_auth_url("random_state", "http://example.com/callback");
        assert!(url.contains("github.com/login/oauth/authorize"));
        assert!(url.contains("client_id=test_client_id"));
        assert!(url.contains("state=random_state"));
    }
}
