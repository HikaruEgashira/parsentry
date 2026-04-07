You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

Surface Under Analysis

- ID: SURFACE-024
- Kind: iac_resource
- Identifier: Docker Compose with SSRF Proxy, Redis, PostgreSQL, Vector DB
- Description: Infrastructure configuration including SSRF proxy (Squid), Redis for caching/queuing, PostgreSQL, and vector databases. Misconfigured SSRF proxy rules, exposed Redis, and insecure default credentials
- Locations: docker/docker-compose.yaml, docker/docker-compose.middleware.yaml, docker/ssrf_proxy/, docker/nginx/conf.d/

Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: docs/reports/dify/SURFACE-024.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
