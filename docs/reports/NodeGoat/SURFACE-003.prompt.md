You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

Surface Under Analysis

- ID: SURFACE-003
- Kind: endpoint
- Identifier: GET /profile
- Description: Profile retrieval endpoint vulnerable to Insecure Direct Object Reference (IDOR) — users may access other users' profiles by manipulating the userId parameter.
- Locations: app/routes/index.js, app/data/profile-dao.js

Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: docs/reports/NodeGoat/SURFACE-003.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
