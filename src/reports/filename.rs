/// Generate a unique output filename based on the relative path from root directory
/// 
/// This function creates unique filenames by:
/// - Stripping the root directory prefix from the file path
/// - Replacing path separators with hyphens to maintain readability
/// - Removing dangerous path components like ".."
/// - Appending ".md" extension
/// 
/// # Arguments
/// * `file_path` - The full path to the source file
/// * `root_dir` - The root directory to strip from the path
/// 
/// # Returns
/// A unique filename string suitable for filesystem use
pub fn generate_output_filename(file_path: &std::path::Path, root_dir: &std::path::Path) -> String {
    
    // Strip the root directory prefix to get relative path
    let relative_path = match file_path.strip_prefix(root_dir) {
        Ok(rel_path) => rel_path,
        Err(_) => file_path, // Fallback to full path if strip fails
    };
    
    // Convert path to string and replace separators with hyphens
    let path_str = relative_path.to_string_lossy();
    
    // Replace path separators and clean up dangerous characters
    let cleaned = path_str
        .replace(std::path::MAIN_SEPARATOR, "-")
        .replace('/', "-")  // Handle both Unix and Windows separators
        .replace('\\', "-")
        .replace("..", "dotdot")  // Remove dangerous path traversal
        .replace(':', "_")  // Replace colon (problematic on Windows)
        .replace('*', "_")  // Replace wildcard characters
        .replace('?', "_")
        .replace('<', "_")
        .replace('>', "_")
        .replace('|', "_")
        .replace('"', "_");
    
    // Append .md extension
    format!("{}.md", cleaned)
}

pub fn generate_pattern_specific_filename(
    file_path: &std::path::Path, 
    root_dir: &std::path::Path, 
    pattern_description: &str
) -> String {
    // First get the base filename without .md extension
    let base_filename = generate_output_filename(file_path, root_dir);
    let base_without_md = base_filename.trim_end_matches(".md");
    
    // Create a safe pattern identifier from the description
    // First replace various characters with dashes, then filter and clean up
    let pattern_id = pattern_description
        .to_lowercase()
        .replace(" ", "-")
        .replace("_", "-")
        .replace("/", "-")
        .replace("\\", "-")
        .replace("(", "-")
        .replace(")", "-")
        .replace("&", "-")
        .replace(".", "-")
        .replace(",", "-")
        .replace(":", "-")
        .replace(";", "-")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>()
        // Remove consecutive dashes and trim
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>()
        .join("-");
    
    // Ensure pattern_id is not empty
    let pattern_id = if pattern_id.is_empty() {
        "pattern".to_string()
    } else {
        pattern_id
    };
    
    // Combine base filename with pattern identifier
    format!("{}-{}.md", base_without_md, pattern_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_generate_output_filename_uniqueness() {
        let root = Path::new("/project");
        
        // Different paths should generate different names
        let file1 = Path::new("/project/app/routes.py");
        let file2 = Path::new("/project/api/routes.py");
        let file3 = Path::new("/project/utils/routes.py");
        
        let name1 = generate_output_filename(file1, root);
        let name2 = generate_output_filename(file2, root);
        let name3 = generate_output_filename(file3, root);
        
        assert_ne!(name1, name2);
        assert_ne!(name1, name3);
        assert_ne!(name2, name3);
        
        assert_eq!(name1, "app-routes.py.md");
        assert_eq!(name2, "api-routes.py.md");
        assert_eq!(name3, "utils-routes.py.md");
    }
    
    #[test]
    fn test_generate_output_filename_readability() {
        let root = Path::new("/project");
        
        // Path structure should be preserved in readable form
        let file = Path::new("/project/src/components/auth/LoginForm.tsx");
        let result = generate_output_filename(file, root);
        
        assert_eq!(result, "src-components-auth-LoginForm.tsx.md");
        
        // Should maintain file extension information
        assert!(result.contains("LoginForm.tsx"));
        assert!(result.ends_with(".md"));
    }
    
    #[test]
    fn test_generate_output_filename_safety() {
        let root = Path::new("/project");
        
        // Dangerous path traversal should be handled
        let file = Path::new("/project/../admin/config.php");
        let result = generate_output_filename(file, root);
        
        // Should not contain ".." 
        assert!(!result.contains(".."));
        assert!(result.contains("dotdot"));
        
        // Test other dangerous characters
        let file2 = Path::new("/project/file:with*special?chars<>|.py");
        let result2 = generate_output_filename(file2, root);
        
        // Dangerous characters should be replaced with underscores
        assert!(!result2.contains(':'));
        assert!(!result2.contains('*'));
        assert!(!result2.contains('?'));
        assert!(!result2.contains('<'));
        assert!(!result2.contains('>'));
        assert!(!result2.contains('|'));
        assert!(result2.contains('_'));
    }
    
    #[test]
    fn test_generate_output_filename_consistency() {
        let root = Path::new("/project");
        let file = Path::new("/project/app/routes.py");
        
        // Same input should always produce same output
        let result1 = generate_output_filename(file, root);
        let result2 = generate_output_filename(file, root);
        let result3 = generate_output_filename(file, root);
        
        assert_eq!(result1, result2);
        assert_eq!(result2, result3);
        assert_eq!(result1, "app-routes.py.md");
    }
    
    #[test]
    fn test_generate_output_filename_edge_cases() {
        let root = Path::new("/project");
        
        // File directly in root
        let file1 = Path::new("/project/main.rs");
        let result1 = generate_output_filename(file1, root);
        assert_eq!(result1, "main.rs.md");
        
        // Deep nested path
        let file2 = Path::new("/project/very/deep/nested/path/file.js");
        let result2 = generate_output_filename(file2, root);
        assert_eq!(result2, "very-deep-nested-path-file.js.md");
        
        // No extension
        let file3 = Path::new("/project/src/Dockerfile");
        let result3 = generate_output_filename(file3, root);
        assert_eq!(result3, "src-Dockerfile.md");
    }
    
    #[test]
    fn test_generate_output_filename_fallback() {
        // Test when file path can't be stripped from root
        let root = Path::new("/different/root");
        let file = Path::new("/project/app/routes.py");
        
        let result = generate_output_filename(file, root);
        
        // Should still generate a valid filename using the full path
        assert!(result.ends_with(".md"));
        assert!(!result.is_empty());
    }
    
    #[test]
    fn test_filename_collision_resolution() {
        // This test demonstrates that the original problem is solved
        let root = Path::new("/repo");
        
        // These files would have caused collisions with the old implementation
        let file1 = Path::new("/repo/app/routes.py");
        let file2 = Path::new("/repo/api/routes.py");
        let file3 = Path::new("/repo/admin/routes.py");
        let file4 = Path::new("/repo/components/Button.tsx");
        let file5 = Path::new("/repo/pages/Button.tsx");
        
        let results = vec![
            generate_output_filename(file1, root),
            generate_output_filename(file2, root),
            generate_output_filename(file3, root),
            generate_output_filename(file4, root),
            generate_output_filename(file5, root),
        ];
        
        // Verify all results are unique (no collisions)
        for i in 0..results.len() {
            for j in (i + 1)..results.len() {
                assert_ne!(
                    results[i], results[j],
                    "Collision detected between {} and {}",
                    results[i], results[j]
                );
            }
        }
        
        // Verify expected format
        assert_eq!(results[0], "app-routes.py.md");
        assert_eq!(results[1], "api-routes.py.md");
        assert_eq!(results[2], "admin-routes.py.md");
        assert_eq!(results[3], "components-Button.tsx.md");
        assert_eq!(results[4], "pages-Button.tsx.md");
    }
    
    #[test] 
    fn test_old_vs_new_filename_generation() {
        // Demonstrate the difference between old and new implementations
        use std::path::Path;
        
        let root = Path::new("/repo");
        let files = vec![
            Path::new("/repo/app/routes.py"),
            Path::new("/repo/api/routes.py"),
            Path::new("/repo/utils/routes.py"),
        ];
        
        // Old implementation (what used to happen)
        let old_results: Vec<String> = files
            .iter()
            .map(|file_path| {
                file_path
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string() + ".md")
                    .unwrap_or_else(|| "report.md".to_string())
            })
            .collect();
            
        // New implementation 
        let new_results: Vec<String> = files
            .iter()
            .map(|file_path| generate_output_filename(file_path, root))
            .collect();
        
        // Old implementation would create collisions (all same name)
        assert_eq!(old_results[0], "routes.py.md");
        assert_eq!(old_results[1], "routes.py.md");
        assert_eq!(old_results[2], "routes.py.md");
        // All three would overwrite each other!
        
        // New implementation creates unique names
        assert_eq!(new_results[0], "app-routes.py.md");
        assert_eq!(new_results[1], "api-routes.py.md");
        assert_eq!(new_results[2], "utils-routes.py.md");
        
        // Verify uniqueness
        assert_ne!(new_results[0], new_results[1]);
        assert_ne!(new_results[1], new_results[2]);
        assert_ne!(new_results[0], new_results[2]);
    }

    #[test]
    fn test_pattern_overwrite_issue() {
        // This test demonstrates the current issue where multiple patterns 
        // on the same file generate the same filename, causing overwrites
        let root = Path::new("/project");
        let file_path = Path::new("/project/routes.py");
        
        // Simulate multiple patterns analyzing the same file
        let filename1 = generate_output_filename(file_path, root);
        let filename2 = generate_output_filename(file_path, root);
        
        // Currently, both patterns generate the same filename
        // This causes the second analysis to overwrite the first
        assert_eq!(filename1, filename2); // This demonstrates the problem
        assert_eq!(filename1, "routes.py.md");
        
        // This is the bug we need to fix: same file + different patterns = same filename
        // The solution should make filenames unique per pattern
    }

    #[test]
    fn test_generate_pattern_specific_filename_basic() {
        let root = Path::new("/project");
        let file_path = Path::new("/project/routes.py");
        
        let filename1 = generate_pattern_specific_filename(file_path, root, "SQL Injection");
        let filename2 = generate_pattern_specific_filename(file_path, root, "XSS Vulnerability");
        
        assert_eq!(filename1, "routes.py-sql-injection.md");
        assert_eq!(filename2, "routes.py-xss-vulnerability.md");
        assert_ne!(filename1, filename2);
    }

    #[test]
    fn test_generate_pattern_specific_filename_special_chars() {
        let root = Path::new("/project");
        let file_path = Path::new("/project/api/users.py");
        
        // Test pattern descriptions with special characters
        let filename1 = generate_pattern_specific_filename(file_path, root, "IDOR (Insecure Direct Object Reference)");
        let filename2 = generate_pattern_specific_filename(file_path, root, "Command_Injection & RCE");
        let filename3 = generate_pattern_specific_filename(file_path, root, "Path/Directory Traversal");
        
        assert_eq!(filename1, "api-users.py-idor-insecure-direct-object-reference.md");
        assert_eq!(filename2, "api-users.py-command-injection-rce.md");
        assert_eq!(filename3, "api-users.py-path-directory-traversal.md");
    }

    #[test]
    fn test_generate_pattern_specific_filename_empty_pattern() {
        let root = Path::new("/project");
        let file_path = Path::new("/project/app.py");
        
        // Test with empty pattern description
        let filename1 = generate_pattern_specific_filename(file_path, root, "");
        let filename2 = generate_pattern_specific_filename(file_path, root, "   ");
        let filename3 = generate_pattern_specific_filename(file_path, root, "---");
        
        assert_eq!(filename1, "app.py-pattern.md");
        assert_eq!(filename2, "app.py-pattern.md");
        assert_eq!(filename3, "app.py-pattern.md");
    }

    #[test]
    fn test_generate_pattern_specific_filename_consistency() {
        let root = Path::new("/project");
        let file_path = Path::new("/project/controllers/auth.py");
        
        // Same inputs should produce same outputs
        let filename1 = generate_pattern_specific_filename(file_path, root, "Authentication Bypass");
        let filename2 = generate_pattern_specific_filename(file_path, root, "Authentication Bypass");
        
        assert_eq!(filename1, filename2);
        assert_eq!(filename1, "controllers-auth.py-authentication-bypass.md");
    }

    #[test]
    fn test_pattern_specific_fixes_overwrite_issue() {
        let root = Path::new("/project");
        let file_path = Path::new("/project/routes.py");
        
        // Now with pattern-specific filenames, different patterns generate different filenames
        let filename1 = generate_pattern_specific_filename(file_path, root, "SQL Injection");
        let filename2 = generate_pattern_specific_filename(file_path, root, "XSS Vulnerability");
        let filename3 = generate_pattern_specific_filename(file_path, root, "CSRF Token Missing");
        
        // All filenames should be unique
        assert_ne!(filename1, filename2);
        assert_ne!(filename1, filename3);
        assert_ne!(filename2, filename3);
        
        // Verify expected format
        assert_eq!(filename1, "routes.py-sql-injection.md");
        assert_eq!(filename2, "routes.py-xss-vulnerability.md");
        assert_eq!(filename3, "routes.py-csrf-token-missing.md");
    }
}