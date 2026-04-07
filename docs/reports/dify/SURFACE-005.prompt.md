You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

Surface Under Analysis

- ID: SURFACE-005
- Kind: endpoint
- Identifier: POST /v1/completion-messages, POST /v1/chat-messages, POST /v1/workflows/run
- Description: Service API endpoints for LLM completion, chat, and workflow execution. Accept user-controlled inputs that flow into prompt templates and tool invocations - prompt injection and variable interpolation risks
- Locations: api/controllers/service_api/app/completion.py, api/controllers/service_api/app/workflow.py

Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: docs/reports/dify/SURFACE-005.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
