mod analyzer;
mod evaluator;
mod llm;
mod parser;
mod prompts;
mod repo;
mod response;
mod security_patterns;
mod symbol_finder;

use anyhow::Result;
use clap::Parser;
use log::{info, warn};
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use analyzer::analyze_file;
use evaluator::evaluate_python_vulnerable_app;
use llm::initialize_llm;
use prompts::{README_SUMMARY_PROMPT_TEMPLATE, SYS_PROMPT_TEMPLATE};
use repo::RepoOps;
use symbol_finder::SymbolExtractor;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the root directory of the project
    #[arg(short, long)]
    root: PathBuf,

    /// Specific path or file within the project to analyze
    #[arg(short, long)]
    analyze: Option<PathBuf>,

    /// LLM client to use (default: claude)
    #[arg(short, long, default_value = "claude")]
    llm: String,

    /// Increase output verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbosity: u8,

    /// Enable evaluation mode for example vulnerable apps
    #[arg(short, long)]
    evaluate: bool,
}

fn load_env_file() -> Result<()> {
    if let Ok(file) = File::open(".env") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            if let Some((key, value)) = line.split_once('=') {
                env::set_var(key.trim(), value.trim());
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    load_env_file()?;

    let args = Args::parse();
    let repo = RepoOps::new(args.root.clone());
    let mut code_extractor = SymbolExtractor::new(&args.root);

    println!("\n🔍 Vulnhuntrs - Security Analysis Tool\n");

    // Get repo files excluding tests and documentation
    let files = repo.get_relevant_files();
    println!("📁 Found relevant source files");

    // Get files to analyze based on command line args
    // If no specific path is provided, analyze files with potential security risks
    let files_to_analyze = if let Some(analyze_path) = args.analyze {
        repo.get_files_to_analyze(Some(analyze_path))?
    } else {
        repo.get_network_related_files(&files)
    };

    // Read README content
    let mut system_prompt = SYS_PROMPT_TEMPLATE.to_string();
    if let Some(readme_content) = repo.get_readme_content() {
        println!("📖 Analyzing project README...");
        info!("Summarizing project README");
        let llm = initialize_llm(&args.llm, "")?;
        let summary = llm
            .chat(&format!(
                "{}\n{}",
                readme_content, README_SUMMARY_PROMPT_TEMPLATE
            ))
            .await?;
        info!("README summary complete");
        system_prompt = format!("{}\n\nProject Context:\n{}", system_prompt, summary);
    } else {
        warn!("No README summary found");
    }

    let llm = initialize_llm(&args.llm, &system_prompt)?;

    for file_path in files_to_analyze {
        let file_name = file_path.display().to_string();
        println!("\n📄 Analyzing: {}\n", file_name);
        println!("{}", "=".repeat(80));

        let analysis_result = analyze_file(
            &file_path,
            &llm,
            &mut code_extractor,
            &files,
            args.verbosity,
        )
        .await?;

        analysis_result.print_readable();

        // If evaluation mode is enabled and we're analyzing an example vulnerable app
        if args.evaluate {
            if file_name.contains("python-vulnerable-app") {
                println!("\n📊 Evaluating Analysis Report...\n");
                println!("{}", "=".repeat(80));
                
                let eval_result = evaluate_python_vulnerable_app(&analysis_result, llm.as_ref()).await?;
                eval_result.print_readable();
            }
        }

        println!("\nPress Enter to continue...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
    }

    println!("\n✅ Analysis complete!\n");

    Ok(())
}
