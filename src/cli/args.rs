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
    /// Run the next phase of the security scan (model → surfaces → report)
    ///
    /// The phase is chosen from cached state: a cold cache starts a full scan,
    /// changed sources re-run only affected surfaces, and a changed repository
    /// structure regenerates the threat model and diffs its findings against
    /// the previous model. Pipe stdout to a CLI agent; each phase prompt tells
    /// the agent to re-run this command until the scan completes.
    Scan {
        /// Target to analyze: local path, GitHub repo (owner/repo), URL, IP, or domain
        #[arg(default_value = ".")]
        target: String,

        /// Also render a PDF report to this path when the scan completes
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Merge per-surface SARIF files into a single report
    #[command(hide = true)]
    Merge {
        /// Target to resolve report directory: local path, owner/repo, URL, IP, or domain
        #[arg(default_value = ".")]
        target: String,

        /// Create GitHub issues for findings (owner/repo format)
        #[arg(long)]
        gh_issue: Option<String>,

        /// Create Jira issues for findings (project key, e.g. SEC)
        #[arg(long)]
        jira: Option<String>,

        /// Create Linear issues for findings (team ID or key, e.g. ENG)
        #[arg(long)]
        linear: Option<String>,

        /// Create Notion pages for findings (database ID)
        #[arg(long)]
        notion: Option<String>,

        /// Minimum severity level to report: error, warning, note (default: warning)
        #[arg(long, default_value = "warning")]
        min_level: String,

        /// Show what would be created without making changes
        #[arg(long)]
        dry_run: bool,
    },
    /// Monitor scan progress (docker compose logs compatible)
    #[command(alias = "logs")]
    Log {
        /// Target to monitor (omit to show all sessions)
        target: Option<String>,

        /// Follow log output
        #[arg(short = 'f', long = "follow", action = clap::ArgAction::SetTrue)]
        follow: bool,

        /// Number of lines to show from the end of the logs
        #[arg(short = 'n', long)]
        tail: Option<usize>,

        /// Show timestamps
        #[arg(short = 't', long = "timestamps", action = clap::ArgAction::SetTrue)]
        timestamps: bool,

        /// Polling interval in seconds
        #[arg(long, default_value = "5")]
        interval: u64,

        /// Timeout in seconds
        #[arg(long)]
        timeout: Option<u64>,

        /// Produce monochrome output
        #[arg(long)]
        no_color: bool,
    },
}
