use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::cli::args::{Agent, ScanArgs};
use crate::mvra::MvraConfig;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ParsentryConfig {
    #[serde(default)]
    pub analysis: AnalysisConfig,

    #[serde(default)]
    pub paths: PathsConfig,

    #[serde(default)]
    pub filtering: FilteringConfig,

    #[serde(default)]
    pub api: ApiConfig,

    #[serde(default)]
    pub repo: RepoConfig,

    #[serde(default)]
    pub generation: GenerationConfig,

    #[serde(default)]
    pub call_graph: CallGraphConfigToml,

    #[serde(default)]
    pub agent: AgentConfig,

    #[serde(default)]
    pub mvra: MvraConfig,

    #[serde(default)]
    pub cache: CacheConfig,
}

/// Agent configuration for LLM analysis
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AgentConfig {
    /// Agent type: "genai" (default) or "claude-code"
    #[serde(default = "default_agent_type")]
    pub agent_type: String,

    /// Path to agent binary (for claude-code)
    pub path: Option<PathBuf>,

    /// Maximum concurrent processes
    #[serde(default = "default_agent_max_concurrent")]
    pub max_concurrent: usize,

    /// Timeout in seconds (for claude-code)
    #[serde(default = "default_agent_timeout")]
    pub timeout_secs: u64,

    /// Enable PoC execution (for claude-code)
    #[serde(default)]
    pub enable_poc: bool,
}

fn default_agent_type() -> String {
    "genai".to_string()
}

fn default_agent_max_concurrent() -> usize {
    10
}

fn default_agent_timeout() -> u64 {
    300
}

impl AgentConfig {
    /// Check if Claude Code agent is enabled
    pub fn is_claude_code(&self) -> bool {
        self.agent_type == "claude-code"
    }

    /// Check if Codex agent is enabled
    pub fn is_codex(&self) -> bool {
        self.agent_type == "codex"
    }

    /// Get the Agent enum value
    pub fn get_agent(&self) -> Agent {
        match self.agent_type.as_str() {
            "claude-code" => Agent::ClaudeCode,
            "codex" => Agent::Codex,
            "genai" => Agent::Genai,
            unknown => {
                tracing::warn!(
                    "Unknown agent type '{}' in config, defaulting to 'genai'. Valid values: 'genai', 'claude-code', 'codex'",
                    unknown
                );
                Agent::Genai
            }
        }
    }
}

/// Cache configuration for LLM responses
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
pub struct AnalysisConfig {
    #[serde(default = "default_model")]
    pub model: String,
    
    #[serde(default = "default_min_confidence")]
    pub min_confidence: i32,
    
    #[serde(default = "default_language")]
    pub language: String,

    #[serde(default)]
    pub evaluate: bool,

    /// Verbosity level: 0=quiet, 1=normal, 2+=debug
    #[serde(default)]
    pub verbosity: u8,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PathsConfig {
    pub target: Option<String>,
    pub output_dir: Option<PathBuf>,
    pub analyze: Option<PathBuf>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FilteringConfig {
    pub vuln_types: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ApiConfig {
    pub base_url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RepoConfig {
    pub url: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GenerationConfig {
    #[serde(default)]
    pub generate_patterns: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CallGraphConfigToml {
    #[serde(default)]
    pub call_graph: bool,
    
    #[serde(default = "default_call_graph_format")]
    pub format: String,
    
    pub output: Option<PathBuf>,
    
    pub start_functions: Option<Vec<String>>,
    
    #[serde(default = "default_call_graph_max_depth")]
    pub max_depth: Option<usize>,
    
    pub include: Option<Vec<String>>,
    
    pub exclude: Option<Vec<String>>,
    
    #[serde(default)]
    pub detect_cycles: bool,
    
    #[serde(default)]
    pub security_focus: bool,
}
fn default_model() -> String {
    "gpt-5.1-codex".to_string()
}

fn default_min_confidence() -> i32 {
    70
}

fn default_language() -> String {
    "ja".to_string()
}

fn default_call_graph_format() -> String {
    "json".to_string()
}

fn default_call_graph_max_depth() -> Option<usize> {
    Some(10)
}
impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            model: default_model(),
            min_confidence: default_min_confidence(),
            language: default_language(),
            evaluate: false,
            verbosity: 0,
        }
    }
}

impl Default for PathsConfig {
    fn default() -> Self {
        Self {
            target: None,
            output_dir: None,
            analyze: None,
        }
    }
}

impl Default for FilteringConfig {
    fn default() -> Self {
        Self {
            vuln_types: None,
        }
    }
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            base_url: None,
        }
    }
}

impl Default for RepoConfig {
    fn default() -> Self {
        Self {
            url: None,
        }
    }
}

impl Default for GenerationConfig {
    fn default() -> Self {
        Self {
            generate_patterns: false,
        }
    }
}

impl Default for CallGraphConfigToml {
    fn default() -> Self {
        Self {
            call_graph: false,
            format: default_call_graph_format(),
            output: None,
            start_functions: None,
            max_depth: default_call_graph_max_depth(),
            include: None,
            exclude: None,
            detect_cycles: false,
            security_focus: false,
        }
    }
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            agent_type: default_agent_type(),
            path: None,
            max_concurrent: default_agent_max_concurrent(),
            timeout_secs: default_agent_timeout(),
            enable_poc: false,
        }
    }
}

impl Default for ParsentryConfig {
    fn default() -> Self {
        Self {
            analysis: AnalysisConfig::default(),
            paths: PathsConfig::default(),
            filtering: FilteringConfig::default(),
            api: ApiConfig::default(),
            repo: RepoConfig::default(),
            generation: GenerationConfig::default(),
            call_graph: CallGraphConfigToml::default(),
            agent: AgentConfig::default(),
            mvra: MvraConfig::default(),
            cache: CacheConfig::default(),
        }
    }
}

impl ParsentryConfig {
    /// Merge another config into this one (other takes precedence for set values)
    pub fn merge(&mut self, other: &ParsentryConfig) {
        // Analysis config - merge non-default values
        if other.analysis.model != default_model() {
            self.analysis.model = other.analysis.model.clone();
        }
        if other.analysis.min_confidence != default_min_confidence() {
            self.analysis.min_confidence = other.analysis.min_confidence;
        }
        if other.analysis.language != default_language() {
            self.analysis.language = other.analysis.language.clone();
        }
        if other.analysis.evaluate {
            self.analysis.evaluate = other.analysis.evaluate;
        }
        if other.analysis.verbosity > 0 {
            self.analysis.verbosity = other.analysis.verbosity;
        }

        // Paths config - merge Option fields
        if other.paths.target.is_some() {
            self.paths.target = other.paths.target.clone();
        }
        if other.paths.output_dir.is_some() {
            self.paths.output_dir = other.paths.output_dir.clone();
        }
        if other.paths.analyze.is_some() {
            self.paths.analyze = other.paths.analyze.clone();
        }

        // Filtering config
        if other.filtering.vuln_types.is_some() {
            self.filtering.vuln_types = other.filtering.vuln_types.clone();
        }

        // API config
        if other.api.base_url.is_some() {
            self.api.base_url = other.api.base_url.clone();
        }

        // Repo config
        if other.repo.url.is_some() {
            self.repo.url = other.repo.url.clone();
        }

        // Generation config
        if other.generation.generate_patterns {
            self.generation.generate_patterns = other.generation.generate_patterns;
        }

        // Call graph config
        if other.call_graph.call_graph {
            self.call_graph.call_graph = other.call_graph.call_graph;
        }
        if other.call_graph.format != default_call_graph_format() {
            self.call_graph.format = other.call_graph.format.clone();
        }
        if other.call_graph.output.is_some() {
            self.call_graph.output = other.call_graph.output.clone();
        }
        if other.call_graph.start_functions.is_some() {
            self.call_graph.start_functions = other.call_graph.start_functions.clone();
        }
        if other.call_graph.max_depth != default_call_graph_max_depth() {
            self.call_graph.max_depth = other.call_graph.max_depth;
        }
        if other.call_graph.include.is_some() {
            self.call_graph.include = other.call_graph.include.clone();
        }
        if other.call_graph.exclude.is_some() {
            self.call_graph.exclude = other.call_graph.exclude.clone();
        }
        if other.call_graph.detect_cycles {
            self.call_graph.detect_cycles = other.call_graph.detect_cycles;
        }
        if other.call_graph.security_focus {
            self.call_graph.security_focus = other.call_graph.security_focus;
        }

        // Agent config
        if other.agent.agent_type != default_agent_type() {
            self.agent.agent_type = other.agent.agent_type.clone();
        }
        if other.agent.path.is_some() {
            self.agent.path = other.agent.path.clone();
        }
        if other.agent.max_concurrent != default_agent_max_concurrent() {
            self.agent.max_concurrent = other.agent.max_concurrent;
        }
        if other.agent.timeout_secs != default_agent_timeout() {
            self.agent.timeout_secs = other.agent.timeout_secs;
        }
        if other.agent.enable_poc {
            self.agent.enable_poc = other.agent.enable_poc;
        }

        // MVRA config
        if other.mvra.search_query.is_some() {
            self.mvra.search_query = other.mvra.search_query.clone();
        }
        if other.mvra.code_query.is_some() {
            self.mvra.code_query = other.mvra.code_query.clone();
        }
        if other.mvra.max_repos != 10 {
            self.mvra.max_repos = other.mvra.max_repos;
        }
        if other.mvra.cache_dir != PathBuf::from(".parsentry-cache") {
            self.mvra.cache_dir = other.mvra.cache_dir.clone();
        }
        if !other.mvra.use_cache {
            self.mvra.use_cache = other.mvra.use_cache;
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("Invalid path in {field}: {path} does not exist")]
    InvalidPath { field: String, path: PathBuf },
    
    #[error("Invalid range in {field}: {value} (valid range: {valid_range})")]
    InvalidRange { field: String, value: i32, valid_range: String },
    
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
# For more information, see documentation

[analysis]
model = "gpt-5.1-codex"
min_confidence = 70
language = "ja"
debug = false
evaluate = false
verbosity = 0

[paths]
# target = "src" or "owner/repo"
# output_dir = "reports"
# analyze = "specific-file.rs"

[filtering]
# vuln_types = ["SQLI", "XSS", "RCE"]

[api]
# base_url = "https://api.example.com"

[generation]
generate_patterns = false

[call_graph]
call_graph = false
format = "json"
# output = "call_graph.json"
# start_functions = ["main"]
max_depth = 10
# include = ["src/**"]
# exclude = ["test/**"]
detect_cycles = false
security_focus = false

[agent]
# Agent type: "genai" (default) or "claude-code"
agent_type = "genai"
# path = "/usr/local/bin/claude"  # Only for claude-code
max_concurrent = 10
timeout_secs = 300
enable_poc = false

[mvra]
# search_query = "language:python stars:>100"
# code_query = "path:*.py import flask"
max_repos = 10
cache_dir = ".parsentry-cache"
use_cache = true
# min_stars = 100
# repositories = ["owner/repo1", "owner/repo2"]
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
    /// Returns the path to the user config file
    pub fn ensure_user_config_exists() -> Result<PathBuf> {
        let user_config_path = Self::get_user_config_path()
            .ok_or_else(|| anyhow!("Could not determine home directory"))?;

        if !user_config_path.exists() {
            // Create parent directory if it doesn't exist
            if let Some(parent) = user_config_path.parent() {
                std::fs::create_dir_all(parent)?;
            }

            // Write default config
            let default_config = Self::generate_default_config();
            std::fs::write(&user_config_path, default_config)?;

            tracing::info!("Created user config file at: {}", user_config_path.display());
        }

        Ok(user_config_path)
    }

    /// Load and merge configs from all sources with priority:
    /// 1. User config (~/.config/parsentry/config.toml) - lowest priority (base)
    /// 2. Current directory (./parsentry.toml)
    /// 3. System config (/etc/parsentry/config.toml) - highest priority
    pub fn load_with_merged_configs() -> Result<Self, ConfigError> {
        let mut config = Self::default();

        // 1. Load user config (lowest priority - base settings)
        if let Some(user_path) = Self::get_user_config_path() {
            if user_path.exists() {
                if let Ok(user_config) = Self::load_from_file(&user_path) {
                    config.merge(&user_config);
                    tracing::debug!("Loaded user config from: {}", user_path.display());
                }
            }
        }

        // 2. Load current directory config (medium priority)
        let current_path = Self::get_current_config_path();
        if current_path.exists() {
            if let Ok(current_config) = Self::load_from_file(&current_path) {
                config.merge(&current_config);
                tracing::debug!("Loaded current directory config from: {}", current_path.display());
            }
        }

        // 3. Load system config (highest priority)
        let system_path = Self::get_system_config_path();
        if system_path.exists() {
            if let Ok(system_config) = Self::load_from_file(&system_path) {
                config.merge(&system_config);
                tracing::debug!("Loaded system config from: {}", system_path.display());
            }
        }

        Ok(config)
    }

    #[deprecated(since = "0.12.0", note = "Use load_with_merged_configs() instead")]
    pub fn find_default_config() -> Option<PathBuf> {
        let search_paths = vec![
            "./parsentry.toml",
            "~/.config/parsentry/config.toml",
            "/etc/parsentry/config.toml",
        ];

        for path_str in search_paths {
            let path = if path_str.starts_with("~/") {
                if let Some(home) = dirs::home_dir() {
                    home.join(&path_str[2..])
                } else {
                    continue;
                }
            } else {
                PathBuf::from(path_str)
            };

            if path.exists() {
                return Some(path);
            }
        }
        None
    }

    #[deprecated(since = "0.12.0", note = "Use load_with_merged_configs() instead")]
    pub fn find_and_load_default() -> Result<Self, ConfigError> {
        Self::load_with_merged_configs()
    }

    pub fn apply_env_vars(&mut self, env_vars: &HashMap<String, String>) -> Result<()> {
        for (key, value) in env_vars {
            if let Some(config_key) = key.strip_prefix("PARSENTRY_") {
                match config_key {
                    "ANALYSIS_MODEL" => self.analysis.model = value.clone(),
                    "ANALYSIS_MIN_CONFIDENCE" => {
                        self.analysis.min_confidence = value.parse()
                            .map_err(|_| anyhow!("Invalid min_confidence value: {}", value))?;
                    }
                    "ANALYSIS_LANGUAGE" => self.analysis.language = value.clone(),
                    "ANALYSIS_DEBUG" => {
                        // Legacy: ANALYSIS_DEBUG=true sets verbosity to 2
                        let debug_enabled: bool = value.parse()
                            .map_err(|_| anyhow!("Invalid debug value: {}", value))?;
                        if debug_enabled && self.analysis.verbosity < 2 {
                            self.analysis.verbosity = 2;
                        }
                    }
                    "ANALYSIS_EVALUATE" => {
                        self.analysis.evaluate = value.parse()
                            .map_err(|_| anyhow!("Invalid evaluate value: {}", value))?;
                    }
                    "ANALYSIS_VERBOSITY" => {
                        self.analysis.verbosity = value.parse()
                            .map_err(|_| anyhow!("Invalid verbosity value: {}", value))?;
                    }
                    "PATHS_TARGET" => self.paths.target = Some(value.clone()),
                    "PATHS_OUTPUT_DIR" => self.paths.output_dir = Some(PathBuf::from(value)),
                    "PATHS_ANALYZE" => self.paths.analyze = Some(PathBuf::from(value)),
                    "FILTERING_VULN_TYPES" => {
                        let types: Vec<String> = value.split(',').map(|s| s.trim().to_string()).collect();
                        self.filtering.vuln_types = Some(types);
                    }
                    "API_BASE_URL" => self.api.base_url = Some(value.clone()),
                    "GENERATION_GENERATE_PATTERNS" => {
                        self.generation.generate_patterns = value.parse()
                            .map_err(|_| anyhow!("Invalid generate_patterns value: {}", value))?;
                    }
                    "CALL_GRAPH_ENABLED" => {
                        self.call_graph.call_graph = value.parse()
                            .map_err(|_| anyhow!("Invalid call_graph value: {}", value))?;
                    }
                    "CALL_GRAPH_FORMAT" => self.call_graph.format = value.clone(),
                    "CALL_GRAPH_OUTPUT" => self.call_graph.output = Some(PathBuf::from(value)),
                    "CALL_GRAPH_START_FUNCTIONS" => {
                        let functions: Vec<String> = value.split(',').map(|s| s.trim().to_string()).collect();
                        self.call_graph.start_functions = Some(functions);
                    }
                    "CALL_GRAPH_MAX_DEPTH" => {
                        self.call_graph.max_depth = Some(value.parse()
                            .map_err(|_| anyhow!("Invalid call_graph_max_depth value: {}", value))?);
                    }
                    "CALL_GRAPH_INCLUDE" => {
                        let patterns: Vec<String> = value.split(',').map(|s| s.trim().to_string()).collect();
                        self.call_graph.include = Some(patterns);
                    }
                    "CALL_GRAPH_EXCLUDE" => {
                        let patterns: Vec<String> = value.split(',').map(|s| s.trim().to_string()).collect();
                        self.call_graph.exclude = Some(patterns);
                    }
                    "CALL_GRAPH_DETECT_CYCLES" => {
                        self.call_graph.detect_cycles = value.parse()
                            .map_err(|_| anyhow!("Invalid detect_cycles value: {}", value))?;
                    }
                    "CALL_GRAPH_SECURITY_FOCUS" => {
                        self.call_graph.security_focus = value.parse()
                            .map_err(|_| anyhow!("Invalid security_focus value: {}", value))?;
                    }
                    _ => {} // Ignore unknown environment variables
                }
            }
        }
        Ok(())
    }


    pub fn apply_scan_args(&mut self, args: &ScanArgs) -> Result<()> {
        if !args.model.is_empty() && args.model != default_model() {
            self.analysis.model = args.model.clone();
        }

        if args.min_confidence != default_min_confidence() {
            self.analysis.min_confidence = args.min_confidence;
        }

        if !args.language.is_empty() && args.language != default_language() {
            self.analysis.language = args.language.clone();
        }

        if args.evaluate {
            self.analysis.evaluate = args.evaluate;
        }

        // --debug is equivalent to -vv (verbosity >= 2)
        if args.debug && self.analysis.verbosity < 2 {
            self.analysis.verbosity = 2;
        }
        if args.verbosity > 0 {
            self.analysis.verbosity = args.verbosity;
        }

        if let Some(ref target) = args.target {
            self.paths.target = Some(target.clone());
        }

        if let Some(ref output_dir) = args.output_dir {
            self.paths.output_dir = Some(output_dir.clone());
        }

        if let Some(ref analyze) = args.analyze {
            self.paths.analyze = Some(analyze.clone());
        }

        if let Some(ref vuln_types_str) = args.vuln_types {
            let types: Vec<String> = vuln_types_str.split(',').map(|s| s.trim().to_string()).collect();
            self.filtering.vuln_types = Some(types);
        }

        if let Some(ref base_url) = args.api_base_url {
            self.api.base_url = Some(base_url.clone());
        }

        if args.generate_patterns {
            self.generation.generate_patterns = args.generate_patterns;
        }

        match args.agent {
            Agent::ClaudeCode => {
                self.agent.agent_type = "claude-code".to_string();
            }
            Agent::Codex => {
                self.agent.agent_type = "codex".to_string();
            }
            Agent::Genai => {
                self.agent.agent_type = "genai".to_string();
            }
        }
        if let Some(ref path) = args.agent_path {
            self.agent.path = Some(path.clone());
        }
        if args.agent_concurrency != default_agent_max_concurrent() {
            self.agent.max_concurrent = args.agent_concurrency.min(50);
        }
        if args.agent_poc {
            self.agent.enable_poc = true;
        }

        // Apply cache flag
        self.cache.enabled = args.cache;

        Ok(())
    }

    /// Load configuration with full precedence chain:
    /// 1. Default values (lowest)
    /// 2. User config (~/.config/parsentry/config.toml) - auto-created on first run
    /// 3. Current directory (./parsentry.toml)
    /// 4. System config (/etc/parsentry/config.toml) - highest file priority
    /// 5. Environment variables (PARSENTRY_*)
    /// 6. CLI arguments (highest)
    ///
    /// If config_path is explicitly provided, it's loaded and merged after step 4.
    pub fn load_with_precedence(
        config_path: Option<PathBuf>,
        cli_args: &ScanArgs,
        env_vars: &HashMap<String, String>
    ) -> Result<Self> {
        // Ensure user config exists (auto-create on first run)
        if let Err(e) = Self::ensure_user_config_exists() {
            tracing::debug!("Could not create user config: {}", e);
        }

        // Load merged configs from all sources
        let mut config = Self::load_with_merged_configs()
            .unwrap_or_else(|_| Self::default());

        // If explicit config path is provided, merge it with highest file priority
        if let Some(path) = config_path {
            let explicit_config = Self::load_from_file(&path)
                .map_err(|e| anyhow!("Failed to load config file {}: {}", path.display(), e))?;
            config.merge(&explicit_config);
        }

        // Apply environment variables
        config.apply_env_vars(env_vars)?;
        // Apply CLI arguments (highest priority)
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

        if let Some(ref analyze) = self.paths.analyze {
            if !analyze.exists() {
                return Err(ConfigError::InvalidPath {
                    field: "paths.analyze".to_string(),
                    path: analyze.clone(),
                });
            }
        }
        
        if self.analysis.min_confidence < 0 || self.analysis.min_confidence > 100 {
            return Err(ConfigError::InvalidRange {
                field: "analysis.min_confidence".to_string(),
                value: self.analysis.min_confidence,
                valid_range: "0-100".to_string(),
            });
        }
        
        if self.analysis.verbosity > 5 {
            return Err(ConfigError::InvalidRange {
                field: "analysis.verbosity".to_string(),
                value: self.analysis.verbosity as i32,
                valid_range: "0-5".to_string(),
            });
        }
        
        Ok(())
    }
    
    pub fn to_args(&self) -> ScanArgs {
        ScanArgs {
            target: self.paths.target.clone(),
            analyze: self.paths.analyze.clone(),
            model: self.analysis.model.clone(),
            verbosity: self.analysis.verbosity,
            evaluate: self.analysis.evaluate,
            output_dir: self.paths.output_dir.clone(),
            min_confidence: self.analysis.min_confidence,
            vuln_types: self.filtering.vuln_types.as_ref().map(|v| v.join(",")),
            generate_patterns: self.generation.generate_patterns,
            debug: self.analysis.verbosity >= 2,  // debug is now derived from verbosity
            api_base_url: self.api.base_url.clone(),
            language: self.analysis.language.clone(),
            config: None,
            generate_config: false,
            agent: self.agent.get_agent(),
            agent_path: self.agent.path.clone(),
            agent_concurrency: self.agent.max_concurrent,
            agent_poc: self.agent.enable_poc,
            mvra: false,
            mvra_search_query: self.mvra.search_query.clone(),
            mvra_code_query: self.mvra.code_query.clone(),
            mvra_max_repos: self.mvra.max_repos,
            mvra_cache_dir: Some(self.mvra.cache_dir.clone()),
            mvra_cache: self.mvra.use_cache,
            cache: self.cache.enabled,
            cache_only: false,  // CLI flags are applied in scan command
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
        assert_eq!(config.analysis.model, "gpt-5.1-codex");
        assert_eq!(config.analysis.min_confidence, 70);
        assert_eq!(config.analysis.language, "ja");
        assert!(!config.analysis.evaluate);
        assert_eq!(config.analysis.verbosity, 0);
    }

    #[test]
    fn test_toml_parsing() {
        let toml_content = r#"
[analysis]
model = "gpt-4"
min_confidence = 80
language = "en"
verbosity = 2

[paths]
target = "src"
output_dir = "reports"

[filtering]
vuln_types = ["SQLI", "XSS"]
"#;

        let config: ParsentryConfig = toml::from_str(toml_content).unwrap();
        assert_eq!(config.analysis.model, "gpt-4");
        assert_eq!(config.analysis.min_confidence, 80);
        assert_eq!(config.analysis.language, "en");
        assert_eq!(config.analysis.verbosity, 2);
        assert_eq!(config.paths.target, Some("src".to_string()));
        assert_eq!(config.paths.output_dir, Some(PathBuf::from("reports")));
        assert_eq!(config.filtering.vuln_types, Some(vec!["SQLI".to_string(), "XSS".to_string()]));
    }

    #[test]
    fn test_env_var_application() {
        let mut config = ParsentryConfig::default();
        let mut env_vars = HashMap::new();
        env_vars.insert("PARSENTRY_ANALYSIS_MODEL".to_string(), "gpt-4".to_string());
        env_vars.insert("PARSENTRY_ANALYSIS_MIN_CONFIDENCE".to_string(), "90".to_string());
        env_vars.insert("PARSENTRY_ANALYSIS_DEBUG".to_string(), "true".to_string());

        config.apply_env_vars(&env_vars).unwrap();

        assert_eq!(config.analysis.model, "gpt-4");
        assert_eq!(config.analysis.min_confidence, 90);
        assert_eq!(config.analysis.verbosity, 2);  // DEBUG=true sets verbosity to 2
    }

    #[test]
    fn test_config_file_loading() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, r#"
[analysis]
model = "claude-3"
min_confidence = 85

[paths]
target = "test"
"#).unwrap();

        let config = ParsentryConfig::load_from_file(temp_file.path()).unwrap();
        assert_eq!(config.analysis.model, "claude-3");
        assert_eq!(config.analysis.min_confidence, 85);
        assert_eq!(config.paths.target, Some("test".to_string()));
    }

    #[test]
    fn test_generate_default_config() {
        let config_string = ParsentryConfig::generate_default_config();
        assert!(config_string.contains("[analysis]"));
        assert!(config_string.contains("model = \"gpt-5.1-codex\""));
        assert!(config_string.contains("min_confidence = 70"));
        assert!(config_string.contains("language = \"ja\""));
    }

    #[test]
    fn test_validation() {
        let mut config = ParsentryConfig::default();

        // Test invalid confidence range
        config.analysis.min_confidence = 150;
        assert!(config.validate().is_err());

        // Test valid configuration
        config.analysis.min_confidence = 70;
        assert!(config.validate().is_ok());

        // Test invalid verbosity
        config.analysis.verbosity = 10;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_agent_enum_default() {
        assert_eq!(Agent::default(), Agent::Genai);
    }

    #[test]
    fn test_agent_config_default() {
        let config = AgentConfig::default();
        assert_eq!(config.agent_type, "genai");
        assert_eq!(config.path, None);
        assert_eq!(config.max_concurrent, 10);
        assert_eq!(config.timeout_secs, 300);
        assert!(!config.enable_poc);
    }

    #[test]
    fn test_agent_config_is_claude_code() {
        let mut config = AgentConfig::default();
        assert!(!config.is_claude_code());

        config.agent_type = "claude-code".to_string();
        assert!(config.is_claude_code());

        config.agent_type = "genai".to_string();
        assert!(!config.is_claude_code());
    }

    #[test]
    fn test_agent_config_get_agent() {
        let mut config = AgentConfig::default();
        assert_eq!(config.get_agent(), Agent::Genai);

        config.agent_type = "claude-code".to_string();
        assert_eq!(config.get_agent(), Agent::ClaudeCode);

        config.agent_type = "genai".to_string();
        assert_eq!(config.get_agent(), Agent::Genai);

        // Unknown agent falls back to Genai (with warning logged)
        config.agent_type = "unknown-agent".to_string();
        assert_eq!(config.get_agent(), Agent::Genai);
    }

    #[test]
    fn test_agent_toml_parsing() {
        let toml_content = r#"
[agent]
agent_type = "claude-code"
path = "/usr/local/bin/claude"
max_concurrent = 5
timeout_secs = 600
enable_poc = true
"#;

        let config: ParsentryConfig = toml::from_str(toml_content).unwrap();
        assert_eq!(config.agent.agent_type, "claude-code");
        assert_eq!(config.agent.path, Some(PathBuf::from("/usr/local/bin/claude")));
        assert_eq!(config.agent.max_concurrent, 5);
        assert_eq!(config.agent.timeout_secs, 600);
        assert!(config.agent.enable_poc);
        assert!(config.agent.is_claude_code());
    }

    #[test]
    fn test_agent_to_args_conversion() {
        let mut config = ParsentryConfig::default();

        // Test genai agent
        config.agent.agent_type = "genai".to_string();
        let args = config.to_args();
        assert_eq!(args.agent, Agent::Genai);

        // Test claude-code agent
        config.agent.agent_type = "claude-code".to_string();
        config.agent.path = Some(PathBuf::from("/custom/claude"));
        config.agent.max_concurrent = 8;
        config.agent.enable_poc = true;

        let args = config.to_args();
        assert_eq!(args.agent, Agent::ClaudeCode);
        assert_eq!(args.agent_path, Some(PathBuf::from("/custom/claude")));
        assert_eq!(args.agent_concurrency, 8);
        assert!(args.agent_poc);
    }

    #[test]
    fn test_config_merge() {
        let mut base = ParsentryConfig::default();

        // Create config with overrides
        let override_config: ParsentryConfig = toml::from_str(r#"
[analysis]
model = "gpt-4"
min_confidence = 90
verbosity = 2

[paths]
target = "custom-target"

[agent]
agent_type = "claude-code"
max_concurrent = 5
"#).unwrap();

        base.merge(&override_config);

        // Verify merged values
        assert_eq!(base.analysis.model, "gpt-4");
        assert_eq!(base.analysis.min_confidence, 90);
        assert_eq!(base.analysis.verbosity, 2);
        assert_eq!(base.paths.target, Some("custom-target".to_string()));
        assert_eq!(base.agent.agent_type, "claude-code");
        assert_eq!(base.agent.max_concurrent, 5);

        // Verify default values are preserved where not overridden
        assert_eq!(base.analysis.language, "ja");
        assert!(!base.analysis.evaluate);
    }

    #[test]
    fn test_config_merge_priority() {
        // Simulate: user config -> current config -> system config
        let mut config = ParsentryConfig::default();

        // User config (lowest priority)
        let user_config: ParsentryConfig = toml::from_str(r#"
[analysis]
model = "user-model"
min_confidence = 60
language = "en"
"#).unwrap();
        config.merge(&user_config);

        // Current directory config (medium priority)
        let current_config: ParsentryConfig = toml::from_str(r#"
[analysis]
model = "current-model"
min_confidence = 75
"#).unwrap();
        config.merge(&current_config);

        // System config (highest priority)
        let system_config: ParsentryConfig = toml::from_str(r#"
[analysis]
model = "system-model"
"#).unwrap();
        config.merge(&system_config);

        // System config's model should win
        assert_eq!(config.analysis.model, "system-model");
        // Current config's min_confidence should win (system didn't override)
        assert_eq!(config.analysis.min_confidence, 75);
        // User config's language should be preserved (not overridden)
        assert_eq!(config.analysis.language, "en");
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

        // Create a temporary directory to simulate home
        let temp_dir = TempDir::new().unwrap();
        let config_dir = temp_dir.path().join(".config/parsentry");
        let config_path = config_dir.join("config.toml");

        // Manually test the logic (since we can't easily mock dirs::home_dir)
        std::fs::create_dir_all(&config_dir).unwrap();
        let default_config = ParsentryConfig::generate_default_config();
        std::fs::write(&config_path, &default_config).unwrap();

        // Verify file was created with valid TOML
        let content = std::fs::read_to_string(&config_path).unwrap();
        let parsed: Result<ParsentryConfig, _> = toml::from_str(&content);
        assert!(parsed.is_ok());
    }
}