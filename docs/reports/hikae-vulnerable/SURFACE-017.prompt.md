You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-017
- **Kind**: endpoint
- **Identifier**: POST /api/template/render
- **Description**: Server-side template injection via unsandboxed Jinja2 Template() with user-controlled template string and context
- **Locations**: api.py

## Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-017.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
