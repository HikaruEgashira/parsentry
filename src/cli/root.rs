use anyhow::Result;
use clap::Parser;

use crate::cli::args::{Args, Commands};
use crate::cli::commands::{run_scan_command, run_model_command, run_watch_command};

pub struct RootCommand;

impl RootCommand {
    pub async fn execute() -> Result<()> {
        let args = Args::parse();

        match args.command {
            Commands::Model { target, output } => {
                run_model_command(&target, output.as_deref()).await
            },
            Commands::Scan { target, threat_model, output_dir, diff_base, filter_lang } => {
                run_scan_command(threat_model.as_deref(), &target, output_dir.as_deref(), diff_base.as_deref(), filter_lang.as_deref()).await
            },
            Commands::Merge { dir, output, baseline } => {
                use parsentry_reports::merge_sarif_dir;
                let merged = merge_sarif_dir(&dir, baseline.as_deref())?;
                let json = serde_json::to_string_pretty(&merged)?;
                if let Some(out_path) = output {
                    if let Some(parent) = out_path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    std::fs::write(&out_path, &json)?;
                    eprintln!("Wrote merged SARIF to {}", out_path.display());
                } else {
                    println!("{}", json);
                }
                Ok(())
            },
            Commands::Log { output_dir, follow: _, no_follow, tail, timestamps: _, no_timestamps, interval, timeout, no_color } => {
                let follow = !no_follow;
                let timestamps = !no_timestamps;
                run_watch_command(&output_dir, follow, tail, timestamps, interval, timeout, no_color).await
            },
        }
    }
}
