use anyhow::{anyhow, Result};
use octocrab::Octocrab;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::Write;
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
        // Security: Verify git binary path to prevent PATH manipulation attacks
        let git_path = std::process::Command::new("which")
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
