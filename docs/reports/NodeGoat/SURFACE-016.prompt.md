You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-016
- **Kind**: file_handler
- **Identifier**: config/env/ environment configuration
- **Description**: Environment-specific configuration files containing database connection strings and secrets. Risk of sensitive data exposure if defaults are insecure.
- **Locations**: config/config.js, config/env/

## Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/NodeGoat/SURFACE-016.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
