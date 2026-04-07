use anyhow::Result;
use clap::Parser;

use crate::cli::args::{Args, Commands, ScanArgs};
use crate::cli::commands::{run_scan_command, run_model_command, run_query_command, handle_cache_command};
use crate::config::ParsentryConfig;

pub struct RootCommand;

impl RootCommand {
    pub async fn execute() -> Result<()> {
        println!(
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

        match &args.command {
            Some(Commands::Model { target }) => {
                let scan_args = ScanArgs::from(&args).with_target(target.clone());
                run_model_command(scan_args).await
            },
            Some(Commands::Query { target }) => {
                let scan_args = ScanArgs::from(&args).with_target(target.clone());
                run_query_command(scan_args).await
            },
            Some(Commands::Merge { dir, output }) => {
                use parsentry_reports::merge_sarif_dir;
                let merged = merge_sarif_dir(dir)?;
                let json = serde_json::to_string_pretty(&merged)?;
                if let Some(out_path) = output {
                    if let Some(parent) = out_path.parent() {
                        std::fs::create_dir_all(parent)?;
                    }
                    std::fs::write(out_path, &json)?;
                    eprintln!("Wrote merged SARIF to {}", out_path.display());
                } else {
                    println!("{}", json);
                }
                Ok(())
            },
            Some(Commands::Cache { action }) => {
                let config = if let Some(config_path) = &args.config {
                    ParsentryConfig::load_from_file(config_path)?
                } else {
                    ParsentryConfig::load_with_merged_configs()?
                };
                handle_cache_command(action, &config).await
            },
            None => {
                let scan_args = ScanArgs::from(&args);

                if scan_args.generate_config {
                    println!("{}", ParsentryConfig::generate_default_config());
                    return Ok(());
                }

                run_scan_command(scan_args).await
            }
        }
    }
}
