You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-006
- **Kind**: endpoint
- **Identifier**: POST /allocations
- **Description**: Allocation threshold processing endpoint. Historically uses eval() on user input for server-side JavaScript injection (A1 Injection).
- **Locations**: app/routes/index.js, app/data/allocations-dao.js

## Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/NodeGoat/SURFACE-006.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
