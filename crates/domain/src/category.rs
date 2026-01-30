use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub parent_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

impl Category {
    pub fn new(name: String, slug: String, parent_id: Option<Uuid>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            slug,
            parent_id,
            created_at: Utc::now(),
        }
    }

    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub slug: String,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub parent_id: Option<Uuid>,
}
