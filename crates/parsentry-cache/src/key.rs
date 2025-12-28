//! Cache key generation using SHA256 hashing

use sha2::{Digest, Sha256};

/// Current cache version - increment when prompt templates change
pub const CACHE_VERSION: &str = "1.0.0";

/// Generates deterministic cache keys from prompts
pub struct CacheKeyGenerator {
    version: String,
}

impl CacheKeyGenerator {
    /// Create a new key generator with the current version
    pub fn new() -> Self {
        Self {
            version: CACHE_VERSION.to_string(),
        }
    }

    /// Create a key generator with a custom version
    pub fn with_version(version: String) -> Self {
        Self { version }
    }

    /// Generate a cache key from prompt, model, and provider
    ///
    /// The key is a SHA256 hash of:
    /// - Cache version (to invalidate on template changes)
    /// - Provider name (genai vs claude-code have different formats)
    /// - Model name (different models produce different outputs)
    /// - Full prompt text (the main determinant)
    pub fn generate_key(&self, prompt: &str, model: &str, provider: &str) -> String {
        let mut hasher = Sha256::new();

        // Include version to invalidate cache when templates change
        hasher.update(self.version.as_bytes());
        hasher.update(b"|");

        // Include provider (different providers have different response formats)
        hasher.update(provider.as_bytes());
        hasher.update(b"|");

        // Include model (different models produce different outputs)
        hasher.update(model.as_bytes());
        hasher.update(b"|");

        // Include full prompt (main determinant)
        hasher.update(prompt.as_bytes());

        let result = hasher.finalize();
        format!("{:x}", result)
    }

    /// Get the current version
    pub fn version(&self) -> &str {
        &self.version
    }
}

impl Default for CacheKeyGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_generation_deterministic() {
        let gen = CacheKeyGenerator::new();

        let key1 = gen.generate_key("test prompt", "gpt-4", "genai");
        let key2 = gen.generate_key("test prompt", "gpt-4", "genai");

        assert_eq!(key1, key2, "Same inputs should produce same key");
    }

    #[test]
    fn test_key_generation_different_prompts() {
        let gen = CacheKeyGenerator::new();

        let key1 = gen.generate_key("prompt1", "gpt-4", "genai");
        let key2 = gen.generate_key("prompt2", "gpt-4", "genai");

        assert_ne!(key1, key2, "Different prompts should produce different keys");
    }

    #[test]
    fn test_key_generation_different_models() {
        let gen = CacheKeyGenerator::new();

        let key1 = gen.generate_key("test", "gpt-4", "genai");
        let key2 = gen.generate_key("test", "gpt-3.5-turbo", "genai");

        assert_ne!(key1, key2, "Different models should produce different keys");
    }

    #[test]
    fn test_key_generation_different_providers() {
        let gen = CacheKeyGenerator::new();

        let key1 = gen.generate_key("test", "gpt-4", "genai");
        let key2 = gen.generate_key("test", "gpt-4", "claude-code");

        assert_ne!(
            key1, key2,
            "Different providers should produce different keys"
        );
    }

    #[test]
    fn test_key_generation_different_versions() {
        let gen1 = CacheKeyGenerator::with_version("1.0.0".to_string());
        let gen2 = CacheKeyGenerator::with_version("2.0.0".to_string());

        let key1 = gen1.generate_key("test", "gpt-4", "genai");
        let key2 = gen2.generate_key("test", "gpt-4", "genai");

        assert_ne!(
            key1, key2,
            "Different versions should produce different keys"
        );
    }

    #[test]
    fn test_key_is_64_chars() {
        let gen = CacheKeyGenerator::new();
        let key = gen.generate_key("test", "gpt-4", "genai");

        assert_eq!(key.len(), 64, "SHA256 hash should be 64 hex characters");
    }
}
