//! Codex-based pattern generation
//!
//! This module provides pattern generation using OpenAI Codex API.

use anyhow::Result;
use serde::Deserialize;
use std::path::Path;

use parsentry_analyzer::{filter_files_by_size, write_patterns_to_file, PatternClassification};
use parsentry_codex::{CodexConfig, CodexExecutor};
use parsentry_core::Language;
use parsentry_parser::Definition;
use parsentry_utils::{FileClassifier, FileDiscovery};

/// Result of pattern generation process
#[derive(Debug, Clone)]
pub struct PatternGenerationResult {
    pub total_files: usize,
    pub analyzed_files: usize,
    pub skipped_files: usize,
    pub patterns_generated: usize,
    pub languages: Vec<Language>,
}

/// Generate custom security patterns using Codex API
pub async fn generate_custom_patterns_with_codex(
    root_dir: &Path,
    config: CodexConfig,
) -> Result<PatternGenerationResult> {
    log::info!("Analyzing directory for definitions: {}", root_dir.display());

    let file_discovery = FileDiscovery::new(root_dir.to_path_buf());
    let files = file_discovery.get_files()?;

    log::debug!("Found {} files", files.len());

    let max_lines = 1000;
    let filtered_files = filter_files_by_size(&files, max_lines)?;
    let skipped_count = files.len() - filtered_files.len();

    if skipped_count > 0 {
        log::warn!("Skipped {} files exceeding {} lines", skipped_count, max_lines);
    }

    log::debug!("Files to analyze: {}", filtered_files.len());

    let mut all_definitions: Vec<(Definition, Language)> = Vec::new();
    let mut all_references: Vec<(Definition, Language)> = Vec::new();
    let mut languages_found = std::collections::HashMap::new();

    for file_path in &filtered_files {
        let mut parser = parsentry_parser::CodeParser::new()?;
        if let Err(e) = parser.add_file(file_path) {
            log::warn!("Failed to add file for parsing: {}: {}", file_path.display(), e);
            continue;
        }

        match parser.build_context_from_file(file_path) {
            Ok(context) => {
                let filename = file_path.to_string_lossy();
                let content = match std::fs::read_to_string(file_path) {
                    Ok(c) => c,
                    Err(e) => {
                        log::warn!("Failed to read file {}: {}", file_path.display(), e);
                        continue;
                    }
                };

                let language = FileClassifier::classify(&filename, &content);
                languages_found.insert(language, true);

                log::debug!(
                    "{} ({:?}): {} definitions, {} references",
                    file_path.display(),
                    language,
                    context.definitions.len(),
                    context.references.len()
                );

                for def in context.definitions {
                    all_definitions.push((def, language));
                }
                for ref_def in context.references {
                    all_references.push((ref_def, language));
                }
            }
            Err(e) => {
                log::warn!("Failed to collect context: {}: {}", file_path.display(), e);
                continue;
            }
        }
    }

    log::info!(
        "Extracted {} definitions, {} references",
        all_definitions.len(),
        all_references.len()
    );

    let executor = CodexExecutor::new(config)?;

    let mut total_patterns = 0;
    let mut processed_languages = Vec::new();

    for (language, _) in &languages_found {
        let lang_definitions: Vec<_> = all_definitions
            .iter()
            .filter(|(_, lang)| lang == language)
            .map(|(def, _)| def)
            .collect();

        let lang_references: Vec<_> = all_references
            .iter()
            .filter(|(_, lang)| lang == language)
            .map(|(def, _)| def)
            .collect();

        let total_items = lang_definitions.len() + lang_references.len();
        if total_items == 0 {
            continue;
        }

        log::info!(
            "Analyzing {:?}: {} definitions, {} references",
            language,
            lang_definitions.len(),
            lang_references.len()
        );

        let mut definition_patterns = Vec::new();
        let mut reference_patterns = Vec::new();

        if !lang_definitions.is_empty() {
            definition_patterns = analyze_definitions_with_codex(
                &executor,
                &lang_definitions,
                *language,
            )
            .await?;
        }

        if !lang_references.is_empty() {
            reference_patterns = analyze_references_with_codex(
                &executor,
                &lang_references,
                *language,
            )
            .await?;
        }

        let mut all_patterns = definition_patterns;
        all_patterns.extend(reference_patterns);

        let mut unique_patterns = Vec::new();
        let mut seen_functions = std::collections::HashSet::new();

        for pattern in all_patterns {
            if seen_functions.insert(pattern.function_name.clone()) {
                unique_patterns.push(pattern);
            }
        }

        if !unique_patterns.is_empty() {
            write_patterns_to_file(root_dir, *language, &unique_patterns)?;
            log::info!("{:?}: {} patterns generated", language, unique_patterns.len());
            total_patterns += unique_patterns.len();
            processed_languages.push(*language);
        } else {
            log::debug!("{:?}: no security patterns detected", language);
        }
    }

    log::info!("Pattern generation completed: {} patterns", total_patterns);

    Ok(PatternGenerationResult {
        total_files: files.len(),
        analyzed_files: filtered_files.len(),
        skipped_files: skipped_count,
        patterns_generated: total_patterns,
        languages: processed_languages,
    })
}

const MAX_DEFINITIONS_PER_BATCH: usize = 30;

async fn analyze_definitions_with_codex(
    executor: &CodexExecutor,
    definitions: &[&Definition],
    language: Language,
) -> Result<Vec<PatternClassification>> {
    if definitions.is_empty() {
        return Ok(Vec::new());
    }

    // Chunk definitions into smaller batches
    let chunks: Vec<_> = definitions.chunks(MAX_DEFINITIONS_PER_BATCH).collect();
    log::info!(
        "Splitting {} definitions into {} batches",
        definitions.len(),
        chunks.len()
    );

    let mut all_patterns = Vec::new();

    for (batch_idx, chunk) in chunks.iter().enumerate() {
        log::debug!(
            "Processing definition batch {}/{} ({} items)",
            batch_idx + 1,
            chunks.len(),
            chunk.len()
        );

        let mut definitions_context = String::new();
        for (idx, def) in chunk.iter().enumerate() {
            let location = def.file_path.as_ref()
                .map(|p| format!("{}:{}", p.display(), def.line_number.unwrap_or(0)))
                .unwrap_or_else(|| "unknown".to_string());
            definitions_context.push_str(&format!(
                "Definition {}: {}\nLocation: {}\n\n",
                idx + 1,
                def.name,
                location
            ));
        }

        let prompt = format!(
            r#"TASK: Analyze function definitions for security patterns and return JSON.

LANGUAGE: {language:?}

DEFINITIONS:
{definitions_context}

CLASSIFICATION RULES:
- "principals": Data entry points (user input, request data, environment vars)
- "actions": Security controls (validation, sanitization, auth checks)
- "resources": Sensitive operations (file/DB/network access, command execution)
- "none": Not security-relevant

Return JSON in this format:
{{
  "patterns": [
    {{
      "classification": "principals|actions|resources|none",
      "function_name": "exact_function_name",
      "query_type": "definition",
      "query": "(function_definition name: (identifier) @name (#eq? @name \"function_name\")) @function",
      "description": "What this pattern detects",
      "reasoning": "Classification rationale",
      "attack_vector": []
    }}
  ]
}}

REQUIREMENTS:
- All queries must have balanced parentheses
- All queries must contain capture names (@name, @function, etc)
- classification must be one of: principals, actions, resources, none
- query_type must be: definition
- Include ALL functions in patterns array"#,
            language = language,
            definitions_context = definitions_context,
        );

        match executor.execute_raw_with_retry(&prompt, 2).await {
            Ok(content) => {
                match parse_pattern_response(&content) {
                    Ok(patterns) => {
                        log::debug!("Batch {}: {} patterns found", batch_idx + 1, patterns.len());
                        all_patterns.extend(patterns);
                    }
                    Err(e) => {
                        log::warn!("Batch {} parse failed: {}", batch_idx + 1, e);
                    }
                }
            }
            Err(e) => {
                log::warn!("Batch {} execution failed: {}", batch_idx + 1, e);
            }
        }
    }

    Ok(all_patterns)
}

const MAX_REFERENCES_TOTAL: usize = 500;
const MAX_REFERENCES_PER_BATCH: usize = 50;

async fn analyze_references_with_codex(
    executor: &CodexExecutor,
    references: &[&Definition],
    language: Language,
) -> Result<Vec<PatternClassification>> {
    if references.is_empty() {
        return Ok(Vec::new());
    }

    // Skip if too many references - focus on definitions which are more important
    if references.len() > MAX_REFERENCES_TOTAL {
        log::warn!(
            "Skipping {} references (exceeds limit of {})",
            references.len(),
            MAX_REFERENCES_TOTAL
        );
        return Ok(Vec::new());
    }

    // Deduplicate references by name to reduce noise
    let mut unique_refs: std::collections::HashMap<String, &Definition> =
        std::collections::HashMap::new();
    for ref_def in references {
        unique_refs.entry(ref_def.name.clone()).or_insert(ref_def);
    }
    let unique_refs: Vec<_> = unique_refs.values().cloned().collect();

    log::info!(
        "Analyzing {} unique references (from {} total)",
        unique_refs.len(),
        references.len()
    );

    let chunks: Vec<_> = unique_refs.chunks(MAX_REFERENCES_PER_BATCH).collect();
    let mut all_patterns = Vec::new();

    for (batch_idx, chunk) in chunks.iter().enumerate() {
        log::debug!(
            "Processing reference batch {}/{} ({} items)",
            batch_idx + 1,
            chunks.len(),
            chunk.len()
        );

        let mut references_context = String::new();
        for (idx, ref_def) in chunk.iter().enumerate() {
            let location = ref_def.file_path.as_ref()
                .map(|p| format!("{}:{}", p.display(), ref_def.line_number.unwrap_or(0)))
                .unwrap_or_else(|| "unknown".to_string());
            references_context.push_str(&format!(
                "Reference {}: {}\nLocation: {}\n\n",
                idx + 1,
                ref_def.name,
                location
            ));
        }

        let prompt = format!(
            r#"TASK: Analyze function call references for security patterns and return JSON.

LANGUAGE: {language:?}

REFERENCES:
{references_context}

CLASSIFICATION RULES:
- "principals": Functions returning untrusted/user-controlled data
- "actions": Security processing functions (validation, sanitization, auth)
- "resources": Sensitive operation targets (file, DB, network, DOM, exec)
- "none": Not security-relevant

Return JSON in this format:
{{
  "patterns": [
    {{
      "classification": "principals|actions|resources|none",
      "function_name": "exact_function_name",
      "query_type": "reference",
      "query": "(call_expression function: (identifier) @name (#eq? @name \"function_name\")) @call",
      "description": "What this pattern detects",
      "reasoning": "Classification rationale",
      "attack_vector": []
    }}
  ]
}}

REQUIREMENTS:
- All queries must have balanced parentheses
- All queries must contain capture names (@name, @call, etc)
- classification must be one of: principals, actions, resources, none
- query_type must be: reference
- Include ALL functions in patterns array"#,
            language = language,
            references_context = references_context,
        );

        match executor.execute_raw_with_retry(&prompt, 2).await {
            Ok(content) => {
                match parse_pattern_response(&content) {
                    Ok(patterns) => {
                        log::debug!("Batch {}: {} patterns found", batch_idx + 1, patterns.len());
                        all_patterns.extend(patterns);
                    }
                    Err(e) => {
                        log::warn!("Batch {} parse failed: {}", batch_idx + 1, e);
                    }
                }
            }
            Err(e) => {
                log::warn!("Batch {} execution failed: {}", batch_idx + 1, e);
            }
        }
    }

    Ok(all_patterns)
}

fn parse_pattern_response(analysis: &str) -> Result<Vec<PatternClassification>> {
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
        #[serde(default)]
        attack_vector: Vec<String>,
    }

    log::debug!("Raw response length: {} bytes", analysis.len());
    log::debug!("Raw response preview: {}", &analysis[..analysis.len().min(500)]);

    // Try to extract JSON from markdown code blocks first
    let json_str = if let Some(json_start) = analysis.find("```json") {
        let content_start = json_start + 7;
        let remaining = &analysis[content_start..];
        if let Some(json_end) = remaining.find("```") {
            remaining[..json_end].trim()
        } else {
            analysis
        }
    } else if let Some(start) = analysis.find('{') {
        if let Some(end) = analysis.rfind('}') {
            &analysis[start..=end]
        } else {
            analysis
        }
    } else {
        analysis
    };

    log::debug!("Extracted JSON preview: {}", &json_str[..json_str.len().min(300)]);

    let response: BatchAnalysisResponse = serde_json::from_str(json_str).map_err(|e| {
        log::error!("Failed to parse response. Error: {}", e);
        log::error!("Extracted JSON: {}", json_str);
        anyhow::anyhow!("Failed to parse Codex response: {}", e)
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

    log::debug!("Codex analysis: {} security patterns detected", security_pattern_count);

    Ok(patterns)
}
