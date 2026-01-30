use crate::CategoryRepository;
use domain::{Category, CreateCategory, Result, UpdateCategory};
use std::sync::Arc;
use uuid::Uuid;

/// Service for category business logic
#[derive(Clone)]
pub struct CategoryService {
    repo: Arc<dyn CategoryRepository>,
}

impl CategoryService {
    pub fn new(repo: Arc<dyn CategoryRepository>) -> Self {
        Self { repo }
    }

    pub async fn create(&self, input: CreateCategory) -> Result<Category> {
        self.validate_slug(&input.slug)?;
        self.validate_name(&input.name)?;

        if let Some(parent_id) = input.parent_id {
            self.repo.get_category(parent_id).await?.ok_or_else(|| {
                domain::Error::Validation("Parent category not found".to_string())
            })?;

            if parent_id == input.parent_id.unwrap() {
                return Err(domain::Error::Validation(
                    "Category cannot be its own parent".to_string(),
                ));
            }
        }

        self.repo
            .create_category(input.name, input.slug, input.parent_id)
            .await
    }

    pub async fn get(&self, id: Uuid) -> Result<Category> {
        self.repo
            .get_category(id)
            .await?
            .ok_or_else(|| domain::Error::NotFound("Category not found".to_string()))
    }

    pub async fn get_by_slug(&self, slug: &str) -> Result<Category> {
        self.repo
            .get_category_by_slug(slug)
            .await?
            .ok_or_else(|| domain::Error::NotFound("Category not found".to_string()))
    }

    pub async fn list(&self) -> Result<Vec<Category>> {
        self.repo.list_categories().await
    }

    pub async fn update(&self, id: Uuid, input: UpdateCategory) -> Result<Category> {
        if let Some(parent_id) = input.parent_id {
            if parent_id == id {
                return Err(domain::Error::Validation(
                    "Category cannot be its own parent".to_string(),
                ));
            }

            self.repo.get_category(parent_id).await?.ok_or_else(|| {
                domain::Error::Validation("Parent category not found".to_string())
            })?;
        }

        if let Some(ref name) = input.name {
            self.validate_name(name)?;
        }

        self.repo
            .update_category(id, input.name, input.parent_id)
            .await
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        self.repo.delete_category(id).await
    }

    pub async fn get_children(&self, parent_id: Option<Uuid>) -> Result<Vec<Category>> {
        self.repo.get_children(parent_id).await
    }
}

impl CategoryService {
    fn validate_slug(&self, slug: &str) -> Result<()> {
        if slug.trim().is_empty() {
            return Err(domain::Error::Validation(
                "Slug cannot be empty".to_string(),
            ));
        }

        if !slug
            .chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return Err(domain::Error::Validation(
                "Slug can only contain letters, numbers, hyphens and underscores".to_string(),
            ));
        }

        Ok(())
    }

    fn validate_name(&self, name: &str) -> Result<()> {
        if name.trim().is_empty() {
            return Err(domain::Error::Validation(
                "Name cannot be empty".to_string(),
            ));
        }

        if name.len() > 100 {
            return Err(domain::Error::Validation(
                "Name too long (max 100 characters)".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::Error;
    use mockall::{mock, predicate::*};

    mock! {
        CategoryRepo {}

        #[async_trait::async_trait]
        impl CategoryRepository for CategoryRepo {
            async fn create_category(&self, name: String, slug: String, parent_id: Option<Uuid>) -> Result<Category>;
            async fn get_category(&self, id: Uuid) -> Result<Option<Category>>;
            async fn get_category_by_slug(&self, slug: &str) -> Result<Option<Category>>;
            async fn list_categories(&self) -> Result<Vec<Category>>;
            async fn update_category(&self, id: Uuid, name: Option<String>, parent_id: Option<Uuid>) -> Result<Category>;
            async fn delete_category(&self, id: Uuid) -> Result<()>;
            async fn get_children(&self, parent_id: Option<Uuid>) -> Result<Vec<Category>>;
        }
    }

    fn create_test_category(id: Uuid, name: &str, slug: &str) -> Category {
        Category {
            id,
            name: name.to_string(),
            slug: slug.to_string(),
            parent_id: None,
            created_at: chrono::Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_create_category_validates_empty_slug() {
        let mock_repo = Arc::new(MockCategoryRepo::new());
        let service = CategoryService::new(mock_repo);

        let input = CreateCategory {
            name: "Test".to_string(),
            slug: "".to_string(),
            parent_id: None,
        };

        let result = service.create(input).await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("cannot be empty")),
            _ => panic!("Expected validation error for empty slug"),
        }
    }

    #[tokio::test]
    async fn test_create_category_validates_invalid_slug() {
        let mock_repo = Arc::new(MockCategoryRepo::new());
        let service = CategoryService::new(mock_repo);

        let input = CreateCategory {
            name: "Test".to_string(),
            slug: "invalid slug!".to_string(),
            parent_id: None,
        };

        let result = service.create(input).await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => {
                assert!(msg.contains("letters, numbers, hyphens and underscores"))
            }
            _ => panic!("Expected validation error for invalid slug"),
        }
    }

    #[tokio::test]
    async fn test_create_category_validates_empty_name() {
        let mock_repo = Arc::new(MockCategoryRepo::new());
        let service = CategoryService::new(mock_repo);

        let input = CreateCategory {
            name: "".to_string(),
            slug: "test".to_string(),
            parent_id: None,
        };

        let result = service.create(input).await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("cannot be empty")),
            _ => panic!("Expected validation error for empty name"),
        }
    }

    #[tokio::test]
    async fn test_create_category_validates_long_name() {
        let mock_repo = Arc::new(MockCategoryRepo::new());
        let service = CategoryService::new(mock_repo);

        let input = CreateCategory {
            name: "a".repeat(101),
            slug: "test".to_string(),
            parent_id: None,
        };

        let result = service.create(input).await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("too long")),
            _ => panic!("Expected validation error for long name"),
        }
    }

    #[tokio::test]
    async fn test_create_category_with_invalid_parent() {
        let mut mock_repo = MockCategoryRepo::new();
        let parent_id = Uuid::new_v4();

        mock_repo
            .expect_get_category()
            .with(eq(parent_id))
            .times(1)
            .returning(|_| Ok(None));

        let service = CategoryService::new(Arc::new(mock_repo));

        let input = CreateCategory {
            name: "Test".to_string(),
            slug: "test".to_string(),
            parent_id: Some(parent_id),
        };

        let result = service.create(input).await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("Parent category not found")),
            _ => panic!("Expected validation error for invalid parent"),
        }
    }

    #[tokio::test]
    async fn test_create_category_success() {
        let mut mock_repo = MockCategoryRepo::new();
        let category_id = Uuid::new_v4();
        mock_repo
            .expect_create_category()
            .with(eq("Test".to_string()), eq("test".to_string()), eq(None))
            .times(1)
            .returning(move |name, slug, _| {
                Ok(Category {
                    id: category_id,
                    name,
                    slug,
                    parent_id: None,
                    created_at: chrono::Utc::now(),
                })
            });

        let service = CategoryService::new(Arc::new(mock_repo));

        let input = CreateCategory {
            name: "Test".to_string(),
            slug: "test".to_string(),
            parent_id: None,
        };

        let result = service.create(input).await;

        assert!(result.is_ok());
        let category = result.unwrap();
        assert_eq!(category.name, "Test");
        assert_eq!(category.slug, "test");
    }

    #[tokio::test]
    async fn test_get_category_not_found() {
        let mut mock_repo = MockCategoryRepo::new();
        let category_id = Uuid::new_v4();

        mock_repo
            .expect_get_category()
            .with(eq(category_id))
            .times(1)
            .returning(|_| Ok(None));

        let service = CategoryService::new(Arc::new(mock_repo));

        let result = service.get(category_id).await;

        assert!(result.is_err());
        match result {
            Err(Error::NotFound(msg)) => assert!(msg.contains("Category not found")),
            _ => panic!("Expected not found error"),
        }
    }

    #[tokio::test]
    async fn test_get_category_success() {
        let mut mock_repo = MockCategoryRepo::new();
        let category_id = Uuid::new_v4();
        mock_repo
            .expect_get_category()
            .with(eq(category_id))
            .times(1)
            .returning(move |_| Ok(Some(create_test_category(category_id, "Test", "test"))));

        let service = CategoryService::new(Arc::new(mock_repo));

        let result = service.get(category_id).await;

        assert!(result.is_ok());
        let category = result.unwrap();
        assert_eq!(category.id, category_id);
        assert_eq!(category.name, "Test");
    }

    #[tokio::test]
    async fn test_update_category_self_parent() {
        let mock_repo = MockCategoryRepo::new();
        let category_id = Uuid::new_v4();

        let service = CategoryService::new(Arc::new(mock_repo));

        let input = UpdateCategory {
            name: Some("Updated".to_string()),
            parent_id: Some(category_id),
        };

        let result = service.update(category_id, input).await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("cannot be its own parent")),
            _ => panic!("Expected validation error for self-parent"),
        }
    }

    #[tokio::test]
    async fn test_list_categories() {
        let mut mock_repo = MockCategoryRepo::new();
        let category1 = create_test_category(Uuid::new_v4(), "Test1", "test1");
        let category2 = create_test_category(Uuid::new_v4(), "Test2", "test2");

        mock_repo
            .expect_list_categories()
            .times(1)
            .returning(move || Ok(vec![category1.clone(), category2.clone()]));

        let service = CategoryService::new(Arc::new(mock_repo));

        let result = service.list().await;

        assert!(result.is_ok());
        let categories = result.unwrap();
        assert_eq!(categories.len(), 2);
    }

    #[tokio::test]
    async fn test_delete_category() {
        let mut mock_repo = MockCategoryRepo::new();
        let category_id = Uuid::new_v4();

        mock_repo
            .expect_delete_category()
            .with(eq(category_id))
            .times(1)
            .returning(|_| Ok(()));

        let service = CategoryService::new(Arc::new(mock_repo));

        let result = service.delete(category_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_children() {
        let mut mock_repo = MockCategoryRepo::new();
        let parent_id = Uuid::new_v4();
        let child = create_test_category(Uuid::new_v4(), "Child", "child");

        mock_repo
            .expect_get_children()
            .with(eq(Some(parent_id)))
            .times(1)
            .returning(move |_| Ok(vec![child.clone()]));

        let service = CategoryService::new(Arc::new(mock_repo));

        let result = service.get_children(Some(parent_id)).await;

        assert!(result.is_ok());
        let children = result.unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].name, "Child");
    }
}
