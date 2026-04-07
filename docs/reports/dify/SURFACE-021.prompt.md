You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-021
- **Kind**: public_api
- **Identifier**: Celery Task Workers (document indexing, app generation, account deletion)
- **Description**: Asynchronous Celery workers processing user data including document indexing and app generation. Task deserialization, Redis queue poisoning, and privilege escalation in background processing
- **Locations**: api/celery_entrypoint.py, api/tasks/document_indexing_task.py, api/tasks/app_generate/, api/tasks/delete_account_task.py, api/tasks/rag_pipeline/rag_pipeline_run_task.py

## Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-021.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
