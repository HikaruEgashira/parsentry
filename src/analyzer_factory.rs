use std::path::PathBuf;
use anyhow::Result;
use genai::Client;

use crate::security_patterns::Language;
use crate::iac_patterns::IaCLanguage;
use crate::analyzer;
use crate::iac_analyzer::{IaCAnalyzer, IaCAnalysisResult};
use crate::response::Response;

#[derive(Debug, Clone)]
pub enum FileType {
    Programming(Language),
    Infrastructure(IaCLanguage),
    Unknown,
}

pub struct AnalyzerFactory {
    iac_analyzer: IaCAnalyzer,
}

impl AnalyzerFactory {
    pub fn new(client: Client) -> Self {
        Self {
            iac_analyzer: IaCAnalyzer::new(client),
        }
    }
    
    pub fn detect_file_type(file_path: &PathBuf) -> FileType {
        if let Some(ext) = file_path.extension().and_then(|e| e.to_str()) {
            // First check if it's an IaC file
            if let Some(iac_lang) = IaCLanguage::from_extension(ext) {
                return FileType::Infrastructure(iac_lang);
            }
            
            // Then check if it's a programming language
            let prog_lang = Language::from_extension(ext);
            if prog_lang != Language::Other {
                return FileType::Programming(prog_lang);
            }
        }
        
        // Content-based detection for ambiguous cases
        if let Ok(content) = std::fs::read_to_string(file_path) {
            if Self::is_iac_content(&content) {
                return FileType::Infrastructure(Self::detect_iac_from_content(&content));
            }
        }
        
        FileType::Unknown
    }
    
    fn is_iac_content(content: &str) -> bool {
        // Common IaC indicators
        content.contains("terraform {") ||
        content.contains("provider \"") ||
        content.contains("resource \"") ||
        content.contains("AWSTemplateFormatVersion") ||
        content.contains("Resources:") ||
        (content.contains("apiVersion:") && content.contains("kind:"))
    }
    
    fn detect_iac_from_content(content: &str) -> IaCLanguage {
        if content.contains("terraform {") || content.contains("provider \"") {
            IaCLanguage::Terraform
        } else if content.contains("AWSTemplateFormatVersion") {
            IaCLanguage::CloudFormation
        } else if content.contains("apiVersion:") && content.contains("kind:") {
            IaCLanguage::Kubernetes
        } else {
            IaCLanguage::Other
        }
    }
    
    pub async fn analyze_file(
        &self,
        file_path: &PathBuf,
        model: &str,
        files: &[PathBuf],
        verbosity: u8,
        context: &crate::parser::Context,
        min_confidence: i32,
    ) -> Result<AnalysisResult> {
        let file_type = Self::detect_file_type(file_path);
        
        match file_type {
            FileType::Programming(_lang) => {
                let response = analyzer::analyze_file(
                    file_path,
                    model,
                    files,
                    verbosity,
                    context,
                    min_confidence,
                ).await?;
                Ok(AnalysisResult::Programming(response))
            }
            
            FileType::Infrastructure(_iac_lang) => {
                let result = self.iac_analyzer.analyze_iac_file(
                    file_path,
                    model,
                    files,
                    min_confidence,
                ).await?;
                Ok(AnalysisResult::Infrastructure(result))
            }
            
            FileType::Unknown => {
                // Fallback to programming language analysis
                let response = analyzer::analyze_file(
                    file_path,
                    model,
                    files,
                    verbosity,
                    context,
                    min_confidence,
                ).await?;
                Ok(AnalysisResult::Programming(response))
            }
        }
    }
}

#[derive(Debug)]
pub enum AnalysisResult {
    Programming(Response),
    Infrastructure(IaCAnalysisResult),
}

impl AnalysisResult {
    pub fn has_vulnerabilities(&self) -> bool {
        match self {
            AnalysisResult::Programming(response) => !response.vulnerability_types.is_empty(),
            AnalysisResult::Infrastructure(result) => {
                !result.static_findings.is_empty() || !result.cross_resource_vulnerabilities.is_empty()
            }
        }
    }
    
    pub fn confidence_score(&self) -> i32 {
        match self {
            AnalysisResult::Programming(response) => response.confidence_score,
            AnalysisResult::Infrastructure(_) => {
                // IaC analysis typically has higher confidence due to static nature
                85
            }
        }
    }
    
    pub fn vulnerability_summary(&self) -> String {
        match self {
            AnalysisResult::Programming(response) => {
                format!("Programming vulnerabilities: {:?}", response.vulnerability_types)
            }
            AnalysisResult::Infrastructure(result) => {
                let summary = result.get_vulnerability_summary();
                format!("IaC misconfigurations: {} types found", summary.len())
            }
        }
    }
}

// Enhanced file discovery that considers both programming and IaC files
pub fn discover_analyzable_files(repo_path: &PathBuf) -> Result<Vec<PathBuf>> {
    use std::fs;
    use std::collections::HashSet;
    
    let mut files = Vec::new();
    let mut visited = HashSet::new();
    
    fn visit_dir(
        dir: &PathBuf,
        files: &mut Vec<PathBuf>,
        visited: &mut HashSet<PathBuf>,
    ) -> Result<()> {
        if visited.contains(dir) {
            return Ok(());
        }
        visited.insert(dir.clone());
        
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                // Skip common non-source directories
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if matches!(dir_name, "node_modules" | "target" | ".git" | "vendor" | "__pycache__") {
                        continue;
                    }
                }
                visit_dir(&path, files, visited)?;
            } else if path.is_file() {
                let file_type = AnalyzerFactory::detect_file_type(&path);
                if !matches!(file_type, FileType::Unknown) {
                    files.push(path);
                }
            }
        }
        
        Ok(())
    }
    
    visit_dir(repo_path, &mut files, &mut visited)?;
    Ok(files)
}