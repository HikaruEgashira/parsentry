You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-003
- **Kind**: endpoint
- **Identifier**: POST /console/api/files/upload, POST /v1/files/upload, POST /api/files/upload
- **Description**: File upload endpoints across all API layers accepting multipart form data. Requires review for file type validation bypass, path traversal, and malicious file content
- **Locations**: api/controllers/console/files.py, api/controllers/service_api/app/file.py, api/controllers/web/files.py, api/factories/file_factory/validation.py

## Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-003.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
