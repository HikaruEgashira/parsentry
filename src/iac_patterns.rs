use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IaCLanguage {
    Terraform,
    CloudFormation,
    Ansible,
    Kubernetes,
    Helm,
    Other,
}

impl IaCLanguage {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "tf" | "hcl" => Some(IaCLanguage::Terraform),
            "yaml" | "yml" => {
                // Context-dependent: could be Kubernetes, Ansible, etc.
                // Would need content analysis for precise detection
                Some(IaCLanguage::Kubernetes)
            }
            "json" => {
                // Could be CloudFormation - needs content analysis
                Some(IaCLanguage::CloudFormation)
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IaCVulnerabilityType {
    // Security Configuration Issues
    WeakAccessControl,      // Overly permissive IAM policies, security groups
    UnencryptedStorage,     // Missing encryption at rest/transit
    PublicExposure,         // Resources exposed to internet unintentionally
    WeakAuthentication,     // Default/weak passwords, missing MFA
    
    // Network Security
    OverlyPermissiveFirewall, // 0.0.0.0/0 access, all ports open
    InsecureProtocols,       // HTTP instead of HTTPS, unencrypted protocols
    MissingNetworkSegmentation, // Resources not properly isolated
    
    // Data Protection
    MissingBackups,         // No backup configuration
    WeakEncryption,         // Weak encryption algorithms, missing keys
    DataLeakage,            // Logs containing sensitive data, public buckets
    
    // Compliance and Governance
    MissingAuditTrails,     // No CloudTrail, logging disabled
    PolicyViolations,       // Violates organizational security policies
    ComplianceFailure,      // GDPR, HIPAA, SOX violations
    
    // Secret Management
    HardcodedSecrets,       // Passwords, API keys in code
    ImproperSecretHandling, // Secrets in outputs, logs
    
    // Resource Management
    MissingResourceProtection, // No deletion protection, force destroy
    PrivilegeEscalation,    // Excessive permissions, admin access
    MissingMonitoring,      // No alerting, monitoring disabled
}

#[derive(Debug, Clone, Deserialize)]
pub struct IaCPatternConfig {
    pub pattern: String,
    pub description: String,
    pub severity: Severity,
    pub vulnerability_type: String, // Maps to IaCVulnerabilityType
    pub remediation: Option<String>,
    pub compliance_frameworks: Option<Vec<String>>, // GDPR, HIPAA, etc.
}

#[derive(Debug, Clone, Deserialize)]
pub enum Severity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Deserialize)]
pub struct IaCLanguagePatterns {
    pub security_misconfigurations: Option<Vec<IaCPatternConfig>>,
    pub network_security: Option<Vec<IaCPatternConfig>>,
    pub data_protection: Option<Vec<IaCPatternConfig>>,
    pub access_control: Option<Vec<IaCPatternConfig>>,
    pub secret_management: Option<Vec<IaCPatternConfig>>,
    pub compliance: Option<Vec<IaCPatternConfig>>,
}

pub struct IaCSecurityPatterns {
    patterns_by_type: HashMap<IaCVulnerabilityType, Vec<CompiledPattern>>,
    language: IaCLanguage,
}

#[derive(Debug)]
struct CompiledPattern {
    regex: Regex,
    config: IaCPatternConfig,
}

impl IaCSecurityPatterns {
    pub fn new(language: IaCLanguage) -> Self {
        let pattern_map = Self::load_iac_patterns();
        let lang_patterns = pattern_map.get(&language).unwrap();
        
        let mut patterns_by_type = HashMap::new();
        
        // Process each category of patterns
        if let Some(configs) = &lang_patterns.security_misconfigurations {
            Self::add_patterns_to_map(&mut patterns_by_type, configs);
        }
        
        if let Some(configs) = &lang_patterns.network_security {
            Self::add_patterns_to_map(&mut patterns_by_type, configs);
        }
        
        if let Some(configs) = &lang_patterns.data_protection {
            Self::add_patterns_to_map(&mut patterns_by_type, configs);
        }
        
        if let Some(configs) = &lang_patterns.access_control {
            Self::add_patterns_to_map(&mut patterns_by_type, configs);
        }
        
        if let Some(configs) = &lang_patterns.secret_management {
            Self::add_patterns_to_map(&mut patterns_by_type, configs);
        }
        
        if let Some(configs) = &lang_patterns.compliance {
            Self::add_patterns_to_map(&mut patterns_by_type, configs);
        }
        
        Self {
            patterns_by_type,
            language,
        }
    }
    
    fn add_patterns_to_map(
        patterns_map: &mut HashMap<IaCVulnerabilityType, Vec<CompiledPattern>>,
        configs: &[IaCPatternConfig],
    ) {
        for config in configs {
            if let Ok(regex) = Regex::new(&config.pattern) {
                let vuln_type = Self::parse_vulnerability_type(&config.vulnerability_type);
                let compiled = CompiledPattern {
                    regex,
                    config: config.clone(),
                };
                
                patterns_map
                    .entry(vuln_type)
                    .or_insert_with(Vec::new)
                    .push(compiled);
            }
        }
    }
    
    fn parse_vulnerability_type(type_str: &str) -> IaCVulnerabilityType {
        match type_str {
            "weak_access_control" => IaCVulnerabilityType::WeakAccessControl,
            "unencrypted_storage" => IaCVulnerabilityType::UnencryptedStorage,
            "public_exposure" => IaCVulnerabilityType::PublicExposure,
            "hardcoded_secrets" => IaCVulnerabilityType::HardcodedSecrets,
            "overly_permissive_firewall" => IaCVulnerabilityType::OverlyPermissiveFirewall,
            "missing_audit_trails" => IaCVulnerabilityType::MissingAuditTrails,
            "missing_backups" => IaCVulnerabilityType::MissingBackups,
            "missing_resource_protection" => IaCVulnerabilityType::MissingResourceProtection,
            _ => IaCVulnerabilityType::WeakAccessControl, // Default fallback
        }
    }
    
    fn load_iac_patterns() -> HashMap<IaCLanguage, IaCLanguagePatterns> {
        // Load from iac_patterns.yml instead of security_patterns/patterns.yml
        let content = std::fs::read_to_string("iac_patterns/patterns.yml")
            .expect("Failed to read IaC patterns file");
        
        serde_yaml::from_str(&content)
            .expect("Failed to parse IaC patterns YAML")
    }
    
    pub fn scan_content(&self, content: &str) -> Vec<IaCSecurityFinding> {
        let mut findings = Vec::new();
        
        for (vuln_type, patterns) in &self.patterns_by_type {
            for pattern in patterns {
                for mat in pattern.regex.find_iter(content) {
                    let finding = IaCSecurityFinding {
                        vulnerability_type: *vuln_type,
                        pattern: pattern.config.pattern.clone(),
                        description: pattern.config.description.clone(),
                        severity: pattern.config.severity.clone(),
                        line_number: content[..mat.start()].lines().count(),
                        matched_text: mat.as_str().to_string(),
                        remediation: pattern.config.remediation.clone(),
                        compliance_frameworks: pattern.config.compliance_frameworks.clone(),
                    };
                    findings.push(finding);
                }
            }
        }
        
        findings
    }
}

#[derive(Debug, Clone)]
pub struct IaCSecurityFinding {
    pub vulnerability_type: IaCVulnerabilityType,
    pub pattern: String,
    pub description: String,
    pub severity: Severity,
    pub line_number: usize,
    pub matched_text: String,
    pub remediation: Option<String>,
    pub compliance_frameworks: Option<Vec<String>>,
}

// Resource relationship analysis for IaC
#[derive(Debug)]
pub struct IaCResourceGraph {
    pub resources: HashMap<String, IaCResource>,
    pub dependencies: Vec<(String, String)>, // (from, to) resource dependencies
}

#[derive(Debug)]
pub struct IaCResource {
    pub resource_type: String,
    pub name: String,
    pub properties: HashMap<String, String>,
    pub security_implications: Vec<String>,
}

impl IaCResourceGraph {
    pub fn analyze_cross_resource_vulnerabilities(&self) -> Vec<CrossResourceVulnerability> {
        let mut vulnerabilities = Vec::new();
        
        // Example: Database with public access + overly permissive security group
        for (db_id, db_resource) in &self.resources {
            if db_resource.resource_type.contains("database") 
                && db_resource.properties.get("publicly_accessible") == Some(&"true".to_string()) {
                
                // Find associated security groups
                for (sg_id, sg_resource) in &self.resources {
                    if sg_resource.resource_type.contains("security_group") 
                        && self.are_resources_connected(db_id, sg_id) {
                        
                        if Self::is_overly_permissive_sg(sg_resource) {
                            vulnerabilities.push(CrossResourceVulnerability {
                                primary_resource: db_id.clone(),
                                secondary_resource: sg_id.clone(),
                                vulnerability_type: IaCVulnerabilityType::PublicExposure,
                                description: "Public database with overly permissive security group".to_string(),
                                severity: Severity::Critical,
                            });
                        }
                    }
                }
            }
        }
        
        vulnerabilities
    }
    
    fn are_resources_connected(&self, resource1: &str, resource2: &str) -> bool {
        self.dependencies.iter().any(|(from, to)| {
            (from == resource1 && to == resource2) || (from == resource2 && to == resource1)
        })
    }
    
    fn is_overly_permissive_sg(sg_resource: &IaCResource) -> bool {
        sg_resource.properties.get("cidr_blocks")
            .map(|cidr| cidr.contains("0.0.0.0/0"))
            .unwrap_or(false)
    }
}

#[derive(Debug)]
pub struct CrossResourceVulnerability {
    pub primary_resource: String,
    pub secondary_resource: String,
    pub vulnerability_type: IaCVulnerabilityType,
    pub description: String,
    pub severity: Severity,
}