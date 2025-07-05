use std::collections::HashMap;

pub fn get_messages() -> HashMap<&'static str, &'static str> {
    let mut messages = HashMap::new();

    // Error messages
    messages.insert("error_clone_failed", "Failed to delete clone directory");
    messages.insert("cloning_repo", "Cloning GitHub repository");
    messages.insert("analysis_target", "Analysis target");
    messages.insert("context_collection_failed", "Failed to collect context");
    messages.insert("analyzing_file", "Analyzing file");
    messages.insert("analysis_completed", "Analysis completed");
    messages.insert("error_directory_creation", "Failed to create directory");
    messages.insert("error_no_write_permission", "No write permission");
    messages.insert("error_test_file_deletion", "Failed to delete test file");
    messages.insert(
        "error_no_file_creation_permission",
        "No file creation permission",
    );
    messages.insert(
        "error_output_dir_check",
        "❌ Failed to check output directory",
    );
    messages.insert("relevant_files_detected", "Detected relevant source files");
    messages.insert(
        "security_pattern_files_detected",
        "Detected security pattern matching files",
    );
    messages.insert("parse_add_failed", "Failed to add file to parser");
    messages.insert("analysis_failed", "Analysis failed");
    messages.insert(
        "markdown_report_output_failed",
        "Failed to output Markdown report",
    );
    messages.insert("markdown_report_output", "Output Markdown report");
    messages.insert(
        "summary_report_output_failed",
        "Failed to output summary report",
    );
    messages.insert("summary_report_output", "Output summary report");
    messages.insert(
        "summary_report_needs_output_dir",
        "Summary report output requires --output-dir option",
    );
    messages.insert(
        "sarif_report_output_failed",
        "Failed to output SARIF report",
    );
    messages.insert("sarif_report_output", "Output SARIF report");
    messages.insert("sarif_output_failed", "Failed to output SARIF");
    messages.insert(
        "github_repo_clone_failed",
        "Failed to clone GitHub repository",
    );
    messages.insert(
        "custom_pattern_generation_start",
        "Starting custom pattern generation mode",
    );
    messages.insert(
        "pattern_generation_completed",
        "Pattern generation completed",
    );

    messages
}

pub const SYS_PROMPT_TEMPLATE: &str = r#"
As a security researcher, analyze code vulnerabilities with special attention to:
- Input validation and sanitization
- Authentication and authorization
- Data handling and leakage
- Command injection possibilities
- Path traversal vulnerabilities
- Timing attacks and race conditions
- Other security-critical patterns
"#;

pub const INITIAL_ANALYSIS_PROMPT_TEMPLATE: &str = r#"
Analyze the given code (function definition or function call) based on the PAR (Principal-Action-Resource) model and determine which single category it belongs to:

**Principal (Untrusted Sources)**: Functions that provide attacker-controllable data
- Functions that retrieve user input (request.params, request.body, etc.)
- Functions that retrieve external data (API responses, file reads, etc.)

**Action (Security Processing)**: Functions that perform security processing
- Functions that perform validation, sanitization, authentication/authorization
- Functions that perform input validation, data transformation, permission checks, encryption, etc.

**Resource (Attack Targets)**: Functions that operate on attack target resources
- Functions that operate on file system, database, system commands, DOM, network

## Function Definition Classification Examples:

**Resource Example:**
```python
def get_user_data(user_id):
    query = f"SELECT * FROM users WHERE id = {user_id}"
    return execute_query(query)
```
→ This function ultimately operates on a database, so it's classified as "Resource"

**Action Example:**
```python
def validate_user_id(user_id):
    if not isinstance(user_id, int):
        raise ValueError("Invalid user ID")
    return user_id
```
→ This function performs input validation, so it's classified as "Action"

## Function Call Classification Examples:

**Principal Example:**
```javascript
request.params.get('user_id')
```
→ This call retrieves untrusted user input, so it's classified as "Principal"

**Action Example:**
```python
validate_email(user_email)
```
→ This call executes validation processing, so it's classified as "Action"

**Resource Example:**
```python
db.query("SELECT * FROM users")
```
→ This call operates on a database, so it's classified as "Resource"

```javascript
document.getElementById('output').innerHTML = content
```
→ This call operates on the DOM, so it's classified as "Resource"

**Important:** For function calls, classify based on the nature of the function being called, not the content of the arguments.
"#;

pub const ANALYSIS_APPROACH_TEMPLATE: &str = r#"
Analysis procedure based on PAR model:
1. **Principal Identification**: Identify dangerous data sources (untrusted inputs)
2. **Resource Identification**: Identify dangerous operations affecting confidentiality, integrity, and availability
3. **Action Evaluation**: Evaluate whether appropriate validation and defense measures are implemented in the path from Principal to Resource
4. **Policy Violation Detection**: Detect inappropriate direct access between Principal and Resource
5. **PAR Relationship Context Evaluation**: Judge whether PAR relationships are appropriate in the context of the entire code
"#;

pub const GUIDELINES_TEMPLATE: &str = r#"
PAR-based Security Policy Evaluation Guidelines:

## Analysis Procedure
1. **Principal Evaluation**: Identify untrusted data sources and evaluate their risk
2. **Resource Evaluation**: Evaluate the risk of operations affecting confidentiality, integrity, and availability
3. **Action Evaluation**: Evaluate the implementation of appropriate defensive measures between Principal and Resource
4. **Policy Violations**: Detect cases where dangerous Principals directly access Resources without appropriate Actions
5. **Context Consideration**: Judge the appropriateness of PAR relationships in the context of the entire code
6. **Declarative Judgment**: Evaluate with declarative policies such as "This Principal can perform this Action"
7. Please respond in English

Important:
- If no vulnerabilities exist, return confidence_score=0, vulnerability_types=[], and empty par_analysis.
- Action patterns (validation/processing) have bypass possibilities, and implementation defects are direct causes of vulnerabilities.
"#;

pub const EVALUATOR_PROMPT_TEMPLATE: &str = r#"You are a security expert evaluating vulnerability analysis reports.
This report aims to identify vulnerabilities in a Python web application known to contain SQL Injection (SQLI), Cross-Site Scripting (XSS), and Remote Code Execution (RCE) vulnerabilities.

Please evaluate the report from the following perspectives:
1. Correctly identified vulnerabilities (SQLI, XSS, RCE)
2. False positives (non-existent vulnerabilities being reported)
3. Quality of analysis (impact assessment, root cause explanation, mitigation proposals)
4. Quality of verification code (clear procedures, example requests, expected results)

Report to evaluate:
{report}
"#;

pub const RESPONSE_LANGUAGE_INSTRUCTION: &str = "Please respond in English";
