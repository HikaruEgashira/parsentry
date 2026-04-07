use anyhow::{anyhow, Result};
use octocrab::Octocrab;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use tracing::debug;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub owner: String,
    pub repo: String,
    pub full_name: String,
    pub clone_url: String,
    pub html_url: String,
    pub stars: u32,
    pub description: Option<String>,
}

pub struct GitHubSearchClient {
    client: Octocrab,
}

/// Get verified git binary path from trusted locations
fn get_verified_git_path() -> Option<String> {
    let git_path = Command::new("which")
        .arg("git")
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                String::from_utf8(o.stdout).ok().map(|s| s.trim().to_string())
            } else {
                None
            }
        })?;

    // Verify git is from a trusted location
    // Allow: /usr/*, /opt/*, ~/.nix-profile/*, /nix/store/*
    let is_trusted = git_path.starts_with("/usr/")
        || git_path.starts_with("/opt/")
        || git_path.contains(".nix-profile/")
        || git_path.starts_with("/nix/store/");

    if !is_trusted {
        debug!("git binary not in trusted location: {}", git_path);
        return None;
    }

    Some(git_path)
}

/// Clone a GitHub repository to the specified destination
///
/// # Arguments
/// * `repo` - Repository in "owner/repo" format
/// * `dest` - Destination directory path
pub fn clone_repo(repo: &str, dest: &Path) -> Result<()> {
    if dest.exists() {
        anyhow::bail!("Destination directory already exists");
    }

    let url = format!("https://github.com/{}.git", repo);

    // Use verified git path if available, otherwise fall back to "git"
    let git_cmd = get_verified_git_path().unwrap_or_else(|| "git".to_string());

    let output = Command::new(&git_cmd)
        .args(["clone", "--depth", "1", &url])
        .arg(dest)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("git clone failed: {}", stderr);
    }

    Ok(())
}

impl GitHubSearchClient {
    /// Create a new GitHub search client
    /// Uses git credential helper for authentication, falling back to GITHUB_TOKEN env var
    pub fn new() -> Result<Self> {
        let mut builder = Octocrab::builder();

        // Try to get token from git credential helper first, then fall back to env var
        if let Some(token) = Self::get_token_from_credential_helper()
            .or_else(|| env::var("GITHUB_TOKEN").ok())
        {
            if !token.is_empty() {
                builder = builder.personal_token(token);
            }
        }

        let client = builder.build()
            .map_err(|e| anyhow!("Failed to create GitHub client: {}", e))?;

        Ok(Self { client })
    }

    /// Get GitHub token from git credential helper
    fn get_token_from_credential_helper() -> Option<String> {
        let git_path = get_verified_git_path()?;

        let mut child = Command::new(&git_path)
            .args(["credential", "fill"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .ok()?;

        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(b"protocol=https\nhost=github.com\n\n");
        }

        let output = child.wait_with_output().ok()?;

        if !output.status.success() {
            debug!("git credential helper failed");
            return None;
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if let Some(password) = line.strip_prefix("password=") {
                debug!("Retrieved token from git credential helper");
                return Some(password.to_string());
            }
        }

        None
    }

    /// Search for repositories using GitHub Search API
    ///
    /// # Arguments
    /// * `query` - GitHub search query (e.g., "language:python stars:>100")
    /// * `max_results` - Maximum number of repositories to return
    ///
    /// # Returns
    /// Vector of SearchResult containing repository information
    pub async fn search_repositories(&self, query: &str, max_results: usize) -> Result<Vec<SearchResult>> {
        let mut results = Vec::new();
        let mut page = 1u32;
        let per_page = 100u8; // GitHub API max per page

        while results.len() < max_results {
            let response: RepositorySearchResponse = self.client
                .get(format!("/search/repositories?q={}&per_page={}&page={}",
                    urlencoding::encode(query), per_page, page), None::<&()>)
                .await
                .map_err(|e| anyhow!("GitHub API search failed: {}", e))?;

            if response.items.is_empty() {
                break;
            }

            for repo in response.items {
                if results.len() >= max_results {
                    break;
                }

                let owner = repo.owner.login.clone();

                results.push(SearchResult {
                    owner: owner.clone(),
                    repo: repo.name.clone(),
                    full_name: repo.full_name.clone(),
                    clone_url: repo.clone_url.clone(),
                    html_url: repo.html_url.clone(),
                    stars: repo.stargazers_count,
                    description: repo.description.clone(),
                });
            }

            page += 1;
        }

        Ok(results)
    }

    /// Search for code in repositories
    ///
    /// # Arguments
    /// * `query` - Code search query (e.g., "path:*.py import requests")
    /// * `max_results` - Maximum number of code results to return
    ///
    /// # Returns
    /// Vector of CodeSearchResult containing code match information
    pub async fn search_code(&self, query: &str, max_results: usize) -> Result<Vec<CodeSearchResult>> {
        let mut results = Vec::new();
        let mut page = 1u32;
        let per_page = 100u8;

        while results.len() < max_results {
            let response = self.client
                .get::<CodeSearchResponse, _, _>(
                    format!("/search/code?q={}&per_page={}&page={}",
                        urlencoding::encode(query), per_page, page),
                    None::<&()>
                )
                .await
                .map_err(|e| anyhow!("GitHub code search failed: {}", e))?;

            if response.items.is_empty() {
                break;
            }

            for item in response.items {
                if results.len() >= max_results {
                    break;
                }

                results.push(CodeSearchResult {
                    name: item.name,
                    path: item.path,
                    sha: item.sha,
                    html_url: item.html_url,
                    repository: item.repository,
                });
            }

            page += 1;
        }

        Ok(results)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeSearchResult {
    pub name: String,
    pub path: String,
    pub sha: String,
    pub html_url: String,
    pub repository: CodeRepository,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeRepository {
    pub name: String,
    pub full_name: String,
    pub owner: CodeOwner,
    pub html_url: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeOwner {
    pub login: String,
}

#[derive(Debug, Deserialize)]
struct CodeSearchResponse {
    items: Vec<CodeSearchItem>,
}

#[derive(Debug, Deserialize)]
struct CodeSearchItem {
    name: String,
    path: String,
    sha: String,
    html_url: String,
    repository: CodeRepository,
}

#[derive(Debug, Deserialize)]
struct RepositorySearchResponse {
    items: Vec<RepositoryItem>,
}

#[derive(Debug, Deserialize)]
struct RepositoryItem {
    name: String,
    full_name: String,
    owner: RepoOwner,
    html_url: String,
    clone_url: String,
    description: Option<String>,
    stargazers_count: u32,
}

#[derive(Debug, Deserialize)]
struct RepoOwner {
    login: String,
}

// ── Issue reporting ───────────────────────────────────────────────────────────

const ISSUE_LABEL: &str = "parsentry";

/// Create GitHub issues from per-surface SARIF reports.
///
/// For each surface a parent issue is created with title `[Parsentry] {surface_name}`.
/// Each finding becomes a child issue.  The parent body contains a tasklist
/// linking all child issues and is patched after each child is created.
///
/// Deduplication:
/// - Parent issues are identified by `<!-- parsentry-surface: {surface_name} -->` in body.
/// - Child issues are identified by `<!-- parsentry-fp: {fp} -->` in body.
/// - `baselineState == "absent"` → close child issue and mark tasklist item `[x]`.
/// - `baselineState == "unchanged"` or fingerprint already exists → skip.
pub async fn run_gh_issue_command(
    reports_dir: &Path,
    repo: &str,
    dry_run: bool,
    min_level: &str,
) -> Result<()> {
    use parsentry_reports::report_common::{
        build_markdown_body, build_title, extract_fingerprint,
        load_surface_reports, parse_fingerprint_from_body, parse_surface_from_body,
        SURFACE_MARKER,
    };

    let parts: Vec<&str> = repo.splitn(2, '/').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        anyhow::bail!("--gh-issue must be in 'owner/repo' format, got: {}", repo);
    }
    let (owner, repo_name) = (parts[0], parts[1]);

    let mut builder = Octocrab::builder();
    if let Some(token) = GitHubSearchClient::get_token_from_credential_helper()
        .or_else(|| env::var("GITHUB_TOKEN").ok())
    {
        if !token.is_empty() {
            builder = builder.personal_token(token);
        }
    }
    let client = builder.build().map_err(|e| anyhow!("Failed to create GitHub client: {}", e))?;

    let surfaces = load_surface_reports(reports_dir, min_level)?;
    if surfaces.is_empty() {
        eprintln!("No findings to report (level >= {min_level}).");
        return Ok(());
    }

    // Fetch all existing open parsentry issues once.
    // Build two maps: fingerprint → issue_number, surface_name → issue_number.
    let mut fp_map = std::collections::HashMap::<String, u64>::new();
    let mut surface_map = std::collections::HashMap::<String, u64>::new();
    let mut page = 1u32;
    loop {
        let labels: Vec<String> = vec![ISSUE_LABEL.to_string()];
        let issues = client
            .issues(owner, repo_name)
            .list()
            .labels(&labels)
            .state(octocrab::params::State::Open)
            .per_page(100)
            .page(page)
            .send()
            .await
            .map_err(|e| anyhow!("Failed to list issues: {}", e))?;
        let items = issues.items;
        if items.is_empty() { break; }
        for issue in &items {
            if let Some(body) = &issue.body {
                if let Some(fp) = parse_fingerprint_from_body(body) {
                    fp_map.insert(fp, issue.number);
                }
                if let Some(surface) = parse_surface_from_body(body) {
                    surface_map.insert(surface, issue.number);
                }
            }
        }
        if items.len() < 100 { break; }
        page += 1;
    }
    eprintln!(
        "Found {} existing child issue(s) and {} surface issue(s) in {owner}/{repo_name}.",
        fp_map.len(), surface_map.len()
    );

    let (mut created, mut skipped, mut closed) = (0usize, 0usize, 0usize);

    for surface in &surfaces {
        let surface_title = format!("[Parsentry] {}", surface.surface_name);

        // Ensure parent surface issue exists.
        let parent_number = if let Some(&num) = surface_map.get(&surface.surface_name) {
            num
        } else if dry_run {
            eprintln!("[dry-run] Would create surface issue: {surface_title}");
            0
        } else {
            let parent_body = format!(
                "## Surface: {}\n\nThis issue tracks all parsentry findings for this surface.\n\n## Findings\n\n---\n*Generated by [parsentry](https://github.com/HikaruEgashira/parsentry)*\n\n{SURFACE_MARKER} {} -->",
                surface.surface_name, surface.surface_name
            );
            let issue = client
                .issues(owner, repo_name)
                .create(&surface_title)
                .body(&parent_body)
                .labels(vec![ISSUE_LABEL.to_string()])
                .send()
                .await
                .map_err(|e| anyhow!("Failed to create surface issue: {e}"))?;
            eprintln!("Created surface issue #{}: {}", issue.number, issue.html_url);
            surface_map.insert(surface.surface_name.clone(), issue.number);
            issue.number
        };

        // Track child issue numbers for tasklist patching.
        let mut tasklist_items: Vec<(u64, String, bool)> = Vec::new(); // (number, label, done)

        for result in &surface.results {
            let fp = extract_fingerprint(result);

            if result.baseline_state.as_deref() == Some("absent") {
                if let Some(&num) = fp.as_ref().and_then(|f| fp_map.get(f)) {
                    if dry_run {
                        eprintln!("[dry-run] Would close issue #{num} (absent: {})", result.rule_id);
                    } else {
                        client
                            .issues(owner, repo_name)
                            .update(num)
                            .state(octocrab::models::IssueState::Closed)
                            .send()
                            .await
                            .map_err(|e| anyhow!("Failed to close issue #{num}: {e}"))?;
                        eprintln!("Closed issue #{num} (resolved: {})", result.rule_id);
                    }
                    let label = format!(
                        "[{}] {} in {}",
                        result.level.to_uppercase(),
                        result.rule_id,
                        result.locations.first()
                            .map(|l| l.physical_location.artifact_location.uri.split('/').next_back().unwrap_or("unknown"))
                            .unwrap_or("unknown")
                    );
                    tasklist_items.push((num, label, true));
                    closed += 1;
                }
                continue;
            }

            if result.baseline_state.as_deref() == Some("unchanged") {
                skipped += 1;
                continue;
            }
            if let Some(f) = &fp {
                if let Some(&num) = fp_map.get(f) {
                    // already exists — add to tasklist but don't recreate
                    let label = format!(
                        "[{}] {} in {}",
                        result.level.to_uppercase(),
                        result.rule_id,
                        result.locations.first()
                            .map(|l| l.physical_location.artifact_location.uri.split('/').next_back().unwrap_or("unknown"))
                            .unwrap_or("unknown")
                    );
                    tasklist_items.push((num, label, false));
                    skipped += 1;
                    continue;
                }
            }

            let title = build_title(result);
            let body = build_markdown_body(result, fp.as_deref());

            if dry_run {
                eprintln!("[dry-run] Would create: {title}");
                created += 1;
            } else {
                let issue = client
                    .issues(owner, repo_name)
                    .create(&title)
                    .body(&body)
                    .labels(vec![ISSUE_LABEL.to_string()])
                    .send()
                    .await
                    .map_err(|e| anyhow!("Failed to create issue: {e}"))?;
                eprintln!("Created: {}", issue.html_url);
                if let Some(f) = &fp {
                    fp_map.insert(f.clone(), issue.number);
                }
                let label = format!(
                    "[{}] {} in {}",
                    result.level.to_uppercase(),
                    result.rule_id,
                    result.locations.first()
                        .map(|l| l.physical_location.artifact_location.uri.split('/').next_back().unwrap_or("unknown"))
                        .unwrap_or("unknown")
                );
                tasklist_items.push((issue.number, label, false));
                created += 1;
            }
        }

        // Patch parent issue body with updated tasklist.
        if !dry_run && parent_number != 0 && !tasklist_items.is_empty() {
            let mut findings_md = String::from("## Findings\n\n");
            for (num, label, done) in &tasklist_items {
                let check = if *done { "x" } else { " " };
                findings_md.push_str(&format!("- [{check}] #{num} {label}\n"));
            }
            let new_body = format!(
                "## Surface: {}\n\nThis issue tracks all parsentry findings for this surface.\n\n{findings_md}\n---\n*Generated by [parsentry](https://github.com/HikaruEgashira/parsentry)*\n\n{SURFACE_MARKER} {} -->",
                surface.surface_name, surface.surface_name
            );
            client
                .issues(owner, repo_name)
                .update(parent_number)
                .body(new_body.as_str())
                .send()
                .await
                .map_err(|e| anyhow!("Failed to update surface issue #{parent_number}: {e}"))?;
            eprintln!("Updated surface issue #{parent_number} with tasklist.");
        }
    }

    eprintln!(
        "Done. created={created}, skipped={skipped}, closed={closed}{}",
        if dry_run { " (dry-run)" } else { "" }
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires GITHUB_TOKEN and network access
    async fn test_search_repositories() {
        let client = GitHubSearchClient::new().unwrap();
        let results = client.search_repositories("language:rust stars:>1000", 5).await.unwrap();

        assert!(!results.is_empty());
        assert!(results.len() <= 5);

        for result in results {
            assert!(!result.owner.is_empty());
            assert!(!result.repo.is_empty());
            assert!(result.stars > 1000);
        }
    }

    #[tokio::test]
    #[ignore] // Requires GITHUB_TOKEN and network access
    async fn test_search_code() {
        let client = GitHubSearchClient::new().unwrap();
        let results = client.search_code("language:python path:*.py flask", 5).await.unwrap();

        // Code search might return no results depending on the query
        if !results.is_empty() {
            assert!(results.len() <= 5);
        }
    }
}
