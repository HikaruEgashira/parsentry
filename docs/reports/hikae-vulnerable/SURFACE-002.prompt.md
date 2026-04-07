You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

Surface Under Analysis

- ID: SURFACE-002
- Kind: endpoint
- Identifier: GET /sqli?username=&order=
- Description: SQL injection page with two injection points: LIKE clause via username parameter and ORDER BY clause via order parameter, both using direct string interpolation
- Locations: app.py

Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: docs/reports/hikae-vulnerable/SURFACE-002.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
