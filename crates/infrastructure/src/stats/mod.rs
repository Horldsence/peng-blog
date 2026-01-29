//! Stats Repository Implementation
//!
//! This module provides a concrete implementation of StatsRepository
//! using SeaORM for database operations.
//!
//! Design Principles:
//! - Simple CRUD operations
//! - Clear error mapping
//! - No special cases

use crate::entity::{stats, post_stats};
use crate::entity::prelude::*;
use async_trait::async_trait;
use domain::{Error, Result, PostStats, VisitStats, StatsResponse};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use sea_orm::prelude::Expr;
use service::StatsRepository;
use std::sync::Arc;

/// Concrete implementation of StatsRepository
///
/// This implementation uses SeaORM to interact with visit_stats and post_stats
/// tables in the database.
pub struct StatsRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

impl Clone for StatsRepositoryImpl {
    fn clone(&self) -> Self {
        Self {
            db: Arc::clone(&self.db),
        }
    }
}

impl StatsRepositoryImpl {
    /// Create a new stats repository
    ///
    /// # Arguments
    /// * `db` - Database connection
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl StatsRepository for StatsRepositoryImpl {
    /// Get global visitor statistics
    async fn get_visit_stats(&self) -> Result<VisitStats> {
        let model = VisitStatsEntity::find_by_id(1)
            .one(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to get visit stats: {}", e)))?
            .ok_or_else(|| Error::Internal("Visit stats not found. Run migrations.".to_string()))?;

        Ok(VisitStats {
            total_visits: model.total_visits as u64,
            today_visits: model.today_visits as u64,
            last_updated: model.last_updated.parse().map_err(|e| {
                Error::Internal(format!("Invalid last_updated in database: {}", e))
            })?,
        })
    }

    /// Increment visitor count
    async fn increment_visit(&self, is_today: bool) -> Result<()> {
        let stats = VisitStatsEntity::find_by_id(1)
            .one(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to find visit stats: {}", e)))?
            .ok_or_else(|| Error::Internal("Visit stats not found. Run migrations.".to_string()))?;

        let mut active_model: stats::ActiveModel = stats.into();

        let new_total = active_model.total_visits.clone().unwrap() + 1;
        active_model.total_visits = Set(new_total);
        if is_today {
            let new_today = active_model.today_visits.clone().unwrap() + 1;
            active_model.today_visits = Set(new_today);
        }
        active_model.last_updated = Set(chrono::Utc::now().to_rfc3339());

        active_model
            .update(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to increment visit stats: {}", e)))?;

        Ok(())
    }

    /// Reset today's visit count
    async fn reset_today_visits(&self) -> Result<()> {
        let stats = VisitStatsEntity::find_by_id(1)
            .one(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to find visit stats: {}", e)))?
            .ok_or_else(|| Error::Internal("Visit stats not found. Run migrations.".to_string()))?;

        let mut active_model: stats::ActiveModel = stats.into();
        active_model.today_visits = Set(0);
        active_model.last_updated = Set(chrono::Utc::now().to_rfc3339());

        active_model
            .update(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to reset today's visits: {}", e)))?;

        Ok(())
    }

    /// Get or create post statistics
    async fn get_or_create_post_stats(&self, post_id: uuid::Uuid) -> Result<PostStats> {
        // Try to get existing stats
        if let Some(model) = PostStatsEntity::find()
            .filter(post_stats::Column::PostId.eq(post_id.to_string()))
            .one(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to get post stats: {}", e)))?
        {
            return Ok(PostStats {
                post_id: uuid::Uuid::parse_str(&model.post_id).map_err(|e| {
                    Error::Internal(format!("Invalid post_id in database: {}", e))
                })?,
                views: model.views as u64,
                last_viewed_at: model.last_viewed_at.parse().map_err(|e| {
                    Error::Internal(format!("Invalid last_viewed_at in database: {}", e))
                })?,
            });
        }

        // Create new stats record
        let new_stats = PostStats::new(post_id);
        let active_model = post_stats::ActiveModel {
            id: Set(uuid::Uuid::new_v4().to_string()),
            post_id: Set(new_stats.post_id.to_string()),
            views: Set(new_stats.views as i64),
            last_viewed_at: Set(new_stats.last_viewed_at.to_rfc3339()),
        };

        active_model
            .insert(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to create post stats: {}", e)))?;

        Ok(new_stats)
    }

    /// Increment post view count
    async fn increment_post_view(&self, post_id: uuid::Uuid) -> Result<()> {
        // Ensure stats record exists
        let _stats = self.get_or_create_post_stats(post_id).await?;

        // Increment views
        PostStatsEntity::update_many()
            .filter(post_stats::Column::PostId.eq(post_id.to_string()))
            .col_expr(post_stats::Column::Views, Expr::col(post_stats::Column::Views).add(1))
            .exec(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to increment post view: {}", e)))?;

        // Update last_viewed_at
        let model = PostStatsEntity::find()
            .filter(post_stats::Column::PostId.eq(post_id.to_string()))
            .one(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to find post stats: {}", e)))?
            .ok_or_else(|| Error::Internal("Post stats not found after increment".to_string()))?;

        let mut active_model: post_stats::ActiveModel = model.into();
        active_model.last_viewed_at = Set(chrono::Utc::now().to_rfc3339());

        active_model
            .update(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to update post stats: {}", e)))?;

        Ok(())
    }

    /// Get total statistics (admin only)
    async fn get_total_stats(&self) -> Result<StatsResponse> {
        let visit_stats = self.get_visit_stats().await?;

        // Get total posts count
        let total_posts = PostEntity::find()
            .count(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to count posts: {}", e)))? as u64;

        // Get total comments count
        let total_comments = CommentEntity::find()
            .count(&*self.db)
            .await
            .map_err(|e| Error::Internal(format!("Failed to count comments: {}", e)))? as u64;

        Ok(StatsResponse {
            total_visits: visit_stats.total_visits,
            today_visits: visit_stats.today_visits,
            total_posts,
            total_comments,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_post_stats_structure() {
        let post_id = uuid::Uuid::new_v4();

        let mut stats = PostStats::new(post_id);
        assert_eq!(stats.post_id, post_id);
        assert_eq!(stats.views, 0);

        stats.increment_view();
        assert_eq!(stats.views, 1);

        stats.increment_view();
        assert_eq!(stats.views, 2);
    }

    #[tokio::test]
    async fn test_visit_stats_structure() {
        let mut stats = VisitStats::new();
        assert_eq!(stats.total_visits, 0);
        assert_eq!(stats.today_visits, 0);

        stats.increment(true);
        assert_eq!(stats.total_visits, 1);
        assert_eq!(stats.today_visits, 1);

        stats.increment(false);
        assert_eq!(stats.total_visits, 2);
        assert_eq!(stats.today_visits, 1);

        stats.reset_today();
        assert_eq!(stats.total_visits, 2);
        assert_eq!(stats.today_visits, 0);
    }
}
