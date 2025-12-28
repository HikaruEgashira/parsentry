use anyhow::Result;
use clap::Parser;

use crate::cli::args::{Args, Commands, ScanArgs, GraphArgs, GenerateArgs, validate_scan_args, validate_graph_args, validate_generate_args};
use crate::cli::commands::{run_scan_command, run_graph_command, run_generate_command};
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
                target,
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
                    target: target.clone().or_else(|| args.target.clone()),
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
            Some(Commands::Generate {
                target,
                output,
                benchmark,
                benchmark_file,
                repos,
                search,
                max_repos,
                claude_code,
            }) => {
                let generate_args = GenerateArgs {
                    target: target.clone(),
                    output: output.clone(),
                    benchmark: *benchmark,
                    benchmark_file: benchmark_file.clone(),
                    repos: repos.clone(),
                    search: search.clone(),
                    max_repos: *max_repos,
                    model: args.model.clone(),
                    verbosity: args.verbosity,
                    debug: args.debug,
                    api_base_url: args.api_base_url.clone(),
                    claude_code: *claude_code,
                };

                validate_generate_args(&generate_args)?;
                run_generate_command(generate_args).await
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