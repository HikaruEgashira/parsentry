use anyhow::Result;
use std::collections::HashSet;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::cli::ui::StatusPrinter;
use crate::github::clone_repo;

use parsentry_core::{
    RepoMetadata, THREAT_MODEL_SYSTEM_PROMPT, build_threat_model_prompt, threat_model_schema,
};

/// Check if the target string is an HTTP(S) URL.
pub fn is_url(target: &str) -> bool {
    target.starts_with("http://") || target.starts_with("https://")
}

/// Compute a filesystem-safe cache key from a URL using SHA-256.
fn url_cache_key(url: &str) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Base cache directory. Respects PARSENTRY_CACHE_DIR, falls back to XDG.
pub fn cache_base() -> PathBuf {
    if let Ok(dir) = std::env::var("PARSENTRY_CACHE_DIR") {
        PathBuf::from(dir)
    } else {
        dirs::cache_dir()
            .expect("failed to resolve XDG cache directory")
            .join("parsentry")
    }
}

/// Per-repository cache directory.
/// e.g. ~/Library/Caches/parsentry/langgenius__dify/
/// For URL targets: ~/Library/Caches/parsentry/url/{sha256}/
pub fn cache_dir_for(target: &str) -> PathBuf {
    if is_url(target) {
        cache_base().join("url").join(url_cache_key(target))
    } else {
        cache_base().join(target.replace('/', "__"))
    }
}

/// Extract short repository name from a target string.
/// e.g. "HikaruEgashira/parsentry" → "parsentry", "/local/path/repo" → "repo"
/// For URL targets: "https://example.com/app" → "example.com"
pub fn repo_name_from_target(target: &str) -> String {
    if is_url(target) {
        target
            .trim_start_matches("http://")
            .trim_start_matches("https://")
            .split('/')
            .next()
            .unwrap_or("url-target")
            .to_string()
    } else {
        target
            .trim_end_matches('/')
            .split('/')
            .last()
            .unwrap_or(target)
            .replace(".git", "")
    }
}

/// Phase 0: Locate and optionally clone the repository.
/// Returns (root_dir, repo_name).
pub async fn locate_repository(
    target: &str,
    printer: &StatusPrinter,
) -> Result<(PathBuf, Option<String>)> {
    if is_url(target) {
        return locate_url_assets(target, printer).await;
    }

    if target.contains('/') && !Path::new(target).exists() {
        let project_cache = cache_dir_for(target);
        let dest = project_cache.join("repo");
        let repo_name = target
            .split('/')
            .last()
            .unwrap_or("unknown-repo")
            .replace(".git", "");

        if dest.join(".git").exists() {
            printer.status("Cached", &format!("{} → {}", target, dest.display()));
        } else {
            if dest.exists() {
                std::fs::remove_dir_all(&dest)?;
            }
            std::fs::create_dir_all(&project_cache)?;
            printer.status("Cloning", &format!("{} → {}", target, dest.display()));
            clone_repo(target, &dest)?;
        }

        Ok((dest, Some(repo_name)))
    } else {
        Ok((PathBuf::from(target), None))
    }
}

/// Fetch frontend assets from a URL target into the cache directory.
async fn locate_url_assets(
    target: &str,
    printer: &StatusPrinter,
) -> Result<(PathBuf, Option<String>)> {
    let project_cache = cache_dir_for(target);
    let asset_dir = project_cache.join("assets");

    if asset_dir.exists() && std::fs::read_dir(&asset_dir)?.next().is_some() {
        printer.status("Cached", &format!("{} → {}", target, asset_dir.display()));
        return Ok((asset_dir, Some(repo_name_from_target(target))));
    }

    std::fs::create_dir_all(&asset_dir)?;
    printer.status("Fetching", &format!("assets from {}", target));

    // Save source URL for log display
    std::fs::write(project_cache.join("source_url.txt"), target)?;

    let collector = crate::url_collector::UrlAssetCollector::new(target)?;
    let assets = collector.collect(&asset_dir).await?;

    if assets.is_empty() {
        anyhow::bail!("No frontend assets found at {}", target);
    }

    printer.status(
        "Collected",
        &format!("{} frontend assets from {}", assets.len(), target),
    );

    Ok((asset_dir, Some(repo_name_from_target(target))))
}

/// Get files changed relative to a diff base ref.
pub fn get_diff_files(root_dir: &Path, diff_base: &str) -> Result<HashSet<PathBuf>> {
    // Reject flag-like values to prevent git argument injection
    if diff_base.starts_with('-') {
        anyhow::bail!("Invalid diff base ref: must not start with '-'");
    }
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

/// Write content to stdout with an explicit flush.
///
/// When stdout is piped (not a TTY), Rust uses full block-buffering by default.
/// `print!` / `println!` only flush on newlines in line-buffered mode (TTY),
/// so downstream processes like `claude -p` may see empty stdin when the
/// producer is slow (e.g. git clone).  This helper guarantees the bytes are
/// delivered immediately.
pub fn write_stdout(content: &str) -> Result<()> {
    let mut out = io::stdout().lock();
    out.write_all(content.as_bytes())?;
    out.flush()?;
    Ok(())
}

/// Build threat model prompt for Claude Code CLI.
pub fn build_threat_model_cli_prompt(metadata: &RepoMetadata, output: &Path) -> String {
    let repo_context = metadata.to_prompt_context();
    let languages: Vec<String> = metadata
        .languages
        .keys()
        .map(|l| format!("{:?}", l))
        .collect();
    let user_prompt = build_threat_model_prompt(&repo_context, &languages);
    let schema = serde_json::to_string_pretty(&threat_model_schema()).unwrap_or_default();

    format!(
        "{system}\n\n{user}\n\nWrite the JSON output to: {output}\nWrite ONLY valid JSON matching this schema. No markdown, no code fences, no explanation.\n{schema}",
        system = THREAT_MODEL_SYSTEM_PROMPT,
        user = user_prompt,
        output = output.display(),
        schema = schema,
    )
}
