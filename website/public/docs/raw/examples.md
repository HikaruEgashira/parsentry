# Examples

Learn how to use Vulnhuntrs effectively with these real-world examples and use cases.

## Quick Start Examples

### Analyzing Local Projects

**Python Web App:**
```bash
# Analyze a Flask/Django application
vulnhuntrs -r ./my-python-app --vuln-types SQLI,XSS,RCE

# Focus on critical web vulnerabilities
vulnhuntrs -r ./my-python-app \
  --min-confidence 7 \
  --vuln-types SQLI,XSS,CSRF,IDOR
```
**Common findings**: SQL injection, XSS, insecure deserialization

**Node.js API:**
```bash
# Analyze Express.js or similar API
vulnhuntrs -r ./my-node-api \
  --exclude-dirs "node_modules" \
  --vuln-types RCE,SQLI,SSRF

# Include TypeScript files
vulnhuntrs -r ./my-node-api \
  --include-patterns "*.js,*.ts" \
  --min-confidence 6
```
**Common findings**: Command injection, NoSQL injection, prototype pollution

**Go Service:**
```bash
# Analyze Go microservice
vulnhuntrs -r ./my-go-service \
  --exclude-dirs "vendor" \
  --vuln-types RCE,SQLI,SSRF

# Focus on specific packages
vulnhuntrs -r ./cmd,./internal \
  --min-confidence 7
```
**Common findings**: Command injection, SQL injection, path traversal

**Rust Application:**
```bash
# Analyze Rust project
vulnhuntrs -r ./my-rust-app \
  --exclude-dirs "target" \
  --vuln-types RCE,SQLI

# Focus on unsafe code
vulnhuntrs -r ./src \
  --min-confidence 8
```
**Common findings**: Unsafe memory operations, external command execution

### GitHub Repository Analysis

```bash
# Analyze popular vulnerable applications
vulnhuntrs --repo OWASP/WebGoat --vuln-types XSS,SQLI,CSRF
vulnhuntrs --repo PentesterLab/cr-go --min-confidence 6
vulnhuntrs --repo juice-shop/juice-shop --summary

# Analyze your own repositories
vulnhuntrs --repo yourusername/your-private-repo
```

## Built-in Vulnerable Examples

Vulnhuntrs includes several vulnerable example applications for testing:

```
example/
├── python-vulnerable-app/
│   ├── app.py
│   └── requirements.txt
├── go-vulnerable-app/
│   ├── main.go
│   └── go.mod
├── ruby-vulnerable-app/
│   └── app.rb
└── rust-vulnerable-app/
    ├── src/main.rs
    └── Cargo.toml
```

### Python Vulnerable App

```bash
# Test with the Python example
vulnhuntrs -r example/python-vulnerable-app/

# Expected findings:
# - SQL Injection in /sqli endpoint
# - Command Injection in /command endpoint  
# - Path Traversal in /file endpoint
```

<details>
<summary>View Python example vulnerabilities</summary>

```python
# SQL Injection
@app.route('/sqli')
def sqli():
    username = request.args.get('username')
    query = f"SELECT * FROM users WHERE username = '{username}'"
    cursor.execute(query)  # ❌ Direct SQL injection
    
# Command Injection  
@app.route('/command')
def command():
    filename = request.args.get('filename')
    os.system(f"cat {filename}")  # ❌ Command injection

# Path Traversal
@app.route('/file')
def file():
    filename = request.args.get('filename') 
    return open(filename).read()  # ❌ Path traversal
```
</details>

### Go Vulnerable App

```bash
# Test with the Go example
vulnhuntrs -r example/go-vulnerable-app/

# Expected findings:
# - SQL Injection in database queries
# - Command Injection in system calls
# - Path Traversal in file operations
```

## CI/CD Integration Examples

### GitHub Actions

```yaml
# .github/workflows/security.yml
name: Security Scan

on: [push, pull_request]

jobs:
  security-scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Run Vulnhuntrs Security Scan
        run: |
          docker run \
            -e OPENAI_API_KEY=${{ secrets.OPENAI_API_KEY }} \
            -v ${{ github.workspace }}:/app \
            -v ${{ github.workspace }}/reports:/reports \
            ghcr.io/hikaruegashira/vulnhuntrs:latest \
            -r /app --output-dir /reports --min-confidence 7
            
      - name: Upload Security Reports
        uses: actions/upload-artifact@v4
        with:
          name: security-reports
          path: reports/
          
      - name: Comment PR with Results
        if: github.event_name == 'pull_request'
        run: |
          if [ -f "reports/summary.md" ]; then
            gh pr comment ${{ github.event.number }} --body-file reports/summary.md
          fi
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### GitLab CI

```yaml
# .gitlab-ci.yml
security-scan:
  stage: test
  image: docker:latest
  services:
    - docker:dind
  script:
    - mkdir -p reports
    - docker run 
        -e OPENAI_API_KEY=$OPENAI_API_KEY
        -v $PWD:/app
        -v $PWD/reports:/reports
        ghcr.io/hikaruegashira/vulnhuntrs:latest
        -r /app --output-dir /reports --summary
  artifacts:
    reports:
      junit: reports/*.xml
    paths:
      - reports/
    expire_in: 1 week
  only:
    - merge_requests
    - main
```

## Advanced Use Cases

### Large Codebase Analysis

For enterprise-scale applications:

```bash
# Phase 1: Quick high-confidence scan
vulnhuntrs -r . \
  --min-confidence 8 \
  --vuln-types RCE,SQLI \
  --exclude-dirs "node_modules,target,vendor,.git" \
  --summary

# Phase 2: Detailed analysis of critical paths
vulnhuntrs -r ./src,./api,./core \
  --min-confidence 6 \
  --output-dir ./detailed-reports

# Phase 3: Language-specific deep dive
vulnhuntrs -r . \
  --include-patterns "*.py" \
  --vuln-types SQLI,XSS,RCE \
  --min-confidence 5
```

### Security Assessment Pipeline

```bash
#!/bin/bash
# security-assessment.sh

set -e

echo "🔍 Starting comprehensive security assessment..."

# Create reports directory
mkdir -p security-reports/{quick,detailed,by-language}

# Quick high-confidence scan
echo "📋 Phase 1: Quick scan for critical vulnerabilities"
vulnhuntrs -r . \
  --min-confidence 8 \
  --vuln-types RCE,SQLI \
  --output-dir security-reports/quick \
  --summary

# Detailed analysis by language
echo "📋 Phase 2: Language-specific analysis"
for lang in "*.py" "*.js" "*.go" "*.rs" "*.java" "*.rb"; do
  echo "  Analyzing $lang files..."
  vulnhuntrs -r . \
    --include-patterns "$lang" \
    --min-confidence 6 \
    --output-dir "security-reports/by-language/${lang//\*/}" \
    --summary || true
done

# Comprehensive detailed scan
echo "📋 Phase 3: Comprehensive detailed analysis"
vulnhuntrs -r . \
  --min-confidence 5 \
  --output-dir security-reports/detailed \
  --summary

echo "✅ Security assessment complete. Check security-reports/ for results."
```

### Docker Compose Workflow

```yaml
# docker-compose.security.yml
version: '3.8'

services:
  vulnhuntrs:
    image: ghcr.io/hikaruegashira/vulnhuntrs:latest
    environment:
      - OPENAI_API_KEY=${OPENAI_API_KEY}
    volumes:
      - .:/app:ro
      - ./security-reports:/reports
    command: >
      -r /app 
      --output-dir /reports 
      --min-confidence 7
      --summary
    user: "${UID:-1000}:${GID:-1000}"

  report-server:
    image: nginx:alpine
    ports:
      - "8080:80"
    volumes:
      - ./security-reports:/usr/share/nginx/html:ro
    depends_on:
      - vulnhuntrs
```

Run with:
```bash
docker-compose -f docker-compose.security.yml up vulnhuntrs
docker-compose -f docker-compose.security.yml up report-server
```

## Report Analysis Examples

### Understanding Output

> **Info**: Vulnhuntrs reports include confidence scores (0-10), vulnerability types, and proof-of-concept code to help you understand and verify findings.

Example report structure:
```
📝 Analysis Report
================================================================================

🔍 Analysis Results:
The application contains a critical SQL injection vulnerability in the login 
function. The user-provided 'username' parameter is directly embedded into 
SQL queries without sanitization.

🔨 PoC (Proof of Concept):
POST /login
username=' OR '1'='1' --&password=anything

📄 Related Code Context:
Function: login_user (lines 45-52)
File: src/auth.py

📓 Analysis Notes:
- Confidence: 9/10
- Type: SQLI
- Impact: Full database access
- Recommendation: Use parameterized queries
```

### Filtering and Prioritization

```bash
# Focus on actionable high-confidence findings
vulnhuntrs -r . --min-confidence 8 --summary

# Security audit for specific vulnerability classes
vulnhuntrs -r . --vuln-types RCE,SQLI --min-confidence 6

# Development workflow - catch medium confidence issues
vulnhuntrs -r . --min-confidence 5 --vuln-types XSS,CSRF,IDOR
```

## Next Steps

- [Learn about configuration options](/docs/configuration) 
- [Understand the analysis architecture](/docs/architecture)
- [Contribute to the project](/docs/contributing)