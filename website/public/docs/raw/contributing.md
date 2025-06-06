# Contributing

We welcome contributions to Vulnhuntrs! This guide will help you get started with contributing to the project.

## Ways to Contribute

### 🐛 Bug Reports
- Report security vulnerabilities responsibly
- Submit detailed bug reports with reproduction steps
- Suggest improvements to existing functionality

### ✨ Feature Requests  
- Propose new language support
- Suggest new vulnerability detection patterns
- Request CLI improvements and new options

### 🔧 Code Contributions
- Implement new features
- Fix bugs and improve performance
- Add tests and improve documentation
- Enhance security pattern detection

### 📚 Documentation
- Improve existing documentation
- Add usage examples and tutorials
- Translate documentation to other languages

## Getting Started

### Step 1: Fork and Clone

```bash
# Fork the repository on GitHub
# Then clone your fork
git clone https://github.com/yourusername/vulnhuntrs.git
cd vulnhuntrs
```

### Step 2: Set Up Development Environment

**Rust Development:**
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install development dependencies
cargo install cargo-insta
cargo install cargo-watch

# Build the project
cargo build

# Run tests
cargo test
```

**Documentation:**
```bash
# Navigate to website directory
cd website

# Install dependencies
npm install
# or
bun install

# Start development server
npm run dev
# or  
bun dev
```

### Step 3: Create Feature Branch

```bash
# Create and switch to a new branch
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/issue-description
```

## Development Workflow

### Building and Testing

```bash
# Build in development mode
cargo build

# Build for release
cargo build --release

# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Watch for changes during development
cargo watch -x test
```

### Snapshot Testing

Vulnhuntrs uses snapshot testing for consistent output validation:

```bash
# Run snapshot tests
cargo test --features snapshot-test

# Update snapshots when output changes
cargo insta test
cargo insta review

# Accept all pending snapshots
cargo insta accept
```

> **Warning**: **Snapshot Updates**: Only update snapshots when you've intentionally changed the output format. Review changes carefully before accepting.

### Testing with Examples

```bash
# Test with built-in vulnerable examples
cargo run -- -r example/python-vulnerable-app/
cargo run -- -r example/go-vulnerable-app/
cargo run -- -r example/rust-vulnerable-app/

# Test specific functionality
cargo run -- -r example/python-vulnerable-app/ --min-confidence 8
cargo run -- --repo PentesterLab/cr-go --summary
```

## Code Guidelines

### Rust Code Style

Follow standard Rust conventions:

```bash
# Format code
cargo fmt

# Check for common mistakes
cargo clippy

# Check for unused dependencies
cargo machete
```

### Code Structure

- **Keep functions focused**: Each function should have a single responsibility
- **Use descriptive names**: Variable and function names should be self-documenting
- **Add comprehensive tests**: Include unit tests and integration tests
- **Handle errors gracefully**: Use proper error handling with `Result` types
- **Document public APIs**: Add doc comments for public functions and types

### Example Code Pattern

```rust
/// Analyzes a source file for security vulnerabilities
/// 
/// # Arguments
/// * `file` - The source file to analyze
/// * `context` - Additional code context for analysis
/// 
/// # Returns
/// * `Result<AnalysisResult>` - Analysis results or error
pub async fn analyze_file(
    &self, 
    file: &SourceFile, 
    context: &CodeContext
) -> Result<AnalysisResult, AnalysisError> {
    // Validate inputs
    if file.content.is_empty() {
        return Err(AnalysisError::EmptyFile(file.path.clone()));
    }
    
    // Perform analysis
    let result = self.perform_analysis(file, context).await?;
    
    // Validate results
    self.validate_result(&result)?;
    
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_analyze_file_success() {
        let analyzer = create_test_analyzer();
        let file = create_test_file("test.py", "print('hello')");
        let context = CodeContext::empty();
        
        let result = analyzer.analyze_file(&file, &context).await;
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_analyze_empty_file() {
        let analyzer = create_test_analyzer();
        let file = create_test_file("empty.py", "");
        let context = CodeContext::empty();
        
        let result = analyzer.analyze_file(&file, &context).await;
        assert!(matches!(result.err(), Some(AnalysisError::EmptyFile(_))));
    }
}
```

## Adding Language Support

To add support for a new programming language:

### Step 1: Add Tree-sitter Parser

```toml
# Add to Cargo.toml
[build-dependencies]
tree-sitter-newlang = "0.x.x"
```

```rust
// Add to build.rs
cc::Build::new()
    .include("tree-sitter-newlang/src")
    .file("tree-sitter-newlang/src/parser.c")
    .compile("tree-sitter-newlang");
```

### Step 2: Create Custom Queries

Create `src/queries/newlang/definitions.scm`:
```scheme
; Extract function definitions
(function_declaration
  name: (identifier) @function.name
  body: (block) @function.body)
```

Create `src/queries/newlang/references.scm`:
```scheme
; Find function calls
(call_expression
  function: (identifier) @function.call)
```

### Step 3: Add Security Patterns

Add to `security_patterns/patterns.yml`:
```yaml
newlang:
  - name: "Command Injection"
    pattern: "system\\(|exec\\("
    description: "Potential command injection"
    confidence: 7
```

### Step 4: Update Code

```rust
// Add to SupportedLanguage enum in lib.rs
#[derive(Debug, Clone, Copy)]
pub enum SupportedLanguage {
    // ... existing languages
    NewLang,
}

// Add parser configuration
impl SupportedLanguage {
    pub fn parser(&self) -> Result<tree_sitter::Parser> {
        let mut parser = tree_sitter::Parser::new();
        match self {
            // ... existing cases
            SupportedLanguage::NewLang => {
                parser.set_language(tree_sitter_newlang::language())?;
            }
        }
        Ok(parser)
    }
}
```

See [docs/ADD_NEW_LANGUAGE.md](/docs/add-new-language) for detailed instructions.

## Documentation Contributions

### Website Development

The documentation website uses [Fumadocs](https://fumadocs.dev/):

```bash
cd website

# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Export static site
npm run export
```

### Content Guidelines

- **Clear and concise**: Write in simple, direct language
- **Include examples**: Provide code examples for all features
- **Use proper formatting**: Follow Markdown and MDX conventions
- **Test examples**: Ensure all code examples work correctly
- **Cross-reference**: Link related sections appropriately

### MDX Components

Use Fumadocs components for enhanced documentation:

```mdx
import { Callout } from 'fumadocs-ui/components/callout'
import { Steps, Step } from 'fumadocs-ui/components/steps'
import { Tabs, Tab } from 'fumadocs-ui/components/tabs'

<Callout type="info">
  Important information for users
</Callout>

<Steps>
<Step>
### Step 1
Content for step 1
</Step>
</Steps>

<Tabs items={['Option 1', 'Option 2']}>
  <Tab value="Option 1">
    Content for option 1
  </Tab>
</Tabs>
```

## Security Considerations

### Responsible Disclosure

When contributing security-related code:

- **Review security patterns carefully**: Ensure patterns don't create false positives
- **Test with known vulnerabilities**: Use CVE databases and known vulnerable code
- **Document security implications**: Explain potential security impacts
- **Follow secure coding practices**: Avoid introducing vulnerabilities

### API Key Security

- **Never commit API keys**: Use environment variables only
- **Test with dummy keys**: Use placeholder keys in examples
- **Clear sensitive data**: Ensure API keys aren't logged or cached

## Pull Request Process

### Step 1: Prepare Your Changes

```bash
# Ensure your branch is up to date
git checkout main
git pull upstream main
git checkout your-feature-branch
git rebase main

# Run all checks
cargo fmt
cargo clippy
cargo test
```

### Step 2: Create Pull Request

1. Push your branch to your fork
2. Create PR on GitHub with descriptive title
3. Fill out the PR template completely
4. Link related issues if applicable

### Step 3: Address Review Feedback

- Respond to review comments promptly
- Make requested changes in new commits
- Update tests and documentation as needed
- Squash commits before final merge if requested

### PR Requirements

✅ **All tests pass**  
✅ **Code is formatted** (`cargo fmt`)  
✅ **No clippy warnings** (`cargo clippy`)  
✅ **Documentation updated** (if applicable)  
✅ **Tests added** for new functionality  
✅ **Snapshots updated** (if output changed)  

## Release Process

Vulnhuntrs follows semantic versioning:

- **Patch versions** (0.x.1): Bug fixes and minor improvements
- **Minor versions** (0.x.0): New features and language support  
- **Major versions** (1.0.0): Breaking changes and major features

## Community and Support

### Getting Help

- **GitHub Discussions**: Ask questions and share ideas
- **GitHub Issues**: Report bugs and request features
- **Documentation**: Check existing docs before asking questions

### Code of Conduct

We follow the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Please be respectful and inclusive in all interactions.

## Recognition

Contributors are recognized in:
- Release notes for significant contributions
- GitHub contributors list
- Documentation acknowledgments

Thank you for contributing to Vulnhuntrs! 🚀

## Next Steps

- [Read the architecture overview](/docs/architecture)
- [View API reference](/docs/api)  
- [Explore usage examples](/docs/examples)
