use anyhow::Result;
use serde::Deserialize;

use crate::threat_model::{AttackSurface, ThreatModel};

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
            "app_type": {"type": "string"},
            "summary": {"type": "string"},
            "surfaces": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string"},
                        "kind": {"type": "string"},
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

pub fn parse_threat_model_response(json_str: &str, repository: &str) -> Result<ThreatModel> {
    let resp: LlmResponse = serde_json::from_str(json_str)
        .map_err(|e| anyhow::anyhow!("Failed to parse threat model response: {}. Content: {}", e, json_str))?;

    let surfaces: Vec<AttackSurface> = resp
        .surfaces
        .into_iter()
        .map(|s| AttackSurface {
            id: s.id,
            kind: s.kind,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_threat_model_prompt_contains_repo_context() {
        let prompt = build_threat_model_prompt("## My Repo\nSome context", &["Python".to_string()]);
        assert!(prompt.contains("## My Repo"));
        assert!(prompt.contains("Some context"));
        assert!(prompt.contains("Python"));
        assert!(!prompt.is_empty());
    }

    #[test]
    fn test_build_threat_model_prompt_contains_languages() {
        let prompt = build_threat_model_prompt("ctx", &["Rust".to_string(), "Go".to_string()]);
        assert!(prompt.contains("Rust, Go"));
    }

    #[test]
    fn test_build_threat_model_prompt_contains_structure() {
        let prompt = build_threat_model_prompt("ctx", &[]);
        assert!(prompt.contains("app_type"));
        assert!(prompt.contains("surfaces"));
        assert!(prompt.contains("kind"));
    }

    #[test]
    fn test_threat_model_schema_structure() {
        let schema = threat_model_schema();
        assert!(schema.is_object());
        assert_eq!(schema["type"], "object");
        assert!(schema["properties"].get("app_type").is_some());
        assert!(schema["properties"].get("summary").is_some());
        assert!(schema["properties"].get("surfaces").is_some());
        let required = schema["required"].as_array().unwrap();
        assert!(required.iter().any(|v| v == "app_type"));
        assert!(required.iter().any(|v| v == "summary"));
        assert!(required.iter().any(|v| v == "surfaces"));
    }

    #[test]
    fn test_parse_threat_model_response_all_surface_kinds() {
        let json = r#"{
            "app_type": "mixed",
            "summary": "A test app",
            "surfaces": [
                {
                    "id": "S-001",
                    "kind": "endpoint",
                    "identifier": "POST /api/users",
                    "locations": ["src/routes.py"],
                    "description": "User registration",
                    "query": "(call) @route"
                },
                {
                    "id": "S-002",
                    "kind": "db_table",
                    "identifier": "users table",
                    "locations": ["src/models.py"],
                    "description": "User data storage",
                    "query": "(assignment) @table"
                },
                {
                    "id": "S-003",
                    "kind": "public_api",
                    "identifier": "pub fn parse()",
                    "locations": ["src/lib.rs"],
                    "description": "Public parsing function",
                    "query": "(function_item) @func"
                },
                {
                    "id": "S-004",
                    "kind": "file_handler",
                    "identifier": "upload_file()",
                    "locations": ["src/upload.py"],
                    "description": "File upload handler",
                    "query": "(call) @upload"
                },
                {
                    "id": "S-005",
                    "kind": "external_call",
                    "identifier": "requests.get(url)",
                    "locations": ["src/fetch.py"],
                    "description": "External HTTP call",
                    "query": "(call) @http"
                },
                {
                    "id": "S-006",
                    "kind": "iac_resource",
                    "identifier": "aws_s3_bucket.data",
                    "locations": ["infra/main.tf"],
                    "description": "S3 bucket resource",
                    "query": "(block) @resource"
                }
            ]
        }"#;

        let model = parse_threat_model_response(json, "test/repo").unwrap();
        assert_eq!(model.repository, "test/repo");
        assert_eq!(model.app_type, "mixed");
        assert_eq!(model.summary, "A test app");
        assert_eq!(model.surfaces.len(), 6);

        assert_eq!(model.surfaces[0].kind, "endpoint");
        assert_eq!(model.surfaces[0].identifier, "POST /api/users");
        assert_eq!(model.surfaces[0].locations, vec!["src/routes.py"]);

        assert_eq!(model.surfaces[1].kind, "db_table");
        assert_eq!(model.surfaces[2].kind, "public_api");
        assert_eq!(model.surfaces[3].kind, "file_handler");
        assert_eq!(model.surfaces[4].kind, "external_call");
        assert_eq!(model.surfaces[5].kind, "iac_resource");
        assert_eq!(model.surfaces[5].locations, vec!["infra/main.tf"]);
    }

    #[test]
    fn test_parse_threat_model_response_empty_surfaces() {
        let json = r#"{"app_type": "cli", "summary": "A CLI tool", "surfaces": []}"#;
        let model = parse_threat_model_response(json, "cli/repo").unwrap();
        assert_eq!(model.surfaces.len(), 0);
        assert_eq!(model.app_type, "cli");
    }

    #[test]
    fn test_parse_threat_model_response_invalid_json() {
        let result = parse_threat_model_response("not json", "repo");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_threat_model_response_sets_generated_at() {
        let json = r#"{"app_type": "lib", "summary": "s", "surfaces": []}"#;
        let model = parse_threat_model_response(json, "r").unwrap();
        assert!(!model.generated_at.is_empty());
    }
}
