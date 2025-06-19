# Configuration Guide

This guide provides comprehensive instructions for configuring Parsentry's environment variables and settings.

## Overview

Parsentry uses environment variables for configuration to ensure secure handling of API keys and flexible deployment options. Configuration can be provided via:

- Environment variables (recommended for production)
- `.env` files (convenient for development)
- Command line arguments (for some options)

## Environment Variables Reference

### Required Configuration

#### `OPENAI_API_KEY`
- **Purpose**: OpenAI API key for LLM-powered vulnerability analysis
- **Required**: Yes (unless using alternative LLM provider)
- **Format**: `sk-...` (starts with `sk-`)
- **Obtain from**: https://platform.openai.com/api-keys
- **Scope**: Full API access for chat completions
- **Security**: Store securely, never commit to version control

```bash
export OPENAI_API_KEY="sk-your-openai-api-key-here"
```

### Optional Authentication

#### `GITHUB_TOKEN`
- **Purpose**: GitHub personal access token for private repository access
- **Required**: Only for private repositories
- **Format**: `ghp_...` (classic token) or `github_pat_...` (fine-grained token)
- **Obtain from**: https://github.com/settings/tokens
- **Scope Required**: 
  - For classic tokens: `repo` (full repository access)
  - For fine-grained tokens: `Contents: Read` and `Metadata: Read`
- **Security**: Use minimal required permissions

```bash
export GITHUB_TOKEN="ghp_your-github-token-here"
```

**Error without token when accessing private repos:**
```
Failed to clone repository: authentication required for private repository
```

#### `ANTHROPIC_API_KEY`
- **Purpose**: Anthropic Claude API key (alternative to OpenAI)
- **Required**: No (alternative LLM provider)
- **Format**: `sk-ant-...`
- **Obtain from**: https://console.anthropic.com/
- **Usage**: Set this instead of `OPENAI_API_KEY` to use Claude

```bash
export ANTHROPIC_API_KEY="sk-ant-your-anthropic-key-here"
```

#### `OPENROUTER_API_KEY`
- **Purpose**: OpenRouter API key (alternative to OpenAI)
- **Required**: No (alternative LLM provider)
- **Format**: `sk-or-...`
- **Obtain from**: https://openrouter.ai/keys
- **Usage**: Provides access to multiple LLM providers through one API

```bash
export OPENROUTER_API_KEY="sk-or-your-openrouter-key-here"
```

### API Configuration

#### `API_BASE_URL`
- **Purpose**: Custom API endpoint base URL
- **Required**: No (defaults to OpenAI endpoints)
- **Format**: Full URL to API endpoint
- **Usage**: Use with `PARSENTRY_DISABLE_V1_PATH` for non-OpenAI compatible APIs
- **Examples**:
  - Groq: `https://api.groq.com/openai/v1/chat/completions`
  - Local LLM: `http://localhost:8000/v1/chat/completions`
  - Azure OpenAI: `https://your-resource.openai.azure.com/openai/deployments/your-deployment`

```bash
export API_BASE_URL="https://api.groq.com/openai/v1/chat/completions"
```

#### `PARSENTRY_DISABLE_V1_PATH`
- **Purpose**: Disable automatic `/v1/chat/completions` path appending
- **Required**: When using non-OpenAI compatible endpoints
- **Format**: Any value (presence enables the feature)
- **Usage**: Set when `API_BASE_URL` contains the complete endpoint path
- **Technical**: Changes adapter from OpenAI to Groq internally

```bash
export PARSENTRY_DISABLE_V1_PATH="1"
# or
export PARSENTRY_DISABLE_V1_PATH="true"
# or any value - presence enables the feature
```

## Configuration Examples

### Default OpenAI Setup
```bash
export OPENAI_API_KEY="sk-your-openai-key"
```

### Private Repository Access
```bash
export OPENAI_API_KEY="sk-your-openai-key"
export GITHUB_TOKEN="ghp_your-github-token"
```

### Custom API Endpoint (Groq)
```bash
export API_BASE_URL="https://api.groq.com/openai/v1/chat/completions"
export PARSENTRY_DISABLE_V1_PATH="1"
# Note: Still need an API key for Groq
export OPENAI_API_KEY="your-groq-api-key"
```

### Alternative LLM Provider (Anthropic)
```bash
export ANTHROPIC_API_KEY="sk-ant-your-anthropic-key"
# No need for OPENAI_API_KEY when using Anthropic
```

### Local LLM Server
```bash
export API_BASE_URL="http://localhost:8000/v1/chat/completions"
export PARSENTRY_DISABLE_V1_PATH="1"
export OPENAI_API_KEY="not-needed-for-local" # Still required but can be dummy value
```

### Azure OpenAI
```bash
export API_BASE_URL="https://your-resource.openai.azure.com/openai/deployments/your-deployment/chat/completions?api-version=2024-02-15-preview"
export PARSENTRY_DISABLE_V1_PATH="1"
export OPENAI_API_KEY="your-azure-api-key"
```

## Environment File (.env)

For local development, create a `.env` file in the project root:

```env
# Required - choose one LLM provider
OPENAI_API_KEY=sk-your-openai-key

# Optional - for private repositories
GITHUB_TOKEN=ghp_your-github-token

# Alternative LLM providers (use instead of OpenAI)
# ANTHROPIC_API_KEY=sk-ant-your-anthropic-key
# OPENROUTER_API_KEY=sk-or-your-openrouter-key

# Custom API endpoints
# API_BASE_URL=https://api.groq.com/openai/v1/chat/completions
# PARSENTRY_DISABLE_V1_PATH=1
```

**Security Note**: Never commit `.env` files to version control. Add `.env` to your `.gitignore`.

## Docker Configuration

### Basic Usage
```bash
docker run -e OPENAI_API_KEY=$OPENAI_API_KEY \
  -v $(pwd)/reports:/reports \
  ghcr.io/hikaruegashira/parsentry:latest \
  --repo owner/repository --output-dir /reports
```

### Multiple Environment Variables
```bash
docker run \
  -e OPENAI_API_KEY=$OPENAI_API_KEY \
  -e GITHUB_TOKEN=$GITHUB_TOKEN \
  -e API_BASE_URL=$API_BASE_URL \
  -e PARSENTRY_DISABLE_V1_PATH=$PARSENTRY_DISABLE_V1_PATH \
  -v $(pwd)/reports:/reports \
  ghcr.io/hikaruegashira/parsentry:latest \
  --repo owner/private-repository --output-dir /reports
```

### Using Environment File with Docker
```bash
docker run --env-file .env \
  -v $(pwd)/reports:/reports \
  ghcr.io/hikaruegashira/parsentry:latest \
  --repo owner/repository --output-dir /reports
```

## Troubleshooting

### Common Configuration Issues

#### "Failed to clone repository: authentication required"
- **Cause**: Missing or invalid `GITHUB_TOKEN` for private repository
- **Solution**: Set valid GitHub token with appropriate permissions
- **Check**: Verify token has `repo` scope for private repositories

#### "API key authentication failed"
- **Cause**: Missing or invalid API key for selected LLM provider
- **Solution**: Verify API key format and validity
- **Check**: Test API key with provider's documentation

#### "Custom endpoint not responding"
- **Cause**: Incorrect `API_BASE_URL` or missing `PARSENTRY_DISABLE_V1_PATH`
- **Solution**: Verify endpoint URL and set `PARSENTRY_DISABLE_V1_PATH=1`
- **Check**: Test endpoint accessibility with curl

#### "Environment variable not found"
- **Cause**: Typo in environment variable name or not set
- **Solution**: Check spelling and verify variable is exported
- **Check**: Use `echo $VARIABLE_NAME` to verify

### Debugging Configuration

Check if environment variables are set:
```bash
# Check if variables are set
echo "OPENAI_API_KEY is set: ${OPENAI_API_KEY:+yes}"
echo "GITHUB_TOKEN is set: ${GITHUB_TOKEN:+yes}"
echo "API_BASE_URL is set: ${API_BASE_URL:+yes}"
echo "PARSENTRY_DISABLE_V1_PATH is set: ${PARSENTRY_DISABLE_V1_PATH:+yes}"
```

Test API connectivity:
```bash
# Test OpenAI API
curl -H "Authorization: Bearer $OPENAI_API_KEY" \
  https://api.openai.com/v1/models

# Test custom endpoint
curl -H "Authorization: Bearer $OPENAI_API_KEY" \
  "$API_BASE_URL"
```

## Security Best Practices

### API Key Management
1. **Never commit API keys** to version control
2. **Use environment variables** instead of hardcoding
3. **Rotate keys regularly** as recommended by providers
4. **Use minimal required permissions** for tokens
5. **Store securely** in production environments

### Production Deployment
1. **Use container secrets** for API keys in containerized environments
2. **Set up key rotation** procedures
3. **Monitor API usage** for unexpected activity
4. **Use separate keys** for different environments (dev/staging/prod)

### Development Environment
1. **Use `.env` files** for local development (never commit)
2. **Test with minimal permissions** first
3. **Document required configuration** for team members
4. **Use example files** (`.env.example`) without real keys

## Environment-Specific Configuration

### Development
```bash
# Local development with file analysis
export OPENAI_API_KEY="sk-dev-key"
# No GITHUB_TOKEN needed for local files
```

### CI/CD
```bash
# Continuous integration environment
export OPENAI_API_KEY="${{ secrets.OPENAI_API_KEY }}"
export GITHUB_TOKEN="${{ secrets.GITHUB_TOKEN }}"
```

### Production
```bash
# Production deployment
export OPENAI_API_KEY="$(cat /var/secrets/openai-key)"
export GITHUB_TOKEN="$(cat /var/secrets/github-token)"
```

## Advanced Configuration

### Custom Model Selection
While not environment variables, models can be specified via CLI:
```bash
# Use specific model
cargo run -- --repo owner/repo --model gpt-4o-mini

# With environment configuration
export OPENAI_API_KEY="sk-key"
cargo run -- --repo owner/repo --model claude-3-haiku-20240307
```

### Multiple API Keys for Different Models
```bash
# Set multiple provider keys
export OPENAI_API_KEY="sk-openai-key"
export ANTHROPIC_API_KEY="sk-ant-key"
export OPENROUTER_API_KEY="sk-or-key"

# Tool will select appropriate key based on model choice
```

### Custom Headers and Authentication
For advanced API configurations, the tool currently supports:
- Bearer token authentication (most common)
- Custom base URLs with path control
- Standard HTTP headers as supported by the `genai` crate

## Support and Further Help

For additional configuration help:
1. Check the [main README](../README.md) for usage examples
2. Review [command line options](../CLAUDE.md) documentation
3. Open an issue on GitHub for configuration problems
4. Check provider documentation for API key setup

## Configuration Validation

Currently, Parsentry performs runtime validation of configuration. Future versions will include:
- Startup configuration validation
- Helpful error messages with documentation links
- Configuration testing utilities
- Environment variable schema validation