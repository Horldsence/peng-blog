use domain::{CreateTag, Result, Tag};
use std::sync::Arc;
use crate::TagRepository;
use uuid::Uuid;

pub struct TagService<TR: TagRepository> {
    repo: Arc<TR>,
}

impl<TR: TagRepository> TagService<TR> {
    pub fn new(repo: Arc<TR>) -> Self {
        Self { repo }
    }

    pub async fn create(&self, input: CreateTag) -> Result<Tag> {
        self.validate_slug(&input.slug)?;
        self.validate_name(&input.name)?;

        self.repo.create_tag(input.name, input.slug).await
    }

    pub async fn get(&self, id: Uuid) -> Result<Tag> {
        self.repo
            .get_tag(id)
            .await?
            .ok_or_else(|| domain::Error::NotFound("Tag not found".to_string()))
    }

    pub async fn get_by_slug(&self, slug: &str) -> Result<Tag> {
        self.repo
            .get_tag_by_slug(slug)
            .await?
            .ok_or_else(|| domain::Error::NotFound("Tag not found".to_string()))
    }

    pub async fn list(&self) -> Result<Vec<Tag>> {
        self.repo.list_tags().await
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        self.repo.delete_tag(id).await
    }

    fn validate_slug(&self, slug: &str) -> Result<()> {
        if slug.trim().is_empty() {
            return Err(domain::Error::Validation("Slug cannot be empty".to_string()));
        }

        if !slug.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(domain::Error::Validation(
                "Slug can only contain letters, numbers, hyphens and underscores".to_string(),
            ));
        }

        Ok(())
    }

    fn validate_name(&self, name: &str) -> Result<()> {
        if name.trim().is_empty() {
            return Err(domain::Error::Validation("Name cannot be empty".to_string()));
        }

        if name.len() > 50 {
            return Err(domain::Error::Validation(
                "Name too long (max 50 characters)".to_string(),
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::{mock, predicate::*};
    use domain::Error;

    mock! {
        TagRepo {}

        #[async_trait::async_trait]
        impl TagRepository for TagRepo {
            async fn create_tag(&self, name: String, slug: String) -> Result<Tag>;
            async fn get_tag(&self, id: Uuid) -> Result<Option<Tag>>;
            async fn get_tag_by_slug(&self, slug: &str) -> Result<Option<Tag>>;
            async fn list_tags(&self) -> Result<Vec<Tag>>;
            async fn delete_tag(&self, id: Uuid) -> Result<()>;
        }
    }

    fn create_test_tag(id: Uuid, name: &str, slug: &str) -> Tag {
        Tag {
            id,
            name: name.to_string(),
            slug: slug.to_string(),
            created_at: chrono::Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_create_tag_validates_empty_slug() {
        let mock_repo = MockTagRepo::new();
        let service = TagService::new(Arc::new(mock_repo));

        let input = CreateTag {
            name: "Test".to_string(),
            slug: "".to_string(),
        };

        let result = service.create(input).await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("cannot be empty")),
            _ => panic!("Expected validation error for empty slug"),
        }
    }

    #[tokio::test]
    async fn test_create_tag_validates_invalid_slug() {
        let mock_repo = MockTagRepo::new();
        let service = TagService::new(Arc::new(mock_repo));

        let input = CreateTag {
            name: "Test".to_string(),
            slug: "invalid slug@".to_string(),
        };

        let result = service.create(input).await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("letters, numbers, hyphens and underscores")),
            _ => panic!("Expected validation error for invalid slug"),
        }
    }

    #[tokio::test]
    async fn test_create_tag_validates_empty_name() {
        let mock_repo = MockTagRepo::new();
        let service = TagService::new(Arc::new(mock_repo));

        let input = CreateTag {
            name: "".to_string(),
            slug: "test".to_string(),
        };

        let result = service.create(input).await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("cannot be empty")),
            _ => panic!("Expected validation error for empty name"),
        }
    }

    #[tokio::test]
    async fn test_create_tag_validates_long_name() {
        let mock_repo = MockTagRepo::new();
        let service = TagService::new(Arc::new(mock_repo));

        let input = CreateTag {
            name: "a".repeat(51),
            slug: "test".to_string(),
        };

        let result = service.create(input).await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("too long")),
            _ => panic!("Expected validation error for long name"),
        }
    }

    #[tokio::test]
    async fn test_create_tag_success() {
        let mut mock_repo = MockTagRepo::new();
        let tag_id = Uuid::new_v4();

        mock_repo
            .expect_create_tag()
            .with(eq("Rust".to_string()), eq("rust".to_string()))
            .times(1)
            .returning(move |name, slug| {
                Ok(Tag {
                    id: tag_id,
                    name,
                    slug,
                    created_at: chrono::Utc::now(),
                })
            });

        let service = TagService::new(Arc::new(mock_repo));

        let input = CreateTag {
            name: "Rust".to_string(),
            slug: "rust".to_string(),
        };

        let result = service.create(input).await;

        assert!(result.is_ok());
        let tag = result.unwrap();
        assert_eq!(tag.name, "Rust");
        assert_eq!(tag.slug, "rust");
    }

    #[tokio::test]
    async fn test_get_tag_not_found() {
        let mut mock_repo = MockTagRepo::new();
        let tag_id = Uuid::new_v4();

        mock_repo
            .expect_get_tag()
            .with(eq(tag_id))
            .times(1)
            .returning(|_| Ok(None));

        let service = TagService::new(Arc::new(mock_repo));

        let result = service.get(tag_id).await;

        assert!(result.is_err());
        match result {
            Err(Error::NotFound(msg)) => assert!(msg.contains("Tag not found")),
            _ => panic!("Expected not found error"),
        }
    }

    #[tokio::test]
    async fn test_get_tag_success() {
        let mut mock_repo = MockTagRepo::new();
        let tag_id = Uuid::new_v4();

        mock_repo
            .expect_get_tag()
            .with(eq(tag_id))
            .times(1)
            .returning(move |_| Ok(Some(create_test_tag(tag_id, "Rust", "rust"))));

        let service = TagService::new(Arc::new(mock_repo));

        let result = service.get(tag_id).await;

        assert!(result.is_ok());
        let tag = result.unwrap();
        assert_eq!(tag.id, tag_id);
        assert_eq!(tag.name, "Rust");
    }

    #[tokio::test]
    async fn test_get_tag_by_slug_not_found() {
        let mut mock_repo = MockTagRepo::new();

        mock_repo
            .expect_get_tag_by_slug()
            .with(eq("nonexistent"))
            .times(1)
            .returning(|_| Ok(None));

        let service = TagService::new(Arc::new(mock_repo));

        let result = service.get_by_slug("nonexistent").await;

        assert!(result.is_err());
        match result {
            Err(Error::NotFound(msg)) => assert!(msg.contains("Tag not found")),
            _ => panic!("Expected not found error"),
        }
    }

    #[tokio::test]
    async fn test_get_tag_by_slug_success() {
        let mut mock_repo = MockTagRepo::new();
        let tag_id = Uuid::new_v4();

        mock_repo
            .expect_get_tag_by_slug()
            .with(eq("rust"))
            .times(1)
            .returning(move |_| Ok(Some(create_test_tag(tag_id, "Rust", "rust"))));

        let service = TagService::new(Arc::new(mock_repo));

        let result = service.get_by_slug("rust").await;

        assert!(result.is_ok());
        let tag = result.unwrap();
        assert_eq!(tag.slug, "rust");
    }

    #[tokio::test]
    async fn test_list_tags() {
        let mut mock_repo = MockTagRepo::new();
        let tag1 = create_test_tag(Uuid::new_v4(), "Rust", "rust");
        let tag2 = create_test_tag(Uuid::new_v4(), "Python", "python");

        mock_repo
            .expect_list_tags()
            .times(1)
            .returning(move || Ok(vec![tag1.clone(), tag2.clone()]));

        let service = TagService::new(Arc::new(mock_repo));

        let result = service.list().await;

        assert!(result.is_ok());
        let tags = result.unwrap();
        assert_eq!(tags.len(), 2);
    }

    #[tokio::test]
    async fn test_delete_tag() {
        let mut mock_repo = MockTagRepo::new();
        let tag_id = Uuid::new_v4();

        mock_repo
            .expect_delete_tag()
            .with(eq(tag_id))
            .times(1)
            .returning(|_| Ok(()));

        let service = TagService::new(Arc::new(mock_repo));

        let result = service.delete(tag_id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_slug_with_hyphens_and_underscores() {
        let mut mock_repo = MockTagRepo::new();

        mock_repo
            .expect_create_tag()
            .times(1)
            .returning(|name, slug| {
                Ok(Tag {
                    id: Uuid::new_v4(),
                    name,
                    slug,
                    created_at: chrono::Utc::now(),
                })
            });

        let service = TagService::new(Arc::new(mock_repo));

        // Valid slugs with hyphens and underscores
        let input = CreateTag {
            name: "Test Tag".to_string(),
            slug: "test-tag_123".to_string(),
        };

        let result = service.create(input).await;
        assert!(result.is_ok());
    }
}
