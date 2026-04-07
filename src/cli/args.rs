use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Security prompt orchestrator for CLI agents",
    long_about = None
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbosity: u8,

    #[arg(long, global = true)]
    pub debug: bool,

    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate threat model prompt from repo metadata
    Model {
        /// Target to analyze: local path or GitHub repository (owner/repo)
        #[arg(default_value = ".")]
        target: String,
    },
    /// Generate per-surface analysis prompts from a threat model
    Scan {
        /// Path to threat model JSON file
        threat_model: PathBuf,

        /// Target to analyze: local path or GitHub repository (owner/repo)
        #[arg(default_value = ".")]
        target: String,

        /// Output directory for prompt files
        #[arg(long)]
        output_dir: Option<PathBuf>,

        /// Git ref to diff against (only changed files)
        #[arg(long)]
        diff_base: Option<String>,

        /// Filter by language (comma-separated)
        #[arg(long)]
        filter_lang: Option<String>,
    },
    /// Merge per-surface SARIF files into a single report
    Merge {
        /// Directory containing *.sarif.json files
        dir: PathBuf,
        /// Output file (default: stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    /// Show surface locations and resolved source files
    Query {
        /// Target to analyze: local path or GitHub repository (owner/repo)
        #[arg(default_value = ".")]
        target: String,

        /// Path to threat model JSON file
        #[arg(long)]
        threat_model: Option<PathBuf>,
    },
    /// Manage result cache
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
