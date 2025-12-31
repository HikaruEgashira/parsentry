use anyhow::Result;
use std::{
    fs::{File, read_to_string},
    io::{BufRead, BufReader, Result as IoResult},
    path::{Path, PathBuf},
};

use parsentry_parser::SecurityRiskPatterns;
use parsentry_utils::{FileClassifier, FileDiscovery};

#[derive(Default)]
pub struct LanguageExclusions {
    pub file_patterns: Vec<String>,
}

pub struct RepoOps {
    file_discovery: FileDiscovery,
    gitignore_patterns: Vec<String>,
    language_exclusions: LanguageExclusions,
    code_parser: crate::parser::CodeParser,
    parser_initialized: bool,
}

impl RepoOps {
    pub fn new(repo_path: PathBuf) -> Self {
        let gitignore_patterns = Self::read_gitignore(&repo_path).unwrap_or_default();

        let language_exclusions = LanguageExclusions {
            file_patterns: vec!["test_".to_string(), "conftest".to_string()],
        };

        let code_parser = crate::parser::CodeParser::new().unwrap();
        let file_discovery = FileDiscovery::new(repo_path);

        Self {
            file_discovery,
            gitignore_patterns,
            language_exclusions,
            code_parser,
            parser_initialized: false,
        }
    }

    pub fn repo_path(&self) -> &Path {
        self.file_discovery.root_path()
    }

    pub fn collect_context_for_security_pattern(
        &mut self,
        file_path: &std::path::Path,
    ) -> anyhow::Result<crate::parser::Context> {
        self.code_parser.build_context_from_file(file_path)
    }

    fn read_gitignore(repo_path: &Path) -> IoResult<Vec<String>> {
        let gitignore_path = repo_path.join(".gitignore");
        if !gitignore_path.exists() {
            return Ok(Vec::new());
        }

        let file = File::open(gitignore_path)?;
        let reader = BufReader::new(file);
        let mut patterns = Vec::new();

        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                patterns.push(trimmed.to_string());
            };
        }

        Ok(patterns)
    }

    fn should_exclude_path(&self, path: &Path) -> bool {
        if let Ok(relative_path) = path.strip_prefix(self.repo_path()) {
            let relative_str = relative_path.to_string_lossy();

            for pattern in &self.gitignore_patterns {
                if Self::matches_gitignore_pattern(&relative_str, pattern) {
                    return true;
                }
            }

            if let Some(file_name) = path.file_name() {
                let file_name = file_name.to_string_lossy().to_lowercase();
                if self
                    .language_exclusions
                    .file_patterns
                    .iter()
                    .any(|pattern| file_name.contains(pattern))
                {
                    return true;
                }
            }
        }
        false
    }

    /// Determine if a path matches a .gitignore style pattern.
    ///
    /// The function is public so that integration tests can verify the
    /// behaviour of pattern matching.
    pub fn matches_gitignore_pattern(path: &str, pattern: &str) -> bool {
        let pattern = pattern.trim_start_matches('/');
        let path = path.trim_start_matches('/');

        if let Some(stripped) = pattern.strip_prefix('*') {
            path.ends_with(stripped)
        } else if let Some(stripped) = pattern.strip_suffix('*') {
            path.starts_with(stripped)
        } else if !pattern.contains('/') {
            if path == pattern {
                true
            } else {
                path.split('/').any(|segment| segment == pattern)
            }
        } else {
            path == pattern || path.starts_with(&format!("{}/", pattern))
        }
    }

    pub fn get_relevant_files(&self) -> Vec<PathBuf> {
        match self.file_discovery.get_files() {
            Ok(files) => files
                .into_iter()
                .filter(|path| !self.should_exclude_path(path))
                .collect(),
            Err(e) => {
                eprintln!("ディレクトリの走査中にエラーが発生しました: {}", e);
                Vec::new()
            }
        }
    }

    pub fn get_network_related_files(&self, files: &[PathBuf]) -> Vec<PathBuf> {
        let mut network_files = Vec::new();
        for file_path in files {
            if let Ok(content) = read_to_string(file_path) {
                // Skip files with more than 50,000 characters
                if content.len() > 50_000 {
                    continue;
                }
                
                let filename = file_path.to_string_lossy();
                let lang = FileClassifier::classify(&filename, &content);
                let patterns = SecurityRiskPatterns::new_with_root(lang, Some(self.repo_path()));
                if patterns.matches(&content) {
                    network_files.push(file_path.clone());
                }
            }
        }

        network_files
    }

    pub fn get_files_to_analyze(&self, analyze_path: Option<PathBuf>) -> Result<Vec<PathBuf>> {
        let path_to_analyze = analyze_path.unwrap_or_else(|| self.repo_path().to_path_buf());
        self.file_discovery.get_files_in_path(&path_to_analyze)
    }

    pub fn parse_repo_files(&mut self, analyze_path: Option<PathBuf>) -> Result<()> {
        let files = self.get_files_to_analyze(analyze_path)?;
        for file in &files {
            self.code_parser.add_file(file)?;
        }
        self.parser_initialized = true;

        Ok(())
    }

    pub fn find_definition_in_repo(
        &mut self,
        name: &str,
        source_file: &Path,
    ) -> anyhow::Result<Option<(PathBuf, crate::parser::Definition)>> {
        if !self.parser_initialized {
            self.parse_repo_files(None)?;
        }

        self.code_parser.find_definition(name, source_file)
    }

    pub fn find_references_in_repo(
        &mut self,
        name: &str,
    ) -> anyhow::Result<Vec<(PathBuf, crate::parser::Definition)>> {
        if !self.parser_initialized {
            self.parse_repo_files(None)?;
        }
        Ok(self.code_parser.find_calls(name)?.into_iter().map(|(path, def, _)| (path, def)).collect())
    }
    pub fn add_file_to_parser(&mut self, path: &std::path::Path) -> anyhow::Result<()> {
        self.code_parser.add_file(path)
    }
}
