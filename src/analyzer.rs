use anyhow::{Error, Result};
use genai::chat::{ChatMessage, ChatOptions, ChatRequest, JsonSpec};
use genai::{Client, ClientConfig};
use log::{debug, error, info, warn};
use regex::escape;
use serde::de::DeserializeOwned;
use std::path::PathBuf;
use std::time::Duration;
use tokio::time::timeout;

use crate::parser::CodeParser;
use crate::prompts::{self, vuln_specific};
use crate::response::{Response, response_json_schema};
use crate::security_patterns::{Language, PatternType, SecurityRiskPatterns};

fn create_api_client() -> Client {
    let client_config = ClientConfig::default().with_chat_options(
        ChatOptions::default()
            .with_response_format(JsonSpec::new("json_object", response_json_schema())),
    );
    Client::builder().with_config(client_config).build()
}

async fn execute_chat_request(
    client: &Client,
    model: &str,
    chat_req: ChatRequest,
) -> Result<String> {
    // 30秒のタイムアウトを設定
    let result = timeout(Duration::from_secs(30), async {
        client.exec_chat(model, chat_req, None).await
    }).await;

    match result {
        Ok(Ok(chat_res)) => {
            match chat_res.content_text_as_str() {
                Some(content) => Ok(content.to_string()),
                None => {
                    error!("Failed to get content text from chat response");
                    Err(anyhow::anyhow!(
                        "Failed to get content text from chat response"
                    ))
                }
            }
        }
        Ok(Err(e)) => {
            error!("Chat request failed: {}", e);
            Err(e.into())
        }
        Err(_) => {
            error!("Chat request timed out after 30 seconds");
            Err(anyhow::anyhow!("Chat request timed out after 30 seconds"))
        }
    }
}

async fn execute_chat_request_with_retry(
    client: &Client,
    model: &str,
    chat_req: ChatRequest,
    max_retries: u32,
) -> Result<String> {
    let mut last_error = None;
    
    for attempt in 0..=max_retries {
        if attempt > 0 {
            warn!("Retrying chat request (attempt {}/{})", attempt + 1, max_retries + 1);
            // 指数バックオフで待機
            tokio::time::sleep(Duration::from_millis(1000 * (1 << attempt))).await;
        }
        
        match execute_chat_request(client, model, chat_req.clone()).await {
            Ok(result) => return Ok(result),
            Err(e) => {
                warn!("Chat request failed on attempt {}: {}", attempt + 1, e);
                last_error = Some(e);
            }
        }
    }
    
    Err(last_error.unwrap_or_else(|| anyhow::anyhow!("All retry attempts failed")))
}

fn parse_json_response<T: DeserializeOwned>(chat_content: &str) -> Result<T> {
    match serde_json::from_str(chat_content) {
        Ok(response) => Ok(response),
        Err(e) => {
            debug!("Failed to parse JSON response: {}", e);
            debug!("Response content: {}", chat_content);
            Err(anyhow::anyhow!("Failed to parse JSON response: {}", e))
        }
    }
}

pub async fn analyze_file(
    file_path: &PathBuf,
    model: &str,
    files: &[PathBuf],
    verbosity: u8,
    context: &crate::parser::Context,
    min_confidence: i32,
) -> Result<Response, Error> {
    info!("Performing initial analysis of {}", file_path.display());

    let mut parser = CodeParser::new()?;

    for file in files {
        if let Err(e) = parser.add_file(file) {
            warn!(
                "Failed to add file to parser {}: {}. Skipping file.",
                file.display(),
                e
            );
        }
    }

    let content = std::fs::read_to_string(file_path)?;
    if content.is_empty() {
        return Ok(Response {
            scratchpad: String::new(),
            analysis: String::new(),
            poc: String::new(),
            confidence_score: 0,
            vulnerability_types: vec![],
            par_analysis: crate::response::ParAnalysis {
                principals: vec![],
                actions: vec![],
                resources: vec![],
                policy_violations: vec![],
            },
            remediation_guidance: crate::response::RemediationGuidance {
                policy_enforcement: vec![],
            },
        });
    }

    let mut context_text = String::new();
    if !context.definitions.is_empty() {
        context_text.push_str("\nContext Definitions:\n");
        for def in &context.definitions {
            context_text.push_str(&format!(
                "\nFunction/Definition: {}\nCode:\n{}\n",
                def.name, def.source
            ));
        }
    }

    let prompt = format!(
        "File: {}\n\nContent:\n{}\n{}\n\n{}\n{}\n{}",
        file_path.display(),
        content,
        context_text,
        prompts::INITIAL_ANALYSIS_PROMPT_TEMPLATE,
        prompts::ANALYSIS_APPROACH_TEMPLATE,
        prompts::GUIDELINES_TEMPLATE,
    );
    debug!("[PROMPT]\n{}", prompt);

    let chat_req = ChatRequest::new(vec![
        ChatMessage::system(
            "You are a security vulnerability analyzer. You must reply with exactly one JSON object that matches the PAR analysis schema with scratchpad, analysis, poc, confidence_score, vulnerability_types, par_analysis (with principals, actions, resources, policy_violations), and remediation_guidance fields. Do not include any explanatory text outside the JSON object.",
        ),
        ChatMessage::user(&prompt),
    ]);

    let json_client = create_api_client();
    let chat_content = execute_chat_request(&json_client, model, chat_req).await?;
    debug!("[LLM Response]\n{}", chat_content);
    let mut response: Response = parse_json_response(&chat_content)?;

    response.confidence_score =
        crate::response::Response::normalize_confidence_score(response.confidence_score);
    
    // Clean up and validate the response
    response.sanitize();

    info!("Initial analysis complete");

    if response.confidence_score >= min_confidence && !response.vulnerability_types.is_empty() {
        let vuln_info_map = vuln_specific::get_vuln_specific_info();

        for vuln_type in response.vulnerability_types.clone() {
            let vuln_info = vuln_info_map.get(&vuln_type).unwrap();

            let mut stored_code_definitions: Vec<(PathBuf, crate::parser::Definition)> = Vec::new();

            {
                info!("Performing vuln-specific analysis for {:?}", vuln_type);
                if verbosity > 0 {
                    println!(
                        "🔎 [{}] 脆弱性タイプ: {:?} の詳細解析",
                        file_path.display(),
                        vuln_type
                    );
                    if !stored_code_definitions.is_empty() {
                        println!("  解析コンテキスト関数:");
                        for (_, def) in &stored_code_definitions {
                            println!("    - {} ({}行)", def.name, def.source.lines().count());
                        }
                    }
                    println!("  考慮バイパス: {}", vuln_info.bypasses.join(", "));
                    println!(
                        "  追加プロンプト: {}",
                        &vuln_info.prompt.chars().take(40).collect::<String>()
                    );
                }

                let mut context_code = String::new();
                for (_, def) in &stored_code_definitions {
                    context_code.push_str(&format!(
                        "\nFunction: {}\nSource:\n{}\n",
                        def.name, def.source
                    ));
                }

                let prompt = format!(
                    "File: {}\n\nContent:\n{}\n\nContext Code:\n{}\n\nVulnerability Type: {:?}\n\nBypasses to Consider:\n{}\n\n{}\n{}\n{}",
                    file_path.display(),
                    content,
                    context_code,
                    vuln_type,
                    vuln_info.bypasses.join("\n"),
                    vuln_info.prompt,
                    prompts::ANALYSIS_APPROACH_TEMPLATE,
                    prompts::GUIDELINES_TEMPLATE,
                );

                let chat_req = ChatRequest::new(vec![
                    ChatMessage::system(
                        "You are a security vulnerability analyzer. You must reply with exactly one JSON object that matches this schema: { \"scratchpad\": string, \"analysis\": string, \"poc\": string, \"confidence_score\": integer, \"vulnerability_types\": array of strings, \"context_code\": array of objects with { \"name\": string, \"reason\": string, \"code_line\": string } }. Do not include any explanatory text outside the JSON object.",
                    ),
                    ChatMessage::user(&prompt),
                ]);

                let json_client = create_api_client();
                let chat_content = execute_chat_request_with_retry(&json_client, model, chat_req, 2).await?;
                debug!("[LLM Response]\n{}", chat_content);
                let mut vuln_response: Response = parse_json_response(&chat_content)?;

                vuln_response.confidence_score =
                    crate::response::Response::normalize_confidence_score(
                        vuln_response.confidence_score,
                    );

                if verbosity > 0 {
                    debug!(
                        "  LLM応答: confidence_score={}, vulnerability_types={:?}",
                        vuln_response.confidence_score, vuln_response.vulnerability_types
                    );
                    println!(
                        "  analysis要約: {}",
                        &vuln_response.analysis.chars().take(60).collect::<String>()
                    );
                    if !vuln_response.par_analysis.policy_violations.is_empty() {
                        println!("  policy_violations:");
                        for violation in &vuln_response.par_analysis.policy_violations {
                            println!("    - {}: {}", violation.rule_id, violation.rule_description);
                        }
                    }
                    return Ok(vuln_response);
                }

                if vuln_response.par_analysis.policy_violations.is_empty() {
                    if verbosity == 0 {
                        return Ok(vuln_response);
                    }
                    break;
                }

                // Get language for pattern detection
                let file_extension = file_path
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap_or("");
                let language = Language::from_extension(file_extension);
                let _patterns = SecurityRiskPatterns::new(language);

                // Extract identifiers from PAR analysis for context building
                let mut identifiers_to_search = Vec::new();
                
                for principal in &vuln_response.par_analysis.principals {
                    identifiers_to_search.push((principal.identifier.clone(), PatternType::Principal));
                }
                for action in &vuln_response.par_analysis.actions {
                    identifiers_to_search.push((action.identifier.clone(), PatternType::Action));
                }
                for resource in &vuln_response.par_analysis.resources {
                    identifiers_to_search.push((resource.identifier.clone(), PatternType::Resource));
                }
                
                for (identifier, pattern_type) in identifiers_to_search {
                    let escaped_name = escape(&identifier);
                    if !stored_code_definitions
                        .iter()
                        .any(|(_, def)| def.name == escaped_name)
                    {

                        match pattern_type {
                            PatternType::Principal => {
                                // For principals, use find_references to track data flow forward
                                match parser.find_references(&escaped_name) {
                                    Ok(refs) => {
                                        stored_code_definitions.extend(refs);
                                    }
                                    Err(e) => {
                                        warn!(
                                            "Failed to find references for principal context {}: {}",
                                            escaped_name, e
                                        );
                                    }
                                }
                            }
                            PatternType::Action => {
                                // For actions, use bidirectional tracking to understand both input and output
                                match parser.find_bidirectional(&escaped_name, file_path) {
                                    Ok(bidirectional_results) => {
                                        stored_code_definitions.extend(bidirectional_results);
                                    }
                                    Err(e) => {
                                        warn!(
                                            "Failed to find bidirectional context for action {}: {}",
                                            escaped_name, e
                                        );
                                    }
                                }
                            }
                            PatternType::Resource => {
                                // For resources, use find_definition to track data origin
                                match parser.find_definition(&escaped_name, file_path) {
                                    Ok(Some(def)) => {
                                        stored_code_definitions.push(def);
                                    }
                                    Ok(None) => {
                                        debug!("No definition found for context: {}", escaped_name);
                                    }
                                    Err(e) => {
                                        warn!(
                                            "Failed to find definition for context {}: {}",
                                            escaped_name, e
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(response)
}
