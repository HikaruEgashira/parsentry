use anyhow::Result;
use serde::Deserialize;

use crate::threat_model::{AttackSurface, SurfaceKind, ThreatModel};

pub const THREAT_MODEL_SYSTEM_PROMPT: &str = r#"You are an attack surface enumerator. Given repository metadata, identify all concrete attack surfaces and generate a tree-sitter query for each to locate them in code.

Focus on listing:

For web applications:
- HTTP/gRPC/GraphQL endpoints (method + path)
- Database tables or collections accessed
- External service calls

For libraries:
- Public API functions/methods that accept untrusted input

For CLI tools:
- Command handlers that process user input
- File I/O operations

For Infrastructure-as-Code:
- Cloud resources with security implications (IAM, storage, network)

Rules:
- Be specific: "POST /api/users" not "user management endpoints"
- Include file paths where each surface is defined
- Each surface must have a valid tree-sitter S-expression query to locate it
- Use capture names (e.g. @func, @call) in queries
- Use #eq? or #match? predicates to narrow matches
- Quality over quantity — only list surfaces that warrant security review"#;

pub fn build_threat_model_prompt(repo_context: &str, languages: &[String]) -> String {
    format!(
        r#"Enumerate the attack surfaces of this repository.

{repo_context}

Languages present: {languages}

For each attack surface:
1. Identify the specific surface (endpoint, table, public function, etc.)
2. List the file(s) where it is defined or used
3. Briefly explain why it warrants security review

Return a JSON object with this structure:
{{
  "app_type": "web_application|library|cli|infrastructure|mixed",
  "summary": "High-level security posture summary (2-3 sentences)",
  "surfaces": [
    {{
      "id": "SURFACE-001",
      "kind": "endpoint|db_table|public_api|file_handler|external_call|iac_resource",
      "identifier": "POST /api/users",
      "locations": ["src/routes/users.py", "src/models/user.py"],
      "description": "User registration endpoint accepting untrusted input",
      "query": "(decorator (call function: (attribute attribute: (identifier) @method (#match? @method \"post|put|delete\")))) @route"
    }}
  ]
}}"#,
        repo_context = repo_context,
        languages = languages.join(", "),
    )
}

pub fn threat_model_schema() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "app_type": {"type": "string", "enum": ["web_application", "library", "cli", "infrastructure", "mixed"]},
            "summary": {"type": "string"},
            "surfaces": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string"},
                        "kind": {"type": "string", "enum": ["endpoint", "db_table", "public_api", "file_handler", "external_call", "iac_resource"]},
                        "identifier": {"type": "string"},
                        "locations": {"type": "array", "items": {"type": "string"}},
                        "description": {"type": "string"},
                        "query": {"type": "string"}
                    },
                    "required": ["id", "kind", "identifier", "locations", "description", "query"]
                }
            }
        },
        "required": ["app_type", "summary", "surfaces"]
    })
}

#[derive(Deserialize)]
struct LlmResponse {
    app_type: String,
    summary: String,
    surfaces: Vec<LlmSurface>,
}

#[derive(Deserialize)]
struct LlmSurface {
    id: String,
    kind: String,
    identifier: String,
    locations: Vec<String>,
    description: String,
    query: String,
}

fn parse_surface_kind(s: &str) -> SurfaceKind {
    match s.to_lowercase().as_str() {
        "endpoint" => SurfaceKind::Endpoint,
        "db_table" => SurfaceKind::DbTable,
        "public_api" => SurfaceKind::PublicApi,
        "file_handler" => SurfaceKind::FileHandler,
        "external_call" => SurfaceKind::ExternalCall,
        "iac_resource" => SurfaceKind::IacResource,
        _ => SurfaceKind::PublicApi, // fallback
    }
}

pub fn parse_threat_model_response(json_str: &str, repository: &str) -> Result<ThreatModel> {
    let resp: LlmResponse = serde_json::from_str(json_str)
        .map_err(|e| anyhow::anyhow!("Failed to parse threat model response: {}. Content: {}", e, json_str))?;

    let surfaces: Vec<AttackSurface> = resp
        .surfaces
        .into_iter()
        .map(|s| AttackSurface {
            id: s.id,
            kind: parse_surface_kind(&s.kind),
            identifier: s.identifier,
            locations: s.locations,
            description: s.description,
            query: s.query,
        })
        .collect();

    Ok(ThreatModel {
        repository: repository.to_string(),
        generated_at: chrono::Utc::now().to_rfc3339(),
        app_type: resp.app_type,
        summary: resp.summary,
        surfaces,
    })
}
