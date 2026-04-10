use anyhow::Result;

use crate::cli::ui::StatusPrinter;

use super::common::{
    build_threat_model_cli_prompt, cache_dir_for, is_network_target, locate_repository,
    repo_name_from_target, write_stdout,
};

use parsentry_core::RepoMetadata;

pub async fn run_model_command(target: &str) -> Result<()> {
    let printer = StatusPrinter::with_service(repo_name_from_target(target));

    let (root_dir, _repo_name) = locate_repository(target, &printer).await?;

    let mut repo_metadata = RepoMetadata::collect(&root_dir)?;

    if is_network_target(target) {
        repo_metadata.source_url = Some(target.to_string());
    }

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
    write_stdout(&prompt)?;

    printer.success(
        "Prompt",
        &format!(
            "threat model prompt emitted (output → {})",
            output.display()
        ),
    );
    Ok(())
}
