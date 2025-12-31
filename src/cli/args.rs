use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

use parsentry_reports::validate_output_directory;

/// LLM agent for analysis
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
pub enum Agent {
    /// Use GenAI API (OpenAI-compatible endpoints)
    #[default]
    Genai,
    /// Use Claude Code CLI for analysis
    ClaudeCode,
    /// Use Codex CLI for analysis
    Codex,
}

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = None
)]
pub struct Args {
    /// Target to analyze: local path or GitHub repository (owner/repo)
    pub target: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(short, long)]
    pub analyze: Option<PathBuf>,

    #[arg(short, long, default_value = "gpt-5.1-codex", global = true)]
    pub model: String,

    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbosity: u8,

    #[arg(short, long)]
    pub evaluate: bool,

    #[arg(long, global = true, default_value = "./reports")]
    pub output_dir: Option<PathBuf>,

    #[arg(long, default_value = "70")]
    pub min_confidence: i32,

    #[arg(long)]
    pub vuln_types: Option<String>,

    #[arg(long)]
    pub generate_patterns: bool,

    #[arg(long, global = true)]
    pub debug: bool,

    #[arg(long, global = true)]
    pub api_base_url: Option<String>,

    #[arg(long, default_value = "ja", global = true)]
    pub language: String,

    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,

    #[arg(long)]
    pub generate_config: bool,

    /// LLM agent to use for analysis
    #[arg(long, value_enum, default_value = "genai", global = true)]
    pub agent: Agent,

    /// Path to claude CLI binary (only used with --agent claude-code)
    #[arg(long)]
    pub agent_path: Option<PathBuf>,

    /// Maximum concurrent processes for agent (Claude Code: max 10, GenAI: max 50)
    #[arg(long, default_value = "10")]
    pub agent_concurrency: usize,

    /// Enable PoC execution (only used with --agent claude-code)
    #[arg(long)]
    pub agent_poc: bool,

    /// Enable multi-repository variant analysis (MVRA)
    #[arg(long)]
    pub mvra: bool,

    /// GitHub search query for MVRA (e.g., "language:python stars:>100")
    #[arg(long = "search-query")]
    pub mvra_search_query: Option<String>,

    /// Code search query for MVRA (e.g., "path:*.py import flask")
    #[arg(long = "code-query")]
    pub mvra_code_query: Option<String>,

    /// Maximum number of repositories to analyze in MVRA
    #[arg(long = "max-repos", default_value = "10")]
    pub mvra_max_repos: usize,

    /// Cache directory for cloned repositories in MVRA
    #[arg(long = "cache-dir", default_value = ".parsentry-cache")]
    pub mvra_cache_dir: Option<PathBuf>,

    /// Enable repository cache in MVRA [default: true]
    #[arg(long = "mvra-cache", default_value = "true", num_args = 0..=1, default_missing_value = "true")]
    pub mvra_cache: bool,

    /// Enable LLM response cache [default: true]
    #[arg(long, global = true, default_value = "true", num_args = 0..=1, default_missing_value = "true")]
    pub cache: bool,

    /// Use cache only (fail if cache miss)
    #[arg(long, global = true)]
    pub cache_only: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate call graphs from source code
    Graph {
        /// Target to analyze: local path or GitHub repository (owner/repo)
        target: Option<String>,

        #[arg(short, long, default_value = "json")]
        format: String,

        #[arg(short, long)]
        output: Option<PathBuf>,

        #[arg(long)]
        start_functions: Option<String>,

        #[arg(long, default_value = "10")]
        max_depth: Option<usize>,

        #[arg(long)]
        include: Option<String>,

        #[arg(long)]
        exclude: Option<String>,

        #[arg(long)]
        detect_cycles: bool,

        #[arg(long)]
        security_focus: bool,
    },

    /// Generate security patterns from source code
    Generate {
        /// Target directory or GitHub repository (owner/repo) to analyze
        target: String,

        /// Output directory for generated patterns
        #[arg(short, long)]
        output: Option<PathBuf>,

        /// Run benchmark validation after generation
        #[arg(long)]
        benchmark: bool,

        /// Path to benchmark.json for validation
        #[arg(long)]
        benchmark_file: Option<PathBuf>,

        /// Additional GitHub repositories to analyze (owner/repo format)
        #[arg(long = "repo", short = 'r')]
        repos: Vec<String>,

        /// GitHub search query to find repositories (e.g., "language:python flask")
        #[arg(long)]
        search: Option<String>,

        /// Maximum number of repositories to analyze from search
        #[arg(long, default_value = "5")]
        max_repos: usize,

        /// LLM agent for pattern generation (overrides global --agent)
        #[arg(long, value_enum)]
        agent: Option<Agent>,
    },

    /// Manage LLM response cache
    Cache {
        #[command(subcommand)]
        action: CacheAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum CacheAction {
    /// Clean up stale cache entries
    Clean {
        /// Clean all entries (default: stale only)
        #[arg(long)]
        all: bool,

        /// Maximum age in days (override config)
        #[arg(long)]
        max_age: Option<usize>,

        /// Maximum idle days (override config)
        #[arg(long)]
        max_idle: Option<usize>,

        /// Dry run (show what would be deleted)
        #[arg(long)]
        dry_run: bool,
    },

    /// Show cache statistics
    Stats,

    /// Clear all cache entries
    Clear {
        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },
}

#[derive(Debug, Clone)]
pub struct ScanArgs {
    pub target: Option<String>,
    pub analyze: Option<PathBuf>,
    pub model: String,
    pub verbosity: u8,
    pub evaluate: bool,
    pub output_dir: Option<PathBuf>,
    pub min_confidence: i32,
    pub vuln_types: Option<String>,
    pub generate_patterns: bool,
    pub debug: bool,
    pub api_base_url: Option<String>,
    pub language: String,
    pub config: Option<PathBuf>,
    pub generate_config: bool,
    pub agent: Agent,
    pub agent_path: Option<PathBuf>,
    pub agent_concurrency: usize,
    pub agent_poc: bool,
    pub mvra: bool,
    pub mvra_search_query: Option<String>,
    pub mvra_code_query: Option<String>,
    pub mvra_max_repos: usize,
    pub mvra_cache_dir: Option<PathBuf>,
    pub mvra_cache: bool,
    pub cache: bool,
    pub cache_only: bool,
}

impl From<&Args> for ScanArgs {
    fn from(args: &Args) -> Self {
        ScanArgs {
            target: args.target.clone(),
            analyze: args.analyze.clone(),
            model: args.model.clone(),
            verbosity: args.verbosity,
            evaluate: args.evaluate,
            output_dir: args.output_dir.clone(),
            min_confidence: args.min_confidence,
            vuln_types: args.vuln_types.clone(),
            generate_patterns: args.generate_patterns,
            debug: args.debug,
            api_base_url: args.api_base_url.clone(),
            language: args.language.clone(),
            config: args.config.clone(),
            generate_config: args.generate_config,
            agent: args.agent,
            agent_path: args.agent_path.clone(),
            agent_concurrency: args.agent_concurrency,
            agent_poc: args.agent_poc,
            mvra: args.mvra,
            mvra_search_query: args.mvra_search_query.clone(),
            mvra_code_query: args.mvra_code_query.clone(),
            mvra_max_repos: args.mvra_max_repos,
            mvra_cache_dir: args.mvra_cache_dir.clone(),
            mvra_cache: args.mvra_cache,
            cache: args.cache,
            cache_only: args.cache_only,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GraphArgs {
    pub target: Option<String>,
    pub format: String,
    pub output: Option<PathBuf>,
    pub start_functions: Option<String>,
    pub max_depth: Option<usize>,
    pub include: Option<String>,
    pub exclude: Option<String>,
    pub detect_cycles: bool,
    pub security_focus: bool,
    pub verbosity: u8,
    pub debug: bool,
    pub config: Option<PathBuf>,
}

pub fn validate_scan_args(args: &ScanArgs) -> Result<()> {
    if let Some(output_dir) = &args.output_dir {
        if let Err(e) = validate_output_directory(output_dir) {
            eprintln!(
                "❌ 出力ディレクトリのチェックに失敗: {}: {}",
                output_dir.display(),
                e
            );
            std::process::exit(1);
        }
    }

    Ok(())
}

pub fn validate_graph_args(args: &GraphArgs) -> Result<()> {
    // Validate target requirement
    if args.target.is_none() {
        return Err(anyhow::anyhow!("Target must be specified"));
    }

    // Validate output format
    match args.format.to_lowercase().as_str() {
        "json" | "dot" | "mermaid" | "csv" => {},
        _ => return Err(anyhow::anyhow!("Unsupported format: {}", args.format)),
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub struct GenerateArgs {
    pub target: String,
    pub output: Option<PathBuf>,
    pub benchmark: bool,
    pub benchmark_file: Option<PathBuf>,
    pub repos: Vec<String>,
    pub search: Option<String>,
    pub max_repos: usize,
    pub model: String,
    pub verbosity: u8,
    pub debug: bool,
    pub api_base_url: Option<String>,
    pub agent: Agent,
}

pub fn validate_generate_args(args: &GenerateArgs) -> Result<()> {
    // Check if target is a GitHub repo (owner/repo format) or local path
    let is_github_repo = args.target.contains('/') && !args.target.starts_with('/') && !args.target.starts_with('.');

    if !is_github_repo {
        let target_path = PathBuf::from(&args.target);
        if !target_path.exists() {
            return Err(anyhow::anyhow!(
                "Target directory does not exist: {}",
                args.target
            ));
        }
        if !target_path.is_dir() {
            return Err(anyhow::anyhow!(
                "Target must be a directory: {}",
                args.target
            ));
        }
    }

    if let Some(benchmark_file) = &args.benchmark_file {
        if !benchmark_file.exists() {
            return Err(anyhow::anyhow!(
                "Benchmark file does not exist: {}",
                benchmark_file.display()
            ));
        }
    }

    Ok(())
}
