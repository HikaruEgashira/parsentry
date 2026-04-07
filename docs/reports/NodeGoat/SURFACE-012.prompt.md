You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-012
- **Kind**: db_table
- **Identifier**: allocations collection (MongoDB)
- **Description**: MongoDB collection for allocation data. Server-side JS injection vector if allocation processing uses eval() or $where operators.
- **Locations**: app/data/allocations-dao.js, artifacts/db-reset.js

## Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/NodeGoat/SURFACE-012.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
