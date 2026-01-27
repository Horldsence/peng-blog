//! # User Service - Business Logic for Users
//!
//! This service implements business logic for user operations.
//! It coordinates repository calls and enforces business rules.

use crate::repository::UserRepository;
use domain::{Error, Result, User, DEFAULT_USER_PERMISSIONS, USER_MANAGE};
use uuid::Uuid;
use std::sync::Arc;

/// Service for user business logic
///
/// This service encapsulates all business rules for user operations.
/// It uses dependency injection for repositories, making it testable.
pub struct UserService<R: UserRepository> {
    repo: Arc<R>,
}

impl<R: UserRepository> UserService<R> {
    /// Create a new UserService with given repository
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }

    /// Register a new user with validation
    ///
    /// This method validates username and password, checks if username is unique,
    /// and assigns appropriate permissions.
    pub async fn register(&self, username: String, password: String) -> Result<User> {
        self.validate_username(&username)?;
        self.validate_password(&password)?;

        // Check if username already exists
        if self.repo.find_by_username(&username).await?.is_some() {
            return Err(Error::Validation("Username already exists".to_string()));
        }

        // Check if this is the first user (make them admin)
        let existing_users = self.repo.list_users(1).await?;
        let is_first_user = existing_users.is_empty();
        let permissions = if is_first_user {
            USER_MANAGE // Full admin permissions
        } else {
            DEFAULT_USER_PERMISSIONS
        };

        self.repo.create_user(username, password, permissions).await
    }

    /// Authenticate user with username and password
    ///
    /// Returns the user if credentials are valid, None otherwise.
    pub async fn login(&self, username: String, password: String) -> Result<User> {
        self.validate_username(&username)?;
        self.validate_password(&password)?;

        self.repo
            .verify_credentials(&username, &password)
            .await?
            .ok_or_else(|| Error::NotFound("Invalid credentials".to_string()))
    }

    /// Get user by ID
    pub async fn get(&self, id: Uuid) -> Result<User> {
        self.repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| Error::NotFound(format!("User with id {} not found", id)))
    }

    /// Get user by username
    pub async fn get_by_username(&self, username: &str) -> Result<User> {
        self.repo
            .find_by_username(username)
            .await?
            .ok_or_else(|| Error::NotFound(format!("User '{}' not found", username)))
    }

    /// Update user permissions (admin only)
    ///
    /// Only users with USER_MANAGE permission can call this.
    pub async fn update_permissions(
        &self,
        requester_id: Uuid,
        requester_permissions: u64,
        target_user_id: Uuid,
        new_permissions: u64,
    ) -> Result<User> {
        // Check if requester has admin permission
        if (requester_permissions & USER_MANAGE) == 0 {
            return Err(Error::Validation(
                "Insufficient permissions to manage users".to_string(),
            ));
        }

        // Get requester and target user
        let _requester = self.repo.find_by_id(requester_id).await?
            .ok_or_else(|| Error::NotFound("Requester not found".to_string()))?;

        let target_user = self.repo.find_by_id(target_user_id).await?
            .ok_or_else(|| Error::NotFound("Target user not found".to_string()))?;

        // Prevent users from removing their own admin privileges
        if requester_id == target_user_id {
            if (new_permissions & USER_MANAGE) == 0 {
                return Err(Error::Validation(
                    "Cannot remove your own admin privileges".to_string(),
                ));
            }
        }

        // Prevent removing permissions from the last admin
        if target_user.has_permission(USER_MANAGE) && (new_permissions & USER_MANAGE) == 0 {
            let admin_count = self
                .repo
                .list_users(1000)
                .await?
                .into_iter()
                .filter(|u| u.has_permission(USER_MANAGE))
                .count();

            if admin_count <= 1 {
                return Err(Error::Validation(
                    "Cannot remove permissions from the last admin".to_string(),
                ));
            }
        }

        self.repo
            .update_permissions(target_user_id, new_permissions)
            .await
    }

    /// List all users (admin only)
    ///
    /// Only users with USER_MANAGE permission can call this.
    pub async fn list(
        &self,
        requester_permissions: u64,
        limit: Option<u64>,
    ) -> Result<Vec<User>> {
        // Check if requester has admin permission
        if (requester_permissions & USER_MANAGE) == 0 {
            return Err(Error::Validation(
                "Insufficient permissions to list users".to_string(),
            ));
        }

        self.repo.list_users(limit.unwrap_or(50)).await
    }

    /// Check if user exists by username
    pub async fn exists(&self, username: &str) -> bool {
        self.repo
            .find_by_username(username)
            .await
            .map(|opt| opt.is_some())
            .unwrap_or(false)
    }

    /// Check if user exists by ID
    pub async fn exists_by_id(&self, id: Uuid) -> bool {
        self.repo
            .find_by_id(id)
            .await
            .map(|opt| opt.is_some())
            .unwrap_or(false)
    }
}

// ============================================================================
// Private Validation Helpers
// ============================================================================

impl<R: UserRepository> UserService<R> {
    fn validate_username(&self, username: &str) -> Result<()> {
        if username.trim().is_empty() {
            return Err(Error::Validation("Username cannot be empty".to_string()));
        }
        if username.len() < 3 {
            return Err(Error::Validation(
                "Username must be at least 3 characters".to_string(),
            ));
        }
        if username.len() > 30 {
            return Err(Error::Validation(
                "Username too long (max 30 characters)".to_string(),
            ));
        }
        
        // Check for invalid characters (alphanumeric and underscore only)
        if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(Error::Validation(
                "Username can only contain letters, numbers, and underscores".to_string(),
            ));
        }
        
        Ok(())
    }

    fn validate_password(&self, password: &str) -> Result<()> {
        if password.len() < 8 {
            return Err(Error::Validation(
                "Password must be at least 8 characters".to_string(),
            ));
        }
        
        // Check for at least one letter and one number
        let has_letter = password.chars().any(|c| c.is_alphabetic());
        let has_number = password.chars().any(|c| c.is_numeric());
        
        if !has_letter || !has_number {
            return Err(Error::Validation(
                "Password must contain at least one letter and one number".to_string(),
            ));
        }
        
        Ok(())
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use domain::{ADMIN_PERMISSIONS, DEFAULT_USER_PERMISSIONS};
    use async_trait::async_trait;
    use mockall::mock;

    // Mock repository for testing
    mock! {
        UserRepo {}

        #[async_trait]
        impl UserRepository for UserRepo {
            async fn create_user(&self, username: String, password: String, permissions: u64) -> Result<User>;
            async fn find_by_username(&self, username: &str) -> Result<Option<User>>;
            async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
            async fn verify_credentials(&self, username: &str, password: &str) -> Result<Option<User>>;
            async fn update_permissions(&self, user_id: Uuid, permissions: u64) -> Result<User>;
            async fn list_users(&self, limit: u64) -> Result<Vec<User>>;
        }
    }

    #[tokio::test]
    async fn test_register_validates_empty_username() {
        let mut mock_repo = MockUserRepo::new();
        let service = UserService::new(Arc::new(mock_repo));

        let result = service.register("".to_string(), "password123".to_string()).await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("cannot be empty")),
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_register_validates_short_username() {
        let mut mock_repo = MockUserRepo::new();
        let service = UserService::new(Arc::new(mock_repo));

        let result = service.register("ab".to_string(), "password123".to_string()).await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("at least 3 characters")),
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_register_validates_long_username() {
        let mut mock_repo = MockUserRepo::new();
        let service = UserService::new(Arc::new(mock_repo));

        let long_username = "a".repeat(31);
        let result = service
            .register(long_username, "password123".to_string())
            .await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("too long")),
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_register_validates_short_password() {
        let mut mock_repo = MockUserRepo::new();
        let service = UserService::new(Arc::new(mock_repo));

        let result = service.register("username".to_string(), "short".to_string()).await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("at least 8 characters")),
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_register_validates_password_requirements() {
        let mut mock_repo = MockUserRepo::new();
        let service = UserService::new(Arc::new(mock_repo));

        // Password without number
        let result = service
            .register("username".to_string(), "password".to_string())
            .await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("letter and one number")),
            _ => panic!("Expected validation error"),
        }

        // Password without letter
        let result = service
            .register("username".to_string(), "12345678".to_string())
            .await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("letter and one number")),
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_update_permissions_requires_admin() {
        let mut mock_repo = MockUserRepo::new();
        let service = UserService::new(Arc::new(mock_repo));

        let user_id = Uuid::new_v4();
        let target_id = Uuid::new_v4();
        let no_permissions = 0;

        let result = service
            .update_permissions(user_id, no_permissions, target_id, ADMIN_PERMISSIONS)
            .await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("Insufficient permissions")),
            _ => panic!("Expected permission error"),
        }
    }

    #[tokio::test]
    async fn test_cannot_remove_own_admin() {
        let mut mock_repo = MockUserRepo::new();
        let service = UserService::new(Arc::new(mock_repo));

        let user_id = Uuid::new_v4();
        let no_admin = DEFAULT_USER_PERMISSIONS;

        let result = service
            .update_permissions(user_id, ADMIN_PERMISSIONS, user_id, no_admin)
            .await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("Cannot remove your own admin")),
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_list_requires_admin() {
        let mut mock_repo = MockUserRepo::new();
        let service = UserService::new(Arc::new(mock_repo));

        let no_permissions = 0;

        let result = service.list(no_permissions, None).await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("Insufficient permissions")),
            _ => panic!("Expected permission error"),
        }
    }
}