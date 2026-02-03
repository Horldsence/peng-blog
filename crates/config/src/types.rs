use super::ConfigError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub auth: AuthConfig,
    pub storage: StorageConfig,
    pub github: GitHubConfig,
    pub site: SiteConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_env_override: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_env_override: Option<bool>,
    pub port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_env_override: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_secret_env_override: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StorageConfig {
    pub upload_dir: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_dir_env_override: Option<bool>,
    pub cache_dir: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_dir_env_override: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GitHubConfig {
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id_env_override: Option<bool>,
    pub client_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret_env_override: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SiteConfig {
    pub allow_registration: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_registration_env_override: Option<bool>,
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
