use anyhow::Result;
use clap::Parser;

use crate::cli::args::{Args, Commands};
use crate::cli::commands::{run_scan_command, run_model_command, run_watch_command};

pub struct RootCommand;

impl RootCommand {
    pub async fn execute() -> Result<()> {
        let args = Args::parse();

        match args.command {
            Commands::Model { target } => {
                run_model_command(&target).await
            },
            Commands::Scan { target, diff_base, filter_lang } => {
                run_scan_command(&target, diff_base.as_deref(), filter_lang.as_deref()).await
            },
            Commands::Merge { target } => {
                use crate::cli::commands::common::cache_dir_for;
                use parsentry_reports::merge_sarif_dir;
                let reports_dir = cache_dir_for(&target).join("reports");
                let merged = merge_sarif_dir(&reports_dir, None)?;
                let json = serde_json::to_string_pretty(&merged)?;
                println!("{}", json);
                Ok(())
            },
            Commands::Log { target, no_follow, tail, no_timestamps, interval, timeout, no_color } => {
                let reports_dir = crate::cli::commands::common::cache_dir_for(&target).join("reports");
                let follow = !no_follow;
                let timestamps = !no_timestamps;
                run_watch_command(&reports_dir, follow, tail, timestamps, interval, timeout, no_color).await
            },
        }
    }
}
