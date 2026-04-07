You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

Surface Under Analysis

- ID: SURFACE-004
- Kind: endpoint
- Identifier: GET /cmdi?hostname=&count=
- Description: OS command injection via os.popen() and os.system() with unsanitized hostname and count parameters, allowing arbitrary command execution
- Locations: app.py

Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: docs/reports/hikae-vulnerable/SURFACE-004.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
