# Configuration

Vulnhuntrs offers extensive configuration options to customize your security analysis workflow.

## Environment Variables

### API Keys

Configure your preferred LLM provider:

```bash
# OpenAI (GPT models)
export OPENAI_API_KEY="sk-..."

# Anthropic (Claude models)  
export ANTHROPIC_API_KEY="sk-ant-..."

# Google (Gemini models)
export GOOGLE_API_KEY="..."
```

### Model Selection

Override the default model via environment variable:

```bash
export ANTHROPIC_MODEL="claude-3-sonnet-20240229"
export OPENAI_MODEL="gpt-4-turbo"
export GOOGLE_MODEL="gemini-pro"
```

## Command Line Options

### Analysis Scope

Control what gets analyzed:

```bash
# Analyze specific directory
vulnhuntrs -r /path/to/project

# Analyze single file
vulnhuntrs -a src/vulnerable.py

# Analyze GitHub repository
vulnhuntrs --repo owner/repository
```

### Model Configuration

**OpenAI:**
```bash
# Use GPT-4 Turbo (recommended)
vulnhuntrs -r . --model gpt-4-turbo

# Use GPT-4 
vulnhuntrs -r . --model gpt-4

# Use GPT-3.5 Turbo (faster, less accurate)
vulnhuntrs -r . --model gpt-3.5-turbo
```

**Anthropic:**
```bash
# Use Claude 3 Sonnet (recommended)
vulnhuntrs -r . --model claude-3-sonnet-20240229

# Use Claude 3 Haiku (faster)
vulnhuntrs -r . --model claude-3-haiku-20240307

# Use Claude 3 Opus (highest accuracy)
vulnhuntrs -r . --model claude-3-opus-20240229
```

**Google:**
```bash
# Use Gemini Pro
vulnhuntrs -r . --model gemini-pro

# Use Gemini Pro Vision (for images)
vulnhuntrs -r . --model gemini-pro-vision
```

### Filtering Options

#### Confidence Levels

Filter results by confidence score (0-10):

```bash
# Only show high-confidence findings
vulnhuntrs -r . --min-confidence 8

# Show medium confidence and above
vulnhuntrs -r . --min-confidence 5

# Show all findings
vulnhuntrs -r . --min-confidence 0
```

#### Vulnerability Types

Focus on specific vulnerability categories:

```bash
# Critical vulnerabilities only
vulnhuntrs -r . --vuln-types RCE,SQLI

# Web application vulnerabilities
vulnhuntrs -r . --vuln-types XSS,CSRF,SSRF

# All supported types
vulnhuntrs -r . --vuln-types RCE,SQLI,XSS,IDOR,LFI,RFI,XXE,SSRF,CSRF
```

### Output Configuration

#### Report Generation

```bash
# Generate detailed reports in directory
vulnhuntrs -r . --output-dir ./security-reports

# Generate summary only
vulnhuntrs -r . --summary

# Combine both
vulnhuntrs -r . --output-dir ./reports --summary
```

#### Verbosity Levels

```bash
# Standard output
vulnhuntrs -r .

# Verbose output
vulnhuntrs -r . -v

# Extra verbose output  
vulnhuntrs -r . -vv
```

## Security Patterns Configuration

### Pattern File Location

Vulnhuntrs uses pattern matching defined in:
```
security_patterns/patterns.yml
```

### Custom Patterns

> **Info**: You can extend or modify security patterns by editing the `patterns.yml` file to focus on specific vulnerability types relevant to your codebase.

Example pattern structure:

```yaml
patterns:
  python:
    - name: "SQL Injection"
      pattern: "execute\\(.*%.*\\)"
      description: "Potential SQL injection via string formatting"
      confidence: 7
      
  javascript:
    - name: "Command Injection"
      pattern: "exec\\(.*\\+.*\\)"
      description: "Command injection via string concatenation"
      confidence: 8
```

## Docker Configuration

### Environment Variables in Docker

```bash
docker run \
  -e OPENAI_API_KEY=$OPENAI_API_KEY \
  -e VULNHUNTRS_MODEL=gpt-4-turbo \
  -v $(pwd):/app \
  ghcr.io/hikaruegashira/vulnhuntrs:latest \
  -r /app --min-confidence 7
```

### Volume Mounts

```bash
# Mount source code and output directory
docker run \
  -e OPENAI_API_KEY=$OPENAI_API_KEY \
  -v $(pwd)/src:/app/src:ro \
  -v $(pwd)/reports:/reports \
  ghcr.io/hikaruegashira/vulnhuntrs:latest \
  -r /app/src --output-dir /reports
```

### User Permissions

```bash
# Run with current user to avoid permission issues
docker run \
  --user $(id -u):$(id -g) \
  -e OPENAI_API_KEY=$OPENAI_API_KEY \
  -v $(pwd):/app \
  ghcr.io/hikaruegashira/vulnhuntrs:latest \
  -r /app
```

## Performance Tuning

### Large Codebases

For large projects, consider these optimizations:

```bash
# Exclude common directories
vulnhuntrs -r . --exclude-dirs "node_modules,target,vendor,.git"

# Include only specific file patterns
vulnhuntrs -r . --include-patterns "*.py,*.js,*.go,*.rs"

# Limit analysis to critical paths
vulnhuntrs -r ./src --vuln-types RCE,SQLI --min-confidence 7
```

### Memory Management

```bash
# For Docker with memory limits
docker run --memory=4g \
  -e OPENAI_API_KEY=$OPENAI_API_KEY \
  ghcr.io/hikaruegashira/vulnhuntrs:latest \
  -r /app
```

## Best Practices

> **Success**: **Recommended Configuration** for most projects:
> - Model: `gpt-4-turbo` or `claude-3-sonnet-20240229`
> - Minimum confidence: `7` for production, `5` for development
> - Output: Generate both detailed reports and summary

> **Warning**: **Cost Optimization**: Higher-end models (GPT-4, Claude Opus) provide better accuracy but cost more. Start with mid-tier models and upgrade if needed.

## Configuration Files

### Project-Level Configuration

Create a `.vulnhuntrs.yml` file in your project root:

```yaml
# .vulnhuntrs.yml
model: "gpt-4-turbo"
min_confidence: 7
vuln_types: ["RCE", "SQLI", "XSS"]
exclude_dirs: ["node_modules", "target", "vendor"]
include_patterns: ["*.py", "*.js", "*.go", "*.rs"]
output_dir: "./security-reports"
summary: true
```

Then run without additional flags:

```bash
vulnhuntrs -r .
```

## Next Steps

- [Learn advanced usage patterns](/docs/usage)
- [Explore real-world examples](/docs/examples)  
- [Understand the analysis architecture](/docs/architecture)