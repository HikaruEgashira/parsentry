You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-004
- **Kind**: endpoint
- **Identifier**: POST /console/api/remote-files/upload, POST /api/remote-files/upload
- **Description**: Remote file upload endpoints that fetch files from user-supplied URLs. High SSRF risk - can be used to probe internal network, access cloud metadata endpoints, or exfiltrate data
- **Locations**: api/controllers/console/remote_files.py, api/controllers/web/remote_files.py, api/factories/file_factory/remote.py

## Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-004.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
