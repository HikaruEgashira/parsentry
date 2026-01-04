//! Infrastructure as Code (IaC) analysis prompt.

use parsentry_i18n::Language;

use crate::templates;
use crate::traits::Prompt;
use crate::types::{sanitize_for_prompt, ComplianceFramework, IacFocusArea, IacType, TargetFile};

/// Declarative IaC analysis prompt.
///
/// Used for analyzing Infrastructure as Code files like Terraform,
/// CloudFormation, Kubernetes manifests, and Docker configurations.
#[derive(Debug, Clone)]
pub struct IacAnalysisPrompt {
    /// Target IaC file to analyze.
    pub target: TargetFile,
    /// Type of IaC configuration.
    pub iac_type: IacType,
    /// Compliance frameworks to check against.
    pub compliance_frameworks: Vec<ComplianceFramework>,
    /// Focus areas for the analysis.
    pub focus_areas: Vec<IacFocusArea>,
    /// Language for the response.
    pub language: Language,
}

impl IacAnalysisPrompt {
    /// Create a new IaC analysis prompt with auto-detected type.
    pub fn new(target: TargetFile) -> Self {
        let iac_type = detect_iac_type(&target.path.display().to_string());
        Self {
            target,
            iac_type,
            compliance_frameworks: Vec::new(),
            focus_areas: Vec::new(),
            language: Language::Japanese,
        }
    }

    /// Set compliance frameworks.
    pub fn with_compliance(mut self, frameworks: Vec<ComplianceFramework>) -> Self {
        self.compliance_frameworks = frameworks;
        self
    }

    /// Set focus areas.
    pub fn with_focus_areas(mut self, areas: Vec<IacFocusArea>) -> Self {
        self.focus_areas = areas;
        self
    }

    /// Set language.
    pub fn with_language(mut self, language: Language) -> Self {
        self.language = language;
        self
    }
}

impl Prompt for IacAnalysisPrompt {
    fn render(&self) -> String {
        let safe_path = sanitize_for_prompt(&self.target.path.display().to_string());
        let safe_content = sanitize_for_prompt(&self.target.content);

        let iac_instructions = templates::render_iac_type_instructions(self.iac_type);
        let compliance_section = templates::render_compliance_frameworks(&self.compliance_frameworks);
        let focus_section = templates::render_focus_areas(&self.focus_areas);
        let lang_instruction = templates::render_language_instruction(&self.language);

        format!(
            r#"You are an Infrastructure as Code security analyzer.

## Analysis Target

File: {file_path}
Type: {iac_type:?}

## Configuration Content

```
{content}
```

{iac_instructions}
{compliance}
{focus}
## Instructions

Analyze this IaC configuration for security misconfigurations:

1. **Identify Security Misconfigurations**: Find insecure defaults, overly permissive settings
2. **Check Access Controls**: Review IAM policies, RBAC, service accounts
3. **Evaluate Network Security**: Examine security groups, network policies, exposed ports
4. **Assess Data Protection**: Check encryption settings, secret management
5. **Verify Compliance**: Ensure alignment with security best practices

## Output Format

{lang_instruction}

Respond with a JSON object containing:

```json
{{
  "scratchpad": "Your analysis reasoning",
  "findings": [
    {{
      "severity": "critical|high|medium|low",
      "category": "security category",
      "resource": "affected resource",
      "issue": "description of the issue",
      "recommendation": "how to fix it",
      "compliance_references": ["CIS 1.2.3", "NIST AC-1"]
    }}
  ],
  "overall_risk_level": "critical|high|medium|low",
  "summary": "Brief security assessment summary"
}}
```"#,
            file_path = safe_path,
            iac_type = self.iac_type,
            content = safe_content,
            iac_instructions = iac_instructions,
            compliance = compliance_section,
            focus = focus_section,
            lang_instruction = lang_instruction,
        )
    }
}

/// Detect IaC type from file path.
fn detect_iac_type(path: &str) -> IacType {
    let path_lower = path.to_lowercase();

    if path_lower.ends_with(".tf") || path_lower.ends_with(".tfvars") {
        IacType::Terraform
    } else if path_lower.contains("cloudformation")
        || path_lower.ends_with(".template")
        || (path_lower.ends_with(".yaml") && path_lower.contains("cfn"))
    {
        IacType::CloudFormation
    } else if path_lower.contains("kubernetes")
        || path_lower.contains("k8s")
        || path_lower.ends_with(".yaml")
        || path_lower.ends_with(".yml")
    {
        IacType::Kubernetes
    } else if path_lower.contains("docker") || path_lower.ends_with("dockerfile") {
        IacType::Docker
    } else {
        IacType::Terraform // Default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iac_analysis_prompt_terraform() {
        let prompt = IacAnalysisPrompt {
            target: TargetFile::new(
                "main.tf",
                r#"resource "aws_s3_bucket" "example" {
  bucket = "my-bucket"
}"#,
            ),
            iac_type: IacType::Terraform,
            compliance_frameworks: vec![ComplianceFramework::CIS, ComplianceFramework::NIST],
            focus_areas: vec![IacFocusArea::DataProtection, IacFocusArea::AccessControl],
            language: Language::Japanese,
        };

        let rendered = prompt.render();

        assert!(rendered.contains("main.tf"));
        assert!(rendered.contains("Terraform"));
        assert!(rendered.contains("CIS Benchmark"));
        assert!(rendered.contains("Data Protection"));
    }

    #[test]
    fn test_iac_analysis_prompt_kubernetes() {
        let prompt = IacAnalysisPrompt::new(TargetFile::new(
            "deployment.yaml",
            r#"apiVersion: apps/v1
kind: Deployment
metadata:
  name: nginx"#,
        ))
        .with_focus_areas(vec![IacFocusArea::NetworkSecurity])
        .with_language(Language::English);

        let rendered = prompt.render();

        assert!(rendered.contains("Kubernetes"));
        assert!(rendered.contains("Network Security"));
    }

    #[test]
    fn test_detect_iac_type() {
        assert_eq!(detect_iac_type("main.tf"), IacType::Terraform);
        assert_eq!(detect_iac_type("vars.tfvars"), IacType::Terraform);
        assert_eq!(
            detect_iac_type("cloudformation-stack.yaml"),
            IacType::CloudFormation
        );
        assert_eq!(detect_iac_type("k8s/deployment.yaml"), IacType::Kubernetes);
        assert_eq!(detect_iac_type("Dockerfile"), IacType::Docker);
    }
}
