You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

Surface Under Analysis

- ID: SURFACE-022
- Kind: endpoint
- Identifier: POST /console/api/apps/<app_id>/audio-to-text, POST /v1/audio-to-text
- Description: Audio file upload and transcription endpoints. Malicious audio files could exploit transcription service vulnerabilities or cause resource exhaustion
- Locations: api/controllers/console/app/audio.py, api/controllers/service_api/app/audio.py, api/controllers/web/audio.py

Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: docs/reports/dify/SURFACE-022.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
