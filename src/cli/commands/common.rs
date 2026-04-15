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

/// Check if the target string is an IPv4 address (optionally with port).
fn is_ip_address(target: &str) -> bool {
    let host = target.split(':').next().unwrap_or(target);
    let parts: Vec<&str> = host.split('.').collect();
    if parts.len() != 4 {
        return false;
    }
    parts.iter().all(|p| p.parse::<u8>().is_ok())
}

/// Check if the target string is a domain/FQDN (optionally with port).
/// Requires at least one dot and a TLD of 2+ alphabetic characters.
fn is_domain(target: &str) -> bool {
    let host = target.split(':').next().unwrap_or(target);
    if !host.contains('.') || host.starts_with('.') || host.starts_with('-') {
        return false;
    }
    let labels: Vec<&str> = host.split('.').collect();
    if labels.len() < 2 {
        return false;
    }
    // TLD must be 2+ alphabetic chars
    let tld = labels.last().unwrap();
    if tld.len() < 2 || !tld.chars().all(|c| c.is_ascii_alphabetic()) {
        return false;
    }
    // All labels must be valid
    labels.iter().all(|l| {
        !l.is_empty()
            && l.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
            && !l.starts_with('-')
            && !l.ends_with('-')
    })
}

/// Resolve a network target (IP or domain) to an HTTP(S) URL.
fn resolve_network_target(target: &str) -> String {
    if is_ip_address(target) {
        format!("http://{}", target)
    } else {
        format!("https://{}", target)
    }
}

/// Check if the target is any network-reachable target (URL, IP, or domain).
pub fn is_network_target(target: &str) -> bool {
    is_url(target) || is_ip_address(target) || is_domain(target)
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
/// For network targets: ~/Library/Caches/parsentry/url/{sha256}/
pub fn cache_dir_for(target: &str) -> PathBuf {
    if is_network_target(target) {
        let url = if is_url(target) {
            target.to_string()
        } else {
            resolve_network_target(target)
        };
        cache_base().join("url").join(url_cache_key(&url))
    } else {
        cache_base().join(target.replace('/', "__"))
    }
}

/// Extract short repository name from a target string.
/// e.g. "HikaruEgashira/parsentry" → "parsentry", "/local/path/repo" → "repo"
/// For network targets: "https://example.com/app" → "example.com", "192.168.1.1" → "192.168.1.1"
pub fn repo_name_from_target(target: &str) -> String {
    if is_network_target(target) {
        let host_part = if is_url(target) {
            target
                .trim_start_matches("http://")
                .trim_start_matches("https://")
        } else {
            target
        };
        host_part
            .split('/')
            .next()
            .unwrap_or("network-target")
            .to_string()
    } else {
        target
            .trim_end_matches('/')
            .split('/')
            .next_back()
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

    if (is_ip_address(target) || is_domain(target)) && !Path::new(target).exists() {
        let url = resolve_network_target(target);
        return locate_url_assets(&url, printer).await;
    }

    if target.contains('/') && !Path::new(target).exists() {
        let project_cache = cache_dir_for(target);
        let dest = project_cache.join("repo");
        let repo_name = target
            .split('/')
            .next_back()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_url() {
        assert!(is_url("https://example.com"));
        assert!(is_url("http://example.com"));
        assert!(!is_url("example.com"));
        assert!(!is_url("192.168.1.1"));
    }

    #[test]
    fn test_is_ip_address() {
        assert!(is_ip_address("192.168.1.1"));
        assert!(is_ip_address("10.0.0.1"));
        assert!(is_ip_address("127.0.0.1"));
        assert!(is_ip_address("0.0.0.0"));
        assert!(is_ip_address("192.168.1.1:8080"));
        assert!(!is_ip_address("256.1.1.1"));
        assert!(!is_ip_address("example.com"));
        assert!(!is_ip_address("1.2.3"));
        assert!(!is_ip_address("1.2.3.4.5"));
    }

    #[test]
    fn test_is_domain() {
        assert!(is_domain("example.com"));
        assert!(is_domain("sub.example.com"));
        assert!(is_domain("my-site.co.uk"));
        assert!(is_domain("example.com:8080"));
        assert!(!is_domain("localhost"));
        assert!(!is_domain(".example.com"));
        assert!(!is_domain("example"));
        assert!(!is_domain("192.168.1.1"));
        assert!(!is_domain("-example.com"));
        assert!(!is_domain("example."));
    }

    #[test]
    fn test_resolve_network_target() {
        assert_eq!(resolve_network_target("192.168.1.1"), "http://192.168.1.1");
        assert_eq!(
            resolve_network_target("192.168.1.1:8080"),
            "http://192.168.1.1:8080"
        );
        assert_eq!(resolve_network_target("example.com"), "https://example.com");
        assert_eq!(
            resolve_network_target("sub.example.com:3000"),
            "https://sub.example.com:3000"
        );
    }

    #[test]
    fn test_cache_dir_for_network_target() {
        let url_cache = cache_dir_for("https://example.com");
        let domain_cache = cache_dir_for("example.com");
        // Both should use the same cache since domain resolves to https://example.com
        assert_eq!(url_cache, domain_cache);
        assert!(url_cache.to_string_lossy().contains("url"));
    }

    #[test]
    fn test_repo_name_from_network_target() {
        assert_eq!(repo_name_from_target("example.com"), "example.com");
        assert_eq!(repo_name_from_target("192.168.1.1"), "192.168.1.1");
        assert_eq!(
            repo_name_from_target("example.com:8080"),
            "example.com:8080"
        );
        assert_eq!(
            repo_name_from_target("https://example.com/app"),
            "example.com"
        );
    }
}
