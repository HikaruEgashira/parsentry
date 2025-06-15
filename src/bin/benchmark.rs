use anyhow::Result;
use clap::Parser;
use parsentry::benchmark::BenchmarkRunner;
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to benchmarks directory (default: repo/benchmarks)
    #[arg(long, default_value = "repo/benchmarks")]
    benchmarks_dir: PathBuf,

    /// Path to Parsentry results directory (default: benchmarks)  
    #[arg(long, default_value = "benchmarks")]
    results_dir: PathBuf,

    /// Output file for benchmark results
    #[arg(long, default_value = "benchmark_results.json")]
    output: PathBuf,

    /// Repository URL to analyze with Parsentry
    #[arg(long)]
    repo: Option<String>,

    /// Model to use for analysis
    #[arg(long, default_value = "o4-mini")]
    model: String,

    /// Run Parsentry analysis before scoring
    #[arg(long)]
    run_analysis: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Print summary to stdout
    #[arg(long)]
    summary: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Setup logging
    let level = if args.verbose { Level::DEBUG } else { Level::INFO };
    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .init();

    let runner = BenchmarkRunner::new(args.benchmarks_dir.clone(), args.results_dir.clone());

    // Run Parsentry analysis if requested
    if args.run_analysis {
        if let Some(repo_url) = &args.repo {
            let verbosity = if args.verbose { Some("-vv") } else { None };
            runner.run_parsentry_analysis(repo_url, &args.model, verbosity).await?;
        } else {
            return Err(anyhow::anyhow!("--repo is required when --run-analysis is specified"));
        }
    }

    // Run benchmark scoring
    info!("Starting benchmark scoring...");
    let score = runner.run_full_benchmark().await?;

    // Save results
    runner.save_results(&score, &args.output).await?;

    // Print summary if requested
    if args.summary {
        runner.print_summary(&score);
    }

    info!("Benchmark completed successfully!");
    Ok(())
}