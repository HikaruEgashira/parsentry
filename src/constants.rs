// System prompt template for LLM analysis
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

// Initial analysis prompt template
pub const INITIAL_ANALYSIS_PROMPT_TEMPLATE: &str = r#"
Analyze the given code based on the PAR (Principal-Action-Resource) model and output the results in the following JSON format:

**Principal (Who/Data Source)**: Entities or input sources that are the origin of data
- User input, API responses, file reads, environment variables, etc.
- Evaluate the trust level of each Principal (trusted/semi_trusted/untrusted)

**Action (What/Validation-Processing)**: Data processing or validation operations (including vulnerability possibilities)
- Input validation, sanitization, authentication/authorization, encryption, etc. (pay attention to bypass possibilities)
- Evaluate implementation quality (adequate/insufficient/missing/bypassed)

**Resource (Where/Dangerous Operations)**: Operations that affect confidentiality, integrity, and availability
- File writes, command execution, database updates, output, etc.
- Evaluate confidentiality level (low/medium/high/critical)

Evaluate what PAR role each code element has, whether appropriate security policies are implemented, and report as policy_violations.
"#;

// Analysis approach template
pub const ANALYSIS_APPROACH_TEMPLATE: &str = r#"
Analysis procedure based on PAR model:
1. **Principal Identification**: Identify dangerous data sources (untrusted inputs)
2. **Resource Identification**: Identify dangerous operations affecting confidentiality, integrity, and availability
3. **Action Evaluation**: Evaluate whether appropriate validation and defense measures are implemented in the path from Principal to Resource
4. **Policy Violation Detection**: Detect inappropriate direct access between Principal and Resource
5. **PAR Relationship Context Evaluation**: Judge whether PAR relationships are appropriate in the context of the entire code
"#;

// Guidelines template
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

// Evaluator prompt template
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

// Response language instruction
pub const RESPONSE_LANGUAGE_INSTRUCTION: &str = "Please respond in English";