//! Stats Service
//!
//! This module provides business logic for statistics management.
//! Statistics include global visitor tracking and per-post view counts.
//!
//! Design Principles:
//! - Simple increment operations
//! - No special cases - all stats follow the same rules
//! - Periodic cleanup for daily resets

use domain::StatsRepository;
use chrono::Utc;
use domain::stats::{RecordViewRequest, StatsResponse};
use domain::{Result, VisitStats};
use std::sync::Arc;

/// Stats service for tracking visitors and post views
///
/// This service handles all statistics-related business logic including:
/// - Recording page views
/// - Incrementing visitor counts
/// - Getting statistics
/// - Resetting daily counters
///
/// All operations are database-backed through the StatsRepository trait.
#[derive(Clone)]
pub struct StatsService {
    stats_repo: Arc<dyn StatsRepository>,
}

impl StatsService {
    /// Create a new stats service
    ///
    /// # Arguments
    /// * `stats_repo` - The stats repository implementation (wrapped in Arc)
    pub fn new(stats_repo: Arc<dyn StatsRepository>) -> Self {
        Self { stats_repo }
    }

    /// Record a page view
    ///
    /// This increments the global visit count and optionally increments
    /// the view count for a specific post.
    ///
    /// # Arguments
    /// * `request` - The view recording request
    ///
    /// # Returns
    /// * `Ok(())` - View recorded
    /// * `Err(Error)` - Database error
    pub async fn record_view(&self, request: RecordViewRequest) -> Result<()> {
        // Determine if it's today
        let _now = Utc::now();
        let is_today = true; // Simplification - always today

        // Increment global visit count
        self.stats_repo.increment_visit(is_today).await?;

        // If it's a post view, increment post view count
        if let Some(post_id) = request.post_id {
            self.stats_repo.increment_post_view(post_id).await?;
        }

        Ok(())
    }

    /// Get global visitor statistics
    ///
    /// # Returns
    /// * `Ok(VisitStats)` - Global statistics
    /// * `Err(Error)` - Database error
    pub async fn get_visit_stats(&self) -> Result<VisitStats> {
        self.stats_repo.get_visit_stats().await
    }

    /// Get or create post statistics
    ///
    /// # Arguments
    /// * `post_id` - The post ID
    ///
    /// # Returns
    /// * `Ok(PostStats)` - Post statistics
    /// * `Err(Error)` - Database error
    pub async fn get_post_stats(&self, post_id: uuid::Uuid) -> Result<domain::PostStats> {
        self.stats_repo.get_or_create_post_stats(post_id).await
    }

    /// Reset today's visit count
    ///
    /// This should be called at midnight (or via a scheduled job) to reset
    /// the daily visit counter while preserving the total count.
    ///
    /// # Returns
    /// * `Ok(())` - Reset completed
    /// * `Err(Error)` - Database error
    pub async fn reset_today_visits(&self) -> Result<()> {
        self.stats_repo.reset_today_visits().await
    }

    /// Get total statistics (admin only)
    ///
    /// This provides aggregated statistics including total visits, today's visits,
    /// total posts, and total comments.
    ///
    /// # Returns
    /// * `Ok(StatsResponse)` - Total statistics
    /// * `Err(Error)` - Database error
    pub async fn get_total_stats(&self) -> Result<StatsResponse> {
        self.stats_repo.get_total_stats().await
    }

    /// Check if daily reset is needed
    ///
    /// This helper checks if the last_updated date in visit_stats is different
    /// from today's date, indicating that today's visits should be reset.
    ///
    /// # Returns
    /// * `Ok(true)` - Reset needed (last updated was not today)
    /// * `Ok(false)` - Reset not needed
    /// * `Err(Error)` - Database error
    pub async fn should_reset_today_visits(&self) -> Result<bool> {
        let stats = self.stats_repo.get_visit_stats().await?;
        let today = Utc::now().date_naive();
        let last_updated = stats.last_updated.date_naive();

        Ok(today != last_updated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::StatsRepository;
    use async_trait::async_trait;
    use domain::{PostStats, Result, StatsResponse, VisitStats};
    use std::sync::Arc;
    use tokio::sync::RwLock;

    // Mock repository for testing
    struct MockStatsRepo {
        visit_stats: Arc<RwLock<VisitStats>>,
        post_stats: Arc<RwLock<std::collections::HashMap<uuid::Uuid, PostStats>>>,
    }

    impl MockStatsRepo {
        fn new() -> Self {
            Self {
                visit_stats: Arc::new(RwLock::new(VisitStats::new())),
                post_stats: Arc::new(RwLock::new(std::collections::HashMap::new())),
            }
        }
    }

    #[async_trait]
    impl StatsRepository for MockStatsRepo {
        async fn get_visit_stats(&self) -> Result<VisitStats> {
            let stats = self.visit_stats.read().await;
            Ok(stats.clone())
        }

        async fn increment_visit(&self, is_today: bool) -> Result<()> {
            let mut stats = self.visit_stats.write().await;
            stats.increment(is_today);
            Ok(())
        }

        async fn reset_today_visits(&self) -> Result<()> {
            let mut stats = self.visit_stats.write().await;
            stats.reset_today();
            Ok(())
        }

        async fn get_or_create_post_stats(&self, post_id: uuid::Uuid) -> Result<PostStats> {
            let mut stats = self.post_stats.write().await;
            Ok(stats
                .entry(post_id)
                .or_insert_with(|| PostStats::new(post_id))
                .clone())
        }

        async fn increment_post_view(&self, post_id: uuid::Uuid) -> Result<()> {
            let mut stats = self.post_stats.write().await;
            if let Some(stat) = stats.get_mut(&post_id) {
                stat.increment_view();
            } else {
                let mut new_stat = PostStats::new(post_id);
                new_stat.increment_view();
                stats.insert(post_id, new_stat);
            }
            Ok(())
        }

        async fn get_total_stats(&self) -> Result<StatsResponse> {
            let visit_stats = self.visit_stats.read().await;
            Ok(StatsResponse {
                total_visits: visit_stats.total_visits,
                today_visits: visit_stats.today_visits,
                total_posts: 0,
                total_comments: 0,
            })
        }
    }

    #[tokio::test]
    async fn test_record_view() {
        let repo = Arc::new(MockStatsRepo::new());
        let service = StatsService::new(repo);

        let request = RecordViewRequest { post_id: None };

        service.record_view(request).await.unwrap();

        let stats = service.get_visit_stats().await.unwrap();
        assert_eq!(stats.total_visits, 1);
        assert_eq!(stats.today_visits, 1);
    }

    #[tokio::test]
    async fn test_record_post_view() {
        let repo = Arc::new(MockStatsRepo::new());
        let service = StatsService::new(repo);

        let post_id = uuid::Uuid::new_v4();
        let request = RecordViewRequest {
            post_id: Some(post_id),
        };

        service.record_view(request).await.unwrap();

        let post_stats = service.get_post_stats(post_id).await.unwrap();
        assert_eq!(post_stats.views, 1);
    }

    #[tokio::test]
    async fn test_reset_today_visits() {
        let repo = Arc::new(MockStatsRepo::new());
        let service = StatsService::new(repo);

        // Record some visits
        let request = RecordViewRequest { post_id: None };
        service.record_view(request.clone()).await.unwrap();
        service.record_view(request.clone()).await.unwrap();

        // Reset today's visits
        service.reset_today_visits().await.unwrap();

        let stats = service.get_visit_stats().await.unwrap();
        assert_eq!(stats.total_visits, 2);
        assert_eq!(stats.today_visits, 0);
    }

    #[tokio::test]
    async fn test_get_total_stats() {
        let repo = Arc::new(MockStatsRepo::new());
        let service = StatsService::new(repo);

        let request = RecordViewRequest { post_id: None };
        service.record_view(request).await.unwrap();

        let stats = service.get_total_stats().await.unwrap();
        assert_eq!(stats.total_visits, 1);
        assert_eq!(stats.today_visits, 1);
    }
}
