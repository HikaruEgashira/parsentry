/// Notion integration for parsentry findings.
///
/// Required env vars:
///   NOTION_TOKEN        - Internal integration token from notion.so/my-integrations
///   NOTION_DATABASE_ID  - Target database ID (or pass as `database_id` argument)
///
/// The database must have these properties (create them once manually):
///   - Name        : title
///   - Status      : status  (with "Done" option)
///   - Severity    : select
///   - Type        : select  (values: "surface" | "finding")
///   - Surface     : select  (surface name for finding pages)
///   - Fingerprint : rich_text  ← used for deduplication
///
/// Structure: one "surface" page per surface, finding pages tagged with the surface name.
/// - New surface → create page with Type="surface", Fingerprint="surface:{name}"
/// - New findings → create page with Type="finding", Surface=surface_name
/// - `baselineState == "absent"` → set Status to "Done"
/// - `baselineState == "unchanged"` or Fingerprint already exists → skip
use anyhow::{Result, anyhow};
use reqwest::Client;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::env;
use std::path::Path;

use crate::report_common::{
    build_markdown_body, build_title, extract_fingerprint, load_surface_reports,
};
use crate::sarif::SarifResult;

const NOTION_API: &str = "https://api.notion.com/v1";
const NOTION_VERSION: &str = "2022-06-28";

// Validate Notion page/database IDs: 32 hex chars, with or without dashes
fn validate_notion_id(id: &str) -> bool {
    let compact: String = id.chars().filter(|&c| c != '-').collect();
    compact.len() == 32 && compact.chars().all(|c| c.is_digit(16))
}

pub async fn run_notion_command(
    reports_dir: &Path,
    database_id: &str,
    dry_run: bool,
    min_level: &str,
) -> Result<()> {
    let token = env::var("NOTION_TOKEN").map_err(|_| anyhow!("NOTION_TOKEN not set"))?;
    let db_id = env::var("NOTION_DATABASE_ID").unwrap_or_else(|_| database_id.to_string());

    // Basic validation for Notion IDs before using them in requests
    if !validate_notion_id(&db_id) {
        return Err(anyhow!("Invalid Notion database id format"));
    }

    let client = Client::new();

    let surfaces = load_surface_reports(reports_dir, min_level)?;
    if surfaces.is_empty() {
        eprintln!("No findings to report (level >= {min_level}).");
        return Ok(());
    }

    // Fetch all existing pages: build fp → page_id and surface → page_id maps.
    let (fp_map, surface_page_map) = fetch_existing_pages(&client, &token, &db_id).await?;
    eprintln!(
        "Found {} existing finding page(s) and {} surface page(s) in Notion database.",
        fp_map.len(),
        surface_page_map.len()
    );

    let (mut created, mut skipped, mut closed) = (0usize, 0usize, 0usize);
    let mut fp_map = fp_map;
    let mut surface_page_map = surface_page_map;

    for surface in &surfaces {
        // Ensure surface page exists.
        let surface_fp = format!("surface:{}", surface.surface_name);
        let _surface_page_id = if let Some(id) = surface_page_map.get(&surface.surface_name) {
            id.clone()
        } else if dry_run {
            eprintln!(
                "[dry-run] Would create surface page: {}",
                surface.surface_name
            );
            String::new()
        } else {
            let page_id =
                create_surface_page(&client, &token, &db_id, &surface.surface_name, &surface_fp)
                    .await?;
            eprintln!("Created surface page: {page_id}");
            surface_page_map.insert(surface.surface_name.clone(), page_id.clone());
            page_id
        };

        for result in &surface.results {
            let fp = extract_fingerprint(result);

            if result.baseline_state.as_deref() == Some("absent") {
                if let Some(page_id) = fp.as_ref().and_then(|f| fp_map.get(f)) {
                    if dry_run {
                        eprintln!(
                            "[dry-run] Would archive page {page_id} (absent: {})",
                            result.rule_id
                        );
                    } else {
                        mark_done(&client, &token, page_id).await?;
                        eprintln!("Marked done: page {page_id} (resolved: {})", result.rule_id);
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
                let url = create_finding_page(
                    &client,
                    &token,
                    &db_id,
                    result,
                    &title,
                    &body,
                    fp.as_deref(),
                    &surface.surface_name,
                )
                .await?;
                eprintln!("Created: {url}");
                if let Some(f) = &fp {
                    // Extract page_id from URL for dedup map
                    let pid = url.rsplit('/').next().unwrap_or("?").replace('-', "");
                    fp_map.insert(f.clone(), pid);
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

/// Query database for all pages.
/// Returns (fingerprint → page_id, surface_name → page_id).
async fn fetch_existing_pages(
    client: &Client,
    token: &str,
    db_id: &str,
) -> Result<(HashMap<String, String>, HashMap<String, String>)> {
    let mut fp_map = HashMap::new();
    let mut surface_map = HashMap::new();
    let mut cursor: Option<String> = None;

    loop {
        let mut payload = json!({ "page_size": 100 });
        if let Some(c) = &cursor {
            payload["start_cursor"] = json!(c);
        }

        let resp: Value = client
            .post(format!("{NOTION_API}/databases/{db_id}/query"))
            .bearer_auth(token)
            .header("Notion-Version", NOTION_VERSION)
            .json(&payload)
            .send()
            .await
            .map_err(|e| anyhow!("Notion query failed: {e}"))?
            .json()
            .await
            .map_err(|e| anyhow!("Notion query parse failed: {e}"))?;

        check_notion_error(&resp)?;

        let results = resp["results"].as_array().cloned().unwrap_or_default();
        for page in &results {
            let page_id = page["id"].as_str().unwrap_or("").to_string();
            let fp_val = page["properties"]["Fingerprint"]["rich_text"][0]["plain_text"]
                .as_str()
                .map(|s| s.to_string());
            let page_type = page["properties"]["Type"]["select"]["name"]
                .as_str()
                .unwrap_or("");

            if let Some(fp) = fp_val {
                if !fp.is_empty() {
                    if page_type == "surface" {
                        // surface pages use "surface:{name}" as fingerprint
                        if let Some(name) = fp.strip_prefix("surface:") {
                            surface_map.insert(name.to_string(), page_id);
                        }
                    } else {
                        fp_map.insert(fp, page_id);
                    }
                }
            }
        }

        if resp["has_more"].as_bool() != Some(true) {
            break;
        }
        cursor = resp["next_cursor"].as_str().map(|s| s.to_string());
    }

    Ok((fp_map, surface_map))
}

async fn create_surface_page(
    client: &Client,
    token: &str,
    db_id: &str,
    surface_name: &str,
    fingerprint: &str,
) -> Result<String> {
    let payload = json!({
        "parent": { "database_id": db_id },
        "properties": {
            "Name": {
                "title": [{ "text": { "content": surface_name } }]
            },
            "Type": {
                "select": { "name": "surface" }
            },
            "Fingerprint": {
                "rich_text": [{ "text": { "content": fingerprint } }]
            }
        }
    });

    let resp: Value = client
        .post(format!("{NOTION_API}/pages"))
        .bearer_auth(token)
        .header("Notion-Version", NOTION_VERSION)
        .json(&payload)
        .send()
        .await
        .map_err(|e| anyhow!("Notion create surface page failed: {e}"))?
        .json()
        .await
        .map_err(|e| anyhow!("Notion create parse failed: {e}"))?;

    check_notion_error(&resp)?;

    let page_id = resp["id"].as_str().unwrap_or("?");
    Ok(page_id.to_string())
}

async fn create_finding_page(
    client: &Client,
    token: &str,
    db_id: &str,
    result: &SarifResult,
    title: &str,
    body: &str,
    fingerprint: Option<&str>,
    surface_name: &str,
) -> Result<String> {
    let mut properties = json!({
        "Name": {
            "title": [{ "text": { "content": title } }]
        },
        "Severity": {
            "select": { "name": capitalize(&result.level) }
        },
        "Type": {
            "select": { "name": "finding" }
        },
        "Surface": {
            "select": { "name": surface_name }
        }
    });

            if let Some(fp) = fingerprint {
        properties["Fingerprint"] = json!({
            "rich_text": [{ "text": { "content": fp } }]
        });
    }

    // Split body into paragraph blocks (max 2000 chars per block)
    let blocks: Vec<Value> = body
        .lines()
        .collect::<Vec<_>>()
        .chunks(20)
        .map(|chunk| {
            let text = chunk.join("\n");
            let text_cut: String = text.chars().take(2000).collect();
            json!({
                "object": "block",
                "type": "paragraph",
                "paragraph": {
                    "rich_text": [{ "type": "text", "text": { "content": text_cut } }]
                }
            })
        })
        .collect();

    let payload = json!({
        "parent": { "database_id": db_id },
        "properties": properties,
        "children": blocks
    });

    let resp: Value = client
        .post(format!("{NOTION_API}/pages"))
        .bearer_auth(token)
        .header("Notion-Version", NOTION_VERSION)
        .json(&payload)
        .send()
        .await
        .map_err(|e| anyhow!("Notion create page failed: {e}"))?
        .json()
        .await
        .map_err(|e| anyhow!("Notion create parse failed: {e}"))?;

    check_notion_error(&resp)?;

    let page_id = resp["id"].as_str().unwrap_or("?");
    Ok(format!("https://notion.so/{}", page_id.replace('-', "")))
}

async fn mark_done(client: &Client, token: &str, page_id: &str) -> Result<()> {
    if !validate_notion_id(page_id) {
        return Err(anyhow!("Invalid Notion page id"));
    }
    let payload = json!({
        "properties": {
            "Status": { "status": { "name": "Done" } }
        }
    });

    let resp: Value = client
        .patch(format!("{NOTION_API}/pages/{page_id}"))
        .bearer_auth(token)
        .header("Notion-Version", NOTION_VERSION)
        .json(&payload)
        .send()
        .await
        .map_err(|e| anyhow!("Notion update page failed: {e}"))?
        .json()
        .await
        .map_err(|e| anyhow!("Notion update parse failed: {e}"))?;

    check_notion_error(&resp)
}

fn check_notion_error(resp: &Value) -> Result<()> {
    if resp["object"].as_str() == Some("error") {
        anyhow::bail!(
            "Notion API error {}: {}",
            resp["status"].as_u64().unwrap_or(0),
            resp["message"].as_str().unwrap_or("unknown")
        );
    }
    Ok(())
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
