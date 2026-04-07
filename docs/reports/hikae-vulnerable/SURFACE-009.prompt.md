You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

Surface Under Analysis

- ID: SURFACE-009
- Kind: endpoint
- Identifier: POST /api/ssrf/fetch
- Description: Server-side request forgery allowing arbitrary HTTP requests to internal services and cloud metadata endpoints with no URL validation
- Locations: api.py

Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: docs/reports/hikae-vulnerable/SURFACE-009.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
