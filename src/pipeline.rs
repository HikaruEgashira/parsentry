//! Pipeline orchestration with cache integration.

use crate::config::ParsentryConfig;
use parsentry_cache::Cache;

/// Orchestrates the analysis pipeline with cache integration.
///
/// For each attack surface prompt:
///   1. Check cache (namespace="analysis", key=cache_key)
///   2. If cached: return cached result immediately
///   3. If not cached: caller executes the agent
///   4. After execution: caller stores result via `cache_set`
pub struct Pipeline {
    cache: Option<Cache>,
    namespace: String,
}

/// Result of a single surface analysis.
pub struct PipelineResult {
    pub surface_id: String,
    pub response: String,
    pub cached: bool,
}

impl Pipeline {
    /// Build a pipeline from config. If caching is disabled, operations become no-ops.
    pub fn new(config: &ParsentryConfig) -> anyhow::Result<Self> {
        let cache = if config.cache.enabled {
            Some(Cache::with_cleanup_config(
                &config.cache.directory,
                config.cache.to_cleanup_policy(),
                config.cache.to_cleanup_trigger(),
            )?)
        } else {
            None
        };
        Ok(Self {
            cache,
            namespace: "analysis".to_string(),
        })
    }

    /// Look up a cached analysis result. Returns `None` on miss or when caching is disabled.
    pub fn cache_get(&self, cache_key: &str) -> Option<String> {
        self.cache
            .as_ref()
            .and_then(|c| c.get(&self.namespace, cache_key).ok().flatten())
    }

    /// Store an analysis result in cache. No-op when caching is disabled.
    pub fn cache_set(&self, cache_key: &str, response: &str, input_size: usize) {
        if let Some(ref cache) = self.cache {
            let _ = cache.set(&self.namespace, cache_key, response, input_size);
        }
    }

    /// Run periodic and size-based cleanup if the configured triggers are met.
    pub fn maybe_cleanup(&self) {
        if let Some(ref cache) = self.cache {
            if cache.should_cleanup_periodic().unwrap_or(false) {
                let _ = cache.cleanup_stale();
            }
            if cache.should_cleanup_size().unwrap_or(false) {
                let _ = cache.cleanup_by_size();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ParsentryConfig;
    use tempfile::TempDir;

    fn config_with_dir(dir: &std::path::Path, enabled: bool) -> ParsentryConfig {
        let mut config = ParsentryConfig::default();
        config.cache.enabled = enabled;
        config.cache.directory = dir.to_path_buf();
        config
    }

    #[test]
    fn test_cache_hit_and_miss() {
        let tmp = TempDir::new().unwrap();
        let config = config_with_dir(tmp.path(), true);
        let pipeline = Pipeline::new(&config).unwrap();

        assert!(pipeline.cache_get("key_abc123").is_none());
        pipeline.cache_set("key_abc123", "result data", 11);
        assert_eq!(
            pipeline.cache_get("key_abc123"),
            Some("result data".to_string())
        );
    }

    #[test]
    fn test_cache_disabled_is_noop() {
        let tmp = TempDir::new().unwrap();
        let config = config_with_dir(tmp.path(), false);
        let pipeline = Pipeline::new(&config).unwrap();

        pipeline.cache_set("key_abc123", "result data", 11);
        assert!(pipeline.cache_get("key_abc123").is_none());
    }

    #[test]
    fn test_maybe_cleanup_does_not_panic() {
        let tmp = TempDir::new().unwrap();
        let config = config_with_dir(tmp.path(), true);
        let pipeline = Pipeline::new(&config).unwrap();
        pipeline.maybe_cleanup();
    }
}
