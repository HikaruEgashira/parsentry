You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-005
- **Kind**: endpoint
- **Identifier**: GET /lfi?file=
- **Description**: Local file inclusion via arbitrary file read with no path validation, enabling access to /etc/passwd and other sensitive system files
- **Locations**: app.py

## Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/hikae-vulnerable/SURFACE-005.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
