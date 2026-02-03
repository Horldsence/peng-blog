//! Configuration types for the blog application.
//!
//! This module defines types for configuration management,
//! including response types and update requests.

use serde::{Deserialize, Serialize};

/// Configuration response type
///
/// This type represents the full configuration structure
/// that can be returned via API.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigResponse {
    pub database: DatabaseConfigResponse,
    pub server: ServerConfigResponse,
    pub auth: AuthConfigResponse,
    pub storage: StorageConfigResponse,
    pub github: GitHubConfigResponse,
    pub site: SiteConfigResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfigResponse {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfigResponse {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfigResponse {
    pub jwt_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfigResponse {
    pub upload_dir: String,
    pub cache_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubConfigResponse {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteConfigResponse {
    pub allow_registration: bool,
}

/// Configuration update request
///
/// All fields are optional to support partial updates.
/// Only the fields that are present will be updated.
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

impl From<config::AppConfig> for ConfigResponse {
    fn from(config: config::AppConfig) -> Self {
        Self {
            database: DatabaseConfigResponse {
                url: config.database.url,
            },
            server: ServerConfigResponse {
                host: config.server.host,
                port: config.server.port,
            },
            auth: AuthConfigResponse {
                jwt_secret: config.auth.jwt_secret,
            },
            storage: StorageConfigResponse {
                upload_dir: config.storage.upload_dir,
                cache_dir: config.storage.cache_dir,
            },
            github: GitHubConfigResponse {
                client_id: config.github.client_id,
                client_secret: config.github.client_secret,
            },
            site: SiteConfigResponse {
                allow_registration: config.site.allow_registration,
            },
        }
    }
}
