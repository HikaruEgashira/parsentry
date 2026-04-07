You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-001
- **Kind**: endpoint
- **Identifier**: POST /console/api/login, POST /console/api/email-code-login, POST /console/api/oauth/login/<provider>
- **Description**: Console authentication endpoints handling login, OAuth, and registration. Vulnerable to brute force, credential stuffing, and OAuth redirect manipulation
- **Locations**: api/controllers/console/auth/login.py, api/controllers/console/auth/oauth.py, api/controllers/console/auth/email_register.py

## Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-001.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
