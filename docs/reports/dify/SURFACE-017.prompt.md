You are a security auditor. Read the source files listed in Locations and analyze them for vulnerabilities.

## Surface Under Analysis

- **ID**: SURFACE-017
- **Kind**: file_handler
- **Identifier**: Multi-backend File Storage (S3, Azure, GCS, Aliyun, Tencent, Supabase)
- **Description**: File storage abstraction with multiple cloud backends. Misconfigured storage credentials, path traversal in storage keys, and insecure direct object reference to stored files
- **Locations**: api/extensions/storage/aws_s3_storage.py, api/extensions/storage/azure_blob_storage.py, api/extensions/storage/google_cloud_storage.py, api/extensions/storage/aliyun_oss_storage.py, api/extensions/storage/tencent_cos_storage.py, api/extensions/storage/supabase_storage.py, api/extensions/storage/opendal_storage.py

## Output Instructions

Read each file in Locations using the Read tool, then output valid SARIF v2.1.0 JSON.
For each finding, provide:
- rule_id: vulnerability type (e.g. SQLI, XSS, LFI, RCE, SSRF)
- level: error/warning/note
- confidence: 0.0-1.0


Write the SARIF JSON output to: /Users/hikae/ghq/github.com/HikaruEgashira/parsentry/docs/reports/dify/SURFACE-017.sarif.json
Write ONLY valid JSON. No markdown, no code fences, no explanation.
