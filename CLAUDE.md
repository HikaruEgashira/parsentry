## Project Overview

Parsentry is a PAR (Principal-Action-Resource) based security scanner that combines static code analysis with LLMs to detect vulnerabilities across multiple languages including IaC. It provides comprehensive multi-language security analysis.

### Building and Testing
```bash
# Build the project
cargo build --release

# Run all tests
cargo test

# Run tests with snapshot testing
cargo test --features snapshot-test

# Run benchmarks
cargo test --features benchmark

# Update test snapshots
cargo insta test
cargo insta review

# Run with verbose output
cargo test -- --nocapture
```

### Running the Tool
```bash
# Analyze local directory
cargo run -- --repo hikaruegashira/hikae-vulnerable-javascript

# Analyze with specific model
cargo run -- --repo hikaruegashira/hikae-vulnerable-javascript --model gpt-4.1-nano

# Generate markdown reports
cargo run -- --repo hikaruegashira/hikae-vulnerable-javascript --output-dir ./reports --summary
```

### Architecture Overview

The codebase follows a pipeline architecture:

1. **File Discovery** (`repo.rs`): Identifies source files to analyze
2. **Pattern Matching** (`security_patterns.rs`): Filters files using regex patterns from `security_patterns/src/patterns.yml`
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

### LLM Configuration

- Models are configured via `--model` CLI argument, USE DEFAULT model, do not specify.
- API keys are read from environment variables (e.g., `OPENAI_API_KEY`)
- The tool uses the `genai` crate for LLM abstraction
- Prompts are templated in `src/prompts/` directory

### Testing Strategy

- Example vulnerable apps in `example/` directory serve as integration tests
- Snapshot tests using `insta` crate for response consistency
- Unit tests for individual components in `tests/` directory

### Performance Validation

Performance validation is conducted using standardized vulnerability benchmarks:

```bash
# Clone validation benchmarks (104 total benchmarks)
git clone git@github.com:xbow-engineering/validation-benchmarks.git benchmarks

# Run individual benchmark validation
cargo run -- --root benchmarks/XBEN-001-24 --output-dir docs/benchmark/results/XBEN-001-24 --generate-patterns

# Validate specific vulnerability types
cargo run -- --root benchmarks/XBEN-010-24 --output-dir docs/benchmark/results/XBEN-010-24 --generate-patterns
cargo run -- --root benchmarks/XBEN-020-24 --output-dir docs/benchmark/results/XBEN-020-24 --generate-patterns
```

Benchmark validation uses `benchmarks/XBEN-XXX-24/benchmark.json`

### Development Workflow

- create branch pr with auto-merge
- after sleep, check ci / comment and fix
- after merge, rebase main branch and run one-ope-mcp/complete_task

## Behavior guideline

top levelのディレクトリに新たなファイルやディレクトリを作成しないでください、gitのバージョン管理を利用してください
ユーザーとは必ずsayコマンドを使って結果を要約して応答してください
