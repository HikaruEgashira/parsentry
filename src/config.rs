use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::cli::args::ScanArgs;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ParsentryConfig {
    #[serde(default)]
    pub paths: PathsConfig,

    #[serde(default)]
    pub agent: AgentConfig,

    #[serde(default)]
    pub cache: CacheConfig,

    /// Verbosity level: 0=quiet, 1=normal, 2+=debug
    #[serde(default)]
    pub verbosity: u8,
}

/// Agent configuration for CLI executor
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AgentConfig {
    /// Path to agent binary
    pub path: Option<PathBuf>,

    /// Maximum concurrent processes
    #[serde(default = "default_agent_max_concurrent")]
    pub max_concurrent: usize,

    /// Timeout in seconds
    #[serde(default = "default_agent_timeout")]
    pub timeout_secs: u64,
}

fn default_agent_max_concurrent() -> usize {
    10
}

fn default_agent_timeout() -> u64 {
    300
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            path: None,
            max_concurrent: default_agent_max_concurrent(),
            timeout_secs: default_agent_timeout(),
        }
    }
}

/// Cache configuration
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CacheConfig {
    /// Enable cache
    #[serde(default = "default_cache_enabled")]
    pub enabled: bool,

    /// Cache directory
    #[serde(default = "default_cache_directory")]
    pub directory: PathBuf,

    /// Cleanup configuration
    #[serde(default)]
    pub cleanup: CacheCleanupConfig,
}

fn default_cache_enabled() -> bool {
    true
}

fn default_cache_directory() -> PathBuf {
    PathBuf::from(".parsentry/cache")
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            enabled: default_cache_enabled(),
            directory: default_cache_directory(),
            cleanup: CacheCleanupConfig::default(),
        }
    }
}

/// Cache cleanup configuration
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CacheCleanupConfig {
    /// Cleanup trigger type
    #[serde(default = "default_cleanup_trigger")]
    pub trigger: String,

    /// Periodic cleanup interval in days
    #[serde(default = "default_periodic_days")]
    pub periodic_days: Option<usize>,

    /// Size limit in MB
    #[serde(default = "default_size_limit_mb")]
    pub size_limit_mb: Option<usize>,

    /// Maximum age in days
    #[serde(default = "default_max_age_days")]
    pub max_age_days: usize,

    /// Maximum idle days
    #[serde(default = "default_max_idle_days")]
    pub max_idle_days: usize,

    /// Remove version mismatch entries
    #[serde(default = "default_remove_version_mismatch")]
    pub remove_version_mismatch: bool,
}

fn default_cleanup_trigger() -> String {
    "combined".to_string()
}

fn default_periodic_days() -> Option<usize> {
    Some(7)
}

fn default_size_limit_mb() -> Option<usize> {
    Some(500)
}

fn default_max_age_days() -> usize {
    90
}

fn default_max_idle_days() -> usize {
    30
}

fn default_remove_version_mismatch() -> bool {
    true
}

impl Default for CacheCleanupConfig {
    fn default() -> Self {
        Self {
            trigger: default_cleanup_trigger(),
            periodic_days: default_periodic_days(),
            size_limit_mb: default_size_limit_mb(),
            max_age_days: default_max_age_days(),
            max_idle_days: default_max_idle_days(),
            remove_version_mismatch: default_remove_version_mismatch(),
        }
    }
}

impl CacheConfig {
    /// Convert to parsentry-cache types
    pub fn to_cleanup_policy(&self) -> parsentry_cache::CleanupPolicy {
        parsentry_cache::CleanupPolicy {
            max_cache_size_mb: self.cleanup.size_limit_mb.unwrap_or(500),
            max_age_days: self.cleanup.max_age_days,
            max_idle_days: self.cleanup.max_idle_days,
            remove_version_mismatch: self.cleanup.remove_version_mismatch,
        }
    }

    pub fn to_cleanup_trigger(&self) -> parsentry_cache::CleanupTrigger {
        match self.cleanup.trigger.as_str() {
            "periodic" => parsentry_cache::CleanupTrigger::Periodic {
                days: self.cleanup.periodic_days.unwrap_or(7),
            },
            "on_size_limit" => parsentry_cache::CleanupTrigger::OnSizeLimit {
                threshold_mb: self.cleanup.size_limit_mb.unwrap_or(500),
            },
            "manual" => parsentry_cache::CleanupTrigger::Manual,
            _ => parsentry_cache::CleanupTrigger::Combined {
                periodic_days: self.cleanup.periodic_days,
                size_limit_mb: self.cleanup.size_limit_mb,
            },
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PathsConfig {
    pub target: Option<String>,
    pub output_dir: Option<PathBuf>,
}

impl Default for PathsConfig {
    fn default() -> Self {
        Self {
            target: None,
            output_dir: None,
        }
    }
}

impl Default for ParsentryConfig {
    fn default() -> Self {
        Self {
            paths: PathsConfig::default(),
            agent: AgentConfig::default(),
            cache: CacheConfig::default(),
            verbosity: 0,
        }
    }
}

impl ParsentryConfig {
    /// Merge another config into this one (other takes precedence for set values)
    pub fn merge(&mut self, other: &ParsentryConfig) {
        if other.verbosity > 0 {
            self.verbosity = other.verbosity;
        }

        // Paths config
        if other.paths.target.is_some() {
            self.paths.target = other.paths.target.clone();
        }
        if other.paths.output_dir.is_some() {
            self.paths.output_dir = other.paths.output_dir.clone();
        }

        // Agent config
        if other.agent.path.is_some() {
            self.agent.path = other.agent.path.clone();
        }
        if other.agent.max_concurrent != default_agent_max_concurrent() {
            self.agent.max_concurrent = other.agent.max_concurrent;
        }
        if other.agent.timeout_secs != default_agent_timeout() {
            self.agent.timeout_secs = other.agent.timeout_secs;
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("Invalid path in {field}: {path} does not exist")]
    InvalidPath { field: String, path: PathBuf },

    #[error("TOML parsing error: {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Config file not found")]
    ConfigNotFound,
}

impl ParsentryConfig {
    pub fn generate_default_config() -> String {
        let default_config = Self::default();
        toml::to_string_pretty(&default_config).unwrap_or_else(|_| {
            r#"# Parsentry Configuration File

verbosity = 0

[paths]
# target = "src" or "owner/repo"
# output_dir = "reports"

[agent]
# path = "/usr/local/bin/claude"
max_concurrent = 10
timeout_secs = 300

[cache]
enabled = true
directory = ".parsentry/cache"
"#.to_string()
        })
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)?;
        let config: ParsentryConfig = toml::from_str(&content)?;
        Ok(config)
    }

    /// Get the user config file path (~/.config/parsentry/config.toml)
    pub fn get_user_config_path() -> Option<PathBuf> {
        dirs::home_dir().map(|home| home.join(".config/parsentry/config.toml"))
    }

    /// Get the system config file path (/etc/parsentry/config.toml)
    pub fn get_system_config_path() -> PathBuf {
        PathBuf::from("/etc/parsentry/config.toml")
    }

    /// Get the current directory config file path (./parsentry.toml)
    pub fn get_current_config_path() -> PathBuf {
        PathBuf::from("./parsentry.toml")
    }

    /// Ensure user config file exists, creating it if necessary
    pub fn ensure_user_config_exists() -> Result<PathBuf> {
        let user_config_path = Self::get_user_config_path()
            .ok_or_else(|| anyhow!("Could not determine home directory"))?;

        if !user_config_path.exists() {
            if let Some(parent) = user_config_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            let default_config = Self::generate_default_config();
            std::fs::write(&user_config_path, default_config)?;

            tracing::info!("Created user config file at: {}", user_config_path.display());
        }

        Ok(user_config_path)
    }

    /// Load and merge configs from all sources with priority:
    /// 1. User config (~/.config/parsentry/config.toml) - lowest priority
    /// 2. Current directory (./parsentry.toml)
    /// 3. System config (/etc/parsentry/config.toml) - highest priority
    pub fn load_with_merged_configs() -> Result<Self, ConfigError> {
        let mut config = Self::default();

        if let Some(user_path) = Self::get_user_config_path() {
            if user_path.exists() {
                if let Ok(user_config) = Self::load_from_file(&user_path) {
                    config.merge(&user_config);
                    tracing::debug!("Loaded user config from: {}", user_path.display());
                }
            }
        }

        let current_path = Self::get_current_config_path();
        if current_path.exists() {
            if let Ok(current_config) = Self::load_from_file(&current_path) {
                config.merge(&current_config);
                tracing::debug!("Loaded current directory config from: {}", current_path.display());
            }
        }

        let system_path = Self::get_system_config_path();
        if system_path.exists() {
            if let Ok(system_config) = Self::load_from_file(&system_path) {
                config.merge(&system_config);
                tracing::debug!("Loaded system config from: {}", system_path.display());
            }
        }

        Ok(config)
    }

    pub fn apply_env_vars(&mut self, env_vars: &HashMap<String, String>) -> Result<()> {
        for (key, value) in env_vars {
            if let Some(config_key) = key.strip_prefix("PARSENTRY_") {
                match config_key {
                    "VERBOSITY" => {
                        self.verbosity = value.parse()
                            .map_err(|_| anyhow!("Invalid verbosity value: {}", value))?;
                    }
                    "PATHS_TARGET" => self.paths.target = Some(value.clone()),
                    "PATHS_OUTPUT_DIR" => self.paths.output_dir = Some(PathBuf::from(value)),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn apply_scan_args(&mut self, args: &ScanArgs) -> Result<()> {
        if args.debug && self.verbosity < 2 {
            self.verbosity = 2;
        }
        if args.verbosity > 0 {
            self.verbosity = args.verbosity;
        }

        if let Some(ref target) = args.target {
            self.paths.target = Some(target.clone());
        }

        Ok(())
    }

    /// Load configuration with full precedence chain
    pub fn load_with_precedence(
        config_path: Option<PathBuf>,
        cli_args: &ScanArgs,
        env_vars: &HashMap<String, String>
    ) -> Result<Self> {
        if let Err(e) = Self::ensure_user_config_exists() {
            tracing::debug!("Could not create user config: {}", e);
        }

        let mut config = Self::load_with_merged_configs()
            .unwrap_or_else(|_| Self::default());

        if let Some(path) = config_path {
            let explicit_config = Self::load_from_file(&path)
                .map_err(|e| anyhow!("Failed to load config file {}: {}", path.display(), e))?;
            config.merge(&explicit_config);
        }

        config.apply_env_vars(env_vars)?;
        config.apply_scan_args(cli_args)?;
        config.validate()?;

        Ok(config)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if let Some(ref target) = self.paths.target {
            if !target.contains('/') || std::path::Path::new(target).exists() {
                let path = PathBuf::from(target);
                if !path.exists() {
                    return Err(ConfigError::InvalidPath {
                        field: "paths.target".to_string(),
                        path,
                    });
                }
            }
        }

        if self.verbosity > 5 {
            return Err(ConfigError::InvalidPath {
                field: "verbosity".to_string(),
                path: PathBuf::from(format!("{} (valid: 0-5)", self.verbosity)),
            });
        }

        Ok(())
    }

    pub fn to_args(&self) -> ScanArgs {
        ScanArgs {
            target: self.paths.target.clone(),
            verbosity: self.verbosity,
            debug: self.verbosity >= 2,
            config: None,
            generate_config: false,
            filter_lang: None,
            diff_base: None,
            threat_model: None,
            output_dir: self.paths.output_dir.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_default_config() {
        let config = ParsentryConfig::default();
        assert_eq!(config.verbosity, 0);
        assert_eq!(config.agent.max_concurrent, 10);
        assert_eq!(config.agent.timeout_secs, 300);
        assert!(config.cache.enabled);
    }

    #[test]
    fn test_toml_parsing() {
        let toml_content = r#"
verbosity = 2

[paths]
target = "src"
output_dir = "reports"

[agent]
max_concurrent = 5
timeout_secs = 600
"#;

        let config: ParsentryConfig = toml::from_str(toml_content).unwrap();
        assert_eq!(config.verbosity, 2);
        assert_eq!(config.paths.target, Some("src".to_string()));
        assert_eq!(config.paths.output_dir, Some(PathBuf::from("reports")));
        assert_eq!(config.agent.max_concurrent, 5);
        assert_eq!(config.agent.timeout_secs, 600);
    }

    #[test]
    fn test_env_var_application() {
        let mut config = ParsentryConfig::default();
        let mut env_vars = HashMap::new();
        env_vars.insert("PARSENTRY_VERBOSITY".to_string(), "2".to_string());
        env_vars.insert("PARSENTRY_PATHS_TARGET".to_string(), "src".to_string());

        config.apply_env_vars(&env_vars).unwrap();

        assert_eq!(config.verbosity, 2);
        assert_eq!(config.paths.target, Some("src".to_string()));
    }

    #[test]
    fn test_config_file_loading() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, r#"
verbosity = 1

[paths]
target = "test"
"#).unwrap();

        let config = ParsentryConfig::load_from_file(temp_file.path()).unwrap();
        assert_eq!(config.verbosity, 1);
        assert_eq!(config.paths.target, Some("test".to_string()));
    }

    #[test]
    fn test_generate_default_config() {
        let config_string = ParsentryConfig::generate_default_config();
        assert!(config_string.contains("[cache]") || config_string.contains("cache"));
        assert!(config_string.contains("[agent]") || config_string.contains("agent"));
    }

    #[test]
    fn test_validation() {
        let mut config = ParsentryConfig::default();
        assert!(config.validate().is_ok());

        config.verbosity = 10;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_agent_config_default() {
        let config = AgentConfig::default();
        assert_eq!(config.path, None);
        assert_eq!(config.max_concurrent, 10);
        assert_eq!(config.timeout_secs, 300);
    }

    #[test]
    fn test_agent_toml_parsing() {
        let toml_content = r#"
[agent]
path = "/usr/local/bin/claude"
max_concurrent = 5
timeout_secs = 600
"#;

        let config: ParsentryConfig = toml::from_str(toml_content).unwrap();
        assert_eq!(config.agent.path, Some(PathBuf::from("/usr/local/bin/claude")));
        assert_eq!(config.agent.max_concurrent, 5);
        assert_eq!(config.agent.timeout_secs, 600);
    }

    #[test]
    fn test_config_merge() {
        let mut base = ParsentryConfig::default();

        let override_config: ParsentryConfig = toml::from_str(r#"
verbosity = 2

[paths]
target = "custom-target"

[agent]
max_concurrent = 5
"#).unwrap();

        base.merge(&override_config);

        assert_eq!(base.verbosity, 2);
        assert_eq!(base.paths.target, Some("custom-target".to_string()));
        assert_eq!(base.agent.max_concurrent, 5);
    }

    #[test]
    fn test_config_merge_priority() {
        let mut config = ParsentryConfig::default();

        let user_config: ParsentryConfig = toml::from_str(r#"
verbosity = 1

[paths]
target = "user-target"
"#).unwrap();
        config.merge(&user_config);

        let current_config: ParsentryConfig = toml::from_str(r#"
verbosity = 2

[paths]
target = "current-target"
"#).unwrap();
        config.merge(&current_config);

        assert_eq!(config.verbosity, 2);
        assert_eq!(config.paths.target, Some("current-target".to_string()));
    }

    #[test]
    fn test_get_user_config_path() {
        let path = ParsentryConfig::get_user_config_path();
        assert!(path.is_some());
        let path = path.unwrap();
        assert!(path.ends_with(".config/parsentry/config.toml"));
    }

    #[test]
    fn test_get_system_config_path() {
        let path = ParsentryConfig::get_system_config_path();
        assert_eq!(path, PathBuf::from("/etc/parsentry/config.toml"));
    }

    #[test]
    fn test_get_current_config_path() {
        let path = ParsentryConfig::get_current_config_path();
        assert_eq!(path, PathBuf::from("./parsentry.toml"));
    }

    #[test]
    fn test_ensure_user_config_exists() {
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join(".config/parsentry");
        let config_path = config_dir.join("config.toml");

        std::fs::create_dir_all(&config_dir).unwrap();
        let default_config = ParsentryConfig::generate_default_config();
        std::fs::write(&config_path, &default_config).unwrap();

        let content = std::fs::read_to_string(&config_path).unwrap();
        let parsed: Result<ParsentryConfig, _> = toml::from_str(&content);
        assert!(parsed.is_ok());
    }
}
