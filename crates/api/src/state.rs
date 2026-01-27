use infrastructure::{PostRepositoryImpl, UserRepositoryImpl};
use std::sync::Arc;

use crate::middleware::auth::AuthState;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<PostRepositoryImpl>,
    pub user_repo: Arc<UserRepositoryImpl>,
    pub auth_state: AuthState,
}

impl AppState {
    pub fn new(
        db: Arc<PostRepositoryImpl>,
        user_repo: Arc<UserRepositoryImpl>,
        auth_state: AuthState,
    ) -> Self {
        Self {
            db,
            user_repo,
            auth_state,
        }
    }

    pub fn auth_state(&self) -> &AuthState {
        &self.auth_state
    }
}
