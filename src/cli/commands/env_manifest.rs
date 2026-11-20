//! Environment manifest (`scan --env <PATH>`) schema and validation.

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// File name of the validated manifest inside the cache directory.
pub const ENV_MANIFEST_FILE: &str = "env.yaml";

/// Describes how to launch the target application for dynamic checks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvManifest {
    pub base_url: String,
    #[serde(default)]
    pub compose_file: Option<String>,
    #[serde(default)]
    pub docker_image: Option<String>,
    #[serde(default)]
    pub command: Option<String>,
    /// Names of environment variables holding credentials (never values).
    #[serde(default)]
    pub auth_env: Vec<String>,
    #[serde(default)]
    pub read_only_mounts: Vec<String>,
}

impl EnvManifest {
    pub fn parse(yaml: &str) -> Result<Self> {
        let manifest: Self = serde_yaml::from_str(yaml).context("invalid env manifest")?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn validate(&self) -> Result<()> {
        if self.base_url.trim().is_empty() {
            bail!("env manifest: base_url must not be empty");
        }
        let launchers = [&self.compose_file, &self.docker_image, &self.command]
            .iter()
            .filter(|l| l.is_some())
            .count();
        if launchers != 1 {
            bail!(
                "env manifest: exactly one of compose_file, docker_image, command is required (found {launchers})"
            );
        }
        for name in &self.auth_env {
            let valid =
                !name.is_empty() && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
            if !valid {
                bail!("env manifest: invalid auth_env name {name:?} (expected [A-Za-z0-9_]+)");
            }
        }
        Ok(())
    }
}

/// Validates the manifest at `src` and copies it to `<cache_root>/env.yaml`.
pub fn install_env_manifest(src: &Path, cache_root: &Path) -> Result<PathBuf> {
    let content = std::fs::read_to_string(src)
        .with_context(|| format!("failed to read env manifest {}", src.display()))?;
    EnvManifest::parse(&content)?;
    std::fs::create_dir_all(cache_root)?;
    let dest = cache_root.join(ENV_MANIFEST_FILE);
    std::fs::write(&dest, content)?;
    Ok(dest)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_manifest() {
        let m = EnvManifest::parse(
            "base_url: http://localhost:8080\ncompose_file: docker-compose.yml\nauth_env: [API_TOKEN, user_2]\nread_only_mounts: [/data]\n",
        )
        .unwrap();
        assert_eq!(m.base_url, "http://localhost:8080");
        assert_eq!(m.auth_env, vec!["API_TOKEN", "user_2"]);
        assert_eq!(m.read_only_mounts, vec!["/data"]);
    }

    #[test]
    fn rejects_zero_launchers() {
        assert!(EnvManifest::parse("base_url: http://x\n").is_err());
    }

    #[test]
    fn rejects_multiple_launchers() {
        assert!(
            EnvManifest::parse("base_url: http://x\ncommand: run\ndocker_image: img\n").is_err()
        );
    }

    #[test]
    fn rejects_empty_base_url() {
        assert!(EnvManifest::parse("base_url: \"\"\ncommand: run\n").is_err());
    }

    #[test]
    fn rejects_invalid_auth_env_name() {
        assert!(
            EnvManifest::parse("base_url: http://x\ncommand: run\nauth_env: [\"TOKEN=secret\"]\n")
                .is_err()
        );
    }

    #[test]
    fn rejects_unknown_field() {
        assert!(EnvManifest::parse("base_url: http://x\ncommand: run\nextra: 1\n").is_err());
    }

    #[test]
    fn install_copies_to_cache() {
        let dir = tempfile::tempdir().unwrap();
        let src = dir.path().join("in.yaml");
        std::fs::write(&src, "base_url: http://x\ncommand: run\n").unwrap();
        let cache = dir.path().join("cache");
        let dest = install_env_manifest(&src, &cache).unwrap();
        assert_eq!(dest, cache.join("env.yaml"));
        assert_eq!(
            std::fs::read_to_string(dest).unwrap(),
            "base_url: http://x\ncommand: run\n"
        );
    }
}
