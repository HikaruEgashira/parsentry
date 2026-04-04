use anyhow::Result;
use genai::chat::{ChatMessage, ChatOptions, ChatRequest, JsonSpec};
use genai::resolver::{AuthData, Endpoint, ServiceTargetResolver};
use genai::{Client, ClientConfig, ModelIden, ServiceTarget, adapter::AdapterKind};
use log::{debug, info};
use serde::Deserialize;

use crate::collector::RepoMetadata;
use crate::model::{AttackSurface, SurfaceKind, ThreatModel};

pub struct ThreatModelGenerator {
    model: String,
    api_base_url: Option<String>,
}

impl ThreatModelGenerator {
    pub fn new(model: &str, api_base_url: Option<&str>) -> Self {
        Self {
            model: model.to_string(),
            api_base_url: api_base_url.map(|s| s.to_string()),
        }
    }

    /// Generate a threat model from repository metadata. Single LLM call.
    pub async fn generate(&self, metadata: &RepoMetadata) -> Result<ThreatModel> {
        let repo_context = metadata.to_prompt_context();
        let languages: Vec<String> = metadata
            .languages
            .keys()
            .map(|l| format!("{:?}", l))
            .collect();

        let prompt = build_prompt(&repo_context, &languages);
        let response = self.call_llm(&prompt).await?;

        let model = parse_response(
            &response,
            &metadata
                .root_dir
                .to_string_lossy()
                .to_string(),
        )?;

        info!(
            "Threat model generated: {} attack surfaces",
            model.total_surfaces()
        );

        Ok(model)
    }

    async fn call_llm(&self, prompt: &str) -> Result<String> {
        let response_schema = threat_model_schema();
        let client = create_client(self.api_base_url.as_deref(), response_schema);

        let chat_req = ChatRequest::new(vec![
            ChatMessage::system(SYSTEM_PROMPT),
            ChatMessage::user(prompt),
        ]);

        let chat_res = client.exec_chat(&self.model, chat_req, None).await?;
        let content = chat_res
            .first_text()
            .ok_or_else(|| anyhow::anyhow!("Empty LLM response"))?;

        debug!("LLM response length: {} chars", content.len());
        Ok(content.to_string())
    }
}

pub const SYSTEM_PROMPT: &str = r#"You are an attack surface enumerator. Given repository metadata, identify all concrete attack surfaces and generate a tree-sitter query for each to locate them in code.

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

pub fn build_prompt(repo_context: &str, languages: &[String]) -> String {
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

pub fn parse_response(json_str: &str, repository: &str) -> Result<ThreatModel> {
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

fn create_client(api_base_url: Option<&str>, response_schema: serde_json::Value) -> Client {
    let client_config = ClientConfig::default().with_chat_options(
        ChatOptions::default()
            .with_normalize_reasoning_content(true)
            .with_response_format(JsonSpec::new("json_object", response_schema)),
    );

    let mut client_builder = Client::builder().with_config(client_config);

    if let Some(base_url) = api_base_url {
        let base_url_owned = base_url.to_string();
        let target_resolver = ServiceTargetResolver::from_resolver_fn(
            move |service_target: ServiceTarget| -> Result<ServiceTarget, genai::resolver::Error> {
                let ServiceTarget { model, .. } = service_target;
                let endpoint = Endpoint::from_owned(base_url_owned.clone());
                let model = ModelIden::new(AdapterKind::OpenAI, model.model_name);
                let auth = AuthData::from_env("OPENAI_API_KEY");
                Ok(ServiceTarget {
                    endpoint,
                    auth,
                    model,
                })
            },
        );
        client_builder = client_builder.with_service_target_resolver(target_resolver);
    }

    client_builder.build()
}
