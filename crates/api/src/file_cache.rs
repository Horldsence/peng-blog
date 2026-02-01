//! File-based cache implementation for persistent caching
//!
//! Provides a simple file-based caching mechanism that stores cached data
//! as JSON files on disk, surviving server restarts.

use crate::ApiError;
use chrono::{DateTime, Utc};
use serde::{de::DeserializeOwned, Serialize};
use std::{io, path::Path, path::PathBuf};
use tokio::fs;

/// File-based cache manager
#[derive(Debug, Clone)]
pub struct FileCache {
    /// Directory where cache files are stored
    cache_dir: PathBuf,
}

impl FileCache {
    /// Create a new file cache manager
    ///
    /// # Arguments
    ///
    /// * `cache_dir` - Directory path for storing cache files
    ///
    /// # Returns
    ///
    /// Returns `Self` if cache directory exists or can be created
    pub fn new(cache_dir: impl AsRef<Path>) -> Result<Self, io::Error> {
        let cache_dir = PathBuf::from(cache_dir.as_ref());
        Ok(Self { cache_dir })
    }

    /// Initialize cache directory (create if not exists)
    ///
    /// Call this during application startup to ensure the cache directory exists.
    pub async fn initialize(&self) -> Result<(), io::Error> {
        fs::create_dir_all(&self.cache_dir).await?;
        tracing::info!("Cache directory initialized: {:?}", self.cache_dir);
        Ok(())
    }

    /// Get the file path for a given cache key
    fn get_cache_path(&self, key: &str) -> PathBuf {
        self.cache_dir.join(format!("{}.json", key))
    }

    /// Retrieve cached data
    ///
    /// # Arguments
    ///
    /// * `key` - Cache key (filename without .json extension)
    ///
    /// # Returns
    ///
    /// * `Some(T)` - Cached data if found and deserializable
    /// * `None` - Cache miss or error
    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, ApiError> {
        let cache_path = self.get_cache_path(key);

        // Check if cache file exists
        match fs::metadata(&cache_path).await {
            Ok(_) => {},
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                tracing::debug!("Cache miss: key={}", key);
                return Ok(None);
            },
            Err(e) => {
                return Err(ApiError::internal(format!("Failed to read cache metadata: {}", e)));
            }
        }

        // Read cache file
        let content = fs::read_to_string(&cache_path).await
            .map_err(|e| ApiError::internal(format!("Failed to read cache file: {}", e)))?;

        // Deserialize
        let data: T = serde_json::from_str(&content)
            .map_err(|e| ApiError::internal(format!("Failed to deserialize cache: {}", e)))?;

        tracing::debug!("Cache hit: key={}", key);
        Ok(Some(data))
    }

    /// Store data in cache
    ///
    /// # Arguments
    ///
    /// * `key` - Cache key (filename without .json extension)
    /// * `data` - Data to cache (must be serializable)
    pub async fn set<T: Serialize>(&self, key: &str, data: &T) -> Result<(), ApiError> {
        let cache_path = self.get_cache_path(key);

        // Serialize data
        let json = serde_json::to_string_pretty(data)
            .map_err(|e| ApiError::internal(format!("Failed to serialize cache data: {}", e)))?;

        // Write to file
        fs::write(&cache_path, json).await
            .map_err(|e| ApiError::internal(format!("Failed to write cache file: {}", e)))?;

        tracing::info!("Cache updated: key={}, path={:?}", key, cache_path);
        Ok(())
    }

    /// Check if cache exists and is not expired
    ///
    /// # Arguments
    ///
    /// * `key` - Cache key
    /// * `max_age_hours` - Maximum age in hours before expiry
    ///
    /// # Returns
    ///
    /// * `true` - Cache exists and is not expired
    /// * `false` - Cache does not exist or is expired
    pub async fn is_valid(&self, key: &str, max_age_hours: i64) -> bool {
        let cache_path = self.get_cache_path(key);

        // Check if file exists
        let metadata = match fs::metadata(&cache_path).await {
            Ok(m) => m,
            Err(_) => return false,
        };

        // Check modification time
        let modified = match metadata.modified() {
            Ok(m) => m,
            Err(_) => return false,
        };

        let modified_dt: DateTime<Utc> = modified.into();
        let now = Utc::now();
        let age = now.signed_duration_since(modified_dt);

        !age.num_hours().ge(&max_age_hours)
    }

    /// Delete cache file
    ///
    /// # Arguments
    ///
    /// * `key` - Cache key to delete
    pub async fn delete(&self, key: &str) -> Result<(), ApiError> {
        let cache_path = self.get_cache_path(key);

        fs::remove_file(&cache_path).await
            .map_err(|e| ApiError::internal(format!("Failed to delete cache file: {}", e)))?;

        tracing::info!("Cache deleted: key={}", key);
        Ok(())
    }

    /// Clear all cache files
    ///
    /// Use with caution - this removes all files in the cache directory.
    pub async fn clear(&self) -> Result<(), ApiError> {
        let mut entries = fs::read_dir(&self.cache_dir).await
            .map_err(|e| ApiError::internal(format!("Failed to read cache directory: {}", e)))?;

        let mut count = 0;
        while let Some(entry) = entries.next_entry().await
            .map_err(|e| ApiError::internal(format!("Failed to read cache entry: {}", e)))?
        {
            let path = entry.path();
            if path.extension().map(|e| e == "json").unwrap_or(false) {
                fs::remove_file(&path).await
                    .map_err(|e| ApiError::internal(format!("Failed to delete cache file: {}", e)))?;
                count += 1;
            }
        }

        tracing::info!("Cache cleared: {} files deleted", count);
        Ok(())
    }

    /// Get cache directory path
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_file_cache_set_get() {
        let temp_dir = std::env::temp_dir().join("test_cache");
        let _ = fs::remove_dir_all(&temp_dir).await;

        let cache = FileCache::new(&temp_dir).unwrap();
        cache.initialize().await.unwrap();

        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct TestData {
            name: String,
            value: i32,
        }

        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };

        // Test set and get
        cache.set("test_key", &data).await.unwrap();
        let retrieved: Option<TestData> = cache.get("test_key").await.unwrap();

        assert_eq!(retrieved, Some(data));

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir).await;
    }

    #[tokio::test]
    async fn test_file_cache_expiry() {
        let temp_dir = std::env::temp_dir().join("test_cache_expiry");
        let _ = fs::remove_dir_all(&temp_dir).await;

        let cache = FileCache::new(&temp_dir).unwrap();
        cache.initialize().await.unwrap();

        cache.set("expiry_test", &"data").await.unwrap();

        // Should be valid for 1 hour
        assert!(cache.is_valid("expiry_test", 1).await);

        // Should be invalid for 0 hours (expired immediately)
        assert!(!cache.is_valid("expiry_test", 0).await);

        // Cleanup
        let _ = fs::remove_dir_all(&temp_dir).await;
    }
}
