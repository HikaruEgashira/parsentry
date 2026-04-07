//! Vulnerability type definitions.

use serde::{Deserialize, Serialize};
use std::str::FromStr;

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

impl FromStr for VulnType {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "LFI" => VulnType::LFI,
            "RCE" => VulnType::RCE,
            "SSRF" => VulnType::SSRF,
            "AFO" => VulnType::AFO,
            "SQLI" => VulnType::SQLI,
            "XSS" => VulnType::XSS,
            "IDOR" => VulnType::IDOR,
            other => VulnType::Other(other.to_string()),
        })
    }
}

impl VulnType {
    /// Get CWE (Common Weakness Enumeration) IDs for this vulnerability type.
    #[must_use]
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

    /// Get MITRE ATT&CK technique IDs for this vulnerability type.
    #[must_use]
    pub fn mitre_attack_ids(&self) -> Vec<String> {
        match self {
            VulnType::SQLI | VulnType::IDOR => vec!["T1190".to_string()],
            VulnType::XSS => vec!["T1190".to_string(), "T1185".to_string()],
            VulnType::RCE => vec!["T1190".to_string(), "T1059".to_string()],
            VulnType::LFI => vec!["T1083".to_string()],
            VulnType::SSRF => vec!["T1090".to_string()],
            VulnType::AFO => vec!["T1083".to_string(), "T1005".to_string()],
            VulnType::Other(_) => vec![],
        }
    }

    /// Get OWASP Top 10 category for this vulnerability type.
    #[must_use]
    pub fn owasp_categories(&self) -> Vec<String> {
        match self {
            VulnType::SQLI | VulnType::XSS | VulnType::RCE => {
                vec!["A03:2021-Injection".to_string()]
            }
            VulnType::LFI | VulnType::AFO | VulnType::IDOR => {
                vec!["A01:2021-Broken Access Control".to_string()]
            }
            VulnType::SSRF => vec!["A10:2021-Server-Side Request Forgery".to_string()],
            VulnType::Other(_) => vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vuln_type_display() {
        assert_eq!(format!("{}", VulnType::LFI), "LFI");
        assert_eq!(
            format!("{}", VulnType::Other("Custom".to_string())),
            "Custom"
        );
    }

    #[test]
    fn test_cwe_ids() {
        assert!(!VulnType::SQLI.cwe_ids().is_empty());
        assert!(VulnType::Other("test".to_string()).cwe_ids().is_empty());
    }

    #[test]
    fn test_from_str() {
        assert_eq!("LFI".parse::<VulnType>().unwrap(), VulnType::LFI);
        assert_eq!("RCE".parse::<VulnType>().unwrap(), VulnType::RCE);
        assert_eq!(
            "Unknown".parse::<VulnType>().unwrap(),
            VulnType::Other("Unknown".to_string())
        );
    }

    // --- Mutant-killing: test every FromStr arm ---

    #[test]
    fn test_from_str_all_variants() {
        assert_eq!("SSRF".parse::<VulnType>().unwrap(), VulnType::SSRF);
        assert_eq!("AFO".parse::<VulnType>().unwrap(), VulnType::AFO);
        assert_eq!("SQLI".parse::<VulnType>().unwrap(), VulnType::SQLI);
        assert_eq!("XSS".parse::<VulnType>().unwrap(), VulnType::XSS);
        assert_eq!("IDOR".parse::<VulnType>().unwrap(), VulnType::IDOR);
    }

    // --- Mutant-killing: test mitre_attack_ids for each variant ---

    #[test]
    fn test_mitre_attack_ids_sqli() {
        let ids = VulnType::SQLI.mitre_attack_ids();
        assert_eq!(ids, vec!["T1190"]);
    }

    #[test]
    fn test_mitre_attack_ids_idor() {
        let ids = VulnType::IDOR.mitre_attack_ids();
        assert_eq!(ids, vec!["T1190"]);
    }

    #[test]
    fn test_mitre_attack_ids_xss() {
        let ids = VulnType::XSS.mitre_attack_ids();
        assert_eq!(ids, vec!["T1190", "T1185"]);
    }

    #[test]
    fn test_mitre_attack_ids_rce() {
        let ids = VulnType::RCE.mitre_attack_ids();
        assert_eq!(ids, vec!["T1190", "T1059"]);
    }

    #[test]
    fn test_mitre_attack_ids_lfi() {
        let ids = VulnType::LFI.mitre_attack_ids();
        assert_eq!(ids, vec!["T1083"]);
    }

    #[test]
    fn test_mitre_attack_ids_ssrf() {
        let ids = VulnType::SSRF.mitre_attack_ids();
        assert_eq!(ids, vec!["T1090"]);
    }

    #[test]
    fn test_mitre_attack_ids_afo() {
        let ids = VulnType::AFO.mitre_attack_ids();
        assert_eq!(ids, vec!["T1083", "T1005"]);
    }

    #[test]
    fn test_mitre_attack_ids_other() {
        let ids = VulnType::Other("custom".to_string()).mitre_attack_ids();
        assert!(ids.is_empty());
    }

    // --- Mutant-killing: test owasp_categories for each variant ---

    #[test]
    fn test_owasp_categories_injection_group() {
        for vt in &[VulnType::SQLI, VulnType::XSS, VulnType::RCE] {
            let cats = vt.owasp_categories();
            assert_eq!(cats, vec!["A03:2021-Injection"], "Failed for {:?}", vt);
        }
    }

    #[test]
    fn test_owasp_categories_broken_access_control_group() {
        for vt in &[VulnType::LFI, VulnType::AFO, VulnType::IDOR] {
            let cats = vt.owasp_categories();
            assert_eq!(
                cats,
                vec!["A01:2021-Broken Access Control"],
                "Failed for {:?}",
                vt
            );
        }
    }

    #[test]
    fn test_owasp_categories_ssrf() {
        let cats = VulnType::SSRF.owasp_categories();
        assert_eq!(cats, vec!["A10:2021-Server-Side Request Forgery"]);
    }

    #[test]
    fn test_owasp_categories_other() {
        let cats = VulnType::Other("x".to_string()).owasp_categories();
        assert!(cats.is_empty());
    }

    // --- Mutant-killing: test cwe_ids for each variant ---

    #[test]
    fn test_cwe_ids_all_variants() {
        assert_eq!(VulnType::SQLI.cwe_ids(), vec!["CWE-89"]);
        assert_eq!(VulnType::XSS.cwe_ids(), vec!["CWE-79", "CWE-80"]);
        assert_eq!(VulnType::RCE.cwe_ids(), vec!["CWE-77", "CWE-78", "CWE-94"]);
        assert_eq!(VulnType::LFI.cwe_ids(), vec!["CWE-22", "CWE-98"]);
        assert_eq!(VulnType::SSRF.cwe_ids(), vec!["CWE-918"]);
        assert_eq!(VulnType::AFO.cwe_ids(), vec!["CWE-22", "CWE-73"]);
        assert_eq!(VulnType::IDOR.cwe_ids(), vec!["CWE-639", "CWE-284"]);
        assert!(VulnType::Other("z".to_string()).cwe_ids().is_empty());
    }

    // --- Mutant-killing: test Display for all variants ---

    #[test]
    fn test_display_all_variants() {
        assert_eq!(format!("{}", VulnType::RCE), "RCE");
        assert_eq!(format!("{}", VulnType::SSRF), "SSRF");
        assert_eq!(format!("{}", VulnType::AFO), "AFO");
        assert_eq!(format!("{}", VulnType::SQLI), "SQLI");
        assert_eq!(format!("{}", VulnType::XSS), "XSS");
        assert_eq!(format!("{}", VulnType::IDOR), "IDOR");
    }
}
