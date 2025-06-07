use anyhow::Result;
use genai::chat::{ChatMessage, ChatOptions, ChatRequest, JsonSpec};
use genai::{Client, ClientConfig};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[allow(unused_imports)]
use std::path::{Path, PathBuf};

use crate::repo::RepoOps;
use crate::security_patterns::Language;

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

#[derive(Serialize, Deserialize, Debug)]
struct SecurityRiskAssessment {
    function_name: String,
    risk_level: String, // "high", "medium", "low", "none"
    reasoning: String,
    security_relevance: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct RiskFilterResponse {
    assessments: Vec<SecurityRiskAssessment>,
}

pub async fn generate_custom_patterns(root_dir: &Path, model: &str) -> Result<()> {
    println!(
        "📂 ディレクトリを解析してdefinitionsを抽出中: {}",
        root_dir.display()
    );

    let repo = RepoOps::new(root_dir.to_path_buf());
    let files = repo.get_files_to_analyze(None)?;

    println!("📁 検出されたファイル数: {}", files.len());
    for file in &files {
        println!("   - {}", file.display());
    }

    let mut all_definitions = Vec::new();
    let mut languages_found = HashMap::new();

    for file_path in &files {
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
                let ext = file_path.extension().and_then(|e| e.to_str()).unwrap_or("");
                let language = Language::from_extension(ext);
                languages_found.insert(language, true);

                println!(
                    "📄 {} (言語: {:?}) から {}個のdefinitionsを検出",
                    file_path.display(),
                    language,
                    context.definitions.len()
                );
                if context.definitions.is_empty() {
                    println!(
                        "   ⚠️  定義が見つかりませんでした。tree-sitterクエリが適切に動作していない可能性があります。"
                    );
                } else {
                    for def in &context.definitions {
                        println!("   - {}", def.name);
                    }
                }
                for def in context.definitions {
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
        "🔍 総計 {}個のdefinitionsを抽出しました",
        all_definitions.len()
    );

    for (language, _) in languages_found {
        let lang_definitions: Vec<_> = all_definitions
            .iter()
            .filter(|(_, lang)| *lang == language)
            .map(|(def, _)| def)
            .collect();

        if lang_definitions.is_empty() {
            continue;
        }

        println!(
            "🧠 {:?}言語の{}個のdefinitionsをLLMで分析中...",
            language,
            lang_definitions.len()
        );

        let patterns =
            analyze_definitions_for_security_patterns(model, &lang_definitions, language).await?;

        if !patterns.is_empty() {
            write_patterns_to_file(root_dir, language, &patterns)?;
            println!(
                "✅ {:?}言語用の{}個のパターンを生成しました",
                language,
                patterns.len()
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
) -> Result<Vec<PatternClassification>> {
    // First filter definitions to only include high-risk security-related ones
    let high_risk_definitions = filter_high_risk_definitions(model, definitions, language).await?;
    
    if high_risk_definitions.is_empty() {
        println!("   ℹ️  セキュリティリスクの高い定義が見つかりませんでした");
        return Ok(Vec::new());
    }

    println!("   🎯 {}個の高リスク定義を詳細分析中...", high_risk_definitions.len());
    
    let definitions_text = high_risk_definitions
        .iter()
        .map(|def| format!("Function: {}\nCode:\n{}\n---", def.name, def.source))
        .collect::<Vec<_>>()
        .join("\n\n");

    let prompt = format!(
        r#"Analyze the following HIGH-RISK function definitions from a {:?} codebase and classify them as security patterns.

These functions have already been pre-filtered as potentially security-relevant. For each function, determine if it should be classified as:
- "principals": Functions that represent sources of authority, user input, external data entry points, or second-order data sources (e.g., database query results, API responses, file contents)
- "actions": Functions that perform validation, sanitization, authorization, or security operations
- "resources": Functions that access, modify, or perform operations on files, databases, networks, or system resources
- null: Functions that don't fit any PAR security pattern category (should be rare for pre-filtered functions)

Note: Pay special attention to second-order principals - functions that return data from databases, APIs, or other external sources that could contain untrusted data originally from user input.

For each function that IS a security pattern, also identify potential MITRE ATT&CK techniques that could be associated with this pattern. Use the format "T1234" for technique IDs.

For each function that IS a security pattern, generate a regex pattern that would match similar functions.

Function Definitions:
{}

Return a JSON object with this exact structure:
{{
  "patterns": [
    {{
      "function_name": "example_function",
      "pattern_type": "principals",
      "pattern": "\\\\bexample_function\\\\s*\\\\(",
      "description": "Example function description",
      "reasoning": "Why this function is classified as this pattern type",
      "attack_vector": ["T1059", "T1190"]
    }}
  ]
}}

Only include functions that ARE security patterns (principals, actions, or resources). Do not include functions that are not security-related."#,
        language, definitions_text
    );

    let pattern_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "patterns": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "function_name": {"type": "string"},
                        "pattern_type": {"type": "string", "enum": ["principals", "actions", "resources"]},
                        "pattern": {"type": "string"},
                        "description": {"type": "string"},
                        "reasoning": {"type": "string"},
                        "attack_vector": {
                            "type": "array",
                            "items": {
                                "type": "string"
                            }
                        }
                    },
                    "required": ["function_name", "pattern_type", "pattern", "description", "reasoning", "attack_vector"]
                }
            }
        },
        "required": ["patterns"]
    });

    let client_config = ClientConfig::default().with_chat_options(
        ChatOptions::default().with_response_format(JsonSpec::new("json_object", pattern_schema)),
    );
    let client = Client::builder().with_config(client_config).build();

    let chat_req = ChatRequest::new(vec![
        ChatMessage::system(
            "You are a security pattern analyzer. You must reply with exactly one JSON object that matches the specified schema. Do not include any explanatory text outside the JSON object.",
        ),
        ChatMessage::user(&prompt),
    ]);

    let chat_res = client.exec_chat(model, chat_req, None).await?;
    let content = chat_res
        .content_text_as_str()
        .ok_or_else(|| anyhow::anyhow!("Failed to get response content"))?;

    let response: PatternAnalysisResponse = serde_json::from_str(content).map_err(|e| {
        anyhow::anyhow!("Failed to parse LLM response: {}. Content: {}", e, content)
    })?;

    Ok(response.patterns)
}

pub async fn filter_high_risk_definitions<'a>(
    model: &str,
    definitions: &'a [&crate::parser::Definition],
    language: Language,
) -> Result<Vec<&'a crate::parser::Definition>> {
    let definitions_summary = definitions
        .iter()
        .map(|def| format!("Function: {}\nSignature: {}", def.name, def.source.lines().next().unwrap_or("")))
        .collect::<Vec<_>>()
        .join("\n");

    let prompt = format!(
        r#"Analyze the following function definitions from a {:?} codebase and assess their security risk level.

For each function, evaluate if it:
1. Handles user input or external data (HTTP requests, file uploads, database queries, command line arguments, environment variables)
2. Performs authentication, authorization, or access control
3. Deals with cryptography, passwords, tokens, or secrets
4. Accesses files, databases, networks, or system resources
5. Performs data validation, sanitization, or encoding/decoding
6. Executes system commands or external processes
7. Handles configuration, logging, or error messages that might leak sensitive information
8. Deals with session management, cookies, or state management
9. Performs operations that could lead to code injection, path traversal, or other common vulnerabilities
10. Is a second-order data source (returns data from databases, APIs, or external sources)

Classify each function with a risk level:
- "high": Directly security-critical functions that handle untrusted input, authentication, authorization, or resource access
- "medium": Functions that might be security-relevant but less directly exposed
- "low": Functions with minimal security implications
- "none": Functions with no apparent security relevance

Function Definitions:
{}

Return a JSON object with this exact structure:
{{
  "assessments": [
    {{
      "function_name": "example_function",
      "risk_level": "high",
      "reasoning": "Detailed explanation of why this function has this risk level",
      "security_relevance": true
    }}
  ]
}}

Be conservative and inclusive - it's better to include a function that might be security-relevant than to miss one."#,
        language, definitions_summary
    );

    let risk_schema = serde_json::json!({
        "type": "object",
        "properties": {
            "assessments": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "function_name": {"type": "string"},
                        "risk_level": {"type": "string", "enum": ["high", "medium", "low", "none"]},
                        "reasoning": {"type": "string"},
                        "security_relevance": {"type": "boolean"}
                    },
                    "required": ["function_name", "risk_level", "reasoning", "security_relevance"]
                }
            }
        },
        "required": ["assessments"]
    });

    let client_config = ClientConfig::default().with_chat_options(
        ChatOptions::default().with_response_format(JsonSpec::new("json_object", risk_schema)),
    );
    let client = Client::builder().with_config(client_config).build();

    let chat_req = ChatRequest::new(vec![
        ChatMessage::system(
            "You are a security risk assessor. You must reply with exactly one JSON object that matches the specified schema. Do not include any explanatory text outside the JSON object.",
        ),
        ChatMessage::user(&prompt),
    ]);

    let chat_res = client.exec_chat(model, chat_req, None).await?;
    let content = chat_res
        .content_text_as_str()
        .ok_or_else(|| anyhow::anyhow!("Failed to get response content"))?;

    let response: RiskFilterResponse = serde_json::from_str(content).map_err(|e| {
        anyhow::anyhow!("Failed to parse LLM response: {}. Content: {}", e, content)
    })?;

    // Filter to only include high-risk and medium-risk functions that are security-relevant
    let high_risk_names: std::collections::HashSet<_> = response
        .assessments
        .iter()
        .filter(|assessment| {
            assessment.security_relevance && 
            (assessment.risk_level == "high" || assessment.risk_level == "medium")
        })
        .map(|assessment| assessment.function_name.as_str())
        .collect();

    let filtered_definitions: Vec<&crate::parser::Definition> = definitions
        .iter()
        .filter(|def| high_risk_names.contains(def.name.as_str()))
        .copied()
        .collect();

    println!(
        "   🔍 {}/{}個の定義が高・中リスクとして識別されました",
        filtered_definitions.len(),
        definitions.len()
    );

    for assessment in &response.assessments {
        if assessment.security_relevance && (assessment.risk_level == "high" || assessment.risk_level == "medium") {
            println!(
                "     - {} [{}]: {}",
                assessment.function_name,
                assessment.risk_level,
                assessment.reasoning
            );
        }
    }

    Ok(filtered_definitions)
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
