use crate::locales::Language;
use crate::locales;
use crate::response::VulnType;
use std::collections::HashMap;

pub fn get_sys_prompt_template(lang: &Language) -> &'static str {
    locales::get_sys_prompt_template(lang)
}

pub fn get_initial_analysis_prompt_template(lang: &Language) -> &'static str {
    locales::get_initial_analysis_prompt_template(lang)
}

pub fn get_analysis_approach_template(lang: &Language) -> &'static str {
    locales::get_analysis_approach_template(lang)
}

pub fn get_guidelines_template(lang: &Language) -> &'static str {
    locales::get_guidelines_template(lang)
}

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
                prompt: "Analyze for Local File Inclusion vulnerabilities. Look for file operations that accept user input without proper validation, especially path traversal patterns and insufficient input sanitization.".to_string(),
                bypasses: vec![
                    "Path traversal sequences(../../)".to_string(),
                    "URL encoding".to_string(),
                    "Null byte injection".to_string(),
                ],
            },
        );

        map.insert(
            VulnType::RCE,
            VulnTypeInfo {
                prompt: "Analyze for Remote Code Execution vulnerabilities...".to_string(),
                bypasses: vec![
                    "Shell metacharacters for command injection".to_string(),
                    "Python execution vectors".to_string(),
                    "Deserialization attacks".to_string(),
                ],
            },
        );

        map.insert(
            VulnType::SSRF,
            VulnTypeInfo {
                prompt: "Analyze for Server-Side Request Forgery vulnerabilities...".to_string(),
                bypasses: vec![
                    "DNS rebinding".to_string(),
                    "IP address encoding tricks".to_string(),
                    "Redirect chain".to_string(),
                ],
            },
        );

        map.insert(
            VulnType::AFO,
            VulnTypeInfo {
                prompt: "Analyze for Arbitrary File Operation vulnerabilities...".to_string(),
                bypasses: vec![
                    "Directory traversal sequences".to_string(),
                    "Following symbolic links".to_string(),
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
                    "HTML entity encoding bypass".to_string(),
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
                    "Parameter tampering".to_string(),
                    "Horizontal privilege escalation".to_string(),
                    "Predictable resource paths".to_string(),
                ],
            },
        );

        map
    }
}

