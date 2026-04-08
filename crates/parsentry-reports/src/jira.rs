/// Jira integration for parsentry findings.
///
/// Required env vars:
///   JIRA_URL        - e.g. https://company.atlassian.net
///   JIRA_EMAIL      - Atlassian account email
///   JIRA_API_TOKEN  - API token from id.atlassian.com/manage-profile/security/api-tokens
///
/// Structure: one Story per surface, one Sub-task per finding.
/// Deduplication: fingerprint / surface name stored in description as HTML comment.
/// - New findings → create Sub-task under the surface Story
/// - `baselineState == "absent"` → transition existing Sub-task to done
/// - `baselineState == "unchanged"` or fingerprint already open → skip
use anyhow::{Result, anyhow};
use reqwest::{Client, StatusCode};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::env;
use std::path::Path;

use crate::report_common::{
    SURFACE_MARKER, build_markdown_body, build_title, extract_fingerprint, load_surface_reports,
    parse_fingerprint_from_body, parse_surface_from_body,
};

const JIRA_LABEL: &str = "parsentry";

pub async fn run_jira_command(
    reports_dir: &Path,
    project_key: &str,
    dry_run: bool,
    min_level: &str,
) -> Result<()> {
    let base_url = env::var("JIRA_URL")
        .map_err(|_| anyhow!("JIRA_URL not set (e.g. https://company.atlassian.net)"))?
        .trim_end_matches('/')
        .to_string();
    let email = env::var("JIRA_EMAIL").map_err(|_| anyhow!("JIRA_EMAIL not set"))?;
    let token = env::var("JIRA_API_TOKEN").map_err(|_| anyhow!("JIRA_API_TOKEN not set"))?;

    let client = Client::new();
    let auth = JiraAuth {
        base_url,
        email,
        token,
    };

    let surfaces = load_surface_reports(reports_dir, min_level)?;
    if surfaces.is_empty() {
        eprintln!("No findings to report (level >= {min_level}).");
        return Ok(());
    }

    // Fetch existing issues once: build fp → key and surface → key maps.
    let (fp_map, surface_issue_map) = fetch_existing_issues(&client, &auth, project_key).await?;
    eprintln!(
        "Found {} existing child issue(s) and {} surface issue(s) in Jira project {project_key}.",
        fp_map.len(),
        surface_issue_map.len()
    );

    let (mut created, mut skipped, mut closed) = (0usize, 0usize, 0usize);
    let mut fp_map = fp_map;
    let mut surface_issue_map = surface_issue_map;

    for surface in &surfaces {
        let surface_title = format!("[Parsentry] {}", surface.surface_name);

        // Ensure parent Story exists.
        let parent_key = if let Some(key) = surface_issue_map.get(&surface.surface_name) {
            key.clone()
        } else if dry_run {
            eprintln!("[dry-run] Would create surface Story: {surface_title}");
            String::new()
        } else {
            let desc = format!(
                "This story tracks all parsentry findings for surface: {}.\n\n{SURFACE_MARKER} {} -->",
                surface.surface_name, surface.surface_name
            );
            let url = create_issue_with_type(
                &client,
                &auth,
                project_key,
                &surface_title,
                &desc,
                "Story",
                None,
            )
            .await?;
            eprintln!("Created surface Story: {url}");
            // Extract key from URL: last path segment after /browse/
            let key = url.rsplit('/').next().unwrap_or("?").to_string();
            surface_issue_map.insert(surface.surface_name.clone(), key.clone());
            key
        };

        for result in &surface.results {
            let fp = extract_fingerprint(result);

            if result.baseline_state.as_deref() == Some("absent") {
                if let Some(issue_key) = fp.as_ref().and_then(|f| fp_map.get(f)) {
                    if dry_run {
                        eprintln!(
                            "[dry-run] Would close {issue_key} (absent: {})",
                            result.rule_id
                        );
                    } else {
                        close_issue(&client, &auth, issue_key).await?;
                        eprintln!("Closed {issue_key} (resolved: {})", result.rule_id);
                    }
                    closed += 1;
                }
                continue;
            }

            if result.baseline_state.as_deref() == Some("unchanged") {
                skipped += 1;
                continue;
            }
            if let Some(f) = &fp {
                if fp_map.contains_key(f) {
                    skipped += 1;
                    continue;
                }
            }

            let title = build_title(result);
            let body = build_markdown_body(result, fp.as_deref());

            if dry_run {
                eprintln!("[dry-run] Would create Sub-task: {title}");
            } else {
                let parent = if parent_key.is_empty() {
                    None
                } else {
                    Some(parent_key.as_str())
                };
                let url = create_issue_with_type(
                    &client,
                    &auth,
                    project_key,
                    &title,
                    &body,
                    "Sub-task",
                    parent,
                )
                .await?;
                eprintln!("Created: {url}");
                if let Some(f) = fp {
                    let key = url.rsplit('/').next().unwrap_or("?").to_string();
                    fp_map.insert(f, key);
                }
            }
            created += 1;
        }
    }

    eprintln!(
        "Done. created={created}, skipped={skipped}, closed={closed}{}",
        if dry_run { " (dry-run)" } else { "" }
    );
    Ok(())
}

struct JiraAuth {
    base_url: String,
    email: String,
    token: String,
}

impl JiraAuth {
    fn basic_auth(&self) -> (String, String) {
        (self.email.clone(), self.token.clone())
    }
}

/// Fetch all open parsentry issues in the project.
/// Returns (fingerprint → issue_key, surface_name → issue_key).
async fn fetch_existing_issues(
    client: &Client,
    auth: &JiraAuth,
    project_key: &str,
) -> Result<(HashMap<String, String>, HashMap<String, String>)> {
    let mut fp_map = HashMap::new();
    let mut surface_map = HashMap::new();
    let mut start = 0usize;
    let max = 100usize;

    // Validate project_key format to prevent JQL injection
    if !project_key.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        anyhow::bail!("Invalid Jira project key: {}", project_key);
    }

    loop {
        let jql = format!(
            r#"project = "{project_key}" AND labels = "{JIRA_LABEL}" AND statusCategory != Done ORDER BY created DESC"#
        );
        let (email, token) = auth.basic_auth();
        let resp: Value = client
            .get(format!("{}/rest/api/3/search", auth.base_url))
            .basic_auth(&email, Some(&token))
            .query(&[
                ("jql", jql.as_str()),
                ("startAt", &start.to_string()),
                ("maxResults", &max.to_string()),
                ("fields", "description,summary"),
            ])
            .send()
            .await
            .map_err(|e| anyhow!("Jira search failed: {e}"))?
            .json()
            .await
            .map_err(|e| anyhow!("Jira search parse failed: {e}"))?;

        let issues = resp["issues"].as_array().cloned().unwrap_or_default();
        if issues.is_empty() {
            break;
        }

        for issue in &issues {
            let key = issue["key"].as_str().unwrap_or("").to_string();
            let desc = extract_adf_text(&issue["fields"]["description"]);
            if let Some(fp) = parse_fingerprint_from_body(&desc) {
                fp_map.insert(fp, key.clone());
            }
            if let Some(surface) = parse_surface_from_body(&desc) {
                surface_map.insert(surface, key);
            }
        }

        start += issues.len();
        let total = resp["total"].as_u64().unwrap_or(0) as usize;
        if start >= total {
            break;
        }
    }

    Ok((fp_map, surface_map))
}

/// Recursively extract plain text from Atlassian Document Format (ADF) JSON.
fn extract_adf_text(node: &Value) -> String {
    if node.is_null() {
        return String::new();
    }
    if let Some(text) = node["text"].as_str() {
        return text.to_string();
    }
    let mut out = String::new();
    if let Some(content) = node["content"].as_array() {
        for child in content {
            out.push_str(&extract_adf_text(child));
            out.push('\n');
        }
    }
    out
}

async fn create_issue_with_type(
    client: &Client,
    auth: &JiraAuth,
    project_key: &str,
    title: &str,
    body: &str,
    issue_type: &str,
    parent_key: Option<&str>,
) -> Result<String> {
    let (email, token) = auth.basic_auth();
    let mut fields = json!({
        "project": { "key": project_key },
        "summary": title,
        "description": {
            "type": "doc",
            "version": 1,
            "content": [{
                "type": "paragraph",
                "content": [{ "type": "text", "text": body }]
            }]
        },
        "issuetype": { "name": issue_type },
        "labels": [JIRA_LABEL]
    });

    if let Some(pk) = parent_key {
        fields["parent"] = json!({ "key": pk });
    }

    let payload = json!({ "fields": fields });

    let resp = client
        .post(format!("{}/rest/api/3/issue", auth.base_url))
        .basic_auth(&email, Some(&token))
        .json(&payload)
        .send()
        .await
        .map_err(|e| anyhow!("Jira create issue failed: {e}"))?;

    let status = resp.status();
    let body: Value = resp
        .json()
        .await
        .map_err(|e| anyhow!("Jira response parse failed: {e}"))?;

    if !status.is_success() {
        anyhow::bail!("Jira create issue failed ({}): {}", status, body);
    }

    let key = body["key"].as_str().unwrap_or("?");
    Ok(format!("{}/browse/{key}", auth.base_url))
}

async fn close_issue(client: &Client, auth: &JiraAuth, issue_key: &str) -> Result<()> {
    let (email, token) = auth.basic_auth();

    // Fetch available transitions
    let transitions: Value = client
        .get(format!(
            "{}/rest/api/3/issue/{issue_key}/transitions",
            auth.base_url
        ))
        .basic_auth(&email, Some(&token))
        .send()
        .await
        .map_err(|e| anyhow!("Failed to fetch transitions for {issue_key}: {e}"))?
        .json()
        .await
        .map_err(|e| anyhow!("Transition parse failed: {e}"))?;

    // Find first transition whose statusCategory is "Done"
    let transition_id = transitions["transitions"]
        .as_array()
        .and_then(|ts| {
            ts.iter()
                .find(|t| t["to"]["statusCategory"]["key"].as_str() == Some("done"))
        })
        .and_then(|t| t["id"].as_str())
        .ok_or_else(|| anyhow!("No 'done' transition found for {issue_key}"))?
        .to_string();

    let status = client
        .post(format!(
            "{}/rest/api/3/issue/{issue_key}/transitions",
            auth.base_url
        ))
        .basic_auth(&email, Some(&token))
        .json(&json!({ "transition": { "id": transition_id } }))
        .send()
        .await
        .map_err(|e| anyhow!("Failed to transition {issue_key}: {e}"))?
        .status();

    if !status.is_success() && status != StatusCode::NO_CONTENT {
        anyhow::bail!("Jira transition failed ({})", status);
    }
    Ok(())
}
