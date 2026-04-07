You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

Surface Under Analysis

- ID: SURFACE-001
- Kind: endpoint
- Identifier: POST /login
- Description: Login endpoint handling credentials against MongoDB. Susceptible to NoSQL injection if user input is passed directly into MongoDB query operators without sanitization.
- Locations: app/routes/index.js, app/data/user-dao.js

Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: docs/reports/NodeGoat/SURFACE-001.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
