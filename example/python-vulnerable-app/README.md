# Advanced Python Vulnerable Application

This is a sophisticated, intentionally vulnerable Python web application designed for testing advanced security analysis tools and educational purposes. It features a multi-layered architecture with complex vulnerability patterns.

## Architecture Overview

- **Multi-layered Design**: Separated models, API endpoints, and web interface
- **Complex Attack Vectors**: Multiple injection points and chained vulnerabilities
- **Real-world Patterns**: Mimics common enterprise application vulnerabilities

## Vulnerabilities Included

### Classic Web Vulnerabilities
- **SQL Injection** (CWE-89) - Multiple injection points in `/sqli`
- **Cross-Site Scripting** (CWE-79) - Various contexts in `/xss`
- **Command Injection** (CWE-78) - Multiple vectors in `/cmdi`
- **Local File Inclusion** (CWE-22) - Path traversal in `/lfi`
- **Server-Side Template Injection** (CWE-94) - SSTI in `/ssti`

### API Vulnerabilities
- **Authentication Bypass** - Vulnerable login at `/api/auth/login`
- **Insecure Direct Object Reference** (IDOR) - User/document access
- **Server-Side Request Forgery** (SSRF) - URL fetching at `/api/ssrf/fetch`
- **XML External Entity** (XXE) - XML parsing at `/api/xml/parse`
- **YAML Deserialization** - Unsafe loading at `/api/yaml/load`
- **Pickle Deserialization** - Code execution at `/api/pickle/deserialize`
- **LDAP Injection** - Directory queries at `/api/ldap/search`

### File Operation Vulnerabilities
- **Unrestricted File Upload** - Arbitrary file upload at `/api/file/upload`
- **Zip Slip** - Archive extraction at `/api/file/extract`
- **Path Traversal** - Document access at `/api/documents/<id>/content`

### Session & Authentication Issues
- **Weak Session Management** - Predictable tokens, session fixation
- **Information Disclosure** - Verbose error messages, credential logging
- **Hardcoded Secrets** - API keys, JWT secrets in code

### Data Exposure
- **Sensitive Information Logging** - Passwords, tokens in logs
- **Database Information Disclosure** - Raw SQL errors exposed
- **API Key Exposure** - Keys returned in responses

## Application Structure

```
python-vulnerable-app/
├── app.py          # Main Flask application with web vulnerabilities
├── api.py          # API endpoints with complex vulnerabilities  
├── models.py       # Database models with injection vulnerabilities
├── requirements.txt # Python dependencies
└── README.md       # This file
```

## Setup

```bash
# Create virtual environment
python -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Run the application
python app.py
```

The application will start on `http://localhost:5000`

## Usage Examples

### Web Interface
- Navigate to `http://localhost:5000` for the main interface
- Login with `admin/admin123` or `guest/guest`
- Explore various vulnerability categories

### API Testing
```bash
# SSRF Example
curl -X POST http://localhost:5000/api/ssrf/fetch \
  -H "Content-Type: application/json" \
  -d '{"url": "http://internal-server/admin"}'

# XXE Example  
curl -X POST http://localhost:5000/api/xml/parse \
  -H "Content-Type: application/xml" \
  -d '<?xml version="1.0"?><!DOCTYPE foo [<!ENTITY xxe SYSTEM "file:///etc/passwd">]><root>&xxe;</root>'

# Command Injection
curl -X POST http://localhost:5000/api/exec/command \
  -H "Content-Type: application/json" \
  -d '{"command": "ls -la; id"}'
```

## Security Testing Focus Areas

1. **Multi-vector Injection**: Test how tools detect injection across different contexts
2. **Complex Data Flow**: Trace vulnerabilities through model -> API -> response layers  
3. **Chained Vulnerabilities**: Authentication bypass -> IDOR -> data exposure
4. **Deserialization Attacks**: Multiple unsafe deserialization endpoints
5. **File System Attacks**: Upload, extraction, and inclusion vulnerabilities

## Educational Value

This application is designed to challenge security analysis tools with:
- **Real-world Complexity**: Multi-layered architecture similar to enterprise apps
- **Advanced Patterns**: Beyond simple GET parameter injection
- **Multiple Attack Surfaces**: Web interface, REST API, file operations
- **Subtle Vulnerabilities**: Information disclosure, session management issues

## Security Notice

⚠️ **CRITICAL WARNING**: This application contains severe security vulnerabilities by design. 

- **DO NOT** deploy in production environments
- **DO NOT** expose to public networks
- **USE ONLY** in isolated testing environments
- **ENSURE** proper network segmentation when testing

This application is intended solely for security research, tool testing, and educational purposes.