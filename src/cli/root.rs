use anyhow::Result;
use clap::Parser;

use crate::cli::args::{Args, Commands, ScanArgs, validate_scan_args};
use crate::cli::commands::{run_scan_command, handle_cache_command};
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

                validate_scan_args(&scan_args)?;
                run_scan_command(scan_args).await
            }
        }
    }
}
