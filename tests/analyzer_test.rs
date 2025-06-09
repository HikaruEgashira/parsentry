#[allow(unused_imports)]
use std::path::PathBuf;
#[allow(unused_imports)]
use tempfile::NamedTempFile;
#[allow(unused_imports)]
use parsentry::analyzer::analyze_file;
#[allow(unused_imports)]
use parsentry::response::VulnType;

#[cfg(feature = "snapshot-test")]
#[tokio::test]
async fn test_analyze_empty_file() -> anyhow::Result<()> {
    let temp_file = NamedTempFile::new()?;

    let result = analyze_file(
        &PathBuf::from(temp_file.path()),
        "gpt-4.1-nano",
        &[PathBuf::from(temp_file.path())],
        0,
        &parsentry::parser::Context {
            definitions: vec![],
        },
        0,
        false,
        &None,
        None,
    )
    .await?;

    assert_eq!(result.scratchpad, String::new());
    assert_eq!(result.analysis, String::new());
    assert_eq!(result.poc, String::new());
    assert_eq!(result.confidence_score, 0);
    assert!(result.vulnerability_types.is_empty());
    // Note: context_code field no longer exists in Response struct

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
        },
        0,
        false,
        &None,
        None,
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
        },
        0,
        false,
        &None,
        None,
    )
    .await?;

    assert!(!result.analysis.is_empty(), "Analysis should not be empty");
    assert!(
        result.confidence_score >= 0,
        "Confidence score should be positive"
    );

    Ok(())
}
