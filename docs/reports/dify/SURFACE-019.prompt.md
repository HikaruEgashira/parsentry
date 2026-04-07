You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-019
- **Kind**: endpoint
- **Identifier**: POST /console/api/workspaces/current/tool-provider/api/add
- **Description**: Custom API tool provider registration. Users can register arbitrary API endpoints as tools, which are then invoked during workflow execution - SSRF and data exfiltration via custom tool definitions
- **Locations**: api/controllers/console/workspace/tool_providers.py

## Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-019.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
