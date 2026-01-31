use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use async_trait::async_trait;
use domain::{Error, Result, User, UserRepository};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct UserRepositoryImpl {
    db: Arc<DatabaseConnection>,
}

impl Clone for UserRepositoryImpl {
    fn clone(&self) -> Self {
        Self {
            db: Arc::clone(&self.db),
        }
    }
}

impl UserRepositoryImpl {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    fn hash_password(&self, password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| Error::Internal(format!("Failed to hash password: {}", e)))
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| Error::Internal(format!("Invalid password hash: {}", e)))?;
        let argon2 = Argon2::default();
        argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .map_err(|_| Error::Internal("Password verification failed".to_string()))?;
        Ok(true)
    }
}

fn parse_datetime(dt_str: &str) -> Result<chrono::DateTime<chrono::Utc>> {
    chrono::DateTime::parse_from_rfc3339(dt_str)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .map_err(|e| Error::Internal(format!("Invalid datetime: {}", e)))
}

fn model_to_user(model: crate::entity::user::Model) -> Result<User> {
    let id = uuid::Uuid::parse_str(&model.id)
        .map_err(|e| Error::Internal(format!("Invalid user id: {}", e)))?;

    let created_at = parse_datetime(&model.created_at)?;

    Ok(User {
        id,
        username: model.username,
        password_hash: model.password_hash,
        permissions: model.permissions as u64,
        created_at,
    })
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create_user(
        &self,
        username: String,
        password: String,
        permissions: u64,
    ) -> Result<User> {
        let password_hash = self.hash_password(&password)?;
        let user_id = Uuid::new_v4();
        let created_at = chrono::Utc::now();

        crate::entity::user::ActiveModel {
            id: Set(user_id.to_string()),
            username: Set(username.clone()),
            password_hash: Set(password_hash.clone()),
            permissions: Set(permissions as i64),
            created_at: Set(created_at.to_rfc3339()),
        }
        .insert(self.db.as_ref())
        .await
        .map_err(|e| Error::Internal(format!("Failed to create user: {}", e)))?;

        // Return User object directly since we have all the data
        Ok(User {
            id: user_id,
            username,
            password_hash,
            permissions,
            created_at,
        })
    }

    async fn find_by_username(&self, username: &str) -> Result<Option<User>> {
        let model = crate::entity::user::Entity::find()
            .filter(crate::entity::user::Column::Username.eq(username))
            .one(self.db.as_ref())
            .await
            .map_err(|e| Error::Internal(format!("Failed to find user: {}", e)))?;

        match model {
            Some(model) => Ok(Some(model_to_user(model)?)),
            None => Ok(None),
        }
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let model = crate::entity::user::Entity::find()
            .filter(crate::entity::user::Column::Id.eq(id.to_string()))
            .one(self.db.as_ref())
            .await
            .map_err(|e| Error::Internal(format!("Failed to find user: {}", e)))?;

        match model {
            Some(model) => Ok(Some(model_to_user(model)?)),
            None => Ok(None),
        }
    }

    async fn verify_credentials(&self, username: &str, password: &str) -> Result<Option<User>> {
        let user = self.find_by_username(username).await?;

        match user {
            Some(user) => {
                let valid = self.verify_password(password, &user.password_hash)?;
                if valid {
                    Ok(Some(user))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    async fn update_permissions(&self, user_id: Uuid, permissions: u64) -> Result<User> {
        let model = crate::entity::user::Entity::find_by_id(user_id.to_string())
            .one(self.db.as_ref())
            .await
            .map_err(|e| Error::Internal(format!("Failed to find user: {}", e)))?
            .ok_or_else(|| Error::NotFound(format!("User with id {} not found", user_id)))?;

        let active_model = crate::entity::user::ActiveModel {
            id: Set(model.id),
            username: Set(model.username),
            password_hash: Set(model.password_hash),
            permissions: Set(permissions as i64),
            created_at: Set(model.created_at),
        };

        let updated_model = active_model
            .update(self.db.as_ref())
            .await
            .map_err(|e| Error::Internal(format!("Failed to update user: {}", e)))?;

        model_to_user(updated_model)
    }

    async fn list_users(&self, limit: u64) -> Result<Vec<User>> {
        let models = crate::entity::user::Entity::find()
            .order_by_asc(crate::entity::user::Column::CreatedAt)
            .limit(limit)
            .all(self.db.as_ref())
            .await
            .map_err(|e| Error::Internal(format!("Failed to list users: {}", e)))?;

        models.into_iter().map(model_to_user).collect()
    }

    async fn update_password(&self, user_id: Uuid, new_password: String) -> Result<()> {
        // Load user and check existence in one query
        let model = crate::entity::user::Entity::find_by_id(user_id.to_string())
            .one(self.db.as_ref())
            .await
            .map_err(|e| Error::Internal(format!("Failed to find user: {}", e)))?
            .ok_or_else(|| Error::NotFound(format!("User with id {} not found", user_id)))?;

        // Hash new password and update
        let password_hash = self.hash_password(&new_password)?;

        crate::entity::user::ActiveModel {
            id: Set(model.id),
            username: Set(model.username),
            password_hash: Set(password_hash),
            permissions: Set(model.permissions),
            created_at: Set(model.created_at),
        }
        .update(self.db.as_ref())
        .await
        .map_err(|e| Error::Internal(format!("Failed to update password: {}", e)))?;

        Ok(())
    }

    async fn delete_user(&self, user_id: Uuid) -> Result<()> {
        // Delete user and check if it existed (cascade will delete related records)
        let result = crate::entity::user::Entity::delete_by_id(user_id.to_string())
            .exec(self.db.as_ref())
            .await
            .map_err(|e| Error::Internal(format!("Failed to delete user: {}", e)))?;

        if result.rows_affected == 0 {
            return Err(Error::NotFound(format!(
                "User with id {} not found",
                user_id
            )));
        }

        Ok(())
    }
}
