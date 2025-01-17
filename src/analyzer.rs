use anyhow::{Error, Result};
use rig::providers::openai::{self, Client};
use rig::completion::{Agent, Prompt};
use log::{debug, error, info, warn};
use regex::escape;
use serde::de::DeserializeOwned;
use std::path::PathBuf;

use crate::parser::CodeParser;
use crate::prompts::{self, vuln_specific};
use crate::response::{response_json_schema, Response};

fn create_api_client() -> (Client, Agent) {
    let client = Client::from_env();
    let agent = client
        .agent("gpt-4")
        .with_system_prompt("You are a security vulnerability analyzer. Reply in JSON format")
        .with_response_format(serde_json::to_string(&response_json_schema()).unwrap())
        .build();
    (client, agent)
}

async fn execute_chat_request(
    agent: &Agent,
    prompt: &str,
) -> Result<String> {
    let response = agent.prompt(prompt).await?;
    match response.content() {
        Some(content) => Ok(content.to_string()),
        None => {
            error!("Failed to get content text from chat response");
            Err(anyhow::anyhow!(
                "Failed to get content text from chat response"
            ))
        }
    }
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
) -> Result<Response, Error> {
    info!("Performing initial analysis of {}", file_path.display());

    let mut parser = CodeParser::new(None)?;

    for file in files {
        parser.add_file(file)?;
    }

    let content = std::fs::read_to_string(file_path)?;
    if content.is_empty() {
        return Ok(Response {
            scratchpad: String::new(),
            analysis: String::new(),
            poc: String::new(),
            confidence_score: 0,
            vulnerability_types: vec![],
            context_code: vec![],
        });
    }

    let prompt = format!(
        "File: {}\n\nContent:\n{}\n\n{}\n{}\n{}",
        file_path.display(),
        content,
        prompts::INITIAL_ANALYSIS_PROMPT_TEMPLATE,
        prompts::ANALYSIS_APPROACH_TEMPLATE,
        prompts::GUIDELINES_TEMPLATE,
    );

    let (client, agent) = create_api_client();
    let chat_content = execute_chat_request(&agent, &prompt).await?;
    let response: Response = parse_json_response(&chat_content)?;

    info!("Initial analysis complete");

    if response.confidence_score > 0 && !response.vulnerability_types.is_empty() {
        let vuln_info_map = vuln_specific::get_vuln_specific_info();

        for vuln_type in response.vulnerability_types.clone() {
            let vuln_info = vuln_info_map.get(&vuln_type).unwrap();

            let mut stored_code_definitions: Vec<(PathBuf, crate::parser::Definition)> = Vec::new();
            let mut previous_analysis = String::new();

            for _ in 0..7 {
                info!(
                    "Performing vuln-specific analysis iteration for {:?}",
                    vuln_type
                );

                let mut context_code = String::new();
                for (_, def) in &stored_code_definitions {
                    context_code.push_str(&format!(
                        "\nFunction: {}\nSource:\n{}\n",
                        def.name, def.source
                    ));
                }

                let prompt = format!(
                    "File: {}\n\nContent:\n{}\n\nContext Code:\n{}\n\nVulnerability Type: {:?}\n\nBypasses to Consider:\n{}\n\n{}\n{}\n{}\nPrevious Analysis:\n{}",
                    file_path.display(),
                    content,
                    context_code,
                    vuln_type,
                    vuln_info.bypasses.join("\n"),
                    vuln_info.prompt,
                    prompts::ANALYSIS_APPROACH_TEMPLATE,
                    prompts::GUIDELINES_TEMPLATE,
                    previous_analysis,
                );

                let (client, agent) = create_api_client();
                let chat_content = execute_chat_request(&agent, &prompt).await?;
                let vuln_response: Response = parse_json_response(&chat_content)?;

                if verbosity > 0 {
                    return Ok(vuln_response);
                }

                if vuln_response.context_code.is_empty() {
                    if verbosity == 0 {
                        return Ok(vuln_response);
                    }
                    break;
                }

                for context in vuln_response.context_code {
                    let escaped_name = escape(&context.name);
                    if !stored_code_definitions
                        .iter()
                        .any(|(_, def)| def.name == escaped_name)
                    {
                        match parser.find_definition(&escaped_name, file_path) {
                            Ok(Some(def)) => {
                                stored_code_definitions.push(def);
                            }
                            Ok(None) => {
                                warn!("No definition found for context: {}", escaped_name);
                                continue;
                            }
                            Err(e) => {
                                warn!(
                                    "Failed to extract code definition for context {}: {}",
                                    escaped_name, e
                                );
                                continue;
                            }
                        }
                    }
                }

                previous_analysis = vuln_response.analysis;

                if vuln_response.confidence_score >= 95 {
                    break;
                }
            }
        }
    }

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::response::{ContextCode, VulnType};
    use tempfile::NamedTempFile;

    #[cfg(feature = "snapshot-test")]
    #[tokio::test]
    async fn test_analyze_empty_file() -> Result<()> {
        let temp_file = NamedTempFile::new()?;

        let result = analyze_file(
            &PathBuf::from(temp_file.path()),
            "gpt-4o-mini",
            &[PathBuf::from(temp_file.path())],
            0,
        )
        .await?;

        assert_eq!(result.scratchpad, String::new());
        assert_eq!(result.analysis, String::new());
        assert_eq!(result.poc, String::new());
        assert_eq!(result.confidence_score, 0);
        assert!(result.vulnerability_types.is_empty());
        assert!(result.context_code.is_empty());

        Ok(())
    }

    #[cfg(feature = "snapshot-test")]
    #[tokio::test]
    async fn test_analyze_hardcoded_password() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        std::fs::write(
            temp_file.path(),
            r#"
fn main() {
    let password = "hardcoded_password";
    println!("{}", password);
}
"#,
        )?;

        let result = analyze_file(
            &PathBuf::from(temp_file.path()),
            "gpt-4o-mini",
            &[PathBuf::from(temp_file.path())],
            0,
        )
        .await?;

        assert!(!result.analysis.is_empty(), "Analysis should not be empty");
        assert!(
            result.confidence_score > 0,
            "Confidence score should be positive"
        );
        assert!(
            !result.vulnerability_types.is_empty(),
            "Should detect vulnerabilities"
        );
        assert!(
            !result.context_code.is_empty(),
            "Should include context code"
        );

        Ok(())
    }

    #[cfg(feature = "snapshot-test")]
    #[tokio::test]
    async fn test_analyze_authentication_function() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        std::fs::write(
            temp_file.path(),
            r#"
fn authenticate(input: &str) -> bool {
    let password = "hardcoded_password";
    input == password
}

fn main() {
    let user_input = "test";
    if authenticate(user_input) {
        println!("Authenticated!");
    }
}
"#,
        )?;

        let result = analyze_file(
            &PathBuf::from(temp_file.path()),
            "gpt-4o-mini",
            &[PathBuf::from(temp_file.path())],
            0,
        )
        .await?;

        assert!(!result.analysis.is_empty(), "Analysis should not be empty");
        assert!(
            result.confidence_score > 0,
            "Confidence score should be positive"
        );
        assert!(
            !result.vulnerability_types.is_empty(),
            "Should detect vulnerabilities"
        );
        assert!(
            !result.context_code.is_empty(),
            "Should include context code"
        );

        Ok(())
    }
}
