use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::args::Args;

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
    pub debug: bool,
    
    #[serde(default)]
    pub evaluate: bool,
    
    #[serde(default)]
    pub verbosity: u8,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PathsConfig {
    pub root: Option<PathBuf>,
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
    "o4-mini".to_string()
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
            debug: false,
            evaluate: false,
            verbosity: 0,
        }
    }
}

impl Default for PathsConfig {
    fn default() -> Self {
        Self {
            root: None,
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
model = "o4-mini"
min_confidence = 70
language = "ja"
debug = false
evaluate = false
verbosity = 0

[paths]
# root = "src"
# output_dir = "reports"
# analyze = "specific-file.rs"

[filtering]
# vuln_types = ["SQLI", "XSS", "RCE"]

[api]
# base_url = "https://api.example.com"

[repo]
# url = "hikaruegashira/hikae-vulnerable-javascript"

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
"#.to_string()
        })
    }

    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let content = std::fs::read_to_string(path)?;
        let config: ParsentryConfig = toml::from_str(&content)?;
        Ok(config)
    }

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

    pub fn find_and_load_default() -> Result<Self, ConfigError> {
        if let Some(path) = Self::find_default_config() {
            Self::load_from_file(path)
        } else {
            Ok(Self::default())
        }
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
                        self.analysis.debug = value.parse()
                            .map_err(|_| anyhow!("Invalid debug value: {}", value))?;
                    }
                    "ANALYSIS_EVALUATE" => {
                        self.analysis.evaluate = value.parse()
                            .map_err(|_| anyhow!("Invalid evaluate value: {}", value))?;
                    }
                    "ANALYSIS_VERBOSITY" => {
                        self.analysis.verbosity = value.parse()
                            .map_err(|_| anyhow!("Invalid verbosity value: {}", value))?;
                    }
                    "PATHS_ROOT" => self.paths.root = Some(PathBuf::from(value)),
                    "PATHS_OUTPUT_DIR" => self.paths.output_dir = Some(PathBuf::from(value)),
                    "PATHS_ANALYZE" => self.paths.analyze = Some(PathBuf::from(value)),
                    "FILTERING_VULN_TYPES" => {
                        let types: Vec<String> = value.split(',').map(|s| s.trim().to_string()).collect();
                        self.filtering.vuln_types = Some(types);
                    }
                    "API_BASE_URL" => self.api.base_url = Some(value.clone()),
                    "REPO_URL" => self.repo.url = Some(value.clone()),
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

    pub fn apply_cli_args(&mut self, args: &Args) -> Result<()> {
        if !args.model.is_empty() && args.model != default_model() {
            self.analysis.model = args.model.clone();
        }
        
        if args.min_confidence != default_min_confidence() {
            self.analysis.min_confidence = args.min_confidence;
        }
        
        if !args.language.is_empty() && args.language != default_language() {
            self.analysis.language = args.language.clone();
        }
        
        if args.debug {
            self.analysis.debug = args.debug;
        }
        
        if args.evaluate {
            self.analysis.evaluate = args.evaluate;
        }
        
        if args.verbosity > 0 {
            self.analysis.verbosity = args.verbosity;
        }
        
        if let Some(ref root) = args.root {
            self.paths.root = Some(root.clone());
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
        
        if let Some(ref repo) = args.repo {
            self.repo.url = Some(repo.clone());
        }
        
        if args.generate_patterns {
            self.generation.generate_patterns = args.generate_patterns;
        }
        
        // Apply call graph CLI arguments
        if args.call_graph {
            self.call_graph.call_graph = args.call_graph;
        }
        
        if !args.call_graph_format.is_empty() && args.call_graph_format != default_call_graph_format() {
            self.call_graph.format = args.call_graph_format.clone();
        }
        
        if let Some(ref output) = args.call_graph_output {
            self.call_graph.output = Some(output.clone());
        }
        
        if let Some(ref start_functions_str) = args.call_graph_start_functions {
            let functions: Vec<String> = start_functions_str.split(',').map(|s| s.trim().to_string()).collect();
            self.call_graph.start_functions = Some(functions);
        }
        
        if let Some(max_depth) = args.call_graph_max_depth {
            self.call_graph.max_depth = Some(max_depth);
        }
        
        if let Some(ref include_str) = args.call_graph_include {
            let patterns: Vec<String> = include_str.split(',').map(|s| s.trim().to_string()).collect();
            self.call_graph.include = Some(patterns);
        }
        
        if let Some(ref exclude_str) = args.call_graph_exclude {
            let patterns: Vec<String> = exclude_str.split(',').map(|s| s.trim().to_string()).collect();
            self.call_graph.exclude = Some(patterns);
        }
        
        if args.call_graph_detect_cycles {
            self.call_graph.detect_cycles = args.call_graph_detect_cycles;
        }
        
        if args.call_graph_security_focus {
            self.call_graph.security_focus = args.call_graph_security_focus;
        }
        
        Ok(())
    }

    pub fn load_with_precedence(
        config_path: Option<PathBuf>,
        cli_args: &Args,
        env_vars: &HashMap<String, String>
    ) -> Result<Self> {
        let mut config = if let Some(path) = config_path {
            Self::load_from_file(&path)
                .map_err(|e| anyhow!("Failed to load config file {}: {}", path.display(), e))?
        } else {
            Self::find_and_load_default()
                .unwrap_or_else(|_| Self::default())
        };
        
        config.apply_env_vars(env_vars)?;
        config.apply_cli_args(cli_args)?;
        config.validate()?;
        
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if let Some(ref root) = self.paths.root {
            if !root.exists() {
                return Err(ConfigError::InvalidPath {
                    field: "paths.root".to_string(),
                    path: root.clone(),
                });
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
    
    pub fn to_args(&self) -> Args {
        Args {
            root: self.paths.root.clone(),
            repo: self.repo.url.clone(),
            analyze: self.paths.analyze.clone(),
            model: self.analysis.model.clone(),
            verbosity: self.analysis.verbosity,
            evaluate: self.analysis.evaluate,
            output_dir: self.paths.output_dir.clone(),
            min_confidence: self.analysis.min_confidence,
            vuln_types: self.filtering.vuln_types.as_ref().map(|v| v.join(",")),
            generate_patterns: self.generation.generate_patterns,
            debug: self.analysis.debug,
            api_base_url: self.api.base_url.clone(),
            language: self.analysis.language.clone(),
            config: None,
            generate_config: false,
            call_graph: self.call_graph.call_graph,
            call_graph_format: self.call_graph.format.clone(),
            call_graph_output: self.call_graph.output.clone(),
            call_graph_start_functions: self.call_graph.start_functions.as_ref().map(|v| v.join(",")),
            call_graph_max_depth: self.call_graph.max_depth,
            call_graph_include: self.call_graph.include.as_ref().map(|v| v.join(",")),
            call_graph_exclude: self.call_graph.exclude.as_ref().map(|v| v.join(",")),
            call_graph_detect_cycles: self.call_graph.detect_cycles,
            call_graph_security_focus: self.call_graph.security_focus,
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
        assert_eq!(config.analysis.model, "o4-mini");
        assert_eq!(config.analysis.min_confidence, 70);
        assert_eq!(config.analysis.language, "ja");
        assert!(!config.analysis.debug);
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
debug = true

[paths]
root = "src"
output_dir = "reports"

[filtering]
vuln_types = ["SQLI", "XSS"]
"#;
        
        let config: ParsentryConfig = toml::from_str(toml_content).unwrap();
        assert_eq!(config.analysis.model, "gpt-4");
        assert_eq!(config.analysis.min_confidence, 80);
        assert_eq!(config.analysis.language, "en");
        assert!(config.analysis.debug);
        assert_eq!(config.paths.root, Some(PathBuf::from("src")));
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
        assert!(config.analysis.debug);
    }

    #[test]
    fn test_config_file_loading() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, r#"
[analysis]
model = "claude-3"
min_confidence = 85

[paths]
root = "test"
"#).unwrap();
        
        let config = ParsentryConfig::load_from_file(temp_file.path()).unwrap();
        assert_eq!(config.analysis.model, "claude-3");
        assert_eq!(config.analysis.min_confidence, 85);
        assert_eq!(config.paths.root, Some(PathBuf::from("test")));
    }

    #[test]
    fn test_generate_default_config() {
        let config_string = ParsentryConfig::generate_default_config();
        assert!(config_string.contains("[analysis]"));
        assert!(config_string.contains("model = \"o4-mini\""));
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
}