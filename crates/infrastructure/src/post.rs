use async_trait::async_trait;
use domain::{Error, Post, PostRepository, Result, SearchPostsResponse};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct PostRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

impl Clone for PostRepositoryImpl {
    fn clone(&self) -> Self {
        Self {
            db: Arc::clone(&self.db),
        }
    }
}

impl PostRepositoryImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }
}

fn post_to_entity(post: &Post) -> crate::entity::post::Model {
    crate::entity::post::Model {
        id: post.id.to_string(),
        user_id: post.user_id.to_string(),
        title: post.title.clone(),
        content: post.content.clone(),
        category_id: post.category_id.map(|id| id.to_string()),
        published_at: post.published_at.map(|d| d.to_rfc3339()),
        created_at: post.created_at.to_rfc3339(),
        views: post.views as i64,
    }
}

fn entity_to_active_model(entity: crate::entity::post::Model) -> crate::entity::post::ActiveModel {
    crate::entity::post::ActiveModel {
        id: Set(entity.id),
        user_id: Set(entity.user_id),
        title: Set(entity.title),
        content: Set(entity.content),
        category_id: Set(entity.category_id),
        published_at: Set(entity.published_at),
        created_at: Set(entity.created_at),
        views: Set(entity.views),
    }
}

fn parse_datetime_option(
    opt_str: &Option<String>,
) -> Result<Option<chrono::DateTime<chrono::Utc>>> {
    match opt_str {
        None => Ok(None),
        Some(s) => chrono::DateTime::parse_from_rfc3339(s)
            .map(|dt| Some(dt.with_timezone(&chrono::Utc)))
            .map_err(|e| Error::Internal(format!("Invalid datetime: {}", e))),
    }
}

fn model_to_post(model: crate::entity::post::Model) -> Result<Post> {
    let id = uuid::Uuid::parse_str(&model.id)
        .map_err(|e| Error::Internal(format!("Invalid post id: {}", e)))?;

    let user_id = uuid::Uuid::parse_str(&model.user_id)
        .map_err(|e| Error::Internal(format!("Invalid user_id: {}", e)))?;

    let category_id = model
        .category_id
        .map(|id_str| {
            uuid::Uuid::parse_str(&id_str)
                .map_err(|e| Error::Internal(format!("Invalid category_id: {}", e)))
        })
        .transpose()?;

    let published_at = parse_datetime_option(&model.published_at)?;

    let created_at = chrono::DateTime::parse_from_rfc3339(&model.created_at)
        .map_err(|e| Error::Internal(format!("Invalid created_at: {}", e)))?
        .with_timezone(&chrono::Utc);

    Ok(Post {
        id,
        user_id,
        title: model.title,
        content: model.content,
        category_id,
        published_at,
        created_at,
        views: model.views as u64,
    })
}

#[async_trait]
impl PostRepository for PostRepositoryImpl {
    async fn create_post(&self, user_id: Uuid, title: String, content: String) -> Result<Post> {
        let post = Post::new(user_id, title, content);
        let entity = post_to_entity(&post);
        let active_model = entity_to_active_model(entity);

        active_model
            .insert(self.db.as_ref())
            .await
            .map_err(|e| Error::Internal(format!("Failed to create post: {}", e)))?;

        Ok(post)
    }

    async fn get_post(&self, id: Uuid) -> Result<Post> {
        let model = crate::entity::post::Entity::find_by_id(id.to_string())
            .one(self.db.as_ref())
            .await
            .map_err(|e| Error::Internal(format!("Failed to find post: {}", e)))?
            .ok_or_else(|| Error::NotFound(format!("Post with id {} not found", id)))?;

        model_to_post(model)
    }

    async fn update_post(&self, post: Post) -> Result<Post> {
        let entity = post_to_entity(&post);
        let active_model = entity_to_active_model(entity);

        active_model
            .update(self.db.as_ref())
            .await
            .map_err(|e| Error::Internal(format!("Failed to update post: {}", e)))?;

        Ok(post)
    }

    async fn list_published_posts(&self, limit: u64) -> Result<Vec<Post>> {
        let models = crate::entity::post::Entity::find()
            .filter(crate::entity::post::Column::PublishedAt.is_not_null())
            .order_by_desc(crate::entity::post::Column::PublishedAt)
            .limit(limit)
            .all(self.db.as_ref())
            .await
            .map_err(|e| Error::Internal(format!("Failed to list published posts: {}", e)))?;

        models.into_iter().map(model_to_post).collect()
    }

    async fn delete_post(&self, id: Uuid) -> Result<()> {
        crate::entity::post::Entity::delete_by_id(id.to_string())
            .exec(self.db.as_ref())
            .await
            .map_err(|e| Error::Internal(format!("Failed to delete post: {}", e)))?;
        Ok(())
    }

    async fn get_posts_by_user(&self, user_id: Uuid, limit: u64) -> Result<Vec<Post>> {
        let models = crate::entity::post::Entity::find()
            .filter(crate::entity::post::Column::UserId.eq(user_id.to_string()))
            .order_by_desc(crate::entity::post::Column::CreatedAt)
            .limit(limit)
            .all(self.db.as_ref())
            .await
            .map_err(|e| Error::Internal(format!("Failed to list posts by user: {}", e)))?;

        models.into_iter().map(model_to_post).collect()
    }

    async fn list_published_posts_by_user(&self, user_id: Uuid, limit: u64) -> Result<Vec<Post>> {
        let models = crate::entity::post::Entity::find()
            .filter(crate::entity::post::Column::UserId.eq(user_id.to_string()))
            .filter(crate::entity::post::Column::PublishedAt.is_not_null())
            .order_by_desc(crate::entity::post::Column::PublishedAt)
            .limit(limit)
            .all(self.db.as_ref())
            .await
            .map_err(|e| {
                Error::Internal(format!("Failed to list published posts by user: {}", e))
            })?;

        models.into_iter().map(model_to_post).collect()
    }

    async fn list_all_posts(&self, limit: u64) -> Result<Vec<Post>> {
        let models = crate::entity::post::Entity::find()
            .order_by_desc(crate::entity::post::Column::CreatedAt)
            .limit(limit)
            .all(self.db.as_ref())
            .await
            .map_err(|e| Error::Internal(format!("Failed to list all posts: {}", e)))?;

        models.into_iter().map(model_to_post).collect()
    }

    async fn update_post_category(&self, post_id: Uuid, category_id: Option<Uuid>) -> Result<()> {
        use sea_orm::ActiveModelTrait;

        let post = crate::entity::post::Entity::find_by_id(post_id.to_string())
            .one(self.db.as_ref())
            .await
            .map_err(|e| Error::Internal(format!("Failed to find post: {}", e)))?
            .ok_or_else(|| Error::NotFound("Post not found".to_string()))?;

        let mut active: crate::entity::post::ActiveModel = post.into();
        active.category_id = Set(category_id.map(|id| id.to_string()));

        active
            .update(self.db.as_ref())
            .await
            .map_err(|e| Error::Internal(format!("Failed to update post: {}", e)))?;

        Ok(())
    }

    async fn get_posts_by_category(&self, category_id: Uuid, limit: u64) -> Result<Vec<Post>> {
        let models = crate::entity::post::Entity::find()
            .filter(crate::entity::post::Column::CategoryId.eq(category_id.to_string()))
            .filter(crate::entity::post::Column::PublishedAt.is_not_null())
            .order_by_desc(crate::entity::post::Column::CreatedAt)
            .limit(limit)
            .all(self.db.as_ref())
            .await
            .map_err(|e| Error::Internal(format!("Failed to get posts by category: {}", e)))?;

        models.into_iter().map(model_to_post).collect()
    }

    async fn add_tag_to_post(&self, post_id: Uuid, tag_id: Uuid) -> Result<()> {
        use sea_orm::ActiveModelTrait;

        let post_tag = crate::entity::post_tag::ActiveModel {
            post_id: Set(post_id.to_string()),
            tag_id: Set(tag_id.to_string()),
        };

        post_tag.insert(self.db.as_ref()).await.map_err(|e| {
            if e.to_string().contains("UNIQUE") {
                Error::Validation("Tag already added to post".to_string())
            } else {
                Error::Internal(format!("Failed to add tag to post: {}", e))
            }
        })?;

        Ok(())
    }

    async fn remove_tag_from_post(&self, post_id: Uuid, tag_id: Uuid) -> Result<()> {
        crate::entity::post_tag::Entity::delete_many()
            .filter(crate::entity::post_tag::Column::PostId.eq(post_id.to_string()))
            .filter(crate::entity::post_tag::Column::TagId.eq(tag_id.to_string()))
            .exec(self.db.as_ref())
            .await
            .map_err(|e| Error::Internal(format!("Failed to remove tag from post: {}", e)))?;

        Ok(())
    }

    async fn get_post_tags(&self, post_id: Uuid) -> Result<Vec<domain::Tag>> {
        use sea_orm::EntityTrait;

        let post_tags = crate::entity::post_tag::Entity::find()
            .filter(crate::entity::post_tag::Column::PostId.eq(post_id.to_string()))
            .find_also_related(crate::entity::tag::Entity)
            .all(self.db.as_ref())
            .await
            .map_err(|e| Error::Internal(format!("Failed to get post tags: {}", e)))?;

        let tags = post_tags
            .into_iter()
            .filter_map(|(_, tag_opt)| tag_opt)
            .map(|tag| domain::Tag {
                id: Uuid::parse_str(&tag.id).unwrap(),
                name: tag.name,
                slug: tag.slug,
                created_at: chrono::DateTime::parse_from_rfc3339(&tag.created_at)
                    .unwrap()
                    .with_timezone(&chrono::Utc),
            })
            .collect();

        Ok(tags)
    }

    async fn get_posts_by_tag(&self, tag_id: Uuid, limit: u64) -> Result<Vec<Post>> {
        use sea_orm::{EntityTrait, QuerySelect};

        let post_tags = crate::entity::post_tag::Entity::find()
            .filter(crate::entity::post_tag::Column::TagId.eq(tag_id.to_string()))
            .find_also_related(crate::entity::post::Entity)
            .limit(limit)
            .all(self.db.as_ref())
            .await
            .map_err(|e| Error::Internal(format!("Failed to get posts by tag: {}", e)))?;

        let posts = post_tags
            .into_iter()
            .filter_map(|(_, post_opt)| post_opt)
            .filter_map(|post| model_to_post(post).ok())
            .collect();

        Ok(posts)
    }

    async fn search_posts(
        &self,
        query: &str,
        limit: u64,
        offset: u64,
    ) -> Result<SearchPostsResponse> {
        let search_pattern = format!("%{query}%");

        // Build condition: (title LIKE ? OR content LIKE ?) AND published_at IS NOT NULL
        let condition = Condition::all()
            .add(
                Condition::any()
                    .add(crate::entity::post::Column::Title.like(&search_pattern))
                    .add(crate::entity::post::Column::Content.like(&search_pattern)),
            )
            .add(crate::entity::post::Column::PublishedAt.is_not_null());

        // Get total count
        let total = crate::entity::post::Entity::find()
            .filter(condition.clone())
            .count(self.db.as_ref())
            .await
            .map_err(|e| Error::Internal(format!("Failed to count search results: {}", e)))?;

        // Get paginated results
        let models = crate::entity::post::Entity::find()
            .filter(condition)
            .order_by_desc(crate::entity::post::Column::PublishedAt)
            .limit(limit)
            .offset(offset)
            .all(self.db.as_ref())
            .await
            .map_err(|e| Error::Internal(format!("Failed to search posts: {}", e)))?;

        let posts = models
            .into_iter()
            .map(model_to_post)
            .collect::<Result<Vec<_>>>()?;

        Ok(SearchPostsResponse {
            posts,
            total,
            query: query.to_string(),
        })
    }
}
