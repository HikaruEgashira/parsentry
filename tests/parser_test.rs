use anyhow::Result;
use parsentry::parser::{CodeParser, Context, Definition};
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_code_parser_new() -> Result<()> {
    let parser = CodeParser::new()?;
    assert!(parser.files.is_empty());
    Ok(())
}

#[test]
fn test_add_file_success() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("test.rs");
    fs::write(&file_path, "fn main() {}")?;

    let mut parser = CodeParser::new()?;
    parser.add_file(&file_path)?;

    assert_eq!(parser.files.len(), 1);
    assert!(parser.files.contains_key(&file_path));
    assert_eq!(parser.files.get(&file_path).unwrap(), "fn main() {}");
    Ok(())
}

#[test]
fn test_add_file_not_found() -> Result<()> {
    let mut parser = CodeParser::new()?;
    let non_existent_path = PathBuf::from("/non/existent/file.rs");

    let result = parser.add_file(&non_existent_path);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("ファイルの読み込みに失敗しました")
    );
    Ok(())
}

#[test]
fn test_get_language_supported_extensions() {
    let parser = CodeParser::new().unwrap();
    let test_cases = vec![
        ("test.rs", true, "rust"),
        ("test.c", true, "c"),
        ("test.cpp", true, "cpp"),
        ("test.cxx", true, "cpp"),
        ("test.cc", true, "cpp"),
        ("test.hpp", true, "cpp"),
        ("test.hxx", true, "cpp"),
        ("test.py", true, "python"),
        ("test.js", true, "javascript"),
        ("test.ts", true, "typescript"),
        ("test.tsx", true, "typescript"),
        ("test.java", true, "java"),
        ("test.go", true, "go"),
        ("test.rb", true, "ruby"),
        ("test.tf", true, "terraform"),
        ("test.hcl", true, "terraform"),
        ("test.php", true, "php"),
        ("test.php3", true, "php"),
        ("test.php4", true, "php"),
        ("test.php5", true, "php"),
        ("test.phtml", true, "php"),
        ("test.txt", false, "unsupported"),
        ("test.unknown", false, "unsupported"),
    ];

    for (filename, should_be_supported, description) in test_cases {
        let path = PathBuf::from(filename);
        let language = parser.get_language(&path);
        assert_eq!(
            language.is_some(),
            should_be_supported,
            "Language support test failed for {} ({})",
            filename,
            description
        );
    }
}

#[test]
fn test_get_query_content_through_file_operations() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("test.rs");
    fs::write(&file_path, "fn test_function() {}")?;

    let mut parser = CodeParser::new()?;
    parser.add_file(&file_path)?;

    let result = parser.find_definition("test_function", &file_path)?;
    assert!(
        result.is_some(),
        "Should find definition through query system"
    );
    Ok(())
}

#[test]
fn test_find_definition_simple_rust() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("test.rs");
    fs::write(
        &file_path,
        "fn hello_world() { println!(\"Hello, world!\"); }",
    )?;

    let mut parser = CodeParser::new()?;
    parser.add_file(&file_path)?;

    let result = parser.find_definition("hello_world", &file_path)?;
    assert!(result.is_some());

    let (found_path, definition) = result.unwrap();
    assert_eq!(found_path, file_path);
    assert_eq!(definition.name, "hello_world");
    assert!(
        !definition.source.is_empty(),
        "Definition source should not be empty"
    );
    assert!(
        definition.source.contains("fn hello_world"),
        "Source should contain function definition"
    );
    Ok(())
}

#[test]
fn test_find_definition_not_found() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("test.rs");
    fs::write(
        &file_path,
        "fn hello_world() { println!(\"Hello, world!\"); }",
    )?;

    let mut parser = CodeParser::new()?;
    parser.add_file(&file_path)?;

    let result = parser.find_definition("non_existent_function", &file_path)?;
    assert!(result.is_none());
    Ok(())
}

#[test]
fn test_find_definition_file_not_in_parser() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("test.rs");
    fs::write(&file_path, "fn hello_world() {}")?;

    let mut parser = CodeParser::new()?;

    let result = parser.find_definition("hello_world", &file_path);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("パーサーにファイルが見つかりません")
    );
    Ok(())
}

#[test]
fn test_find_references_simple_rust() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("test.rs");
    fs::write(
        &file_path,
        "fn hello_world() { println!(\"Hello, world!\"); }\nfn main() { hello_world(); hello_world(); }",
    )?;

    let mut parser = CodeParser::new()?;
    parser.add_file(&file_path)?;

    let results = parser.find_references("hello_world")?;
    assert!(!results.is_empty());

    let references: Vec<_> = results
        .iter()
        .filter(|(_, def)| def.name == "hello_world")
        .collect();
    assert!(!references.is_empty());
    Ok(())
}

#[test]
fn test_find_references_no_matches() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("test.rs");
    fs::write(
        &file_path,
        "fn hello_world() { println!(\"Hello, world!\"); }",
    )?;

    let mut parser = CodeParser::new()?;
    parser.add_file(&file_path)?;

    let results = parser.find_references("non_existent_function")?;
    assert!(results.is_empty());
    Ok(())
}

#[test]
fn test_find_bidirectional() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("test.rs");
    fs::write(
        &file_path,
        "fn hello_world() { println!(\"Hello, world!\"); }\nfn main() { hello_world(); }",
    )?;

    let mut parser = CodeParser::new()?;
    parser.add_file(&file_path)?;

    let results = parser.find_bidirectional("hello_world", &file_path)?;
    assert!(!results.is_empty(), "Should find some results");

    let has_definition = results
        .iter()
        .any(|(_, def)| def.source.contains("fn") || def.source.contains("{"));
    let has_reference = results.iter().any(|(_, def)| def.name == "hello_world");

    assert!(
        has_definition || has_reference,
        "Should find either definition or reference"
    );
    Ok(())
}

#[test]
fn test_build_context_from_file_rust() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("test.rs");
    fs::write(
        &file_path,
        "fn helper() { println!(\"helper\"); }\nfn main() { helper(); }",
    )?;

    let mut parser = CodeParser::new()?;
    parser.add_file(&file_path)?;

    let context = parser.build_context_from_file(&file_path)?;

    assert!(!context.definitions.is_empty(), "Should find definitions");
    assert!(!context.references.is_empty(), "Should find references");

    let helper_defs: Vec<_> = context
        .definitions
        .iter()
        .filter(|def| def.name == "helper")
        .collect();
    assert!(
        !helper_defs.is_empty(),
        "Should find helper function definition"
    );

    Ok(())
}

#[test]
fn test_build_context_from_file_unsupported_language() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("test.txt");
    fs::write(&file_path, "some text content")?;

    let mut parser = CodeParser::new()?;
    parser.add_file(&file_path)?;

    let context = parser.build_context_from_file(&file_path)?;

    assert!(context.definitions.is_empty());
    assert!(context.references.is_empty());
    Ok(())
}

#[test]
fn test_build_context_from_file_missing_file() -> Result<()> {
    let mut parser = CodeParser::new()?;
    let non_existent_path = PathBuf::from("/non/existent/file.rs");

    let result = parser.build_context_from_file(&non_existent_path);
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("ファイルが見つかりません")
    );
    Ok(())
}

#[test]
fn test_context_creation_and_validation() -> Result<()> {
    let def1 = Definition {
        name: "func1".to_string(),
        start_byte: 0,
        end_byte: 13,
        source: "fn func1() {}".to_string(),
    };

    let def2 = Definition {
        name: "func2".to_string(),
        start_byte: 20,
        end_byte: 33,
        source: "fn func2() {}".to_string(),
    };

    let context = Context {
        definitions: vec![def1.clone()],
        references: vec![def2.clone()],
    };

    assert_eq!(context.definitions.len(), 1);
    assert_eq!(context.references.len(), 1);
    assert_eq!(context.definitions[0].name, "func1");
    assert_eq!(context.definitions[0].source, "fn func1() {}");
    assert_eq!(context.references[0].name, "func2");
    assert_eq!(context.references[0].source, "fn func2() {}");
    
    // Validate byte positions
    assert!(context.definitions[0].start_byte < context.definitions[0].end_byte);
    assert!(context.references[0].start_byte < context.references[0].end_byte);
    Ok(())
}

#[test]
fn test_multiple_files_references() -> Result<()> {
    let temp_dir = TempDir::new()?;

    let file1_path = temp_dir.path().join("lib.rs");
    fs::write(
        &file1_path,
        "pub fn shared_function() { println!(\"shared\"); }",
    )?;

    let file2_path = temp_dir.path().join("main.rs");
    fs::write(&file2_path, "fn main() { shared_function(); }")?;

    let mut parser = CodeParser::new()?;
    parser.add_file(&file1_path)?;
    parser.add_file(&file2_path)?;

    let references = parser.find_references("shared_function")?;

    assert!(!references.is_empty());
    Ok(())
}

#[test]
fn test_find_definition_with_malformed_code() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("malformed.rs");
    fs::write(&file_path, "fn incomplete_function(")?;

    let mut parser = CodeParser::new()?;
    parser.add_file(&file_path)?;

    // Should handle malformed code gracefully without panicking
    let result = parser.find_definition("incomplete_function", &file_path);
    // Either succeeds or fails with an error, but should not panic
    assert!(result.is_ok() || result.is_err(), "Parser should handle malformed code gracefully");
    Ok(())
}

#[test]
fn test_find_references_with_empty_file() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("empty.rs");
    fs::write(&file_path, "")?;

    let mut parser = CodeParser::new()?;
    parser.add_file(&file_path)?;

    let results = parser.find_references("any_function")?;
    assert!(results.is_empty());
    Ok(())
}

#[test]
fn test_parser_with_large_function_name() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("test.rs");
    let long_name = "a".repeat(1000);
    let code = format!("fn {}() {{}}", long_name);
    fs::write(&file_path, code)?;

    let mut parser = CodeParser::new()?;
    parser.add_file(&file_path)?;

    let result = parser.find_definition(&long_name, &file_path)?;
    if let Some((_, definition)) = result {
        assert_eq!(definition.name, long_name);
    }
    Ok(())
}

#[test]
fn test_build_context_with_recursive_functions() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("recursive.rs");
    fs::write(
        &file_path,
        "fn factorial(n: u32) -> u32 { if n <= 1 { 1 } else { n * factorial(n - 1) } }",
    )?;

    let mut parser = CodeParser::new()?;
    parser.add_file(&file_path)?;

    let context = parser.build_context_from_file(&file_path)?;

    assert!(!context.definitions.is_empty());
    let factorial_defs: Vec<_> = context
        .definitions
        .iter()
        .filter(|def| def.name == "factorial")
        .collect();
    assert!(!factorial_defs.is_empty());
    Ok(())
}

#[test]
fn test_parser_with_unicode_in_code() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let file_path = temp_dir.path().join("unicode.rs");
    fs::write(&file_path, "fn こんにちは() { println!(\"世界\"); }")?;

    let mut parser = CodeParser::new()?;
    parser.add_file(&file_path)?;

    let result = parser.find_definition("こんにちは", &file_path)?;
    assert!(result.is_some());

    let (_, definition) = result.unwrap();
    assert_eq!(definition.name, "こんにちは");
    assert!(definition.source.contains("こんにちは"));
    Ok(())
}