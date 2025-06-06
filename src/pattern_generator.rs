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
}

#[derive(Serialize, Deserialize, Debug)]
struct PatternAnalysisResponse {
    patterns: Vec<PatternClassification>,
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

async fn analyze_definitions_for_security_patterns(
    model: &str,
    definitions: &[&crate::parser::Definition],
    language: Language,
) -> Result<Vec<PatternClassification>> {
    let definitions_text = definitions
        .iter()
        .map(|def| format!("Function: {}\nCode:\n{}\n---", def.name, def.source))
        .collect::<Vec<_>>()
        .join("\n\n");

    let prompt = format!(
        r#"Analyze the following function definitions from a {:?} codebase and classify them as security patterns.

For each function, determine if it should be classified as:
- "sources": Functions that introduce user input, external data, or untrusted data into the application
- "sinks": Functions that can execute, write, or perform dangerous operations with data
- "validate": Functions that validate, sanitize, or secure data
- null: Functions that don't fit any security pattern category

For each function that IS a security pattern, generate a regex pattern that would match similar functions.

Function Definitions:
{}

Return a JSON object with this exact structure:
{{
  "patterns": [
    {{
      "function_name": "example_function",
      "pattern_type": "sources",
      "pattern": "\\\\bexample_function\\\\s*\\\\(",
      "description": "Example function description",
      "reasoning": "Why this function is classified as this pattern type"
    }}
  ]
}}

Only include functions that ARE security patterns (sources, sinks, or validate). Do not include functions that are not security-related."#,
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
                        "pattern_type": {"type": "string", "enum": ["sources", "sinks", "validate"]},
                        "pattern": {"type": "string"},
                        "description": {"type": "string"},
                        "reasoning": {"type": "string"}
                    },
                    "required": ["function_name", "pattern_type", "pattern", "description", "reasoning"]
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

    println!("🔍 LLM Response: {}", content);

    let response: PatternAnalysisResponse = serde_json::from_str(content).map_err(|e| {
        anyhow::anyhow!("Failed to parse LLM response: {}. Content: {}", e, content)
    })?;

    Ok(response.patterns)
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
        Language::Other => return Ok(()),
    };

    let mut sources = Vec::new();
    let mut sinks = Vec::new();
    let mut validate = Vec::new();

    for pattern in patterns {
        match pattern.pattern_type.as_deref() {
            Some("sources") => sources.push(pattern),
            Some("sinks") => sinks.push(pattern),
            Some("validate") => validate.push(pattern),
            _ => {}
        }
    }

    let mut yaml_content = format!("{}:\n", lang_name);

    if !sources.is_empty() {
        yaml_content.push_str("  sources:\n");
        for pattern in sources {
            yaml_content.push_str(&format!(
                "    - pattern: \"{}\"\n      description: \"{}\"\n",
                pattern.pattern, pattern.description
            ));
        }
    }

    if !validate.is_empty() {
        yaml_content.push_str("  validate:\n");
        for pattern in validate {
            yaml_content.push_str(&format!(
                "    - pattern: \"{}\"\n      description: \"{}\"\n",
                pattern.pattern, pattern.description
            ));
        }
    }

    if !sinks.is_empty() {
        yaml_content.push_str("  sinks:\n");
        for pattern in sinks {
            yaml_content.push_str(&format!(
                "    - pattern: \"{}\"\n      description: \"{}\"\n",
                pattern.pattern, pattern.description
            ));
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
