use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use anyhow::Result;

/// Policy As Code の Principal-Action-Resource 三組モデル
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PARTriplet {
    pub principal: Principal,
    pub action: Action,
    pub resource: Resource,
    pub effect: Effect,
    pub conditions: Vec<Condition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Principal {
    pub principal_type: PrincipalType,
    pub identifier: String,
    pub attributes: HashMap<String, String>, // tags, groups, etc.
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PrincipalType {
    User,
    Role,
    ServiceAccount,
    Group,
    Service,
    External, // Federated users, etc.
    Wildcard,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Action {
    pub service: String,      // s3, ec2, iam, etc.
    pub operation: String,    // GetObject, DescribeInstances, etc.
    pub is_wildcard: bool,    // "*" permissions
    pub privilege_level: PrivilegeLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PrivilegeLevel {
    Read,     // Get, List, Describe
    Write,    // Put, Post, Create, Update
    Delete,   // Delete, Terminate
    Admin,    // Full control, policy changes
    Wildcard, // "*" permissions
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Resource {
    pub resource_type: String,    // bucket, instance, database, etc.
    pub identifier: String,       // ARN, name, or pattern
    pub is_wildcard: bool,       // "*" resources
    pub sensitivity: DataSensitivity,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DataSensitivity {
    Public,
    Internal,
    Confidential,
    Restricted,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Effect {
    Allow,
    Deny,
    Implicit, // No explicit policy
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Condition {
    pub condition_type: String,
    pub key: String,
    pub value: String,
}

/// PAR三組に基づく権限分析エンジン
pub struct PARAnalyzer {
    triplets: Vec<PARTriplet>,
    baseline_policies: BaselinePolicies,
}

#[derive(Debug)]
pub struct BaselinePolicies {
    /// 最小権限の原則に基づく必要最低限の権限
    pub minimum_required: HashMap<String, Vec<PARTriplet>>,
    /// 組織のセキュリティポリシーで禁止されている権限
    pub prohibited: Vec<PARPattern>,
    /// 高リスクとされる権限パターン
    pub high_risk: Vec<PARPattern>,
}

#[derive(Debug, Clone)]
pub struct PARPattern {
    pub principal_pattern: Option<String>,
    pub action_pattern: Option<String>,
    pub resource_pattern: Option<String>,
    pub description: String,
    pub risk_level: RiskLevel,
}

#[derive(Debug, Clone)]
pub enum RiskLevel {
    Critical,
    High,
    Medium,
    Low,
}

impl PARAnalyzer {
    pub fn new() -> Self {
        Self {
            triplets: Vec::new(),
            baseline_policies: BaselinePolicies::default(),
        }
    }
    
    /// IaCコンテンツからPAR三組を抽出
    pub fn extract_par_triplets(&mut self, iac_content: &str, iac_type: &str) -> Result<()> {
        match iac_type {
            "terraform" => self.extract_terraform_pars(iac_content),
            "cloudformation" => self.extract_cloudformation_pars(iac_content),
            "kubernetes" => self.extract_kubernetes_pars(iac_content),
            _ => Ok(()),
        }
    }
    
    fn extract_terraform_pars(&mut self, content: &str) -> Result<()> {
        // Terraform IAM policy 分析
        self.extract_terraform_iam_policies(content)?;
        // Security Group 分析
        self.extract_terraform_security_groups(content)?;
        // Resource-based policies 分析
        self.extract_terraform_resource_policies(content)?;
        
        Ok(())
    }
    
    fn extract_terraform_iam_policies(&mut self, content: &str) -> Result<()> {
        // IAM policy document parsing
        let policy_regex = regex::Regex::new(r#"policy\s*=\s*jsonencode\(\{([^}]+)\}\)"#)?;
        
        for cap in policy_regex.captures_iter(content) {
            let policy_json = &cap[1];
            self.parse_iam_policy_document(policy_json)?;
        }
        
        // IAM role assumptions
        let role_regex = regex::Regex::new(r#"resource\s+"aws_iam_role"\s+"([^"]+)""#)?;
        for cap in role_regex.captures_iter(content) {
            let role_name = &cap[1];
            self.extract_role_assumptions(content, role_name)?;
        }
        
        Ok(())
    }
    
    fn parse_iam_policy_document(&mut self, policy_json: &str) -> Result<()> {
        // Simplified IAM policy parsing
        if policy_json.contains(r#""Action": "*""#) {
            self.triplets.push(PARTriplet {
                principal: Principal {
                    principal_type: PrincipalType::Wildcard,
                    identifier: "*".to_string(),
                    attributes: HashMap::new(),
                },
                action: Action {
                    service: "*".to_string(),
                    operation: "*".to_string(),
                    is_wildcard: true,
                    privilege_level: PrivilegeLevel::Wildcard,
                },
                resource: Resource {
                    resource_type: "*".to_string(),
                    identifier: "*".to_string(),
                    is_wildcard: true,
                    sensitivity: DataSensitivity::Unknown,
                },
                effect: Effect::Allow,
                conditions: Vec::new(),
            });
        }
        
        Ok(())
    }
    
    fn extract_terraform_security_groups(&mut self, content: &str) -> Result<()> {
        // Security groups are network-level PAR triplets
        let sg_regex = regex::Regex::new(r#"resource\s+"aws_security_group"\s+"([^"]+)""#)?;
        
        for cap in sg_regex.captures_iter(content) {
            let sg_name = &cap[1];
            
            // Extract ingress rules
            if content.contains("from_port = 0") && content.contains("to_port = 65535") {
                self.triplets.push(PARTriplet {
                    principal: Principal {
                        principal_type: PrincipalType::External,
                        identifier: "0.0.0.0/0".to_string(),
                        attributes: HashMap::new(),
                    },
                    action: Action {
                        service: "network".to_string(),
                        operation: "all_ports".to_string(),
                        is_wildcard: true,
                        privilege_level: PrivilegeLevel::Wildcard,
                    },
                    resource: Resource {
                        resource_type: "security_group".to_string(),
                        identifier: sg_name.to_string(),
                        is_wildcard: false,
                        sensitivity: DataSensitivity::Internal,
                    },
                    effect: Effect::Allow,
                    conditions: Vec::new(),
                });
            }
        }
        
        Ok(())
    }
    
    fn extract_terraform_resource_policies(&mut self, content: &str) -> Result<()> {
        // S3 bucket policies, etc.
        Ok(())
    }
    
    fn extract_cloudformation_pars(&mut self, content: &str) -> Result<()> {
        // CloudFormation IAM resources analysis
        Ok(())
    }
    
    fn extract_kubernetes_pars(&mut self, content: &str) -> Result<()> {
        // RBAC, Network Policies, Pod Security Policies
        self.extract_kubernetes_rbac(content)?;
        self.extract_kubernetes_network_policies(content)?;
        
        Ok(())
    }
    
    fn extract_kubernetes_rbac(&mut self, content: &str) -> Result<()> {
        // ClusterRole, Role, RoleBinding, ClusterRoleBinding
        if content.contains("kind: ClusterRole") && content.contains("- \"*\"") {
            self.triplets.push(PARTriplet {
                principal: Principal {
                    principal_type: PrincipalType::ServiceAccount,
                    identifier: "unknown".to_string(),
                    attributes: HashMap::new(),
                },
                action: Action {
                    service: "kubernetes".to_string(),
                    operation: "*".to_string(),
                    is_wildcard: true,
                    privilege_level: PrivilegeLevel::Wildcard,
                },
                resource: Resource {
                    resource_type: "cluster".to_string(),
                    identifier: "*".to_string(),
                    is_wildcard: true,
                    sensitivity: DataSensitivity::Confidential,
                },
                effect: Effect::Allow,
                conditions: Vec::new(),
            });
        }
        
        Ok(())
    }
    
    fn extract_kubernetes_network_policies(&mut self, content: &str) -> Result<()> {
        // NetworkPolicy analysis for network-level PAR
        Ok(())
    }
    
    fn extract_role_assumptions(&mut self, content: &str, role_name: &str) -> Result<()> {
        // Extract who can assume this role
        Ok(())
    }
    
    /// 権限過大の検出
    pub fn detect_excessive_privileges(&self) -> Vec<PrivilegeViolation> {
        let mut violations = Vec::new();
        
        for triplet in &self.triplets {
            // 1. ワイルドカード権限の検出
            if triplet.action.is_wildcard && triplet.resource.is_wildcard {
                violations.push(PrivilegeViolation {
                    violation_type: ViolationType::ExcessivePrivilege,
                    triplet: triplet.clone(),
                    description: "Wildcard permissions (*) on all resources".to_string(),
                    risk_level: RiskLevel::Critical,
                    remediation: "Apply principle of least privilege - grant only specific permissions needed".to_string(),
                });
            }
            
            // 2. 管理者権限の不適切な付与
            if matches!(triplet.action.privilege_level, PrivilegeLevel::Admin) &&
               !matches!(triplet.principal.principal_type, PrincipalType::User) {
                violations.push(PrivilegeViolation {
                    violation_type: ViolationType::AdminPrivilegeToNonUser,
                    triplet: triplet.clone(),
                    description: "Administrative privileges granted to service account or role".to_string(),
                    risk_level: RiskLevel::High,
                    remediation: "Review if admin privileges are necessary, consider more granular permissions".to_string(),
                });
            }
            
            // 3. 機密リソースへの広範囲アクセス
            if matches!(triplet.resource.sensitivity, DataSensitivity::Confidential | DataSensitivity::Restricted) &&
               triplet.principal.principal_type == PrincipalType::External {
                violations.push(PrivilegeViolation {
                    violation_type: ViolationType::SensitiveResourceExposure,
                    triplet: triplet.clone(),
                    description: "External principals have access to sensitive resources".to_string(),
                    risk_level: RiskLevel::Critical,
                    remediation: "Restrict access to confidential resources to internal principals only".to_string(),
                });
            }
        }
        
        violations
    }
    
    /// 権限昇格パスの検出
    pub fn detect_privilege_escalation_paths(&self) -> Vec<EscalationPath> {
        let mut paths = Vec::new();
        
        // Build privilege graph
        let privilege_graph = self.build_privilege_graph();
        
        // Find paths from low-privilege to high-privilege
        for start_principal in self.get_low_privilege_principals() {
            for escalation_path in privilege_graph.find_escalation_paths(&start_principal) {
                paths.push(escalation_path);
            }
        }
        
        paths
    }
    
    fn build_privilege_graph(&self) -> PrivilegeGraph {
        let mut graph = PrivilegeGraph::new();
        
        for triplet in &self.triplets {
            graph.add_triplet(triplet);
        }
        
        // Add transitive relationships
        graph.compute_transitive_privileges();
        
        graph
    }
    
    fn get_low_privilege_principals(&self) -> Vec<Principal> {
        self.triplets.iter()
            .filter(|t| matches!(t.action.privilege_level, PrivilegeLevel::Read))
            .map(|t| t.principal.clone())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }
    
    /// 最小権限原則のチェック
    pub fn check_least_privilege_principle(&self) -> Vec<LeastPrivilegeViolation> {
        let mut violations = Vec::new();
        
        // Group triplets by principal
        let mut principal_permissions: HashMap<Principal, Vec<&PARTriplet>> = HashMap::new();
        for triplet in &self.triplets {
            principal_permissions.entry(triplet.principal.clone())
                .or_insert_with(Vec::new)
                .push(triplet);
        }
        
        for (principal, triplets) in principal_permissions {
            let required = self.baseline_policies.minimum_required
                .get(&principal.identifier)
                .unwrap_or(&Vec::new());
            
            let excessive = self.find_excessive_permissions(triplets, required);
            if !excessive.is_empty() {
                violations.push(LeastPrivilegeViolation {
                    principal: principal.clone(),
                    excessive_permissions: excessive.into_iter().cloned().collect(),
                    description: format!("Principal {} has more permissions than required", principal.identifier),
                });
            }
        }
        
        violations
    }
    
    fn find_excessive_permissions(
        &self,
        granted: Vec<&PARTriplet>,
        required: &[PARTriplet],
    ) -> Vec<&PARTriplet> {
        granted.into_iter()
            .filter(|g| !required.iter().any(|r| self.is_permission_subset(g, r)))
            .collect()
    }
    
    fn is_permission_subset(&self, granted: &PARTriplet, required: &PARTriplet) -> bool {
        // Check if granted permission is a subset of required permission
        granted.action.service == required.action.service &&
        granted.action.operation == required.action.operation &&
        granted.resource.identifier == required.resource.identifier
    }
}

#[derive(Debug, Clone)]
pub struct PrivilegeViolation {
    pub violation_type: ViolationType,
    pub triplet: PARTriplet,
    pub description: String,
    pub risk_level: RiskLevel,
    pub remediation: String,
}

#[derive(Debug, Clone)]
pub enum ViolationType {
    ExcessivePrivilege,
    AdminPrivilegeToNonUser,
    SensitiveResourceExposure,
    CrossAccountAccess,
    PrivilegeEscalation,
}

#[derive(Debug)]
pub struct EscalationPath {
    pub start_principal: Principal,
    pub end_principal: Principal,
    pub steps: Vec<PARTriplet>,
    pub risk_assessment: String,
}

#[derive(Debug)]
pub struct LeastPrivilegeViolation {
    pub principal: Principal,
    pub excessive_permissions: Vec<PARTriplet>,
    pub description: String,
}

#[derive(Debug)]
pub struct PrivilegeGraph {
    principals: HashSet<Principal>,
    edges: Vec<PrivilegeEdge>,
}

#[derive(Debug)]
pub struct PrivilegeEdge {
    pub from: Principal,
    pub to: Principal,
    pub via_action: Action,
    pub on_resource: Resource,
}

impl PrivilegeGraph {
    pub fn new() -> Self {
        Self {
            principals: HashSet::new(),
            edges: Vec::new(),
        }
    }
    
    pub fn add_triplet(&mut self, triplet: &PARTriplet) {
        self.principals.insert(triplet.principal.clone());
        
        // Check for privilege escalation opportunities
        if triplet.action.operation == "AssumeRole" {
            // This represents a potential escalation edge
            let target_principal = Principal {
                principal_type: PrincipalType::Role,
                identifier: triplet.resource.identifier.clone(),
                attributes: HashMap::new(),
            };
            
            self.principals.insert(target_principal.clone());
            self.edges.push(PrivilegeEdge {
                from: triplet.principal.clone(),
                to: target_principal,
                via_action: triplet.action.clone(),
                on_resource: triplet.resource.clone(),
            });
        }
    }
    
    pub fn compute_transitive_privileges(&mut self) {
        // Implement Floyd-Warshall or similar for transitive closure
    }
    
    pub fn find_escalation_paths(&self, start: &Principal) -> Vec<EscalationPath> {
        let mut paths = Vec::new();
        
        // DFS to find escalation paths
        self.dfs_escalation_paths(start, start, Vec::new(), &mut paths, 0, 5);
        
        paths
    }
    
    fn dfs_escalation_paths(
        &self,
        current: &Principal,
        start: &Principal,
        path: Vec<PARTriplet>,
        results: &mut Vec<EscalationPath>,
        depth: usize,
        max_depth: usize,
    ) {
        if depth > max_depth {
            return;
        }
        
        for edge in &self.edges {
            if &edge.from == current && &edge.to != start {
                let mut new_path = path.clone();
                new_path.push(PARTriplet {
                    principal: edge.from.clone(),
                    action: edge.via_action.clone(),
                    resource: edge.on_resource.clone(),
                    effect: Effect::Allow,
                    conditions: Vec::new(),
                });
                
                // Check if this represents significant privilege escalation
                if self.is_significant_escalation(start, &edge.to) {
                    results.push(EscalationPath {
                        start_principal: start.clone(),
                        end_principal: edge.to.clone(),
                        steps: new_path.clone(),
                        risk_assessment: self.assess_escalation_risk(&new_path),
                    });
                }
                
                // Continue exploring
                self.dfs_escalation_paths(&edge.to, start, new_path, results, depth + 1, max_depth);
            }
        }
    }
    
    fn is_significant_escalation(&self, start: &Principal, end: &Principal) -> bool {
        // Define what constitutes significant privilege escalation
        matches!(end.principal_type, PrincipalType::Role) &&
        end.identifier.contains("admin") ||
        end.identifier.contains("root")
    }
    
    fn assess_escalation_risk(&self, path: &[PARTriplet]) -> String {
        format!("Escalation through {} steps", path.len())
    }
}

impl Default for BaselinePolicies {
    fn default() -> Self {
        Self {
            minimum_required: HashMap::new(),
            prohibited: vec![
                PARPattern {
                    principal_pattern: Some("*".to_string()),
                    action_pattern: Some("*".to_string()),
                    resource_pattern: Some("*".to_string()),
                    description: "Wildcard permissions prohibited".to_string(),
                    risk_level: RiskLevel::Critical,
                },
            ],
            high_risk: vec![
                PARPattern {
                    principal_pattern: None,
                    action_pattern: Some("iam:*".to_string()),
                    resource_pattern: Some("*".to_string()),
                    description: "IAM administrative permissions".to_string(),
                    risk_level: RiskLevel::High,
                },
            ],
        }
    }
}