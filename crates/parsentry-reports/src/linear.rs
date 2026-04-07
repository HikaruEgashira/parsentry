/// Linear integration for parsentry findings.
///
/// Required env vars:
///   LINEAR_API_KEY  - API key from Linear Settings > API
///
/// `team_id` argument: Linear team ID (UUID) or team key (e.g. "ENG").
///
/// Structure: one parent issue per surface, one child issue per finding.
/// Deduplication: fingerprint / surface name stored in description as HTML comment.
/// - New findings → create Linear issue with label "parsentry" and parentId set
/// - `baselineState == "absent"` → cancel existing issue
/// - `baselineState == "unchanged"` or fingerprint already exists → skip
use anyhow::{Result, anyhow};
use reqwest::Client;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::env;
use std::path::Path;

use crate::report_common::{
    SURFACE_MARKER, build_markdown_body, build_title, extract_fingerprint, load_surface_reports,
    parse_fingerprint_from_body, parse_surface_from_body,
};

const LINEAR_API: &str = "https://api.linear.app/graphql";
const LINEAR_LABEL: &str = "parsentry";

pub async fn run_linear_command(
    reports_dir: &Path,
    team_id: &str,
    dry_run: bool,
    min_level: &str,
) -> Result<()> {
    let api_key = env::var("LINEAR_API_KEY").map_err(|_| anyhow!("LINEAR_API_KEY not set"))?;

    let client = Client::new();

    let surfaces = load_surface_reports(reports_dir, min_level)?;
    if surfaces.is_empty() {
        eprintln!("No findings to report (level >= {min_level}).");
        return Ok(());
    }

    // Resolve team UUID (accepts both UUID and key like "ENG")
    let team_uuid = resolve_team_id(&client, &api_key, team_id).await?;

    // Ensure "parsentry" label exists and get its ID
    let label_id = ensure_label(&client, &api_key, &team_uuid).await?;

    // Fetch existing issues: build fp → id and surface → id maps.
    let (fp_map, surface_issue_map) = fetch_existing_issues(&client, &api_key, &team_uuid).await?;
    eprintln!(
        "Found {} existing child issue(s) and {} surface issue(s) in Linear team {team_id}.",
        fp_map.len(),
        surface_issue_map.len()
    );

    let (mut created, mut skipped, mut closed) = (0usize, 0usize, 0usize);
    let mut fp_map = fp_map;
    let mut surface_issue_map = surface_issue_map;

    for surface in &surfaces {
        let surface_title = format!("[Parsentry] {}", surface.surface_name);

        // Ensure parent issue exists.
        let parent_id = if let Some(id) = surface_issue_map.get(&surface.surface_name) {
            id.clone()
        } else if dry_run {
            eprintln!("[dry-run] Would create surface issue: {surface_title}");
            String::new()
        } else {
            let parent_desc = format!(
                "This issue tracks all parsentry findings for surface: {}.\n\n{SURFACE_MARKER} {} -->",
                surface.surface_name, surface.surface_name
            );
            let url = create_issue(
                &client,
                &api_key,
                &team_uuid,
                &label_id,
                &surface_title,
                &parent_desc,
                None,
            )
            .await?;
            eprintln!("Created surface issue: {url}");
            // Fetch the id for the newly created issue by re-querying
            let id = fetch_issue_id_by_title(&client, &api_key, &team_uuid, &surface_title).await?;
            surface_issue_map.insert(surface.surface_name.clone(), id.clone());
            id
        };

        for result in &surface.results {
            let fp = extract_fingerprint(result);

            if result.baseline_state.as_deref() == Some("absent") {
                if let Some(issue_id) = fp.as_ref().and_then(|f| fp_map.get(f)) {
                    if dry_run {
                        eprintln!(
                            "[dry-run] Would cancel issue {issue_id} (absent: {})",
                            result.rule_id
                        );
                    } else {
                        cancel_issue(&client, &api_key, issue_id).await?;
                        eprintln!("Cancelled issue {issue_id} (resolved: {})", result.rule_id);
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
                eprintln!("[dry-run] Would create: {title}");
            } else {
                let parent = if parent_id.is_empty() {
                    None
                } else {
                    Some(parent_id.as_str())
                };
                let url = create_issue(
                    &client, &api_key, &team_uuid, &label_id, &title, &body, parent,
                )
                .await?;
                eprintln!("Created: {url}");
                if let Some(f) = fp {
                    // Fetch id for dedup on next run
                    if let Ok(id) =
                        fetch_issue_id_by_title(&client, &api_key, &team_uuid, &title).await
                    {
                        fp_map.insert(f, id);
                    }
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

async fn gql(client: &Client, api_key: &str, query: &str, variables: Value) -> Result<Value> {
    let resp = client
        .post(LINEAR_API)
        .bearer_auth(api_key)
        .json(&json!({ "query": query, "variables": variables }))
        .send()
        .await
        .map_err(|e| anyhow!("Linear API request failed: {e}"))?;

    let status = resp.status();
    let body: Value = resp
        .json()
        .await
        .map_err(|e| anyhow!("Linear response parse failed: {e}"))?;

    if !status.is_success() {
        anyhow::bail!("Linear API error ({}): {}", status, body);
    }
    if let Some(errors) = body["errors"].as_array() {
        anyhow::bail!("Linear GraphQL errors: {}", serde_json::to_string(errors)?);
    }
    Ok(body["data"].clone())
}

/// Resolve team key (e.g. "ENG") or UUID to UUID.
async fn resolve_team_id(client: &Client, api_key: &str, team_id: &str) -> Result<String> {
    if team_id.len() == 36 && team_id.contains('-') {
        return Ok(team_id.to_string());
    }

    let data = gql(
        client,
        api_key,
        r#"query($key: String!) { team(id: $key) { id } }"#,
        json!({ "key": team_id }),
    )
    .await?;

    data["team"]["id"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow!("Team '{}' not found in Linear", team_id))
}

/// Ensure the "parsentry" label exists in the team, create if missing. Returns label ID.
async fn ensure_label(client: &Client, api_key: &str, team_id: &str) -> Result<String> {
    let data = gql(
        client,
        api_key,
        r#"query($teamId: String!, $name: String!) {
            issueLabels(filter: { team: { id: { eq: $teamId } }, name: { eq: $name } }) {
                nodes { id }
            }
        }"#,
        json!({ "teamId": team_id, "name": LINEAR_LABEL }),
    )
    .await?;

    if let Some(id) = data["issueLabels"]["nodes"][0]["id"].as_str() {
        return Ok(id.to_string());
    }

    let data = gql(
        client,
        api_key,
        r#"mutation($teamId: String!, $name: String!, $color: String!) {
            issueLabelCreate(input: { teamId: $teamId, name: $name, color: $color }) {
                issueLabel { id }
            }
        }"#,
        json!({ "teamId": team_id, "name": LINEAR_LABEL, "color": "#e11d48" }),
    )
    .await?;

    data["issueLabelCreate"]["issueLabel"]["id"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow!("Failed to create Linear label '{LINEAR_LABEL}'"))
}

/// Fetch open parsentry issues.
/// Returns (fingerprint → issue ID, surface_name → issue ID).
async fn fetch_existing_issues(
    client: &Client,
    api_key: &str,
    team_id: &str,
) -> Result<(HashMap<String, String>, HashMap<String, String>)> {
    let mut fp_map = HashMap::new();
    let mut surface_map = HashMap::new();
    let mut cursor: Option<String> = None;

    loop {
        let after_clause = cursor
            .as_deref()
            .map(|c| format!(r#", after: "{}""#, c))
            .unwrap_or_default();

        let query = format!(
            r#"query($teamId: String!, $label: String!) {{
                issues(
                    filter: {{
                        team: {{ id: {{ eq: $teamId }} }}
                        labels: {{ name: {{ eq: $label }} }}
                        state: {{ type: {{ nin: ["completed", "cancelled"] }} }}
                    }}
                    first: 100{after_clause}
                ) {{
                    nodes {{ id description }}
                    pageInfo {{ hasNextPage endCursor }}
                }}
            }}"#
        );

        let data = gql(
            client,
            api_key,
            &query,
            json!({ "teamId": team_id, "label": LINEAR_LABEL }),
        )
        .await?;

        let nodes = data["issues"]["nodes"]
            .as_array()
            .cloned()
            .unwrap_or_default();
        for node in &nodes {
            let id = node["id"].as_str().unwrap_or("").to_string();
            let desc = node["description"].as_str().unwrap_or("");
            if let Some(fp) = parse_fingerprint_from_body(desc) {
                fp_map.insert(fp, id.clone());
            }
            if let Some(surface) = parse_surface_from_body(desc) {
                surface_map.insert(surface, id);
            }
        }

        let page_info = &data["issues"]["pageInfo"];
        if page_info["hasNextPage"].as_bool() != Some(true) {
            break;
        }
        cursor = page_info["endCursor"].as_str().map(|s| s.to_string());
    }

    Ok((fp_map, surface_map))
}

async fn create_issue(
    client: &Client,
    api_key: &str,
    team_id: &str,
    label_id: &str,
    title: &str,
    body: &str,
    parent_id: Option<&str>,
) -> Result<String> {
    let mut input = json!({
        "teamId": team_id,
        "title": title,
        "description": body,
        "labelIds": [label_id],
    });

    if let Some(pid) = parent_id {
        input["parentId"] = json!(pid);
    }

    let data = gql(
        client,
        api_key,
        r#"mutation($input: IssueCreateInput!) {
            issueCreate(input: $input) {
                issue { url }
            }
        }"#,
        json!({ "input": input }),
    )
    .await?;

    data["issueCreate"]["issue"]["url"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow!("Linear issueCreate returned no URL"))
}

/// Fetch the issue ID for a recently created issue by exact title match.
async fn fetch_issue_id_by_title(
    client: &Client,
    api_key: &str,
    team_id: &str,
    title: &str,
) -> Result<String> {
    let data = gql(
        client,
        api_key,
        r#"query($teamId: String!, $title: String!) {
            issues(filter: {
                team: { id: { eq: $teamId } }
                title: { eq: $title }
            }, first: 1) {
                nodes { id }
            }
        }"#,
        json!({ "teamId": team_id, "title": title }),
    )
    .await?;

    data["issues"]["nodes"][0]["id"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| anyhow!("Could not find issue with title '{title}'"))
}

async fn cancel_issue(client: &Client, api_key: &str, issue_id: &str) -> Result<()> {
    let data = gql(
        client,
        api_key,
        r#"query($id: String!) {
            issue(id: $id) {
                team {
                    states(filter: { type: { eq: "cancelled" } }) {
                        nodes { id }
                    }
                }
            }
        }"#,
        json!({ "id": issue_id }),
    )
    .await?;

    let state_id = data["issue"]["team"]["states"]["nodes"][0]["id"]
        .as_str()
        .ok_or_else(|| anyhow!("No cancelled state found for issue {issue_id}"))?
        .to_string();

    gql(
        client,
        api_key,
        r#"mutation($id: String!, $stateId: String!) {
            issueUpdate(id: $id, input: { stateId: $stateId }) {
                issue { id }
            }
        }"#,
        json!({ "id": issue_id, "stateId": state_id }),
    )
    .await?;

    Ok(())
}
