use anyhow::Result;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::cli::ui::StatusPrinter;
use crate::github::clone_repo;

use parsentry_core::{
    RepoMetadata, THREAT_MODEL_SYSTEM_PROMPT,
    build_threat_model_prompt, threat_model_schema,
};

/// Phase 0: Locate and optionally clone the repository.
/// Returns (root_dir, repo_name).
pub fn locate_repository(
    target: &str,
    printer: &StatusPrinter,
) -> Result<(PathBuf, Option<String>)> {
    if target.contains('/') && !Path::new(target).exists() {
        let cache_base = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("/tmp"))
            .join("parsentry")
            .join("repos");
        let dest = cache_base.join(target.replace('/', "__"));
        if dest.exists() {
            std::fs::remove_dir_all(&dest)?;
        }
        std::fs::create_dir_all(cache_base)?;
        printer.status("Cloning", &format!("{} → {}", target, dest.display()));
        clone_repo(target, &dest)?;
        let repo_name = target
            .split('/')
            .last()
            .unwrap_or("unknown-repo")
            .replace(".git", "");
        Ok((dest, Some(repo_name)))
    } else {
        Ok((PathBuf::from(target), None))
    }
}

/// Resolve output directory from base path and optional repo name.
pub fn resolve_output_dir(base: &Option<PathBuf>, repo_name: &Option<String>) -> Option<PathBuf> {
    base.as_ref().map(|dir| {
        if let Some(name) = repo_name {
            dir.join(name)
        } else {
            dir.clone()
        }
    })
}

/// Get files changed relative to a diff base ref.
pub fn get_diff_files(root_dir: &Path, diff_base: &str) -> Result<HashSet<PathBuf>> {
    let three_dot = format!("{}...HEAD", diff_base);
    let output = std::process::Command::new("git")
        .args(["diff", "--name-only", "--diff-filter=ACMR", &three_dot])
        .current_dir(root_dir)
        .output();

    let output = match output {
        Ok(o) if o.status.success() => o,
        _ => std::process::Command::new("git")
            .args(["diff", "--name-only", "--diff-filter=ACMR", diff_base])
            .current_dir(root_dir)
            .output()
            .map_err(|e| anyhow::anyhow!("git diff failed: {}", e))?,
    };

    if !output.status.success() {
        return Err(anyhow::anyhow!(
            "git diff failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| root_dir.join(l.trim()))
        .collect())
}

/// Build threat model prompt for Claude Code CLI.
pub fn build_threat_model_cli_prompt(metadata: &RepoMetadata) -> String {
    let repo_context = metadata.to_prompt_context();
    let languages: Vec<String> = metadata
        .languages
        .keys()
        .map(|l| format!("{:?}", l))
        .collect();
    let user_prompt = build_threat_model_prompt(&repo_context, &languages);
    let schema = serde_json::to_string_pretty(&threat_model_schema()).unwrap_or_default();

    format!(
        "{system}\n\n{user}\n\nIMPORTANT: Return ONLY valid JSON matching this schema, with no markdown wrapping or extra text:\n{schema}",
        system = THREAT_MODEL_SYSTEM_PROMPT,
        user = user_prompt,
        schema = schema,
    )
}
