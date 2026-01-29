use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a comment on a post
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Option<Uuid>,          // None for GitHub users
    pub github_username: Option<String>, // Set for GitHub users
    pub github_avatar_url: Option<String>,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Comment {
    /// Create a new comment from a registered user
    pub fn from_user(post_id: Uuid, user_id: Uuid, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            post_id,
            user_id: Some(user_id),
            github_username: None,
            github_avatar_url: None,
            content,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Create a new comment from GitHub user
    pub fn from_github(
        post_id: Uuid,
        github_user: &GitHubUser,
        content: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            post_id,
            user_id: None,
            github_username: Some(github_user.login.clone()),
            github_avatar_url: Some(github_user.avatar_url.clone()),
            content,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Update comment content
    pub fn update_content(&mut self, content: String) {
        self.content = content;
        self.updated_at = Utc::now();
    }

    /// Check if comment is owned by user
    pub fn is_owned_by(&self, user_id: Uuid) -> bool {
        self.user_id == Some(user_id)
    }

    /// Check if comment is from GitHub user
    pub fn is_from_github(&self) -> bool {
        self.user_id.is_none() && self.github_username.is_some()
    }
}

/// Request to create a comment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateComment {
    pub post_id: Uuid,
    pub content: String,
}

/// Request to create comment with GitHub user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCommentGitHub {
    pub post_id: Uuid,
    pub github_code: String,  // OAuth authorization code
    pub content: String,
}

/// Response for comment operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentResponse {
    pub id: Uuid,
    pub post_id: Uuid,
    pub username: String,           // Either user.username or github_username
    pub avatar_url: Option<String>, // User avatar or GitHub avatar
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub is_github_user: bool,
}

impl From<&Comment> for CommentResponse {
    fn from(comment: &Comment) -> Self {
        let (username, avatar_url, is_github_user) = if let Some(github_username) = &comment.github_username {
            (github_username.clone(), comment.github_avatar_url.clone(), true)
        } else {
            // For registered users, username will be filled by service layer
            (String::new(), None, false)
        };

        Self {
            id: comment.id,
            post_id: comment.post_id,
            username,
            avatar_url,
            content: comment.content.clone(),
            created_at: comment.created_at,
            updated_at: comment.updated_at,
            is_github_user,
        }
    }
}

/// GitHub user information from OAuth
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubUser {
    pub id: u64,
    pub login: String,
    pub avatar_url: String,
    pub name: Option<String>,
    pub email: Option<String>,
}

/// GitHub OAuth token response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
}

/// Request to get GitHub authorization URL
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubAuthRequest {
    pub state: String, // CSRF protection
}