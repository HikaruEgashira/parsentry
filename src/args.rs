use clap::Parser;
use std::path::PathBuf;
use anyhow::Result;
use std::fs;
use std::io::Write;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = None,
    group = clap::ArgGroup::new("target")
        .required(true)
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
}

pub fn validate_output_directory(output_dir: &PathBuf) -> Result<()> {
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)
            .map_err(|e| anyhow::anyhow!("ディレクトリの作成に失敗: {}", e))?;
    }

    let mut test_file_path = output_dir.clone();
    test_file_path.push(".parsentry_write_test");
    
    match fs::File::create(&test_file_path) {
        Ok(mut file) => {
            if let Err(e) = file.write_all(b"test") {
                let _ = fs::remove_file(&test_file_path);
                return Err(anyhow::anyhow!("書き込み権限がありません: {}", e));
            }
            drop(file);
            fs::remove_file(&test_file_path)
                .map_err(|e| anyhow::anyhow!("テストファイルの削除に失敗: {}", e))?;
        }
        Err(e) => {
            return Err(anyhow::anyhow!("ファイル作成権限がありません: {}", e));
        }
    }

    Ok(())
}

pub fn validate_args(args: &Args) -> Result<()> {
    if let Some(output_dir) = &args.output_dir {
        if let Err(e) = validate_output_directory(output_dir) {
            eprintln!("❌ 出力ディレクトリのチェックに失敗: {}: {}", output_dir.display(), e);
            std::process::exit(1);
        }
    }

    Ok(())
}