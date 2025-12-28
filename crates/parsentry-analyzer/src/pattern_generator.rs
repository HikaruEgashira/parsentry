use anyhow::Result;
use genai::chat::{ChatMessage, ChatOptions, ChatRequest, JsonSpec};
use genai::resolver::{AuthData, Endpoint, ServiceTargetResolver};
use genai::{Client, ClientConfig};
use genai::{ModelIden, ServiceTarget, adapter::AdapterKind};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use log::warn;
use parsentry_core::Language;
use parsentry_parser::{CodeParser, Definition};
use parsentry_reports::prompts::vuln_specific;
use parsentry_utils::{FileClassifier, FileDiscovery};

fn create_pattern_client(api_base_url: Option<&str>, response_schema: serde_json::Value) -> Client {
    let client_config = ClientConfig::default().with_chat_options(
        ChatOptions::default()
            .with_normalize_reasoning_content(true)
            .with_response_format(JsonSpec::new("json_object", response_schema)),
    );

    let mut client_builder = Client::builder().with_config(client_config);

    // Add custom service target resolver if base URL is provided
    if let Some(base_url) = api_base_url {
        let target_resolver = create_pattern_target_resolver(base_url);
        client_builder = client_builder.with_service_target_resolver(target_resolver);
    }

    client_builder.build()
}

fn create_pattern_target_resolver(base_url: &str) -> ServiceTargetResolver {
    let base_url_owned = base_url.to_string();

    ServiceTargetResolver::from_resolver_fn(
        move |service_target: ServiceTarget| -> Result<ServiceTarget, genai::resolver::Error> {
            let ServiceTarget { model, .. } = service_target;

            // Use the custom base URL and force OpenAI adapter for compatibility
            let endpoint = Endpoint::from_owned(base_url_owned.clone());

            // When using custom base URL, assume OpenAI-compatible API
            let model = ModelIden::new(AdapterKind::OpenAI, model.model_name);

            // Use the OPENAI_API_KEY environment variable as the new key when using custom URL
            let auth = AuthData::from_env("OPENAI_API_KEY");
            Ok(ServiceTarget {
                endpoint,
                auth,
                model,
            })
        },
    )
}

pub fn filter_files_by_size(files: &[PathBuf], max_lines: usize) -> Result<Vec<PathBuf>> {
    let mut filtered_files = Vec::new();

    for file_path in files {
        match std::fs::read_to_string(file_path) {
            Ok(content) => {
                let line_count = content.lines().count();
                if line_count <= max_lines {
                    filtered_files.push(file_path.clone());
                }
            }
            Err(e) => {
                warn!("ファイル読み込みエラー: {}: {}", file_path.display(), e);
                continue;
            }
        }
    }

    Ok(filtered_files)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PatternClassification {
    pub function_name: String,
    pub pattern_type: Option<String>,
    pub query_type: String,
    pub query: String,
    pub description: String,
    pub reasoning: String,
    pub attack_vector: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PatternAnalysisResponse {
    patterns: Vec<PatternClassification>,
}

pub async fn generate_custom_patterns(
    root_dir: &Path,
    model: &str,
    api_base_url: Option<&str>,
) -> Result<()> {
    generate_custom_patterns_impl(root_dir, model, 4, api_base_url).await
}

async fn generate_custom_patterns_impl(
    root_dir: &Path,
    model: &str,
    _max_parallel: usize,
    api_base_url: Option<&str>,
) -> Result<()> {
    println!(
        "📂 ディレクトリを解析してdefinitionsを抽出中: {}",
        root_dir.display()
    );

    let file_discovery = FileDiscovery::new(root_dir.to_path_buf());
    let files = file_discovery.get_files()?;

    println!("📁 検出されたファイル数: {}", files.len());

    let max_lines = 1000;
    let filtered_files = filter_files_by_size(&files, max_lines)?;
    let skipped_count = files.len() - filtered_files.len();

    if skipped_count > 0 {
        println!(
            "⚠️  {}行を超える大きなファイル{}個をスキップしました",
            max_lines, skipped_count
        );
    }

    println!("📁 解析対象ファイル数: {}", filtered_files.len());

    let mut all_definitions: Vec<(Definition, Language)> = Vec::new();
    let mut all_references: Vec<(Definition, Language)> = Vec::new();
    let mut languages_found = HashMap::new();
    let mut seen_names = std::collections::HashSet::new();

    for file_path in &filtered_files {
        let mut parser = CodeParser::new()?;
        if let Err(e) = parser.add_file(file_path) {
            eprintln!(
                "⚠️  ファイルのパース追加に失敗: {}: {}",
                file_path.display(),
                e
            );
            continue;
        }

        match parser.build_context_from_file(file_path) {
            Ok(context) => {
                let filename = file_path.to_string_lossy();
                let content = std::fs::read_to_string(file_path).unwrap_or_default();
                let language = FileClassifier::classify(&filename, &content);
                languages_found.insert(language, true);

                println!(
                    "📄 {} (言語: {:?}) から {}個のdefinitions、{}個のreferencesを検出",
                    file_path.display(),
                    language,
                    context.definitions.len(),
                    context.references.len()
                );

                for def in context.definitions {
                    seen_names.insert(def.name.clone());
                    all_definitions.push((def, language));
                }
                for ref_def in context.references {
                    all_references.push((ref_def, language));
                }
            }
            Err(e) => {
                eprintln!("⚠️  コンテキスト収集に失敗: {}: {}", file_path.display(), e);
                continue;
            }
        }
    }

    println!(
        "🔍 総計 {}個のdefinitions、{}個のreferencesを抽出しました",
        all_definitions.len(),
        all_references.len()
    );

    for (language, _) in languages_found {
        // Combine definitions and references for this language
        let lang_definitions: Vec<_> = all_definitions
            .iter()
            .filter(|(_, lang)| *lang == language)
            .map(|(def, _)| def)
            .collect();

        let lang_references: Vec<_> = all_references
            .iter()
            .filter(|(_, lang)| *lang == language)
            .map(|(def, _)| def)
            .collect();

        let total_items = lang_definitions.len() + lang_references.len();
        if total_items == 0 {
            continue;
        }

        println!(
            "🧠 {:?}言語の{}個のdefinitions、{}個のreferencesを分析中...",
            language,
            lang_definitions.len(),
            lang_references.len()
        );

        // Process definitions and references separately to maintain their distinctions
        let mut definition_patterns = Vec::new();
        let mut reference_patterns = Vec::new();

        if !lang_definitions.is_empty() {
            definition_patterns = analyze_definitions_for_security_patterns(
                model,
                &lang_definitions,
                language,
                api_base_url,
            )
            .await?;
        }

        if !lang_references.is_empty() {
            reference_patterns = analyze_references_for_security_patterns(
                model,
                &lang_references,
                language,
                api_base_url,
            )
            .await?;
        }

        // Combine all patterns
        let mut all_patterns = definition_patterns;
        all_patterns.extend(reference_patterns);

        let mut unique_patterns = Vec::new();
        let mut seen: std::collections::HashSet<(String, Option<String>, String)> =
            std::collections::HashSet::new();

        for pattern in all_patterns {
            let key = (
                pattern.function_name.clone(),
                pattern.pattern_type.clone(),
                pattern.query_type.clone(),
            );
            if seen.insert(key) {
                unique_patterns.push(pattern);
            }
        }

        if !unique_patterns.is_empty() {
            write_patterns_to_file(root_dir, language, &unique_patterns)?;
            println!(
                "✅ {:?}言語用の{}個のパターンを生成しました",
                language,
                unique_patterns.len()
            );
        } else {
            println!(
                "ℹ️  {:?}言語でセキュリティパターンは検出されませんでした",
                language
            );
        }
    }

    println!("🎉 カスタムパターン生成が完了しました");
    Ok(())
}

pub async fn analyze_definitions_for_security_patterns<'a>(
    model: &str,
    definitions: &'a [&Definition],
    language: Language,
    api_base_url: Option<&str>,
) -> Result<Vec<PatternClassification>> {
    analyze_all_definitions_at_once(model, definitions, language, api_base_url).await
}

pub async fn analyze_references_for_security_patterns<'a>(
    model: &str,
    references: &'a [&Definition],
    language: Language,
    api_base_url: Option<&str>,
) -> Result<Vec<PatternClassification>> {
    analyze_all_references_at_once(model, references, language, api_base_url).await
}

async fn analyze_all_definitions_at_once(
    model: &str,
    definitions: &[&Definition],
    language: Language,
    api_base_url: Option<&str>,
) -> Result<Vec<PatternClassification>> {
    if definitions.is_empty() {
        return Ok(Vec::new());
    }

    // Create context for all definitions
    let mut definitions_context = String::new();
    for (idx, def) in definitions.iter().enumerate() {
        definitions_context.push_str(&format!(
            "Definition {}: {}\nCode:\n{}\n\n",
            idx + 1,
            def.name,
            def.source
        ));
    }

    let vuln_info_map = vuln_specific::get_vuln_specific_info();
    let vuln_hints: String = vuln_info_map
        .iter()
        .map(|(vt, info)| {
            format!(
                "- {:?}: {} (Bypasses: {})",
                vt,
                info.prompt,
                info.bypasses.join(", ")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let prompt = format!(
        r#"Analyze these function definitions from a {language:?} codebase and determine which represent security patterns.

{definitions_context}

## Vulnerability-Specific Detection Hints

Consider these vulnerability types when classifying functions:
{vuln_hints}

## Classification Guidelines

For each function, determine if it should be classified as:
- "principals": Sources that act as data entry points and should be treated as tainted/untrusted (e.g., request.args, user input, file reads, external API responses)
- "actions": Functions that perform validation, sanitization, authorization, or security operations (e.g., input validation, SQL escaping, authentication checks)
- "resources": Functions that access, modify, or perform operations on files, databases, networks, or system resources (e.g., SQL queries, file writes, command execution)
- "none": Not a security pattern

## Output Format

Generate tree-sitter queries instead of regex patterns. Use the following format:

IMPORTANT: For definition patterns, add @function capture to the entire function_definition.
For reference patterns, add @call capture to the entire call_expression or @attribute capture to the entire attribute access.
This ensures we capture the complete context, not just the identifier names.

Return a JSON object with this structure:

{{
  "patterns": [
    {{
      "classification": "principals|actions|resources|none",
      "function_name": "function_name",
      "query_type": "definition",
      "query": "(function_definition name: (identifier) @name (#eq? @name \"function_name\")) @function",
      "description": "Brief description of what this pattern detects",
      "reasoning": "Why this function fits this classification",
      "attack_vector": ["T1234", "T5678"]
    }}
  ]
}}

All fields are required for each object. Use proper tree-sitter query syntax for the {language:?} language."#,
        language = language, definitions_context = definitions_context, vuln_hints = vuln_hints
    );

    let response_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "patterns": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "classification": {"type": "string", "enum": ["principals", "actions", "resources", "none"]},
                        "function_name": {"type": "string"},
                        "query_type": {"type": "string", "enum": ["definition", "reference"]},
                        "query": {"type": "string"},
                        "description": {"type": "string"},
                        "reasoning": {"type": "string"},
                        "attack_vector": {
                            "type": "array",
                            "items": {"type": "string"}
                        }
                    },
                    "required": ["classification", "function_name", "query_type", "query", "description", "reasoning", "attack_vector"]
                }
            }
        },
        "required": ["patterns"]
    });

    let client = create_pattern_client(api_base_url, response_schema);

    let chat_req = ChatRequest::new(vec![
        ChatMessage::system(
            "You are a security pattern analyzer. Reply with exactly one JSON object containing a 'patterns' array with analysis for all functions. Be conservative - only classify as security patterns if clearly relevant.",
        ),
        ChatMessage::user(&prompt),
    ]);

    let chat_res = client.exec_chat(model, chat_req, None).await?;
    let content = chat_res
        .first_text()
        .ok_or_else(|| anyhow::anyhow!("Failed to get response content"))?;

    #[derive(Deserialize)]
    struct BatchAnalysisResponse {
        patterns: Vec<PatternResponse>,
    }

    #[derive(Deserialize)]
    struct PatternResponse {
        classification: String,
        function_name: String,
        query_type: String,
        query: String,
        description: String,
        reasoning: String,
        attack_vector: Vec<String>,
    }

    let response: BatchAnalysisResponse = serde_json::from_str(content).map_err(|e| {
        anyhow::anyhow!("Failed to parse LLM response: {}. Content: {}", e, content)
    })?;

    let mut patterns = Vec::new();
    let mut security_pattern_count = 0;

    for pattern_resp in response.patterns {
        if pattern_resp.classification != "none" {
            patterns.push(PatternClassification {
                function_name: pattern_resp.function_name,
                pattern_type: Some(pattern_resp.classification),
                query_type: pattern_resp.query_type,
                query: pattern_resp.query,
                description: pattern_resp.description,
                reasoning: pattern_resp.reasoning,
                attack_vector: pattern_resp.attack_vector,
            });
            security_pattern_count += 1;
        }
    }

    println!(
        "✅ 完了: 全{}個分析, セキュリティパターン{}個検出",
        definitions.len(),
        security_pattern_count
    );

    Ok(patterns)
}

async fn analyze_all_references_at_once(
    model: &str,
    references: &[&Definition],
    language: Language,
    api_base_url: Option<&str>,
) -> Result<Vec<PatternClassification>> {
    if references.is_empty() {
        return Ok(Vec::new());
    }

    // Create context for all references
    let mut references_context = String::new();
    for (idx, ref_def) in references.iter().enumerate() {
        references_context.push_str(&format!(
            "Reference {}: {}\nCode:\n{}\n\n",
            idx + 1,
            ref_def.name,
            ref_def.source
        ));
    }

    let vuln_info_map = vuln_specific::get_vuln_specific_info();
    let vuln_hints: String = vuln_info_map
        .iter()
        .map(|(vt, info)| {
            format!(
                "- {:?}: {} (Bypasses: {})",
                vt,
                info.prompt,
                info.bypasses.join(", ")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let prompt = format!(
        r#"Analyze these function references/calls from a {language:?} codebase and determine which represent calls to security-sensitive functions.

{references_context}

## Vulnerability-Specific Detection Hints

Consider these vulnerability types when classifying function calls:
{vuln_hints}

## Classification Guidelines

For each function reference, determine if it should be classified as:
- "principals": Functions that return or provide untrusted data that attackers can control (e.g., request.get(), input(), file read operations)
- "actions": Functions that perform security processing (validation, sanitization, authorization) (e.g., escape(), validate(), authenticate())
- "resources": Functions that operate on attack targets (files, databases, system commands, DOM) (e.g., execute(), open(), query(), innerHTML)
- "none": Not a security-relevant call

## Output Format

Return a JSON object with this structure:

{{
  "patterns": [
    {{
      "classification": "principals|actions|resources|none",
      "function_name": "function_name",
      "query_type": "reference",
      "query": "(call_expression function: (identifier) @name (#eq? @name \"function_name\")) @call",
      "description": "Brief description of what this pattern detects",
      "reasoning": "Why this function call fits this classification",
      "attack_vector": ["T1234", "T5678"]
    }}
  ]
}}

All fields are required for each object. Use proper tree-sitter query syntax for the {language:?} language."#,
        language = language, references_context = references_context, vuln_hints = vuln_hints
    );

    let response_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "patterns": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "classification": {"type": "string", "enum": ["principals", "actions", "resources", "none"]},
                        "function_name": {"type": "string"},
                        "query_type": {"type": "string", "enum": ["definition", "reference"]},
                        "query": {"type": "string"},
                        "description": {"type": "string"},
                        "reasoning": {"type": "string"},
                        "attack_vector": {
                            "type": "array",
                            "items": {"type": "string"}
                        }
                    },
                    "required": ["classification", "function_name", "query_type", "query", "description", "reasoning", "attack_vector"]
                }
            }
        },
        "required": ["patterns"]
    });

    let client = create_pattern_client(api_base_url, response_schema);

    let chat_req = ChatRequest::new(vec![
        ChatMessage::system(
            "You are a security pattern analyzer for function references. Reply with exactly one JSON object containing a 'patterns' array with analysis for all function calls. Focus on calls to security-sensitive functions.",
        ),
        ChatMessage::user(&prompt),
    ]);

    let chat_res = client.exec_chat(model, chat_req, None).await?;
    let content = chat_res
        .first_text()
        .ok_or_else(|| anyhow::anyhow!("Failed to get response content"))?;

    #[derive(Deserialize)]
    struct BatchAnalysisResponse {
        patterns: Vec<PatternResponse>,
    }

    #[derive(Deserialize)]
    struct PatternResponse {
        classification: String,
        function_name: String,
        query_type: String,
        query: String,
        description: String,
        reasoning: String,
        attack_vector: Vec<String>,
    }

    let response: BatchAnalysisResponse = serde_json::from_str(content).map_err(|e| {
        anyhow::anyhow!("Failed to parse LLM response: {}. Content: {}", e, content)
    })?;

    let mut patterns = Vec::new();
    let mut security_pattern_count = 0;

    for pattern_resp in response.patterns {
        if pattern_resp.classification != "none" {
            patterns.push(PatternClassification {
                function_name: pattern_resp.function_name,
                pattern_type: Some(pattern_resp.classification),
                query_type: pattern_resp.query_type,
                query: pattern_resp.query,
                description: pattern_resp.description,
                reasoning: pattern_resp.reasoning,
                attack_vector: pattern_resp.attack_vector,
            });
            security_pattern_count += 1;
        }
    }

    println!(
        "✅ 完了: 全{}個参照分析, セキュリティパターン{}個検出",
        references.len(),
        security_pattern_count
    );

    Ok(patterns)
}

pub fn write_patterns_to_file(
    root_dir: &Path,
    language: Language,
    patterns: &[PatternClassification],
) -> Result<()> {
    let mut vuln_patterns_path = root_dir.to_path_buf();
    vuln_patterns_path.push("vuln-patterns.yml");

    let lang_name = match language {
        Language::Python => "Python",
        Language::JavaScript => "JavaScript",
        Language::TypeScript => "TypeScript",
        Language::Rust => "Rust",
        Language::Java => "Java",
        Language::Go => "Go",
        Language::Ruby => "Ruby",
        Language::C => "C",
        Language::Cpp => "Cpp",
        Language::Terraform => "Terraform",
        Language::CloudFormation => "CloudFormation",
        Language::Kubernetes => "Kubernetes",
        Language::Yaml => "YAML",
        Language::Bash => "Bash",
        Language::Shell => "Shell",
        Language::Php => "Php",
        Language::Other => return Ok(()),
    };

    let mut principals = Vec::new();
    let mut actions = Vec::new();
    let mut resources = Vec::new();

    for pattern in patterns {
        match pattern.pattern_type.as_deref() {
            Some("principals") => principals.push(pattern),
            Some("actions") => actions.push(pattern),
            Some("resources") => resources.push(pattern),
            _ => {}
        }
    }

    let mut yaml_content = format!("{}:\n", lang_name);

    if !principals.is_empty() {
        yaml_content.push_str("  principals:\n");
        for pattern in principals {
            yaml_content.push_str(&format!("    - {}: |\n", pattern.query_type));
            for line in pattern.query.lines() {
                yaml_content.push_str(&format!("        {}\n", line));
            }
            yaml_content.push_str(&format!("      description: \"{}\"\n", pattern.description));
            yaml_content.push_str("      attack_vector:\n");
            if !pattern.attack_vector.is_empty() {
                for technique in &pattern.attack_vector {
                    yaml_content.push_str(&format!("        - \"{}\"\n", technique));
                }
            } else {
                yaml_content.push_str("        []\n");
            }
        }
    }

    if !actions.is_empty() {
        yaml_content.push_str("  actions:\n");
        for pattern in actions {
            yaml_content.push_str(&format!("    - {}: |\n", pattern.query_type));
            for line in pattern.query.lines() {
                yaml_content.push_str(&format!("        {}\n", line));
            }
            yaml_content.push_str(&format!("      description: \"{}\"\n", pattern.description));
            yaml_content.push_str("      attack_vector:\n");
            if !pattern.attack_vector.is_empty() {
                for technique in &pattern.attack_vector {
                    yaml_content.push_str(&format!("        - \"{}\"\n", technique));
                }
            } else {
                yaml_content.push_str("        []\n");
            }
        }
    }

    if !resources.is_empty() {
        yaml_content.push_str("  resources:\n");
        for pattern in resources {
            yaml_content.push_str(&format!("    - {}: |\n", pattern.query_type));
            for line in pattern.query.lines() {
                yaml_content.push_str(&format!("        {}\n", line));
            }
            yaml_content.push_str(&format!("      description: \"{}\"\n", pattern.description));
            yaml_content.push_str("      attack_vector:\n");
            if !pattern.attack_vector.is_empty() {
                for technique in &pattern.attack_vector {
                    yaml_content.push_str(&format!("        - \"{}\"\n", technique));
                }
            } else {
                yaml_content.push_str("        []\n");
            }
        }
    }

    if vuln_patterns_path.exists() {
        let existing_content = std::fs::read_to_string(&vuln_patterns_path)?;
        let updated_content = format!("{}\n{}", existing_content, yaml_content);
        std::fs::write(&vuln_patterns_path, updated_content)?;
    } else {
        std::fs::write(&vuln_patterns_path, yaml_content)?;
    }

    println!(
        "📝 パターンファイルに追記: {}",
        vuln_patterns_path.display()
    );
    Ok(())
}
