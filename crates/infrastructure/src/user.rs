use async_trait::async_trait;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use domain::{Error, Result, User};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use std::sync::Arc;

pub struct UserRepositoryImpl {
    db: Arc<DatabaseConnection>,
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
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, username: String, password: String, permissions: u64) -> Result<User>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>>;
    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<User>>;
    async fn verify_credentials(&self, username: &str, password: &str) -> Result<Option<User>>;
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn create_user(&self, username: String, password: String, permissions: u64) -> Result<User> {
        // Check if username already exists
        if let Some(_) = self.find_by_username(&username).await? {
            return Err(Error::Validation("Username already exists".to_string()));
        }

        let password_hash = self.hash_password(&password)?;
        let user = User::new(username.clone(), password_hash, permissions);

        crate::entity::user::ActiveModel {
            id: Set(user.id.to_string()),
            username: Set(user.username.clone()),
            password_hash: Set(user.password_hash.clone()),
            permissions: Set(user.permissions as i64),
            created_at: Set(user.created_at.to_rfc3339()),
        }
        .insert(self.db.as_ref())
        .await
        .map_err(|e| Error::Internal(format!("Failed to create user: {}", e)))?;

        Ok(user)
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

    async fn find_by_id(&self, id: uuid::Uuid) -> Result<Option<User>> {
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
                    // Don't return password hash in response
                    Ok(Some(user))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}