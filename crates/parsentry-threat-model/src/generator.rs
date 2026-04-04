use anyhow::Result;
use genai::chat::{ChatMessage, ChatOptions, ChatRequest, JsonSpec};
use genai::resolver::{AuthData, Endpoint, ServiceTargetResolver};
use genai::{Client, ClientConfig, ModelIden, ServiceTarget, adapter::AdapterKind};
use log::{debug, info};
use serde::Deserialize;

use crate::collector::RepoMetadata;
use crate::model::{ThreatEntry, ThreatModel, ThreatQuery};
use parsentry_core::Language;

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
            "Threat model generated: {} threats, {} queries",
            model.threats.len(),
            model.total_queries()
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

const SYSTEM_PROMPT: &str = r#"You are a security threat modeler. Given repository metadata, identify concrete threats and generate tree-sitter queries to detect them.

Rules for tree-sitter queries:
- Use valid tree-sitter S-expression syntax for the target language
- Always include a capture name for the match target (e.g. @call, @function, @attribute)
- Prefer specific patterns over broad ones to reduce false positives
- Use #eq? or #match? predicates to narrow matches
- Each query must be a single valid S-expression

PAR classification:
- "principal": Sources of untrusted data (user input, HTTP params, env vars, file reads)
- "action": Security controls (validation, sanitization, auth checks)
- "resource": Sinks that affect system state (DB writes, file writes, command execution, network calls)"#;

fn build_prompt(repo_context: &str, languages: &[String]) -> String {
    format!(
        r#"Analyze this repository and generate a threat model with tree-sitter queries.

{repo_context}

Languages present: {languages}

For each identified threat:
1. Explain WHY this threat applies to THIS specific repository (based on its dependencies, structure, and entry points)
2. Generate tree-sitter queries for the relevant language to detect the principal (source), action (control), and resource (sink) patterns
3. Focus on the most impactful threats — quality over quantity

Return a JSON object with this structure:
{{
  "summary": "High-level security posture summary (2-3 sentences)",
  "threats": [
    {{
      "id": "THREAT-001",
      "category": "e.g. SQL Injection",
      "rationale": "Why this threat applies to this repo specifically",
      "attack_vectors": ["T1190"],
      "attack_surface": ["src/db/", "api/handlers/"],
      "language": "Python",
      "queries": [
        {{
          "par_type": "principal|action|resource",
          "query_type": "definition|reference",
          "query": "(call function: (identifier) @func (#eq? @func \"execute\")) @call",
          "description": "What this query detects"
        }}
      ]
    }}
  ]
}}"#,
        repo_context = repo_context,
        languages = languages.join(", "),
    )
}

fn threat_model_schema() -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "summary": {"type": "string"},
            "threats": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "id": {"type": "string"},
                        "category": {"type": "string"},
                        "rationale": {"type": "string"},
                        "attack_vectors": {"type": "array", "items": {"type": "string"}},
                        "attack_surface": {"type": "array", "items": {"type": "string"}},
                        "language": {"type": "string"},
                        "queries": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "par_type": {"type": "string", "enum": ["principal", "action", "resource"]},
                                    "query_type": {"type": "string", "enum": ["definition", "reference"]},
                                    "query": {"type": "string"},
                                    "description": {"type": "string"}
                                },
                                "required": ["par_type", "query_type", "query", "description"]
                            }
                        }
                    },
                    "required": ["id", "category", "rationale", "attack_vectors", "attack_surface", "language", "queries"]
                }
            }
        },
        "required": ["summary", "threats"]
    })
}

#[derive(Deserialize)]
struct LlmResponse {
    summary: String,
    threats: Vec<LlmThreat>,
}

#[derive(Deserialize)]
struct LlmThreat {
    id: String,
    category: String,
    rationale: String,
    attack_vectors: Vec<String>,
    attack_surface: Vec<String>,
    language: String,
    queries: Vec<LlmQuery>,
}

#[derive(Deserialize)]
struct LlmQuery {
    par_type: String,
    query_type: String,
    query: String,
    description: String,
}

fn parse_language(s: &str) -> Language {
    match s.to_lowercase().as_str() {
        "python" => Language::Python,
        "javascript" => Language::JavaScript,
        "typescript" => Language::TypeScript,
        "rust" => Language::Rust,
        "java" => Language::Java,
        "go" => Language::Go,
        "ruby" => Language::Ruby,
        "c" => Language::C,
        "cpp" | "c++" => Language::Cpp,
        "terraform" | "hcl" => Language::Terraform,
        "php" => Language::Php,
        "yaml" => Language::Yaml,
        _ => Language::Other,
    }
}

fn parse_response(json_str: &str, repository: &str) -> Result<ThreatModel> {
    let resp: LlmResponse = serde_json::from_str(json_str)
        .map_err(|e| anyhow::anyhow!("Failed to parse threat model response: {}. Content: {}", e, json_str))?;

    let threats: Vec<ThreatEntry> = resp
        .threats
        .into_iter()
        .map(|t| ThreatEntry {
            id: t.id,
            category: t.category,
            rationale: t.rationale,
            attack_vectors: t.attack_vectors,
            attack_surface: t.attack_surface,
            language: parse_language(&t.language),
            queries: t
                .queries
                .into_iter()
                .map(|q| ThreatQuery {
                    par_type: q.par_type,
                    query_type: q.query_type,
                    query: q.query,
                    description: q.description,
                })
                .collect(),
        })
        .collect();

    Ok(ThreatModel {
        repository: repository.to_string(),
        generated_at: chrono::Utc::now().to_rfc3339(),
        summary: resp.summary,
        threats,
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
