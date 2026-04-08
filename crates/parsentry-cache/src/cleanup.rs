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
        Self::with_config(
            cache_dir,
            CleanupPolicy::default(),
            CleanupTrigger::default(),
        )
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

        let content =
            fs::read_to_string(&self.state_file).context("Failed to read cleanup state file")?;

        let state: CleanupState =
            serde_json::from_str(&content).context("Failed to parse cleanup state")?;

        Ok(state)
    }

    /// Save cleanup state
    fn save_state(&self, state: CleanupState) -> Result<()> {
        fs::create_dir_all(self.cache_dir.parent().unwrap_or(&self.cache_dir))?;

        let content =
            serde_json::to_string_pretty(&state).context("Failed to serialize cleanup state")?;

        fs::write(&self.state_file, content).context("Failed to write cleanup state file")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entry::CacheEntry;
    use tempfile::TempDir;

    fn make_entry(version: &str, ns: &str, key: &str, value: &str) -> CacheEntry {
        CacheEntry::new(
            version.to_string(),
            ns.to_string(),
            key.to_string(),
            value.to_string(),
            10,
        )
    }

    #[test]
    fn test_cleanup_policy_stale_by_age() {
        let policy = CleanupPolicy {
            max_age_days: 90,
            max_idle_days: 30,
            remove_version_mismatch: true,
            max_cache_size_mb: 500,
        };

        let mut entry = make_entry("1.0.0", "ns", "abc123", "test");
        entry.metadata.created_at = Utc::now() - chrono::Duration::days(100);

        assert!(policy.is_stale(&entry, "1.0.0"));
    }

    #[test]
    fn test_cleanup_policy_stale_by_version() {
        let policy = CleanupPolicy::default();
        let entry = make_entry("0.9.0", "ns", "abc123", "test");
        assert!(policy.is_stale(&entry, "1.0.0"));
    }

    #[test]
    fn test_cleanup_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let manager = CleanupManager::new(temp_dir.path()).unwrap();
        assert!(manager.should_run_periodic_cleanup().unwrap());
    }

    #[test]
    fn test_cleanup_stale_entries() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path().join("cache");
        fs::create_dir_all(&cache_dir).unwrap();

        let manager = CleanupManager::new(&cache_dir).unwrap();

        let mut old_entry = make_entry("0.9.0", "ns", "abc123", "test");
        old_entry.metadata.created_at = Utc::now() - chrono::Duration::days(100);

        let path = cache_dir.join("ns").join("ab");
        fs::create_dir_all(&path).unwrap();
        let file_path = path.join("abc123.json");
        fs::write(&file_path, serde_json::to_string(&old_entry).unwrap()).unwrap();

        let stats = manager.cleanup_stale_entries().unwrap();

        assert_eq!(stats.removed_count, 1);
        assert!(stats.freed_bytes > 0);
        assert!(!file_path.exists());
    }

    // --- is_stale boundary tests ---

    #[test]
    fn test_is_stale_not_stale_when_fresh() {
        let policy = CleanupPolicy {
            max_age_days: 90,
            max_idle_days: 30,
            remove_version_mismatch: true,
            max_cache_size_mb: 500,
        };

        let entry = make_entry("1.0.0", "ns", "abc123", "test");
        assert!(!policy.is_stale(&entry, "1.0.0"));
    }

    #[test]
    fn test_is_stale_version_mismatch_disabled() {
        let policy = CleanupPolicy {
            max_age_days: 90,
            max_idle_days: 30,
            remove_version_mismatch: false,
            max_cache_size_mb: 500,
        };

        let entry = make_entry("0.9.0", "ns", "abc123", "test");
        assert!(!policy.is_stale(&entry, "1.0.0"));
    }

    #[test]
    fn test_is_stale_age_boundary_equal() {
        let policy = CleanupPolicy {
            max_age_days: 90,
            max_idle_days: 30,
            remove_version_mismatch: false,
            max_cache_size_mb: 500,
        };

        let mut entry = make_entry("1.0.0", "ns", "abc123", "test");

        // age_days == max_age_days (boundary: > not >=, so NOT stale)
        entry.metadata.created_at = Utc::now() - chrono::Duration::days(90);
        entry.metadata.last_accessed = Utc::now();
        assert!(!policy.is_stale(&entry, "1.0.0"));

        // age_days == max_age_days + 1 (stale)
        entry.metadata.created_at = Utc::now() - chrono::Duration::days(91);
        assert!(policy.is_stale(&entry, "1.0.0"));
    }

    #[test]
    fn test_is_stale_idle_boundary_equal() {
        let policy = CleanupPolicy {
            max_age_days: 90,
            max_idle_days: 30,
            remove_version_mismatch: false,
            max_cache_size_mb: 500,
        };

        let mut entry = make_entry("1.0.0", "ns", "abc123", "test");

        entry.metadata.last_accessed = Utc::now() - chrono::Duration::days(30);
        assert!(!policy.is_stale(&entry, "1.0.0"));

        entry.metadata.last_accessed = Utc::now() - chrono::Duration::days(31);
        assert!(policy.is_stale(&entry, "1.0.0"));
    }

    #[test]
    fn test_is_stale_both_conditions_and() {
        let policy = CleanupPolicy {
            max_age_days: 90,
            max_idle_days: 30,
            remove_version_mismatch: true,
            max_cache_size_mb: 500,
        };

        let mut entry = make_entry("1.0.0", "ns", "abc123", "test");
        entry.metadata.created_at = Utc::now() - chrono::Duration::days(10);
        entry.metadata.last_accessed = Utc::now() - chrono::Duration::days(5);
        assert!(!policy.is_stale(&entry, "1.0.0"));
    }

    // --- should_run_periodic_cleanup tests ---

    #[test]
    fn test_should_run_periodic_cleanup_manual_trigger() {
        let temp_dir = TempDir::new().unwrap();
        let manager = CleanupManager::with_config(
            temp_dir.path(),
            CleanupPolicy::default(),
            CleanupTrigger::Manual,
        )
        .unwrap();

        assert!(!manager.should_run_periodic_cleanup().unwrap());
    }

    #[test]
    fn test_should_run_periodic_cleanup_on_size_limit_trigger() {
        let temp_dir = TempDir::new().unwrap();
        let manager = CleanupManager::with_config(
            temp_dir.path(),
            CleanupPolicy::default(),
            CleanupTrigger::OnSizeLimit { threshold_mb: 100 },
        )
        .unwrap();

        assert!(!manager.should_run_periodic_cleanup().unwrap());
    }

    #[test]
    fn test_should_run_periodic_cleanup_periodic_trigger() {
        let temp_dir = TempDir::new().unwrap();
        let manager = CleanupManager::with_config(
            temp_dir.path(),
            CleanupPolicy::default(),
            CleanupTrigger::Periodic { days: 7 },
        )
        .unwrap();

        assert!(manager.should_run_periodic_cleanup().unwrap());
    }

    #[test]
    fn test_should_run_periodic_cleanup_boundary() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path();

        let manager = CleanupManager::with_config(
            cache_dir,
            CleanupPolicy::default(),
            CleanupTrigger::Periodic { days: 7 },
        )
        .unwrap();

        let state = CleanupState {
            last_cleanup_timestamp: Utc::now() - chrono::Duration::days(7),
            last_cleanup_type: "test".to_string(),
        };
        manager.save_state(state).unwrap();
        assert!(manager.should_run_periodic_cleanup().unwrap());

        let state = CleanupState {
            last_cleanup_timestamp: Utc::now() - chrono::Duration::days(6),
            last_cleanup_type: "test".to_string(),
        };
        manager.save_state(state).unwrap();
        assert!(!manager.should_run_periodic_cleanup().unwrap());
    }

    #[test]
    fn test_should_run_periodic_cleanup_combined_no_periodic() {
        let temp_dir = TempDir::new().unwrap();
        let manager = CleanupManager::with_config(
            temp_dir.path(),
            CleanupPolicy::default(),
            CleanupTrigger::Combined {
                periodic_days: None,
                size_limit_mb: Some(500),
            },
        )
        .unwrap();

        assert!(!manager.should_run_periodic_cleanup().unwrap());
    }

    // --- is_over_size_limit tests ---

    #[test]
    fn test_is_over_size_limit_manual_trigger() {
        let temp_dir = TempDir::new().unwrap();
        let manager = CleanupManager::with_config(
            temp_dir.path(),
            CleanupPolicy::default(),
            CleanupTrigger::Manual,
        )
        .unwrap();

        assert!(!manager.is_over_size_limit().unwrap());
    }

    #[test]
    fn test_is_over_size_limit_periodic_trigger() {
        let temp_dir = TempDir::new().unwrap();
        let manager = CleanupManager::with_config(
            temp_dir.path(),
            CleanupPolicy::default(),
            CleanupTrigger::Periodic { days: 7 },
        )
        .unwrap();

        assert!(!manager.is_over_size_limit().unwrap());
    }

    #[test]
    fn test_is_over_size_limit_on_size_limit_under() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path();

        let manager = CleanupManager::with_config(
            cache_dir,
            CleanupPolicy::default(),
            CleanupTrigger::OnSizeLimit { threshold_mb: 1 },
        )
        .unwrap();

        assert!(!manager.is_over_size_limit().unwrap());
    }

    #[test]
    fn test_is_over_size_limit_on_size_limit_over() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path();

        let big_data = vec![b'x'; 1_048_577];
        fs::write(cache_dir.join("bigfile.bin"), &big_data).unwrap();

        let manager = CleanupManager::with_config(
            cache_dir,
            CleanupPolicy::default(),
            CleanupTrigger::OnSizeLimit { threshold_mb: 1 },
        )
        .unwrap();

        assert!(manager.is_over_size_limit().unwrap());
    }

    #[test]
    fn test_is_over_size_limit_combined_with_size() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path();

        let big_data = vec![b'x'; 1_100_000];
        fs::write(cache_dir.join("bigfile.bin"), &big_data).unwrap();

        let manager = CleanupManager::with_config(
            cache_dir,
            CleanupPolicy::default(),
            CleanupTrigger::Combined {
                periodic_days: Some(7),
                size_limit_mb: Some(1),
            },
        )
        .unwrap();

        assert!(manager.is_over_size_limit().unwrap());
    }

    #[test]
    fn test_is_over_size_limit_combined_no_size_limit() {
        let temp_dir = TempDir::new().unwrap();
        let manager = CleanupManager::with_config(
            temp_dir.path(),
            CleanupPolicy::default(),
            CleanupTrigger::Combined {
                periodic_days: Some(7),
                size_limit_mb: None,
            },
        )
        .unwrap();

        assert!(!manager.is_over_size_limit().unwrap());
    }

    // --- calculate_total_size tests ---

    #[test]
    fn test_calculate_total_size_empty() {
        let temp_dir = TempDir::new().unwrap();
        let manager = CleanupManager::new(temp_dir.path()).unwrap();
        assert_eq!(manager.calculate_total_size().unwrap(), 0);
    }

    #[test]
    fn test_calculate_total_size_nonexistent_dir() {
        let temp_dir = TempDir::new().unwrap();
        let nonexistent = temp_dir.path().join("does_not_exist");
        let manager = CleanupManager::with_config(
            &nonexistent,
            CleanupPolicy::default(),
            CleanupTrigger::Manual,
        )
        .unwrap();

        assert_eq!(manager.calculate_total_size().unwrap(), 0);
    }

    #[test]
    fn test_calculate_total_size_with_files() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path();

        fs::write(cache_dir.join("file1.txt"), "hello").unwrap();
        fs::write(cache_dir.join("file2.txt"), "world!").unwrap();

        let manager = CleanupManager::new(cache_dir).unwrap();
        let size = manager.calculate_total_size().unwrap();
        assert!(size >= 11, "Expected at least 11 bytes, got {}", size);
    }

    // --- cleanup_by_size LRU tests ---

    #[test]
    fn test_cleanup_by_size_under_limit() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path();

        let policy = CleanupPolicy {
            max_cache_size_mb: 500,
            max_age_days: 90,
            max_idle_days: 30,
            remove_version_mismatch: true,
        };

        let manager =
            CleanupManager::with_config(cache_dir, policy, CleanupTrigger::Manual).unwrap();

        let entry = make_entry("1.0.0", "ns", "abc123", "small value");
        let dir = cache_dir.join("ns").join("ab");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("abc123.json"),
            serde_json::to_string(&entry).unwrap(),
        )
        .unwrap();

        let stats = manager.cleanup_by_size().unwrap();
        assert_eq!(stats.removed_count, 0);
        assert_eq!(stats.freed_bytes, 0);
    }

    #[test]
    fn test_cleanup_by_size_lru_ordering() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path();

        let policy = CleanupPolicy {
            max_cache_size_mb: 0,
            max_age_days: 90,
            max_idle_days: 30,
            remove_version_mismatch: true,
        };

        let manager =
            CleanupManager::with_config(cache_dir, policy, CleanupTrigger::Manual).unwrap();

        let mut old_entry = make_entry("1.0.0", "ns", "old111", "old value");
        old_entry.metadata.last_accessed = Utc::now() - chrono::Duration::days(10);

        let new_entry = make_entry("1.0.0", "ns", "new222", "new value");

        let dir_old = cache_dir.join("ns").join("ol");
        fs::create_dir_all(&dir_old).unwrap();
        fs::write(
            dir_old.join("old111.json"),
            serde_json::to_string(&old_entry).unwrap(),
        )
        .unwrap();

        let dir_new = cache_dir.join("ns").join("ne");
        fs::create_dir_all(&dir_new).unwrap();
        fs::write(
            dir_new.join("new222.json"),
            serde_json::to_string(&new_entry).unwrap(),
        )
        .unwrap();

        let stats = manager.cleanup_by_size().unwrap();
        assert!(stats.removed_count >= 1);
        assert!(stats.freed_bytes > 0);
    }

    #[test]
    fn test_cleanup_by_size_removes_oldest_first() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path();

        let policy = CleanupPolicy {
            max_cache_size_mb: 0,
            max_age_days: 90,
            max_idle_days: 30,
            remove_version_mismatch: true,
        };

        let manager =
            CleanupManager::with_config(cache_dir, policy, CleanupTrigger::Manual).unwrap();

        let mut entry_oldest = make_entry("1.0.0", "ns", "aaa111", "oldest");
        entry_oldest.metadata.last_accessed = Utc::now() - chrono::Duration::days(100);

        let dir = cache_dir.join("ns").join("aa");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("aaa111.json"),
            serde_json::to_string(&entry_oldest).unwrap(),
        )
        .unwrap();

        let stats = manager.cleanup_by_size().unwrap();
        assert_eq!(stats.removed_count, 1);
        assert!(!dir.join("aaa111.json").exists());
    }

    // --- load_state / save_state tests ---

    #[test]
    fn test_load_state_returns_default_when_no_file() {
        let temp_dir = TempDir::new().unwrap();
        let manager = CleanupManager::new(temp_dir.path()).unwrap();

        let state = manager.load_state().unwrap();
        assert_eq!(state.last_cleanup_type, "none");
        assert_eq!(
            state.last_cleanup_timestamp,
            DateTime::from_timestamp(0, 0).unwrap()
        );
    }

    #[test]
    fn test_save_state_then_load_state() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path();
        fs::create_dir_all(cache_dir).unwrap();

        let manager = CleanupManager::new(cache_dir).unwrap();

        let now = Utc::now();
        let state = CleanupState {
            last_cleanup_timestamp: now,
            last_cleanup_type: "stale".to_string(),
        };

        manager.save_state(state.clone()).unwrap();

        let loaded = manager.load_state().unwrap();
        assert_eq!(loaded.last_cleanup_type, "stale");
        assert_eq!(loaded.last_cleanup_timestamp.timestamp(), now.timestamp());
    }

    #[test]
    fn test_save_state_actually_writes_file() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path();
        fs::create_dir_all(cache_dir).unwrap();

        let manager = CleanupManager::new(cache_dir).unwrap();

        let state = CleanupState {
            last_cleanup_timestamp: Utc::now(),
            last_cleanup_type: "size".to_string(),
        };

        manager.save_state(state).unwrap();

        let state_path = cache_dir.join("cleanup_state.json");
        assert!(state_path.exists());

        let content = fs::read_to_string(&state_path).unwrap();
        assert!(content.contains("size"));
    }

    // --- cleanup_stale_entries preserves fresh entries ---

    #[test]
    fn test_cleanup_stale_entries_preserves_fresh() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path();
        fs::create_dir_all(cache_dir).unwrap();

        let policy = CleanupPolicy {
            max_age_days: 90,
            max_idle_days: 30,
            remove_version_mismatch: true,
            max_cache_size_mb: 500,
        };

        let manager =
            CleanupManager::with_config(cache_dir, policy, CleanupTrigger::Manual).unwrap();

        let entry = make_entry(CACHE_VERSION, "ns", "abc123", "fresh");

        let dir = cache_dir.join("ns").join("ab");
        fs::create_dir_all(&dir).unwrap();
        let file_path = dir.join("abc123.json");
        fs::write(&file_path, serde_json::to_string(&entry).unwrap()).unwrap();

        let stats = manager.cleanup_stale_entries().unwrap();
        assert_eq!(stats.removed_count, 0);
        assert_eq!(stats.freed_bytes, 0);
        assert!(file_path.exists());
    }

    #[test]
    fn test_cleanup_stale_entries_by_idle() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path();
        fs::create_dir_all(cache_dir).unwrap();

        let policy = CleanupPolicy {
            max_age_days: 90,
            max_idle_days: 30,
            remove_version_mismatch: false,
            max_cache_size_mb: 500,
        };

        let manager =
            CleanupManager::with_config(cache_dir, policy, CleanupTrigger::Manual).unwrap();

        let mut entry = make_entry(CACHE_VERSION, "ns", "abc123", "idle");
        entry.metadata.last_accessed = Utc::now() - chrono::Duration::days(31);

        let dir = cache_dir.join("ns").join("ab");
        fs::create_dir_all(&dir).unwrap();
        let file_path = dir.join("abc123.json");
        fs::write(&file_path, serde_json::to_string(&entry).unwrap()).unwrap();

        let stats = manager.cleanup_stale_entries().unwrap();
        assert_eq!(stats.removed_count, 1);
        assert!(!file_path.exists());
    }

    #[test]
    fn test_cleanup_by_size_nonexistent_dir() {
        let temp_dir = TempDir::new().unwrap();
        let nonexistent = temp_dir.path().join("does_not_exist");

        let manager = CleanupManager::with_config(
            &nonexistent,
            CleanupPolicy::default(),
            CleanupTrigger::Manual,
        )
        .unwrap();

        let stats = manager.cleanup_by_size().unwrap();
        assert_eq!(stats.removed_count, 0);
        assert_eq!(stats.freed_bytes, 0);
    }

    #[test]
    fn test_cleanup_stale_entries_nonexistent_dir() {
        let temp_dir = TempDir::new().unwrap();
        let nonexistent = temp_dir.path().join("does_not_exist");

        let manager = CleanupManager::with_config(
            &nonexistent,
            CleanupPolicy::default(),
            CleanupTrigger::Manual,
        )
        .unwrap();

        let stats = manager.cleanup_stale_entries().unwrap();
        assert_eq!(stats.removed_count, 0);
        assert_eq!(stats.freed_bytes, 0);
    }

    #[test]
    fn test_is_over_size_limit_threshold_arithmetic() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path();

        fs::write(cache_dir.join("small.bin"), &[b'x'; 100]).unwrap();

        let manager = CleanupManager::with_config(
            cache_dir,
            CleanupPolicy::default(),
            CleanupTrigger::OnSizeLimit { threshold_mb: 1 },
        )
        .unwrap();

        assert!(!manager.is_over_size_limit().unwrap());
    }

    #[test]
    fn test_is_over_size_limit_exact_boundary() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path();

        let data = vec![b'x'; 1_048_576];
        fs::write(cache_dir.join("exact.bin"), &data).unwrap();

        let manager = CleanupManager::with_config(
            cache_dir,
            CleanupPolicy::default(),
            CleanupTrigger::OnSizeLimit { threshold_mb: 1 },
        )
        .unwrap();

        assert!(!manager.is_over_size_limit().unwrap());
    }

    #[test]
    fn test_cleanup_by_size_partial_removal() {
        // Kills - → + in target_removal = total_size - max_size
        // Use non-zero max_cache_size_mb so subtraction matters
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path();

        let policy = CleanupPolicy {
            max_cache_size_mb: 1, // 1 MB limit
            max_age_days: 90,
            max_idle_days: 30,
            remove_version_mismatch: true,
        };

        let manager =
            CleanupManager::with_config(cache_dir, policy, CleanupTrigger::Manual).unwrap();

        // Create entries totaling >1MB so cleanup is triggered
        let big_val = "x".repeat(600_000);
        let mut old_entry = make_entry("1.0.0", "ns", "old111", &big_val);
        old_entry.metadata.last_accessed = Utc::now() - chrono::Duration::days(10);

        let big_val2 = "y".repeat(600_000);
        let new_entry = make_entry("1.0.0", "ns", "new222", &big_val2);

        let dir = cache_dir.join("ns").join("ol");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("old111.json"), serde_json::to_string(&old_entry).unwrap()).unwrap();

        let dir2 = cache_dir.join("ns").join("ne");
        fs::create_dir_all(&dir2).unwrap();
        fs::write(dir2.join("new222.json"), serde_json::to_string(&new_entry).unwrap()).unwrap();

        let stats = manager.cleanup_by_size().unwrap();
        // With - : target_removal > 0, so oldest entry removed
        // With + : target_removal huge, both removed
        assert_eq!(stats.removed_count, 1, "should remove only the oldest to get under limit");
    }

    #[test]
    fn test_cleanup_by_size_target_removal_subtraction() {
        let temp_dir = TempDir::new().unwrap();
        let cache_dir = temp_dir.path();

        let policy = CleanupPolicy {
            max_cache_size_mb: 0,
            max_age_days: 90,
            max_idle_days: 30,
            remove_version_mismatch: true,
        };

        let manager =
            CleanupManager::with_config(cache_dir, policy, CleanupTrigger::Manual).unwrap();

        let mut entry1 = make_entry("1.0.0", "ns", "hash11", "resp1");
        entry1.metadata.last_accessed = Utc::now() - chrono::Duration::days(10);

        let mut entry2 = make_entry("1.0.0", "ns", "hash22", "resp2");
        entry2.metadata.last_accessed = Utc::now() - chrono::Duration::days(5);

        let dir1 = cache_dir.join("ns").join("ha");
        fs::create_dir_all(&dir1).unwrap();
        fs::write(
            dir1.join("hash11.json"),
            serde_json::to_string(&entry1).unwrap(),
        )
        .unwrap();
        fs::write(
            dir1.join("hash22.json"),
            serde_json::to_string(&entry2).unwrap(),
        )
        .unwrap();

        let stats = manager.cleanup_by_size().unwrap();
        assert_eq!(stats.removed_count, 2);
        assert!(stats.freed_bytes > 0);
    }
}
