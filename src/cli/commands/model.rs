use anyhow::Result;

use crate::cli::ui::StatusPrinter;

use super::common::{build_threat_model_cli_prompt, cache_dir_for, locate_repository};

use parsentry_core::RepoMetadata;

pub async fn run_model_command(target: &str) -> Result<()> {
    let printer = StatusPrinter::new();

    let (root_dir, _repo_name) = locate_repository(target, &printer)?;

    let repo_metadata = RepoMetadata::collect(&root_dir)?;

    printer.status(
        "Collected",
        &format!(
            "{} source files across {} languages",
            repo_metadata.total_files,
            repo_metadata.languages.len()
        ),
    );

    let output = cache_dir_for(target).join("model.json");
    let prompt = build_threat_model_cli_prompt(&repo_metadata, &output);
    print!("{}", prompt);

    printer.success("Prompt", &format!("threat model prompt emitted (output → {})", output.display()));
    Ok(())
}
