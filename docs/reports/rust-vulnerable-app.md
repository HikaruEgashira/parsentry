# Rust Vulnerable App - Vulnerability Report

## Analysis Summary

**Confidence Score:** 80 (🟠 Medium-High)

**Detected Vulnerability Types:**
- SQL Injection (SQLI)
- Remote Code Execution (RCE)
- Local File Inclusion (LFI)

## Vulnerability Analysis

This application contains the following three critical vulnerabilities:

### 1. SQL Injection (SQLI)
- **Entry Point:** GET /sqli?username=...
- **Issue:** Username obtained from user is directly embedded into query string using format! macro, allowing execution of arbitrary SQL through malicious input.
- **Impact:** Authentication bypass, leakage of all user information, data tampering, etc.

### 2. Command Injection (RCE)
- **Entry Point:** GET /cmdi?hostname=...
- **Issue:** Hostname parameter is passed directly to shell commands, allowing execution of arbitrary commands within `sh -c "ping -c 1 ${hostname}"`.
- **Impact:** Execution of arbitrary commands on server, data loss, malware infection, etc.

### 3. Path Traversal (LFI)
- **Entry Point:** GET /file?name=...
- **Issue:** User-specified filenames are read by `std::fs::read_to_string` without sanitization, allowing arbitrary file reference using "../".
- **Impact:** Reading of system confidential files, information leakage.

All endpoints lack input validation or sanitization entirely, and authentication/authorization is not implemented, creating immediate serious risks.

## Proof of Concept (PoC)

### 1) SQL Injection
```
GET /sqli?username=' OR '1'='1
```
→ Returns all user information

### 2) Command Injection
```
GET /cmdi?hostname=localhost;id
```
→ Executes shell command `id` and returns results

### 3) Path Traversal
```
GET /file?name=../../etc/passwd
```
→ Returns contents of /etc/passwd

## Code Context

### Function: sql_injection
- **Issue:** User input is directly embedded into SQL queries, creating SQL injection vulnerability
- **Code:** `let query_str = format!("SELECT * FROM users WHERE username = '{}'", username);`
- **Location:** src/main.rs

### Function: command_injection
- **Issue:** User-controlled hostname is passed to shell commands, enabling command injection
- **Code:** `let output = Command::new("sh").arg("-c").arg(format!("ping -c 1 {}", hostname)).output()`
- **Location:** src/main.rs

### Function: file_read
- **Issue:** User-specified file paths are read without sanitization, enabling path traversal
- **Code:** `match std::fs::read_to_string(&filename)`
- **Location:** src/main.rs

## Analysis Notes

Found three vulnerabilities: SQLI in sql_injection, RCE in command_injection, LFI in file_read. No sanitization or parameterization. Provided Japanese analysis and PoC.