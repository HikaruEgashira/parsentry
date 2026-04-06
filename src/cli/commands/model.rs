use anyhow::Result;

use crate::cli::args::ScanArgs;
use crate::cli::ui::StatusPrinter;
use crate::config::ParsentryConfig;

use super::common::{locate_repository, build_threat_model_cli_prompt};

use parsentry_core::RepoMetadata;

pub async fn run_model_command(args: ScanArgs) -> Result<()> {
    let printer = StatusPrinter::new();

    let env_vars: std::collections::HashMap<String, String> = std::env::vars().collect();
    let config = ParsentryConfig::load_with_precedence(args.config.clone(), &args, &env_vars)?;
    let final_args = config.to_args();

    let target = final_args.target.clone().unwrap_or_else(|| ".".to_string());
    let (root_dir, _repo_name) = locate_repository(&target, &printer)?;

    // Collect repository metadata (local only, no LLM call)
    let repo_metadata = RepoMetadata::collect(&root_dir)?;

    printer.status(
        "Collected",
        &format!(
            "{} source files across {} languages",
            repo_metadata.total_files,
            repo_metadata.languages.len()
        ),
    );

    // Output prompt to stdout
    let prompt = build_threat_model_cli_prompt(&repo_metadata);
    print!("{}", prompt);

    printer.success("Prompt", "threat model prompt emitted");
    Ok(())
}
