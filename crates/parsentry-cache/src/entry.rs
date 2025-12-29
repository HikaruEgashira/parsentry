//! Cache entry structure and metadata

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A cache entry containing LLM response and metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    /// Cache format version
    pub version: String,

    /// Agent name (genai, claude-code)
    pub agent: String,

    /// Model name
    pub model: String,

    /// SHA256 hash of the prompt
    pub prompt_hash: String,

    /// LLM response (JSON string or raw text)
    pub response: String,

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

    /// Cost in USD (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_usd: Option<f64>,

    /// Response time in milliseconds (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,

    /// Size of the prompt in bytes
    pub prompt_size: usize,

    /// Size of the response in bytes
    pub response_size: usize,
}

impl CacheEntry {
    /// Create a new cache entry
    pub fn new(
        version: String,
        agent: String,
        model: String,
        prompt_hash: String,
        response: String,
        prompt_size: usize,
    ) -> Self {
        let now = Utc::now();
        let response_size = response.len();

        Self {
            version,
            agent,
            model,
            prompt_hash,
            response,
            metadata: CacheMetadata {
                created_at: now,
                last_accessed: now,
                access_count: 0,
                cost_usd: None,
                duration_ms: None,
                prompt_size,
                response_size,
            },
        }
    }

    /// Update access metadata
    pub fn record_access(&mut self) {
        self.metadata.last_accessed = Utc::now();
        self.metadata.access_count += 1;
    }

    /// Set cost metadata
    pub fn set_cost(&mut self, cost_usd: f64) {
        self.metadata.cost_usd = Some(cost_usd);
    }

    /// Set duration metadata
    pub fn set_duration(&mut self, duration_ms: u64) {
        self.metadata.duration_ms = Some(duration_ms);
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
            "genai".to_string(),
            "gpt-4".to_string(),
            "abc123".to_string(),
            "test response".to_string(),
            100,
        );

        assert_eq!(entry.version, "1.0.0");
        assert_eq!(entry.agent, "genai");
        assert_eq!(entry.model, "gpt-4");
        assert_eq!(entry.prompt_hash, "abc123");
        assert_eq!(entry.response, "test response");
        assert_eq!(entry.metadata.access_count, 0);
        assert_eq!(entry.metadata.prompt_size, 100);
        assert_eq!(entry.metadata.response_size, 13);
    }

    #[test]
    fn test_record_access() {
        let mut entry = CacheEntry::new(
            "1.0.0".to_string(),
            "genai".to_string(),
            "gpt-4".to_string(),
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
}
