# Advanced Rust Vulnerable Application

A sophisticated, intentionally vulnerable Rust web application designed for testing advanced security analysis tools and educational purposes. Features enterprise-level complexity with multi-layered architecture and advanced vulnerability patterns.

## Architecture Overview

- **Multi-layered Design**: Separated database models, API endpoints, and web interface
- **Complex Attack Vectors**: 15+ vulnerability types across different contexts
- **Real-world Patterns**: Enterprise application structure with chained vulnerabilities
- **Rust-specific Vulnerabilities**: Memory safety bypasses and unsafe operations

## Vulnerabilities Included

### Classic Web Vulnerabilities
- **SQL Injection** (CWE-89) - Multiple injection points in authentication and search
- **Command Injection** (CWE-78) - System command execution via multiple vectors
- **Path Traversal** (CWE-22) - File inclusion and directory traversal attacks
- **Insecure Direct Object Reference** (CWE-639) - User and document access without authorization

### API Security Vulnerabilities
- **Authentication Bypass** - Weak session management and credential validation
- **Server-Side Request Forgery** (CWE-918) - SSRF via URL parameter at `/api/ssrf/fetch`
- **XML External Entity** (CWE-611) - XXE vulnerability in XML parsing at `/api/xml/parse`
- **Unsafe Deserialization** (CWE-502) - Binary deserialization at `/api/deserialize`
- **YAML Deserialization** - Unsafe YAML loading at `/api/yaml/load`
- **LDAP Injection** (CWE-90) - Directory query manipulation at `/api/ldap/search`

### File Operation Vulnerabilities
- **Unrestricted File Upload** - Arbitrary file upload at `/api/file/upload`
- **Directory Traversal** - Unauthorized directory listing at `/api/files/list`
- **Static File Exposure** - Vulnerable static file serving with path traversal

### Information Disclosure
- **Sensitive Data Logging** - Passwords and tokens logged in audit trails
- **Verbose Error Messages** - Database errors and stack traces exposed
- **Hardcoded Secrets** - API keys and JWT secrets embedded in code

## Application Structure

```
rust-vulnerable-app/
├── src/
│   ├── main.rs         # Main application with web interface and API endpoints
│   └── lib.rs          # Database models and vulnerable utility functions
├── Cargo.toml          # Dependencies including web, database, and crypto libraries
├── Cargo.lock          # Dependency lock file
└── README.md           # This file
```

## Setup

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build the project
cargo build

# Run the application
cargo run
```

The server will start at `http://127.0.0.1:8080`

## Usage Examples

### Web Interface
- Navigate to `http://127.0.0.1:8080` for the comprehensive vulnerability showcase
- Interactive documentation with examples for each vulnerability type

### API Testing
```bash
# SSRF Example
curl -X POST http://127.0.0.1:8080/api/ssrf/fetch \
  -H "Content-Type: application/json" \
  -d '{"url": "http://internal-server/admin"}'

# XXE Example
curl -X POST http://127.0.0.1:8080/api/xml/parse \
  -H "Content-Type: application/xml" \
  -d '<?xml version="1.0"?><!DOCTYPE foo [<!ENTITY xxe SYSTEM "file:///etc/passwd">]><root>&xxe;</root>'

# Command Injection
curl -X POST http://127.0.0.1:8080/api/exec/command \
  -H "Content-Type: application/json" \
  -d '{"command": "ls -la; id"}'

# Authentication Bypass
curl -X POST http://127.0.0.1:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin\' OR 1=1 --", "password": "anything"}'
```

### Vulnerability Categories by Endpoint

#### Web Interface
- `GET /` - Main dashboard with vulnerability overview
- `GET /sqli?username=admin&order=id` - SQL injection playground
- `GET /cmdi?hostname=localhost&count=1` - Command injection testing
- `GET /file?name=../../etc/passwd` - Path traversal demonstration

#### Authentication & Authorization
- `POST /api/auth/login` - Vulnerable authentication with injection
- `GET /api/user/{id}` - IDOR vulnerability in user access
- `GET /api/logs/{user_id}` - Audit log access without authorization

#### Network & Injection Attacks
- `POST /api/ssrf/fetch` - Server-side request forgery
- `POST /api/xml/parse` - XML external entity injection
- `POST /api/yaml/load` - YAML deserialization attacks
- `POST /api/deserialize` - Binary deserialization vulnerabilities
- `POST /api/ldap/search` - LDAP injection in directory queries

#### File Operations
- `POST /api/file/upload` - Unrestricted file upload
- `GET /api/files/list?dir=/etc` - Directory traversal
- `GET /api/documents/{id}/content` - Document access with path traversal
- `GET /static/*` - Static file serving with path traversal

#### Data Access
- `GET /api/documents/search?q=test&user_id=1` - Document search with injection
- `POST /api/exec/command` - Direct command execution

## Security Testing Focus Areas

1. **Rust-specific Patterns**: How tools detect unsafe operations and memory issues
2. **Multi-vector Injection**: SQL, command, XML, YAML, and LDAP injection across endpoints
3. **Complex Data Flow**: Trace vulnerabilities through model -> API -> response layers
4. **Authentication Bypass**: Multiple methods including SQL injection and weak tokens
5. **File System Attacks**: Upload, traversal, and inclusion vulnerabilities
6. **Deserialization**: Binary and text-based unsafe deserialization

## Educational Value

This application challenges security analysis tools with:
- **Enterprise Complexity**: Multi-layered Rust architecture with proper error handling
- **Advanced Patterns**: Beyond simple parameter injection
- **Multiple Attack Surfaces**: Web interface, REST API, file operations
- **Subtle Vulnerabilities**: Information disclosure, weak cryptography, session management
- **Rust-specific Issues**: Unsafe blocks, memory operations, and type confusion

## Dependencies

The application uses realistic enterprise dependencies:
- `actix-web` - Modern web framework
- `rusqlite` - SQLite database integration
- `reqwest` - HTTP client for SSRF
- `serde` - Serialization framework
- `xml-rs`, `yaml-rust` - Parsing libraries with vulnerabilities
- `base64`, `sha2` - Cryptographic operations
- `log`, `env_logger` - Logging framework

## Security Notice

⚠️ **CRITICAL WARNING**: This application contains severe security vulnerabilities by design.

- **DO NOT** deploy in production environments
- **DO NOT** expose to public networks
- **USE ONLY** in isolated testing environments
- **ENSURE** proper network segmentation when testing

This application is intended solely for security research, tool testing, and educational purposes.