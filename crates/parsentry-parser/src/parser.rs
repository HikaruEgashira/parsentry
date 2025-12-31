//! Code parser using tree-sitter.

use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use streaming_iterator::StreamingIterator;
use tree_sitter::{Language, Node, Parser, Query, QueryCursor};

/// A code definition (function, class, method, etc.)
#[derive(Debug, Clone)]
pub struct Definition {
    pub name: String,
    pub start_byte: usize,
    pub end_byte: usize,
    pub source: String,
    pub file_path: Option<PathBuf>,
    pub line_number: Option<usize>,
}

/// Context containing definitions and references from parsed code.
#[derive(Debug, Clone)]
pub struct Context {
    pub definitions: Vec<Definition>,
    pub references: Vec<Definition>,
}

/// Tree-sitter based code parser.
pub struct CodeParser {
    pub files: HashMap<PathBuf, String>,
    pub parser: Parser,
}

impl CodeParser {
    /// Create a new code parser.
    pub fn new() -> Result<Self> {
        Ok(Self {
            files: HashMap::new(),
            parser: Parser::new(),
        })
    }

    /// Add a file to the parser.
    pub fn add_file(&mut self, path: &Path) -> Result<()> {
        let content = fs::read_to_string(path).map_err(|e| {
            anyhow!("Failed to read file: {}: {}", path.display(), e)
        })?;
        self.files.insert(path.to_path_buf(), content);
        Ok(())
    }

    /// Get the tree-sitter language for a file based on its extension.
    #[must_use]
    pub fn get_language(&self, path: &Path) -> Option<Language> {
        let extension = path.extension().and_then(|ext| ext.to_str());
        match extension {
            Some("c") | Some("h") => Some(tree_sitter_c::LANGUAGE.into()),
            Some("cpp") | Some("cxx") | Some("cc") | Some("hpp") | Some("hxx") => {
                Some(tree_sitter_cpp::LANGUAGE.into())
            }
            Some("py") => Some(tree_sitter_python::LANGUAGE.into()),
            Some("js") => Some(tree_sitter_javascript::LANGUAGE.into()),
            Some("ts") => Some(tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
            Some("tsx") => Some(tree_sitter_typescript::LANGUAGE_TSX.into()),
            Some("java") => Some(tree_sitter_java::LANGUAGE.into()),
            Some("rs") => Some(tree_sitter_rust::LANGUAGE.into()),
            Some("go") => Some(tree_sitter_go::LANGUAGE.into()),
            Some("rb") => Some(tree_sitter_ruby::LANGUAGE.into()),
            Some("tf") | Some("hcl") => Some(tree_sitter_hcl::LANGUAGE.into()),
            Some("php") | Some("php3") | Some("php4") | Some("php5") | Some("phtml") => {
                Some(tree_sitter_php::LANGUAGE_PHP.into())
            }
            _ => None,
        }
    }

    /// Convert a tree-sitter Language to its string name.
    fn language_to_name(language: &Language) -> Option<&'static str> {
        let ts_c: Language = tree_sitter_c::LANGUAGE.into();
        let ts_cpp: Language = tree_sitter_cpp::LANGUAGE.into();
        let ts_python: Language = tree_sitter_python::LANGUAGE.into();
        let ts_javascript: Language = tree_sitter_javascript::LANGUAGE.into();
        let ts_typescript: Language = tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into();
        let ts_tsx: Language = tree_sitter_typescript::LANGUAGE_TSX.into();
        let ts_java: Language = tree_sitter_java::LANGUAGE.into();
        let ts_go: Language = tree_sitter_go::LANGUAGE.into();
        let ts_rust: Language = tree_sitter_rust::LANGUAGE.into();
        let ts_ruby: Language = tree_sitter_ruby::LANGUAGE.into();
        let ts_hcl: Language = tree_sitter_hcl::LANGUAGE.into();
        let ts_php: Language = tree_sitter_php::LANGUAGE_PHP.into();

        if language == &ts_c {
            Some("c")
        } else if language == &ts_cpp {
            Some("cpp")
        } else if language == &ts_python {
            Some("python")
        } else if language == &ts_javascript {
            Some("javascript")
        } else if language == &ts_typescript || language == &ts_tsx {
            Some("typescript")
        } else if language == &ts_java {
            Some("java")
        } else if language == &ts_go {
            Some("go")
        } else if language == &ts_rust {
            Some("rust")
        } else if language == &ts_ruby {
            Some("ruby")
        } else if language == &ts_hcl {
            Some("terraform")
        } else if language == &ts_php {
            Some("php")
        } else {
            None
        }
    }

    /// Get query content for a specific language and query type.
    pub fn get_query_content(&self, language: &Language, query_name: &str) -> Result<&'static str> {
        let lang_name = Self::language_to_name(language)
            .ok_or_else(|| anyhow!("Unsupported language for queries"))?;

        if query_name.contains('/') || query_name.contains('\\') || query_name.contains("..") {
            return Err(anyhow!("Invalid query name: {}", query_name));
        }

        let query_content = match (lang_name, query_name) {
            ("c", "definitions") => include_str!("queries/c/definitions.scm"),
            ("c", "calls") => include_str!("queries/c/calls.scm"),
            ("cpp", "definitions") => include_str!("queries/cpp/definitions.scm"),
            ("cpp", "calls") => include_str!("queries/cpp/calls.scm"),
            ("python", "definitions") => include_str!("queries/python/definitions.scm"),
            ("python", "calls") => include_str!("queries/python/calls.scm"),
            ("javascript", "definitions") => include_str!("queries/javascript/definitions.scm"),
            ("javascript", "calls") => include_str!("queries/javascript/calls.scm"),
            ("typescript", "definitions") => include_str!("queries/typescript/definitions.scm"),
            ("typescript", "calls") => include_str!("queries/typescript/calls.scm"),
            ("java", "definitions") => include_str!("queries/java/definitions.scm"),
            ("java", "calls") => include_str!("queries/java/calls.scm"),
            ("go", "definitions") => include_str!("queries/go/definitions.scm"),
            ("go", "calls") => include_str!("queries/go/calls.scm"),
            ("rust", "definitions") => include_str!("queries/rust/definitions.scm"),
            ("rust", "calls") => include_str!("queries/rust/calls.scm"),
            ("ruby", "definitions") => include_str!("queries/ruby/definitions.scm"),
            ("ruby", "calls") => include_str!("queries/ruby/calls.scm"),
            ("terraform", "definitions") => include_str!("queries/terraform/definitions.scm"),
            ("terraform", "calls") => include_str!("queries/terraform/calls.scm"),
            ("php", "definitions") => include_str!("queries/php/definitions.scm"),
            ("php", "calls") => include_str!("queries/php/calls.scm"),
            (_, query) => return Err(anyhow!("Unsupported query: {} for {}", query, lang_name)),
        };

        Ok(query_content)
    }

    /// Find a definition by name in a specific file.
    pub fn find_definition(
        &mut self,
        name: &str,
        source_file: &Path,
    ) -> Result<Option<(PathBuf, Definition)>> {
        let content = self.files.get(source_file).ok_or_else(|| {
            anyhow!("File not found in parser: {}", source_file.display())
        })?;

        let language = match self.get_language(source_file) {
            Some(lang) => lang,
            None => return Ok(None),
        };

        self.parser
            .set_language(&language)
            .map_err(|e| anyhow!("Failed to set language: {}", e))?;

        let tree = self
            .parser
            .parse(content, None)
            .ok_or_else(|| anyhow!("Failed to parse file: {}", source_file.display()))?;

        let query_str = self.get_query_content(&language, "definitions")?;

        let query = Query::new(&language, query_str)
            .map_err(|e| anyhow!("Failed to create query: {}", e))?;

        let mut query_cursor = QueryCursor::new();
        let mut matches = query_cursor.matches(&query, tree.root_node(), content.as_bytes());

        while let Some(mat) = matches.next() {
            let mut definition_node: Option<Node> = None;
            let mut name_node: Option<Node> = None;

            for cap in mat.captures {
                let capture_name = &query.capture_names()[cap.index as usize];
                match capture_name {
                    s if *s == "definition" => definition_node = Some(cap.node),
                    s if *s == "name" => name_node = Some(cap.node),
                    _ => {}
                }
            }

            if let (Some(def_node), Some(name_node_inner)) = (definition_node, name_node)
                && name_node_inner.utf8_text(content.as_bytes())? == name
            {
                let start_byte = def_node.start_byte();
                let end_byte = def_node.end_byte();
                let source = def_node.utf8_text(content.as_bytes())?.to_string();

                let line_number = content[..start_byte].matches('\n').count() + 1;
                let definition = Definition {
                    name: name.to_string(),
                    start_byte,
                    end_byte,
                    source,
                    file_path: Some(source_file.to_path_buf()),
                    line_number: Some(line_number),
                };
                return Ok(Some((source_file.to_path_buf(), definition)));
            }
        }

        Ok(None)
    }

    /// Find all calls to a function/method by name across all loaded files.
    pub fn find_calls(&mut self, name: &str) -> Result<Vec<(PathBuf, Definition, String)>> {
        let mut results = Vec::new();

        for (file_path, content) in &self.files {
            let language = match self.get_language(file_path) {
                Some(lang) => lang,
                None => continue,
            };

            self.parser.set_language(&language).map_err(|e| {
                anyhow!("Failed to set language for {}: {}", file_path.display(), e)
            })?;

            let tree = match self.parser.parse(content, None) {
                Some(t) => t,
                None => {
                    eprintln!("Warning: Failed to parse file: {}", file_path.display());
                    continue;
                }
            };

            let query_str = match self.get_query_content(&language, "calls") {
                Ok(s) => s,
                Err(e) => {
                    eprintln!(
                        "Warning: Failed to get calls query for {}: {}",
                        file_path.display(),
                        e
                    );
                    continue;
                }
            };

            let query = match Query::new(&language, query_str) {
                Ok(q) => q,
                Err(e) => {
                    eprintln!("Warning: Failed to create calls query: {}", e);
                    continue;
                }
            };

            let mut query_cursor = QueryCursor::new();
            let mut matches = query_cursor.matches(&query, tree.root_node(), content.as_bytes());

            while let Some(mat) = matches.next() {
                for cap in mat.captures {
                    let capture_name = query.capture_names()[cap.index as usize];
                    let valid_captures = [
                        "direct_call",
                        "method_call",
                        "macro_call",
                        "reference",
                        "callback",
                        "import",
                        "assignment",
                    ];

                    if valid_captures.contains(&capture_name) {
                        let node = cap.node;
                        if node.utf8_text(content.as_bytes())? == name {
                            let start_byte = node.start_byte();
                            let end_byte = node.end_byte();
                            let source = name.to_string();
                            let line_number = content[..start_byte].matches('\n').count() + 1;

                            results.push((
                                file_path.clone(),
                                Definition {
                                    name: name.to_string(),
                                    start_byte,
                                    end_byte,
                                    source,
                                    file_path: Some(file_path.clone()),
                                    line_number: Some(line_number),
                                },
                                capture_name.to_string(),
                            ));
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    /// Find both definitions and references for bidirectional tracking.
    pub fn find_bidirectional(
        &mut self,
        name: &str,
        source_file: &Path,
    ) -> Result<Vec<(PathBuf, Definition)>> {
        let mut results = Vec::new();

        if let Some(definition) = self.find_definition(name, source_file)? {
            results.push(definition);
        }

        let calls = self.find_calls(name)?;
        results.extend(calls.into_iter().map(|(path, def, _)| (path, def)));

        results.sort_by_key(|(path, def)| (path.clone(), def.start_byte));
        results.dedup_by_key(|(path, def)| (path.clone(), def.start_byte));

        Ok(results)
    }

    /// Build context (definitions and references) from a file.
    pub fn build_context_from_file(&mut self, start_path: &Path) -> Result<Context> {
        use std::collections::HashSet;

        let mut collected: HashSet<String> = HashSet::new();
        let mut definitions: Vec<Definition> = Vec::new();
        let mut references: Vec<Definition> = Vec::new();

        let file_content = self
            .files
            .get(start_path)
            .ok_or_else(|| anyhow!("File not found: {}", start_path.display()))?;

        let language = match self.get_language(start_path) {
            Some(lang) => lang,
            None => {
                return Ok(Context {
                    definitions: Vec::new(),
                    references: Vec::new(),
                });
            }
        };

        self.parser
            .set_language(&language)
            .map_err(|e| anyhow!("Failed to set language: {}", e))?;

        let tree = self
            .parser
            .parse(file_content, None)
            .ok_or_else(|| anyhow!("Failed to parse: {}", start_path.display()))?;

        let definitions_query_str = self.get_query_content(&language, "definitions")?;
        let definitions_query = Query::new(&language, definitions_query_str)?;

        let mut query_cursor = QueryCursor::new();
        let mut matches = query_cursor.matches(
            &definitions_query,
            tree.root_node(),
            file_content.as_bytes(),
        );

        let mut to_visit: Vec<(PathBuf, String)> = Vec::new();

        while let Some(mat) = matches.next() {
            let mut def_node: Option<Node> = None;
            let mut name_node: Option<Node> = None;
            for cap in mat.captures {
                let capture_name = &definitions_query.capture_names()[cap.index as usize];
                match &capture_name[..] {
                    "definition" => def_node = Some(cap.node),
                    "name" => name_node = Some(cap.node),
                    _ => {}
                }
            }
            if let (Some(def_node), Some(name_node)) = (def_node, name_node) {
                let name = name_node.utf8_text(file_content.as_bytes())?.to_string();
                if !collected.contains(&name) {
                    let start_byte = def_node.start_byte();
                    let end_byte = def_node.end_byte();
                    let source = def_node.utf8_text(file_content.as_bytes())?.to_string();
                    let line_number = file_content[..start_byte].matches('\n').count() + 1;
                    definitions.push(Definition {
                        name: name.clone(),
                        start_byte,
                        end_byte,
                        source,
                        file_path: Some(start_path.to_path_buf()),
                        line_number: Some(line_number),
                    });
                    collected.insert(name.clone());
                    to_visit.push((start_path.to_path_buf(), name));
                }
            }
        }

        let references_query_str = match self.get_query_content(&language, "calls") {
            Ok(s) => s,
            Err(_) => {
                return Ok(Context {
                    definitions,
                    references,
                });
            }
        };

        let references_query = match Query::new(&language, references_query_str) {
            Ok(q) => q,
            Err(_) => {
                return Ok(Context {
                    definitions,
                    references,
                });
            }
        };

        let mut references_cursor = QueryCursor::new();
        let mut ref_matches =
            references_cursor.matches(&references_query, tree.root_node(), file_content.as_bytes());

        while let Some(mat) = ref_matches.next() {
            for cap in mat.captures {
                let capture_name = &references_query.capture_names()[cap.index as usize];
                if [
                    "direct_call",
                    "method_call",
                    "macro_call",
                    "reference",
                    "callback",
                    "import",
                    "assignment",
                ]
                .contains(capture_name)
                {
                    let node = cap.node;
                    let name = node.utf8_text(file_content.as_bytes())?.to_string();
                    let start_byte = node.start_byte();
                    let end_byte = node.end_byte();
                    let source = node.utf8_text(file_content.as_bytes())?.to_string();
                    let line_number = file_content[..start_byte].matches('\n').count() + 1;

                    references.push(Definition {
                        name,
                        start_byte,
                        end_byte,
                        source,
                        file_path: Some(start_path.to_path_buf()),
                        line_number: Some(line_number),
                    });
                }
            }
        }

        while let Some((file_path, func_name)) = to_visit.pop() {
            if let Some((_, def)) = self.find_definition(&func_name, &file_path)? {
                let refs = self.find_calls(&def.name)?;
                for (ref_file, ref_def, _) in refs {
                    if !collected.contains(&ref_def.name) {
                        definitions.push(ref_def.clone());
                        collected.insert(ref_def.name.clone());
                        to_visit.push((ref_file, ref_def.name.clone()));
                    }
                }
            }
        }

        Ok(Context {
            definitions,
            references,
        })
    }
}

impl Default for CodeParser {
    fn default() -> Self {
        Self {
            files: HashMap::new(),
            parser: Parser::new(),
        }
    }
}
