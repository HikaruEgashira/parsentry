# Dify Security Analysis Report

**Date:** 2026-04-07
**Target:** langgenius/dify (main branch)
**Tool:** Parsentry v0.17.0
**Surfaces Analyzed:** 25

---

## Executive Summary

| Severity | Count |
|----------|-------|
| Error (Critical/High) | 62 |
| Warning (Medium) | 76 |
| Note (Low/Info) | 21 |
| **Total** | **159** |

The analysis identified **159 security findings** across 25 attack surfaces. The most critical issues involve:

1. **Unauthenticated endpoints** exposing critical functionality (MCP, webhooks, human input forms)
2. **Remote Code Execution** via unsandboxed Jinja2 SSTI, code executor injection, and plugin backwards invocation
3. **SSRF vulnerabilities** across multiple layers (remote file upload, website crawl, tool provider registration, document extraction)
4. **Path traversal** in all 7 cloud storage backends
5. **Hardcoded secrets** in Docker Compose infrastructure configuration
6. **Plugin system** with disabled signature verification and unrestricted backwards invocation

---

## Findings by Category

### 1. Authentication & Authorization Bypass

| Rule ID | Level | Confidence | Location | Description |
|---------|-------|-----------|----------|-------------|
| AUTH-BYPASS | error | 0.95 | `api/controllers/mcp/mcp.py:40` | MCP endpoint has zero authentication. Knowledge of server_code is the sole access control |
| AUTH-BYPASS | error | 0.95 | `api/controllers/trigger/webhook.py:40` | Webhook endpoint accepts all HTTP methods without any authentication |
| AUTH-BYPASS | error | 0.92 | `api/controllers/web/human_input_form.py:95` | Human input form endpoint intentionally unauthenticated; no identity binding |
| AUTH-BYPASS | error | 0.85 | `api/controllers/inner_api/wraps.py:62` | Inner API uses static shared key with no network-layer restriction |
| AUTH-BYPASS-USER-CREATION | error | 0.90 | `api/controllers/mcp/mcp.py:164` | Unauthenticated EndUser creation via MCP InitializeRequest |
| AUTHZ-BYPASS | error | 0.90 | `api/controllers/service_api/app/workflow.py:275` | `set_stop_flag_no_user_check` allows any user to stop another's workflow |
| AUTHZ-BYPASS | error | 0.85 | `api/core/plugin/backwards_invocation/app.py:65` | Plugin can impersonate any user by supplying arbitrary user_id |
| AUTHZ-MISSING-ROLE-CHECK | error | 0.85 | `api/controllers/console/workspace/members.py:85` | Any workspace member can invite new members regardless of role |
| PRIVESC-ROLE-ASSIGNMENT | error | 0.80 | `api/controllers/console/workspace/members.py:91` | Normal user can invite with admin role; no role hierarchy check |
| WORKFLOW-HIJACK | error | 0.88 | `api/controllers/web/human_input_form.py:118` | Unauthenticated attacker can approve/reject workflow steps |
| CSRF | error | 0.95 | `api/controllers/console/auth/oauth.py:83` | OAuth state parameter repurposed as invite_token, defeating CSRF protection |
| BRUTE-FORCE | error | 0.90 | `api/controllers/console/auth/login.py:167` | No rate limiting on verification code attempts |

### 2. Remote Code Execution (RCE)

| Rule ID | Level | Confidence | Location | Description |
|---------|-------|-----------|----------|-------------|
| SSTI-001 | error | 0.95 | `api/core/helper/code_executor/jinja2/jinja2_formatter.py:8` | Unsandboxed `jinja2.Template` enables classic SSTI via introspection |
| RCE-003 | error | 0.95 | `api/core/helper/code_executor/code_executor.py:72` | `enable_network=True` hardcoded for all sandbox executions |
| RCE-002 | error | 0.90 | `api/core/helper/code_executor/template_transformer.py:72` | User code injected via string replacement with no sanitization |
| RCE-004 | error | 0.90 | `api/core/helper/code_executor/python3/python3_transformer.py:9` | Python3 runner embeds user code at `{{code}}` placeholder without AST validation |
| RCE-005 | error | 0.85 | `api/core/helper/code_executor/javascript/javascript_code_provider.py:1` | Node.js runner embeds user code without validation |
| RCE | error | 0.90 | `api/core/plugin/backwards_invocation/tool.py:16` | Unrestricted tool invocation with no allowlist |
| RCE | error | 0.85 | `api/core/mcp/server/streamable_http.py:109` | Unauthenticated MCP tool invocation triggers arbitrary app execution |
| DESERIALIZATION | error | 0.65 | `api/core/rag/embedding/cached_embedding.py:68` | `pickle.loads()` on embedding cache enables arbitrary code execution |

### 3. Server-Side Request Forgery (SSRF)

| Rule ID | Level | Confidence | Location | Description |
|---------|-------|-----------|----------|-------------|
| SSRF | error | 0.90 | `api/core/datasource/website_crawl/website_crawl_plugin.py:33` | User-supplied URLs forwarded to plugin daemon with zero validation |
| SSRF-FETCH | error | 0.90 | `api/services/tools/api_tools_manage_service.py:195` | Remote schema fetch via `httpx.get()` with no IP range filtering |
| SSRF | error | 0.85 | `api/controllers/console/datasets/website.py:38` | Website crawl accepts arbitrary URLs for knowledge base ingestion |
| SSRF | error | 0.85 | `api/core/plugin/impl/datasource.py:117` | Plugin datasource manager forwards URLs without SSRF mitigation |
| SSRF-SCHEME | error | 0.85 | `api/controllers/console/remote_files.py:18` | Console endpoint accepts bare `str` URL (no scheme validation) unlike web endpoint's `HttpUrl` |
| SSRF | error | 0.85 | `api/controllers/console/workspace/tool_providers.py:391` | OpenAPI schema server URLs stored without allowlist validation |
| SSRF-001 | error | 0.85 | `api/core/helper/code_executor/code_executor.py:72` | Network-enabled sandbox can reach internal services |
| SSRF | error | 0.80 | `api/core/rag/extractor/word_extractor.py:83` | DOCX external image references trigger SSRF via `ssrf_proxy.get()` |
| SSRF | error | 0.75 | `api/controllers/console/workspace/model_providers.py:178` | Credential validation forwards to external services with arbitrary URLs |
| SSRF | error | 0.75 | `api/core/rag/extractor/extract_processor.py:44` | URL fetched server-side with attacker-controlled Content-Type influencing parser selection |
| XXE | error | 0.70 | `api/core/rag/extractor/unstructured/unstructured_xml_extractor.py:1` | XML parsed without disabling external entity resolution |

### 4. Path Traversal & File System Access

| Rule ID | Level | Confidence | Location | Description |
|---------|-------|-----------|----------|-------------|
| PATH-TRAVERSAL | error | 0.95 | `api/extensions/storage/aliyun_oss_storage.py:53` | `posixpath.join` does not prevent traversal with absolute paths |
| PATH-TRAVERSAL | error | 0.95 | `api/extensions/storage/opendal_storage.py:42` | Local filesystem scheme enables arbitrary file read/write |
| PATH-TRAVERSAL | error | 0.90 | `api/extensions/storage/aws_s3_storage.py:43` | Filename passed directly to S3 Key without sanitization |
| PATH-TRAVERSAL | error | 0.90 | `api/extensions/storage/azure_blob_storage.py:29` | Filename passed directly to Azure blob operations |
| PATH-TRAVERSAL | error | 0.90 | `api/extensions/storage/google_cloud_storage.py:33` | Filename used directly as GCS blob name |
| PATH-TRAVERSAL | error | 0.90 | `api/extensions/storage/tencent_cos_storage.py:30` | Filename passed directly as COS Key |
| PATH-TRAVERSAL | error | 0.90 | `api/extensions/storage/supabase_storage.py:26` | Filename passed directly to Supabase operations |
| ARBITRARY-FILE-WRITE | error | 0.85 | `api/extensions/storage/aws_s3_storage.py:68` | `download()` writes to caller-supplied `target_filepath` without validation |

### 5. Prompt Injection

| Rule ID | Level | Confidence | Location | Description |
|---------|-------|-----------|----------|-------------|
| PROMPT-INJECTION | error | 0.85 | `api/controllers/service_api/app/completion.py:104` | Unsanitized `inputs`/`query` passed to LLM generation |
| PROMPT-INJECTION | error | 0.85 | `api/controllers/web/completion.py:90` | Query field flows directly into LLM prompt construction |
| WORKFLOW-INJECTION | error | 0.75 | `api/controllers/web/workflow.py:37` | Workflow inputs interpolated into nodes without type validation |
| PROMPT-INJECTION | error | 0.80 | `api/controllers/service_api/app/workflow.py:166` | Workflow inputs interpolated into prompt templates |

### 6. Plugin System Vulnerabilities

| Rule ID | Level | Confidence | Location | Description |
|---------|-------|-----------|----------|-------------|
| PRIV-ESCALATION | error | 0.95 | `api/core/plugin/backwards_invocation/app.py:56` | Plugin invokes any tenant app with SERVICE_API privilege |
| PLUGIN-SIGNATURE-BYPASS | error | 0.90 | `api/core/plugin/impl/plugin.py:79` | `verify_signature=False` default allows unsigned plugins |
| CRYPTO-ORACLE | error | 0.85 | `api/core/plugin/backwards_invocation/encrypt.py:8` | Decrypt operation exposed to plugins, leaking provider API keys |
| HTTP-SMUGGLING | error | 0.85 | `api/core/plugin/utils/http_parser.py:25` | Raw HTTP deserialization lacks CRLF injection validation |
| IDOR | error | 0.90 | `api/controllers/inner_api/plugin/wraps.py:58` | Caller-supplied tenant_id/user_id trusted without ownership verification |
| EMAIL-ABUSE | error | 0.90 | `api/controllers/inner_api/mail.py:28` | Open email relay with no rate limiting or recipient validation |

### 7. Infrastructure & Configuration

| Rule ID | Level | Confidence | Location | Description |
|---------|-------|-----------|----------|-------------|
| HARDCODED-SECRET | error | 0.95 | `docker/docker-compose.middleware.yaml:12` | Default PostgreSQL password `difyai123456` |
| HARDCODED-SECRET | error | 0.95 | `docker/docker-compose.yaml:30` | Default Flask SECRET_KEY enables session forgery |
| INSECURE-DEFAULT | error | 0.90 | `docker/docker-compose.yaml:111` | CORS allows all origins (`*`) by default |
| NETWORK-EXPOSURE | error | 0.90 | `docker/docker-compose.middleware.yaml:26` | PostgreSQL 5432 exposed to host with default credentials |
| HARDCODED-SECRET | error | 0.95 | `docker/docker-compose.yaml` | Redis password `difyai123456`, Sandbox API key `dify-sandbox` |

### 8. JWT & Token Security

| Rule ID | Level | Confidence | Location | Description |
|---------|-------|-----------|----------|-------------|
| JWT-NO-EXPIRY | error | 0.95 | `api/controllers/web/passport.py:75` | Public-flow JWT passports issued without `exp` claim |
| IDOR | error | 0.90 | `api/controllers/web/passport.py:56` | Session ID lookup without ownership verification |
| SESSION-FIXATION | error | 0.85 | `api/controllers/web/passport.py:39` | `user_id` query parameter used directly as session_id |
| INSECURE-TOKEN | error | 0.95 | `api/controllers/console/auth/email_register.py:126` | Token returned in response body instead of HTTP-only cookies |

### 9. Information Disclosure

| Rule ID | Level | Confidence | Location | Description |
|---------|-------|-----------|----------|-------------|
| INFO-LEAK | warning | 0.90 | `api/controllers/trigger/webhook.py:58` | Raw exception message returned to unauthenticated callers |
| INFO-DISCLOSURE | warning | 0.90 | `api/core/mcp/server/streamable_http.py:72` | `str(e)` in JSON-RPC error responses leaks internals |
| INFO-LEAK-001 | warning | 0.85 | `api/core/workflow/nodes/trigger_webhook/node.py:178` | Raw webhook data (Authorization headers, cookies) exposed to all downstream nodes |
| ACCOUNT-ENUM | warning | 0.80 | `api/controllers/console/auth/oauth.py:99` | Distinct error messages enable account state enumeration |
| AUTH-ENUM | warning | 0.90 | `api/controllers/web/forgot_password.py:80` | Differential responses enable email enumeration |

### 10. Input Validation & File Upload

| Rule ID | Level | Confidence | Location | Description |
|---------|-------|-----------|----------|-------------|
| TOOLFILE-BYPASS | error | 0.70 | `api/factories/file_factory/validation.py:14` | TOOL_FILE transfer method bypasses all validation |
| MALICIOUS-FILE-UPLOAD | error | 0.70 | `api/controllers/service_api/dataset/document.py:370` | Client-controlled filename/mimetype without sanitization |
| MIME-TRUST | warning | 0.85 | `api/controllers/console/files.py:73` | Client-supplied MIME type used without server-side detection |
| BLACKLIST-BYPASS | warning | 0.80 | `api/services/file_service.py:67` | Extension blacklist instead of allowlist |
| DOS-001 | warning | 0.90 | `api/services/audio_service.py:54` | Full file read into memory before size check |

### 11. Celery & Background Task Security

| Rule ID | Level | Confidence | Location | Description |
|---------|-------|-----------|----------|-------------|
| PRIVESC-001 | error | 0.60 | `api/tasks/rag_pipeline/rag_pipeline_run_task.py:96` | Deserialized user_id/tenant_id used for impersonation |
| DESER-002 | error | 0.65 | `api/tasks/rag_pipeline/rag_pipeline_run_task.py:49` | File content fetched by controllable file_id deserialized into pipeline entities |
| QUEUE-POISON-001 | warning | 0.55 | `api/tasks/document_indexing_task.py:141` | Redis queue contents consumed without integrity checks |

---

## Remediation Recommendations

### Critical Priority (Immediate Action Required)

1. **Add authentication to MCP, webhook, and human input form endpoints.** Implement HMAC signature verification or token-based auth for webhooks. Add JWT validation for MCP endpoints. Bind form submissions to authenticated sessions.

2. **Use `jinja2.SandboxedEnvironment`** instead of `jinja2.Template` in the code executor to prevent SSTI attacks.

3. **Disable network access by default** in the code executor sandbox (`enable_network=False`). Only enable it when explicitly required by the workflow configuration.

4. **Enable plugin signature verification by default.** Change `verify_signature` to `True` and enforce it for all plugin installations.

5. **Implement path sanitization in all storage backends.** Validate filenames against `../`, absolute paths, and null bytes. Enforce prefix-bounding to prevent traversal.

6. **Change all default credentials** in Docker Compose configurations. Use environment variables with no fallback defaults, and fail-closed when secrets are not configured.

7. **Set JWT expiration** on all passport tokens. Implement token rotation and revocation mechanisms.

### High Priority

8. **Implement SSRF protection at the application layer.** Add URL validation (scheme allowlist, private IP blocklist, DNS rebinding protection) before all server-side URL fetches, not just at the proxy layer.

9. **Add authorization checks to plugin backwards invocation.** Implement per-app ACLs and prevent plugins from impersonating arbitrary users or accessing other tenants.

10. **Restrict the inner API** to specific network ranges via application-level IP allowlisting or mTLS, in addition to the shared key.

11. **Add role hierarchy checks** for member invitation to prevent privilege escalation.

12. **Implement input sanitization** for prompt injection defense. Add content filtering and variable interpolation controls at the controller layer.

### Medium Priority

13. **Switch file extension validation** from blacklist to allowlist approach.

14. **Add server-side MIME type detection** using magic bytes instead of trusting client-supplied Content-Type.

15. **Implement rate limiting** on all authentication endpoints, webhook triggers, and unauthenticated API endpoints.

16. **Use constant-time comparison** (`hmac.compare_digest`) for all security-sensitive string comparisons (verification codes, tokens).

17. **Replace `pickle` serialization** in CacheEmbedding with JSON or MessagePack to eliminate deserialization attacks.

18. **Restrict CORS origins** to specific trusted domains instead of wildcard `*`.

19. **Sanitize error messages** across all endpoints. Return generic error messages to clients and log detailed errors server-side only.

### Low Priority

20. **Add nonce tracking** to plugin file upload signature verification to prevent replay attacks.

21. **Implement Content-Length pre-checks** in audio upload endpoints to prevent memory exhaustion.

22. **Escape LIKE wildcards** in search query parameters to prevent pattern manipulation.

23. **Remove debug webhook endpoint** from production deployments or add authentication.
