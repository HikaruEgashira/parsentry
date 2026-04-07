You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-025
- **Kind**: endpoint
- **Identifier**: GET /api/passport
- **Description**: JWT passport issuance endpoint for web app authentication. Token generation security, JWT algorithm confusion, and token scope validation require review
- **Locations**: api/controllers/web/passport.py

## Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-025.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
