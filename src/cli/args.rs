use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

use parsentry_reports::validate_output_directory;

/// LLM agent for analysis
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
pub enum Agent {
    /// Use GenAI API (OpenAI-compatible endpoints)
    Genai,
    /// Use Claude Code CLI for analysis
    #[default]
    ClaudeCode,
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

    #[arg(short, long, default_value = "gpt-5.4", global = true)]
    pub model: String,

    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbosity: u8,

    #[arg(long, global = true, default_value = "./reports")]
    pub output_dir: Option<PathBuf>,

    #[arg(long, default_value = "70")]
    pub min_confidence: i32,

    #[arg(long)]
    pub vuln_types: Option<String>,

    #[arg(long, global = true)]
    pub debug: bool,

    #[arg(long, global = true)]
    pub api_base_url: Option<String>,

    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,

    #[arg(long)]
    pub generate_config: bool,

    /// LLM agent to use for analysis
    #[arg(long, value_enum, default_value = "claude-code", global = true)]
    pub agent: Agent,

    /// Path to claude CLI binary
    #[arg(long)]
    pub agent_path: Option<PathBuf>,

    /// Maximum concurrent processes for agent
    #[arg(long, default_value = "10")]
    pub agent_concurrency: usize,

    /// Enable PoC execution
    #[arg(long)]
    pub agent_poc: bool,

    /// Enable LLM response cache [default: true]
    #[arg(long, global = true, default_value = "true", num_args = 0..=1, default_missing_value = "true")]
    pub cache: bool,

    /// Use cache only (fail if cache miss)
    #[arg(long, global = true)]
    pub cache_only: bool,

    /// Filter files by programming language (comma-separated)
    #[arg(long, global = true)]
    pub filter_lang: Option<String>,

    /// Git ref to diff against (e.g., "origin/main"). Only scans changed files.
    #[arg(long)]
    pub diff_base: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
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
        #[arg(long)]
        all: bool,
        #[arg(long)]
        max_age: Option<usize>,
        #[arg(long)]
        max_idle: Option<usize>,
        #[arg(long)]
        dry_run: bool,
    },
    /// Show cache statistics
    Stats,
    /// Clear all cache entries
    Clear {
        #[arg(short, long)]
        yes: bool,
    },
}

#[derive(Debug, Clone)]
pub struct ScanArgs {
    pub target: Option<String>,
    pub model: String,
    pub verbosity: u8,
    pub output_dir: Option<PathBuf>,
    pub min_confidence: i32,
    pub vuln_types: Option<String>,
    pub debug: bool,
    pub api_base_url: Option<String>,
    pub config: Option<PathBuf>,
    pub generate_config: bool,
    pub agent: Agent,
    pub agent_path: Option<PathBuf>,
    pub agent_concurrency: usize,
    pub agent_poc: bool,
    pub cache: bool,
    pub cache_only: bool,
    pub filter_lang: Option<String>,
    pub diff_base: Option<String>,
}

impl From<&Args> for ScanArgs {
    fn from(args: &Args) -> Self {
        ScanArgs {
            target: args.target.clone(),
            model: args.model.clone(),
            verbosity: args.verbosity,
            output_dir: args.output_dir.clone(),
            min_confidence: args.min_confidence,
            vuln_types: args.vuln_types.clone(),
            debug: args.debug,
            api_base_url: args.api_base_url.clone(),
            config: args.config.clone(),
            generate_config: args.generate_config,
            agent: args.agent,
            agent_path: args.agent_path.clone(),
            agent_concurrency: args.agent_concurrency,
            agent_poc: args.agent_poc,
            cache: args.cache,
            cache_only: args.cache_only,
            filter_lang: args.filter_lang.clone(),
            diff_base: args.diff_base.clone(),
        }
    }
}

pub fn validate_scan_args(args: &ScanArgs) -> Result<()> {
    if let Some(output_dir) = &args.output_dir {
        if let Err(e) = validate_output_directory(output_dir) {
            eprintln!(
                "Output directory check failed: {}: {}",
                output_dir.display(),
                e
            );
            std::process::exit(1);
        }
    }
    Ok(())
}
