#[cfg(test)]
mod tests {
    use parsentry::security_patterns::{SecurityRiskPatterns, Language};
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_vuln_patterns_loading() {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Create a test vuln-patterns.yml file
        let test_content = r#"JavaScript:
  principals:
    - pattern: "test_custom_pattern"
      description: "Test custom pattern for JavaScript"
      attack_vector: ["T1190"]
"#;
        
        let vuln_patterns_path = temp_path.join("vuln-patterns.yml");
        fs::write(&vuln_patterns_path, test_content).expect("Failed to write test file");
        
        // Test loading patterns with root directory
        let patterns = SecurityRiskPatterns::new_with_root(Language::JavaScript, Some(temp_path));
        
        // Test that the pattern matches
        let test_code = "function test_custom_pattern() { }";
        assert!(patterns.matches(test_code), "Custom pattern from vuln-patterns.yml should match");
        
        // Test that a non-matching pattern doesn't match
        let non_matching_code = "function other_function() { }";
        assert!(!patterns.matches(non_matching_code), "Non-matching code should not match custom pattern");
    }

    #[test]
    fn test_vuln_patterns_merge_with_existing() {
        // Create a temporary directory for testing
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        // Create a test vuln-patterns.yml file that adds to existing JavaScript patterns
        let test_content = r#"JavaScript:
  principals:
    - pattern: "custom_input_handler"
      description: "Custom input handler pattern"
      attack_vector: ["T1190"]
"#;
        
        let vuln_patterns_path = temp_path.join("vuln-patterns.yml");
        fs::write(&vuln_patterns_path, test_content).expect("Failed to write test file");
        
        // Test loading patterns with root directory
        let patterns = SecurityRiskPatterns::new_with_root(Language::JavaScript, Some(temp_path));
        
        // Test that both custom and built-in patterns work
        let custom_code = "function custom_input_handler() { }";
        assert!(patterns.matches(custom_code), "Custom pattern should match");
        
        // Test a built-in pattern still works (assuming fetch is in built-in patterns)
        let builtin_code = "fetch('http://example.com')";
        assert!(patterns.matches(builtin_code), "Built-in patterns should still work");
    }

    #[test]
    fn test_no_vuln_patterns_file() {
        // Create a temporary directory without vuln-patterns.yml
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();
        
        // This should work without errors, just using built-in patterns
        let patterns = SecurityRiskPatterns::new_with_root(Language::JavaScript, Some(temp_path));
        
        // Test that built-in patterns still work
        let builtin_code = "fetch('http://example.com')";
        assert!(patterns.matches(builtin_code), "Built-in patterns should work when no vuln-patterns.yml exists");
    }
}