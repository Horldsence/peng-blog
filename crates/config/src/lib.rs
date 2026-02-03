mod loader;
pub mod types;

pub use loader::default_config_path;
pub use loader::load_config;
pub use loader::load_config_from_path;
pub use loader::save_config;
pub use types::{
    AuthConfig, Config, DatabaseConfig, GitHubConfig, ServerConfig, SiteConfig, StorageConfig,
};

pub use load_config as load;
pub use Config as AppConfig;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    Io(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Validation error: {0}")]
    Validation(String),
}

impl From<ConfigError> for String {
    fn from(err: ConfigError) -> Self {
        err.to_string()
    }
}
