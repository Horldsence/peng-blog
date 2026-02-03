use super::ConfigError;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub auth: AuthConfig,
    pub storage: StorageConfig,
    pub github: GitHubConfig,
    pub site: SiteConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorageConfig {
    pub upload_dir: String,
    pub cache_dir: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GitHubConfig {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SiteConfig {
    pub allow_registration: bool,
}

impl Config {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.database.url.is_empty() {
            return Err(ConfigError::Validation(
                "DATABASE_URL cannot be empty".to_string(),
            ));
        }

        if self.auth.jwt_secret == "change-this-secret-in-production" {
            tracing::warn!("Using default JWT secret. Change this in production!");
        }

        if self.server.port == 0 {
            return Err(ConfigError::Validation(
                "Server port cannot be zero".to_string(),
            ));
        }

        Ok(())
    }
}
