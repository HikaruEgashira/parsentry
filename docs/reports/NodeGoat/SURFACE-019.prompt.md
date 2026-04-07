You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

Surface Under Analysis

- ID: SURFACE-019
- Kind: file_handler
- Identifier: Swig template rendering
- Description: Swig 1.4.2 (unmaintained) template engine rendering user data. Autoescape configuration determines XSS exposure across all views.
- Locations: app/views/, server.js

Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: docs/reports/NodeGoat/SURFACE-019.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
