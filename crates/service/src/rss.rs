//! # RSS Service - RSS Feed Generation
//!
//! This service handles RSS feed generation for blog posts.
//! It provides an interface for generating RSS feeds from published posts.

use async_trait::async_trait;
use domain::{Post, PostRepository, Result};
use rss::{ChannelBuilder, Guid, ItemBuilder};
use std::sync::Arc;
use tokio::sync::RwLock;

const DEFAULT_FEED_LIMIT: usize = 20;
const DEFAULT_FEED_TITLE: &str = "Peng Blog";
const DEFAULT_FEED_DESCRIPTION: &str = "Latest posts from Peng Blog";

/// Trait for RSS feed generation operations
#[async_trait]
pub trait RssService: Send + Sync {
    /// Generate RSS feed with all published posts
    ///
    /// Returns the RSS feed as an XML string
    async fn generate_rss(&self) -> Result<String>;

    /// Generate RSS feed with a limit on the number of posts
    ///
    /// # Arguments
    ///
    /// * `limit` - Maximum number of posts to include in the feed
    ///
    /// Returns the RSS feed as an XML string
    async fn generate_rss_with_limit(&self, limit: usize) -> Result<String>;

    /// Refresh the RSS feed cache
    ///
    /// This should be called whenever posts are created, updated, or deleted
    async fn refresh_cache(&self) -> Result<()>;
}

/// RSS service implementation
///
/// This service generates RSS feeds from published posts.
/// It caches the generated feed to improve performance.
pub struct RssServiceImpl {
    post_repo: Arc<dyn PostRepository>,
    base_url: String,
    feed_title: String,
    feed_description: String,
    cache: Arc<RwLock<Option<String>>>,
}

impl RssServiceImpl {
    /// Create a new RSS service
    ///
    /// # Arguments
    ///
    /// * `post_repo` - Post repository for fetching published posts
    /// * `base_url` - Base URL of the blog (e.g., "http://localhost:3000")
    /// * `feed_title` - Title for the RSS feed
    /// * `feed_description` - Description for the RSS feed
    pub fn new(
        post_repo: Arc<dyn PostRepository>,
        base_url: String,
        feed_title: Option<String>,
        feed_description: Option<String>,
    ) -> Self {
        Self {
            post_repo,
            base_url,
            feed_title: feed_title.unwrap_or_else(|| DEFAULT_FEED_TITLE.to_string()),
            feed_description: feed_description
                .unwrap_or_else(|| DEFAULT_FEED_DESCRIPTION.to_string()),
            cache: Arc::new(RwLock::new(None)),
        }
    }

    /// Generate RSS channel from posts
    fn build_channel(&self, posts: Vec<Post>) -> Result<String> {
        let mut channel = ChannelBuilder::default()
            .title(&self.feed_title)
            .link(&self.base_url)
            .description(&self.feed_description)
            .build();

        for post in posts {
            let post_url = format!("{}/posts/{}", self.base_url, post.id);

            let item = ItemBuilder::default()
                .title(post.title)
                .link(post_url)
                .content(post.content)
                .pub_date(post.published_at.unwrap_or(post.created_at).to_rfc2822())
                .guid(Guid {
                    value: format!("{}", post.id),
                    permalink: false,
                })
                .build();

            channel.items.push(item);
        }

        Ok(channel.to_string())
    }
}

#[async_trait]
impl RssService for RssServiceImpl {
    /// Generate RSS feed with all published posts
    async fn generate_rss(&self) -> Result<String> {
        {
            let cache = self.cache.read().await;
            if let Some(ref cached) = *cache {
                return Ok(cached.clone());
            }
        }

        let posts = self
            .post_repo
            .list_published_posts(DEFAULT_FEED_LIMIT as u64)
            .await?;

        let feed = self.build_channel(posts)?;

        {
            let mut cache = self.cache.write().await;
            *cache = Some(feed.clone());
        }

        Ok(feed)
    }

    /// Generate RSS feed with a limit on the number of posts
    async fn generate_rss_with_limit(&self, limit: usize) -> Result<String> {
        let posts = self.post_repo.list_published_posts(limit as u64).await?;

        self.build_channel(posts)
    }

    /// Refresh the RSS feed cache
    async fn refresh_cache(&self) -> Result<()> {
        {
            let mut cache = self.cache.write().await;
            *cache = None;
        }

        self.generate_rss().await?;

        Ok(())
    }
}
