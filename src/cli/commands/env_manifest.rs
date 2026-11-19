//! Environment manifest (`--env` / cache `env.yaml`) describing how to run
//! the target so indeterminate findings can be verified dynamically.

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// How to bring up the target and reach it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvManifest {
    /// Base URL of the running target, e.g. `http://localhost:8000`.
    pub base_url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compose_file: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docker_image: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    /// Names (not values) of environment variables holding credentials.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub auth_env: Vec<String>,
    /// Host paths mounted read-only into the environment.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub readonly_mounts: Vec<String>,
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
        Self::parse(&yaml)
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
            bail!("env manifest: specify exactly one of compose_file, docker_image, command");
        }
        if let Some(bad) = self
            .auth_env
            .iter()
            .find(|n| n.is_empty() || !n.chars().all(|c| c.is_ascii_alphanumeric() || c == '_'))
        {
            bail!("env manifest: invalid auth env var name {bad:?}");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_full_manifest() {
        let m = EnvManifest::parse(
            "base_url: http://localhost:8000\ncompose_file: docker-compose.yml\n\
             auth_env: [API_TOKEN]\nreadonly_mounts: [./fixtures]\n",
        )
        .unwrap();
        assert_eq!(m.base_url, "http://localhost:8000");
        assert_eq!(m.compose_file.as_deref(), Some("docker-compose.yml"));
        assert_eq!(m.auth_env, vec!["API_TOKEN"]);
        assert_eq!(m.readonly_mounts, vec!["./fixtures"]);
    }

    #[test]
    fn requires_exactly_one_launcher() {
        assert!(EnvManifest::parse("base_url: http://x\n").is_err());
        assert!(EnvManifest::parse("base_url: http://x\ncommand: a\ndocker_image: b\n").is_err());
        assert!(EnvManifest::parse("base_url: http://x\ncommand: make run\n").is_ok());
    }

    #[test]
    fn rejects_empty_base_url_bad_env_names_and_unknown_fields() {
        assert!(EnvManifest::parse("base_url: ''\ncommand: a\n").is_err());
        assert!(EnvManifest::parse("base_url: http://x\ncommand: a\nauth_env: ['A=b']\n").is_err());
        assert!(EnvManifest::parse("base_url: http://x\ncommand: a\nextra: 1\n").is_err());
    }
}
