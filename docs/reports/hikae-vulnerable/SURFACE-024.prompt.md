You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-024
- **Kind**: db_table
- **Identifier**: audit_logs
- **Description**: Audit log table with SQL injection in INSERT via string formatting, logging sensitive data including passwords in cleartext
- **Locations**: models.py

## Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-024.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
