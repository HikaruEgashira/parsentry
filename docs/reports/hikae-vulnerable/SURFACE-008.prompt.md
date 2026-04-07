You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

Surface Under Analysis

- ID: SURFACE-008
- Kind: endpoint
- Identifier: GET /api/user/<user_id>
- Description: IDOR vulnerability exposing any user's data without authorization check, plus SQL injection in user lookup query via unvalidated user_id
- Locations: api.py, models.py

Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: docs/reports/hikae-vulnerable/SURFACE-008.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
