use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Global visitor statistics
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VisitStats {
    pub total_visits: u64,
    pub today_visits: u64,
    pub last_updated: DateTime<Utc>,
}

impl Default for VisitStats {
    fn default() -> Self {
        Self::new()
    }
}

impl VisitStats {
    /// Create new stats with zero visits
    pub fn new() -> Self {
        Self {
            total_visits: 0,
            today_visits: 0,
            last_updated: Utc::now(),
        }
    }

    /// Increment visit counters
    pub fn increment(&mut self, is_today: bool) {
        self.total_visits += 1;
        if is_today {
            self.today_visits += 1;
        }
        self.last_updated = Utc::now();
    }

    /// Reset today's visits (called at midnight)
    pub fn reset_today(&mut self) {
        self.today_visits = 0;
        self.last_updated = Utc::now();
    }
}

/// Statistics for a single post
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostStats {
    pub post_id: Uuid,
    pub views: u64,
    pub last_viewed_at: DateTime<Utc>,
}

impl PostStats {
    /// Create new stats for a post
    pub fn new(post_id: Uuid) -> Self {
        Self {
            post_id,
            views: 0,
            last_viewed_at: Utc::now(),
        }
    }

    /// Increment view count
    pub fn increment_view(&mut self) {
        self.views += 1;
        self.last_viewed_at = Utc::now();
    }
}

/// Daily statistics snapshot
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DailyStats {
    pub date: NaiveDate,
    pub visits: u64,
    pub unique_visitors: u64,
    pub page_views: u64,
}

impl DailyStats {
    /// Create new daily stats
    pub fn new(date: NaiveDate) -> Self {
        Self {
            date,
            visits: 0,
            unique_visitors: 0,
            page_views: 0,
        }
    }

    /// Increment visit counter
    pub fn increment_visits(&mut self) {
        self.visits += 1;
        self.page_views += 1;
    }
}

/// Response for statistics query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsResponse {
    pub total_visits: u64,
    pub today_visits: u64,
    pub total_posts: u64,
    pub total_comments: u64,
}

/// Request to record a page view
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordViewRequest {
    pub post_id: Option<Uuid>, // None for homepage, Some for specific post
}
