use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = None
)]
pub struct Args {
    /// Target to analyze: local path or GitHub repository (owner/repo).
    /// Defaults to current directory.
    pub target: Option<String>,

    #[command(subcommand)]
    pub command: Option<Commands>,

    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbosity: u8,

    #[arg(long, global = true)]
    pub debug: bool,

    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,

    #[arg(long)]
    pub generate_config: bool,

    /// Filter files by programming language (comma-separated)
    #[arg(long, global = true)]
    pub filter_lang: Option<String>,

    /// Git ref to diff against (e.g., "origin/main"). Only scans changed files.
    #[arg(long)]
    pub diff_base: Option<String>,

    /// Path to threat model JSON file (for query subcommand).
    /// Use "-" for stdin: `parsentry model repo | parsentry query repo`
    #[arg(long, global = true)]
    pub threat_model: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate threat model prompt. Outputs to stdout for `| claude`.
    Model {
        /// Target to analyze: local path or GitHub repository (owner/repo)
        target: Option<String>,
    },
    /// Run tree-sitter pattern matching. Outputs JSON to stdout.
    /// Requires threat model via --threat-model or stdin.
    Query {
        /// Target to analyze: local path or GitHub repository (owner/repo)
        target: Option<String>,
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
    pub verbosity: u8,
    pub debug: bool,
    pub config: Option<PathBuf>,
    pub generate_config: bool,
    pub filter_lang: Option<String>,
    pub diff_base: Option<String>,
    pub threat_model: Option<PathBuf>,
}

impl From<&Args> for ScanArgs {
    fn from(args: &Args) -> Self {
        ScanArgs {
            target: args.target.clone(),
            verbosity: args.verbosity,
            debug: args.debug,
            config: args.config.clone(),
            generate_config: args.generate_config,
            filter_lang: args.filter_lang.clone(),
            diff_base: args.diff_base.clone(),
            threat_model: args.threat_model.clone(),
        }
    }
}

impl ScanArgs {
    pub fn with_target(mut self, target: Option<String>) -> Self {
        if target.is_some() {
            self.target = target;
        }
        self
    }
}

pub fn validate_scan_args(_args: &ScanArgs) -> Result<()> {
    Ok(())
}
