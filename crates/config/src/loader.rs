use super::types::Config;
use super::ConfigError;
use std::fs;
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

[site]
allow_registration = true

[indexnow]
enabled = false
api_key = ""
endpoint = "https://api.indexnow.org/IndexNow"
"#;

pub fn load_config() -> Result<Config, ConfigError> {
    let mut config = load_config_from_path("config/config.toml")?;
    load_from_env(&mut config)?;
    config.validate()?;
    Ok(config)
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
    base.site.allow_registration = overlay.site.allow_registration;
    base.indexnow.enabled = overlay.indexnow.enabled;
    if !overlay.indexnow.api_key.is_empty() {
        base.indexnow.api_key = overlay.indexnow.api_key;
    }
    if !overlay.indexnow.endpoint.is_empty() {
        base.indexnow.endpoint = overlay.indexnow.endpoint;
    }
}

fn load_from_env(config: &mut Config) -> Result<(), ConfigError> {
    if let Ok(url) = std::env::var("DATABASE_URL") {
        config.database.url = url;
        config.database.url_env_override = Some(true);
    }
    if let Ok(host) = std::env::var("HOST") {
        config.server.host = host;
        config.server.host_env_override = Some(true);
    }
    if let Ok(port) = std::env::var("PORT") {
        config.server.port = port.parse().unwrap_or(config.server.port);
        config.server.port_env_override = Some(true);
    }
    if let Ok(secret) = std::env::var("JWT_SECRET") {
        config.auth.jwt_secret = secret;
        config.auth.jwt_secret_env_override = Some(true);
    }
    if let Ok(dir) = std::env::var("UPLOAD_DIR") {
        config.storage.upload_dir = dir;
        config.storage.upload_dir_env_override = Some(true);
    }
    if let Ok(dir) = std::env::var("CACHE_DIR") {
        config.storage.cache_dir = dir;
        config.storage.cache_dir_env_override = Some(true);
    }
    if let Ok(client_id) = std::env::var("GITHUB_CLIENT_ID") {
        config.github.client_id = client_id;
        config.github.client_id_env_override = Some(true);
    }
    if let Ok(client_secret) = std::env::var("GITHUB_CLIENT_SECRET") {
        config.github.client_secret = client_secret;
        config.github.client_secret_env_override = Some(true);
    }
    if let Ok(allow) = std::env::var("ALLOW_REGISTRATION") {
        config.site.allow_registration = allow.parse().unwrap_or(config.site.allow_registration);
        config.site.allow_registration_env_override = Some(true);
    }
    if let Ok(api_key) = std::env::var("INDEXNOW_API_KEY") {
        config.indexnow.api_key = api_key;
        config.indexnow.api_key_env_override = Some(true);
    }

    Ok(())
}

pub fn save_config<P: AsRef<Path>>(config: &Config, path: P) -> Result<(), ConfigError> {
    let path = path.as_ref();

    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| {
                ConfigError::Io(format!(
                    "Failed to create config directory {}: {}",
                    parent.display(),
                    e
                ))
            })?;
        }
    }

    let toml_str = toml::to_string_pretty(config)
        .map_err(|e| ConfigError::Parse(format!("Failed to serialize config to TOML: {}", e)))?;

    fs::write(path, toml_str).map_err(|e| {
        ConfigError::Io(format!(
            "Failed to write config file {}: {}",
            path.display(),
            e
        ))
    })?;

    Ok(())
}

pub fn default_config_path() -> &'static Path {
    Path::new("config/config.toml")
}
