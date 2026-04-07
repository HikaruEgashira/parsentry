You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

Surface Under Analysis

- ID: SURFACE-014
- Kind: external_call
- Identifier: needle HTTP client requests
- Description: Outbound HTTP requests using needle 2.2.4. SSRF risk if request target is user-controllable. needle 2.2.4 has known prototype pollution vulnerabilities.
- Locations: app/routes/index.js

Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: docs/reports/NodeGoat/SURFACE-014.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
