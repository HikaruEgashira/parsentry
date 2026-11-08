use anyhow::Result;
use clap::Parser;

use crate::cli::args::{Args, Commands};
use crate::cli::commands::common::write_stdout;
use crate::cli::commands::{run_log_command, run_scan_command};

pub struct RootCommand;

impl RootCommand {
    pub async fn execute() -> Result<()> {
        let args = Args::parse();

        match args.command {
            Commands::Scan { target, output } => run_scan_command(&target, output.as_deref()).await,
            Commands::Merge {
                target,
                gh_issue,
                jira,
                linear,
                notion,
                min_level,
                dry_run,
            } => {
                use crate::cli::commands::common::cache_dir_for;
                use crate::cli::commands::scan::current_sarif_files;
                use crate::github::run_gh_issue_command;
                use parsentry_core::ThreatModel;
                use parsentry_reports::{
                    merge_sarif_dir, merge_sarif_files, run_jira_command, run_linear_command,
                    run_notion_command,
                };
                let project_cache = cache_dir_for(&target);
                let reports_dir = project_cache.join("reports");
                // Prefer the current model's surfaces so removed surfaces never leak in.
                let model = std::fs::read_to_string(project_cache.join("model.json"))
                    .ok()
                    .and_then(|s| serde_json::from_str::<ThreatModel>(&s).ok());
                let merged = match model {
                    Some(model) => {
                        merge_sarif_files(&current_sarif_files(&reports_dir, &model), None)?
                    }
                    None => merge_sarif_dir(&reports_dir, None)?,
                };
                write_stdout(&format!("{}\n", serde_json::to_string_pretty(&merged)?))?;
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
            }
            Commands::Log {
                target,
                follow,
                tail,
                timestamps,
                interval,
                timeout,
                no_color,
            } => {
                run_log_command(
                    target.as_deref(),
                    follow,
                    tail,
                    timestamps,
                    interval,
                    timeout,
                    no_color,
                )
                .await
            }
        }
    }
}
