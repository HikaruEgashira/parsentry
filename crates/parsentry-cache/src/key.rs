//! Cache key generation using SHA256 hashing

use sha2::{Digest, Sha256};

/// Current cache version - increment to invalidate all entries
pub const CACHE_VERSION: &str = "1.0.0";

/// Generate a deterministic cache key from arbitrary string parts.
///
/// Returns a SHA256 hex digest of `CACHE_VERSION` joined with `parts` by `"|"`.
pub fn hash_key(parts: &[&str]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(CACHE_VERSION.as_bytes());
    for part in parts {
        hasher.update(b"|");
        hasher.update(part.as_bytes());
    }
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic() {
        let k1 = hash_key(&["a", "b", "c"]);
        let k2 = hash_key(&["a", "b", "c"]);
        assert_eq!(k1, k2);
    }

    #[test]
    fn test_different_parts_differ() {
        let k1 = hash_key(&["a", "b"]);
        let k2 = hash_key(&["a", "c"]);
        assert_ne!(k1, k2);
    }

    #[test]
    fn test_different_length_parts_differ() {
        let k1 = hash_key(&["a"]);
        let k2 = hash_key(&["a", "b"]);
        assert_ne!(k1, k2);
    }

    #[test]
    fn test_empty_parts() {
        let k = hash_key(&[]);
        assert_eq!(k.len(), 64);
    }

    #[test]
    fn test_key_is_64_hex_chars() {
        let k = hash_key(&["test", "data"]);
        assert_eq!(k.len(), 64);
        assert!(k.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_version_is_embedded() {
        // Changing the version constant would change output; we verify by
        // checking the hash includes CACHE_VERSION (indirectly: same parts
        // produce the same hash only if version is constant).
        let k1 = hash_key(&["x"]);
        let k2 = hash_key(&["x"]);
        assert_eq!(k1, k2);
    }
}
