//! Configuration types for the blog application.
//!
//! This module defines types for configuration management,
//! including domain types and update requests.

use crate::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// Configuration domain type
///
/// This type represents the full configuration structure
/// that can be returned via API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub server: ServerConfig,
    pub auth: AuthConfig,
    pub storage: StorageConfig,
    pub github: GitHubConfig,
    pub site: SiteConfig,
}

/// Public configuration exposed to frontend without authentication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicConfig {
    pub allow_registration: bool,
}

impl From<&Config> for PublicConfig {
    fn from(config: &Config) -> Self {
        Self {
            allow_registration: config.site.allow_registration,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_env_override: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_env_override: Option<bool>,
    pub port: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_env_override: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwt_secret_env_override: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub upload_dir: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_dir_env_override: Option<bool>,
    pub cache_dir: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_dir_env_override: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id_env_override: Option<bool>,
    pub client_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret_env_override: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteConfig {
    pub allow_registration: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_registration_env_override: Option<bool>,
}

/// Repository for configuration persistence operations
#[async_trait]
pub trait ConfigRepository: Send + Sync {
    /// Get the current configuration
    async fn get_config(&self) -> Result<Config>;

    /// Save configuration
    async fn save_config(&self, config: &Config) -> Result<()>;
}

/// Configuration update request
///
/// All fields are optional to support partial updates.
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateConfigRequest {
    pub database: Option<UpdateDatabaseConfig>,
    pub server: Option<UpdateServerConfig>,
    pub auth: Option<UpdateAuthConfig>,
    pub storage: Option<UpdateStorageConfig>,
    pub github: Option<UpdateGitHubConfig>,
    pub site: Option<UpdateSiteConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateDatabaseConfig {
    pub url: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateServerConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateAuthConfig {
    pub jwt_secret: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateStorageConfig {
    pub upload_dir: Option<String>,
    pub cache_dir: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateGitHubConfig {
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateSiteConfig {
    pub allow_registration: Option<bool>,
}

impl From<config::AppConfig> for Config {
    fn from(app_config: config::AppConfig) -> Self {
        Self {
            database: DatabaseConfig {
                url: app_config.database.url,
                url_env_override: app_config.database.url_env_override,
            },
            server: ServerConfig {
                host: app_config.server.host,
                host_env_override: app_config.server.host_env_override,
                port: app_config.server.port,
                port_env_override: app_config.server.port_env_override,
            },
            auth: AuthConfig {
                jwt_secret: app_config.auth.jwt_secret,
                jwt_secret_env_override: app_config.auth.jwt_secret_env_override,
            },
            storage: StorageConfig {
                upload_dir: app_config.storage.upload_dir,
                upload_dir_env_override: app_config.storage.upload_dir_env_override,
                cache_dir: app_config.storage.cache_dir,
                cache_dir_env_override: app_config.storage.cache_dir_env_override,
            },
            github: GitHubConfig {
                client_id: app_config.github.client_id,
                client_id_env_override: app_config.github.client_id_env_override,
                client_secret: app_config.github.client_secret,
                client_secret_env_override: app_config.github.client_secret_env_override,
            },
            site: SiteConfig {
                allow_registration: app_config.site.allow_registration,
                allow_registration_env_override: app_config.site.allow_registration_env_override,
            },
        }
    }
}

impl From<Config> for config::AppConfig {
    fn from(domain_config: Config) -> Self {
        Self {
            database: config::DatabaseConfig {
                url: domain_config.database.url,
                url_env_override: domain_config.database.url_env_override,
            },
            server: config::ServerConfig {
                host: domain_config.server.host,
                host_env_override: domain_config.server.host_env_override,
                port: domain_config.server.port,
                port_env_override: domain_config.server.port_env_override,
            },
            auth: config::AuthConfig {
                jwt_secret: domain_config.auth.jwt_secret,
                jwt_secret_env_override: domain_config.auth.jwt_secret_env_override,
            },
            storage: config::StorageConfig {
                upload_dir: domain_config.storage.upload_dir,
                upload_dir_env_override: domain_config.storage.upload_dir_env_override,
                cache_dir: domain_config.storage.cache_dir,
                cache_dir_env_override: domain_config.storage.cache_dir_env_override,
            },
            github: config::GitHubConfig {
                client_id: domain_config.github.client_id,
                client_id_env_override: domain_config.github.client_id_env_override,
                client_secret: domain_config.github.client_secret,
                client_secret_env_override: domain_config.github.client_secret_env_override,
            },
            site: config::SiteConfig {
                allow_registration: domain_config.site.allow_registration,
                allow_registration_env_override: domain_config.site.allow_registration_env_override,
            },
        }
    }
}
