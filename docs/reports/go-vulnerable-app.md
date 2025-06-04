# Go Vulnerable App - Vulnerability Report

## Analysis Summary

**Confidence Score:** 80 (🟠 Medium-High)

**Detected Vulnerability Types:**
- SQL Injection (SQLI)
- Cross-Site Scripting (XSS)
- Remote Code Execution (RCE)
- Local File Inclusion (LFI)

## Vulnerability Analysis

This Go application contains multiple critical vulnerabilities. User input is directly embedded into format strings without proper escaping or validation, resulting in the following vulnerabilities:

### 1. SQL Injection (SQLI)
The sqliHandler embeds user input directly using fmt.Sprintf, allowing query manipulation.

### 2. Cross-Site Scripting (XSS)
The xssHandler concatenates templates as strings, bypassing html/template's escape mechanisms.

### 3. Command Injection (RCE)
The cmdiHandler directly connects input to ping commands and executes via exec.Command("sh","-c",...) through shell.

### 4. Path Traversal/Local File Reading (LFI)
The fileHandler only logs directory traversal attempts but doesn't restrict actual file reading.

## Proof of Concept (PoC)

### 1. SQL Injection
```
GET http://127.0.0.1:8080/sqli?username=' OR '1'='1
```

### 2. Cross-Site Scripting
```
GET http://127.0.0.1:8080/xss?name=<script>alert(1)</script>
```

### 3. Command Injection
```
GET http://127.0.0.1:8080/cmdi?hostname=localhost;id
```

### 4. File Traversal (LFI)
```
GET http://127.0.0.1:8080/file?name=../main.go
```

## Code Context

### Function: sqliHandler
- **Issue:** User input is directly embedded into SQL statements without parameterization, allowing SQL injection
- **Code:** `query := fmt.Sprintf("SELECT id, username, password FROM users WHERE username = '%s'", username)`
- **Location:** example/go-vulnerable-app/main.go

### Function: xssHandler
- **Issue:** User input is embedded into format strings to generate raw HTML, disabling escaping and enabling XSS
- **Code:**
```go
tmplStr := fmt.Sprintf(`
<!DOCTYPE html>
<html>
<head>
    <title>XSS Test</title>
</head>
<body>
    <h2>Hello, %s!</h2>
    <p>Your input was: %s</p>
</body>
</html>`, name, name)
```
- **Location:** example/go-vulnerable-app/main.go

### Function: cmdiHandler
- **Issue:** User input is directly embedded into shell commands and executed via exec.Command("sh","-c",...), allowing command injection
- **Code:** `command := fmt.Sprintf("ping -c 1 %s", hostname)`
- **Location:** example/go-vulnerable-app/main.go

### Function: fileHandler
- **Issue:** File name ".." check is only for logging and doesn't block actual reading, allowing arbitrary file reading (LFI)
- **Code:** `content, err := os.ReadFile(filename)`
- **Location:** example/go-vulnerable-app/main.go

## Analysis Notes

Investigated user-controlled input per endpoint. Focused on fmt.Sprintf usage locations and os.ReadFile, exec.Command. No escaping/validation present, typical code injection and LFI vulnerabilities exist.