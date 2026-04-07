You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-014
- **Kind**: endpoint
- **Identifier**: POST /console/api/datasets/<id>/documents, POST /v1/datasets/<id>/documents
- **Description**: Document upload and website import endpoints for knowledge base. Website import enables SSRF; document upload flows into extraction pipeline with potential for malicious file processing
- **Locations**: api/controllers/console/datasets/datasets_document.py, api/controllers/service_api/dataset/document.py, api/controllers/console/datasets/website.py

## Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-014.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
