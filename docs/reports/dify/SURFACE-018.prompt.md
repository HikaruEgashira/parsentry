You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-018
- **Kind**: endpoint
- **Identifier**: POST /console/api/workspaces/current/model-providers/<provider>/credentials/validate
- **Description**: Model provider credential validation endpoint. Accepts provider credentials and validates them against external services - potential for credential theft or SSRF via malicious provider configuration
- **Locations**: api/controllers/console/workspace/model_providers.py

## Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-018.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
