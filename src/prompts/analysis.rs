use crate::response::VulnType;
use std::collections::HashMap;

pub const README_SUMMARY_PROMPT_TEMPLATE: &str = r#"
Analyze the provided README content and create a concise summary that captures:
1. The project's main purpose and functionality
2. Key features and capabilities
3. Important technical details or requirements
4. Any security-related information

Format your response within <summary></summary> tags.
"#;

pub const SYS_PROMPT_TEMPLATE: &str = r#"
You are a security researcher analyzing code for potential vulnerabilities. Focus on:
1. Input validation and sanitization
2. Authentication and authorization
3. Data handling and exposure
4. Command injection possibilities
5. Path traversal vulnerabilities
6. Other security-critical patterns

Consider the project context from the README summary below when analyzing the code.
"#;

pub const INITIAL_ANALYSIS_PROMPT_TEMPLATE: &str = r#"
Analyze the provided code for potential security vulnerabilities. Consider:
1. User input handling and validation
2. Authentication and authorization mechanisms
3. Data sanitization and escaping
4. File system operations
5. Network requests and responses
6. Command execution
7. Database queries

Provide your analysis in a structured format with:
- Step-by-step analysis process
- Identified vulnerabilities
- Confidence level
- Supporting code context
"#;

pub const ANALYSIS_APPROACH_TEMPLATE: &str = r#"
Follow these steps in your analysis:
1. Identify entry points and user-controlled input
2. Trace data flow through the application
3. Examine security-critical operations
4. Consider bypass techniques for existing protections
5. Evaluate the impact of potential vulnerabilities
"#;

pub const GUIDELINES_TEMPLATE: &str = r#"
Adhere to these guidelines:
1. Focus on concrete vulnerabilities with clear exploitation paths
2. Provide specific code references and line numbers
3. Consider the full context of the application
4. Rate confidence based on code visibility and analysis depth
5. Request additional context if needed for better analysis
"#;

pub mod vuln_specific {
    use super::*;

    pub struct VulnTypeInfo {
        pub prompt: String,
        pub bypasses: Vec<String>,
    }

    pub fn get_vuln_specific_info() -> HashMap<VulnType, VulnTypeInfo> {
        let mut map = HashMap::new();

        map.insert(
            VulnType::LFI,
            VulnTypeInfo {
                prompt: "Analyze for Local File Inclusion vulnerabilities...".to_string(),
                bypasses: vec![
                    "Path traversal sequences (../../)".to_string(),
                    "URL encoding bypasses".to_string(),
                    "Null byte injection".to_string(),
                ],
            },
        );

        map.insert(
            VulnType::RCE,
            VulnTypeInfo {
                prompt: "Analyze for Remote Code Execution vulnerabilities...".to_string(),
                bypasses: vec![
                    "Command injection through shell metacharacters".to_string(),
                    "Python code execution vectors".to_string(),
                    "Deserialization attacks".to_string(),
                ],
            },
        );

        map.insert(
            VulnType::SSRF,
            VulnTypeInfo {
                prompt: "Analyze for Server-Side Request Forgery vulnerabilities...".to_string(),
                bypasses: vec![
                    "DNS rebinding attacks".to_string(),
                    "IP address encoding tricks".to_string(),
                    "Redirect chains".to_string(),
                ],
            },
        );

        map.insert(
            VulnType::AFO,
            VulnTypeInfo {
                prompt: "Analyze for Arbitrary File Operation vulnerabilities...".to_string(),
                bypasses: vec![
                    "Directory traversal sequences".to_string(),
                    "Symbolic link following".to_string(),
                    "Race conditions".to_string(),
                ],
            },
        );

        map.insert(
            VulnType::SQLI,
            VulnTypeInfo {
                prompt: "Analyze for SQL Injection vulnerabilities...".to_string(),
                bypasses: vec![
                    "UNION-based injection".to_string(),
                    "Boolean-based blind injection".to_string(),
                    "Time-based blind injection".to_string(),
                ],
            },
        );

        map.insert(
            VulnType::XSS,
            VulnTypeInfo {
                prompt: "Analyze for Cross-Site Scripting vulnerabilities...".to_string(),
                bypasses: vec![
                    "HTML entity encoding bypasses".to_string(),
                    "JavaScript template injection".to_string(),
                    "DOM-based XSS vectors".to_string(),
                ],
            },
        );

        map.insert(
            VulnType::IDOR,
            VulnTypeInfo {
                prompt: "Analyze for Insecure Direct Object Reference vulnerabilities..."
                    .to_string(),
                bypasses: vec![
                    "Parameter manipulation".to_string(),
                    "Horizontal privilege escalation".to_string(),
                    "Predictable resource locations".to_string(),
                ],
            },
        );

        map
    }
}
