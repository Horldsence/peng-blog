use async_trait::async_trait;
use domain::{Config, ConfigRepository, Error, Result};
use std::path::Path;

use config::{default_config_path, load_config_from_path, save_config, AppConfig};

/// Implementation of ConfigRepository for file-based configuration
pub struct ConfigRepositoryImpl {
    config_path: Option<&'static Path>,
}

impl ConfigRepositoryImpl {
    /// Create a new ConfigRepositoryImpl with default path
    pub fn new() -> Self {
        Self {
            config_path: Some(default_config_path()),
        }
    }

    /// Create a new ConfigRepositoryImpl with custom path
    pub fn with_path(path: &'static Path) -> Self {
        Self {
            config_path: Some(path),
        }
    }
}

impl Default for ConfigRepositoryImpl {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ConfigRepository for ConfigRepositoryImpl {
    async fn get_config(&self) -> Result<Config> {
        let path = self.config_path.unwrap_or(default_config_path());
        let app_config = load_config_from_path_async(path)
            .await
            .map_err(|e| Error::Internal(format!("Failed to load config: {}", e)))?;
        Ok(Config::from(app_config))
    }

    async fn save_config(&self, config: &Config) -> Result<()> {
        let path = self.config_path.unwrap_or(default_config_path());
        let app_config = AppConfig::from(config.clone());
        save_config_async(&app_config, path)
            .await
            .map_err(|e| Error::Internal(format!("Failed to save config: {}", e)))?;
        Ok(())
    }
}

async fn load_config_from_path_async<P: AsRef<Path> + Send + 'static>(
    path: P,
) -> Result<AppConfig> {
    tokio::task::spawn_blocking(move || load_config_from_path(path))
        .await
        .map_err(|e| Error::Internal(format!("Task join error: {}", e)))?
        .map_err(|e| Error::Internal(format!("Failed to load config: {}", e)))
}

async fn save_config_async<P: AsRef<Path> + Send + 'static>(
    config: &AppConfig,
    path: P,
) -> Result<()> {
    let config = config.clone();
    let path = path.as_ref().to_path_buf();

    tokio::task::spawn_blocking(move || save_config(&config, path))
        .await
        .map_err(|e| Error::Internal(format!("Task join error: {}", e)))?
        .map_err(|e| Error::Internal(format!("Failed to save config: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_config() {
        let repo = ConfigRepositoryImpl::new();
        let config = repo.get_config().await;
        assert!(config.is_ok());
    }
}
