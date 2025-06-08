use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use streaming_iterator::StreamingIterator;
unsafe extern "C" {
    fn tree_sitter_rust() -> tree_sitter::Language;
}
use tree_sitter::{Language, Node, Parser, Query, QueryCursor};

unsafe extern "C" {
    fn tree_sitter_c() -> Language;
    fn tree_sitter_cpp() -> Language;
    fn tree_sitter_python() -> Language;
    fn tree_sitter_javascript() -> Language;
    fn tree_sitter_typescript() -> Language;
    fn tree_sitter_tsx() -> Language;
    fn tree_sitter_java() -> Language;
    fn tree_sitter_go() -> Language;
    fn tree_sitter_ruby() -> Language;
    fn tree_sitter_hcl() -> Language;
}

#[derive(Debug, Clone)]
pub struct Definition {
    pub name: String,
    pub start_byte: usize,
    pub end_byte: usize,
    pub source: String,
}

#[derive(Debug, Clone)]
pub struct Context {
    pub definitions: Vec<Definition>,
}

pub struct CodeParser {
    pub files: HashMap<PathBuf, String>,
    pub parser: Parser,
}

impl CodeParser {
    pub fn new() -> Result<Self> {
        Ok(Self {
            files: HashMap::new(),
            parser: Parser::new(),
        })
    }

    pub fn add_file(&mut self, path: &Path) -> Result<()> {
        let content = fs::read_to_string(path).map_err(|e| {
            anyhow!(
                "ファイルの読み込みに失敗しました: {}: {}",
                path.display(),
                e
            )
        })?;
        self.files.insert(path.to_path_buf(), content.clone());
        Ok(())
    }

    fn get_language(&self, path: &Path) -> Option<Language> {
        let extension = path.extension().and_then(|ext| ext.to_str());
        match extension {
            Some("c") | Some("h") => Some(unsafe { tree_sitter_c() }),
            Some("cpp") | Some("cxx") | Some("cc") | Some("hpp") | Some("hxx") => Some(unsafe { tree_sitter_cpp() }),
            Some("py") => Some(unsafe { tree_sitter_python() }),
            Some("js") => Some(unsafe { tree_sitter_javascript() }),
            Some("ts") => Some(unsafe { tree_sitter_typescript() }),
            Some("tsx") => Some(unsafe { tree_sitter_tsx() }),
            Some("java") => Some(unsafe { tree_sitter_java() }),
            Some("rs") => Some(unsafe { tree_sitter_rust() }),
            Some("go") => Some(unsafe { tree_sitter_go() }),
            Some("rb") => Some(unsafe { tree_sitter_ruby() }),
            Some("tf") | Some("hcl") => Some(unsafe { tree_sitter_hcl() }),
            _ => None,
        }
    }

    fn get_query_content(&self, language: &Language, query_name: &str) -> Result<&'static str> {
        let lang_name = if language == &unsafe { tree_sitter_c() } {
            "c"
        } else if language == &unsafe { tree_sitter_cpp() } {
            "cpp"
        } else if language == &unsafe { tree_sitter_python() } {
            "python"
        } else if language == &unsafe { tree_sitter_javascript() } {
            "javascript"
        } else if language == &unsafe { tree_sitter_typescript() }
            || language == &unsafe { tree_sitter_tsx() }
        {
            "typescript"
        } else if language == &unsafe { tree_sitter_java() } {
            "java"
        } else if language == &unsafe { tree_sitter_go() } {
            "go"
        } else if language == &unsafe { tree_sitter_rust() } {
            "rust"
        } else if language == &unsafe { tree_sitter_ruby() } {
            "ruby"
        } else if language == &unsafe { tree_sitter_hcl() } {
            "terraform"
        } else {
            return Err(anyhow!("クエリに対応していない言語です"));
        };

        if lang_name.contains('/') || lang_name.contains('\\') || lang_name.contains("..") {
            return Err(anyhow!("クエリパスの言語名が不正です: {}", lang_name));
        }
        if query_name.contains('/') || query_name.contains('\\') || query_name.contains("..") {
            return Err(anyhow!("クエリパスのクエリ名が不正です: {}", query_name));
        }

        let query_content = match lang_name {
            "c" => match query_name {
                "definitions" => include_str!("queries/c/definitions.scm"),
                "references" => include_str!("queries/c/references.scm"),
                _ => return Err(anyhow!("未対応のクエリ名: {}", query_name)),
            },
            "cpp" => match query_name {
                "definitions" => include_str!("queries/cpp/definitions.scm"),
                "references" => include_str!("queries/cpp/references.scm"),
                _ => return Err(anyhow!("未対応のクエリ名: {}", query_name)),
            },
            "python" => match query_name {
                "definitions" => include_str!("queries/python/definitions.scm"),
                "references" => include_str!("queries/python/references.scm"),
                _ => return Err(anyhow!("未対応のクエリ名: {}", query_name)),
            },
            "javascript" => match query_name {
                "definitions" => include_str!("queries/javascript/definitions.scm"),
                "references" => include_str!("queries/javascript/references.scm"),
                _ => return Err(anyhow!("未対応のクエリ名: {}", query_name)),
            },
            "typescript" => match query_name {
                "definitions" => include_str!("queries/typescript/definitions.scm"),
                "references" => include_str!("queries/typescript/references.scm"),
                _ => return Err(anyhow!("未対応のクエリ名: {}", query_name)),
            },
            "java" => match query_name {
                "definitions" => include_str!("queries/java/definitions.scm"),
                "references" => include_str!("queries/java/references.scm"),
                _ => return Err(anyhow!("未対応のクエリ名: {}", query_name)),
            },
            "go" => match query_name {
                "definitions" => include_str!("queries/go/definitions.scm"),
                "references" => include_str!("queries/go/references.scm"),
                _ => return Err(anyhow!("未対応のクエリ名: {}", query_name)),
            },
            "rust" => match query_name {
                "definitions" => include_str!("queries/rust/definitions.scm"),
                "references" => include_str!("queries/rust/references.scm"),
                _ => return Err(anyhow!("未対応のクエリ名: {}", query_name)),
            },
            "ruby" => match query_name {
                "definitions" => include_str!("queries/ruby/definitions.scm"),
                "references" => include_str!("queries/ruby/references.scm"),
                _ => return Err(anyhow!("未対応のクエリ名: {}", query_name)),
            },
            "terraform" => match query_name {
                "definitions" => include_str!("queries/terraform/definitions.scm"),
                "references" => include_str!("queries/terraform/references.scm"),
                _ => return Err(anyhow!("未対応のクエリ名: {}", query_name)),
            },
            _ => return Err(anyhow!("未対応の言語: {}", lang_name)),
        };

        Ok(query_content)
    }

    pub fn find_definition(
        &mut self,
        name: &str,
        source_file: &Path,
    ) -> Result<Option<(PathBuf, Definition)>> {
        let content = self.files.get(source_file).ok_or_else(|| {
            anyhow!(
                "パーサーにファイルが見つかりません: {}",
                source_file.display()
            )
        })?;

        let language = match self.get_language(source_file) {
            Some(lang) => lang,
            None => return Ok(None),
        };

        self.parser
            .set_language(&language)
            .map_err(|e| anyhow!("言語の設定に失敗しました: {}", e))?;

        let tree = self
            .parser
            .parse(content, None)
            .ok_or_else(|| anyhow!("ファイルのパースに失敗しました: {}", source_file.display()))?;

        let query_str = self.get_query_content(&language, "definitions")?;

        let query = Query::new(&language, &query_str).map_err(|e| {
            anyhow!("クエリの生成に失敗しました: {}", e)
        })?;

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

            if let (Some(def_node), Some(name_node_inner)) = (definition_node, name_node) {
                if name_node_inner.utf8_text(content.as_bytes())? == name {
                    let start_byte = def_node.start_byte();
                    let end_byte = def_node.end_byte();
                    let source = def_node.utf8_text(content.as_bytes())?.to_string();

                    let definition = Definition {
                        name: name.to_string(),
                        start_byte,
                        end_byte,
                        source,
                    };
                    return Ok(Some((source_file.to_path_buf(), definition)));
                }
            }
        }

        Ok(None)
    }

    pub fn find_references(&mut self, name: &str) -> Result<Vec<(PathBuf, Definition)>> {
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
                    eprintln!(
                        "警告: ファイルのパースに失敗しました: {}",
                        file_path.display()
                    );
                    continue;
                }
            };

            let query_str = match self.get_query_content(&language, "references") {
                Ok(s) => s,
                Err(e) => {
                    eprintln!(
                        "警告: 参照クエリの取得に失敗しました: {}: {}",
                        file_path.display(),
                        e
                    );
                    continue;
                }
            };

            let query = match Query::new(&language, &query_str) {
                Ok(q) => q,
                Err(e) => {
                    eprintln!(
                        "警告: 参照クエリの生成に失敗しました: {}",
                        e
                    );
                    continue;
                }
            };

            let mut query_cursor = QueryCursor::new();
            let mut matches = query_cursor.matches(&query, tree.root_node(), content.as_bytes());

            while let Some(mat) = matches.next() {
                for cap in mat.captures {
                    if query.capture_names()[cap.index as usize] == "reference" {
                        let node = cap.node;
                        if node.utf8_text(content.as_bytes())? == name {
                            let start_byte = node.start_byte();
                            let end_byte = node.end_byte();
                            let source = name.to_string();

                            results.push((
                                file_path.clone(),
                                Definition {
                                    name: name.to_string(),
                                    start_byte,
                                    end_byte,
                                    source,
                                },
                            ));
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    /// Find both definitions and references for Action patterns to enable bidirectional tracking.
    /// This provides comprehensive context by showing both where data comes from (definitions) 
    /// and where it flows to (references).
    pub fn find_bidirectional(
        &mut self, 
        name: &str, 
        source_file: &Path
    ) -> Result<Vec<(PathBuf, Definition)>> {
        let mut results = Vec::new();
        
        // First, find the definition (backward tracking)
        if let Some(definition) = self.find_definition(name, source_file)? {
            results.push(definition);
        }
        
        // Then, find all references (forward tracking)
        let references = self.find_references(name)?;
        results.extend(references);
        
        // Remove duplicates based on file path and start byte
        results.sort_by_key(|(path, def)| (path.clone(), def.start_byte));
        results.dedup_by_key(|(path, def)| (path.clone(), def.start_byte));
        
        Ok(results)
    }
    pub fn build_context_from_file(&mut self, start_path: &Path) -> Result<Context> {
        use std::collections::HashSet;

        let mut collected: HashSet<String> = HashSet::new();
        let mut definitions: Vec<Definition> = Vec::new();

        let file_content = self
            .files
            .get(start_path)
            .ok_or_else(|| anyhow::anyhow!("ファイルが見つかりません: {}", start_path.display()))?;
        
        // If the language is not supported by tree-sitter (e.g., Terraform, YAML, JSON),
        // return an empty context instead of failing
        let language = match self.get_language(start_path) {
            Some(lang) => lang,
            None => {
                // For IaC files and other unsupported file types, return empty context
                return Ok(Context { definitions: Vec::new() });
            }
        };
        self.parser
            .set_language(&language)
            .map_err(|e| anyhow::anyhow!("言語の設定に失敗: {}", e))?;
        let tree = self
            .parser
            .parse(file_content, None)
            .ok_or_else(|| anyhow::anyhow!("パース失敗: {}", start_path.display()))?;

        let query_str = self.get_query_content(&language, "definitions")?;
        let query = tree_sitter::Query::new(&language, &query_str)?;

        let mut query_cursor = tree_sitter::QueryCursor::new();
        let mut matches = query_cursor.matches(&query, tree.root_node(), file_content.as_bytes());

        let mut to_visit: Vec<(PathBuf, String)> = Vec::new();

        while let Some(mat) = matches.next() {
            let mut def_node: Option<tree_sitter::Node> = None;
            let mut name_node: Option<tree_sitter::Node> = None;
            for cap in mat.captures {
                let capture_name = &query.capture_names()[cap.index as usize];
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
                    definitions.push(Definition {
                        name: name.clone(),
                        start_byte,
                        end_byte,
                        source,
                    });
                    collected.insert(name.clone());
                    to_visit.push((start_path.to_path_buf(), name));
                }
            }
        }

        while let Some((file_path, func_name)) = to_visit.pop() {
            if let Some((_, def)) = self.find_definition(&func_name, &file_path)? {
                // referencesクエリで呼び出し先を抽出
                let refs = self.find_references(&def.name)?;
                for (ref_file, ref_def) in refs {
                    if !collected.contains(&ref_def.name) {
                        definitions.push(ref_def.clone());
                        collected.insert(ref_def.name.clone());
                        to_visit.push((ref_file, ref_def.name.clone()));
                    }
                }
            }
        }

        Ok(Context { definitions })
    }
}
