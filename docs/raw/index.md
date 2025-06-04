# Vulnhuntrs Documentation

Vulnhuntrs is an AI-powered security vulnerability scanner that combines static code analysis with LLMs to detect remotely exploitable vulnerabilities.

## Features

- **Multi-language support**: Supports Rust, Python, JavaScript, TypeScript, Go, Java, and Ruby
- **AI-powered analysis**: Uses large language models to identify complex security vulnerabilities
- **Detailed reports**: Generates comprehensive vulnerability reports with proof-of-concept code
- **Static code analysis**: Combines pattern matching with semantic analysis using tree-sitter

## Quick Start

### Installation

```bash
# Using Docker (recommended)
docker pull ghcr.io/hikaruegashira/vulnhuntrs:latest

# Or build from source
git clone https://github.com/HikaruEgashira/vulnhuntrs
cd vulnhuntrs
cargo build --release
```

### Basic Usage

```bash
# Analyze a local directory
vulnhuntrs -r /path/to/project

# Analyze a GitHub repository
vulnhuntrs --repo owner/repository

# Generate summary report
vulnhuntrs -r /path/to/project --summary
```

## Example Output

```
🔍 Vulnhuntrs - Security Analysis Tool
📁 Found source files (1)
  [1] example/python-vulnerable-app/app.py
🔎 Found security pattern matches (1)
  [P1] example/python-vulnerable-app/app.py
📄 Analyzing: example/python-vulnerable-app/app.py (1 / 1)

📝 Analysis Report
================================================================================

🔍 Analysis Results:
This application contains 3 major vulnerabilities. First, the /sqli endpoint 
directly embeds user-provided 'username' parameter into SQL queries without 
sanitization, enabling SQL injection attacks...
```

## Documentation Sections

- [Getting Started](getting-started.md) - Quick start guide and setup
- [Installation](installation.md) - Detailed installation instructions  
- [Configuration](configuration.md) - Configuration options and settings
- [Usage](usage.md) - Command-line usage and examples
- [Examples](examples.md) - Real-world usage examples and workflows
- [Architecture](architecture.md) - System architecture and components
- [API Reference](api.md) - Complete CLI and library API reference
- [Contributing](contributing.md) - Development and contribution guide