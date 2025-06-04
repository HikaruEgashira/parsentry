# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Vulnhuntrs is an AI-powered security vulnerability scanner that combines static code analysis with LLMs to detect remotely exploitable vulnerabilities. It supports Rust, Python, JavaScript, TypeScript, Go, Java, and Ruby.

## Key Commands

### Building and Testing
```bash
# Build the project
cargo build --release

# Run all tests
cargo test

# Run tests with snapshot testing
cargo test --features snapshot-test

# Update test snapshots
cargo insta test
cargo insta review

# Run with verbose output
cargo test -- --nocapture
```

### Running the Tool
```bash
# Analyze local directory
cargo run -- -r /path/to/project

# Analyze with specific model
cargo run -- -r /path/to/project --model gpt-4.1-nano

# Generate markdown reports
cargo run -- -r /path/to/project --output-dir ./reports --summary

# Filter by confidence and vulnerability types
cargo run -- -r /path/to/project --min-confidence 7 --vuln-types RCE,SQLI
```

### Docker Operations
```bash
# Build multi-arch image
docker buildx build --platform linux/amd64,linux/arm64 -t ghcr.io/hikaruegashira/vulnhuntrs:latest --push .

# Run with Docker
docker run ghcr.io/hikaruegashira/vulnhuntrs:latest --repo https://github.com/PentesterLab/cr-go

docker run -e OPENAI_API_KEY=$OPENAI_API_KEY \
  -v $(pwd)/reports:/reports \
  --user $(id -u):$(id -g) \
  ghcr.io/hikaruegashira/vulnhuntrs:latest \
  --repo https://github.com/PentesterLab/cr-go --output-dir /reports --summary
```

## Architecture Overview

The codebase follows a pipeline architecture:

1. **File Discovery** (`repo.rs`): Identifies source files to analyze
2. **Pattern Matching** (`security_patterns.rs`): Filters files using regex patterns from `security_patterns/patterns.yml`
3. **Code Parsing** (`parser.rs`): Uses tree-sitter to parse code and extract semantic information
4. **Context Building** (`parser.rs`): Collects function definitions and references for context
5. **LLM Analysis** (`analyzer.rs`): Sends code + context to LLM for vulnerability detection
6. **Response Handling** (`response.rs`): Formats and validates LLM responses

### Key Modules

- `analyzer.rs`: Core vulnerability analysis logic, handles LLM interactions
- `parser.rs`: Tree-sitter integration, extracts code context (definitions/references)
- `security_patterns.rs`: Pattern-based file filtering using YAML configurations
- `prompts/`: LLM prompt templates for analysis and evaluation
- `repo_clone.rs`: GitHub repository cloning functionality

### Adding Language Support

To add a new language, follow `docs/ADD_NEW_LANGUAGE.md`:
1. Add tree-sitter parser as dependency
2. Update `build.rs` to compile the parser
3. Add language to `SupportedLanguage` enum
4. Create custom queries in `custom_queries/<language>/`
5. Update pattern matching in `security_patterns/patterns.yml`

### LLM Configuration

- Models are configured via `--model` CLI argument
- API keys are read from environment variables (e.g., `OPENAI_API_KEY`)
- The tool uses the `genai` crate for LLM abstraction
- Prompts are templated in `src/prompts/` directory

### Testing Strategy

- Example vulnerable apps in `example/` directory serve as integration tests
- Snapshot tests using `insta` crate for response consistency
- Unit tests for individual components in `tests/` directory
