You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-002
- **Kind**: endpoint
- **Identifier**: POST /api/login, POST /api/email-code-login, POST /api/forgot-password/resets
- **Description**: Public web app authentication with password reset flow. Password reset token validation and email-based login require review for token predictability and rate limiting
- **Locations**: api/controllers/web/login.py, api/controllers/web/forgot_password.py

## Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-002.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
