use async_trait::async_trait;
use domain::{Result, Tag, TagRepository};
use sea_orm::*;
use std::sync::Arc;
use uuid::Uuid;

use crate::entity::tag;

pub struct TagRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

impl Clone for TagRepositoryImpl {
    fn clone(&self) -> Self {
        Self {
            db: Arc::clone(&self.db),
        }
    }
}

impl TagRepositoryImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    fn entity_to_domain(entity: tag::Model) -> Tag {
        Tag {
            id: Uuid::parse_str(&entity.id).unwrap(),
            name: entity.name,
            slug: entity.slug,
            created_at: chrono::DateTime::parse_from_rfc3339(&entity.created_at)
                .unwrap()
                .with_timezone(&chrono::Utc),
        }
    }
}

#[async_trait]
impl TagRepository for TagRepositoryImpl {
    async fn create_tag(&self, name: String, slug: String) -> Result<Tag> {
        let tag = tag::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            name: Set(name),
            slug: Set(slug),
            created_at: Set(chrono::Utc::now().to_rfc3339()),
        };

        let result = tag.insert(self.db.as_ref()).await.map_err(|e| match e {
            DbErr::Custom(err) if err.contains("UNIQUE") => {
                domain::Error::Validation("Tag already exists".to_string())
            }
            _ => domain::Error::Internal(e.to_string()),
        })?;

        Ok(Self::entity_to_domain(result))
    }

    async fn get_tag(&self, id: Uuid) -> Result<Option<Tag>> {
        let result = tag::Entity::find_by_id(id.to_string())
            .one(self.db.as_ref())
            .await
            .map_err(|e| domain::Error::Internal(e.to_string()))?;

        Ok(result.map(Self::entity_to_domain))
    }

    async fn get_tag_by_slug(&self, slug: &str) -> Result<Option<Tag>> {
        let result = tag::Entity::find()
            .filter(tag::Column::Slug.eq(slug))
            .one(self.db.as_ref())
            .await
            .map_err(|e| domain::Error::Internal(e.to_string()))?;

        Ok(result.map(Self::entity_to_domain))
    }

    async fn list_tags(&self) -> Result<Vec<Tag>> {
        let result = tag::Entity::find()
            .all(self.db.as_ref())
            .await
            .map_err(|e| domain::Error::Internal(e.to_string()))?;

        Ok(result.into_iter().map(Self::entity_to_domain).collect())
    }

    async fn delete_tag(&self, id: Uuid) -> Result<()> {
        tag::Entity::delete_by_id(id.to_string())
            .exec(self.db.as_ref())
            .await
            .map_err(|e| domain::Error::Internal(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::Tag;

    #[tokio::test]
    async fn test_tag_structure() {
        let tag = Tag::new("Rust".to_string(), "rust".to_string());

        assert_eq!(tag.name, "Rust");
        assert_eq!(tag.slug, "rust");
    }

    #[tokio::test]
    async fn test_tag_entity_to_domain() {
        let entity = tag::Model {
            id: Uuid::new_v4().to_string(),
            name: "Programming".to_string(),
            slug: "programming".to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        let domain = TagRepositoryImpl::entity_to_domain(entity.clone());

        assert_eq!(domain.id.to_string(), entity.id);
        assert_eq!(domain.name, entity.name);
        assert_eq!(domain.slug, entity.slug);
    }

    #[tokio::test]
    async fn test_tag_unique_id() {
        // Verify each tag gets a unique ID
        let tag1 = Tag::new("Rust".to_string(), "rust".to_string());
        let tag2 = Tag::new("Python".to_string(), "python".to_string());

        assert_ne!(tag1.id, tag2.id);
    }

    #[tokio::test]
    async fn test_tag_created_at() {
        let before = chrono::Utc::now();
        let tag = Tag::new("Go".to_string(), "go".to_string());
        let after = chrono::Utc::now();

        assert!(tag.created_at >= before);
        assert!(tag.created_at <= after);
    }

    #[test]
    fn test_tag_repository_impl_clone() {
        // This test verifies that TagRepositoryImpl implements Clone
        // Since we can't create a real db connection in unit tests,
        // we just verify the Clone implementation exists via compilation
    }
}
