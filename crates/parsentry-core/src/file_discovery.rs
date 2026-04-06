use anyhow::Result;
use std::path::{Path, PathBuf};

/// Common file discovery functionality for traversing directories
/// and finding files with specific extensions.
pub struct FileDiscovery {
    root_path: PathBuf,
    supported_extensions: Vec<String>,
}

impl FileDiscovery {
    /// Default supported extensions for security analysis
    const DEFAULT_EXTENSIONS: &'static [&'static str] = &[
        "py", "js", "jsx", "ts", "tsx", "rs", "go", "java", "rb", "c", "h", "cpp", "cxx", "cc",
        "hpp", "hxx", "tf", "hcl", "yml", "yaml", "sh", "bash", "php", "php3", "php4", "php5",
        "phtml",
    ];

    /// Create a new FileDiscovery with default extensions
    pub fn new(root_path: PathBuf) -> Self {
        Self {
            root_path,
            supported_extensions: Self::DEFAULT_EXTENSIONS
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }

    /// Create a new FileDiscovery with custom extensions
    pub fn with_extensions(root_path: PathBuf, extensions: Vec<String>) -> Self {
        Self {
            root_path,
            supported_extensions: extensions,
        }
    }

    /// Get the root path
    pub fn root_path(&self) -> &Path {
        &self.root_path
    }

    /// Get supported extensions
    pub fn supported_extensions(&self) -> &[String] {
        &self.supported_extensions
    }

    /// Check if an extension is supported
    pub fn is_supported_extension(&self, ext: &str) -> bool {
        self.supported_extensions.contains(&ext.to_lowercase())
    }

    /// Get all files matching supported extensions in the root directory
    pub fn get_files(&self) -> Result<Vec<PathBuf>> {
        self.get_files_in_path(&self.root_path)
    }

    /// Get all files matching supported extensions in a specific path
    pub fn get_files_in_path(&self, path: &Path) -> Result<Vec<PathBuf>> {
        if path.is_file() {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if self.supported_extensions.contains(&ext_str) {
                    return Ok(vec![path.to_path_buf()]);
                }
            }
            return Ok(vec![]);
        }

        if !path.is_dir() {
            anyhow::bail!("Path does not exist: {}", path.display());
        }

        let mut files = Vec::new();
        self.visit_dirs(path, &mut |p: &Path| {
            if let Some(ext) = p.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if self.supported_extensions.contains(&ext_str) {
                    files.push(p.to_path_buf());
                }
            }
        })?;

        Ok(files)
    }

    /// Recursively visit directories and call callback for each file
    pub fn visit_dirs<F>(&self, dir: &Path, cb: &mut F) -> std::io::Result<()>
    where
        F: FnMut(&Path),
    {
        if dir.is_dir() {
            for entry in std::fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    self.visit_dirs(&path, cb)?;
                } else {
                    cb(&path);
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use tempfile::TempDir;

    #[test]
    fn test_new_creates_with_default_extensions() {
        let discovery = FileDiscovery::new(PathBuf::from("/tmp"));
        assert!(discovery.is_supported_extension("py"));
        assert!(discovery.is_supported_extension("js"));
        assert!(discovery.is_supported_extension("rs"));
        assert!(!discovery.is_supported_extension("txt"));
    }

    #[test]
    fn test_with_extensions_uses_custom() {
        let discovery =
            FileDiscovery::with_extensions(PathBuf::from("/tmp"), vec!["txt".to_string()]);
        assert!(discovery.is_supported_extension("txt"));
        assert!(!discovery.is_supported_extension("py"));
    }

    #[test]
    fn test_get_files_returns_matching_files() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        File::create(temp_path.join("test.py")).unwrap();
        File::create(temp_path.join("test.js")).unwrap();
        File::create(temp_path.join("test.txt")).unwrap();

        let discovery = FileDiscovery::new(temp_path.to_path_buf());
        let files = discovery.get_files().unwrap();

        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|f| f.ends_with("test.py")));
        assert!(files.iter().any(|f| f.ends_with("test.js")));
    }

    #[test]
    fn test_get_files_recursive() {
        let temp_dir = TempDir::new().unwrap();
        let temp_path = temp_dir.path();

        fs::create_dir(temp_path.join("subdir")).unwrap();
        File::create(temp_path.join("root.py")).unwrap();
        File::create(temp_path.join("subdir/nested.rs")).unwrap();

        let discovery = FileDiscovery::new(temp_path.to_path_buf());
        let files = discovery.get_files().unwrap();

        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_get_files_single_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("single.py");
        File::create(&file_path).unwrap();

        let discovery = FileDiscovery::new(file_path.clone());
        let files = discovery.get_files_in_path(&file_path).unwrap();

        assert_eq!(files.len(), 1);
        assert_eq!(files[0], file_path);
    }
}
