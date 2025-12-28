//! Cache cleanup and maintenance

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

use crate::entry::CacheEntry;
use crate::key::CACHE_VERSION;

/// Cleanup statistics
#[derive(Debug, Default, Clone)]
pub struct CleanupStats {
    /// Number of entries removed
    pub removed_count: usize,
    /// Bytes freed
    pub freed_bytes: u64,
}

/// Cleanup policy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupPolicy {
    /// Maximum cache size in MB
    pub max_cache_size_mb: usize,

    /// Maximum age in days before entry is considered stale
    pub max_age_days: usize,

    /// Maximum idle days (since last access) before entry is stale
    pub max_idle_days: usize,

    /// Remove entries with version mismatch
    pub remove_version_mismatch: bool,
}

impl Default for CleanupPolicy {
    fn default() -> Self {
        Self {
            max_cache_size_mb: 500,
            max_age_days: 90,
            max_idle_days: 30,
            remove_version_mismatch: true,
        }
    }
}

impl CleanupPolicy {
    /// Check if an entry is stale according to the policy
    pub fn is_stale(&self, entry: &CacheEntry, current_version: &str) -> bool {
        // Version mismatch
        if self.remove_version_mismatch && entry.version != current_version {
            return true;
        }

        // Age check
        if entry.age_days() > self.max_age_days as i64 {
            return true;
        }

        // Idle check
        if entry.idle_days() > self.max_idle_days as i64 {
            return true;
        }

        false
    }
}

/// Cleanup trigger configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CleanupTrigger {
    /// Periodic cleanup based on days since last cleanup
    Periodic { days: usize },

    /// Cleanup when size limit is exceeded
    OnSizeLimit { threshold_mb: usize },

    /// Combined triggers
    Combined {
        periodic_days: Option<usize>,
        size_limit_mb: Option<usize>,
    },

    /// Manual cleanup only
    Manual,
}

impl Default for CleanupTrigger {
    fn default() -> Self {
        Self::Combined {
            periodic_days: Some(7),
            size_limit_mb: Some(500),
        }
    }
}

/// State of the last cleanup operation
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CleanupState {
    last_cleanup_timestamp: DateTime<Utc>,
    last_cleanup_type: String,
}

impl Default for CleanupState {
    fn default() -> Self {
        Self {
            last_cleanup_timestamp: DateTime::from_timestamp(0, 0).unwrap(),
            last_cleanup_type: "none".to_string(),
        }
    }
}

/// Cache cleanup manager
pub struct CleanupManager {
    cache_dir: PathBuf,
    policy: CleanupPolicy,
    trigger: CleanupTrigger,
    state_file: PathBuf,
}

impl CleanupManager {
    /// Create a new cleanup manager
    pub fn new<P: AsRef<Path>>(cache_dir: P) -> Result<Self> {
        Self::with_config(cache_dir, CleanupPolicy::default(), CleanupTrigger::default())
    }

    /// Create a cleanup manager with custom configuration
    pub fn with_config<P: AsRef<Path>>(
        cache_dir: P,
        policy: CleanupPolicy,
        trigger: CleanupTrigger,
    ) -> Result<Self> {
        let cache_dir = cache_dir.as_ref().to_path_buf();
        let state_file = cache_dir.join("cleanup_state.json");

        Ok(Self {
            cache_dir,
            policy,
            trigger,
            state_file,
        })
    }

    /// Check if periodic cleanup should run (fast check)
    pub fn should_run_periodic_cleanup(&self) -> Result<bool> {
        match &self.trigger {
            CleanupTrigger::Manual => Ok(false),
            CleanupTrigger::Periodic { days } => {
                let state = self.load_state()?;
                let elapsed_days = (Utc::now() - state.last_cleanup_timestamp).num_days();
                Ok(elapsed_days >= *days as i64)
            }
            CleanupTrigger::Combined { periodic_days, .. } => {
                if let Some(days) = periodic_days {
                    let state = self.load_state()?;
                    let elapsed_days = (Utc::now() - state.last_cleanup_timestamp).num_days();
                    Ok(elapsed_days >= *days as i64)
                } else {
                    Ok(false)
                }
            }
            CleanupTrigger::OnSizeLimit { .. } => Ok(false),
        }
    }

    /// Check if cache is over size limit (heavier check)
    pub fn is_over_size_limit(&self) -> Result<bool> {
        let threshold_mb = match &self.trigger {
            CleanupTrigger::OnSizeLimit { threshold_mb } => *threshold_mb,
            CleanupTrigger::Combined { size_limit_mb, .. } => {
                if let Some(limit) = size_limit_mb {
                    *limit
                } else {
                    return Ok(false);
                }
            }
            _ => return Ok(false),
        };

        let total_size = self.calculate_total_size()?;
        let threshold_bytes = (threshold_mb * 1_048_576) as u64;

        Ok(total_size > threshold_bytes)
    }

    /// Calculate total cache size in bytes
    fn calculate_total_size(&self) -> Result<u64> {
        let mut total = 0u64;

        if !self.cache_dir.exists() {
            return Ok(0);
        }

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

    /// Clean up stale entries according to policy
    pub fn cleanup_stale_entries(&self) -> Result<CleanupStats> {
        let mut stats = CleanupStats::default();

        if !self.cache_dir.exists() {
            return Ok(stats);
        }

        for entry in walkdir::WalkDir::new(&self.cache_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();
            if !path.extension().map_or(false, |ext| ext == "json") {
                continue;
            }

            // Read and check cache entry
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(cache_entry) = serde_json::from_str::<CacheEntry>(&content) {
                    if self.policy.is_stale(&cache_entry, CACHE_VERSION) {
                        // Get file size before deleting
                        if let Ok(metadata) = fs::metadata(path) {
                            stats.freed_bytes += metadata.len();
                        }

                        // Delete the file
                        if fs::remove_file(path).is_ok() {
                            stats.removed_count += 1;
                            log::debug!("Removed stale cache entry: {}", path.display());
                        }
                    }
                }
            }
        }

        // Update state
        self.save_state(CleanupState {
            last_cleanup_timestamp: Utc::now(),
            last_cleanup_type: "stale".to_string(),
        })?;

        Ok(stats)
    }

    /// Clean up by size using LRU (Least Recently Used) strategy
    pub fn cleanup_by_size(&self) -> Result<CleanupStats> {
        let mut stats = CleanupStats::default();

        if !self.cache_dir.exists() {
            return Ok(stats);
        }

        // Collect all entries with their metadata
        let mut entries: Vec<(PathBuf, CacheEntry, u64)> = Vec::new();

        for entry in walkdir::WalkDir::new(&self.cache_dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if !entry.file_type().is_file() {
                continue;
            }

            let path = entry.path();
            if !path.extension().map_or(false, |ext| ext == "json") {
                continue;
            }

            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(cache_entry) = serde_json::from_str::<CacheEntry>(&content) {
                    if let Ok(metadata) = fs::metadata(path) {
                        entries.push((path.to_path_buf(), cache_entry, metadata.len()));
                    }
                }
            }
        }

        // Sort by last accessed (LRU)
        entries.sort_by_key(|(_, entry, _)| entry.metadata.last_accessed);

        // Calculate how much to remove
        let total_size = entries.iter().map(|(_, _, size)| size).sum::<u64>();
        let max_size = (self.policy.max_cache_size_mb * 1_048_576) as u64;

        if total_size <= max_size {
            return Ok(stats);
        }

        let mut target_removal = total_size - max_size;

        // Remove oldest entries until under limit
        for (path, _, size) in entries {
            if target_removal == 0 {
                break;
            }

            if fs::remove_file(&path).is_ok() {
                stats.removed_count += 1;
                stats.freed_bytes += size;
                target_removal = target_removal.saturating_sub(size);
                log::debug!("Removed LRU cache entry: {}", path.display());
            }
        }

        // Update state
        self.save_state(CleanupState {
            last_cleanup_timestamp: Utc::now(),
            last_cleanup_type: "size".to_string(),
        })?;

        Ok(stats)
    }

    /// Load cleanup state
    fn load_state(&self) -> Result<CleanupState> {
        if !self.state_file.exists() {
            return Ok(CleanupState::default());
        }

        let content = fs::read_to_string(&self.state_file)
            .context("Failed to read cleanup state file")?;

        let state: CleanupState = serde_json::from_str(&content)
            .context("Failed to parse cleanup state")?;

        Ok(state)
    }

    /// Save cleanup state
    fn save_state(&self, state: CleanupState) -> Result<()> {
        fs::create_dir_all(self.cache_dir.parent().unwrap_or(&self.cache_dir))?;

        let content = serde_json::to_string_pretty(&state)
            .context("Failed to serialize cleanup state")?;

        fs::write(&self.state_file, content)
            .context("Failed to write cleanup state file")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entry::CacheEntry;
    use tempfile::TempDir;

    #[test]
    fn test_cleanup_policy_stale_by_age() {
        let policy = CleanupPolicy {
            max_age_days: 90,
            max_idle_days: 30,
            remove_version_mismatch: true,
            max_cache_size_mb: 500,
        };

        let mut old_entry = CacheEntry::new(
            "1.0.0".to_string(),
            "genai".to_string(),
            "gpt-4".to_string(),
            "abc123".to_string(),
            "test".to_string(),
            10,
        );

        // Simulate old entry
        old_entry.metadata.created_at = Utc::now() - chrono::Duration::days(100);

        assert!(policy.is_stale(&old_entry, "1.0.0"));
    }

    #[test]
    fn test_cleanup_policy_stale_by_version() {
        let policy = CleanupPolicy::default();

        let entry = CacheEntry::new(
            "0.9.0".to_string(),
            "genai".to_string(),
            "gpt-4".to_string(),
            "abc123".to_string(),
            "test".to_string(),
            10,
        );

        assert!(policy.is_stale(&entry, "1.0.0"));
    }

    #[test]
    fn test_cleanup_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let manager = CleanupManager::new(temp_dir.path()).unwrap();

        // With default Combined trigger and no prior cleanup, should run
        assert!(manager.should_run_periodic_cleanup().unwrap());
    }

    #[test]
    fn test_cleanup_stale_entries() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");
        fs::create_dir_all(&cache_dir).unwrap();

        let manager = CleanupManager::new(&cache_dir).unwrap();

        // Create an old entry
        let mut old_entry = CacheEntry::new(
            "0.9.0".to_string(),
            "genai".to_string(),
            "gpt-4".to_string(),
            "abc123".to_string(),
            "test".to_string(),
            10,
        );
        old_entry.metadata.created_at = Utc::now() - chrono::Duration::days(100);

        // Save it
        let path = cache_dir.join("genai").join("gpt-4").join("ab");
        fs::create_dir_all(&path).unwrap();
        let file_path = path.join("abc123.json");
        fs::write(&file_path, serde_json::to_string(&old_entry).unwrap()).unwrap();

        // Clean up
        let stats = manager.cleanup_stale_entries().unwrap();

        assert_eq!(stats.removed_count, 1);
        assert!(stats.freed_bytes > 0);
        assert!(!file_path.exists());
    }
}
