# Threat Model: repo

Generated: 2026-04-04T13:10:21.712173+00:00

**Application type**: web_application

## Summary

This is an intentionally vulnerable Flask application exposing a wide range of classic and modern web vulnerabilities across web routes and REST API endpoints. Every major OWASP Top 10 category is represented, including SQLi, XSS, SSTI, SSRF, command injection, insecure deserialization, and IDOR. The attack surface is extensive and all user-supplied input is treated as trusted.

## Overview

- **Attack surfaces identified**: 24

## Attack Surfaces

### SURFACE-001 — endpoint `POST /api/auth/login`

Authentication endpoint that constructs SQL query via string interpolation (SQLi), logs plaintext passwords, exposes API keys in response, and uses a hardcoded JWT secret.

**Locations**:
- `api.py`

**Query**:
```scheme
(decorated_definition (decorator (call function: (attribute object: (identifier) @bp (#eq? @bp "api_bp")) attribute: (identifier) @route_method (#eq? @route_method "route")) arguments: (argument_list (string) @path (keyword_argument name: (identifier) @methods_kw value: (list (string) @method (#match? @method "POST")))))) @endpoint
```

---

### SURFACE-002 — endpoint `GET /api/user/<user_id>`

IDOR: No authorization check; any caller can retrieve any user's data by guessing an integer ID. The underlying query also uses string interpolation allowing SQL injection.

**Locations**:
- `api.py`
- `models.py`

**Query**:
```scheme
(decorated_definition (decorator (call function: (attribute object: (identifier) @bp (#eq? @bp "api_bp")) attribute: (identifier) @route) arguments: (argument_list (string) @path (#match? @path "user")))) @endpoint
```

---

### SURFACE-003 — endpoint `POST /api/ssrf/fetch`

SSRF: Accepts arbitrary URL from request body and performs an outbound HTTP GET with no allowlist or scheme validation, enabling access to internal services.

**Locations**:
- `api.py`

**Query**:
```scheme
(call function: (attribute object: (identifier) @mod (#eq? @mod "requests") attribute: (identifier) @method (#eq? @method "get")) arguments: (argument_list (identifier) @url)) @ssrf_call
```

---

### SURFACE-004 — endpoint `POST /api/xml/parse`

XXE: Parses raw XML from the request body using xml.etree.ElementTree without disabling external entity resolution, enabling XXE attacks.

**Locations**:
- `api.py`

**Query**:
```scheme
(call function: (attribute object: (identifier) @mod (#eq? @mod "ET") attribute: (identifier) @fn (#eq? @fn "fromstring")) arguments: (argument_list)) @xxe_call
```

---

### SURFACE-005 — endpoint `POST /api/yaml/load`

Unsafe deserialization: Uses yaml.load with FullLoader on untrusted input. FullLoader is insufficient protection; yaml.safe_load should be used instead.

**Locations**:
- `api.py`

**Query**:
```scheme
(call function: (attribute object: (identifier) @mod (#eq? @mod "yaml") attribute: (identifier) @fn (#eq? @fn "load")) arguments: (argument_list)) @yaml_load
```

---

### SURFACE-006 — endpoint `POST /api/pickle/deserialize`

Insecure deserialization: Accepts base64-encoded data from the request and calls pickle.loads directly, enabling arbitrary code execution.

**Locations**:
- `api.py`

**Query**:
```scheme
(call function: (attribute object: (identifier) @mod (#eq? @mod "pickle") attribute: (identifier) @fn (#eq? @fn "loads")) arguments: (argument_list)) @pickle_loads
```

---

### SURFACE-007 — endpoint `POST /api/exec/command`

OS command injection: Passes attacker-controlled 'command' field directly to subprocess.run with shell=True, enabling arbitrary command execution.

**Locations**:
- `api.py`

**Query**:
```scheme
(call function: (attribute object: (identifier) @mod (#eq? @mod "subprocess") attribute: (identifier) @fn (#eq? @fn "run")) arguments: (argument_list (identifier) @cmd (keyword_argument name: (identifier) @shell_kw (#eq? @shell_kw "shell") value: (true)))) @cmd_exec
```

---

### SURFACE-008 — endpoint `POST /api/eval/python`

Code injection: Calls eval() directly on attacker-supplied 'code' field with no sandboxing, enabling arbitrary Python execution.

**Locations**:
- `api.py`

**Query**:
```scheme
(call function: (identifier) @fn (#eq? @fn "eval") arguments: (argument_list (identifier) @code)) @eval_call
```

---

### SURFACE-009 — endpoint `POST /api/template/render`

SSTI: Renders attacker-controlled Jinja2 template string without sandboxing, enabling full server-side template injection and RCE.

**Locations**:
- `api.py`

**Query**:
```scheme
(call function: (attribute object: (identifier) @cls (#eq? @cls "Template") attribute: (identifier) @fn (#eq? @fn "render")) arguments: (argument_list)) @ssti_render
```

---

### SURFACE-010 — endpoint `POST /api/file/upload`

Unrestricted file upload with path traversal: No MIME/extension validation and uses unsanitized filename directly in os.path.join, allowing upload to arbitrary paths.

**Locations**:
- `api.py`

**Query**:
```scheme
(call function: (attribute object: (identifier) @obj attribute: (identifier) @fn (#eq? @fn "save")) arguments: (argument_list (identifier) @path)) @file_save
```

---

### SURFACE-011 — endpoint `POST /api/file/extract`

Zip Slip: Extracts archive with zipfile.extractall to a temp directory without validating member paths, allowing overwrite of arbitrary files via crafted archive.

**Locations**:
- `api.py`

**Query**:
```scheme
(call function: (attribute object: (identifier) @zf attribute: (identifier) @fn (#eq? @fn "extractall")) arguments: (argument_list (identifier) @extract_dir)) @zip_slip
```

---

### SURFACE-012 — endpoint `GET /api/redirect`

Open redirect: Redirects to attacker-controlled 'next' URL parameter without validating the destination host, enabling phishing attacks.

**Locations**:
- `api.py`

**Query**:
```scheme
(call function: (identifier) @fn (#eq? @fn "redirect") arguments: (argument_list (identifier) @next_url)) @open_redirect
```

---

### SURFACE-013 — endpoint `GET /api/logs/<user_id>`

IDOR + SQL Injection: Log retrieval uses unsanitized user_id in a raw SQL query with no authorization check, exposing audit logs of any user.

**Locations**:
- `api.py`
- `models.py`

**Query**:
```scheme
(decorated_definition (decorator (call function: (attribute object: (identifier) @bp (#eq? @bp "api_bp")) attribute: (identifier) @route) arguments: (argument_list (string) @path (#match? @path "logs")))) @endpoint
```

---

### SURFACE-014 — endpoint `GET /sqli`

SQL injection: Both 'username' (LIKE clause) and 'order' (ORDER BY clause) query parameters are interpolated directly into SQL strings, enabling full SQLi including ORDER BY injection.

**Locations**:
- `app.py`

**Query**:
```scheme
(call function: (attribute object: (identifier) @conn attribute: (identifier) @fn (#match? @fn "execute")) arguments: (argument_list (identifier) @query)) @sqli_execute
```

---

### SURFACE-015 — endpoint `GET /cmdi`

Command injection: 'hostname' and 'count' parameters are interpolated into os.popen and os.system calls without sanitization, enabling arbitrary OS command execution.

**Locations**:
- `app.py`

**Query**:
```scheme
(call function: (attribute object: (identifier) @os (#eq? @os "os") attribute: (identifier) @fn (#match? @fn "popen|system")) arguments: (argument_list (formatted_string) @cmd)) @cmdi_call
```

---

### SURFACE-016 — endpoint `GET /lfi`

Local File Inclusion: 'file' parameter is passed directly to open() with no path validation, allowing arbitrary file read including /etc/passwd.

**Locations**:
- `app.py`

**Query**:
```scheme
(call function: (identifier) @fn (#eq? @fn "open") arguments: (argument_list (identifier) @path)) @lfi_open
```

---

### SURFACE-017 — endpoint `GET /ssti`

SSTI: Attacker-controlled 'template' parameter is rendered via Jinja2 Template() without sandboxing, enabling RCE via class traversal payloads.

**Locations**:
- `app.py`

**Query**:
```scheme
(call function: (attribute object: (call function: (identifier) @cls (#eq? @cls "Template") arguments: (argument_list (identifier) @tmpl)) attribute: (identifier) @fn (#eq? @fn "render")) arguments: (argument_list)) @ssti
```

---

### SURFACE-018 — endpoint `GET /xss`

Reflected XSS: 'name' and 'comment' parameters are interpolated directly into an f-string HTML template rendered via render_template_string, bypassing Jinja2 auto-escaping.

**Locations**:
- `app.py`

**Query**:
```scheme
(call function: (identifier) @fn (#eq? @fn "render_template_string") arguments: (argument_list (identifier) @tmpl)) @xss_render
```

---

### SURFACE-019 — endpoint `GET /logs`

IDOR: 'user_id' parameter overrides the session user ID with no ownership check, allowing any authenticated user to view another user's audit logs.

**Locations**:
- `app.py`
- `models.py`

**Query**:
```scheme
(decorated_definition (decorator (call function: (attribute object: (identifier) @app (#eq? @app "app")) attribute: (identifier) @route) arguments: (argument_list (string) @path (#eq? @path "/logs")))) @logs_endpoint
```

---

### SURFACE-020 — db_table `users`

Stores plaintext passwords, API keys, and session tokens. Multiple query sites use string interpolation instead of parameterized queries, exposing the table to SQL injection across the application.

**Locations**:
- `models.py`

**Query**:
```scheme
(call function: (attribute object: (identifier) @cursor attribute: (identifier) @fn (#eq? @fn "execute")) arguments: (argument_list (concatenated_string) @query)) @raw_sql
```

---

### SURFACE-021 — db_table `audit_logs`

AuditLogger.log_action builds INSERT via f-string with user-controlled 'details' and 'ip_address', making the audit log itself injectable and untrusted as a forensic record.

**Locations**:
- `models.py`

**Query**:
```scheme
(function_definition name: (identifier) @fn (#eq? @fn "log_action") body: (block (expression_statement (call function: (attribute object: (identifier) @cursor attribute: (identifier) @exec (#eq? @exec "execute")) arguments: (argument_list (identifier) @query))) @audit_insert)) @log_action_def
```

---

### SURFACE-022 — public_api `UserModel.authenticate_user`

Constructs authentication SQL via f-string interpolation with unsanitized username and password, enabling authentication bypass via SQL injection (e.g. ' OR '1'='1).

**Locations**:
- `models.py`

**Query**:
```scheme
(function_definition name: (identifier) @fn (#eq? @fn "authenticate_user") body: (block (expression_statement (assignment left: (identifier) @query right: (formatted_string) @fstring)))) @auth_fn
```

---

### SURFACE-023 — public_api `UserModel.update_user_preferences`

Deserializes attacker-controlled input using pickle.loads encoded as latin1, enabling arbitrary code execution when called with untrusted preference data.

**Locations**:
- `models.py`

**Query**:
```scheme
(call function: (attribute object: (identifier) @mod (#eq? @mod "pickle") attribute: (identifier) @fn (#eq? @fn "loads")) arguments: (argument_list)) @pickle_loads_model
```

---

### SURFACE-024 — endpoint `POST /api/ldap/search`

LDAP injection: 'username' parameter is interpolated directly into an LDAP filter string without escaping, enabling filter manipulation and potential authentication bypass if connected to a real LDAP server.

**Locations**:
- `api.py`

**Query**:
```scheme
(assignment left: (identifier) @var (#match? @var "ldap_query") right: (formatted_string) @fstring) @ldap_inject
```

---

