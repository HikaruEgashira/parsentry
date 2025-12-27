use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use parsentry_reports::validate_output_directory;

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

    /// Use Claude Code CLI for analysis instead of API
    #[arg(long)]
    pub claude_code: bool,

    /// Path to claude CLI binary
    #[arg(long)]
    pub claude_code_path: Option<PathBuf>,

    /// Maximum concurrent Claude Code processes (max 10)
    #[arg(long, default_value = "10")]
    pub claude_code_concurrency: usize,

    /// Enable PoC execution in Claude Code
    #[arg(long)]
    pub claude_code_poc: bool,

    /// Enable multi-repository variant analysis (MVRA)
    #[arg(long)]
    pub mvra: bool,

    /// GitHub search query for MVRA (e.g., "language:python stars:>100")
    #[arg(long)]
    pub mvra_search_query: Option<String>,

    /// Code search query for MVRA (e.g., "path:*.py import flask")
    #[arg(long)]
    pub mvra_code_query: Option<String>,

    /// Maximum number of repositories to analyze in MVRA
    #[arg(long, default_value = "10")]
    pub mvra_max_repos: usize,

    /// Cache directory for cloned repositories in MVRA
    #[arg(long, default_value = ".parsentry-cache")]
    pub mvra_cache_dir: Option<PathBuf>,

    /// Disable cache in MVRA (always clone fresh)
    #[arg(long)]
    pub mvra_no_cache: bool,
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
    pub claude_code: bool,
    pub claude_code_path: Option<PathBuf>,
    pub claude_code_concurrency: usize,
    pub claude_code_poc: bool,
    pub mvra: bool,
    pub mvra_search_query: Option<String>,
    pub mvra_code_query: Option<String>,
    pub mvra_max_repos: usize,
    pub mvra_cache_dir: Option<PathBuf>,
    pub mvra_no_cache: bool,
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
            claude_code: args.claude_code,
            claude_code_path: args.claude_code_path.clone(),
            claude_code_concurrency: args.claude_code_concurrency,
            claude_code_poc: args.claude_code_poc,
            mvra: args.mvra,
            mvra_search_query: args.mvra_search_query.clone(),
            mvra_code_query: args.mvra_code_query.clone(),
            mvra_max_repos: args.mvra_max_repos,
            mvra_cache_dir: args.mvra_cache_dir.clone(),
            mvra_no_cache: args.mvra_no_cache,
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
