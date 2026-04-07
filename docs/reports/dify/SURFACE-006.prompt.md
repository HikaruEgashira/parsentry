You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-006
- **Kind**: endpoint
- **Identifier**: POST /api/chat-messages, POST /api/completion-messages, POST /api/workflows/run
- **Description**: Public web API endpoints for chat and workflow execution with JWT passport auth. User inputs flow into LLM prompts and workflow variable interpolation
- **Locations**: api/controllers/web/completion.py, api/controllers/web/workflow.py

## Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-006.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
