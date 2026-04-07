use anyhow::Result;
use clap::Parser;

use crate::cli::args::{Args, Commands};
use crate::cli::commands::{run_scan_command, run_model_command, run_query_command, handle_cache_command};
use crate::config::ParsentryConfig;

pub struct RootCommand;

impl RootCommand {
    pub async fn execute() -> Result<()> {
        eprintln!(
            r#"
                ▲
               ╱ ╲
              ╱   ╲
             ╱ ░░░ ╲
            ╱ ░▓▓▓░ ╲
           ╱ ░▓███▓░ ╲
          ╱ ░▓█████▓░ ╲
         ╱_░▓███████▓░_╲
           ─────┬─────
                │
        P A R S E N T R Y
                │
             v{}
"#,
            env!("CARGO_PKG_VERSION")
        );

        let args = Args::parse();

        match args.command {
            Commands::Model { target } => {
                run_model_command(&target).await
            },
            Commands::Scan { threat_model, target, output_dir, diff_base, filter_lang } => {
                run_scan_command(&threat_model, &target, output_dir.as_deref(), diff_base.as_deref(), filter_lang.as_deref()).await
            },
            Commands::Merge { dir, output } => {
                use parsentry_reports::merge_sarif_dir;
                let merged = merge_sarif_dir(&dir)?;
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
            Commands::Query { target, threat_model } => {
                run_query_command(&target, threat_model.as_deref()).await
            },
            Commands::Cache { action } => {
                let config = if let Some(config_path) = &args.config {
                    ParsentryConfig::load_from_file(config_path)?
                } else {
                    ParsentryConfig::load_with_merged_configs()?
                };
                handle_cache_command(&action, &config).await
            },
        }
    }
}
