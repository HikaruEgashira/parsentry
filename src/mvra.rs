use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use parsentry_reports::AnalysisSummary;
use crate::github::{GitHubSearchClient, SearchResult};

/// Multi-repository variant analysis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MvraConfig {
    /// GitHub search query for repositories
    pub search_query: Option<String>,

    /// Code search query (for finding specific code patterns)
    pub code_query: Option<String>,

    /// Maximum number of repositories to analyze
    pub max_repos: usize,

    /// Directory to cache cloned repositories
    pub cache_dir: PathBuf,

    /// Whether to skip repositories that already exist in cache
    pub use_cache: bool,

    /// Minimum stars for repository filtering
    pub min_stars: Option<u32>,

    /// List of specific repositories to analyze (owner/repo format)
    pub repositories: Option<Vec<String>>,
}

impl Default for MvraConfig {
    fn default() -> Self {
        Self {
            search_query: None,
            code_query: None,
            max_repos: 10,
            cache_dir: PathBuf::from(".parsentry-cache"),
            use_cache: true,
            min_stars: None,
            repositories: None,
        }
    }
}

/// Result of analyzing a single repository in MVRA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MvraRepositoryResult {
    pub owner: String,
    pub repo: String,
    pub full_name: String,
    pub html_url: String,
    pub stars: u32,
    pub description: Option<String>,
    pub local_path: PathBuf,
    pub analysis: AnalysisSummary,
    pub scan_error: Option<String>,
}

/// Aggregated results from multi-repository variant analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MvraResults {
    pub repositories: Vec<MvraRepositoryResult>,
    pub total_repositories: usize,
    pub successful_scans: usize,
    pub failed_scans: usize,
    pub variant_patterns: Vec<VariantPattern>,
}

/// A security pattern variant found across multiple repositories
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantPattern {
    pub vulnerability_type: String,
    pub description: String,
    pub occurrences: Vec<PatternOccurrence>,
    pub repository_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternOccurrence {
    pub repository: String,
    pub file_path: String,
    pub confidence_score: i32,
    pub matched_code: Option<String>,
}

/// MVRA Scanner - Orchestrates multi-repository variant analysis
pub struct MvraScanner {
    config: MvraConfig,
    github_client: Option<GitHubSearchClient>,
}

impl MvraScanner {
    pub fn new(config: MvraConfig) -> Result<Self> {
        // Only create GitHub client if we need to search
        let github_client = if config.search_query.is_some() || config.code_query.is_some() {
            Some(GitHubSearchClient::new()?)
        } else {
            None
        };

        Ok(Self {
            config,
            github_client,
        })
    }

    /// Get list of repositories to analyze
    pub async fn get_target_repositories(&self) -> Result<Vec<SearchResult>> {
        let mut repos = Vec::new();

        // First, add explicitly specified repositories
        if let Some(ref repo_list) = self.config.repositories {
            for repo_str in repo_list {
                let parts: Vec<&str> = repo_str.split('/').collect();
                if parts.len() != 2 {
                    return Err(anyhow!("Invalid repository format: {}. Expected 'owner/repo'", repo_str));
                }

                repos.push(SearchResult {
                    owner: parts[0].to_string(),
                    repo: parts[1].to_string(),
                    full_name: repo_str.clone(),
                    clone_url: format!("https://github.com/{}.git", repo_str),
                    html_url: format!("https://github.com/{}", repo_str),
                    stars: 0,
                    description: None,
                });
            }
        }

        // Then, search for repositories if query is provided
        if let Some(ref query) = self.config.search_query {
            let client = self.github_client.as_ref()
                .ok_or_else(|| anyhow!("GitHub client not initialized"))?;

            let remaining = self.config.max_repos.saturating_sub(repos.len());
            let mut search_results = client.search_repositories(query, remaining).await?;

            // Filter by minimum stars if specified
            if let Some(min_stars) = self.config.min_stars {
                search_results.retain(|r| r.stars >= min_stars);
            }

            repos.extend(search_results);
        }

        // Use code search to find repositories with specific code patterns
        if let Some(ref code_query) = self.config.code_query {
            let client = self.github_client.as_ref()
                .ok_or_else(|| anyhow!("GitHub client not initialized"))?;

            let code_results = client.search_code(code_query, self.config.max_repos).await?;

            // Extract unique repositories from code search results
            let mut repo_map: HashMap<String, SearchResult> = HashMap::new();
            for code_result in code_results {
                let full_name = code_result.repository.full_name.clone();
                if !repo_map.contains_key(&full_name) && repo_map.len() < self.config.max_repos {
                    repo_map.insert(full_name.clone(), SearchResult {
                        owner: code_result.repository.owner.login.clone(),
                        repo: code_result.repository.name.clone(),
                        full_name: full_name.clone(),
                        clone_url: format!("https://github.com/{}.git", full_name),
                        html_url: code_result.repository.html_url.clone(),
                        stars: 0,
                        description: code_result.repository.description.clone(),
                    });
                }
            }

            repos.extend(repo_map.into_values());
        }

        // Limit to max_repos
        repos.truncate(self.config.max_repos);

        if repos.is_empty() {
            return Err(anyhow!("No repositories found to analyze"));
        }

        Ok(repos)
    }

    /// Clone or get cached repository
    pub fn get_repository(&self, repo: &SearchResult) -> Result<PathBuf> {
        let repo_path = self.config.cache_dir.join(&repo.owner).join(&repo.repo);

        if self.config.use_cache && repo_path.exists() {
            println!("✓ Using cached repository: {}", repo.full_name);
            return Ok(repo_path);
        }

        // Ensure parent directory exists
        if let Some(parent) = repo_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Clone the repository
        println!("⬇ Cloning repository: {}", repo.full_name);
        crate::repo::clone_github_repo(&repo.full_name, &repo_path)?;

        Ok(repo_path)
    }

    /// Analyze variant patterns across all repositories
    pub fn analyze_variants(&self, results: &[MvraRepositoryResult]) -> Vec<VariantPattern> {
        let mut variant_map: HashMap<String, VariantPattern> = HashMap::new();

        for repo_result in results {
            if repo_result.scan_error.is_some() {
                continue; // Skip failed scans
            }

            for file_result in &repo_result.analysis.results {
                let response = &file_result.response;
                for vuln_type in &response.vulnerability_types {
                    let key = format!("{:?}", vuln_type);

                    variant_map.entry(key.clone())
                        .and_modify(|pattern| {
                            pattern.occurrences.push(PatternOccurrence {
                                repository: repo_result.full_name.clone(),
                                file_path: response.file_path.clone().unwrap_or_default(),
                                confidence_score: response.confidence_score,
                                matched_code: response.matched_source_code.clone(),
                            });
                            pattern.repository_count = pattern.occurrences
                                .iter()
                                .map(|o| o.repository.as_str())
                                .collect::<std::collections::HashSet<_>>()
                                .len();
                        })
                        .or_insert_with(|| {
                            VariantPattern {
                                vulnerability_type: format!("{:?}", vuln_type),
                                description: response.pattern_description.clone().unwrap_or_default(),
                                occurrences: vec![PatternOccurrence {
                                    repository: repo_result.full_name.clone(),
                                    file_path: response.file_path.clone().unwrap_or_default(),
                                    confidence_score: response.confidence_score,
                                    matched_code: response.matched_source_code.clone(),
                                }],
                                repository_count: 1,
                            }
                        });
                }
            }
        }

        let mut variants: Vec<VariantPattern> = variant_map.into_values().collect();

        // Sort by repository count (most widespread first)
        variants.sort_by(|a, b| b.repository_count.cmp(&a.repository_count));

        variants
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mvra_config_default() {
        let config = MvraConfig::default();
        assert_eq!(config.max_repos, 10);
        assert!(config.use_cache);
        assert_eq!(config.cache_dir, PathBuf::from(".parsentry-cache"));
    }

    #[test]
    fn test_variant_pattern_aggregation() {
        // This would test the analyze_variants logic
        // Requires mock data setup
    }
}
