# Getting Started

Get up and running with Vulnhuntrs in minutes. This guide will walk you through the essential steps to start scanning your code for security vulnerabilities.

## Prerequisites

> **Note**: Before you begin, ensure you have:
> - Docker installed (recommended) OR Rust 1.70+ with Cargo
> - An API key from a supported LLM provider (OpenAI, Anthropic, or Google)
> - Access to the code you want to analyze

## Quick Setup

### Step 1: Choose Installation Method

**Docker:**
```bash
# Pull the latest Docker image
docker pull ghcr.io/hikaruegashira/vulnhuntrs:latest
```

**From Source:**
```bash
# Clone and build from source
git clone https://github.com/HikaruEgashira/vulnhuntrs.git
cd vulnhuntrs
cargo build --release
```

### Step 2: Set up API Key

Configure your LLM provider API key:

```bash
# For OpenAI
export OPENAI_API_KEY="your-api-key-here"

# For Anthropic (Claude)
export ANTHROPIC_API_KEY="your-api-key-here"

# For Google (Gemini)
export GOOGLE_API_KEY="your-api-key-here"
```

### Step 3: Run Your First Scan

**Docker:**
```bash
# Analyze current directory
docker run -e OPENAI_API_KEY=$OPENAI_API_KEY \
  -v $(pwd):/app \
  ghcr.io/hikaruegashira/vulnhuntrs:latest \
  -r /app
```

**Binary:**
```bash
# Analyze current directory
./target/release/vulnhuntrs -r .

# Or if built with cargo run
cargo run -- -r .
```

## Your First Security Scan

Let's test Vulnhuntrs with one of the included vulnerable examples:

```bash
# Test with the Python vulnerable app example
vulnhuntrs -r example/python-vulnerable-app/

# Expected output:
🔍 Vulnhuntrs - Security Analysis Tool
📁 Found source files (1)
  [1] example/python-vulnerable-app/app.py
🔎 Found security pattern matches (1)
  [P1] example/python-vulnerable-app/app.py
📄 Analyzing: example/python-vulnerable-app/app.py (1 / 1)

📝 Analysis Report
================================================================================
🔍 Analysis Results:
This application contains 3 major vulnerabilities...
```

> **Success!** If you see vulnerability findings, Vulnhuntrs is working correctly.

## Understanding the Output

The scan results include:

- **📁 Source files found**: Total analyzable files discovered
- **🔎 Pattern matches**: Files that match security vulnerability patterns  
- **📄 Analysis**: Detailed LLM analysis of each matched file
- **📝 Report**: Summary of vulnerabilities with confidence scores

## Next Steps

Now that you have Vulnhuntrs running:

1. **[Configure models and settings](/docs/configuration)** - Customize analysis behavior
2. **[Learn advanced usage patterns](/docs/usage)** - Filter results, generate reports
3. **[Explore examples](/docs/examples)** - See real-world scanning scenarios
4. **[Understand the architecture](/docs/architecture)** - Learn how analysis works

## Common Issues

> **Warning**: **API Rate Limits**: If you encounter rate limiting, try using a different model or adjusting request frequency.

> **Error**: **No vulnerabilities found?** Ensure your code contains actual security issues or test with the provided examples first.

### Getting Help

- Check the [troubleshooting section](/docs/troubleshooting) for common solutions
- View [examples](/docs/examples) for detailed usage scenarios  
- Report issues on [GitHub](https://github.com/HikaruEgashira/vulnhuntrs/issues)