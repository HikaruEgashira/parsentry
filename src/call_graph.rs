use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use streaming_iterator::StreamingIterator;

use crate::parser::{CodeParser, Definition};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallNode {
    pub function_name: String,
    pub file_path: PathBuf,
    pub line_number: usize,
    pub node_type: NodeType,
    pub start_byte: usize,
    pub end_byte: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeType {
    Function,
    Method,
    Lambda,
    Closure,
    Constructor,
    Module,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallEdge {
    pub caller: String,
    pub callee: String,
    pub call_site: Location,
    pub relation_type: RelationType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub file_path: PathBuf,
    pub line_number: usize,
    pub start_byte: usize,
    pub end_byte: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationType {
    DirectCall,     // foo()
    MethodCall,     // obj.method()
    MacroCall,      // macro!()
    Reference,      // let f = foo;
    Callback,       // addEventListener(foo)
    Import,         // use foo; / import foo
    Assignment,     // foo = bar
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallGraph {
    pub nodes: HashMap<String, CallNode>,
    pub edges: Vec<CallEdge>,
    pub metadata: GraphMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphMetadata {
    pub total_nodes: usize,
    pub total_edges: usize,
    pub languages: Vec<String>,
    pub root_functions: Vec<String>,
    pub cycles: Vec<Vec<String>>,
}

pub struct CallGraphBuilder {
    pub parser: CodeParser,
    pub graph: CallGraph,
    pub visited: HashSet<String>,
}

#[derive(Debug, Clone)]
pub struct CallGraphConfig {
    pub max_depth: Option<usize>,
    pub start_functions: Vec<String>,
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub detect_cycles: bool,
    pub security_focus: bool,
}

impl Default for CallGraphConfig {
    fn default() -> Self {
        Self {
            max_depth: Some(10),
            start_functions: Vec::new(),
            include_patterns: vec![".*".to_string()],
            exclude_patterns: Vec::new(),
            detect_cycles: true,
            security_focus: false,
        }
    }
}

impl CallGraphBuilder {
    pub fn new(parser: CodeParser) -> Self {
        Self {
            parser,
            graph: CallGraph {
                nodes: HashMap::new(),
                edges: Vec::new(),
                metadata: GraphMetadata {
                    total_nodes: 0,
                    total_edges: 0,
                    languages: Vec::new(),
                    root_functions: Vec::new(),
                    cycles: Vec::new(),
                },
            },
            visited: HashSet::new(),
        }
    }

    pub fn build(&mut self, config: &CallGraphConfig) -> Result<&CallGraph> {
        // Clear previous state
        self.graph.nodes.clear();
        self.graph.edges.clear();
        self.visited.clear();

        // If start functions are specified, build from them
        if !config.start_functions.is_empty() {
            for start_func in &config.start_functions {
                self.build_from_function(start_func, 0, config)?;
            }
        } else {
            // Otherwise, build from all discovered functions
            self.build_full_graph(config)?;
        }

        // Detect cycles if requested
        if config.detect_cycles {
            self.detect_cycles()?;
        }

        // Update metadata
        self.update_metadata();

        Ok(&self.graph)
    }

    fn build_from_function(
        &mut self, 
        function_name: &str, 
        current_depth: usize, 
        config: &CallGraphConfig
    ) -> Result<()> {
        // Check depth limit
        if let Some(max_depth) = config.max_depth {
            if current_depth >= max_depth {
                return Ok(());
            }
        }

        // Skip if already visited
        if self.visited.contains(function_name) {
            return Ok(());
        }
        self.visited.insert(function_name.to_string());

        // Find the function definition
        let mut definition_found = false;
        let file_paths: Vec<PathBuf> = self.parser.files.keys().cloned().collect();
        for file_path in file_paths {
            if let Some((def_file, definition)) = self.parser.find_definition(function_name, &file_path)? {
                // Add node to graph
                let node = CallNode {
                    function_name: function_name.to_string(),
                    file_path: def_file.clone(),
                    line_number: self.byte_to_line_number(&definition.source, definition.start_byte),
                    node_type: self.determine_node_type(&definition),
                    start_byte: definition.start_byte,
                    end_byte: definition.end_byte,
                };
                self.graph.nodes.insert(function_name.to_string(), node);
                definition_found = true;

                // Extract function calls from the function definition itself
                if let Ok(called_functions) = self.extract_function_calls(&def_file, &definition) {
                    for (called_func, relation_type) in called_functions {
                        // Skip self-references to avoid infinite loops
                        if called_func == function_name {
                            continue;
                        }
                        
                        // Add edge
                        let edge = CallEdge {
                            caller: function_name.to_string(),
                            callee: called_func.clone(),
                            call_site: Location {
                                file_path: def_file.clone(),
                                line_number: self.byte_to_line_number(&definition.source, definition.start_byte),
                                start_byte: definition.start_byte,
                                end_byte: definition.end_byte,
                            },
                            relation_type,
                        };
                        self.graph.edges.push(edge);

                        // Only recurse if within depth limit
                        if config.max_depth.is_none() || current_depth + 1 < config.max_depth.unwrap() {
                            self.build_from_function(&called_func, current_depth + 1, config)?;
                        }
                    }
                }
                break;
            }
        }

        if !definition_found {
            // Add as external/unknown node
            let node = CallNode {
                function_name: function_name.to_string(),
                file_path: PathBuf::from("external"),
                line_number: 0,
                node_type: NodeType::Function,
                start_byte: 0,
                end_byte: 0,
            };
            self.graph.nodes.insert(function_name.to_string(), node);
        }

        Ok(())
    }

    fn build_full_graph(&mut self, config: &CallGraphConfig) -> Result<()> {
        // Collect all function definitions first
        let mut all_functions = Vec::new();
        let file_paths: Vec<PathBuf> = self.parser.files.keys().cloned().collect();
        
        for file_path in file_paths {
            let context = self.parser.build_context_from_file(&file_path)?;
            for definition in context.definitions {
                all_functions.push(definition.name);
            }
        }

        // Build graph from all functions
        for function_name in all_functions {
            if !self.visited.contains(&function_name) {
                self.build_from_function(&function_name, 0, config)?;
            }
        }

        Ok(())
    }

    fn extract_function_calls(&mut self, file_path: &PathBuf, definition: &Definition) -> Result<Vec<(String, RelationType)>> {
        let mut calls = Vec::new();
        
        // Get the language for this file
        let language = match self.parser.get_language(file_path) {
            Some(lang) => lang,
            None => return Ok(calls), // Return empty for unsupported languages
        };

        // Set the language in the parser
        self.parser.parser.set_language(&language).map_err(|e| {
            anyhow!("Failed to set language for {}: {}", file_path.display(), e)
        })?;

        // Parse the function definition source
        let tree = match self.parser.parser.parse(&definition.source, None) {
            Some(t) => t,
            None => return Ok(calls), // Return empty if parsing fails
        };

        // Get the calls query for this language
        let query_str = match self.parser.get_query_content(&language, "calls") {
            Ok(s) => s,
            Err(_) => {
                // Fallback to regex-based extraction if no query is available
                return self.extract_function_calls_regex(definition);
            }
        };

        // Create and execute the query
        let query = match tree_sitter::Query::new(&language, &query_str) {
            Ok(q) => q,
            Err(_) => {
                // Fallback to regex-based extraction if query creation fails
                return self.extract_function_calls_regex(definition);
            }
        };

        let mut query_cursor = tree_sitter::QueryCursor::new();
        let mut matches = query_cursor.matches(&query, tree.root_node(), definition.source.as_bytes());

        while let Some(mat) = matches.next() {
            for cap in mat.captures {
                let capture_name = query.capture_names()[cap.index as usize];
                let relation_type = match capture_name {
                    "direct_call" => RelationType::DirectCall,
                    "method_call" => RelationType::MethodCall,
                    "macro_call" => RelationType::MacroCall,
                    "reference" => RelationType::Reference,
                    "callback" => RelationType::Callback,
                    "import" => RelationType::Import,
                    "assignment" => RelationType::Assignment,
                    _ => continue, // Skip unknown captures
                };
                
                let node = cap.node;
                if let Ok(call_name) = node.utf8_text(definition.source.as_bytes()) {
                    let name = call_name.to_string();
                    if !name.is_empty() && !calls.iter().any(|(n, _)| n == &name) {
                        calls.push((name, relation_type));
                    }
                }
            }
        }

        Ok(calls)
    }

    fn extract_function_calls_regex(&self, definition: &Definition) -> Result<Vec<(String, RelationType)>> {
        let mut calls = Vec::new();
        let content = &definition.source;
        
        // Common keywords and built-in functions to exclude
        let excluded_names = [
            "if", "for", "while", "match", "loop", "break", "continue", "return",
            "let", "mut", "const", "static", "fn", "struct", "enum", "impl", "trait",
            "pub", "use", "mod", "crate", "super", "self", "Self", "where", "type",
            "println", "print", "eprintln", "eprint", "dbg", "panic", "todo", "unimplemented",
            "assert", "assert_eq", "assert_ne", "debug_assert", "unreachable",
            "Some", "None", "Ok", "Err", "Vec", "String", "Option", "Result"
        ];
        
        // Basic pattern matching for function calls as fallback
        let patterns = [
            r"(\w+)\s*\(",  // Simple function calls like func()
            r"\.(\w+)\s*\(",  // Method calls like obj.method()
        ];
        
        for pattern in &patterns {
            if let Ok(regex) = regex::Regex::new(pattern) {
                for capture in regex.captures_iter(content) {
                    if let Some(func_name) = capture.get(1) {
                        let name = func_name.as_str().to_string();
                        if !name.is_empty() 
                            && !calls.iter().any(|(n, _)| n == &name)
                            && !excluded_names.contains(&name.as_str())
                            && !name.chars().next().unwrap_or('a').is_uppercase() // Skip types/constructors
                        {
                            // Default to DirectCall for regex-based extraction
                            calls.push((name, RelationType::DirectCall));
                        }
                    }
                }
            }
        }
        
        Ok(calls)
    }

    fn determine_node_type(&self, definition: &Definition) -> NodeType {
        // Simplified node type detection based on source content
        let source = &definition.source;
        
        if source.contains("class ") || source.contains("struct ") {
            NodeType::Constructor
        } else if source.contains("lambda") || source.contains("=>") {
            NodeType::Lambda
        } else if source.contains("def ") || source.contains("function ") {
            NodeType::Function
        } else {
            NodeType::Method
        }
    }

    fn byte_to_line_number(&self, content: &str, byte_offset: usize) -> usize {
        content[..byte_offset.min(content.len())]
            .chars()
            .filter(|&c| c == '\n')
            .count() + 1
    }

    fn detect_cycles(&mut self) -> Result<()> {
        // Tarjan's strongly connected components algorithm
        let mut index = 0;
        let mut stack = Vec::new();
        let mut indices = HashMap::new();
        let mut lowlinks = HashMap::new();
        let mut on_stack = HashSet::new();
        let mut cycles = Vec::new();

        for node_name in self.graph.nodes.keys() {
            if !indices.contains_key(node_name) {
                self.tarjan_scc(
                    node_name,
                    &mut index,
                    &mut stack,
                    &mut indices,
                    &mut lowlinks,
                    &mut on_stack,
                    &mut cycles,
                )?;
            }
        }

        self.graph.metadata.cycles = cycles;
        Ok(())
    }

    fn tarjan_scc(
        &self,
        node: &str,
        index: &mut usize,
        stack: &mut Vec<String>,
        indices: &mut HashMap<String, usize>,
        lowlinks: &mut HashMap<String, usize>,
        on_stack: &mut HashSet<String>,
        cycles: &mut Vec<Vec<String>>,
    ) -> Result<()> {
        indices.insert(node.to_string(), *index);
        lowlinks.insert(node.to_string(), *index);
        *index += 1;
        stack.push(node.to_string());
        on_stack.insert(node.to_string());

        // Find successors
        for edge in &self.graph.edges {
            if edge.caller == node {
                let successor = &edge.callee;
                if !indices.contains_key(successor) {
                    self.tarjan_scc(successor, index, stack, indices, lowlinks, on_stack, cycles)?;
                    let successor_lowlink = *lowlinks.get(successor).unwrap_or(&0);
                    let current_lowlink = *lowlinks.get(node).unwrap_or(&0);
                    lowlinks.insert(node.to_string(), current_lowlink.min(successor_lowlink));
                } else if on_stack.contains(successor) {
                    let successor_index = *indices.get(successor).unwrap_or(&0);
                    let current_lowlink = *lowlinks.get(node).unwrap_or(&0);
                    lowlinks.insert(node.to_string(), current_lowlink.min(successor_index));
                }
            }
        }

        // If node is a root node, pop the stack and create an SCC
        if lowlinks.get(node) == indices.get(node) {
            let mut component = Vec::new();
            loop {
                if let Some(w) = stack.pop() {
                    on_stack.remove(&w);
                    component.push(w.clone());
                    if w == node {
                        break;
                    }
                } else {
                    break;
                }
            }
            
            // Only add cycles with more than one node
            if component.len() > 1 {
                cycles.push(component);
            }
        }

        Ok(())
    }

    fn update_metadata(&mut self) {
        // Remove duplicate edges before updating metadata
        self.remove_duplicate_edges();
        
        self.graph.metadata.total_nodes = self.graph.nodes.len();
        self.graph.metadata.total_edges = self.graph.edges.len();
        
        // Collect languages from file extensions
        let mut languages = HashSet::new();
        for node in self.graph.nodes.values() {
            if let Some(ext) = node.file_path.extension() {
                if let Some(ext_str) = ext.to_str() {
                    languages.insert(ext_str.to_string());
                }
            }
        }
        self.graph.metadata.languages = languages.into_iter().collect();
        
        // Find root functions (functions that are not called by others)
        let called_functions: HashSet<String> = self.graph.edges.iter()
            .map(|edge| edge.callee.clone())
            .collect();
        
        self.graph.metadata.root_functions = self.graph.nodes.keys()
            .filter(|name| !called_functions.contains(*name))
            .cloned()
            .collect();
    }

    fn remove_duplicate_edges(&mut self) {
        let mut seen_edges = HashSet::new();
        self.graph.edges.retain(|edge| {
            let edge_key = (edge.caller.clone(), edge.callee.clone());
            if seen_edges.contains(&edge_key) {
                false
            } else {
                seen_edges.insert(edge_key);
                true
            }
        });
    }
}