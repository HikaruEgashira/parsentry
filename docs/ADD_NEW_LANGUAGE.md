# Adding New Language Support

This guide explains how to add support for a new programming language to Parsentry.

## Prerequisites

- Familiarity with the target language's syntax and common vulnerability patterns
- Basic understanding of tree-sitter and its query language
- Rust development environment set up

## Steps to Add a New Language

### 1. Add Tree-sitter Grammar

First, add the tree-sitter grammar for your target language:

```bash
# Add as a git submodule
git submodule add https://github.com/tree-sitter/tree-sitter-<lang> tree-sitter-<lang>

# Navigate to the submodule and build
cd tree-sitter-<lang>
npm install  # or cargo build, depending on the grammar
```

### 2. Create Custom Queries

Create query files for semantic analysis:

```bash
mkdir -p src/queries/<lang>
```

Create two files:
- `src/queries/<lang>/definitions.scm` - For identifying function/method definitions
- `src/queries/<lang>/references.scm` - For tracking variable references and data flow

Example for Python:
```scheme
; definitions.scm
(function_definition
  name: (identifier) @function.name
  parameters: (parameters) @function.params
  body: (block) @function.body)

(class_definition
  name: (identifier) @class.name
  body: (block) @class.body)
```

### 3. Update Rust Code

#### 3.1 Add Cargo Dependency

In `Cargo.toml`, add the tree-sitter language:

```toml
[dependencies]
tree-sitter-<lang> = { path = "./tree-sitter-<lang>" }
```

#### 3.2 Update Language Enum

In `src/security_patterns.rs`, add your language to the enum:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Ruby,
    Go,
    Java,
    YourNewLanguage, // Add this
}
```

Update the `from_extension` method:

```rust
impl Language {
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "rs" => Some(Language::Rust),
            "py" => Some(Language::Python),
            // ... other languages
            "ext" => Some(Language::YourNewLanguage), // Add this
            _ => None,
        }
    }
}
```

#### 3.3 Update Parser Module

In `src/parser.rs`, add language support:

```rust
fn get_language(language: Language) -> tree_sitter::Language {
    match language {
        Language::Rust => tree_sitter_rust::language(),
        Language::Python => tree_sitter_python::language(),
        // ... other languages
        Language::YourNewLanguage => tree_sitter_your_language::language(),
    }
}

fn get_query_path(language: Language, query_type: &str) -> PathBuf {
    match language {
        // ... other cases
        Language::YourNewLanguage => {
            PathBuf::from(format!("src/queries/yourlang/{}.scm", query_type))
        }
    }
}
```

### 4. Add Security Patterns

Update `security_patterns/src/patterns.yml` with language-specific vulnerability patterns:

```yaml
languages:
  yourlanguage:
    file_patterns:
      - pattern: "dangerous_function"
        risk_score: 8
        category: "code_execution"
        description: "Potential code execution vulnerability"
    # Add more patterns...
```

### 5. Add Tests

Create test files and add test cases:

```bash
mkdir -p example/<lang>-vulnerable-app
```

Add a test in `tests/analyzer_test.rs`:

```rust
#[test]
fn test_your_language_analysis() {
    let content = r#"
    // Your vulnerable code sample
    "#;
    
    let result = analyze_code(content, Language::YourNewLanguage);
    assert!(result.vulnerabilities.len() > 0);
}
```

### 6. Build and Test

```bash
# Build the project
cargo build

# Run tests
cargo test

# Test with a sample file
parsentry -r ./example/<lang>-vulnerable-app --model gpt-4o-mini
```

## Implementation Checklist

- [ ] Tree-sitter grammar added as submodule
- [ ] Custom queries created (definitions.scm, references.scm)
- [ ] Cargo.toml updated with dependency
- [ ] Language enum updated in security_patterns.rs
- [ ] Parser module updated with language support
- [ ] Security patterns added to src/patterns.yml
- [ ] Example vulnerable code created
- [ ] Tests added and passing
- [ ] Documentation updated

## Tips

1. Study existing language implementations in the codebase
2. Start with basic patterns and gradually add more complex ones
3. Test with real-world vulnerable code samples
4. Consider language-specific vulnerability types
5. Ensure queries are efficient for large codebases

## Common Pitfalls

- Forgetting to update all switch/match statements for the new language
- Not testing with various code styles and edge cases
- Missing important vulnerability patterns specific to the language
- Incorrect tree-sitter query syntax

## Need Help?

If you encounter issues:
1. Check existing language implementations for reference
2. Consult tree-sitter documentation for query syntax
3. Open an issue on GitHub with details about your implementation
