use domain::{Config, ConfigRepository, Result, UpdateConfigRequest};
use std::sync::Arc;

pub struct ConfigService {
    repo: Arc<dyn ConfigRepository>,
}

impl ConfigService {
    pub fn new(repo: Arc<dyn ConfigRepository>) -> Self {
        Self { repo }
    }

    /// Get the current configuration
    pub async fn get(&self) -> Result<Config> {
        self.repo.get_config().await
    }

    /// Update configuration
    pub async fn update(&self, request: UpdateConfigRequest) -> Result<Config> {
        let mut config = self.repo.get_config().await?;

        if let Some(database) = request.database {
            if let Some(url) = database.url {
                config.database.url = url;
            }
        }

        if let Some(server) = request.server {
            if let Some(host) = server.host {
                config.server.host = host;
            }
            if let Some(port) = server.port {
                config.server.port = port;
            }
        }

        if let Some(auth) = request.auth {
            if let Some(jwt_secret) = auth.jwt_secret {
                config.auth.jwt_secret = jwt_secret;
            }
        }

        if let Some(storage) = request.storage {
            if let Some(upload_dir) = storage.upload_dir {
                config.storage.upload_dir = upload_dir;
            }
            if let Some(cache_dir) = storage.cache_dir {
                config.storage.cache_dir = cache_dir;
            }
        }

        if let Some(github) = request.github {
            if let Some(client_id) = github.client_id {
                config.github.client_id = client_id;
            }
            if let Some(client_secret) = github.client_secret {
                config.github.client_secret = client_secret;
            }
        }

        if let Some(site) = request.site {
            if let Some(allow_registration) = site.allow_registration {
                config.site.allow_registration = allow_registration;
            }
        }

        self.repo.save_config(&config).await?;
        Ok(config)
    }
}
