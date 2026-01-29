use async_trait::async_trait;
use domain::{Error, Post, Result};
use service::repository::PostRepository;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
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
        published_at: post.published_at.map(|d| d.to_rfc3339()),
        created_at: post.created_at.to_rfc3339(),
        views: 0,
    }
}

fn entity_to_active_model(entity: crate::entity::post::Model) -> crate::entity::post::ActiveModel {
    crate::entity::post::ActiveModel {
        id: Set(entity.id),
        user_id: Set(entity.user_id),
        title: Set(entity.title),
        content: Set(entity.content),
        published_at: Set(entity.published_at),
        created_at: Set(entity.created_at),
        views: Set(entity.views),
    }
}

fn parse_datetime_option(opt_str: &Option<String>) -> Result<Option<chrono::DateTime<chrono::Utc>>> {
    match opt_str {
        None => Ok(None),
        Some(s) => {
            chrono::DateTime::parse_from_rfc3339(s)
                .map(|dt| Some(dt.with_timezone(&chrono::Utc)))
                .map_err(|e| Error::Internal(format!("Invalid datetime: {}", e)))
        }
    }
}

fn model_to_post(model: crate::entity::post::Model) -> Result<Post> {
    let id = uuid::Uuid::parse_str(&model.id)
        .map_err(|e| Error::Internal(format!("Invalid post id: {}", e)))?;

    let user_id = uuid::Uuid::parse_str(&model.user_id)
        .map_err(|e| Error::Internal(format!("Invalid user_id: {}", e)))?;

    let published_at = parse_datetime_option(&model.published_at)?;

    let created_at = chrono::DateTime::parse_from_rfc3339(&model.created_at)
        .map_err(|e| Error::Internal(format!("Invalid created_at: {}", e)))?
        .with_timezone(&chrono::Utc);

    Ok(Post {
        id,
        user_id,
        title: model.title,
        content: model.content,
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
            .map_err(|e| {
                Error::Internal(format!(
                    "Failed to list published posts: {}",
                    e
                ))
            })?;

        models
            .into_iter()
            .map(model_to_post)
            .collect()
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
            .map_err(|e| {
                Error::Internal(format!(
                    "Failed to list posts by user: {}",
                    e
                ))
            })?;

        models
            .into_iter()
            .map(model_to_post)
            .collect()
    }
}
