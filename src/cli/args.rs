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
    #[command(subcommand)]
    pub command: Option<Commands>,

    // Global options for backward compatibility and scan command
    #[arg(short, long, global = true)]
    pub root: Option<PathBuf>,

    #[arg(long, global = true)]
    pub repo: Option<String>,

    #[arg(short, long)]
    pub analyze: Option<PathBuf>,

    #[arg(short, long, default_value = "o4-mini", global = true)]
    pub model: String,

    #[arg(short, long, action = clap::ArgAction::Count, global = true)]
    pub verbosity: u8,

    #[arg(short, long)]
    pub evaluate: bool,

    #[arg(long, global = true)]
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
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate call graphs from source code
    Graph {
        #[arg(short, long)]
        root: Option<PathBuf>,

        #[arg(long)]
        repo: Option<String>,

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

// Backward compatibility struct for existing scan functionality
#[derive(Debug, Clone)]
pub struct ScanArgs {
    pub root: Option<PathBuf>,
    pub repo: Option<String>,
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
}

impl From<&Args> for ScanArgs {
    fn from(args: &Args) -> Self {
        ScanArgs {
            root: args.root.clone(),
            repo: args.repo.clone(),
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
        }
    }
}

#[derive(Debug, Clone)]
pub struct GraphArgs {
    pub root: Option<PathBuf>,
    pub repo: Option<String>,
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
        if let Err(e) = crate::reports::validate_output_directory(output_dir) {
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
    // Validate root/repo requirement
    if args.root.is_none() && args.repo.is_none() {
        return Err(anyhow::anyhow!("Either --root or --repo must be specified"));
    }

    // Validate output format
    match args.format.to_lowercase().as_str() {
        "json" | "dot" | "mermaid" | "csv" => {},
        _ => return Err(anyhow::anyhow!("Unsupported format: {}", args.format)),
    }

    Ok(())
}
