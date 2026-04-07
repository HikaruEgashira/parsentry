You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-008
- **Kind**: endpoint
- **Identifier**: GET /benefits
- **Description**: Benefits lookup that makes outbound HTTP requests via needle 2.2.4. Vulnerable to SSRF if the target URL is user-controlled.
- **Locations**: app/routes/index.js

## Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/NodeGoat/SURFACE-008.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
