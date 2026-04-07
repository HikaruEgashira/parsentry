You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

Surface Under Analysis

- ID: SURFACE-010
- Kind: public_api
- Identifier: Workflow Execution Engine (node execution, variable interpolation)
- Description: Workflow engine executing directed acyclic graphs of nodes with user-controlled variable interpolation. Variable injection, node execution order manipulation, and data leakage between workflow contexts
- Locations: api/core/workflow/nodes/agent/agent_node.py, api/core/workflow/nodes/knowledge_retrieval/knowledge_retrieval_node.py, api/core/workflow/nodes/datasource/datasource_node.py, api/core/workflow/nodes/trigger_webhook/node.py

Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: docs/reports/dify/SURFACE-010.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
