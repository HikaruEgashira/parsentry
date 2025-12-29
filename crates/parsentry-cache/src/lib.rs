//! Parsentry cache system for LLM responses
//!
//! This crate provides caching functionality for LLM agent responses,
//! enabling faster testing, reduced costs, and offline operation.

pub mod cleanup;
pub mod entry;
pub mod key;
pub mod storage;

pub use cleanup::{CleanupManager, CleanupPolicy, CleanupStats, CleanupTrigger};
pub use entry::{CacheEntry, CacheMetadata};
pub use key::{CacheKeyGenerator, CACHE_VERSION};
pub use storage::CacheStorage;

use anyhow::Result;
use std::path::Path;

/// Main cache interface
pub struct Cache {
    storage: CacheStorage,
    key_gen: CacheKeyGenerator,
    cleanup: CleanupManager,
    enabled: bool,
}

impl Cache {
    /// Create a new cache with default configuration
    pub fn new<P: AsRef<Path>>(cache_dir: P) -> Result<Self> {
        let cache_dir = cache_dir.as_ref().to_path_buf();
        let storage = CacheStorage::new(&cache_dir)?;
        let key_gen = CacheKeyGenerator::new();
        let cleanup = CleanupManager::new(&cache_dir)?;

        Ok(Self {
            storage,
            key_gen,
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
        let key_gen = CacheKeyGenerator::new();
        let cleanup = CleanupManager::with_config(&cache_dir, policy, trigger)?;

        Ok(Self {
            storage,
            key_gen,
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

    /// Get a cached response
    pub fn get(&self, prompt: &str, model: &str, agent: &str) -> Result<Option<String>> {
        if !self.enabled {
            return Ok(None);
        }

        let key = self.key_gen.generate_key(prompt, model, agent);
        log::debug!("Cache lookup: key={}, agent={}, model={}", &key[..8], agent, model);

        if let Some(entry) = self.storage.get(agent, model, &key)? {
            log::info!("Cache hit: {}", &key[..8]);
            Ok(Some(entry.response))
        } else {
            log::info!("Cache miss: {}", &key[..8]);
            Ok(None)
        }
    }

    /// Set a cached response
    pub fn set(&self, prompt: &str, model: &str, agent: &str, response: &str) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        let key = self.key_gen.generate_key(prompt, model, agent);
        let entry = CacheEntry::new(
            self.key_gen.version().to_string(),
            agent.to_string(),
            model.to_string(),
            key.clone(),
            response.to_string(),
            prompt.len(),
        );

        self.storage.set(&entry)?;
        log::info!("Cache stored: {}", &key[..8]);

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

        let prompt = "test prompt";
        let model = "gpt-4";
        let provider = "genai";
        let response = "test response";

        // Cache miss
        let result = cache.get(prompt, model, agent).unwrap();
        assert!(result.is_none());

        // Set cache
        cache.set(prompt, model, agent, response).unwrap();

        // Cache hit
        let result = cache.get(prompt, model, agent).unwrap();
        assert_eq!(result, Some(response.to_string()));
    }

    #[test]
    fn test_cache_disabled() {
        let temp_dir = TempDir::new().unwrap();
        let mut cache = Cache::new(temp_dir.path()).unwrap();

        cache.disable();
        assert!(!cache.is_enabled());

        // Set should be no-op
        cache.set("test", "gpt-4", "genai", "response").unwrap();

        // Get should return None
        let result = cache.get("test", "gpt-4", "genai").unwrap();
        assert!(result.is_none());

        // Re-enable
        cache.enable();
        assert!(cache.is_enabled());
    }

    #[test]
    fn test_cache_stats() {
        let temp_dir = TempDir::new().unwrap();
        let cache = Cache::new(temp_dir.path()).unwrap();

        cache.set("test1", "gpt-4", "genai", "response1").unwrap();
        cache.set("test2", "gpt-4", "genai", "response2").unwrap();

        let stats = cache.stats().unwrap();
        assert_eq!(stats.total_entries, 2);
        assert!(stats.total_size_bytes > 0);
    }

    #[test]
    fn test_cache_clear_all() {
        let temp_dir = TempDir::new().unwrap();
        let cache = Cache::new(temp_dir.path()).unwrap();

        cache.set("test1", "gpt-4", "genai", "response1").unwrap();
        cache.set("test2", "gpt-4", "genai", "response2").unwrap();

        let stats = cache.stats().unwrap();
        assert_eq!(stats.total_entries, 2);

        let removed = cache.clear_all().unwrap();
        assert_eq!(removed, 2);

        let stats = cache.stats().unwrap();
        assert_eq!(stats.total_entries, 0);
    }
}
