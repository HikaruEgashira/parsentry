You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

Surface Under Analysis

- ID: SURFACE-022
- Kind: endpoint
- Identifier: GET /logs?user_id=
- Description: IDOR in audit log viewer allowing any authenticated user to view other users' logs via user_id parameter, plus SQL injection in log retrieval
- Locations: app.py, models.py

Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: docs/reports/hikae-vulnerable/SURFACE-022.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
