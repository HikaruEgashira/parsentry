use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;

/// 統一パターンタイプ: Programming言語とIaCを区別しない
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnifiedPatternType {
    // Programming言語の概念
    Source,      // ユーザー入力、外部データ
    Sink,        // ファイル書き込み、DB実行、システムコール
    Sanitizer,   // 検証、エンコーディング、フィルタリング
    
    // IaC PAR概念 (同じ抽象化レベルにマップ)
    Principal,   // 権限主体 (Source相当)
    Action,      // 操作・権限 (中間処理相当)
    Resource,    // 対象リソース (Sink相当)
    Condition,   // アクセス制御条件 (Sanitizer相当)
}

/// 言語・技術を統一して扱う列挙型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnifiedLanguage {
    // Programming Languages
    Python,
    JavaScript, 
    TypeScript,
    Rust,
    Java,
    Go,
    Ruby,
    C,
    Cpp,
    
    // Infrastructure as Code
    Terraform,
    CloudFormation,
    Kubernetes,
    Ansible,
    Helm,
    
    // Other
    Other,
}

impl UnifiedLanguage {
    pub fn from_extension(ext: &str) -> Self {
        match ext {
            // Programming
            "py" => UnifiedLanguage::Python,
            "js" => UnifiedLanguage::JavaScript,
            "ts" => UnifiedLanguage::TypeScript,
            "rs" => UnifiedLanguage::Rust,
            "java" => UnifiedLanguage::Java,
            "go" => UnifiedLanguage::Go,
            "rb" => UnifiedLanguage::Ruby,
            "c" | "h" => UnifiedLanguage::C,
            "cpp" | "cxx" | "cc" | "hpp" | "hxx" => UnifiedLanguage::Cpp,
            
            // IaC
            "tf" | "hcl" => UnifiedLanguage::Terraform,
            "yaml" | "yml" => UnifiedLanguage::Kubernetes, // Context-dependent
            "json" => UnifiedLanguage::CloudFormation,     // Context-dependent
            
            _ => UnifiedLanguage::Other,
        }
    }
    
    pub fn is_programming_language(&self) -> bool {
        matches!(self, 
            UnifiedLanguage::Python | UnifiedLanguage::JavaScript | UnifiedLanguage::TypeScript |
            UnifiedLanguage::Rust | UnifiedLanguage::Java | UnifiedLanguage::Go |
            UnifiedLanguage::Ruby | UnifiedLanguage::C | UnifiedLanguage::Cpp
        )
    }
    
    pub fn is_infrastructure_code(&self) -> bool {
        matches!(self,
            UnifiedLanguage::Terraform | UnifiedLanguage::CloudFormation | 
            UnifiedLanguage::Kubernetes | UnifiedLanguage::Ansible | UnifiedLanguage::Helm
        )
    }
}

/// 統一パターン設定
#[derive(Debug, Clone, Deserialize)]
pub struct UnifiedPatternConfig {
    pub pattern: String,
    pub description: String,
    pub pattern_type: String, // "source", "sink", "sanitizer", "principal", "action", "resource", "condition"
    pub severity: Option<String>,
    pub context: Option<PatternContext>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PatternContext {
    pub programming_context: Option<String>, // "web", "database", "file_system"
    pub iac_context: Option<String>,         // "network", "storage", "identity"
    pub cross_boundary: Option<bool>,        // Cross trust boundary access
}

/// 言語別の統一パターン
#[derive(Debug, Clone, Deserialize)]
pub struct UnifiedLanguagePatterns {
    // 統一されたパターン分類
    pub sources: Option<Vec<UnifiedPatternConfig>>,
    pub sinks: Option<Vec<UnifiedPatternConfig>>,
    pub sanitizers: Option<Vec<UnifiedPatternConfig>>,
    pub principals: Option<Vec<UnifiedPatternConfig>>,   // IaC specific
    pub actions: Option<Vec<UnifiedPatternConfig>>,      // IaC specific  
    pub resources: Option<Vec<UnifiedPatternConfig>>,    // IaC specific
    pub conditions: Option<Vec<UnifiedPatternConfig>>,   // IaC specific
}

/// 統一セキュリティパターン解析器
pub struct UnifiedSecurityPatterns {
    language: UnifiedLanguage,
    
    // プログラミング言語とIaCの両方に対応したパターン
    source_patterns: Vec<CompiledPattern>,
    sink_patterns: Vec<CompiledPattern>, 
    sanitizer_patterns: Vec<CompiledPattern>,
    
    // IaC固有だが、同じ抽象化で扱う
    principal_patterns: Vec<CompiledPattern>,   // → Source相当
    action_patterns: Vec<CompiledPattern>,      // → 中間処理
    resource_patterns: Vec<CompiledPattern>,    // → Sink相当
    condition_patterns: Vec<CompiledPattern>,   // → Sanitizer相当
    
    pattern_mappings: PatternMappings,
}

#[derive(Debug)]
struct CompiledPattern {
    regex: Regex,
    config: UnifiedPatternConfig,
    unified_type: UnifiedPatternType,
}

/// Programming ↔ IaC パターンマッピング
#[derive(Debug)]
struct PatternMappings {
    // Programming → IaC conceptual mappings
    source_to_principal: HashMap<String, String>,
    sink_to_resource: HashMap<String, String>, 
    sanitizer_to_condition: HashMap<String, String>,
    
    // IaC → Programming reverse mappings
    principal_to_source: HashMap<String, String>,
    resource_to_sink: HashMap<String, String>,
    condition_to_sanitizer: HashMap<String, String>,
}

impl UnifiedSecurityPatterns {
    pub fn new(language: UnifiedLanguage) -> Self {
        let pattern_map = Self::load_unified_patterns();
        let lang_patterns = pattern_map.get(&language)
            .or_else(|| pattern_map.get(&UnifiedLanguage::Other))
            .unwrap();
        
        let mut source_patterns = Vec::new();
        let mut sink_patterns = Vec::new();
        let mut sanitizer_patterns = Vec::new();
        let mut principal_patterns = Vec::new();
        let mut action_patterns = Vec::new();
        let mut resource_patterns = Vec::new();
        let mut condition_patterns = Vec::new();
        
        // Load traditional programming patterns
        if let Some(sources) = &lang_patterns.sources {
            source_patterns = Self::compile_patterns(sources, UnifiedPatternType::Source);
        }
        
        if let Some(sinks) = &lang_patterns.sinks {
            sink_patterns = Self::compile_patterns(sinks, UnifiedPatternType::Sink);
        }
        
        if let Some(sanitizers) = &lang_patterns.sanitizers {
            sanitizer_patterns = Self::compile_patterns(sanitizers, UnifiedPatternType::Sanitizer);
        }
        
        // Load IaC PAR patterns (treated with same abstraction)
        if let Some(principals) = &lang_patterns.principals {
            principal_patterns = Self::compile_patterns(principals, UnifiedPatternType::Principal);
        }
        
        if let Some(actions) = &lang_patterns.actions {
            action_patterns = Self::compile_patterns(actions, UnifiedPatternType::Action);
        }
        
        if let Some(resources) = &lang_patterns.resources {
            resource_patterns = Self::compile_patterns(resources, UnifiedPatternType::Resource);
        }
        
        if let Some(conditions) = &lang_patterns.conditions {
            condition_patterns = Self::compile_patterns(conditions, UnifiedPatternType::Condition);
        }
        
        Self {
            language,
            source_patterns,
            sink_patterns,
            sanitizer_patterns,
            principal_patterns,
            action_patterns,
            resource_patterns,
            condition_patterns,
            pattern_mappings: Self::build_pattern_mappings(),
        }
    }
    
    fn compile_patterns(configs: &[UnifiedPatternConfig], pattern_type: UnifiedPatternType) -> Vec<CompiledPattern> {
        configs.iter().filter_map(|config| {
            Regex::new(&config.pattern).ok().map(|regex| {
                CompiledPattern {
                    regex,
                    config: config.clone(),
                    unified_type: pattern_type.clone(),
                }
            })
        }).collect()
    }
    
    fn build_pattern_mappings() -> PatternMappings {
        let mut mappings = PatternMappings {
            source_to_principal: HashMap::new(),
            sink_to_resource: HashMap::new(),
            sanitizer_to_condition: HashMap::new(),
            principal_to_source: HashMap::new(),
            resource_to_sink: HashMap::new(),
            condition_to_sanitizer: HashMap::new(),
        };
        
        // Programming → IaC conceptual mappings
        mappings.source_to_principal.insert("user_input".to_string(), "external_principal".to_string());
        mappings.source_to_principal.insert("external_api".to_string(), "cross_account_principal".to_string());
        mappings.source_to_principal.insert("file_input".to_string(), "external_service".to_string());
        
        mappings.sink_to_resource.insert("database_query".to_string(), "database_resource".to_string());
        mappings.sink_to_resource.insert("file_write".to_string(), "storage_resource".to_string());
        mappings.sink_to_resource.insert("system_call".to_string(), "compute_resource".to_string());
        mappings.sink_to_resource.insert("network_request".to_string(), "network_resource".to_string());
        
        mappings.sanitizer_to_condition.insert("input_validation".to_string(), "access_policy".to_string());
        mappings.sanitizer_to_condition.insert("sql_escape".to_string(), "query_validation".to_string());
        mappings.sanitizer_to_condition.insert("html_encode".to_string(), "content_policy".to_string());
        
        // IaC → Programming reverse mappings
        for (prog, iac) in &mappings.source_to_principal {
            mappings.principal_to_source.insert(iac.clone(), prog.clone());
        }
        for (prog, iac) in &mappings.sink_to_resource {
            mappings.resource_to_sink.insert(iac.clone(), prog.clone());
        }
        for (prog, iac) in &mappings.sanitizer_to_condition {
            mappings.condition_to_sanitizer.insert(iac.clone(), prog.clone());
        }
        
        mappings
    }
    
    /// 統一スキャン: Programming & IaC を同じアルゴリズムで処理
    pub fn scan_unified(&self, content: &str) -> Vec<UnifiedSecurityFinding> {
        let mut findings = Vec::new();
        
        // Programming patterns (if applicable)
        if self.language.is_programming_language() {
            findings.extend(self.scan_pattern_group(&self.source_patterns, content));
            findings.extend(self.scan_pattern_group(&self.sink_patterns, content));
            findings.extend(self.scan_pattern_group(&self.sanitizer_patterns, content));
        }
        
        // IaC patterns (if applicable) - BUT treated with same logic
        if self.language.is_infrastructure_code() {
            findings.extend(self.scan_pattern_group(&self.principal_patterns, content));
            findings.extend(self.scan_pattern_group(&self.action_patterns, content));
            findings.extend(self.scan_pattern_group(&self.resource_patterns, content));
            findings.extend(self.scan_pattern_group(&self.condition_patterns, content));
        }
        
        // Cross-pattern analysis: find flows regardless of Programming vs IaC
        findings.extend(self.analyze_unified_flows(content, &findings));
        
        findings
    }
    
    fn scan_pattern_group(&self, patterns: &[CompiledPattern], content: &str) -> Vec<UnifiedSecurityFinding> {
        let mut findings = Vec::new();
        
        for pattern in patterns {
            for mat in pattern.regex.find_iter(content) {
                let finding = UnifiedSecurityFinding {
                    pattern_type: pattern.unified_type.clone(),
                    pattern: pattern.config.pattern.clone(),
                    description: pattern.config.description.clone(),
                    matched_text: mat.as_str().to_string(),
                    line_number: content[..mat.start()].lines().count() + 1,
                    severity: pattern.config.severity.clone().unwrap_or("medium".to_string()),
                    language: self.language,
                    context: pattern.config.context.clone(),
                };
                findings.push(finding);
            }
        }
        
        findings
    }
    
    /// Programming言語とIaCを区別しない統一フロー解析
    fn analyze_unified_flows(&self, content: &str, findings: &[UnifiedSecurityFinding]) -> Vec<UnifiedSecurityFinding> {
        let mut flow_findings = Vec::new();
        
        // Group findings by conceptual type
        let sources: Vec<_> = findings.iter().filter(|f| 
            matches!(f.pattern_type, UnifiedPatternType::Source | UnifiedPatternType::Principal)
        ).collect();
        
        let sinks: Vec<_> = findings.iter().filter(|f|
            matches!(f.pattern_type, UnifiedPatternType::Sink | UnifiedPatternType::Resource)
        ).collect();
        
        let sanitizers: Vec<_> = findings.iter().filter(|f|
            matches!(f.pattern_type, UnifiedPatternType::Sanitizer | UnifiedPatternType::Condition)
        ).collect();
        
        // Analyze flows: Source/Principal → Sink/Resource (with/without Sanitizer/Condition)
        for source in &sources {
            for sink in &sinks {
                let path_exists = self.check_flow_path(content, source, sink);
                if path_exists {
                    let has_protection = sanitizers.iter().any(|s| 
                        self.is_protection_relevant(source, sink, s)
                    );
                    
                    if !has_protection {
                        let vuln_type = self.determine_vulnerability_type(source, sink);
                        flow_findings.push(UnifiedSecurityFinding {
                            pattern_type: UnifiedPatternType::Sink, // Flow result
                            pattern: format!("{} → {}", source.pattern, sink.pattern),
                            description: format!("Unprotected flow: {} to {}", 
                                self.get_conceptual_description(&source.pattern_type),
                                self.get_conceptual_description(&sink.pattern_type)
                            ),
                            matched_text: format!("{} ... {}", source.matched_text, sink.matched_text),
                            line_number: source.line_number,
                            severity: vuln_type.severity(),
                            language: self.language,
                            context: Some(PatternContext {
                                programming_context: if self.language.is_programming_language() { 
                                    Some(vuln_type.programming_context()) 
                                } else { None },
                                iac_context: if self.language.is_infrastructure_code() { 
                                    Some(vuln_type.iac_context()) 
                                } else { None },
                                cross_boundary: Some(true),
                            }),
                        });
                    }
                }
            }
        }
        
        flow_findings
    }
    
    fn check_flow_path(&self, content: &str, source: &UnifiedSecurityFinding, sink: &UnifiedSecurityFinding) -> bool {
        // Simplified: check if source and sink appear in same logical block
        let source_line = source.line_number;
        let sink_line = sink.line_number;
        
        // Consider them connected if within reasonable distance or in same function/resource block
        (sink_line as i32 - source_line as i32).abs() < 20 ||
        self.in_same_logical_block(content, source_line, sink_line)
    }
    
    fn in_same_logical_block(&self, content: &str, line1: usize, line2: usize) -> bool {
        let lines: Vec<_> = content.lines().collect();
        if line1 >= lines.len() || line2 >= lines.len() {
            return false;
        }
        
        let start = std::cmp::min(line1, line2);
        let end = std::cmp::max(line1, line2);
        
        // Count braces/indentation to determine if in same block
        let mut brace_count = 0;
        let mut in_same_block = true;
        
        for i in start..=end {
            if i < lines.len() {
                let line = lines[i];
                brace_count += line.matches('{').count() as i32;
                brace_count -= line.matches('}').count() as i32;
                
                // If we go negative, we've exited the original block
                if brace_count < 0 {
                    in_same_block = false;
                    break;
                }
            }
        }
        
        in_same_block
    }
    
    fn is_protection_relevant(&self, source: &UnifiedSecurityFinding, sink: &UnifiedSecurityFinding, protection: &UnifiedSecurityFinding) -> bool {
        // Check if the sanitizer/condition is relevant for this source→sink flow
        let source_concept = self.get_conceptual_type(&source.pattern_type);
        let sink_concept = self.get_conceptual_type(&sink.pattern_type);
        let protection_concept = self.get_conceptual_type(&protection.pattern_type);
        
        // Look for semantic relevance
        (source_concept == "input" && sink_concept == "output" && protection_concept == "validation") ||
        (source_concept == "principal" && sink_concept == "resource" && protection_concept == "condition")
    }
    
    fn get_conceptual_type(&self, pattern_type: &UnifiedPatternType) -> &str {
        match pattern_type {
            UnifiedPatternType::Source | UnifiedPatternType::Principal => "input",
            UnifiedPatternType::Sink | UnifiedPatternType::Resource => "output", 
            UnifiedPatternType::Sanitizer | UnifiedPatternType::Condition => "validation",
            UnifiedPatternType::Action => "processing",
        }
    }
    
    fn get_conceptual_description(&self, pattern_type: &UnifiedPatternType) -> &str {
        match pattern_type {
            UnifiedPatternType::Source => "external input",
            UnifiedPatternType::Principal => "external principal",
            UnifiedPatternType::Sink => "sensitive operation",
            UnifiedPatternType::Resource => "protected resource",
            UnifiedPatternType::Sanitizer => "input validation",
            UnifiedPatternType::Condition => "access control",
            UnifiedPatternType::Action => "operation",
        }
    }
    
    fn determine_vulnerability_type(&self, source: &UnifiedSecurityFinding, sink: &UnifiedSecurityFinding) -> UnifiedVulnerabilityType {
        match (&source.pattern_type, &sink.pattern_type) {
            (UnifiedPatternType::Source, UnifiedPatternType::Sink) => {
                if sink.pattern.contains("sql") || sink.pattern.contains("query") {
                    UnifiedVulnerabilityType::SqlInjection
                } else if sink.pattern.contains("system") || sink.pattern.contains("exec") {
                    UnifiedVulnerabilityType::CommandInjection
                } else if sink.pattern.contains("html") || sink.pattern.contains("dom") {
                    UnifiedVulnerabilityType::XSS
                } else {
                    UnifiedVulnerabilityType::DataFlow
                }
            },
            (UnifiedPatternType::Principal, UnifiedPatternType::Resource) => {
                if source.matched_text.contains("*") || source.matched_text.contains("0.0.0.0") {
                    UnifiedVulnerabilityType::PrivilegeEscalation
                } else if sink.matched_text.contains("confidential") || sink.matched_text.contains("sensitive") {
                    UnifiedVulnerabilityType::UnauthorizedAccess
                } else {
                    UnifiedVulnerabilityType::ConfigurationMistake
                }
            },
            _ => UnifiedVulnerabilityType::Other,
        }
    }
    
    fn load_unified_patterns() -> HashMap<UnifiedLanguage, UnifiedLanguagePatterns> {
        // This would load from a unified patterns.yml file
        // For now, return empty patterns
        HashMap::new()
    }
}

#[derive(Debug, Clone)]
pub struct UnifiedSecurityFinding {
    pub pattern_type: UnifiedPatternType,
    pub pattern: String,
    pub description: String,
    pub matched_text: String,
    pub line_number: usize,
    pub severity: String,
    pub language: UnifiedLanguage,
    pub context: Option<PatternContext>,
}

#[derive(Debug, Clone)]
pub enum UnifiedVulnerabilityType {
    // Programming vulnerabilities
    SqlInjection,
    CommandInjection, 
    XSS,
    DataFlow,
    
    // IaC vulnerabilities (conceptually mapped)
    PrivilegeEscalation,    // → Command Injection equivalent
    UnauthorizedAccess,     // → SQL Injection equivalent  
    ConfigurationMistake,   // → Data Flow equivalent
    
    Other,
}

impl UnifiedVulnerabilityType {
    pub fn severity(&self) -> String {
        match self {
            UnifiedVulnerabilityType::SqlInjection | 
            UnifiedVulnerabilityType::CommandInjection |
            UnifiedVulnerabilityType::PrivilegeEscalation => "critical".to_string(),
            
            UnifiedVulnerabilityType::XSS |
            UnifiedVulnerabilityType::UnauthorizedAccess => "high".to_string(),
            
            UnifiedVulnerabilityType::DataFlow |
            UnifiedVulnerabilityType::ConfigurationMistake => "medium".to_string(),
            
            UnifiedVulnerabilityType::Other => "low".to_string(),
        }
    }
    
    pub fn programming_context(&self) -> String {
        match self {
            UnifiedVulnerabilityType::SqlInjection => "database".to_string(),
            UnifiedVulnerabilityType::CommandInjection => "system".to_string(),
            UnifiedVulnerabilityType::XSS => "web".to_string(),
            _ => "general".to_string(),
        }
    }
    
    pub fn iac_context(&self) -> String {
        match self {
            UnifiedVulnerabilityType::PrivilegeEscalation => "identity".to_string(),
            UnifiedVulnerabilityType::UnauthorizedAccess => "access_control".to_string(),
            UnifiedVulnerabilityType::ConfigurationMistake => "configuration".to_string(),
            _ => "general".to_string(),
        }
    }
}