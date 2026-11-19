//! Environment manifest (`scan --env <PATH>`, cached as `env.yaml`).
//!
//! Describes how to launch the target so indeterminate findings can be
//! verified dynamically. Only the *names* of auth environment variables are
//! recorded; their values never enter the cache.

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvManifest {
    /// Base URL the running target is reachable at.
    pub base_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compose_file: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docker_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    /// Names of environment variables holding credentials for the target.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub auth_env: Vec<String>,
    /// Host paths mounted read-only into the verification environment.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub read_only_mounts: Vec<String>,
}

impl EnvManifest {
    pub fn parse(yaml: &str) -> Result<Self> {
        let manifest: Self = serde_yaml::from_str(yaml).context("invalid env manifest")?;
        manifest.validate()?;
        Ok(manifest)
    }

    pub fn load(path: &Path) -> Result<Self> {
        let yaml = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read env manifest {}", path.display()))?;
        Self::parse(&yaml).with_context(|| format!("in {}", path.display()))
    }

    fn validate(&self) -> Result<()> {
        if self.base_url.trim().is_empty() {
            bail!("env manifest: base_url must not be empty");
        }
        let launchers = [&self.compose_file, &self.docker_image, &self.command]
            .iter()
            .filter(|l| l.is_some())
            .count();
        if launchers != 1 {
            bail!("env manifest: exactly one of compose_file, docker_image, command is required");
        }
        if let Some(name) = self
            .auth_env
            .iter()
            .find(|n| n.is_empty() || !n.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'))
        {
            bail!("env manifest: auth_env entry {name:?} is not an environment variable name");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_manifest() {
        let m = EnvManifest::parse(
            "base_url: http://localhost:8080\ndocker_image: app:latest\n\
             auth_env: [API_TOKEN]\nread_only_mounts: [./data]\n",
        )
        .unwrap();
        assert_eq!(m.docker_image.as_deref(), Some("app:latest"));
        assert_eq!(m.auth_env, vec!["API_TOKEN"]);
        assert_eq!(m.read_only_mounts, vec!["./data"]);
    }

    #[test]
    fn requires_exactly_one_launcher() {
        assert!(EnvManifest::parse("base_url: http://x\n").is_err());
        assert!(EnvManifest::parse("base_url: http://x\ncommand: run\ndocker_image: a\n").is_err());
    }

    #[test]
    fn rejects_unknown_fields_and_bad_values() {
        assert!(EnvManifest::parse("base_url: http://x\ncommand: run\nfoo: 1\n").is_err());
        assert!(EnvManifest::parse("base_url: ''\ncommand: run\n").is_err());
        assert!(
            EnvManifest::parse("base_url: http://x\ncommand: run\nauth_env: ['A=b']\n").is_err()
        );
    }
}
