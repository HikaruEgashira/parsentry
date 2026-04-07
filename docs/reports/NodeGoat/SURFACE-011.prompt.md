You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

Surface Under Analysis

- ID: SURFACE-011
- Kind: db_table
- Identifier: users collection (MongoDB)
- Description: MongoDB users collection storing credentials and PII. NoSQL injection risk if query construction uses unsanitized input. Password hashing uses deprecated bcrypt-nodejs.
- Locations: app/data/user-dao.js, app/data/profile-dao.js, artifacts/db-reset.js

Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: docs/reports/NodeGoat/SURFACE-011.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
