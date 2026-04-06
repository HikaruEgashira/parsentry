//! Content-addressable file cache with namespace isolation
//!
//! This crate provides a generic caching layer with file-based persistence,
//! namespace-based isolation, and configurable cleanup policies.

pub mod cleanup;
pub mod entry;
pub mod key;
pub mod storage;

pub use cleanup::{CleanupManager, CleanupPolicy, CleanupStats, CleanupTrigger};
pub use entry::{CacheEntry, CacheMetadata};
pub use key::{hash_key, CACHE_VERSION};
pub use storage::CacheStorage;

use anyhow::Result;
use std::path::Path;

/// Content-addressable file cache with namespace isolation
pub struct Cache {
    storage: CacheStorage,
    cleanup: CleanupManager,
    enabled: bool,
}

impl Cache {
    /// Create a new cache with default configuration
    pub fn new<P: AsRef<Path>>(cache_dir: P) -> Result<Self> {
        let cache_dir = cache_dir.as_ref().to_path_buf();
        let storage = CacheStorage::new(&cache_dir)?;
        let cleanup = CleanupManager::new(&cache_dir)?;

        Ok(Self {
            storage,
            cleanup,
            enabled: true,
        })
    }

    /// Create a cache with custom cleanup configuration
    pub fn with_cleanup_config<P: AsRef<Path>>(
        cache_dir: P,
        policy: CleanupPolicy,
        trigger: CleanupTrigger,
    ) -> Result<Self> {
        let cache_dir = cache_dir.as_ref().to_path_buf();
        let storage = CacheStorage::new(&cache_dir)?;
        let cleanup = CleanupManager::with_config(&cache_dir, policy, trigger)?;

        Ok(Self {
            storage,
            cleanup,
            enabled: true,
        })
    }

    /// Disable the cache (no-op operations)
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Enable the cache
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Check if cache is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Get a cached value by namespace and key
    pub fn get(&self, namespace: &str, key: &str) -> Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        log::debug!("Cache lookup: ns={}, key={}", namespace, &key[..key.len().min(8)]);

        if let Some(entry) = self.storage.get(namespace, key)? {
            log::info!("Cache hit: {}", &key[..key.len().min(8)]);
            Ok(Some(entry.value))
        } else {
            log::info!("Cache miss: {}", &key[..key.len().min(8)]);
            Ok(None)
        }
    }

    /// Set a cached value under a namespace and key
    pub fn set(&self, namespace: &str, key: &str, value: &str, input_size: usize) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let entry = CacheEntry::new(
            CACHE_VERSION.to_string(),
            namespace.to_string(),
            key.to_string(),
            value.to_string(),
            input_size,
        );

        self.storage.set(&entry)?;
        log::info!("Cache stored: ns={}, key={}", namespace, &key[..key.len().min(8)]);

        Ok(())
    }

    /// Check if periodic cleanup should run
    pub fn should_cleanup_periodic(&self) -> Result<bool> {
        self.cleanup.should_run_periodic_cleanup()
    }

    /// Check if cache is over size limit
    pub fn should_cleanup_size(&self) -> Result<bool> {
        self.cleanup.is_over_size_limit()
    }

    /// Run stale entry cleanup
    pub fn cleanup_stale(&self) -> Result<CleanupStats> {
        self.cleanup.cleanup_stale_entries()
    }

    /// Run size-based cleanup
    pub fn cleanup_by_size(&self) -> Result<CleanupStats> {
        self.cleanup.cleanup_by_size()
    }

    /// Get cache statistics
    pub fn stats(&self) -> Result<CacheStats> {
        let total_size = self.storage.total_size()?;
        let entry_count = self.storage.entry_count()?;

        Ok(CacheStats {
            total_entries: entry_count,
            total_size_bytes: total_size,
            total_size_mb: total_size / 1_048_576,
        })
    }

    /// Clear all cache entries
    pub fn clear_all(&self) -> Result<usize> {
        self.storage.clear_all()
    }

    /// Get the cache directory path
    pub fn cache_dir(&self) -> &Path {
        self.storage.cache_dir()
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_entries: usize,
    pub total_size_bytes: u64,
    pub total_size_mb: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_cache_creation() {
        let temp_dir = TempDir::new().unwrap();
        let cache = Cache::new(temp_dir.path()).unwrap();
        assert!(cache.is_enabled());
    }

    #[test]
    fn test_cache_get_set() {
        let temp_dir = TempDir::new().unwrap();
        let cache = Cache::new(temp_dir.path()).unwrap();

        let ns = "test-ns";
        let key = "abc123def456";
        let value = "test value";

        // Cache miss
        let result = cache.get(ns, key).unwrap();
        assert!(result.is_none());

        // Set cache
        cache.set(ns, key, value, 42).unwrap();

        // Cache hit
        let result = cache.get(ns, key).unwrap();
        assert_eq!(result, Some(value.to_string()));
    }

    #[test]
    fn test_cache_disabled() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = Cache::new(temp_dir.path()).unwrap();

        cache.disable();
        assert!(!cache.is_enabled());

        // Set should be no-op
        cache.set("ns", "key123", "value", 5).unwrap();

        // Get should return None
        let result = cache.get("ns", "key123").unwrap();
        assert!(result.is_none());

        // Re-enable
        cache.enable();
        assert!(cache.is_enabled());
    }

    #[test]
    fn test_cache_stats() {
        let temp_dir = TempDir::new().unwrap();
        let cache = Cache::new(temp_dir.path()).unwrap();

        cache.set("ns", "key1abc", "value1", 10).unwrap();
        cache.set("ns", "key2def", "value2", 10).unwrap();

        let stats = cache.stats().unwrap();
        assert_eq!(stats.total_entries, 2);
        assert!(stats.total_size_bytes > 0);
    }

    #[test]
    fn test_cache_clear_all() {
        let temp_dir = TempDir::new().unwrap();
        let cache = Cache::new(temp_dir.path()).unwrap();

        cache.set("ns", "key1abc", "value1", 10).unwrap();
        cache.set("ns", "key2def", "value2", 10).unwrap();

        let stats = cache.stats().unwrap();
        assert_eq!(stats.total_entries, 2);

        let removed = cache.clear_all().unwrap();
        assert_eq!(removed, 2);

        let stats = cache.stats().unwrap();
        assert_eq!(stats.total_entries, 0);
    }

    #[test]
    fn test_stats_mb_calculation() {
        let temp_dir = TempDir::new().unwrap();
        let cache = Cache::new(temp_dir.path()).unwrap();

        let stats = cache.stats().unwrap();
        assert_eq!(stats.total_size_bytes, 0);
        assert_eq!(stats.total_size_mb, 0);

        cache.set("ns", "key1abc", &"x".repeat(1000), 1000).unwrap();
        let stats = cache.stats().unwrap();
        assert!(stats.total_size_bytes > 0);
        assert_eq!(stats.total_size_mb, stats.total_size_bytes / 1_048_576);
    }

    #[test]
    fn test_should_cleanup_periodic_delegates() {
        let temp_dir = TempDir::new().unwrap();
        let cache = Cache::new(temp_dir.path()).unwrap();
        let result = cache.should_cleanup_periodic().unwrap();
        assert!(result);
    }

    #[test]
    fn test_should_cleanup_size_delegates() {
        let temp_dir = TempDir::new().unwrap();
        let cache = Cache::new(temp_dir.path()).unwrap();
        let result = cache.should_cleanup_size().unwrap();
        assert!(!result);
    }

    #[test]
    fn test_cleanup_stale_delegates() {
        let temp_dir = TempDir::new().unwrap();
        let cache = Cache::new(temp_dir.path()).unwrap();
        let stats = cache.cleanup_stale().unwrap();
        assert_eq!(stats.removed_count, 0);
        assert_eq!(stats.freed_bytes, 0);
    }

    #[test]
    fn test_cleanup_by_size_delegates() {
        let temp_dir = TempDir::new().unwrap();
        let cache = Cache::new(temp_dir.path()).unwrap();
        let stats = cache.cleanup_by_size().unwrap();
        assert_eq!(stats.removed_count, 0);
        assert_eq!(stats.freed_bytes, 0);
    }

    #[test]
    fn test_stats_mb_is_division_not_modulo() {
        let temp_dir = TempDir::new().unwrap();
        let cache = Cache::new(temp_dir.path()).unwrap();

        cache.set("ns", "key1abc", &"x".repeat(1000), 1000).unwrap();
        let stats = cache.stats().unwrap();

        assert_eq!(stats.total_size_mb, 0);
        assert!(stats.total_size_bytes > 0);
        assert!(stats.total_size_mb < stats.total_size_bytes);
    }

    #[test]
    fn test_with_cleanup_config() {
        let temp_dir = TempDir::new().unwrap();
        let policy = CleanupPolicy {
            max_cache_size_mb: 100,
            max_age_days: 30,
            max_idle_days: 10,
            remove_version_mismatch: true,
        };
        let trigger = CleanupTrigger::Manual;

        let cache = Cache::with_cleanup_config(temp_dir.path(), policy, trigger).unwrap();
        assert!(cache.is_enabled());
        assert!(!cache.should_cleanup_periodic().unwrap());
    }

    #[test]
    fn test_should_cleanup_size_with_data() {
        let temp_dir = TempDir::new().unwrap();
        let policy = CleanupPolicy::default();
        let trigger = CleanupTrigger::OnSizeLimit { threshold_mb: 0 };

        let cache = Cache::with_cleanup_config(temp_dir.path(), policy, trigger).unwrap();
        cache.set("ns", "key1abc", "value", 5).unwrap();

        assert!(cache.should_cleanup_size().unwrap());
    }

    #[test]
    fn test_cleanup_stale_with_stale_data() {
        let temp_dir = TempDir::new().unwrap();
        let policy = CleanupPolicy {
            max_cache_size_mb: 500,
            max_age_days: 90,
            max_idle_days: 30,
            remove_version_mismatch: true,
        };
        let trigger = CleanupTrigger::Manual;

        let cache = Cache::with_cleanup_config(temp_dir.path(), policy, trigger).unwrap();

        let entry = CacheEntry::new(
            "0.0.1".to_string(),
            "ns".to_string(),
            "stalekey".to_string(),
            "resp".to_string(),
            10,
        );
        let dir = temp_dir.path().join("ns").join("st");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("stalekey.json"), serde_json::to_string(&entry).unwrap()).unwrap();

        let stats = cache.cleanup_stale().unwrap();
        assert_eq!(stats.removed_count, 1);
        assert!(stats.freed_bytes > 0);
    }

    #[test]
    fn test_cleanup_by_size_with_data() {
        let temp_dir = TempDir::new().unwrap();
        let policy = CleanupPolicy {
            max_cache_size_mb: 0,
            max_age_days: 90,
            max_idle_days: 30,
            remove_version_mismatch: true,
        };
        let trigger = CleanupTrigger::Manual;

        let cache = Cache::with_cleanup_config(temp_dir.path(), policy, trigger).unwrap();

        let entry = CacheEntry::new(
            CACHE_VERSION.to_string(),
            "ns".to_string(),
            "sizekey".to_string(),
            "resp".to_string(),
            10,
        );
        let dir = temp_dir.path().join("ns").join("si");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("sizekey.json"), serde_json::to_string(&entry).unwrap()).unwrap();

        let stats = cache.cleanup_by_size().unwrap();
        assert_eq!(stats.removed_count, 1);
        assert!(stats.freed_bytes > 0);
    }
}
