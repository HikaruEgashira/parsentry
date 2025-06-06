use anyhow::{Error, Result};
use genai::chat::{ChatMessage, ChatRequest};
use genai::Client;
use log::info;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::iac_patterns::{
    IaCLanguage, IaCSecurityPatterns, IaCSecurityFinding, IaCResourceGraph, 
    IaCVulnerabilityType, Severity, CrossResourceVulnerability
};
use crate::parser::CodeParser;
use crate::response::Response;

pub struct IaCAnalyzer {
    security_patterns: HashMap<IaCLanguage, IaCSecurityPatterns>,
    client: Client,
}

impl IaCAnalyzer {
    pub fn new(client: Client) -> Self {
        let mut security_patterns = HashMap::new();
        
        // Initialize patterns for each IaC language
        security_patterns.insert(IaCLanguage::Terraform, IaCSecurityPatterns::new(IaCLanguage::Terraform));
        security_patterns.insert(IaCLanguage::CloudFormation, IaCSecurityPatterns::new(IaCLanguage::CloudFormation));
        security_patterns.insert(IaCLanguage::Kubernetes, IaCSecurityPatterns::new(IaCLanguage::Kubernetes));
        
        Self {
            security_patterns,
            client,
        }
    }
    
    pub async fn analyze_iac_file(
        &self,
        file_path: &PathBuf,
        model: &str,
        context_files: &[PathBuf],
        _min_confidence: i32,
    ) -> Result<IaCAnalysisResult, Error> {
        info!("Analyzing IaC file: {}", file_path.display());
        
        // Detect IaC language
        let iac_language = self.detect_iac_language(file_path)?;
        let content = std::fs::read_to_string(file_path)?;
        
        if content.is_empty() {
            return Ok(IaCAnalysisResult::empty());
        }
        
        // Step 1: Pattern-based static analysis
        let static_findings = self.perform_static_analysis(&content, iac_language)?;
        
        // Step 2: Resource relationship analysis
        let resource_graph = self.build_resource_graph(&content, iac_language, context_files)?;
        let cross_resource_vulns = resource_graph.analyze_cross_resource_vulnerabilities();
        
        // Step 3: LLM-enhanced analysis for complex configurations
        let llm_analysis = self.perform_llm_analysis(
            file_path,
            &content,
            &static_findings,
            &cross_resource_vulns,
            model,
            iac_language,
        ).await?;
        
        // Step 4: Compliance checking
        let compliance_violations = self.check_compliance(&static_findings, &cross_resource_vulns);
        
        Ok(IaCAnalysisResult {
            file_path: file_path.clone(),
            language: iac_language,
            static_findings,
            cross_resource_vulnerabilities: cross_resource_vulns,
            llm_analysis,
            compliance_violations,
            resource_graph,
        })
    }
    
    fn detect_iac_language(&self, file_path: &PathBuf) -> Result<IaCLanguage> {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            if let Some(lang) = IaCLanguage::from_extension(ext) {
                return Ok(lang);
            }
        }
        
        // Content-based detection for ambiguous extensions
        let content = std::fs::read_to_string(file_path)?;
        
        if content.contains("terraform {") || content.contains("provider \"") {
            return Ok(IaCLanguage::Terraform);
        }
        
        if content.contains("AWSTemplateFormatVersion") || content.contains("Resources:") {
            return Ok(IaCLanguage::CloudFormation);
        }
        
        if content.contains("apiVersion:") && content.contains("kind:") {
            return Ok(IaCLanguage::Kubernetes);
        }
        
        Ok(IaCLanguage::Other)
    }
    
    fn perform_static_analysis(
        &self,
        content: &str,
        language: IaCLanguage,
    ) -> Result<Vec<IaCSecurityFinding>> {
        let patterns = self.security_patterns.get(&language)
            .ok_or_else(|| anyhow::anyhow!("No patterns found for language: {:?}", language))?;
        
        Ok(patterns.scan_content(content))
    }
    
    fn build_resource_graph(
        &self,
        content: &str,
        language: IaCLanguage,
        context_files: &[PathBuf],
    ) -> Result<IaCResourceGraph> {
        match language {
            IaCLanguage::Terraform => self.build_terraform_resource_graph(content, context_files),
            IaCLanguage::CloudFormation => self.build_cloudformation_resource_graph(content),
            IaCLanguage::Kubernetes => self.build_kubernetes_resource_graph(content),
            _ => Ok(IaCResourceGraph {
                resources: HashMap::new(),
                dependencies: Vec::new(),
            }),
        }
    }
    
    fn build_terraform_resource_graph(
        &self,
        content: &str,
        context_files: &[PathBuf],
    ) -> Result<IaCResourceGraph> {
        // Parse Terraform HCL using tree-sitter
        let mut parser = CodeParser::new()?;
        
        // This would use tree-sitter-hcl to parse the structure
        // For now, simplified regex-based parsing
        let mut resources = HashMap::new();
        let mut dependencies = Vec::new();
        
        // Extract resources using regex (simplified)
        let resource_regex = regex::Regex::new(r#"resource\s+"([^"]+)"\s+"([^"]+)"\s*\{"#)?;
        
        for cap in resource_regex.captures_iter(content) {
            let resource_type = cap[1].to_string();
            let resource_name = cap[2].to_string();
            let resource_id = format!("{}_{}", resource_type, resource_name);
            
            // Extract properties for this resource (simplified)
            let properties = self.extract_resource_properties(content, &resource_id);
            
            let resource = crate::iac_patterns::IaCResource {
                resource_type,
                name: resource_name,
                properties,
                security_implications: Vec::new(),
            };
            
            resources.insert(resource_id, resource);
        }
        
        // Find dependencies between resources
        dependencies = self.find_terraform_dependencies(content, &resources);
        
        Ok(IaCResourceGraph {
            resources,
            dependencies,
        })
    }
    
    fn extract_resource_properties(&self, content: &str, resource_id: &str) -> HashMap<String, String> {
        // Simplified property extraction
        let mut properties = HashMap::new();
        
        // Common security-relevant properties
        if content.contains("publicly_accessible = true") {
            properties.insert("publicly_accessible".to_string(), "true".to_string());
        }
        
        if content.contains("encrypted = false") {
            properties.insert("encrypted".to_string(), "false".to_string());
        }
        
        if content.contains(r#"cidr_blocks = ["0.0.0.0/0"]"#) {
            properties.insert("cidr_blocks".to_string(), "0.0.0.0/0".to_string());
        }
        
        properties
    }
    
    fn find_terraform_dependencies(
        &self,
        content: &str,
        resources: &HashMap<String, crate::iac_patterns::IaCResource>,
    ) -> Vec<(String, String)> {
        let mut dependencies = Vec::new();
        
        // Find resource references like aws_security_group.sg_name.id
        let ref_regex = regex::Regex::new(r"(\w+)\.(\w+)\.").unwrap();
        
        for cap in ref_regex.captures_iter(content) {
            let referenced_type = &cap[1];
            let referenced_name = &cap[2];
            let referenced_id = format!("{}_{}", referenced_type, referenced_name);
            
            if resources.contains_key(&referenced_id) {
                // This is a simplified dependency detection
                // In reality, we'd need to track which resource contains this reference
                dependencies.push(("unknown".to_string(), referenced_id));
            }
        }
        
        dependencies
    }
    
    fn build_cloudformation_resource_graph(&self, content: &str) -> Result<IaCResourceGraph> {
        // CloudFormation resource graph building
        Ok(IaCResourceGraph {
            resources: HashMap::new(),
            dependencies: Vec::new(),
        })
    }
    
    fn build_kubernetes_resource_graph(&self, content: &str) -> Result<IaCResourceGraph> {
        // Kubernetes resource graph building
        Ok(IaCResourceGraph {
            resources: HashMap::new(),
            dependencies: Vec::new(),
        })
    }
    
    async fn perform_llm_analysis(
        &self,
        file_path: &PathBuf,
        content: &str,
        static_findings: &[IaCSecurityFinding],
        cross_resource_vulns: &[CrossResourceVulnerability],
        model: &str,
        language: IaCLanguage,
    ) -> Result<String> {
        let prompt = self.build_iac_analysis_prompt(
            file_path,
            content,
            static_findings,
            cross_resource_vulns,
            language,
        );
        
        let chat_req = ChatRequest::new(vec![ChatMessage::user(prompt)]);
        
        let chat_res = self.client.exec_chat(model, chat_req, None).await?;
        
        match chat_res.content_text_as_str() {
            Some(content) => Ok(content.to_string()),
            None => Err(anyhow::anyhow!("Failed to get LLM analysis response")),
        }
    }
    
    fn build_iac_analysis_prompt(
        &self,
        file_path: &PathBuf,
        content: &str,
        static_findings: &[IaCSecurityFinding],
        cross_resource_vulns: &[CrossResourceVulnerability],
        language: IaCLanguage,
    ) -> String {
        format!(
            r#"You are an expert infrastructure security analyst. Analyze this {:?} configuration for security vulnerabilities and misconfigurations.

File: {}

Content:
{}

Static Analysis Findings:
{}

Cross-Resource Vulnerabilities:
{}

Please provide:
1. A comprehensive security assessment
2. Risk prioritization based on potential impact
3. Specific remediation steps for each issue
4. Compliance considerations (GDPR, HIPAA, SOX, etc.)
5. Best practices recommendations

Focus on:
- Infrastructure misconfigurations
- Security policy violations  
- Compliance failures
- Resource relationships and their security implications
- Potential for privilege escalation or data exposure"#,
            language,
            file_path.display(),
            content,
            self.format_static_findings(static_findings),
            self.format_cross_resource_vulns(cross_resource_vulns)
        )
    }
    
    fn format_static_findings(&self, findings: &[IaCSecurityFinding]) -> String {
        findings.iter()
            .map(|f| format!("- {:?}: {} (Line {})", f.severity, f.description, f.line_number))
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    fn format_cross_resource_vulns(&self, vulns: &[CrossResourceVulnerability]) -> String {
        vulns.iter()
            .map(|v| format!("- {:?}: {} ({} -> {})", v.severity, v.description, v.primary_resource, v.secondary_resource))
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    fn check_compliance(
        &self,
        static_findings: &[IaCSecurityFinding],
        cross_resource_vulns: &[CrossResourceVulnerability],
    ) -> Vec<ComplianceViolation> {
        let mut violations = Vec::new();
        
        for finding in static_findings {
            if let Some(frameworks) = &finding.compliance_frameworks {
                for framework in frameworks {
                    violations.push(ComplianceViolation {
                        framework: framework.clone(),
                        violation_type: finding.vulnerability_type.clone(),
                        description: finding.description.clone(),
                        severity: finding.severity.clone(),
                    });
                }
            }
        }
        
        violations
    }
}

#[derive(Debug)]
pub struct IaCAnalysisResult {
    pub file_path: PathBuf,
    pub language: IaCLanguage,
    pub static_findings: Vec<IaCSecurityFinding>,
    pub cross_resource_vulnerabilities: Vec<CrossResourceVulnerability>,
    pub llm_analysis: String,
    pub compliance_violations: Vec<ComplianceViolation>,
    pub resource_graph: IaCResourceGraph,
}

impl IaCAnalysisResult {
    pub fn empty() -> Self {
        Self {
            file_path: PathBuf::new(),
            language: IaCLanguage::Other,
            static_findings: Vec::new(),
            cross_resource_vulnerabilities: Vec::new(),
            llm_analysis: String::new(),
            compliance_violations: Vec::new(),
            resource_graph: IaCResourceGraph {
                resources: HashMap::new(),
                dependencies: Vec::new(),
            },
        }
    }
    
    pub fn has_critical_issues(&self) -> bool {
        self.static_findings.iter().any(|f| matches!(f.severity, Severity::Critical)) ||
        self.cross_resource_vulnerabilities.iter().any(|v| matches!(v.severity, Severity::Critical))
    }
    
    pub fn get_vulnerability_summary(&self) -> HashMap<IaCVulnerabilityType, usize> {
        let mut summary = HashMap::new();
        
        for finding in &self.static_findings {
            *summary.entry(finding.vulnerability_type.clone()).or_insert(0) += 1;
        }
        
        for vuln in &self.cross_resource_vulnerabilities {
            *summary.entry(vuln.vulnerability_type.clone()).or_insert(0) += 1;
        }
        
        summary
    }
}

#[derive(Debug, Clone)]
pub struct ComplianceViolation {
    pub framework: String,
    pub violation_type: IaCVulnerabilityType,
    pub description: String,
    pub severity: Severity,
}