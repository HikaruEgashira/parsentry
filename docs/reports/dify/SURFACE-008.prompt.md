You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-008
- **Kind**: public_api
- **Identifier**: Plugin System (install, execute, backwards invocation)
- **Description**: Plugin system allowing third-party code installation and execution with backwards invocation into host app. Plugin supply chain attacks, privilege escalation via backwards invocation, and HTTP parsing vulnerabilities
- **Locations**: api/core/plugin/impl/plugin.py, api/core/plugin/impl/tool.py, api/core/plugin/impl/endpoint.py, api/core/plugin/backwards_invocation/app.py, api/core/plugin/backwards_invocation/tool.py, api/core/plugin/backwards_invocation/encrypt.py, api/core/plugin/utils/http_parser.py

## Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-008.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
