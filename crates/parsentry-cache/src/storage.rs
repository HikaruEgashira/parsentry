//! Cache storage management with file-based persistence

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::entry::CacheEntry;

/// Cache storage manager
pub struct CacheStorage {
    /// Root cache directory
    cache_dir: PathBuf,
}

impl CacheStorage {
    /// Create a new cache storage with the given directory
    pub fn new<P: AsRef<Path>>(cache_dir: P) -> Result<Self> {
        let cache_dir = cache_dir.as_ref().to_path_buf();

        // Create cache directory if it doesn't exist
        if !cache_dir.exists() {
            fs::create_dir_all(&cache_dir)
                .with_context(|| format!("Failed to create cache directory: {}", cache_dir.display()))?;
        }

        Ok(Self { cache_dir })
    }

    /// Get the cache file path for a given hash
    ///
    /// Uses first 2 characters as subdirectory for distribution
    /// Example: abc123... -> cache_dir/genai/gpt-4/ab/abc123....json
    fn get_cache_path(&self, provider: &str, model: &str, hash: &str) -> PathBuf {
        let prefix = if hash.len() >= 2 {
            &hash[..2]
        } else {
            hash
        };

        self.cache_dir
            .join(provider)
            .join(model)
            .join(prefix)
            .join(format!("{}.json", hash))
    }

    /// Check if a cache entry exists
    pub fn exists(&self, provider: &str, model: &str, hash: &str) -> bool {
        let path = self.get_cache_path(provider, model, hash);
        path.exists()
    }

    /// Get a cache entry by hash
    pub fn get(&self, provider: &str, model: &str, hash: &str) -> Result<Option<CacheEntry>> {
        let path = self.get_cache_path(provider, model, hash);

        if !path.exists() {
            return Ok(None);
        }

        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read cache file: {}", path.display()))?;

        let mut entry: CacheEntry = serde_json::from_str(&content)
            .with_context(|| format!("Failed to parse cache entry: {}", path.display()))?;

        // Update access metadata
        entry.record_access();

        // Write back updated metadata
        let updated_content = serde_json::to_string_pretty(&entry)?;
        fs::write(&path, updated_content)
            .with_context(|| format!("Failed to update cache metadata: {}", path.display()))?;

        Ok(Some(entry))
    }

    /// Set a cache entry
    pub fn set(&self, entry: &CacheEntry) -> Result<()> {
        let path = self.get_cache_path(&entry.provider, &entry.model, &entry.prompt_hash);

        // Create parent directories
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create cache subdirectory: {}", parent.display()))?;
        }

        // Write entry to file
        let content = serde_json::to_string_pretty(entry)
            .context("Failed to serialize cache entry")?;

        fs::write(&path, content)
            .with_context(|| format!("Failed to write cache file: {}", path.display()))?;

        log::debug!("Cache entry saved: {}", path.display());

        Ok(())
    }

    /// Delete a cache entry
    pub fn delete(&self, provider: &str, model: &str, hash: &str) -> Result<()> {
        let path = self.get_cache_path(provider, model, hash);

        if path.exists() {
            fs::remove_file(&path)
                .with_context(|| format!("Failed to delete cache file: {}", path.display()))?;
            log::debug!("Cache entry deleted: {}", path.display());
        }

        Ok(())
    }

    /// Get the cache directory path
    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
    }

    /// Calculate total cache size in bytes
    pub fn total_size(&self) -> Result<u64> {
        let mut total = 0u64;

        for entry in walkdir::WalkDir::new(&self.cache_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                if let Ok(metadata) = entry.metadata() {
                    total += metadata.len();
                }
            }
        }

        Ok(total)
    }

    /// Count total cache entries
    pub fn entry_count(&self) -> Result<usize> {
        let mut count = 0;

        for entry in walkdir::WalkDir::new(&self.cache_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "json") {
                count += 1;
            }
        }

        Ok(count)
    }

    /// Clear all cache entries
    pub fn clear_all(&self) -> Result<usize> {
        let mut removed = 0;

        for entry in walkdir::WalkDir::new(&self.cache_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() && entry.path().extension().map_or(false, |ext| ext == "json") {
                fs::remove_file(entry.path())?;
                removed += 1;
            }
        }

        Ok(removed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entry::CacheEntry;
    use tempfile::TempDir;

    #[test]
    fn test_storage_creation() {
        let temp_dir = TempDir::new().unwrap();
        let storage = CacheStorage::new(temp_dir.path()).unwrap();

        assert!(storage.cache_dir().exists());
    }

    #[test]
    fn test_set_and_get() {
        let temp_dir = TempDir::new().unwrap();
        let storage = CacheStorage::new(temp_dir.path()).unwrap();

        let entry = CacheEntry::new(
            "1.0.0".to_string(),
            "genai".to_string(),
            "gpt-4".to_string(),
            "abc123".to_string(),
            "test response".to_string(),
            100,
        );

        storage.set(&entry).unwrap();

        let retrieved = storage.get("genai", "gpt-4", "abc123").unwrap();
        assert!(retrieved.is_some());

        let retrieved_entry = retrieved.unwrap();
        assert_eq!(retrieved_entry.response, "test response");
        assert_eq!(retrieved_entry.metadata.access_count, 1);
    }

    #[test]
    fn test_exists() {
        let temp_dir = TempDir::new().unwrap();
        let storage = CacheStorage::new(temp_dir.path()).unwrap();

        assert!(!storage.exists("genai", "gpt-4", "nonexistent"));

        let entry = CacheEntry::new(
            "1.0.0".to_string(),
            "genai".to_string(),
            "gpt-4".to_string(),
            "abc123".to_string(),
            "test".to_string(),
            10,
        );

        storage.set(&entry).unwrap();

        assert!(storage.exists("genai", "gpt-4", "abc123"));
    }

    #[test]
    fn test_delete() {
        let temp_dir = TempDir::new().unwrap();
        let storage = CacheStorage::new(temp_dir.path()).unwrap();

        let entry = CacheEntry::new(
            "1.0.0".to_string(),
            "genai".to_string(),
            "gpt-4".to_string(),
            "abc123".to_string(),
            "test".to_string(),
            10,
        );

        storage.set(&entry).unwrap();
        assert!(storage.exists("genai", "gpt-4", "abc123"));

        storage.delete("genai", "gpt-4", "abc123").unwrap();
        assert!(!storage.exists("genai", "gpt-4", "abc123"));
    }

    #[test]
    fn test_total_size() {
        let temp_dir = TempDir::new().unwrap();
        let storage = CacheStorage::new(temp_dir.path()).unwrap();

        let initial_size = storage.total_size().unwrap();

        let entry = CacheEntry::new(
            "1.0.0".to_string(),
            "genai".to_string(),
            "gpt-4".to_string(),
            "abc123".to_string(),
            "test response".to_string(),
            100,
        );

        storage.set(&entry).unwrap();

        let new_size = storage.total_size().unwrap();
        assert!(new_size > initial_size);
    }

    #[test]
    fn test_entry_count() {
        let temp_dir = TempDir::new().unwrap();
        let storage = CacheStorage::new(temp_dir.path()).unwrap();

        assert_eq!(storage.entry_count().unwrap(), 0);

        let entry1 = CacheEntry::new(
            "1.0.0".to_string(),
            "genai".to_string(),
            "gpt-4".to_string(),
            "abc123".to_string(),
            "test1".to_string(),
            10,
        );

        storage.set(&entry1).unwrap();
        assert_eq!(storage.entry_count().unwrap(), 1);

        let entry2 = CacheEntry::new(
            "1.0.0".to_string(),
            "genai".to_string(),
            "gpt-4".to_string(),
            "def456".to_string(),
            "test2".to_string(),
            10,
        );

        storage.set(&entry2).unwrap();
        assert_eq!(storage.entry_count().unwrap(), 2);
    }

    #[test]
    fn test_clear_all() {
        let temp_dir = TempDir::new().unwrap();
        let storage = CacheStorage::new(temp_dir.path()).unwrap();

        let entry1 = CacheEntry::new(
            "1.0.0".to_string(),
            "genai".to_string(),
            "gpt-4".to_string(),
            "abc123".to_string(),
            "test1".to_string(),
            10,
        );

        let entry2 = CacheEntry::new(
            "1.0.0".to_string(),
            "genai".to_string(),
            "gpt-4".to_string(),
            "def456".to_string(),
            "test2".to_string(),
            10,
        );

        storage.set(&entry1).unwrap();
        storage.set(&entry2).unwrap();

        assert_eq!(storage.entry_count().unwrap(), 2);

        let removed = storage.clear_all().unwrap();
        assert_eq!(removed, 2);
        assert_eq!(storage.entry_count().unwrap(), 0);
    }
}
