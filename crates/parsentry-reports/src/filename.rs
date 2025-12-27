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
pub fn generate_output_filename(
    file_path: &std::path::Path,
    root_dir: &std::path::Path,
) -> String {
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
        .replace('/', "-") // Handle both Unix and Windows separators
        .replace('\\', "-")
        .replace("..", "dotdot") // Remove dangerous path traversal
        .replace(':', "_") // Replace colon (problematic on Windows)
        .replace('*', "_") // Replace wildcard characters
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
    pattern_description: &str,
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
    fn test_generate_pattern_specific_filename_basic() {
        let root = Path::new("/project");
        let file_path = Path::new("/project/routes.py");

        let filename1 = generate_pattern_specific_filename(file_path, root, "SQL Injection");
        let filename2 = generate_pattern_specific_filename(file_path, root, "XSS Vulnerability");

        assert_eq!(filename1, "routes.py-sql-injection.md");
        assert_eq!(filename2, "routes.py-xss-vulnerability.md");
        assert_ne!(filename1, filename2);
    }
}
