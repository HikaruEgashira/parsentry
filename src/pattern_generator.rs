use anyhow::Result;
use futures::stream::{self, StreamExt};
use genai::chat::{ChatMessage, ChatOptions, ChatRequest, JsonSpec};
use genai::{Client, ClientConfig};
use genai::resolver::{Endpoint, ServiceTargetResolver};
use genai::{ServiceTarget, ModelIden, adapter::AdapterKind};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[allow(unused_imports)]
use std::path::{Path, PathBuf};
use streaming_iterator::StreamingIterator;

use crate::repo::RepoOps;
use crate::security_patterns::Language;
use crate::parser::Definition;

fn should_skip_reference(
    name: &str, 
    seen_names: &std::collections::HashSet<String>, 
    found_references: &std::collections::HashSet<String>
) -> bool {
    // Skip if already seen as definition or already found in this file
    if seen_names.contains(name) || found_references.contains(name) {
        return true;
    }
    
    // Skip if too short
    if name.len() < 3 {
        return true;
    }
    
    // Only consider names that likely represent external libraries or APIs
    // Prefer names with capital letters, underscores, or longer names that suggest external APIs
    let has_capitals = name.chars().any(|c| c.is_uppercase());
    let has_underscores = name.contains('_');
    let is_long_enough = name.len() >= 5;
    
    // Accept if it looks like an external API call
    !(has_capitals || has_underscores || is_long_enough)
}

fn create_pattern_client(api_base_url: Option<&str>, response_schema: serde_json::Value) -> Client {
    let client_config = ClientConfig::default().with_chat_options(
        ChatOptions::default().with_response_format(JsonSpec::new("json_object", response_schema)),
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
            let ServiceTarget { model, auth, .. } = service_target;
            
            // Use the custom base URL and force OpenAI adapter for compatibility
            let endpoint = Endpoint::from_owned(base_url_owned.clone());
            
            // When using custom base URL, assume OpenAI-compatible API
            let model = ModelIden::new(AdapterKind::OpenAI, model.model_name);
            
            Ok(ServiceTarget { endpoint, auth, model })
        },
    )
}

fn filter_files_by_size(files: &[PathBuf], max_lines: usize) -> Result<Vec<PathBuf>> {
    let mut filtered_files = Vec::new();
    
    for file_path in files {
        match std::fs::read_to_string(file_path) {
            Ok(content) => {
                let line_count = content.lines().count();
                if line_count <= max_lines {
                    filtered_files.push(file_path.clone());
                }
            },
            Err(e) => {
                eprintln!(
                    "⚠️  ファイル読み込みエラー: {}: {}", 
                    file_path.display(), 
                    e
                );
                // Skip files that can't be read
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
    pub pattern: String,
    pub description: String,
    pub reasoning: String,
    pub attack_vector: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PatternAnalysisResponse {
    patterns: Vec<PatternClassification>,
}


pub async fn generate_custom_patterns(root_dir: &Path, model: &str, api_base_url: Option<&str>) -> Result<()> {
    generate_custom_patterns_impl(root_dir, model, 4, api_base_url).await
}

async fn generate_custom_patterns_impl(root_dir: &Path, model: &str, _max_parallel: usize, api_base_url: Option<&str>) -> Result<()> {
    println!(
        "📂 ディレクトリを解析してdefinitionsを抽出中: {}",
        root_dir.display()
    );

    let repo = RepoOps::new(root_dir.to_path_buf());
    let files = repo.get_files_to_analyze(None)?;

    println!("📁 検出されたファイル数: {}", files.len());
    
    let max_lines = 1000;
    let filtered_files = filter_files_by_size(&files, max_lines)?;
    let skipped_count = files.len() - filtered_files.len();
    
    if skipped_count > 0 {
        println!("⚠️  {}行を超える大きなファイル{}個をスキップしました", max_lines, skipped_count);
    }
    
    println!("📁 解析対象ファイル数: {}", filtered_files.len());

    let mut all_definitions: Vec<(Definition, Language)> = Vec::new();
    let all_references: Vec<(Definition, Language)> = Vec::new();
    let mut languages_found = HashMap::new();
    let mut seen_names = std::collections::HashSet::new();

    for file_path in &filtered_files {
        let mut parser = crate::parser::CodeParser::new()?;
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
                let language = crate::file_classifier::FileClassifier::classify(&filename, &content);
                languages_found.insert(language, true);

                println!(
                    "📄 {} (言語: {:?}) から {}個のdefinitionsを検出",
                    file_path.display(),
                    language,
                    context.definitions.len()
                );
                if !context.definitions.is_empty() {
                    for def in &context.definitions {
                        print!("{},", def.name);
                    }
                    println!();
                }
                for def in context.definitions {
                    seen_names.insert(def.name.clone());
                    all_definitions.push((def, language));
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

        // Analyze both definitions and references
        let mut all_patterns = Vec::new();
        
        if !lang_definitions.is_empty() {
            let def_patterns =
                analyze_definitions_for_security_patterns(model, &lang_definitions, language, api_base_url).await?;
            all_patterns.extend(def_patterns);
        }
        
        if !lang_references.is_empty() {
            let ref_patterns =
                analyze_definitions_for_security_patterns(model, &lang_references, language, api_base_url).await?;
            all_patterns.extend(ref_patterns);
        }

        // Deduplicate patterns based on function name
        let mut unique_patterns = Vec::new();
        let mut seen_functions = std::collections::HashSet::new();
        
        for pattern in all_patterns {
            if seen_functions.insert(pattern.function_name.clone()) {
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
    definitions: &'a [&crate::parser::Definition],
    language: Language,
    api_base_url: Option<&str>,
) -> Result<Vec<PatternClassification>> {
    let max_parallel = 8; // デフォルトの並列数
    println!(
        "   🧠 {}個の定義を{}並列で個別分析中...",
        definitions.len(),
        max_parallel
    );

    // Create tasks for each definition analysis
    let model_str = model.to_string();
    let api_base_url_clone = api_base_url.map(|s| s.to_string());
    let tasks = definitions.iter().enumerate().map(|(idx, def)| {
        let model_clone = model_str.clone();
        let def_clone = (*def).clone();
        let api_base_url_ref = api_base_url_clone.as_deref();
        async move {
            if idx % 100 == 0 {
                println!("     🔍 定義 {}/{} を処理中...", idx + 1, definitions.len());
            }
            analyze_single_definition_for_pattern(&model_clone, &def_clone, language, api_base_url_ref).await
        }
    });

    // Execute tasks in parallel with controlled concurrency
    let results: Vec<Result<Option<PatternClassification>>> = stream::iter(tasks)
        .buffer_unordered(max_parallel)
        .collect()
        .await;

    // Collect successful patterns
    let mut all_patterns = Vec::new();
    let mut success_count = 0;
    let mut error_count = 0;

    for result in results {
        match result {
            Ok(Some(pattern)) => {
                all_patterns.push(pattern);
                success_count += 1;
            }
            Ok(None) => {
                // Definition was not a security pattern
                success_count += 1;
            }
            Err(e) => {
                error_count += 1;
                if error_count <= 5 { // Only show first 5 errors
                    eprintln!("     ⚠️  定義分析でエラー: {}", e);
                }
            }
        }
    }

    println!(
        "   ✅ 完了: 成功={}, エラー={}, 検出パターン={}個",
        success_count, error_count, all_patterns.len()
    );

    Ok(all_patterns)
}

async fn analyze_single_definition_for_pattern(
    model: &str,
    definition: &crate::parser::Definition,
    language: Language,
    api_base_url: Option<&str>,
) -> Result<Option<PatternClassification>> {

    let prompt = format!(
        r#"Analyze this single function definition from a {:?} codebase and determine if it represents a security pattern.

Function: {}
Code:
{}

Determine if this function should be classified as:
- "principals": Sources of authority, user input, external data entry points, or second-order data sources (e.g., database query results, API responses, file contents)
- "actions": Functions that perform validation, sanitization, authorization, or security operations  
- "resources": Functions that access, modify, or perform operations on files, databases, networks, or system resources
- "none": Not a security pattern

For ALL responses, provide:
1. A regex pattern (even if classification is "none", use a basic pattern)
2. A description (brief explanation)
3. Potential MITRE ATT&CK techniques (use empty array if none applicable)

Return a JSON object with this structure:
{{
  "classification": "principals|actions|resources|none",
  "function_name": "{}",
  "pattern": "\\\\bfunction_name\\\\s*\\\\(",
  "description": "Brief description of what this pattern detects",
  "reasoning": "Why this function fits this classification",
  "attack_vector": ["T1234", "T5678"]
}}

All fields are required."#,
        language,
        definition.name,
        definition.source,
        definition.name
    );

    let response_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "classification": {"type": "string", "enum": ["principals", "actions", "resources", "none"]},
            "function_name": {"type": "string"},
            "pattern": {"type": "string"},
            "description": {"type": "string"},
            "reasoning": {"type": "string"},
            "attack_vector": {
                "type": "array",
                "items": {"type": "string"}
            }
        },
        "required": ["classification", "function_name", "pattern", "description", "reasoning", "attack_vector"]
    });

    let client = create_pattern_client(api_base_url, response_schema);

    let chat_req = ChatRequest::new(vec![
        ChatMessage::system(
            "You are a security pattern analyzer. Reply with exactly one JSON object. Be conservative - only classify as security patterns if clearly relevant.",
        ),
        ChatMessage::user(&prompt),
    ]);

    let chat_res = client.exec_chat(model, chat_req, None).await?;
    let content = chat_res
        .content_text_as_str()
        .ok_or_else(|| anyhow::anyhow!("Failed to get response content"))?;

    #[derive(Deserialize)]
    struct SingleAnalysisResponse {
        classification: String,
        function_name: String,
        pattern: String,
        description: String,
        reasoning: String,
        attack_vector: Vec<String>,
    }

    let response: SingleAnalysisResponse = serde_json::from_str(content).map_err(|e| {
        anyhow::anyhow!("Failed to parse LLM response: {}. Content: {}", e, content)
    })?;

    if response.classification == "none" {
        return Ok(None);
    }

    Ok(Some(PatternClassification {
        function_name: response.function_name,
        pattern_type: Some(response.classification),
        pattern: response.pattern,
        description: response.description,
        reasoning: response.reasoning,
        attack_vector: response.attack_vector,
    }))
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
            yaml_content.push_str(&format!(
                "    - pattern: \"{}\"\n      description: \"{}\"\n",
                pattern.pattern, pattern.description
            ));
            if !pattern.attack_vector.is_empty() {
                yaml_content.push_str("      attack_vector:\n");
                for technique in &pattern.attack_vector {
                    yaml_content.push_str(&format!("        - \"{}\"\n", technique));
                }
            }
        }
    }

    if !actions.is_empty() {
        yaml_content.push_str("  actions:\n");
        for pattern in actions {
            yaml_content.push_str(&format!(
                "    - pattern: \"{}\"\n      description: \"{}\"\n",
                pattern.pattern, pattern.description
            ));
            if !pattern.attack_vector.is_empty() {
                yaml_content.push_str("      attack_vector:\n");
                for technique in &pattern.attack_vector {
                    yaml_content.push_str(&format!("        - \"{}\"\n", technique));
                }
            }
        }
    }

    if !resources.is_empty() {
        yaml_content.push_str("  resources:\n");
        for pattern in resources {
            yaml_content.push_str(&format!(
                "    - pattern: \"{}\"\n      description: \"{}\"\n",
                pattern.pattern, pattern.description
            ));
            if !pattern.attack_vector.is_empty() {
                yaml_content.push_str("      attack_vector:\n");
                for technique in &pattern.attack_vector {
                    yaml_content.push_str(&format!("        - \"{}\"\n", technique));
                }
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
