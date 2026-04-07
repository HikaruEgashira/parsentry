You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-009
- **Kind**: endpoint
- **Identifier**: POST /mcp/server/<server_code>/mcp (JSON-RPC 2.0)
- **Description**: Model Context Protocol server accepting JSON-RPC requests without explicit authentication. MCP tool invocation can trigger arbitrary tool execution and data access
- **Locations**: api/controllers/mcp/mcp.py, api/core/mcp/server/streamable_http.py, api/core/mcp/client/streamable_client.py, api/core/mcp/auth/auth_flow.py, api/core/mcp/session/base_session.py

## Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-009.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
