//! Prompt builder for Codex security analysis.

use std::path::Path;

/// Sanitize input to prevent prompt injection attacks.
fn sanitize_for_prompt(text: &str) -> String {
    text.replace("```", "\\`\\`\\`")
        .chars()
        .filter(|c| !c.is_control() || c.is_whitespace())
        .collect()
}

/// Builder for constructing Claude Code security analysis prompts.
#[derive(Debug, Clone)]
pub struct PromptBuilder {
    /// Enable PoC execution in prompts.
    enable_poc: bool,
    /// Enable file operations (reading related files).
    enable_file_ops: bool,
    /// Language for responses (e.g., "ja", "en").
    language: String,
}

impl Default for PromptBuilder {
    fn default() -> Self {
        Self {
            enable_poc: false,
            enable_file_ops: true,
            language: "ja".to_string(),
        }
    }
}

impl PromptBuilder {
    /// Create a new prompt builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable PoC execution.
    pub fn with_poc(mut self, enable: bool) -> Self {
        self.enable_poc = enable;
        self
    }

    /// Enable file operations.
    pub fn with_file_ops(mut self, enable: bool) -> Self {
        self.enable_file_ops = enable;
        self
    }

    /// Set the response language.
    pub fn with_language(mut self, lang: &str) -> Self {
        self.language = lang.to_string();
        self
    }

    /// Build a security analysis prompt for a file.
    pub fn build_security_analysis_prompt(
        &self,
        file_path: &Path,
        content: &str,
        pattern_context: Option<&PatternContext>,
    ) -> String {
        let file_ops_instruction = if self.enable_file_ops {
            r#"
## Deep Context Analysis

You have access to file operations. Use them to:
- Read imported modules to understand function implementations
- Search for security-related patterns in the codebase
- Trace call chains to identify complete attack paths
- Read configuration files that may affect security"#
        } else {
            ""
        };

        let poc_instruction = if self.enable_poc {
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
        } else {
            r#"
## PoC Generation

If a vulnerability is confirmed, generate a PoC code that demonstrates the vulnerability.
Do NOT execute the PoC - only generate the code."#
        };

        let pattern_section = if let Some(ctx) = pattern_context {
            format!(
                r#"
## Pattern Context

- **Pattern Type**: {}
- **Description**: {}
- **Matched Code**:
```
{}
```
- **Attack Vectors**: {}
"#,
                ctx.pattern_type,
                ctx.description,
                ctx.matched_code,
                ctx.attack_vectors.join(", ")
            )
        } else {
            String::new()
        };

        let lang_instruction = if self.language == "ja" {
            "Respond in Japanese."
        } else {
            "Respond in English."
        };

        let safe_file_path = sanitize_for_prompt(&file_path.display().to_string());
        let safe_content = sanitize_for_prompt(content);

        format!(
            r#"You are a security vulnerability analyzer with access to code execution and file operations.

## Analysis Target

File: {file_path}
{pattern_section}

## Source Code

```
{content}
```

## Instructions

Analyze this code for security vulnerabilities using the PAR (Principal-Action-Resource) framework:

1. **Identify Principals**: Find untrusted data sources (user input, external APIs, file uploads)
2. **Identify Resources**: Find sensitive operations (database queries, file system, command execution)
3. **Evaluate Actions**: Assess security controls between principals and resources
4. **Detect Policy Violations**: Identify paths where untrusted data reaches resources without proper validation
{file_ops_instruction}
{poc_instruction}

## PAR Analysis Framework

- **Principal**: Untrusted data sources (user input, external APIs, environment variables)
- **Action**: Security controls (validation, sanitization, authentication, authorization)
- **Resource**: Sensitive operations (DB, file system, command execution, network)

## Output Format

{lang_instruction}

Respond with a JSON object containing:

```json
{{
  "scratchpad": "Your analysis reasoning and notes",
  "analysis": "Comprehensive security assessment",
  "poc": "Proof of concept code if vulnerability found",
  "confidence_score": 0-100,
  "vulnerability_types": ["LFI", "RCE", "SSRF", "AFO", "SQLI", "XSS", "IDOR"],
  "par_analysis": {{
    "principals": [
      {{
        "identifier": "variable or function name",
        "trust_level": "trusted|semi_trusted|untrusted",
        "source_context": "description of data source",
        "risk_factors": ["list of risk factors"]
      }}
    ],
    "actions": [
      {{
        "identifier": "validation function name",
        "security_function": "what it's supposed to do",
        "implementation_quality": "adequate|insufficient|missing|bypassed",
        "detected_weaknesses": ["list of weaknesses"],
        "bypass_vectors": ["potential bypass methods"]
      }}
    ],
    "resources": [
      {{
        "identifier": "sensitive operation",
        "sensitivity_level": "low|medium|high|critical",
        "operation_type": "type of operation",
        "protection_mechanisms": ["existing protections"]
      }}
    ],
    "policy_violations": [
      {{
        "rule_id": "violation identifier",
        "rule_description": "what policy was violated",
        "violation_path": "principal -> action -> resource path",
        "severity": "low|medium|high|critical",
        "confidence": 0.0-1.0
      }}
    ]
  }},
  "remediation_guidance": {{
    "policy_enforcement": [
      {{
        "component": "what needs fixing",
        "required_improvement": "what improvement is needed",
        "specific_guidance": "how to fix it",
        "priority": "low|medium|high|critical"
      }}
    ]
  }}
}}
```

## Important Notes

- confidence_score: Set to 0 if no vulnerability is found
- Normalize confidence_score to multiples of 10 (0, 10, 20, ..., 100)
- Only report vulnerabilities with high confidence (>= 70)
- Include full attack path in violation_path
"#,
            file_path = safe_file_path,
            content = safe_content,
            pattern_section = pattern_section,
            file_ops_instruction = file_ops_instruction,
            poc_instruction = poc_instruction,
            lang_instruction = lang_instruction,
        )
    }

    /// Build a prompt for vulnerability verification.
    pub fn build_verification_prompt(
        &self,
        vuln_type: &str,
        file_path: &Path,
        analysis: &str,
    ) -> String {
        format!(
            r#"## Vulnerability Verification Task

Verify the following potential vulnerability:

- **Type**: {vuln_type}
- **Location**: {file_path}
- **Initial Analysis**: {analysis}

## Verification Instructions

1. Read the source code at the specified location
2. Trace the data flow from source to sink
3. Identify any security controls in the path
4. Determine if the vulnerability is exploitable
5. Create a minimal PoC if confirmed

## Output Format

Respond with the same JSON format as the initial analysis, but with:
- Updated confidence_score based on verification
- More detailed par_analysis
- Verified or refined poc code
"#,
            vuln_type = vuln_type,
            file_path = file_path.display(),
            analysis = analysis,
        )
    }

    /// Build a prompt for direct SARIF output mode.
    /// The LLM will output SARIF JSON directly to a file.
    pub fn build_sarif_output_prompt(
        &self,
        file_path: &Path,
        content: &str,
        sarif_output_path: &Path,
        pattern_context: Option<&PatternContext>,
    ) -> String {
        let file_ops_instruction = if self.enable_file_ops {
            r#"
## Deep Context Analysis

You have access to file operations. Use them to:
- Read imported modules to understand function implementations
- Search for security-related patterns in the codebase
- Trace call chains to identify complete attack paths
- Read configuration files that may affect security"#
        } else {
            ""
        };

        let poc_instruction = if self.enable_poc {
            r#"
## PoC Verification

If a vulnerability is confirmed:
1. Create a minimal PoC demonstrating the vulnerability
2. Execute the PoC in a safe, read-only manner if possible
3. Document the actual behavior observed"#
        } else {
            ""
        };

        let pattern_section = if let Some(ctx) = pattern_context {
            format!(
                r#"
## Pattern Context

- **Pattern Type**: {}
- **Description**: {}
- **Matched Code**:
```
{}
```
- **Attack Vectors**: {}
"#,
                ctx.pattern_type,
                ctx.description,
                ctx.matched_code,
                ctx.attack_vectors.join(", ")
            )
        } else {
            String::new()
        };

        let safe_file_path = sanitize_for_prompt(&file_path.display().to_string());
        let safe_content = sanitize_for_prompt(content);
        let safe_sarif_path = sarif_output_path.display().to_string();

        format!(
            r#"You are a security vulnerability analyzer. Analyze the code and output results in SARIF format.

## Analysis Target

File: {file_path}
{pattern_section}

## Source Code

```
{content}
```

## Instructions

Analyze this code for security vulnerabilities using the PAR (Principal-Action-Resource) framework:

1. **Identify Principals**: Find untrusted data sources (user input, external APIs, file uploads)
2. **Identify Resources**: Find sensitive operations (database queries, file system, command execution)
3. **Evaluate Actions**: Assess security controls between principals and resources
4. **Detect Policy Violations**: Identify paths where untrusted data reaches resources without proper validation
{file_ops_instruction}
{poc_instruction}

## Output Instructions

**IMPORTANT**: Output SARIF JSON directly. Use the Write tool to save to: `{sarif_path}`

The SARIF file MUST follow SARIF v2.1.0 schema with this structure:

- $schema: https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json
- version: 2.1.0
- runs[0].tool.driver.name: Parsentry
- runs[0].tool.driver.version: 0.13.0
- runs[0].tool.driver.rules: Array of rule definitions
- runs[0].results: Array of findings

Each result must have:
- ruleId: One of SQLI, XSS, RCE, LFI, SSRF, AFO, IDOR
- ruleIndex: Index of the rule in rules array
- level: error (confidence >= 90), warning (>= 70), note (>= 50)
- message.text: Brief description
- message.markdown: Detailed analysis
- locations[0].physicalLocation.artifactLocation.uri: File path
- locations[0].physicalLocation.region.startLine: Line number
- locations[0].physicalLocation.region.snippet.text: Matched code
- properties.confidence: Float 0.0-1.0
- properties.cwe: Array of CWE IDs
- properties.owasp: Array of OWASP categories
- properties.mitre_attack: Array of MITRE ATT&CK IDs

## Vulnerability Types (ruleId)

Use these exact values:
- SQLI: SQL Injection
- XSS: Cross-Site Scripting
- RCE: Remote Code Execution
- LFI: Local File Inclusion
- SSRF: Server-Side Request Forgery
- AFO: Arbitrary File Operation
- IDOR: Insecure Direct Object Reference

## Important Notes

- If no vulnerability found or confidence < 0.5, output empty results array
- Only report vulnerabilities with confidence >= 0.7
- Include CWE, OWASP, and MITRE ATT&CK mappings when applicable
- The help field in rules should contain remediation guidance
"#,
            file_path = safe_file_path,
            content = safe_content,
            pattern_section = pattern_section,
            file_ops_instruction = file_ops_instruction,
            poc_instruction = poc_instruction,
            sarif_path = safe_sarif_path,
        )
    }

    /// Build a security analysis prompt using file references only.
    ///
    /// This is optimized for agents (Claude Code, Codex) that can read files themselves,
    /// reducing context size by passing only file paths instead of full content.
    pub fn build_file_reference_prompt(
        &self,
        file_path: &Path,
        pattern_context: Option<&PatternContext>,
    ) -> String {
        let pattern_section = if let Some(ctx) = pattern_context {
            format!(
                r#"
## Pattern Context
- **Pattern Type**: {}
- **Description**: {}
- **Matched Code**:
```
{}
```
- **Attack Vectors**: {}
"#,
                ctx.pattern_type,
                ctx.description,
                ctx.matched_code,
                ctx.attack_vectors.join(", ")
            )
        } else {
            String::new()
        };

        let lang_instruction = if self.language == "ja" {
            "Respond in Japanese."
        } else {
            "Respond in English."
        };

        let safe_file_path = sanitize_for_prompt(&file_path.display().to_string());

        format!(
            r#"You are a security vulnerability analyzer.

## Analysis Target
{file_path}
{pattern_section}
## Instructions

Analyze the target for security vulnerabilities using the PAR (Principal-Action-Resource) framework:

1. **Identify Principals**: Find untrusted data sources (user input, external APIs, file uploads)
2. **Identify Resources**: Find sensitive operations (database queries, file system, command execution)
3. **Evaluate Actions**: Assess security controls between principals and resources
4. **Detect Policy Violations**: Identify paths where untrusted data reaches resources without proper validation

## PAR Analysis Framework

- **Principal**: Untrusted data sources (user input, external APIs, environment variables)
- **Action**: Security controls (validation, sanitization, authentication, authorization)
- **Resource**: Sensitive operations (DB, file system, command execution, network)

## Output Format

{lang_instruction}

Respond with a JSON object containing:

```json
{{
  "scratchpad": "Your analysis reasoning and notes",
  "analysis": "Comprehensive security assessment",
  "poc": "Proof of concept code if vulnerability found",
  "confidence_score": 0-100,
  "vulnerability_types": ["LFI", "RCE", "SSRF", "AFO", "SQLI", "XSS", "IDOR"],
  "par_analysis": {{
    "principals": [...],
    "actions": [...],
    "resources": [...],
    "policy_violations": [...]
  }},
  "remediation_guidance": {{
    "policy_enforcement": [...]
  }}
}}
```

## Important Notes

- confidence_score: Set to 0 if no vulnerability is found
- Normalize confidence_score to multiples of 10 (0, 10, 20, ..., 100)
- Only report vulnerabilities with high confidence (>= 70)
"#,
            file_path = safe_file_path,
            pattern_section = pattern_section,
            lang_instruction = lang_instruction,
        )
    }
}

/// Context information about a security pattern match.
#[derive(Debug, Clone)]
pub struct PatternContext {
    /// Type of pattern (e.g., "command_injection", "sql_injection").
    pub pattern_type: String,
    /// Description of the pattern.
    pub description: String,
    /// The matched source code.
    pub matched_code: String,
    /// Known attack vectors for this pattern.
    pub attack_vectors: Vec<String>,
}

impl PatternContext {
    /// Create a new pattern context.
    pub fn new(pattern_type: &str, description: &str, matched_code: &str) -> Self {
        Self {
            pattern_type: pattern_type.to_string(),
            description: description.to_string(),
            matched_code: matched_code.to_string(),
            attack_vectors: Vec::new(),
        }
    }

    /// Add attack vectors.
    pub fn with_attack_vectors(mut self, vectors: Vec<String>) -> Self {
        self.attack_vectors = vectors;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_build_security_analysis_prompt() {
        let builder = PromptBuilder::new();
        let prompt = builder.build_security_analysis_prompt(
            &PathBuf::from("src/main.rs"),
            "fn main() { println!(\"Hello\"); }",
            None,
        );

        assert!(prompt.contains("src/main.rs"));
        assert!(prompt.contains("PAR"));
        assert!(prompt.contains("confidence_score"));
    }

    #[test]
    fn test_build_prompt_with_pattern() {
        let builder = PromptBuilder::new();
        let pattern = PatternContext::new(
            "sql_injection",
            "Direct SQL query construction",
            "query = f\"SELECT * FROM users WHERE id = {user_id}\"",
        )
        .with_attack_vectors(vec!["UNION injection".to_string()]);

        let prompt = builder.build_security_analysis_prompt(
            &PathBuf::from("src/db.py"),
            "query = f\"SELECT * FROM users WHERE id = {user_id}\"",
            Some(&pattern),
        );

        assert!(prompt.contains("sql_injection"));
        assert!(prompt.contains("UNION injection"));
    }

    #[test]
    fn test_build_prompt_with_poc() {
        let builder = PromptBuilder::new().with_poc(true);
        let prompt = builder.build_security_analysis_prompt(
            &PathBuf::from("test.py"),
            "os.system(user_input)",
            None,
        );

        assert!(prompt.contains("Execute the PoC"));
        assert!(prompt.contains("Safety Constraints"));
    }

    #[test]
    fn test_build_sarif_output_prompt() {
        let builder = PromptBuilder::new();
        let output_path = PathBuf::from("/tmp/output.sarif");
        let prompt = builder.build_sarif_output_prompt(
            &PathBuf::from("test.rs"),
            "fn main() { let x = 1; }",
            &output_path,
            None,
        );

        assert!(prompt.contains("SARIF"));
        assert!(prompt.contains("/tmp/output.sarif"));
        assert!(prompt.contains("test.rs"));
        assert!(prompt.contains("SQLI"));
        assert!(prompt.contains("XSS"));
    }

    #[test]
    fn test_build_sarif_output_prompt_with_pattern_context() {
        let builder = PromptBuilder::new();
        let output_path = PathBuf::from("/tmp/output.sarif");
        let pattern_context = PatternContext::new(
            "sql_injection",
            "Potential SQL injection",
            "query = f\"SELECT * FROM users WHERE id = {user_id}\"",
        );

        let prompt = builder.build_sarif_output_prompt(
            &PathBuf::from("app.py"),
            "query = f\"SELECT * FROM users WHERE id = {user_id}\"",
            &output_path,
            Some(&pattern_context),
        );

        assert!(prompt.contains("Pattern Context"));
        assert!(prompt.contains("sql_injection"));
        assert!(prompt.contains("Potential SQL injection"));
    }
}
