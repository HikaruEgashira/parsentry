# Ruby Vulnerable App - Vulnerability Report

## Analysis Summary

**Confidence Score:** 80 (🟠 Medium-High)

**Detected Vulnerability Types:**
- SQL Injection (SQLI)
- Cross-Site Scripting (XSS)

## Vulnerability Analysis

This application has critical vulnerabilities where the username parameter from GET '/me' is concatenated into SQL query strings without sanitization and embedded into HTML without escaping.

### 1. Entry Point and User-Controllable Input
- GET '/me' username parameter

### 2. Data Flow and Vulnerability Points
- params['username'] → `query = "SELECT * FROM users WHERE username = '#{username}'"` direct concatenation → SQL Injection (SQLI)
- Same value passed to HTML form value attribute and `<pre>` tag without escaping → Reflected XSS

### 3. Impact
- SQLI: Authentication bypass, database information leakage, table destruction
- XSS: Session hijacking and phishing attacks

### 4. Mitigation
- Use prepared statements or parameter binding for query execution
- Apply HTML escape libraries (Rack::Utils.escape_html, etc.)

## Proof of Concept (PoC)

### 1) SQL Injection for Authentication Bypass
```
URL: http://localhost:4567/me?username=' OR '1'='1
```
→ WHERE clause always evaluates to true, returning information for all users

### 2) Reflected XSS for JavaScript Execution
```
URL: http://localhost:4567/me?username=<script>alert(1)</script>
```
→ Alert displays in browser

## Code Context

### Function: SQL Injection
- **Issue:** User input is directly concatenated into SQL strings
- **Code:** `query = "SELECT * FROM users WHERE username = '#{username}'"`
- **Location:** example/ruby-vulnerable-app/app.rb

### Function: Reflected XSS
- **Issue:** User input is embedded into HTML without escaping
- **Code:** `<input type="text" name="username" value="#{username}">`
- **Location:** example/ruby-vulnerable-app/app.rb

## Analysis Notes

Entry point: GET /me username parameter
→ SQLI at SQL string direct concatenation location
→ XSS at HTML output location
→ Lacking PreparedStatement and HTML escaping