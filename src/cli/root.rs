use anyhow::Result;
use clap::Parser;

use crate::cli::args::{Args, Commands};
use crate::cli::commands::{run_scan_command, run_model_command, run_log_command};

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
            Commands::Merge { target, gh_issue, jira, linear, notion, min_level, dry_run } => {
                use crate::cli::commands::common::cache_dir_for;
                use crate::github::run_gh_issue_command;
                use parsentry_reports::{merge_sarif_dir, run_jira_command, run_linear_command, run_notion_command};
                let reports_dir = cache_dir_for(&target).join("reports");
                let merged = merge_sarif_dir(&reports_dir, None)?;
                println!("{}", serde_json::to_string_pretty(&merged)?);
                if let Some(repo) = gh_issue {
                    run_gh_issue_command(&reports_dir, &repo, dry_run, &min_level).await?;
                }
                if let Some(project) = jira {
                    run_jira_command(&reports_dir, &project, dry_run, &min_level).await?;
                }
                if let Some(team) = linear {
                    run_linear_command(&reports_dir, &team, dry_run, &min_level).await?;
                }
                if let Some(db_id) = notion {
                    run_notion_command(&reports_dir, &db_id, dry_run, &min_level).await?;
                }
                Ok(())
            },
            Commands::Log { target, follow, tail, timestamps, interval, timeout, no_color } => {
                run_log_command(target.as_deref(), follow, tail, timestamps, interval, timeout, no_color).await
            },
        }
    }
}
