use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tag {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
}

impl Tag {
    pub fn new(name: String, slug: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            slug,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTag {
    pub name: String,
    pub slug: String,
}
