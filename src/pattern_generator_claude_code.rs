//! Claude Code CLI-based pattern generation
//!
//! This module provides pattern generation using Claude Code CLI instead of API calls.

use anyhow::Result;
use serde::Deserialize;
use std::path::Path;

use parsentry_analyzer::{filter_files_by_size, write_patterns_to_file, PatternClassification};
use parsentry_claude_code::{ClaudeCodeConfig, ClaudeCodeExecutor};
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

/// Generate custom security patterns using Claude Code CLI
pub async fn generate_custom_patterns_with_claude_code(
    root_dir: &Path,
    config: ClaudeCodeConfig,
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

    let executor = ClaudeCodeExecutor::new(config)?;

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
            definition_patterns = analyze_definitions_with_claude_code(
                &executor,
                &lang_definitions,
                *language,
            )
            .await?;
        }

        if !lang_references.is_empty() {
            reference_patterns = analyze_references_with_claude_code(
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

async fn analyze_definitions_with_claude_code(
    executor: &ClaudeCodeExecutor,
    definitions: &[&Definition],
    language: Language,
) -> Result<Vec<PatternClassification>> {
    if definitions.is_empty() {
        return Ok(Vec::new());
    }

    let mut definitions_context = String::new();
    for (idx, def) in definitions.iter().enumerate() {
        definitions_context.push_str(&format!(
            "Definition {}: {}\nCode:\n{}\n\n",
            idx + 1,
            def.name,
            def.source
        ));
    }

    let prompt = format!(
        r#"Analyze these function definitions from a {:?} codebase and determine which represent security patterns.

{}

For each function, determine if it should be classified as:
- "principals": Sources that act as data entry points and should be treated as tainted/untrusted
- "actions": Functions that perform validation, sanitization, authorization, or security operations
- "resources": Functions that access, modify, or perform operations on files, databases, networks, or system resources
- "none": Not a security pattern

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

All fields are required for each object. Use proper tree-sitter query syntax for the {:?} language."#,
        language, definitions_context, language
    );

    let output = executor.execute_with_retry(&prompt, 2).await?;
    parse_pattern_response(&output.response.analysis)
}

async fn analyze_references_with_claude_code(
    executor: &ClaudeCodeExecutor,
    references: &[&Definition],
    language: Language,
) -> Result<Vec<PatternClassification>> {
    if references.is_empty() {
        return Ok(Vec::new());
    }

    let mut references_context = String::new();
    for (idx, ref_def) in references.iter().enumerate() {
        references_context.push_str(&format!(
            "Reference {}: {}\nCode:\n{}\n\n",
            idx + 1,
            ref_def.name,
            ref_def.source
        ));
    }

    let prompt = format!(
        r#"Analyze these function references/calls from a {:?} codebase and determine which represent calls to security-sensitive functions.

{}

For each function reference, determine if it should be classified as:
- "principals": Functions that return or provide untrusted data that attackers can control
- "actions": Functions that perform security processing (validation, sanitization, authorization)
- "resources": Functions that operate on attack targets (files, databases, system commands, DOM)
- "none": Not a security-relevant call

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

All fields are required for each object. Use proper tree-sitter query syntax for the {:?} language."#,
        language, references_context, language
    );

    let output = executor.execute_with_retry(&prompt, 2).await?;
    parse_pattern_response(&output.response.analysis)
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
        attack_vector: Vec<String>,
    }

    let json_str = if let Some(start) = analysis.find('{') {
        if let Some(end) = analysis.rfind('}') {
            &analysis[start..=end]
        } else {
            analysis
        }
    } else {
        analysis
    };

    let response: BatchAnalysisResponse = serde_json::from_str(json_str).map_err(|e| {
        log::debug!("Failed response preview: {}", &analysis[..analysis.len().min(200)]);
        anyhow::anyhow!("Failed to parse Claude Code response: {}", e)
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

    log::debug!("Claude Code analysis: {} security patterns detected", security_pattern_count);

    Ok(patterns)
}
