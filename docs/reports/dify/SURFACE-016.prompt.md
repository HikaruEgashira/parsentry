You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

Surface Under Analysis

- ID: SURFACE-016
- Kind: external_call
- Identifier: Website Crawl Provider (HTTP fetching of user-supplied URLs)
- Description: Fetches content from user-supplied URLs for knowledge base ingestion. Primary SSRF vector - must validate against internal network access, cloud metadata endpoints, and URL scheme restrictions
- Locations: api/core/datasource/website_crawl/website_crawl_provider.py, api/core/datasource/website_crawl/website_crawl_plugin.py

Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: docs/reports/dify/SURFACE-016.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
