# Security Analysis Report: hikae-vulnerable-python

**Date:** 2026-04-07
**Tool:** Parsentry v0.17.0
**Target:** HikaruEgashira/hikae-vulnerable-python
**Surfaces Analyzed:** 25

## Executive Summary

| Severity | Count |
|----------|-------|
| Error (Critical/High) | 46 |
| Warning (Medium/Low) | 21 |
| **Total** | **67** |

### Top Vulnerability Categories

| Category | Count | Impact |
|----------|-------|--------|
| SQL Injection (SQLI) | 4 | Authentication bypass, data exfiltration |
| Remote Code Execution (RCE) | 8 | Full system compromise |
| Information Disclosure | 8 | Internal path/schema leakage |
| Server-Side Template Injection (SSTI) | 2 | RCE via template engine |
| Server-Side Request Forgery (SSRF) | 3 | Internal network access, cloud metadata theft |
| Insecure Object Reference (IDOR) | 3 | Unauthorized data access |
| Local File Inclusion (LFI) | 2 | Arbitrary file read |
| Path Traversal | 2 | Filesystem escape |
| Hardcoded Secrets/Credentials | 4 | Credential theft, session forgery |
| Sensitive Data Logging | 2 | Password exposure in logs |
| Insecure Deserialization | 1 | Arbitrary code execution |
| Cross-Site Scripting (XSS) | 1 | Client-side code execution |
| XML External Entity (XXE) | 1 | File read, SSRF |
| LDAP Injection | 1 | Auth bypass, data exfiltration |
| Open Redirect | 2 | Phishing attacks |
| Other (auth, config, upload, zip slip, DoS) | 13 | Various |

---

## Findings Detail

### 1. SQLI -- SQL Injection in Authentication Query
- **Rule ID:** SQLI
- **Severity:** error | **Confidence:** 1.0
- **Location:** `models.py:80`
- **Description:** User-supplied `username` and `password` are interpolated directly into the SQL statement via f-string formatting, enabling complete authentication bypass.
- **Remediation:** Use parameterized queries with `?` placeholders. Never use string interpolation for SQL.

### 2. SENSITIVE-LOG -- Plaintext Credentials in Application Log
- **Rule ID:** SENSITIVE-LOG
- **Severity:** error | **Confidence:** 1.0
- **Location:** `app.py:123`
- **Description:** `logger.info()` records plaintext password on every login attempt.
- **Remediation:** Never log credentials. If logging login events, log only the username and outcome.

### 3. SENSITIVE-LOG -- SQL Query with Password in Debug Log
- **Rule ID:** SENSITIVE-LOG
- **Severity:** warning | **Confidence:** 1.0
- **Location:** `models.py:81`
- **Description:** Authentication query containing plaintext password is logged at DEBUG level.
- **Remediation:** Remove credential data from log statements. Set production log level to INFO or higher.

### 4. WEAK-CRYPTO -- Predictable MD5 Session Tokens
- **Rule ID:** WEAK-CRYPTO
- **Severity:** error | **Confidence:** 1.0
- **Location:** `app.py:132`
- **Description:** Session token derived from `MD5(username+password)` is deterministic and forgeable.
- **Remediation:** Use `secrets.token_hex()` or framework-provided session management.

### 5. INFO-DISCLOSURE -- Username Enumeration via Login Error
- **Rule ID:** INFO-DISCLOSURE
- **Severity:** warning | **Confidence:** 1.0
- **Location:** `app.py:143`
- **Description:** Login failure message includes the username, confirming account existence to attackers.
- **Remediation:** Use a generic error message like "Invalid credentials."

### 6. HARDCODED-SECRET -- Hardcoded Flask Secret Key
- **Rule ID:** HARDCODED-SECRET
- **Severity:** error | **Confidence:** 1.0
- **Location:** `app.py:27`
- **Description:** Flask `secret_key` is hardcoded as `"vulnerable_secret_key_123"`, enabling session forgery.
- **Remediation:** Load the secret key from environment variables or a secrets manager.

### 7. SESSION-MISCONFIG -- Insecure Cookie Configuration
- **Rule ID:** SESSION-MISCONFIG
- **Severity:** warning | **Confidence:** 1.0
- **Location:** `app.py:28`
- **Description:** `SESSION_COOKIE_SECURE=False` and `SESSION_COOKIE_HTTPONLY=False`.
- **Remediation:** Set both flags to `True` in production.

### 8. SENSITIVE-EXPOSURE -- API Key in Session Cookie
- **Rule ID:** SENSITIVE-EXPOSURE
- **Severity:** error | **Confidence:** 0.95
- **Location:** `models.py:91`
- **Description:** `authenticate_user` returns `api_key` in the result, which is stored in the session cookie.
- **Remediation:** Do not include API keys in session data. Store server-side only.

### 9. HARDCODED-CREDS -- Default Credentials Seeded
- **Rule ID:** HARDCODED-CREDS
- **Severity:** error | **Confidence:** 1.0
- **Location:** `models.py:53`
- **Description:** Default credentials (admin/admin123, guest/guest) seeded on initialization.
- **Remediation:** Remove default credentials. Require first-run setup with strong passwords.

### 10. PLAINTEXT-PASSWORD -- No Password Hashing
- **Rule ID:** PLAINTEXT-PASSWORD
- **Severity:** error | **Confidence:** 1.0
- **Location:** `models.py:39`
- **Description:** Passwords stored and compared in plaintext.
- **Remediation:** Use bcrypt or argon2 for password hashing.

### 11. SQLI -- SQL Injection in LIKE Clause
- **Rule ID:** SQLI
- **Severity:** error | **Confidence:** 1.0
- **Location:** `app.py:191`
- **Description:** `username` parameter interpolated directly into `WHERE username LIKE '%{username}%'`.
- **Remediation:** Use parameterized queries.

### 12. SQLI -- SQL Injection in ORDER BY Clause
- **Rule ID:** SQLI
- **Severity:** error | **Confidence:** 1.0
- **Location:** `app.py:194`
- **Description:** `order` parameter interpolated into `ORDER BY {order_by}`.
- **Remediation:** Validate against an allowlist of column names.

### 13-14. INFO-DISCLOSURE -- SQL Query and Error Reflection
- **Rule ID:** INFO-DISCLOSURE
- **Severity:** warning | **Confidence:** 0.95
- **Locations:** `app.py:214`, `app.py:199`
- **Description:** Raw SQL queries and sqlite3 exception messages rendered in HTML responses.
- **Remediation:** Return generic error messages. Log details server-side only.

### 15. XSS -- Reflected Cross-Site Scripting (Multiple Contexts)
- **Rule ID:** XSS
- **Severity:** error | **Confidence:** 1.0
- **Location:** `app.py:286`
- **Description:** `name` and `comment` parameters interpolated via Python f-string into HTML template, bypassing Jinja2 autoescaping. Injection in HTML, attribute, JavaScript, and CSS contexts.
- **Remediation:** Use Jinja2 template variables (`{{ name }}`) instead of f-string interpolation.

### 16-18. RCE -- OS Command Injection via os.popen/os.system
- **Rule ID:** RCE
- **Severity:** error | **Confidence:** 0.97-0.99
- **Locations:** `app.py:207`, `app.py:210`, `app.py:214`
- **Description:** `hostname` and `count` parameters injected directly into shell commands via `os.popen()` and `os.system()`.
- **Remediation:** Use `subprocess.run()` with `shell=False` and pass arguments as a list. Validate `count` as integer. Allowlist hostnames.

### 19. PATH_INJECTION -- Temp File Path Traversal
- **Rule ID:** PATH_INJECTION
- **Severity:** warning | **Confidence:** 0.92
- **Location:** `app.py:213`
- **Description:** `hostname` embedded in temp file path enabling path traversal writes.
- **Remediation:** Sanitize filename components. Use `tempfile.NamedTemporaryFile()`.

### 20. LFI -- Local File Inclusion via Direct File Read
- **Rule ID:** LFI
- **Severity:** error | **Confidence:** 1.0
- **Location:** `app.py:248`
- **Description:** `file` parameter passed directly to `open()` with no path validation.
- **Remediation:** Validate against a base directory using `os.path.realpath()` and path prefix checks.

### 21. PATH_TRAVERSAL -- Directory Traversal in File Read
- **Rule ID:** PATH_TRAVERSAL
- **Severity:** error | **Confidence:** 1.0
- **Location:** `app.py:253`
- **Description:** No `os.path.realpath()` or base-directory check applied; `../../etc/passwd` works.
- **Remediation:** Canonicalize the path and verify it falls within the allowed directory.

### 22. INFO_DISCLOSURE -- Error Message in File Read
- **Rule ID:** INFO_DISCLOSURE
- **Severity:** warning | **Confidence:** 0.95
- **Location:** `app.py:255`
- **Description:** Failed `open()` exception rendered verbatim in HTTP response.
- **Remediation:** Return a generic "file not found" message.

### 23. SSTI -- Server-Side Template Injection (app.py)
- **Rule ID:** SSTI
- **Severity:** error | **Confidence:** 1.0
- **Location:** `app.py:241`
- **Description:** `template` parameter passed directly to `jinja2.Template()` constructor.
- **Remediation:** Use `jinja2.sandbox.SandboxedEnvironment()` or render pre-defined templates only.

### 24. RCE -- RCE via SSTI MRO Chain
- **Rule ID:** RCE
- **Severity:** error | **Confidence:** 0.98
- **Location:** `app.py:245`
- **Description:** Unsandboxed Jinja2 Template enables `__subclasses__()` traversal for arbitrary command execution.
- **Remediation:** Same as SSTI fix above.

### 25. HARDCODED_SECRET -- JWT Secret in Source Code
- **Rule ID:** HARDCODED_SECRET
- **Severity:** error | **Confidence:** 1.0
- **Location:** `api.py:22`
- **Description:** `JWT_SECRET = "super_secret_key_123"` hardcoded at module level.
- **Remediation:** Use environment variables and a strong random secret (256+ bits).

### 26-27. SENSITIVE_DATA_LOGGING -- Password in Audit Logs and Debug
- **Rule ID:** SENSITIVE_DATA_LOGGING
- **Severity:** error | **Confidence:** 1.0
- **Locations:** `api.py:50`, `models.py:77`
- **Description:** Plaintext passwords persisted to audit_logs and written to debug logs.
- **Remediation:** Never log credentials. Mask or omit sensitive fields.

### 28. API_KEY_EXPOSURE -- API Key in Login Response
- **Rule ID:** API_KEY_EXPOSURE
- **Severity:** error | **Confidence:** 1.0
- **Location:** `api.py:58`
- **Description:** User's API key returned in login response JSON.
- **Remediation:** Do not expose internal API keys to clients.

### 29. WEAK_JWT -- Weak JWT Configuration
- **Rule ID:** WEAK_JWT
- **Severity:** error | **Confidence:** 1.0
- **Location:** `api.py:43`
- **Description:** HS256 with weak 22-character hardcoded secret; role claims not re-validated server-side.
- **Remediation:** Use RS256 or a cryptographically strong HS256 key. Validate claims server-side.

### 30. INFO_DISCLOSURE -- Exception in Auth Response
- **Rule ID:** INFO_DISCLOSURE
- **Severity:** warning | **Confidence:** 0.95
- **Location:** `api.py:64`
- **Description:** `str(e)` returned in HTTP 500 response body.
- **Remediation:** Return generic error. Log exceptions server-side.

### 31. PLAINTEXT_PASSWORD_STORAGE -- No Password Hashing (API)
- **Rule ID:** PLAINTEXT_PASSWORD_STORAGE
- **Severity:** error | **Confidence:** 1.0
- **Location:** `models.py:27`
- **Description:** Passwords stored as plaintext TEXT in the users table.
- **Remediation:** Use bcrypt/argon2 for hashing.

### 32. IDOR -- Unauthorized User Data Access
- **Rule ID:** IDOR
- **Severity:** error | **Confidence:** 0.95
- **Location:** `api.py:73`
- **Description:** `GET /api/user/<user_id>` retrieves any user's data without authorization check.
- **Remediation:** Verify that the requesting user has permission to access the requested resource.

### 33. INFO-DISCLOSURE -- Exception in User Endpoint
- **Rule ID:** INFO-DISCLOSURE
- **Severity:** warning | **Confidence:** 0.9
- **Location:** `api.py:82`
- **Description:** Raw exception string returned to client.
- **Remediation:** Return generic error message.

### 34-36. SSRF -- Server-Side Request Forgery (3 variants)
- **Rule ID:** SSRF, SSRF-METADATA, SSRF-INTERNAL
- **Severity:** error | **Confidence:** 0.95-1.0
- **Location:** `api.py:112-117`
- **Description:** User-supplied URL passed to `requests.get()` with no validation. Enables access to cloud metadata and internal network.
- **Remediation:** Validate URL scheme (https only), implement allowlist/blocklist for hostnames and IP ranges.

### 37-41. RCE/AUTH -- Unauthenticated Command Execution
- **Rule ID:** RCE-001, RCE-002, RCE-003, INFO-DISCLOSURE-001, AUTH-001
- **Severity:** error | **Confidence:** 1.0
- **Location:** `api.py:163-183`
- **Description:** `POST /api/exec/command` executes arbitrary shell commands via `subprocess.run(command, shell=True)` with no authentication.
- **Remediation:** Remove this endpoint entirely. If required, add strong auth, allowlist commands, and use `shell=False`.

### 42. RCE -- Arbitrary Python eval()
- **Rule ID:** RCE
- **Severity:** error | **Confidence:** 1.0
- **Location:** `api.py:270`
- **Description:** `eval()` called on user-supplied `code` parameter at `POST /api/eval/python`.
- **Remediation:** Remove this endpoint. Never use `eval()` on user input.

### 43. INPUT-VALIDATION -- Unauthenticated Pickle Deserialization
- **Rule ID:** INPUT-VALIDATION
- **Severity:** warning | **Confidence:** 0.95
- **Location:** `api.py:149`
- **Description:** No authentication on `/api/pickle/deserialize`. Any client can trigger deserialization.
- **Remediation:** Remove endpoint or add authentication and use safe serialization (JSON).

### 44. XXE -- XML External Entity Injection
- **Rule ID:** XXE
- **Severity:** error | **Confidence:** 0.92
- **Location:** `api.py:134`
- **Description:** `ET.fromstring()` with default parser; no external entity restrictions.
- **Remediation:** Use `defusedxml` library or disable entity resolution explicitly.

### 45. DOS -- XML Bomb / Billion Laughs
- **Rule ID:** DOS
- **Severity:** warning | **Confidence:** 0.9
- **Location:** `api.py:130`
- **Description:** No request body size limit on XML input.
- **Remediation:** Add request size limits and use `defusedxml`.

### 46. PATH_TRAVERSAL -- File Upload Path Traversal
- **Rule ID:** PATH_TRAVERSAL
- **Severity:** error | **Confidence:** 0.98
- **Location:** `api.py:186`
- **Description:** `file.filename` used directly in `os.path.join` without sanitization.
- **Remediation:** Use `werkzeug.utils.secure_filename()` and validate the resolved path.

### 47. UNRESTRICTED_FILE_UPLOAD -- No File Type Validation
- **Rule ID:** UNRESTRICTED_FILE_UPLOAD
- **Severity:** error | **Confidence:** 0.97
- **Location:** `api.py:170`
- **Description:** No file type, extension, or magic-byte validation.
- **Remediation:** Validate file extensions against an allowlist and check MIME types.

### 48. ZIP_SLIP -- Zip Slip via extractall()
- **Rule ID:** ZIP_SLIP
- **Severity:** error | **Confidence:** 0.98
- **Location:** `api.py:224`
- **Description:** `zipfile.ZipFile.extractall()` without validating member paths.
- **Remediation:** Validate each entry name with `os.path.realpath()` before extraction.

### 49. UNRESTRICTED_UPLOAD -- No Upload Validation on Extract
- **Rule ID:** UNRESTRICTED_UPLOAD
- **Severity:** warning | **Confidence:** 0.85
- **Location:** `api.py:211`
- **Description:** No content-type or size validation on uploaded archive.
- **Remediation:** Validate file type and enforce size limits.

### 50. RESOURCE_LEAK -- Temp Directory Never Cleaned
- **Rule ID:** RESOURCE_LEAK
- **Severity:** warning | **Confidence:** 0.9
- **Location:** `api.py:216`
- **Description:** `tempfile.mkdtemp()` never cleaned up, enabling disk exhaustion.
- **Remediation:** Use `tempfile.TemporaryDirectory()` context manager.

### 51. SSTI -- Template Injection (API)
- **Rule ID:** SSTI
- **Severity:** error | **Confidence:** 1.0
- **Location:** `api.py:295`
- **Description:** User-supplied `template` field passed directly to `jinja2.Template()`.
- **Remediation:** Use `SandboxedEnvironment` or pre-defined templates only.

### 52. INJECTION -- Context Injection in Template Render
- **Rule ID:** INJECTION
- **Severity:** warning | **Confidence:** 0.95
- **Location:** `api.py:288`
- **Description:** User-controlled `context` dict unpacked with `**context` into `template.render()`.
- **Remediation:** Whitelist allowed context keys.

### 53. LDAPI -- LDAP Injection
- **Rule ID:** LDAPI
- **Severity:** error | **Confidence:** 0.98
- **Location:** `api.py:254`
- **Description:** Username interpolated directly into LDAP filter string.
- **Remediation:** Use `ldap.filter.escape_filter_chars()` or validate input against a strict pattern.

### 54. AUTH-BYPASS -- Unauthenticated LDAP Search
- **Rule ID:** AUTH-BYPASS
- **Severity:** warning | **Confidence:** 0.9
- **Location:** `api.py:247`
- **Description:** No authentication on `/api/ldap/search`.
- **Remediation:** Add authentication middleware.

### 55. OPEN_REDIRECT -- Unvalidated Redirect
- **Rule ID:** OPEN_REDIRECT
- **Severity:** error | **Confidence:** 0.98
- **Location:** `api.py:229`
- **Description:** `next` parameter passed directly to `redirect()` with no validation.
- **Remediation:** Parse URL and reject any value with a `scheme` or `netloc` component.

### 56. OPEN_REDIRECT_SCHEME_BYPASS -- Protocol-Relative Redirect
- **Rule ID:** OPEN_REDIRECT_SCHEME_BYPASS
- **Severity:** warning | **Confidence:** 0.85
- **Location:** `api.py:232`
- **Description:** Accepts arbitrary URI schemes including `//`, `data:`, etc.
- **Remediation:** Restrict redirects to relative paths only.

### 57. LFI -- Chained SQLI + LFI in Document Content
- **Rule ID:** LFI
- **Severity:** error | **Confidence:** 0.98
- **Location:** `models.py:151`
- **Description:** Attacker-controlled `file_path` from SQL injection result read via `open()` with no validation.
- **Remediation:** Validate file paths against an allowlist and use parameterized queries.

### 58. SQLI -- Missing Input Validation at Controller
- **Rule ID:** SQLI
- **Severity:** warning | **Confidence:** 0.95
- **Location:** `api.py:83`
- **Description:** Flask route uses `<doc_id>` (string) instead of `<int:doc_id>`.
- **Remediation:** Use `<int:doc_id>` in the route definition for defense-in-depth.

### 59. IDOR -- Unauthorized Audit Log Access
- **Rule ID:** IDOR
- **Severity:** error | **Confidence:** 0.95
- **Location:** `app.py:240`
- **Description:** `/logs` endpoint fetches any user's audit logs via `user_id` parameter without authorization.
- **Remediation:** Verify requesting user matches the requested `user_id`.

### 60. PLAINTEXT_PASSWORD -- Plaintext Password Storage (models)
- **Rule ID:** PLAINTEXT_PASSWORD
- **Severity:** error | **Confidence:** 1.0
- **Location:** `models.py:27`
- **Description:** Password column defined as TEXT with no hashing.
- **Remediation:** Use bcrypt or argon2.

### 61. HARDCODED_CREDENTIALS -- Default Admin/Guest Accounts
- **Rule ID:** HARDCODED_CREDENTIALS
- **Severity:** error | **Confidence:** 1.0
- **Location:** `models.py:64`
- **Description:** admin/admin123 and guest/guest seeded on every startup.
- **Remediation:** Remove hardcoded credentials. Require admin setup on first run.

### 62. PLAINTEXT_API_KEY -- Unencrypted API Key Storage
- **Rule ID:** PLAINTEXT_API_KEY
- **Severity:** error | **Confidence:** 1.0
- **Location:** `models.py:69`
- **Description:** API keys stored as plaintext and returned in auth responses.
- **Remediation:** Encrypt API keys at rest. Hash stored copies and only display once at creation.

### 63. INSECURE_DESERIALIZATION -- pickle.loads() on User Input
- **Rule ID:** INSECURE_DESERIALIZATION
- **Severity:** error | **Confidence:** 1.0
- **Location:** `models.py:143`
- **Description:** `pickle.loads()` called on user-supplied data enabling arbitrary code execution.
- **Remediation:** Use JSON for serialization. Never deserialize untrusted data with pickle.

### 64. IDOR -- Document Search Returns All Users' Data
- **Rule ID:** IDOR
- **Severity:** warning | **Confidence:** 1.0
- **Location:** `models.py:165`
- **Description:** `search_documents()` ignores `user_id`; no ownership filter in SQL query.
- **Remediation:** Add `WHERE owner_id = ?` clause with parameterized query.

### 65. INFO_DISCLOSURE -- Database Exception Re-raised
- **Rule ID:** INFO_DISCLOSURE
- **Severity:** warning | **Confidence:** 0.9
- **Location:** `models.py:127`
- **Description:** `get_user_by_id()` re-raises raw database exceptions to callers.
- **Remediation:** Catch and return generic error. Log details server-side.

### 66. SECURITY_MISCONFIGURATION -- Debug Logging in Production
- **Rule ID:** SECURITY_MISCONFIGURATION
- **Severity:** warning | **Confidence:** 0.85
- **Location:** `models.py:14`
- **Description:** Root logger set to DEBUG level, exposing sensitive data in log output.
- **Remediation:** Set production log level to WARNING or INFO.

### 67. INFO-DISCLOSURE -- Error Details in Audit Log API
- **Rule ID:** INFO-DISCLOSURE
- **Severity:** warning | **Confidence:** 0.8
- **Location:** `models.py:205`
- **Description:** Raw exception and sensitive log content exposed via error responses.
- **Remediation:** Return generic error messages.

---

## Remediation Priority

### Immediate (Critical Risk)
1. **Remove dangerous endpoints**: `/api/exec/command`, `/api/eval/python`, `/api/pickle/deserialize`
2. **Fix all SQL injection**: Replace f-string interpolation with parameterized queries everywhere
3. **Implement password hashing**: bcrypt or argon2 for all stored passwords
4. **Remove hardcoded secrets**: Move Flask secret key, JWT secret, API keys to environment variables
5. **Remove hardcoded credentials**: Delete default admin/guest accounts

### High Priority
6. **Fix SSTI**: Use `SandboxedEnvironment` or disallow user-controlled templates
7. **Fix SSRF**: Implement URL allowlist/blocklist with scheme and IP validation
8. **Fix LFI/Path Traversal**: Validate all file paths against base directory
9. **Fix XSS**: Use Jinja2 template variables instead of f-string interpolation
10. **Add authentication**: Protect all API endpoints with proper auth middleware

### Medium Priority
11. **Fix file upload**: Add type validation, use `secure_filename()`, enforce size limits
12. **Fix open redirect**: Validate redirect URLs, restrict to relative paths
13. **Fix LDAP injection**: Use proper escaping functions
14. **Fix XXE**: Use `defusedxml` library
15. **Fix information disclosure**: Return generic error messages everywhere

### Low Priority
16. **Fix session configuration**: Enable Secure and HttpOnly cookie flags
17. **Fix logging**: Set production log level, remove sensitive data from logs
18. **Fix resource leaks**: Use context managers for temp directories
