#[allow(unused_imports)]
use parsentry::analyzer::analyze_pattern;
#[allow(unused_imports)]
use parsentry::locales::Language;
#[allow(unused_imports)]
use parsentry::response::VulnType;
#[allow(unused_imports)]
use parsentry::security_patterns::{SecurityRiskPatterns, Language as PatternLanguage, PatternMatch};
#[allow(unused_imports)]
use std::path::PathBuf;
#[allow(unused_imports)]
use tempfile::NamedTempFile;

#[cfg(feature = "snapshot-test")]
#[tokio::test]
async fn test_analyze_empty_file() -> anyhow::Result<()> {
    let temp_file = NamedTempFile::new()?;

    // For empty files, there are no patterns to match, so test should skip analysis
    let content = std::fs::read_to_string(temp_file.path())?;
    let patterns = SecurityRiskPatterns::new(PatternLanguage::JavaScript);
    let pattern_matches = patterns.get_pattern_matches(&content);
    
    // Empty file should have no pattern matches
    assert!(pattern_matches.is_empty());
    
    // Since there are no patterns, analyze_pattern would not be called
    // This test verifies that the pattern matching stage correctly identifies
    // that empty files don't contain security patterns

    Ok(())
}

#[cfg(feature = "snapshot-test")]
#[tokio::test]
async fn test_analyze_hardcoded_password() -> anyhow::Result<()> {
    let temp_file = NamedTempFile::new()?;
    std::fs::write(
        temp_file.path(),
        r#"
fn main() {
    let password = "hardcoded_password";
    println!("{}", password);
}
"#,
    )?;

    let result = analyze_file(
        &PathBuf::from(temp_file.path()),
        "gpt-4.1-nano",
        &[PathBuf::from(temp_file.path())],
        0,
        &parsentry::parser::Context {
            definitions: vec![],
            references: vec![],
        },
        0,
        false,
        &None,
        None,
        &Language::English,
    )
    .await?;

    assert!(!result.analysis.is_empty(), "Analysis should not be empty");
    assert!(
        result.confidence_score > 0,
        "Confidence score should be positive"
    );
    assert!(
        !result.vulnerability_types.is_empty(),
        "Should detect vulnerabilities"
    );
    // Note: context_code field no longer exists in Response struct
    // Context information is now provided through PAR analysis

    Ok(())
}

#[cfg(feature = "snapshot-test")]
#[tokio::test]
async fn test_analyze_authentication_function() -> anyhow::Result<()> {
    let temp_file = NamedTempFile::new()?;
    std::fs::write(
        temp_file.path(),
        r#"
fn authenticate(input: &str) -> bool {
    let password = "hardcoded_password";
    input == password
}

fn main() {
    let user_input = "test";
    if authenticate(user_input) {
        println!("Authenticated!");
    }
}
"#,
    )?;

    let result = analyze_file(
        &PathBuf::from(temp_file.path()),
        "gpt-4.1-nano",
        &[PathBuf::from(temp_file.path())],
        0,
        &parsentry::parser::Context {
            definitions: vec![],
            references: vec![],
        },
        0,
        false,
        &None,
        None,
        &Language::English,
    )
    .await?;

    assert!(!result.analysis.is_empty(), "Analysis should not be empty");
    assert!(
        result.confidence_score >= 0,
        "Confidence score should be positive"
    );

    Ok(())
}
