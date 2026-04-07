You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-007
- **Kind**: public_api
- **Identifier**: Code Executor (Python3, JavaScript, Jinja2)
- **Description**: Executes user-provided Python, JavaScript, and Jinja2 code via AST transformation and sandboxing. Sandbox escape, AST transformation bypass, and Jinja2 SSTI are critical risks
- **Locations**: api/core/helper/code_executor/code_executor.py, api/core/helper/code_executor/python3/python3_code_provider.py, api/core/helper/code_executor/python3/python3_transformer.py, api/core/helper/code_executor/javascript/javascript_code_provider.py, api/core/helper/code_executor/jinja2/jinja2_formatter.py, api/core/helper/code_executor/template_transformer.py

## Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-007.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
