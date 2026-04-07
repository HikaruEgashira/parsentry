You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

Surface Under Analysis

- ID: SURFACE-012
- Kind: endpoint
- Identifier: POST /api/pickle/deserialize
- Description: Insecure deserialization via pickle.loads() on base64-decoded user input, enabling arbitrary code execution through crafted pickle payloads
- Locations: api.py

Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: docs/reports/hikae-vulnerable/SURFACE-012.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
