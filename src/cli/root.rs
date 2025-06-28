use anyhow::Result;
use clap::Parser;

use crate::cli::args::{Args, Commands, ScanArgs, GraphArgs, validate_scan_args, validate_graph_args};
use crate::cli::commands::{run_scan_command, run_graph_command};
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
            Some(Commands::Graph { 
                root, 
                repo, 
                format, 
                output, 
                start_functions, 
                max_depth, 
                include, 
                exclude, 
                detect_cycles, 
                security_focus 
            }) => {
                let graph_args = GraphArgs {
                    root: root.clone().or_else(|| args.root.clone()),
                    repo: repo.clone().or_else(|| args.repo.clone()),
                    format: format.clone(),
                    output: output.clone(),
                    start_functions: start_functions.clone(),
                    max_depth: *max_depth,
                    include: include.clone(),
                    exclude: exclude.clone(),
                    detect_cycles: *detect_cycles,
                    security_focus: *security_focus,
                    verbosity: args.verbosity,
                    debug: args.debug,
                    config: args.config.clone(),
                };
                
                validate_graph_args(&graph_args)?;
                run_graph_command(graph_args).await
            },
            None => {
                // Default to scan command for backward compatibility
                let scan_args = ScanArgs::from(&args);
                
                // Handle config generation mode
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