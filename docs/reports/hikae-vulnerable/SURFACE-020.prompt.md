You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

Surface Under Analysis

- ID: SURFACE-020
- Kind: endpoint
- Identifier: GET /api/documents/search?q=&user_id=
- Description: SQL injection in document search LIKE clause plus IDOR allowing access to any user's documents without authorization
- Locations: api.py, models.py

Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: docs/reports/hikae-vulnerable/SURFACE-020.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
