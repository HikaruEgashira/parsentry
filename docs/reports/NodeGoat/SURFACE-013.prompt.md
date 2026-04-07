You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

Surface Under Analysis

- ID: SURFACE-013
- Kind: db_table
- Identifier: memos collection (MongoDB)
- Description: Stores user-created memos rendered as HTML via vulnerable marked library. Stored XSS payload persistence.
- Locations: app/data/memos-dao.js

Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: docs/reports/NodeGoat/SURFACE-013.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
