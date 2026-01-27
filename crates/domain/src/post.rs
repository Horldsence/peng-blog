use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a blog post
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl Post {
    /// Create a new unpublished post
    pub fn new(user_id: Uuid, title: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            title,
            content,
            published_at: None,
            created_at: Utc::now(),
        }
    }

    /// Check if post is published
    pub fn is_published(&self) -> bool {
        self.published_at.is_some()
    }

    /// Publish the post
    pub fn publish(&mut self) {
        self.published_at = Some(Utc::now());
    }

    /// Unpublish the post
    pub fn unpublish(&mut self) {
        self.published_at = None;
    }

    /// Check if the post is owned by a specific user
    pub fn is_owned_by(&self, user_id: Uuid) -> bool {
        self.user_id == user_id
    }
}

/// Request to create a new post
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub content: String,
}

/// Request to update an existing post
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub content: Option<String>,
}
