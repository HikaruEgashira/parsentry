You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-002
- **Kind**: endpoint
- **Identifier**: POST /signup
- **Description**: User registration endpoint using bcrypt-nodejs (deprecated, unmaintained) for password hashing. Accepts untrusted input for user creation.
- **Locations**: app/routes/index.js, app/data/user-dao.js

## Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/NodeGoat/SURFACE-002.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
