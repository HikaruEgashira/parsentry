//! Generic cache entry and metadata

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A content-addressable cache entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// Cache format version
    pub version: String,

    /// SHA256 key identifying this entry
    pub key: String,

    /// User-defined grouping (replaces agent+model with a single dimension)
    pub namespace: String,

    /// Cached value (JSON string or raw text)
    pub value: String,

    /// Metadata for cache management
    pub metadata: CacheMetadata,
}

/// Metadata for cache entry management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheMetadata {
    /// When this entry was created
    pub created_at: DateTime<Utc>,

    /// Last time this entry was accessed
    pub last_accessed: DateTime<Utc>,

    /// Number of times this entry has been accessed
    pub access_count: usize,

    /// Size of the input that produced this entry, in bytes
    pub input_size: usize,

    /// Size of the cached value in bytes
    pub value_size: usize,
}

impl CacheEntry {
    /// Create a new cache entry
    pub fn new(
        version: String,
        namespace: String,
        key: String,
        value: String,
        input_size: usize,
    ) -> Self {
        let now = Utc::now();
        let value_size = value.len();

        Self {
            version,
            key,
            namespace,
            value,
            metadata: CacheMetadata {
                created_at: now,
                last_accessed: now,
                access_count: 0,
                input_size,
                value_size,
            },
        }
    }

    /// Update access metadata
    pub fn record_access(&mut self) {
        self.metadata.last_accessed = Utc::now();
        self.metadata.access_count += 1;
    }

    /// Get age in days since creation
    pub fn age_days(&self) -> i64 {
        let now = Utc::now();
        (now - self.metadata.created_at).num_days()
    }

    /// Get idle days since last access
    pub fn idle_days(&self) -> i64 {
        let now = Utc::now();
        (now - self.metadata.last_accessed).num_days()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_entry_creation() {
        let entry = CacheEntry::new(
            "1.0.0".to_string(),
            "my-ns".to_string(),
            "abc123".to_string(),
            "test value".to_string(),
            100,
        );

        assert_eq!(entry.version, "1.0.0");
        assert_eq!(entry.namespace, "my-ns");
        assert_eq!(entry.key, "abc123");
        assert_eq!(entry.value, "test value");
        assert_eq!(entry.metadata.access_count, 0);
        assert_eq!(entry.metadata.input_size, 100);
        assert_eq!(entry.metadata.value_size, 10);
    }

    #[test]
    fn test_record_access() {
        let mut entry = CacheEntry::new(
            "1.0.0".to_string(),
            "ns".to_string(),
            "abc123".to_string(),
            "test".to_string(),
            100,
        );

        assert_eq!(entry.metadata.access_count, 0);

        entry.record_access();
        assert_eq!(entry.metadata.access_count, 1);

        entry.record_access();
        assert_eq!(entry.metadata.access_count, 2);
    }

    #[test]
    fn test_idle_days_returns_actual_value() {
        let mut entry = CacheEntry::new(
            "1.0.0".to_string(),
            "ns".to_string(),
            "abc123".to_string(),
            "test".to_string(),
            10,
        );

        assert_eq!(entry.idle_days(), 0);

        entry.metadata.last_accessed = Utc::now() - chrono::Duration::days(5);
        assert_eq!(entry.idle_days(), 5);

        entry.metadata.last_accessed = Utc::now() - chrono::Duration::days(100);
        assert_eq!(entry.idle_days(), 100);
    }

    #[test]
    fn test_age_days_returns_actual_value() {
        let mut entry = CacheEntry::new(
            "1.0.0".to_string(),
            "ns".to_string(),
            "abc123".to_string(),
            "test".to_string(),
            10,
        );

        assert_eq!(entry.age_days(), 0);

        entry.metadata.created_at = Utc::now() - chrono::Duration::days(10);
        assert_eq!(entry.age_days(), 10);
    }
}
