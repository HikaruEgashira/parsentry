use std::collections::{HashMap, HashSet, VecDeque};
use crate::par_analyzer::{PARTriplet, Principal, Action, Resource, Effect, PrivilegeLevel};
use crate::security_patterns::{PatternType};

/// PARモデルをデータフロー解析に変換するトレイト
/// Programming言語: Source → Sink (データの流れ)
/// IaC: Principal → Resource (権限の流れ)
pub trait PARDataFlow {
    fn to_dataflow_node(&self) -> DataFlowNode;
    fn get_flow_type(&self) -> FlowType;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataFlowNode {
    pub node_id: String,
    pub node_type: DataFlowNodeType,
    pub sensitivity_level: SensitivityLevel,
    pub trust_boundary: TrustBoundary,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataFlowNodeType {
    // Programming language equivalent
    Source,      // User input, external data
    Sink,        // File write, DB query, system call
    Sanitizer,   // Validation, encoding
    
    // IaC PAR equivalent  
    Principal,   // Who (user, role, service)
    Action,      // What operation
    Resource,    // Target resource
    Condition,   // When/How constraints
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SensitivityLevel {
    Public,
    Internal, 
    Confidential,
    Restricted,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TrustBoundary {
    Internal,     // Same security context
    CrossAccount, // Different AWS account
    Internet,     // Public internet
    ThirdParty,   // External service
}

#[derive(Debug, Clone)]
pub enum FlowType {
    // Programming: data flow
    DataFlow {
        from: DataFlowNode,
        to: DataFlowNode,
        data_type: String,
        transformations: Vec<String>,
    },
    
    // IaC: privilege flow  
    PrivilegeFlow {
        principal: DataFlowNode,
        action: DataFlowNode,
        resource: DataFlowNode,
        conditions: Vec<DataFlowNode>,
    },
}

/// PAR三組をデータフロー形式に変換
impl PARDataFlow for PARTriplet {
    fn to_dataflow_node(&self) -> DataFlowNode {
        // Principal → Source (権限の起点)
        DataFlowNode {
            node_id: format!("{}_{}", self.principal.principal_type_str(), self.principal.identifier),
            node_type: DataFlowNodeType::Principal,
            sensitivity_level: self.principal.get_sensitivity_level(),
            trust_boundary: self.principal.get_trust_boundary(),
            attributes: self.principal.attributes.clone(),
        }
    }
    
    fn get_flow_type(&self) -> FlowType {
        let principal_node = DataFlowNode {
            node_id: format!("principal_{}", self.principal.identifier),
            node_type: DataFlowNodeType::Principal,
            sensitivity_level: self.principal.get_sensitivity_level(),
            trust_boundary: self.principal.get_trust_boundary(),
            attributes: self.principal.attributes.clone(),
        };
        
        let action_node = DataFlowNode {
            node_id: format!("action_{}_{}", self.action.service, self.action.operation),
            node_type: DataFlowNodeType::Action,
            sensitivity_level: SensitivityLevel::Internal,
            trust_boundary: TrustBoundary::Internal,
            attributes: HashMap::new(),
        };
        
        let resource_node = DataFlowNode {
            node_id: format!("resource_{}_{}", self.resource.resource_type, self.resource.identifier),
            node_type: DataFlowNodeType::Resource,
            sensitivity_level: match self.resource.sensitivity {
                crate::par_analyzer::DataSensitivity::Public => SensitivityLevel::Public,
                crate::par_analyzer::DataSensitivity::Internal => SensitivityLevel::Internal,
                crate::par_analyzer::DataSensitivity::Confidential => SensitivityLevel::Confidential,
                crate::par_analyzer::DataSensitivity::Restricted => SensitivityLevel::Restricted,
                crate::par_analyzer::DataSensitivity::Unknown => SensitivityLevel::Internal,
            },
            trust_boundary: TrustBoundary::Internal,
            attributes: HashMap::new(),
        };
        
        let condition_nodes: Vec<DataFlowNode> = self.conditions.iter().map(|c| {
            DataFlowNode {
                node_id: format!("condition_{}_{}", c.condition_type, c.key),
                node_type: DataFlowNodeType::Condition,
                sensitivity_level: SensitivityLevel::Internal,
                trust_boundary: TrustBoundary::Internal,
                attributes: HashMap::from([
                    ("key".to_string(), c.key.clone()),
                    ("value".to_string(), c.value.clone()),
                ]),
            }
        }).collect();
        
        FlowType::PrivilegeFlow {
            principal: principal_node,
            action: action_node,
            resource: resource_node,
            conditions: condition_nodes,
        }
    }
}

/// 統一データフロー解析エンジン
/// Programming言語とIaCの両方を同じ枠組みで解析
pub struct UnifiedDataFlowAnalyzer {
    pub flows: Vec<FlowType>,
    pub nodes: HashMap<String, DataFlowNode>,
    pub edges: Vec<DataFlowEdge>,
    pub vulnerability_patterns: Vec<VulnerabilityPattern>,
}

#[derive(Debug, Clone)]
pub struct DataFlowEdge {
    pub from: String,  // node_id
    pub to: String,    // node_id  
    pub edge_type: EdgeType,
    pub conditions: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum EdgeType {
    // Programming patterns
    DataDependency,    // Variable assignment, function parameter
    ControlDependency, // If/else, loop
    CallDependency,    // Function call
    
    // IaC patterns
    PermissionGrant,   // Principal → Resource via Action
    TrustRelation,     // Role assumption, delegation
    NetworkAccess,     // Security group, firewall rule
    ResourceDependency, // Resource references another resource
}

#[derive(Debug, Clone)]
pub struct VulnerabilityPattern {
    pub pattern_id: String,
    pub description: String,
    pub severity: Severity,
    pub pattern_type: UnifiedPatternType,
    pub detection_rule: DetectionRule,
}

#[derive(Debug, Clone)]
pub enum UnifiedPatternType {
    // Programming vulnerabilities mapped to IaC concepts
    TaintedDataFlow {
        source_pattern: String,      // "user_input" → "external_principal"
        sink_pattern: String,        // "system_call" → "critical_resource"  
        missing_sanitizer: String,   // "validation" → "access_control"
    },
    
    // IaC specific patterns
    PrivilegeEscalation {
        start_principal: String,
        escalation_path: Vec<String>,
        target_privilege: String,
    },
    
    TrustBoundaryViolation {
        source_boundary: TrustBoundary,
        target_boundary: TrustBoundary,
        violation_type: String,
    },
}

#[derive(Debug, Clone)]
pub enum Severity {
    Critical,
    High, 
    Medium,
    Low,
}

#[derive(Debug, Clone)]
pub struct DetectionRule {
    pub rule_type: RuleType,
    pub conditions: Vec<RuleCondition>,
}

#[derive(Debug, Clone)]
pub enum RuleType {
    PathExists,      // Source → Sink path exists
    PatternMatch,    // Regex/pattern matching
    GraphQuery,      // Complex graph traversal
}

#[derive(Debug, Clone)]
pub struct RuleCondition {
    pub field: String,
    pub operator: String,
    pub value: String,
}

impl UnifiedDataFlowAnalyzer {
    pub fn new() -> Self {
        Self {
            flows: Vec::new(),
            nodes: HashMap::new(),
            edges: Vec::new(),
            vulnerability_patterns: Self::initialize_vulnerability_patterns(),
        }
    }
    
    /// Programming言語とIaCの脆弱性パターンを統一的に初期化
    fn initialize_vulnerability_patterns() -> Vec<VulnerabilityPattern> {
        vec![
            // Programming → IaC mapping: SQL Injection → Unvalidated Resource Access  
            VulnerabilityPattern {
                pattern_id: "unvalidated_resource_access".to_string(),
                description: "External principal accessing sensitive resource without proper validation".to_string(),
                severity: Severity::High,
                pattern_type: UnifiedPatternType::TaintedDataFlow {
                    source_pattern: "external_principal".to_string(),
                    sink_pattern: "sensitive_resource".to_string(),
                    missing_sanitizer: "access_control_policy".to_string(),
                },
                detection_rule: DetectionRule {
                    rule_type: RuleType::PathExists,
                    conditions: vec![
                        RuleCondition {
                            field: "source.trust_boundary".to_string(),
                            operator: "equals".to_string(), 
                            value: "Internet".to_string(),
                        },
                        RuleCondition {
                            field: "sink.sensitivity_level".to_string(),
                            operator: "in".to_string(),
                            value: "Confidential,Restricted".to_string(),
                        },
                    ],
                },
            },
            
            // Programming → IaC mapping: Command Injection → Privilege Escalation
            VulnerabilityPattern {
                pattern_id: "privilege_escalation_path".to_string(),
                description: "Low-privilege principal can escalate to high privileges".to_string(),
                severity: Severity::Critical,
                pattern_type: UnifiedPatternType::PrivilegeEscalation {
                    start_principal: "low_privilege".to_string(),
                    escalation_path: vec!["assume_role".to_string(), "policy_modification".to_string()],
                    target_privilege: "admin".to_string(),
                },
                detection_rule: DetectionRule {
                    rule_type: RuleType::GraphQuery,
                    conditions: vec![
                        RuleCondition {
                            field: "path.length".to_string(),
                            operator: "greater_than".to_string(),
                            value: "1".to_string(),
                        },
                    ],
                },
            },
            
            // Trust boundary violation (specific to IaC)
            VulnerabilityPattern {
                pattern_id: "trust_boundary_violation".to_string(),
                description: "Cross-account access without proper controls".to_string(),
                severity: Severity::High,
                pattern_type: UnifiedPatternType::TrustBoundaryViolation {
                    source_boundary: TrustBoundary::ThirdParty,
                    target_boundary: TrustBoundary::Internal,
                    violation_type: "cross_account_access".to_string(),
                },
                detection_rule: DetectionRule {
                    rule_type: RuleType::PatternMatch,
                    conditions: vec![
                        RuleCondition {
                            field: "principal.identifier".to_string(),
                            operator: "contains".to_string(),
                            value: "arn:aws:iam::".to_string(),
                        },
                    ],
                },
            },
        ]
    }
    
    /// PARTripletをデータフロー形式に追加
    pub fn add_par_triplet(&mut self, triplet: &PARTriplet) {
        let flow = triplet.get_flow_type();
        
        match &flow {
            FlowType::PrivilegeFlow { principal, action, resource, conditions } => {
                // Add nodes
                self.nodes.insert(principal.node_id.clone(), principal.clone());
                self.nodes.insert(action.node_id.clone(), action.clone());
                self.nodes.insert(resource.node_id.clone(), resource.clone());
                
                for condition in conditions {
                    self.nodes.insert(condition.node_id.clone(), condition.clone());
                }
                
                // Add edges: Principal → Action → Resource
                self.edges.push(DataFlowEdge {
                    from: principal.node_id.clone(),
                    to: action.node_id.clone(),
                    edge_type: EdgeType::PermissionGrant,
                    conditions: vec![],
                });
                
                self.edges.push(DataFlowEdge {
                    from: action.node_id.clone(),
                    to: resource.node_id.clone(),
                    edge_type: EdgeType::PermissionGrant,
                    conditions: conditions.iter().map(|c| c.node_id.clone()).collect(),
                });
                
                // Add conditions as constraints
                for condition in conditions {
                    self.edges.push(DataFlowEdge {
                        from: action.node_id.clone(),
                        to: condition.node_id.clone(),
                        edge_type: EdgeType::ControlDependency,
                        conditions: vec![],
                    });
                }
            },
            FlowType::DataFlow { from, to, data_type, transformations } => {
                // Handle traditional programming language data flow
                self.nodes.insert(from.node_id.clone(), from.clone());
                self.nodes.insert(to.node_id.clone(), to.clone());
                
                self.edges.push(DataFlowEdge {
                    from: from.node_id.clone(),
                    to: to.node_id.clone(),
                    edge_type: EdgeType::DataDependency,
                    conditions: transformations.clone(),
                });
            }
        }
        
        self.flows.push(flow);
    }
    
    /// 統一脆弱性検出 - Programming言語とIaCの同じアルゴリズム
    pub fn detect_vulnerabilities(&self) -> Vec<UnifiedVulnerability> {
        let mut vulnerabilities = Vec::new();
        
        for pattern in &self.vulnerability_patterns {
            match &pattern.detection_rule.rule_type {
                RuleType::PathExists => {
                    vulnerabilities.extend(self.detect_path_based_vulnerabilities(pattern));
                },
                RuleType::PatternMatch => {
                    vulnerabilities.extend(self.detect_pattern_based_vulnerabilities(pattern));
                },
                RuleType::GraphQuery => {
                    vulnerabilities.extend(self.detect_graph_based_vulnerabilities(pattern));
                },
            }
        }
        
        vulnerabilities
    }
    
    fn detect_path_based_vulnerabilities(&self, pattern: &VulnerabilityPattern) -> Vec<UnifiedVulnerability> {
        let mut vulnerabilities = Vec::new();
        
        match &pattern.pattern_type {
            UnifiedPatternType::TaintedDataFlow { source_pattern, sink_pattern, missing_sanitizer } => {
                // Find all paths from sources to sinks
                let source_nodes = self.find_nodes_matching_pattern(source_pattern);
                let sink_nodes = self.find_nodes_matching_pattern(sink_pattern);
                
                for source in &source_nodes {
                    for sink in &sink_nodes {
                        if let Some(path) = self.find_path(&source.node_id, &sink.node_id) {
                            // Check if path has sanitization
                            if !self.path_has_sanitizer(&path, missing_sanitizer) {
                                vulnerabilities.push(UnifiedVulnerability {
                                    vulnerability_id: format!("{}_{}_to_{}", pattern.pattern_id, source.node_id, sink.node_id),
                                    pattern: pattern.clone(),
                                    affected_nodes: path.clone(),
                                    description: format!("Unsanitized flow from {} to {}", source.node_id, sink.node_id),
                                    remediation: format!("Add {} between source and sink", missing_sanitizer),
                                });
                            }
                        }
                    }
                }
            },
            _ => {}
        }
        
        vulnerabilities
    }
    
    fn detect_pattern_based_vulnerabilities(&self, pattern: &VulnerabilityPattern) -> Vec<UnifiedVulnerability> {
        let mut vulnerabilities = Vec::new();
        
        for (node_id, node) in &self.nodes {
            if self.node_matches_pattern(node, pattern) {
                vulnerabilities.push(UnifiedVulnerability {
                    vulnerability_id: format!("{}_{}", pattern.pattern_id, node_id),
                    pattern: pattern.clone(),
                    affected_nodes: vec![node_id.clone()],
                    description: pattern.description.clone(),
                    remediation: "Review and restrict access".to_string(),
                });
            }
        }
        
        vulnerabilities
    }
    
    fn detect_graph_based_vulnerabilities(&self, pattern: &VulnerabilityPattern) -> Vec<UnifiedVulnerability> {
        let mut vulnerabilities = Vec::new();
        
        match &pattern.pattern_type {
            UnifiedPatternType::PrivilegeEscalation { start_principal, escalation_path, target_privilege } => {
                // Find all privilege escalation paths
                let escalation_paths = self.find_escalation_paths(start_principal, target_privilege);
                
                for path in escalation_paths {
                    vulnerabilities.push(UnifiedVulnerability {
                        vulnerability_id: format!("{}_{}", pattern.pattern_id, path.join("_")),
                        pattern: pattern.clone(),
                        affected_nodes: path,
                        description: "Privilege escalation path detected".to_string(),
                        remediation: "Remove unnecessary privilege grants or add additional controls".to_string(),
                    });
                }
            },
            _ => {}
        }
        
        vulnerabilities
    }
    
    fn find_nodes_matching_pattern(&self, pattern: &str) -> Vec<&DataFlowNode> {
        self.nodes.values()
            .filter(|node| self.matches_pattern_string(node, pattern))
            .collect()
    }
    
    fn matches_pattern_string(&self, node: &DataFlowNode, pattern: &str) -> bool {
        match pattern {
            "external_principal" => matches!(node.trust_boundary, TrustBoundary::Internet | TrustBoundary::ThirdParty),
            "sensitive_resource" => matches!(node.sensitivity_level, SensitivityLevel::Confidential | SensitivityLevel::Restricted),
            "low_privilege" => node.node_id.contains("read") || node.node_id.contains("list"),
            _ => node.node_id.contains(pattern) || node.attributes.values().any(|v| v.contains(pattern)),
        }
    }
    
    fn find_path(&self, from: &str, to: &str) -> Option<Vec<String>> {
        // BFS to find path from source to sink
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut parent = HashMap::new();
        
        queue.push_back(from.to_string());
        visited.insert(from.to_string());
        
        while let Some(current) = queue.pop_front() {
            if current == to {
                // Reconstruct path
                let mut path = Vec::new();
                let mut node = to.to_string();
                
                while let Some(prev) = parent.get(&node) {
                    path.push(node.clone());
                    node = prev.clone();
                }
                path.push(from.to_string());
                path.reverse();
                
                return Some(path);
            }
            
            for edge in &self.edges {
                if edge.from == current && !visited.contains(&edge.to) {
                    visited.insert(edge.to.clone());
                    parent.insert(edge.to.clone(), current.clone());
                    queue.push_back(edge.to.clone());
                }
            }
        }
        
        None
    }
    
    fn path_has_sanitizer(&self, path: &[String], sanitizer_pattern: &str) -> bool {
        path.iter().any(|node_id| {
            if let Some(node) = self.nodes.get(node_id) {
                matches!(node.node_type, DataFlowNodeType::Sanitizer) ||
                node.node_id.contains(sanitizer_pattern) ||
                node.attributes.values().any(|v| v.contains(sanitizer_pattern))
            } else {
                false
            }
        })
    }
    
    fn node_matches_pattern(&self, node: &DataFlowNode, pattern: &VulnerabilityPattern) -> bool {
        for condition in &pattern.detection_rule.conditions {
            if !self.evaluate_condition(node, condition) {
                return false;
            }
        }
        true
    }
    
    fn evaluate_condition(&self, node: &DataFlowNode, condition: &RuleCondition) -> bool {
        let field_value = match condition.field.as_str() {
            "trust_boundary" => format!("{:?}", node.trust_boundary),
            "sensitivity_level" => format!("{:?}", node.sensitivity_level),
            "node_type" => format!("{:?}", node.node_type),
            _ => node.attributes.get(&condition.field).cloned().unwrap_or_default(),
        };
        
        match condition.operator.as_str() {
            "equals" => field_value == condition.value,
            "contains" => field_value.contains(&condition.value),
            "in" => condition.value.split(',').any(|v| v.trim() == field_value),
            _ => false,
        }
    }
    
    fn find_escalation_paths(&self, start_pattern: &str, target_pattern: &str) -> Vec<Vec<String>> {
        let mut paths = Vec::new();
        
        let start_nodes = self.find_nodes_matching_pattern(start_pattern);
        let target_nodes = self.find_nodes_matching_pattern(target_pattern);
        
        for start in &start_nodes {
            for target in &target_nodes {
                if let Some(path) = self.find_path(&start.node_id, &target.node_id) {
                    if path.len() > 2 { // Multi-step escalation
                        paths.push(path);
                    }
                }
            }
        }
        
        paths
    }
}

#[derive(Debug, Clone)]
pub struct UnifiedVulnerability {
    pub vulnerability_id: String,
    pub pattern: VulnerabilityPattern,
    pub affected_nodes: Vec<String>,
    pub description: String,
    pub remediation: String,
}

/// Principal trait extensions for dataflow conversion
impl Principal {
    pub fn principal_type_str(&self) -> &str {
        match self.principal_type {
            crate::par_analyzer::PrincipalType::User => "user",
            crate::par_analyzer::PrincipalType::Role => "role",
            crate::par_analyzer::PrincipalType::ServiceAccount => "service_account",
            crate::par_analyzer::PrincipalType::Group => "group",
            crate::par_analyzer::PrincipalType::Service => "service",
            crate::par_analyzer::PrincipalType::External => "external",
            crate::par_analyzer::PrincipalType::Wildcard => "wildcard",
        }
    }
    
    pub fn get_sensitivity_level(&self) -> SensitivityLevel {
        if self.identifier == "*" {
            SensitivityLevel::Public
        } else if self.identifier.contains("admin") || self.identifier.contains("root") {
            SensitivityLevel::Restricted
        } else if matches!(self.principal_type, crate::par_analyzer::PrincipalType::External) {
            SensitivityLevel::Public
        } else {
            SensitivityLevel::Internal
        }
    }
    
    pub fn get_trust_boundary(&self) -> TrustBoundary {
        match self.principal_type {
            crate::par_analyzer::PrincipalType::External => TrustBoundary::ThirdParty,
            crate::par_analyzer::PrincipalType::Wildcard => TrustBoundary::Internet,
            _ => {
                if self.identifier.contains("cross-account") || self.identifier.contains("federated") {
                    TrustBoundary::CrossAccount
                } else {
                    TrustBoundary::Internal
                }
            }
        }
    }
}