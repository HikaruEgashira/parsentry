//! Vulnerability type definitions.

use serde::{Deserialize, Serialize};

/// Represents different types of security vulnerabilities.
#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub enum VulnType {
    /// Local File Inclusion
    LFI,
    /// Remote Code Execution
    RCE,
    /// Server-Side Request Forgery
    SSRF,
    /// Arbitrary File Operation
    AFO,
    /// SQL Injection
    SQLI,
    /// Cross-Site Scripting
    XSS,
    /// Insecure Direct Object Reference
    IDOR,
    /// Other vulnerability type
    Other(String),
}

impl std::fmt::Display for VulnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VulnType::LFI => write!(f, "LFI"),
            VulnType::RCE => write!(f, "RCE"),
            VulnType::SSRF => write!(f, "SSRF"),
            VulnType::AFO => write!(f, "AFO"),
            VulnType::SQLI => write!(f, "SQLI"),
            VulnType::XSS => write!(f, "XSS"),
            VulnType::IDOR => write!(f, "IDOR"),
            VulnType::Other(name) => write!(f, "{}", name),
        }
    }
}

impl VulnType {
    /// Get CWE (Common Weakness Enumeration) IDs for this vulnerability type
    pub fn cwe_ids(&self) -> Vec<String> {
        match self {
            VulnType::SQLI => vec!["CWE-89".to_string()],
            VulnType::XSS => vec!["CWE-79".to_string(), "CWE-80".to_string()],
            VulnType::RCE => vec![
                "CWE-77".to_string(),
                "CWE-78".to_string(),
                "CWE-94".to_string(),
            ],
            VulnType::LFI => vec!["CWE-22".to_string(), "CWE-98".to_string()],
            VulnType::SSRF => vec!["CWE-918".to_string()],
            VulnType::AFO => vec!["CWE-22".to_string(), "CWE-73".to_string()],
            VulnType::IDOR => vec!["CWE-639".to_string(), "CWE-284".to_string()],
            VulnType::Other(_) => vec![],
        }
    }

    /// Get MITRE ATT&CK technique IDs for this vulnerability type
    pub fn mitre_attack_ids(&self) -> Vec<String> {
        match self {
            VulnType::SQLI => vec!["T1190".to_string()],
            VulnType::XSS => vec!["T1190".to_string(), "T1185".to_string()],
            VulnType::RCE => vec!["T1190".to_string(), "T1059".to_string()],
            VulnType::LFI => vec!["T1083".to_string()],
            VulnType::SSRF => vec!["T1090".to_string()],
            VulnType::AFO => vec!["T1083".to_string(), "T1005".to_string()],
            VulnType::IDOR => vec!["T1190".to_string()],
            VulnType::Other(_) => vec![],
        }
    }

    /// Get OWASP Top 10 category for this vulnerability type
    pub fn owasp_categories(&self) -> Vec<String> {
        match self {
            VulnType::SQLI => vec!["A03:2021-Injection".to_string()],
            VulnType::XSS => vec!["A03:2021-Injection".to_string()],
            VulnType::RCE => vec!["A03:2021-Injection".to_string()],
            VulnType::LFI => vec!["A01:2021-Broken Access Control".to_string()],
            VulnType::SSRF => vec!["A10:2021-Server-Side Request Forgery".to_string()],
            VulnType::AFO => vec!["A01:2021-Broken Access Control".to_string()],
            VulnType::IDOR => vec!["A01:2021-Broken Access Control".to_string()],
            VulnType::Other(_) => vec![],
        }
    }

    /// Parse a string into a VulnType
    pub fn from_str(s: &str) -> Self {
        match s {
            "LFI" => VulnType::LFI,
            "RCE" => VulnType::RCE,
            "SSRF" => VulnType::SSRF,
            "AFO" => VulnType::AFO,
            "SQLI" => VulnType::SQLI,
            "XSS" => VulnType::XSS,
            "IDOR" => VulnType::IDOR,
            other => VulnType::Other(other.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vuln_type_display() {
        assert_eq!(format!("{}", VulnType::LFI), "LFI");
        assert_eq!(format!("{}", VulnType::Other("Custom".to_string())), "Custom");
    }

    #[test]
    fn test_cwe_ids() {
        assert!(!VulnType::SQLI.cwe_ids().is_empty());
        assert!(VulnType::Other("test".to_string()).cwe_ids().is_empty());
    }
}
