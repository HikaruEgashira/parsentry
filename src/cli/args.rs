use clap::{Parser, Subcommand};

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
        /// Target to analyze: local path or GitHub repository (owner/repo)
        #[arg(default_value = ".")]
        target: String,

        /// Git ref to diff against (only changed files)
        #[arg(long)]
        diff_base: Option<String>,

        /// Filter by language (comma-separated)
        #[arg(long)]
        filter_lang: Option<String>,

        /// Run agent in background (detached); print output directory and exit
        #[arg(short = 'd', long = "detach", action = clap::ArgAction::SetTrue)]
        detach: bool,
    },
    /// Merge per-surface SARIF files into a single report
    #[command(hide = true)]
    Merge {
        /// Target to resolve report directory (default: .)
        #[arg(default_value = ".")]
        target: String,
    },
    /// Monitor scan progress (docker compose logs -f style)
    #[command(alias = "logs")]
    Log {
        /// Target to monitor (omit to show all sessions)
        target: Option<String>,

        /// Disable follow mode
        #[arg(long = "no-follow", action = clap::ArgAction::SetTrue)]
        no_follow: bool,

        /// Number of recent events to show initially
        #[arg(long)]
        tail: Option<usize>,

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
