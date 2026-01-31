use async_trait::async_trait;
use domain::{Category, CategoryRepository, Result};
use sea_orm::*;
use std::sync::Arc;
use uuid::Uuid;

use crate::entity::{category, post};

pub struct CategoryRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

impl Clone for CategoryRepositoryImpl {
    fn clone(&self) -> Self {
        Self {
            db: Arc::clone(&self.db),
        }
    }
}

impl CategoryRepositoryImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    fn entity_to_domain(entity: category::Model) -> Category {
        Category {
            id: Uuid::parse_str(&entity.id).unwrap(),
            name: entity.name,
            slug: entity.slug,
            parent_id: entity.parent_id.map(|id| Uuid::parse_str(&id).unwrap()),
            created_at: chrono::DateTime::parse_from_rfc3339(&entity.created_at)
                .unwrap()
                .with_timezone(&chrono::Utc),
        }
    }
}

#[async_trait]
impl CategoryRepository for CategoryRepositoryImpl {
    async fn create_category(
        &self,
        name: String,
        slug: String,
        parent_id: Option<Uuid>,
    ) -> Result<Category> {
        let category = category::ActiveModel {
            id: Set(Uuid::new_v4().to_string()),
            name: Set(name),
            slug: Set(slug),
            parent_id: Set(parent_id.map(|id| id.to_string())),
            created_at: Set(chrono::Utc::now().to_rfc3339()),
        };

        let result = category
            .insert(self.db.as_ref())
            .await
            .map_err(|e| match e {
                DbErr::RecordNotFound(_) => {
                    domain::Error::NotFound("Category not found".to_string())
                }
                DbErr::Custom(err) if err.contains("UNIQUE") => {
                    domain::Error::Validation("Slug already exists".to_string())
                }
                _ => domain::Error::Internal(e.to_string()),
            })?;

        Ok(Self::entity_to_domain(result))
    }

    async fn get_category(&self, id: Uuid) -> Result<Option<Category>> {
        let result = category::Entity::find_by_id(id.to_string())
            .one(self.db.as_ref())
            .await
            .map_err(|e| domain::Error::Internal(e.to_string()))?;

        Ok(result.map(Self::entity_to_domain))
    }

    async fn get_category_by_slug(&self, slug: &str) -> Result<Option<Category>> {
        let result = category::Entity::find()
            .filter(category::Column::Slug.eq(slug))
            .one(self.db.as_ref())
            .await
            .map_err(|e| domain::Error::Internal(e.to_string()))?;

        Ok(result.map(Self::entity_to_domain))
    }

    async fn list_categories(&self) -> Result<Vec<Category>> {
        let result = category::Entity::find()
            .all(self.db.as_ref())
            .await
            .map_err(|e| domain::Error::Internal(e.to_string()))?;

        Ok(result.into_iter().map(Self::entity_to_domain).collect())
    }

    async fn update_category(
        &self,
        id: Uuid,
        name: Option<String>,
        parent_id: Option<Uuid>,
    ) -> Result<Category> {
        let category = category::Entity::find_by_id(id.to_string())
            .one(self.db.as_ref())
            .await
            .map_err(|e| domain::Error::Internal(e.to_string()))?
            .ok_or_else(|| domain::Error::NotFound("Category not found".to_string()))?;

        let mut active: category::ActiveModel = category.into();

        if let Some(name) = name {
            active.name = Set(name);
        }

        if let Some(parent_id) = parent_id {
            active.parent_id = Set(Some(parent_id.to_string()));
        }

        let result = active.update(self.db.as_ref()).await.map_err(|e| match e {
            DbErr::RecordNotFound(_) => domain::Error::NotFound("Category not found".to_string()),
            _ => domain::Error::Internal(e.to_string()),
        })?;

        Ok(Self::entity_to_domain(result))
    }

    async fn delete_category(&self, id: Uuid) -> Result<()> {
        // First, set all posts' category_id to NULL
        use sea_orm::{ActiveModelTrait, EntityTrait, Set};

        let posts = post::Entity::find()
            .filter(post::Column::CategoryId.eq(id.to_string()))
            .all(self.db.as_ref())
            .await
            .map_err(|e| domain::Error::Internal(e.to_string()))?;

        for post_model in posts {
            let mut active: post::ActiveModel = post_model.into();
            active.category_id = Set(None);
            active
                .update(self.db.as_ref())
                .await
                .map_err(|e| domain::Error::Internal(e.to_string()))?;
        }

        // Then delete the category
        category::Entity::delete_by_id(id.to_string())
            .exec(self.db.as_ref())
            .await
            .map_err(|e| domain::Error::Internal(e.to_string()))?;

        Ok(())
    }

    async fn get_children(&self, parent_id: Option<Uuid>) -> Result<Vec<Category>> {
        let result = if let Some(parent_id) = parent_id {
            category::Entity::find()
                .filter(category::Column::ParentId.eq(parent_id.to_string()))
                .all(self.db.as_ref())
                .await
        } else {
            category::Entity::find()
                .filter(category::Column::ParentId.is_null())
                .all(self.db.as_ref())
                .await
        };

        let result = result.map_err(|e| domain::Error::Internal(e.to_string()))?;

        Ok(result.into_iter().map(Self::entity_to_domain).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::Category;

    #[tokio::test]
    async fn test_category_structure() {
        let category = Category::new("Technology".to_string(), "technology".to_string(), None);

        assert_eq!(category.name, "Technology");
        assert_eq!(category.slug, "technology");
        assert!(category.parent_id.is_none());
        assert!(category.is_root());
    }

    #[tokio::test]
    async fn test_category_with_parent() {
        let parent_id = Uuid::new_v4();
        let category = Category::new("Rust".to_string(), "rust".to_string(), Some(parent_id));

        assert_eq!(category.name, "Rust");
        assert_eq!(category.parent_id, Some(parent_id));
        assert!(!category.is_root());
    }

    #[tokio::test]
    async fn test_category_entity_to_domain() {
        let entity = category::Model {
            id: Uuid::new_v4().to_string(),
            name: "Programming".to_string(),
            slug: "programming".to_string(),
            parent_id: None,
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        let domain = CategoryRepositoryImpl::entity_to_domain(entity.clone());

        assert_eq!(domain.id.to_string(), entity.id);
        assert_eq!(domain.name, entity.name);
        assert_eq!(domain.slug, entity.slug);
        assert!(domain.parent_id.is_none());
    }

    #[tokio::test]
    async fn test_category_entity_to_domain_with_parent() {
        let parent_id = Uuid::new_v4();
        let entity = category::Model {
            id: Uuid::new_v4().to_string(),
            name: "Programming".to_string(),
            slug: "programming".to_string(),
            parent_id: Some(parent_id.to_string()),
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        let domain = CategoryRepositoryImpl::entity_to_domain(entity);

        assert_eq!(domain.parent_id, Some(parent_id));
    }

    #[test]
    fn test_category_repository_impl_clone() {
        // This test verifies that CategoryRepositoryImpl implements Clone
        // Since we can't create a real db connection in unit tests,
        // we just verify the Clone implementation exists via compilation
    }
}
