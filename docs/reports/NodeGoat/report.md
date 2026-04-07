# NodeGoat Security Analysis Report

**Date:** 2026-04-07
**Tool:** Parsentry v0.17.0
**Target:** OWASP NodeGoat
**Surfaces Analyzed:** 19

---

## Executive Summary

| Severity | Count |
|----------|-------|
| Error (Critical/High) | 37 |
| Warning (Medium) | 27 |
| Note (Low) | 1 |
| **Total** | **65** |

The OWASP NodeGoat application contains **65 security findings** across 19 attack surfaces. The most prevalent vulnerability class is **Cross-Site Scripting (XSS)** with 12 instances, enabled by a globally disabled autoescape in the Swig template engine. Critical findings include **Remote Code Execution via eval()**, **NoSQL Injection**, **Server-Side Request Forgery (SSRF)**, and **plaintext password storage**.

### Top Vulnerability Categories

| Category | Count | Max Severity |
|----------|-------|-------------|
| XSS (Cross-Site Scripting) | 12 | error |
| NoSQL Injection | 6 | error |
| IDOR (Insecure Direct Object Reference) | 3 | error |
| Hardcoded Secrets/Credentials | 5 | error |
| Session Misconfiguration | 6 | error |
| Authentication Failures | 3 | error |
| Transport Security | 2 | error |
| SSRF | 1 | error |
| RCE | 1 | error |

---

## Findings Detail

### 1. Remote Code Execution (RCE)

#### RCE-001: Server-Side JavaScript Injection via eval()
- **Rule ID:** RCE
- **Severity:** error | **Confidence:** 1.0
- **Location:** `app/routes/contributions.js:32`
- **Description:** `req.body.preTax`, `req.body.afterTax`, and `req.body.roth` are passed directly to `eval()`. An attacker can submit arbitrary JavaScript (e.g., `require("child_process").exec("rm -rf /")`) to achieve full server compromise.
- **Remediation:** Replace `eval()` with `parseInt()` or a safe numeric parser. The fix exists in comments but is inactive.

---

### 2. NoSQL Injection

#### NOSQLI-001: Authentication Bypass via userName Injection
- **Rule ID:** NOSQLI
- **Severity:** error | **Confidence:** 0.95
- **Location:** `app/data/user-dao.js:91-93`
- **Description:** `userName` is passed unsanitized into MongoDB `findOne()`. Attacker can submit `{"$gt":""}` to match any user document and bypass authentication.
- **Remediation:** Validate that `userName` is a string type before query. Use `mongo-sanitize` or explicit type checking.

#### NOSQLI-002: $where Clause Injection in Allocations
- **Rule ID:** NOSQLI
- **Severity:** error | **Confidence:** 0.99
- **Location:** `app/data/allocations-dao.js:78-79`
- **Description:** The `threshold` parameter is string-interpolated into a `$where` JavaScript expression. Payloads like `0';while(true){}'` cause DoS; `1'; return 1 == '1` exfiltrates data.
- **Remediation:** Use `parseInt()` with range validation (fix exists in comments). Avoid `$where` entirely; use standard query operators.

#### NOSQLI-003: Contributions DAO Type Confusion
- **Rule ID:** NOSQLI
- **Severity:** warning | **Confidence:** 0.70
- **Location:** `app/data/contributions-dao.js:21-26`
- **Description:** User-controlled values embedded into MongoDB documents without type enforcement. Crafted payloads could manipulate document structure.
- **Remediation:** Enforce explicit type casting (e.g., `parseInt()`) at the DAO layer before document construction.

#### NOSQLI-004: userName in getUserByUserName
- **Rule ID:** NOSQL-INJECTION
- **Severity:** error | **Confidence:** 0.90
- **Location:** `app/data/user-dao.js:91-93`
- **Description:** Same unsanitized userName flows into `findOne` via `getUserByUserName`, enabling operator injection.
- **Remediation:** Same as NOSQLI-001.

#### NOSQLI-005: Memo Document Injection
- **Rule ID:** NOSQL-INJECTION
- **Severity:** warning | **Confidence:** 0.60
- **Location:** `app/data/memos-dao.js:23`
- **Description:** With `bodyParser.json()` enabled, attacker could send an object instead of string for memo field via Content-Type manipulation.
- **Remediation:** Validate memo input is a string before insertion.

#### NOSQLI-006: Memo DAO Type Confusion
- **Rule ID:** NOSQLI
- **Severity:** warning | **Confidence:** 0.70
- **Location:** `app/data/memos-dao.js:18-21`
- **Description:** Memo parameter embedded without type checking. Arbitrary fields or nested objects could be persisted.
- **Remediation:** Add schema validation or explicit type checking.

---

### 3. Cross-Site Scripting (XSS)

#### XSS-001: Stored XSS via Memos (marked rendering)
- **Rule ID:** XSS
- **Severity:** error | **Confidence:** 0.95
- **Location:** `app/views/memos.html:31`
- **Description:** User-supplied memo content rendered via `marked()` with autoescape disabled. Full stored XSS chain.
- **Remediation:** Enable Swig autoescape globally. Upgrade `marked` to a current version with proper sanitization. Use DOMPurify for HTML output.

#### XSS-002: Reflected XSS via SSRF Response
- **Rule ID:** XSS
- **Severity:** error | **Confidence:** 0.95
- **Location:** `app/routes/research.js:25`
- **Description:** Response body from `needle.get()` written directly to HTTP response with `text/html` content type. Second-order XSS chained with SSRF.
- **Remediation:** Never render external response bodies as HTML. Set `Content-Type: text/plain` or sanitize output.

#### XSS-003: Stored XSS via Profile Fields
- **Rule ID:** XSS
- **Severity:** error | **Confidence:** 0.95
- **Location:** `app/views/profile.html:24`
- **Description:** Multiple unescaped user fields (lastName, ssn, bankAcc, website, etc.) rendered in profile template.
- **Remediation:** Enable autoescape in Swig. Apply context-appropriate encoding for each output.

#### XSS-004: Reflected XSS in Login Page
- **Rule ID:** XSS
- **Severity:** error | **Confidence:** 0.85
- **Location:** `app/views/login.html:101`
- **Description:** `{{loginError}}` and `{{userName}}` rendered unescaped.
- **Remediation:** Enable autoescape.

#### XSS-005: Reflected XSS in Signup Page
- **Rule ID:** XSS
- **Severity:** error | **Confidence:** 0.85
- **Location:** `app/views/signup.html:57`
- **Description:** Multiple error messages and form values rendered unescaped.
- **Remediation:** Enable autoescape.

#### XSS-006: Stored XSS in Layout Navigation
- **Rule ID:** XSS
- **Severity:** error | **Confidence:** 0.90
- **Location:** `app/views/layout.html:75`
- **Description:** `{{firstName}} {{lastName}}` in global nav bar rendered unescaped. Affects all authenticated pages.
- **Remediation:** Enable autoescape. This is the highest-impact XSS as it affects every page.

#### XSS-007: Stored XSS in Benefits Admin Table
- **Rule ID:** XSS
- **Severity:** error | **Confidence:** 0.90
- **Location:** `app/views/benefits.html:51-52`
- **Description:** User names rendered unescaped in admin benefits table. Privilege escalation vector (user->admin XSS).
- **Remediation:** Enable autoescape.

#### XSS-008: Stored XSS in Allocations View
- **Rule ID:** XSS
- **Severity:** error | **Confidence:** 0.90
- **Location:** `app/views/allocations.html:35`
- **Description:** User names rendered unescaped in allocations page.
- **Remediation:** Enable autoescape.

#### XSS-009: Reflected XSS in Error Template
- **Rule ID:** XSS
- **Severity:** error | **Confidence:** 0.80
- **Location:** `app/views/error-template.html:11`
- **Description:** `{{error}}` rendered unescaped.
- **Remediation:** Enable autoescape.

#### XSS-010: Stored XSS in Memo DAO (insert)
- **Rule ID:** XSS
- **Severity:** error | **Confidence:** 0.95
- **Location:** `app/data/memos-dao.js:15-23`
- **Description:** Memo parameter stored without sanitization. Rendered as HTML via marked library.
- **Remediation:** Sanitize input before storage. Use DOMPurify on output.

#### XSS-011: URL Context Encoding Error
- **Rule ID:** XSS
- **Severity:** warning | **Confidence:** 0.85
- **Location:** `app/routes/profile.js:28`
- **Description:** Website field encoded with `encodeForHTML()` but used in URL href context. Does not prevent `javascript:` URI injection.
- **Remediation:** Use `encodeForURL()` for URL context outputs.

#### XSS-012: Development LiveReload Injection
- **Rule ID:** XSS
- **Severity:** warning | **Confidence:** 0.65
- **Location:** `config/env/development.js:11`
- **Description:** `document.write` injects script from dynamically constructed HTTP URL. Host header manipulation risk.
- **Remediation:** Remove from production configuration. Use static script references.

#### XSS-CONTEXT-001: URL Context Encoding Mismatch
- **Rule ID:** XSS-CONTEXT
- **Severity:** warning | **Confidence:** 0.90
- **Location:** `app/routes/profile.js:28`
- **Description:** Wrong encoding context for website field in href attribute.
- **Remediation:** Use URL-context encoding (`encodeForURL()`).

#### XSS-TEMPLATE-001: Global Autoescape Disabled
- **Rule ID:** XSS-TEMPLATE / SECURITY-MISCONFIG
- **Severity:** error | **Confidence:** 1.0
- **Location:** `server.js:135-142`
- **Description:** Swig `autoescape: false` globally disables HTML escaping. Root cause of all template XSS findings.
- **Remediation:** Set `autoescape: true`. This single fix mitigates the majority of XSS findings.

---

### 4. Server-Side Request Forgery (SSRF)

#### SSRF-001: Unvalidated URL in Research Endpoint
- **Rule ID:** SSRF
- **Severity:** error | **Confidence:** 0.95
- **Location:** `app/routes/research.js:15-16`
- **Description:** `req.query.url` concatenated with `req.query.symbol` and passed to `needle.get()` with zero validation. Attacker can reach internal services or cloud metadata endpoints (e.g., `http://169.254.169.254/`).
- **Remediation:** Implement URL allowlisting. Block private/internal IP ranges. Validate URL scheme (HTTPS only).

---

### 5. Authentication & Session Management

#### AUTH-001: Plaintext Password Storage
- **Rule ID:** BROKEN-AUTH / PLAINTEXT-PASSWORD-STORAGE
- **Severity:** error | **Confidence:** 1.0
- **Location:** `app/data/user-dao.js:20-31`
- **Description:** Passwords stored in plaintext. bcrypt hashing code is commented out.
- **Remediation:** Uncomment and enable bcrypt hashing. Use `bcryptjs` (not the deprecated `bcrypt-nodejs`).

#### AUTH-002: Plaintext Password Comparison
- **Rule ID:** BROKEN-AUTH-COMPARE
- **Severity:** error | **Confidence:** 0.99
- **Location:** `app/data/user-dao.js:61`
- **Description:** Password comparison via `===` instead of `bcrypt.compareSync`.
- **Remediation:** Enable bcrypt comparison.

#### SESSION-001: Session Fixation
- **Rule ID:** SESSION-FIXATION
- **Severity:** error | **Confidence:** 0.95
- **Location:** `app/routes/session.js:116`
- **Description:** Login handler sets `req.session.userId` without calling `req.session.regenerate()`. Pre-set session IDs survive authentication.
- **Remediation:** Call `req.session.regenerate()` before setting session data on login.

#### SESSION-002: Weak Session Secret
- **Rule ID:** SESSION-WEAK-SECRET / HARDCODED_SECRET / HARDCODED-SECRET
- **Severity:** error | **Confidence:** 0.95
- **Location:** `config/env/all.js:8`
- **Description:** Session secret hardcoded as `session_cookie_secret_key_here`. Enables session forgery.
- **Remediation:** Use a high-entropy secret from environment variable.

#### SESSION-003: Missing httpOnly Flag
- **Rule ID:** SESSION-NO-HTTPONLY
- **Severity:** error | **Confidence:** 0.95
- **Location:** `server.js:78-102`
- **Description:** Session cookie accessible to client-side JavaScript, enabling theft via XSS.
- **Remediation:** Set `cookie: { httpOnly: true }` in session configuration.

#### SESSION-004: Missing secure Flag
- **Rule ID:** SESSION-NO-SECURE
- **Severity:** error | **Confidence:** 0.95
- **Location:** `server.js:78-102`
- **Description:** Session cookie sent over plaintext HTTP.
- **Remediation:** Set `cookie: { secure: true }` and enable HTTPS.

#### SESSION-005: No Session Expiry
- **Rule ID:** SESSION-NO-EXPIRY
- **Severity:** warning | **Confidence:** 0.90
- **Location:** `server.js:78-102`
- **Description:** No `maxAge` or `expires` configured. Sessions persist indefinitely.
- **Remediation:** Set appropriate `maxAge` (e.g., 30 minutes for sensitive applications).

#### SESSION-006: Default Session Cookie Name
- **Rule ID:** SESSION-DEFAULT-NAME
- **Severity:** warning | **Confidence:** 0.85
- **Location:** `server.js:78-102`
- **Description:** Default `connect.sid` cookie name leaks framework identity.
- **Remediation:** Set a custom cookie name via `key` option.

---

### 6. Access Control

#### IDOR-001: Allocation Data Access via URL Manipulation
- **Rule ID:** IDOR
- **Severity:** error | **Confidence:** 0.95
- **Location:** `app/routes/allocations.js:17-18`
- **Description:** `userId` read from `req.params` instead of `req.session`. Any authenticated user can access other users' allocations.
- **Remediation:** Read `userId` from `req.session.userId`.

#### IDOR-002: Profile Update Without Authorization
- **Rule ID:** IDOR
- **Severity:** warning | **Confidence:** 0.80
- **Location:** `app/data/profile-dao.js:78-81`
- **Description:** `updateUser` accepts userId parameter with no authorization check at DAO layer.
- **Remediation:** Verify `userId` matches `req.session.userId` before update.

#### IDOR-003: Contributions Type Mismatch
- **Rule ID:** IDOR
- **Severity:** warning | **Confidence:** 0.60
- **Location:** `app/data/contributions-dao.js:28-33`
- **Description:** Type mismatch between query filter (raw userId) and stored value (parseInt) could cause authorization issues.
- **Remediation:** Normalize userId type consistently.

#### ACL-001: Missing Admin Authorization on Benefits
- **Rule ID:** BROKEN-ACCESS-CONTROL
- **Severity:** error | **Confidence:** 0.95
- **Location:** `app/routes/index.js:55-56`
- **Description:** Benefits routes lack `isAdmin` middleware. Any authenticated user can access admin-only functionality.
- **Remediation:** Uncomment and enable `isAdmin` middleware on benefits routes.

#### ACL-002: Mass Assignment in Benefits
- **Rule ID:** MASS-ASSIGNMENT
- **Severity:** warning | **Confidence:** 0.80
- **Location:** `app/routes/benefits.js:29-53`
- **Description:** `userId` and `benefitStartDate` accepted from `req.body` without authorization. Any user can modify any other user's benefits.
- **Remediation:** Add admin authorization check and validate userId ownership.

#### ACL-003: Unauthenticated Tutorial Access
- **Rule ID:** MISSING-AUTHZ
- **Severity:** note | **Confidence:** 0.60
- **Location:** `app/routes/index.js:79`
- **Description:** Tutorial router mounted without `isLoggedIn` middleware. Likely intentional but exposes internal application structure.
- **Remediation:** Confirm intentional. Consider whether tutorial content reveals exploitable information.

---

### 7. Sensitive Data Exposure

#### DATA-001: Plaintext SSN and DOB Storage
- **Rule ID:** SENSITIVE-DATA-EXPOSURE
- **Severity:** error | **Confidence:** 0.95
- **Location:** `app/data/profile-dao.js:62-66`
- **Description:** SSN and date-of-birth stored unencrypted. Encryption helpers are commented out.
- **Remediation:** Enable encryption for PII fields. Use authenticated encryption (AES-256-GCM).

#### DATA-002: Hardcoded Credentials in Seed Script
- **Rule ID:** HARDCODED-CREDENTIALS / HARDCODED_CREDENTIALS
- **Severity:** error | **Confidence:** 1.0
- **Location:** `artifacts/db-reset.js:12-37`
- **Description:** Plaintext passwords (`Admin_123`, `User1_123`, `User2_123`) committed to source control. bcrypt hashes commented out.
- **Remediation:** Use bcrypt-hashed passwords in seed scripts. Never commit plaintext credentials.

#### DATA-003: Hardcoded Crypto Key
- **Rule ID:** HARDCODED_CRYPTO_KEY / WEAK-CRYPTO
- **Severity:** error | **Confidence:** 0.95
- **Location:** `config/env/all.js:9`
- **Description:** Cryptographic key hardcoded as `a_secure_key_for_crypto_here`. All encrypted data is compromised.
- **Remediation:** Load crypto keys from environment variables or secret manager.

#### DATA-004: Configuration Secrets Logged to Console
- **Rule ID:** INFO_EXPOSURE
- **Severity:** warning | **Confidence:** 0.90
- **Location:** `config/config.js:13`
- **Description:** Full config (including secrets) dumped via `util.inspect` at startup.
- **Remediation:** Redact sensitive fields before logging.

#### DATA-005: Hardcoded ZAP API Keys
- **Rule ID:** HARDCODED_API_KEY
- **Severity:** warning | **Confidence:** 0.70
- **Location:** `config/env/development.js:6`, `config/env/test.js:5`
- **Description:** ZAP API key hardcoded and reused across environments.
- **Remediation:** Move API keys to environment variables.

---

### 8. Transport Security

#### TRANSPORT-001: HTTP Only (No HTTPS)
- **Rule ID:** TRANSPORT-HTTP / CLEARTEXT-TRANSPORT
- **Severity:** error | **Confidence:** 0.95
- **Location:** `server.js:145`
- **Description:** Application served over plain HTTP. HTTPS setup exists but is commented out. All traffic is cleartext.
- **Remediation:** Enable HTTPS. Uncomment the HTTPS server configuration.

#### TRANSPORT-002: Security Headers Disabled
- **Rule ID:** MISCONFIG
- **Severity:** warning | **Confidence:** 0.90
- **Location:** `server.js:39`
- **Description:** Helmet (frameguard, noCache, CSP, HSTS, nosniff) and CSRF protection all commented out.
- **Remediation:** Uncomment and enable helmet and csurf middleware.

---

### 9. Cryptography

#### CRYPTO-001: Deprecated bcrypt-nodejs
- **Rule ID:** DEPRECATED-CRYPTO
- **Severity:** warning | **Confidence:** 0.95
- **Location:** `app/data/user-dao.js:1`
- **Description:** bcrypt-nodejs is deprecated and unmaintained since 2017.
- **Remediation:** Migrate to `bcrypt` or `bcryptjs`.

#### CRYPTO-002: Weak AES-CBC without Authentication
- **Rule ID:** WEAK_CRYPTO
- **Severity:** warning | **Confidence:** 0.75
- **Location:** `config/env/all.js:10`
- **Description:** AES-256-CBC without authenticated encryption is vulnerable to padding oracle attacks.
- **Remediation:** Use AES-256-GCM for authenticated encryption.

---

### 10. Input Validation

#### INPUT-001: Missing Registration Validation
- **Rule ID:** INPUT-VALIDATION
- **Severity:** warning | **Confidence:** 0.85
- **Location:** `app/data/user-dao.js:17`
- **Description:** No type/length/format validation on registration fields.
- **Remediation:** Add server-side input validation with schema validation library.

#### INPUT-002: Missing Memo Validation
- **Rule ID:** MISSING-INPUT-VALIDATION
- **Severity:** warning | **Confidence:** 0.90
- **Location:** `app/routes/memos.js:13`
- **Description:** No validation on memo content before database insertion.
- **Remediation:** Validate memo type, length, and content.

#### INPUT-003: ReDoS in Bank Routing Validation
- **Rule ID:** REDOS
- **Severity:** warning | **Confidence:** 0.95
- **Location:** `app/routes/profile.js:59`
- **Description:** Regex `/([0-9]+)+\#/` with nested quantifiers causes catastrophic backtracking.
- **Remediation:** Replace with `/[0-9]+#/` (remove nested group).

---

### 11. Open Redirect

#### REDIRECT-001: Unvalidated Redirect in /learn
- **Rule ID:** OPEN-REDIRECT
- **Severity:** warning | **Confidence:** 0.95
- **Location:** `app/routes/index.js:70-73`
- **Description:** `req.query.url` passed directly to `res.redirect()` without allowlist validation.
- **Remediation:** Implement URL allowlist or restrict to relative paths.

---

### 12. CSRF

#### CSRF-001: Missing CSRF Protection on Signup
- **Rule ID:** CSRF
- **Severity:** warning | **Confidence:** 0.80
- **Location:** `app/routes/index.js:38`
- **Description:** POST /signup lacks CSRF token middleware.
- **Remediation:** Enable csurf middleware (currently commented out in server.js).

---

### 13. Dependency Vulnerabilities

#### DEP-001: marked 0.3.5 (Multiple CVEs)
- **Rule ID:** INSECURE-DEPENDENCY
- **Severity:** error | **Confidence:** 0.95
- **Location:** `server.js:126-128`
- **Description:** CVE-2017-17461, CVE-2022-21680, CVE-2022-21681. Sanitize option is deprecated and bypassable.
- **Remediation:** Upgrade to latest `marked` version. Use DOMPurify for sanitization.

#### DEP-002: Swig 1.4.2 (Unmaintained)
- **Rule ID:** DEPRECATED-LIBRARY
- **Severity:** warning | **Confidence:** 1.0
- **Location:** `server.js:9`
- **Description:** Last updated 2014, archived, no security patches.
- **Remediation:** Migrate to Nunjucks, EJS, or Handlebars.

#### DEP-003: needle 2.2.4 (Prototype Pollution)
- **Rule ID:** PROTO-POLLUTION
- **Severity:** warning | **Confidence:** 0.85
- **Location:** `app/routes/research.js:2`
- **Description:** CVE-2020-7669: prototype pollution via crafted response headers.
- **Remediation:** Upgrade needle to latest version.

#### DEP-004: bcrypt-nodejs (Deprecated)
- **Rule ID:** DEPRECATED-LIBRARY
- **Severity:** warning | **Confidence:** 1.0
- **Location:** `app/data/user-dao.js:1`
- **Description:** Deprecated since 2017, unmaintained.
- **Remediation:** Replace with `bcrypt` or `bcryptjs`.

---

### 14. Database Security

#### DB-001: Unauthenticated MongoDB Connection
- **Rule ID:** NOAUTH
- **Severity:** error | **Confidence:** 0.95
- **Location:** `config/env/all.js:3`, `server.js:30`
- **Description:** MongoDB connection string has no credentials. Database accessible without authentication if network-exposed.
- **Remediation:** Configure MongoDB authentication. Use connection strings with credentials from environment variables.

---

### 15. Logic Errors

#### LOGIC-001: Double Callback in Allocations DAO
- **Rule ID:** DOUBLE_CALLBACK
- **Severity:** warning | **Confidence:** 0.95
- **Location:** `app/data/allocations-dao.js:53`
- **Description:** Callback invoked twice on success path due to missing `return` or `else` guard. Can cause response header double-send crashes.
- **Remediation:** Add `return` before `callback(null, allocations)` or use `else` guard.

---

## Remediation Priority

### Immediate (Critical Risk)
1. **Enable Swig autoescape** (`server.js:137`) - Fixes 10+ XSS findings with a single change
2. **Remove eval() calls** (`contributions.js:32-34`) - Replace with `parseInt()`
3. **Enable bcrypt password hashing** (`user-dao.js:29`) - Uncomment bcrypt code
4. **Fix NoSQL injection in allocations** (`allocations-dao.js:78`) - Use `parseInt()` for threshold
5. **Fix SSRF in research endpoint** (`research.js:15`) - Add URL allowlist

### Short-term (High Risk)
6. Enable HTTPS and secure cookie flags
7. Regenerate session on login (session fixation)
8. Replace hardcoded secrets with environment variables
9. Enable helmet security headers and CSRF protection
10. Fix IDOR by reading userId from session

### Medium-term (Moderate Risk)
11. Upgrade deprecated dependencies (marked, swig, needle, bcrypt-nodejs)
12. Encrypt PII at rest (SSN, DOB, bank details)
13. Add input validation across all endpoints
14. Fix ReDoS in bank routing regex
15. Remove hardcoded credentials from seed scripts
