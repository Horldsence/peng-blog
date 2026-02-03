//! # User Service - Business Logic for Users
//!
//! This service implements business logic for user operations.
//! It coordinates repository calls and enforces business rules.

use domain::UserRepository;
use domain::{Error, Result, User, DEFAULT_USER_PERMISSIONS, USER_MANAGE};
use std::sync::Arc;
use uuid::Uuid;

// ============================================================================
// Constants
// ============================================================================

/// Default limit for listing users
const DEFAULT_LIST_LIMIT: u64 = 50;

/// Limit used when checking for admin count
const ADMIN_COUNT_CHECK_LIMIT: u64 = 1000;

/// Service for user business logic
///
/// This service encapsulates all business rules for user operations.
/// It uses dependency injection for repositories, making it testable.
pub struct UserService {
    repo: Arc<dyn UserRepository>,
    allow_registration: bool,
}

impl UserService {
    /// Create a new UserService with given repository and config
    pub fn new(repo: Arc<dyn UserRepository>, allow_registration: bool) -> Self {
        Self {
            repo,
            allow_registration,
        }
    }

    /// Register a new user with validation
    ///
    /// This method validates username and password, checks if username is unique,
    /// and assigns appropriate permissions.
    pub async fn register(&self, username: String, password: String) -> Result<User> {
        if !self.allow_registration {
            return Err(Error::Validation("Registration is disabled".to_string()));
        }

        self.validate_username(&username)?;
        self.validate_password(&password)?;

        // Check if username already exists
        if self.repo.find_by_username(&username).await?.is_some() {
            return Err(Error::Validation("Username already exists".to_string()));
        }

        // Check if this is first user (make them admin)
        let existing_users = self.repo.list_users(1).await?;
        let is_first_user = existing_users.is_empty();
        let permissions = if is_first_user {
            domain::ADMIN_PERMISSIONS // Full admin permissions
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
        let _requester = self
            .repo
            .find_by_id(requester_id)
            .await?
            .ok_or_else(|| Error::NotFound("Requester not found".to_string()))?;

        let target_user = self
            .repo
            .find_by_id(target_user_id)
            .await?
            .ok_or_else(|| Error::NotFound("Target user not found".to_string()))?;

        // Prevent users from removing their own admin privileges
        if requester_id == target_user_id && (new_permissions & USER_MANAGE) == 0 {
            return Err(Error::Validation(
                "Cannot remove your own admin privileges".to_string(),
            ));
        }

        // Prevent removing permissions from the last admin
        if target_user.has_permission(USER_MANAGE) && (new_permissions & USER_MANAGE) == 0 {
            let admin_count = self
                .repo
                .list_users(ADMIN_COUNT_CHECK_LIMIT)
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
    pub async fn list(&self, requester_permissions: u64, limit: Option<u64>) -> Result<Vec<User>> {
        // Check if requester has admin permission
        if (requester_permissions & USER_MANAGE) == 0 {
            return Err(Error::Validation(
                "Insufficient permissions to list users".to_string(),
            ));
        }

        self.repo
            .list_users(limit.unwrap_or(DEFAULT_LIST_LIMIT))
            .await
    }

    /// Check if user exists by username
    pub async fn exists(&self, username: &str) -> bool {
        match self.repo.find_by_username(username).await {
            Ok(user_opt) => user_opt.is_some(),
            Err(_) => false,
        }
    }

    /// Check if user exists by ID
    pub async fn exists_by_id(&self, id: Uuid) -> bool {
        match self.repo.find_by_id(id).await {
            Ok(user_opt) => user_opt.is_some(),
            Err(_) => false,
        }
    }

    /// Delete a user (admin only or self-delete)
    ///
    /// Users can delete their own account. Admins can delete any account
    /// except the last admin account.
    pub async fn delete(
        &self,
        target_user_id: Uuid,
        requester_id: Uuid,
        requester_permissions: u64,
    ) -> Result<()> {
        // Check if requester is the target or has admin permission
        let is_self = target_user_id == requester_id;
        let is_admin = (requester_permissions & USER_MANAGE) != 0;

        if !is_self && !is_admin {
            return Err(Error::Unauthorized(
                "You can only delete your own account".to_string(),
            ));
        }

        // Check if target user exists
        let target_user = self
            .repo
            .find_by_id(target_user_id)
            .await?
            .ok_or_else(|| Error::NotFound("User not found".to_string()))?;

        // If deleting an admin, check that this is not the last admin
        let is_target_admin = (target_user.permissions & USER_MANAGE) != 0;
        if is_target_admin && is_admin {
            // Count remaining admins
            let users = self.repo.list_users(ADMIN_COUNT_CHECK_LIMIT).await?;
            let admin_count = users
                .iter()
                .filter(|u| (u.permissions & USER_MANAGE) != 0)
                .count();

            if admin_count <= 1 {
                return Err(Error::Validation(
                    "Cannot delete the last admin user".to_string(),
                ));
            }
        }

        // Delete the user
        self.repo.delete_user(target_user_id).await
    }
}

// ============================================================================
// Private Validation Helpers
// ============================================================================

impl UserService {
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
    use async_trait::async_trait;
    use domain::{ADMIN_PERMISSIONS, DEFAULT_USER_PERMISSIONS};
    use mockall::mock;

    mock! {
        UserRepo {}

        #[async_trait]
        impl UserRepository for UserRepo {
            async fn create_user(&self, username: String, password: String, permissions: u64) -> Result<User>;
            async fn find_by_username(&self, username: &str) -> Result<Option<User>>;
            async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
            async fn verify_credentials(&self, username: &str, password: &str) -> Result<Option<User>>;
            async fn update_permissions(&self, user_id: Uuid, permissions: u64) -> Result<User>;
            async fn update_password(&self, user_id: Uuid, new_password: String) -> Result<()>;
            async fn list_users(&self, limit: u64) -> Result<Vec<User>>;
            async fn delete_user(&self, user_id: Uuid) -> Result<()>;
        }
    }

    fn setup_service() -> UserService {
        let mock_repo = Arc::new(MockUserRepo::new());
        UserService::new(mock_repo, true)
    }

    #[tokio::test]
    async fn test_register_validates_empty_username() {
        let service = setup_service();

        let result = service
            .register("".to_string(), "password123".to_string())
            .await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("cannot be empty")),
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_register_validates_short_username() {
        let service = setup_service();

        let result = service
            .register("ab".to_string(), "password123".to_string())
            .await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("at least 3 characters")),
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_register_validates_long_username() {
        let service = setup_service();

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
        let service = setup_service();

        let result = service
            .register("username".to_string(), "short".to_string())
            .await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("at least 8 characters")),
            _ => panic!("Expected validation error"),
        }
    }

    #[tokio::test]
    async fn test_register_validates_password_requirements() {
        let service = setup_service();

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
        let service = setup_service();

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
        let user_id = Uuid::new_v4();

        mock_repo
            .expect_find_by_id()
            .times(2)
            .with(mockall::predicate::eq(user_id))
            .returning(move |_| {
                Ok(Some(User::new(
                    user_id,
                    "testuser".to_string(),
                    "password".to_string(),
                    ADMIN_PERMISSIONS,
                )))
            });

        let service = UserService::new(Arc::new(mock_repo), true);

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
        let service = setup_service();

        let no_permissions = 0;

        let result = service.list(no_permissions, None).await;

        assert!(result.is_err());
        match result {
            Err(Error::Validation(msg)) => assert!(msg.contains("Insufficient permissions")),
            _ => panic!("Expected permission error"),
        }
    }
}
