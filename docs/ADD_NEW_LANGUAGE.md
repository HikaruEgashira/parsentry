# Adding New Language Support

This guide explains how to add support for a new programming language to Parsentry.

## Prerequisites

- Familiarity with the target language's syntax and common vulnerability patterns
- Basic understanding of tree-sitter and its query language
- Rust development environment set up

## Architecture Overview

Language support spans three crates:

| Component | Crate | File |
|-----------|-------|------|
| Language enum | `parsentry-core` | `crates/parsentry-core/src/language.rs` |
| Tree-sitter queries | `parsentry-parser` | `crates/parsentry-parser/src/queries/<lang>/` |
| Security patterns | `parsentry-parser` | `crates/parsentry-parser/src/patterns/<lang>.yml` |
| Parser wiring | `parsentry-parser` | `crates/parsentry-parser/src/parser.rs` |
| Pattern wiring | `parsentry-parser` | `crates/parsentry-parser/src/patterns.rs` |
| Cargo dependency | `parsentry-parser` | `crates/parsentry-parser/Cargo.toml` |

## Steps to Add a New Language

### 1. Add Tree-sitter Grammar Dependency

In `crates/parsentry-parser/Cargo.toml`, add the tree-sitter grammar:

```toml
# crates/parsentry-parser/Cargo.toml
[dependencies]
tree-sitter-<lang> = "0.23"  # Use appropriate version from crates.io
```

No git submodules needed — grammars are pulled as Cargo dependencies.

### 2. Update Language Enum

In `crates/parsentry-core/src/language.rs`:

#### 2.1 Add variant to the enum

```rust
pub enum Language {
    Python,
    JavaScript,
    // ... existing variants
    YourNewLanguage,  // Add this
}
```

#### 2.2 Update `from_extension`

```rust
pub fn from_extension(ext: &str) -> Self {
    match ext {
        "py" => Language::Python,
        // ... existing mappings
        "ext" => Language::YourNewLanguage,  // Add file extension mapping
        _ => Language::Other,
    }
}
```

#### 2.3 Update `display_name`, `FromStr`, and `is_iac`

Update all remaining match statements to include the new variant. The compiler will catch any missed arms.

### 3. Add Tree-sitter Language Mapping

#### 3.1 In `crates/parsentry-parser/src/parser.rs`

Update `get_language` to map the new extension to a tree-sitter grammar:

```rust
pub fn get_language(&self, path: &Path) -> Option<Language> {
    let extension = path.extension().and_then(|ext| ext.to_str());
    match extension {
        // ... existing mappings
        Some("ext") => Some(tree_sitter_your_language::LANGUAGE.into()),
        _ => None,
    }
}
```

Update `language_to_name` to add the reverse mapping:

```rust
fn language_to_name(language: &Language) -> Option<&'static str> {
    // ... existing comparisons
    if language == &ts_your_language { Some("yourlanguage") } else { None }
}
```

Update `get_query_content` to add query file includes:

```rust
let query_content = match (lang_name, query_name) {
    // ... existing entries
    ("yourlanguage", "definitions") => include_str!("queries/yourlanguage/definitions.scm"),
    ("yourlanguage", "calls") => include_str!("queries/yourlanguage/calls.scm"),
    // ...
};
```

#### 3.2 In `crates/parsentry-parser/src/patterns.rs`

Update `get_tree_sitter_language`:

```rust
fn get_tree_sitter_language(language: Language) -> TreeSitterLanguage {
    match language {
        // ... existing mappings
        Language::YourNewLanguage => tree_sitter_your_language::LANGUAGE.into(),
        _ => tree_sitter_javascript::LANGUAGE.into(),
    }
}
```

Update `load_patterns` to include the YAML:

```rust
let languages = [
    // ... existing entries
    (YourNewLanguage, include_str!("patterns/yourlanguage.yml")),
];
```

### 4. Create Tree-sitter Queries

```bash
mkdir -p crates/parsentry-parser/src/queries/yourlanguage
```

Create two query files:

#### `definitions.scm` — Identifies function/method/class definitions

Example for Python:
```scheme
(function_definition
  name: (identifier) @name) @definition

(class_definition
  name: (identifier) @name) @definition
```

#### `calls.scm` — Tracks function calls and references

Example for Python:
```scheme
; Direct function calls
(call
  function: (identifier) @direct_call)

; Method calls
(call
  function: (attribute
    attribute: (identifier) @method_call))

; Import statements
(import_statement
  name: (dotted_name) @import)
```

Use `@name` and `@definition` capture names for definitions queries. Use `@direct_call`, `@method_call`, `@reference`, `@callback`, `@import`, `@assignment` for calls queries.

### 5. Add Security Patterns

Create `crates/parsentry-parser/src/patterns/yourlanguage.yml`:

```yaml
principals:
  - definition: |
      (your_request_handler_node) @function
    description: "HTTP request handlers"
    attack_vector:
      - "T1190"

actions:
  - reference: |
      (your_sanitization_function) @call
    description: "Input sanitization"
    attack_vector:
      - "T1070"

resources:
  - reference: |
      (your_dangerous_function) @call
    description: "Dangerous function calls"
    attack_vector:
      - "T1059"
```

Pattern structure:
- **principals**: Data entry points (request handlers, file reads, env vars)
- **actions**: Security controls (sanitization, validation, encryption)
- **resources**: Sensitive operations (DB queries, command execution, file writes)

Each pattern has:
- `definition` or `reference`: Tree-sitter query string
- `description`: Human-readable description
- `attack_vector`: MITRE ATT&CK technique IDs

### 6. Add Tests

Add language-specific tests alongside existing tests:

```rust
#[test]
fn test_your_language_parsing() {
    let mut parser = CodeParser::new().unwrap();
    let content = r#"
    // Your language code sample
    "#;
    // Write to temp file and test parsing
}
```

### 7. Build and Test

```bash
cargo build
cargo test
cargo run -- model . | head  # Verify language detection in metadata
```

## Implementation Checklist

- [ ] `tree-sitter-<lang>` dependency added to `crates/parsentry-parser/Cargo.toml`
- [ ] Language enum updated in `crates/parsentry-core/src/language.rs`
- [ ] `from_extension`, `display_name`, `FromStr` updated
- [ ] Tree-sitter language mapping added in `crates/parsentry-parser/src/parser.rs`
- [ ] `get_query_content` updated with `include_str!` for new query files
- [ ] Tree-sitter language mapping added in `crates/parsentry-parser/src/patterns.rs`
- [ ] `load_patterns` updated with `include_str!` for new YAML
- [ ] Query files created: `queries/<lang>/definitions.scm`, `queries/<lang>/calls.scm`
- [ ] Security patterns added: `patterns/<lang>.yml`
- [ ] Tests added and passing
- [ ] `cargo check` passes

## Tips

1. Study existing language implementations — Python and JavaScript have the most comprehensive patterns
2. Use [tree-sitter playground](https://tree-sitter.github.io/tree-sitter/playground) to develop queries interactively
3. Start with basic `definitions.scm` and `calls.scm`, then add security patterns
4. The `Other` language variant acts as a fallback — no grammar needed for basic support
5. Run `cargo test -p parsentry-core` to verify Language enum changes compile

## Common Pitfalls

- Forgetting to update all match statements (the compiler catches most of these)
- Missing `language_to_name` entry in parser.rs — queries won't load
- Missing `include_str!` entry — patterns or queries silently unavailable
- Tree-sitter query syntax errors — use the playground to validate before committing
- Not adding the YAML entry in `load_patterns` — patterns won't be loaded
