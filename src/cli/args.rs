use clap::{Parser, Subcommand};
use std::path::PathBuf;


#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Security prompt orchestrator for CLI agents",
    long_about = None,
    before_help = r#"
            ▲
           ╱ ╲
          ╱   ╲
         ╱ ░░░ ╲
        ╱ ░▓▓▓░ ╲
       ╱ ░▓███▓░ ╲
      ╱ ░▓█████▓░ ╲
     ╱_░▓███████▓░_╲
       ─────┬─────
            │
    P A R S E N T R Y
"#
)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
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
        /// Path to threat model JSON file, or "-" for stdin
        threat_model: String,

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
    #[command(hide = true)]
    Merge {
        /// Directory containing *.sarif.json files
        dir: PathBuf,
        /// Output file (default: stdout)
        #[arg(short, long)]
        output: Option<PathBuf>,
        /// Baseline SARIF for comparison (preserves triage state)
        #[arg(short, long)]
        baseline: Option<PathBuf>,
    },
    /// Monitor scan progress (docker compose logs -f style)
    #[command(alias = "logs", alias = "watch")]
    Log {
        /// Output directory containing prompt and SARIF files
        output_dir: PathBuf,

        /// Follow log output
        #[arg(short, long, default_value_t = true, action = clap::ArgAction::SetTrue)]
        follow: bool,

        /// Disable follow mode
        #[arg(long = "no-follow", action = clap::ArgAction::SetTrue)]
        no_follow: bool,

        /// Number of recent events to show initially
        #[arg(long)]
        tail: Option<usize>,

        /// Show timestamps
        #[arg(short, long, default_value_t = true, action = clap::ArgAction::SetTrue)]
        timestamps: bool,

        /// Disable timestamps
        #[arg(long = "no-timestamps", action = clap::ArgAction::SetTrue)]
        no_timestamps: bool,

        /// Polling interval in seconds
        #[arg(long, default_value = "5")]
        interval: u64,

        /// Timeout in seconds
        #[arg(long)]
        timeout: Option<u64>,

        /// Disable color output
        #[arg(long)]
        no_color: bool,
    },
}
