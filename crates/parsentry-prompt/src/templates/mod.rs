//! Template sections for prompt construction.

use parsentry_i18n::Language;

use crate::{ComplianceFramework, IacFocusArea, IacType, OutputFormat, PatternContext, PocMode, RelatedFunction};

/// Render the role section.
pub fn render_role() -> &'static str {
    "You are a security vulnerability analyzer with access to code execution and file operations."
}

/// Render the target section.
pub fn render_target(file_path: &str, content: &str) -> String {
    format!(
        r#"## Analysis Target

File: {}

## Source Code

```
{}
```"#,
        file_path, content
    )
}

/// Render the pattern context section.
pub fn render_pattern(pattern: &PatternContext) -> String {
    format!(
        r#"
## Pattern Context

- **Pattern Type**: {}
- **Description**: {}
- **Matched Code**:
```
{}
```
- **Attack Vectors**: {}"#,
        pattern.pattern_type,
        pattern.description,
        pattern.matched_code,
        pattern.attack_vectors.join(", ")
    )
}

/// Render the PAR framework instructions.
pub fn render_par_instructions() -> &'static str {
    r#"## Instructions

Analyze this code for security vulnerabilities using the PAR (Principal-Action-Resource) framework:

1. **Identify Principals**: Find untrusted data sources (user input, external APIs, file uploads)
2. **Identify Resources**: Find sensitive operations (database queries, file system, command execution)
3. **Evaluate Actions**: Assess security controls between principals and resources
4. **Detect Policy Violations**: Identify paths where untrusted data reaches resources without proper validation

## PAR Analysis Framework

- **Principal**: Untrusted data sources (user input, external APIs, environment variables)
- **Action**: Security controls (validation, sanitization, authentication, authorization)
- **Resource**: Sensitive operations (DB, file system, command execution, network)"#
}

/// Render the deep context instructions.
pub fn render_deep_context() -> &'static str {
    r#"
## Deep Context Analysis

You have access to file operations. Use them to:
- Read imported modules to understand function implementations
- Search for security-related patterns in the codebase
- Trace call chains to identify complete attack paths
- Read configuration files that may affect security"#
}

/// Render the PoC instructions based on mode.
pub fn render_poc_instructions(mode: PocMode) -> &'static str {
    match mode {
        PocMode::Disabled => "",
        PocMode::GenerateOnly => {
            r#"
## PoC Generation

If a vulnerability is confirmed, generate a PoC code that demonstrates the vulnerability.
Do NOT execute the PoC - only generate the code."#
        }
        PocMode::ExecuteIfSafe => {
            r#"
## PoC Verification

If a vulnerability is confirmed:
1. Create a minimal PoC demonstrating the vulnerability
2. Execute the PoC in a safe, read-only manner if possible
3. Document the actual behavior observed

Safety Constraints:
- DO NOT execute destructive operations
- DO NOT access sensitive files outside the target
- Use read-only operations when possible
- Report theoretical impact if execution is unsafe"#
        }
    }
}

/// Render the language instruction.
pub fn render_language_instruction(language: &Language) -> &'static str {
    match language {
        Language::Japanese => {
            "Respond in Japanese. All analysis text, descriptions, and explanations MUST be written in Japanese (日本語)."
        }
        Language::English => "Respond in English.",
    }
}

/// Render the JSON output format.
pub fn render_json_output() -> &'static str {
    r#"Respond with a JSON object containing:

```json
{
  "scratchpad": "Your analysis reasoning and notes",
  "analysis": "Comprehensive security assessment",
  "poc": "Proof of concept code if vulnerability found",
  "confidence_score": 0-100,
  "vulnerability_types": ["LFI", "RCE", "SSRF", "AFO", "SQLI", "XSS", "IDOR"],
  "par_analysis": {
    "principals": [
      {
        "identifier": "variable or function name",
        "trust_level": "trusted|semi_trusted|untrusted",
        "source_context": "description of data source",
        "risk_factors": ["list of risk factors"]
      }
    ],
    "actions": [
      {
        "identifier": "validation function name",
        "security_function": "what it's supposed to do",
        "implementation_quality": "adequate|insufficient|missing|bypassed",
        "detected_weaknesses": ["list of weaknesses"],
        "bypass_vectors": ["potential bypass methods"]
      }
    ],
    "resources": [
      {
        "identifier": "sensitive operation",
        "sensitivity_level": "low|medium|high|critical",
        "operation_type": "type of operation",
        "protection_mechanisms": ["existing protections"]
      }
    ],
    "policy_violations": [
      {
        "rule_id": "violation identifier",
        "rule_description": "what policy was violated",
        "violation_path": "principal -> action -> resource path",
        "severity": "low|medium|high|critical",
        "confidence": 0.0-1.0
      }
    ]
  },
  "remediation_guidance": {
    "policy_enforcement": [
      {
        "component": "what needs fixing",
        "required_improvement": "what improvement is needed",
        "specific_guidance": "how to fix it",
        "priority": "low|medium|high|critical"
      }
    ]
  }
}
```"#
}

/// Render the SARIF output format instructions.
pub fn render_sarif_output(output_path: &str, language: &Language) -> String {
    let lang_name = match language {
        Language::Japanese => "Japanese",
        Language::English => "English",
    };

    format!(
        r#"**IMPORTANT**: You MUST output the analysis result as a SARIF JSON file.

1. First, read the target file to analyze the code
2. Perform security analysis using the PAR framework
3. If vulnerabilities are found (confidence >= 70), use the Write tool to save SARIF JSON to: `{}`

The SARIF file MUST follow this exact structure:

```json
{{
  "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
  "version": "2.1.0",
  "runs": [{{
    "tool": {{
      "driver": {{
        "name": "Parsentry",
        "version": "0.14.0",
        "rules": [
          {{
            "id": "SQLI",
            "name": "SQL Injection",
            "shortDescription": {{ "text": "..." }}
          }}
        ]
      }}
    }},
    "results": [
      {{
        "ruleId": "SQLI",
        "ruleIndex": 0,
        "level": "error",
        "message": {{ "text": "...", "markdown": "..." }},
        "locations": [{{
          "physicalLocation": {{
            "artifactLocation": {{ "uri": "path/to/file.py" }},
            "region": {{ "startLine": 42 }}
          }}
        }}],
        "properties": {{ "confidence": 0.9 }}
      }}
    ]
  }}]
}}
```

**CRITICAL REQUIRED FIELDS**:
- Each rule in `rules` array MUST have: `id`, `name`, `shortDescription`
- Each result MUST have: `ruleId`, `ruleIndex` (index of the rule in rules array), `level`, `message`, `locations`
- `ruleIndex` MUST match the index of the corresponding rule in the `rules` array (0-based)

Vulnerability Types (ruleId/name):
- SQLI / SQL Injection
- XSS / Cross-Site Scripting
- RCE / Remote Code Execution
- LFI / Local File Inclusion
- SSRF / Server-Side Request Forgery
- AFO / Arbitrary File Operation
- IDOR / Insecure Direct Object Reference

Write the detailed analysis in `message.markdown` field in {}.

If no vulnerability found or confidence < 0.7, do NOT create the SARIF file."#,
        output_path, lang_name
    )
}

/// Render the output format section.
pub fn render_output_format(format: &OutputFormat, language: &Language) -> String {
    let lang_instruction = render_language_instruction(language);

    match format {
        OutputFormat::Json => {
            format!(
                r#"## Output Format

{}

{}"#,
                lang_instruction,
                render_json_output()
            )
        }
        OutputFormat::Sarif { output_path } => {
            format!(
                r#"## Output Instructions

{}"#,
                render_sarif_output(&output_path.display().to_string(), language)
            )
        }
    }
}

/// Render the important notes section.
pub fn render_important_notes() -> &'static str {
    r#"## Important Notes

- confidence_score: Set to 0 if no vulnerability is found
- Normalize confidence_score to multiples of 10 (0, 10, 20, ..., 100)
- Only report vulnerabilities with high confidence (>= 70)
- Include full attack path in violation_path"#
}

/// Render related functions section.
pub fn render_related_functions(functions: &[RelatedFunction]) -> String {
    if functions.is_empty() {
        return String::new();
    }

    let mut section = String::from("\n## Related Functions\n");
    for func in functions {
        section.push_str(&format!("{}:{} {}\n", func.path, func.line, func.name));
    }
    section
}

/// Render IaC type specific instructions.
pub fn render_iac_type_instructions(iac_type: IacType) -> &'static str {
    match iac_type {
        IacType::Terraform => {
            r#"## Terraform-Specific Analysis

Focus on:
- Provider configuration security
- Resource access policies
- State file handling
- Backend configuration
- Module security"#
        }
        IacType::CloudFormation => {
            r#"## CloudFormation-Specific Analysis

Focus on:
- IAM roles and policies
- Security group configurations
- KMS key management
- Secrets management
- Cross-stack references"#
        }
        IacType::Kubernetes => {
            r#"## Kubernetes-Specific Analysis

Focus on:
- Pod security contexts
- Network policies
- RBAC configurations
- Secret management
- Resource limits"#
        }
        IacType::Docker => {
            r#"## Docker-Specific Analysis

Focus on:
- Base image security
- User privileges
- Exposed ports
- Volume mounts
- Build-time secrets"#
        }
    }
}

/// Render compliance framework references.
pub fn render_compliance_frameworks(frameworks: &[ComplianceFramework]) -> String {
    if frameworks.is_empty() {
        return String::new();
    }

    let mut section = String::from("\n## Compliance Frameworks\n\nCheck against:\n");
    for framework in frameworks {
        let name = match framework {
            ComplianceFramework::CIS => "CIS Benchmark",
            ComplianceFramework::NIST => "NIST Cybersecurity Framework",
            ComplianceFramework::SOC2 => "SOC 2 Type II",
            ComplianceFramework::PCIDSS => "PCI DSS",
            ComplianceFramework::HIPAA => "HIPAA",
        };
        section.push_str(&format!("- {}\n", name));
    }
    section
}

/// Render focus areas for IaC analysis.
pub fn render_focus_areas(areas: &[IacFocusArea]) -> String {
    if areas.is_empty() {
        return String::new();
    }

    let mut section = String::from("\n## Focus Areas\n\n");
    for area in areas {
        let desc = match area {
            IacFocusArea::NetworkSecurity => "Network Security: VPC, subnets, security groups, NACLs",
            IacFocusArea::AccessControl => "Access Control: IAM, RBAC, service accounts",
            IacFocusArea::DataProtection => "Data Protection: Encryption, key management, secrets",
            IacFocusArea::MonitoringLogging => "Monitoring & Logging: CloudWatch, audit logs, alerts",
            IacFocusArea::ResourceManagement => "Resource Management: Limits, quotas, tagging",
        };
        section.push_str(&format!("- {}\n", desc));
    }
    section
}
