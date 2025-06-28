use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = None,
    group = clap::ArgGroup::new("target")
        .required(false)
        .args(&["root", "repo"])
)]
pub struct Args {
    #[arg(short, long, group = "target")]
    pub root: Option<PathBuf>,

    #[arg(long, group = "target")]
    pub repo: Option<String>,

    #[arg(short, long)]
    pub analyze: Option<PathBuf>,

    #[arg(short, long, default_value = "o4-mini")]
    pub model: String,

    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbosity: u8,

    #[arg(short, long)]
    pub evaluate: bool,

    #[arg(long)]
    pub output_dir: Option<PathBuf>,

    #[arg(long, default_value = "70")]
    pub min_confidence: i32,

    #[arg(long)]
    pub vuln_types: Option<String>,

    #[arg(long)]
    pub generate_patterns: bool,

    #[arg(long)]
    pub debug: bool,

    #[arg(long)]
    pub api_base_url: Option<String>,

    #[arg(long, default_value = "ja")]
    pub language: String,

    #[arg(short, long)]
    pub config: Option<PathBuf>,

    #[arg(long)]
    pub generate_config: bool,
}

pub fn validate_args(args: &Args) -> Result<()> {
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
