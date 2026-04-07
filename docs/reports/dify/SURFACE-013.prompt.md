You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

Surface Under Analysis

- ID: SURFACE-013
- Kind: public_api
- Identifier: RAG Pipeline (embedding, retrieval, document indexing)
- Description: RAG pipeline processing user-uploaded documents through extraction, embedding, and retrieval. Document extraction can trigger SSRF, XXE, or code execution via malicious file formats (PDF, DOCX, etc.)
- Locations: api/core/rag/pipeline/, api/core/rag/retrieval/, api/core/rag/embedding/, api/core/rag/datasource/, api/core/rag/extractor/

Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: docs/reports/dify/SURFACE-013.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
