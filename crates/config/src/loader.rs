use super::types::Config;
use super::ConfigError;
use std::path::Path;

const DEFAULT_CONFIG: &str = r#"
[database]
url = "postgresql://postgres:postgres@localhost:5432/peng_blog"

[server]
host = "0.0.0.0"
port = 3000

[auth]
jwt_secret = "change-this-secret-in-production"

[storage]
upload_dir = "./uploads"
cache_dir = "./cache"

[github]
client_id = ""
client_secret = ""
"#;

pub fn load_config() -> Result<Config, ConfigError> {
    load_config_from_path("config/config.toml")
}

pub fn load_config_from_path<P: AsRef<Path>>(path: P) -> Result<Config, ConfigError> {
    let path = path.as_ref();

    let mut config: Config = toml::from_str(DEFAULT_CONFIG)
        .map_err(|e| ConfigError::Parse(format!("Failed to parse default config: {}", e)))?;

    if path.exists() {
        let content = std::fs::read_to_string(path).map_err(|e| {
            ConfigError::Io(format!(
                "Failed to read config file {}: {}",
                path.display(),
                e
            ))
        })?;

        let file_config: Config = toml::from_str(&content).map_err(|e| {
            ConfigError::Parse(format!(
                "Failed to parse config file {}: {}",
                path.display(),
                e
            ))
        })?;

        merge_config(&mut config, file_config);
    }

    load_from_env(&mut config)?;

    config.validate()?;

    Ok(config)
}

fn merge_config(base: &mut Config, overlay: Config) {
    if !overlay.database.url.is_empty() {
        base.database.url = overlay.database.url;
    }
    if !overlay.server.host.is_empty() {
        base.server.host = overlay.server.host;
    }
    if overlay.server.port != 0 {
        base.server.port = overlay.server.port;
    }
    if !overlay.auth.jwt_secret.is_empty() {
        base.auth.jwt_secret = overlay.auth.jwt_secret;
    }
    if !overlay.storage.upload_dir.is_empty() {
        base.storage.upload_dir = overlay.storage.upload_dir;
    }
    if !overlay.storage.cache_dir.is_empty() {
        base.storage.cache_dir = overlay.storage.cache_dir;
    }
    if !overlay.github.client_id.is_empty() {
        base.github.client_id = overlay.github.client_id;
    }
    if !overlay.github.client_secret.is_empty() {
        base.github.client_secret = overlay.github.client_secret;
    }
}

fn load_from_env(config: &mut Config) -> Result<(), ConfigError> {
    if let Ok(url) = std::env::var("DATABASE_URL") {
        config.database.url = url;
    }
    if let Ok(host) = std::env::var("HOST") {
        config.server.host = host;
    }
    if let Ok(port) = std::env::var("PORT") {
        config.server.port = port
            .parse()
            .map_err(|_| ConfigError::Validation(format!("Invalid PORT value: {}", port)))?;
    }
    if let Ok(secret) = std::env::var("JWT_SECRET") {
        config.auth.jwt_secret = secret;
    }
    if let Ok(dir) = std::env::var("UPLOAD_DIR") {
        config.storage.upload_dir = dir;
    }
    if let Ok(dir) = std::env::var("CACHE_DIR") {
        config.storage.cache_dir = dir;
    }
    if let Ok(id) = std::env::var("GITHUB_CLIENT_ID") {
        config.github.client_id = id;
    }
    if let Ok(secret) = std::env::var("GITHUB_CLIENT_SECRET") {
        config.github.client_secret = secret;
    }

    Ok(())
}
