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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub jwt_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub upload_dir: String,
    pub cache_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfig {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteConfig {
    pub allow_registration: bool,
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
            },
            server: ServerConfig {
                host: app_config.server.host,
                port: app_config.server.port,
            },
            auth: AuthConfig {
                jwt_secret: app_config.auth.jwt_secret,
            },
            storage: StorageConfig {
                upload_dir: app_config.storage.upload_dir,
                cache_dir: app_config.storage.cache_dir,
            },
            github: GitHubConfig {
                client_id: app_config.github.client_id,
                client_secret: app_config.github.client_secret,
            },
            site: SiteConfig {
                allow_registration: app_config.site.allow_registration,
            },
        }
    }
}

impl From<Config> for config::AppConfig {
    fn from(domain_config: Config) -> Self {
        Self {
            database: config::DatabaseConfig {
                url: domain_config.database.url,
            },
            server: config::ServerConfig {
                host: domain_config.server.host,
                port: domain_config.server.port,
            },
            auth: config::AuthConfig {
                jwt_secret: domain_config.auth.jwt_secret,
            },
            storage: config::StorageConfig {
                upload_dir: domain_config.storage.upload_dir,
                cache_dir: domain_config.storage.cache_dir,
            },
            github: config::GitHubConfig {
                client_id: domain_config.github.client_id,
                client_secret: domain_config.github.client_secret,
            },
            site: config::SiteConfig {
                allow_registration: domain_config.site.allow_registration,
            },
        }
    }
}
