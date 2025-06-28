use anyhow::{Result, anyhow};
use serde_json;
use std::collections::HashSet;
use std::fmt::Write;

use crate::call_graph::{CallGraph, EdgeType};

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Dot,
    Json,
    Mermaid,
    Csv,
}

impl std::str::FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "dot" => Ok(OutputFormat::Dot),
            "json" => Ok(OutputFormat::Json),
            "mermaid" => Ok(OutputFormat::Mermaid),
            "csv" => Ok(OutputFormat::Csv),
            _ => Err(anyhow!("Unsupported output format: {}", s)),
        }
    }
}

pub struct CallGraphRenderer;

impl CallGraphRenderer {
    pub fn render(graph: &CallGraph, format: &OutputFormat) -> Result<String> {
        match format {
            OutputFormat::Dot => Self::render_dot(graph),
            OutputFormat::Json => Self::render_json(graph),
            OutputFormat::Mermaid => Self::render_mermaid(graph),
            OutputFormat::Csv => Self::render_csv(graph),
        }
    }

    fn render_dot(graph: &CallGraph) -> Result<String> {
        let mut output = String::new();
        writeln!(output, "digraph CallGraph {{")?;
        writeln!(output, "  rankdir=TB;")?;
        writeln!(output, "  node [shape=box, style=rounded];")?;
        writeln!(output)?;

        // Add nodes
        writeln!(output, "  // Nodes")?;
        for (name, node) in &graph.nodes {
            let escaped_name = Self::escape_dot_string(name);
            let file_name = node.file_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            let label = format!("{}\\n({}:{})", name, file_name, node.line_number);
            let escaped_label = Self::escape_dot_string(&label);
            
            let color = match node.node_type {
                crate::call_graph::NodeType::Function => "lightblue",
                crate::call_graph::NodeType::Method => "lightgreen",
                crate::call_graph::NodeType::Lambda => "lightyellow",
                crate::call_graph::NodeType::Constructor => "lightcoral",
                crate::call_graph::NodeType::Module => "lightgray",
                crate::call_graph::NodeType::Closure => "lightpink",
            };
            
            writeln!(output, "  \"{}\" [label=\"{}\", fillcolor={}, style=\"rounded,filled\"];", 
                escaped_name, escaped_label, color)?;
        }

        writeln!(output)?;

        // Add edges
        writeln!(output, "  // Edges")?;
        for edge in &graph.edges {
            let escaped_caller = Self::escape_dot_string(&edge.caller);
            let escaped_callee = Self::escape_dot_string(&edge.callee);
            
            let style = match edge.edge_type {
                EdgeType::Direct => "solid",
                EdgeType::Indirect => "dashed",
                EdgeType::Virtual => "dotted",
                EdgeType::Dynamic => "bold",
            };
            
            writeln!(output, "  \"{}\" -> \"{}\" [style={}];", 
                escaped_caller, escaped_callee, style)?;
        }

        // Add metadata as a comment
        writeln!(output)?;
        writeln!(output, "  // Metadata")?;
        writeln!(output, "  // Total nodes: {}", graph.metadata.total_nodes)?;
        writeln!(output, "  // Total edges: {}", graph.metadata.total_edges)?;
        writeln!(output, "  // Languages: {}", graph.metadata.languages.join(", "))?;
        writeln!(output, "  // Root functions: {}", graph.metadata.root_functions.join(", "))?;
        
        writeln!(output, "}}")?;
        Ok(output)
    }

    fn render_json(graph: &CallGraph) -> Result<String> {
        let json = serde_json::to_string_pretty(graph)?;
        Ok(json)
    }

    fn render_mermaid(graph: &CallGraph) -> Result<String> {
        let mut output = String::new();
        writeln!(output, "graph TD")?;
        
        // Create a mapping for node IDs (Mermaid has restrictions on node names)
        let mut node_id_map = std::collections::HashMap::new();
        let mut counter = 0;
        
        for name in graph.nodes.keys() {
            let id = format!("N{}", counter);
            node_id_map.insert(name.clone(), id);
            counter += 1;
        }

        // Add nodes with labels
        for (name, node) in &graph.nodes {
            let node_id = &node_id_map[name];
            let file_name = node.file_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            let label = format!("{}\\n{}:{}", name, file_name, node.line_number);
            
            let shape = match node.node_type {
                crate::call_graph::NodeType::Function => ("(", ")"),
                crate::call_graph::NodeType::Method => ("[", "]"),
                crate::call_graph::NodeType::Lambda => ("{{", "}}"),
                crate::call_graph::NodeType::Constructor => ("([", "])"),
                crate::call_graph::NodeType::Module => ("((", "))"),
                crate::call_graph::NodeType::Closure => (">", "]"),
            };
            
            writeln!(output, "  {}{}\"{}\"{}", node_id, shape.0, label, shape.1)?;
        }

        // Add edges
        for edge in &graph.edges {
            if let (Some(caller_id), Some(callee_id)) = 
                (node_id_map.get(&edge.caller), node_id_map.get(&edge.callee)) {
                
                let arrow = match edge.edge_type {
                    EdgeType::Direct => "-->",
                    EdgeType::Indirect => "-.->",
                    EdgeType::Virtual => "==>",
                    EdgeType::Dynamic => "==>",
                };
                
                writeln!(output, "  {} {} {}", caller_id, arrow, callee_id)?;
            } else {
                // Debug: Log missing nodes
                if !node_id_map.contains_key(&edge.caller) {
                    eprintln!("Warning: Missing caller node '{}' in Mermaid output", edge.caller);
                }
                if !node_id_map.contains_key(&edge.callee) {
                    eprintln!("Warning: Missing callee node '{}' in Mermaid output", edge.callee);
                }
            }
        }

        // Add metadata as comments
        writeln!(output)?;
        writeln!(output, "%% Metadata:")?;
        writeln!(output, "%% Total nodes: {}", graph.metadata.total_nodes)?;
        writeln!(output, "%% Total edges: {}", graph.metadata.total_edges)?;
        writeln!(output, "%% Languages: {}", graph.metadata.languages.join(", "))?;
        if !graph.metadata.cycles.is_empty() {
            writeln!(output, "%% Cycles detected: {}", graph.metadata.cycles.len())?;
        }

        Ok(output)
    }

    fn render_csv(graph: &CallGraph) -> Result<String> {
        let mut output = String::new();
        
        // Write header
        writeln!(output, "Type,Caller,Callee,CallerFile,CallerLine,CalleeFile,CalleeLine,EdgeType")?;
        
        // Write edges
        for edge in &graph.edges {
            let caller_node = graph.nodes.get(&edge.caller);
            let callee_node = graph.nodes.get(&edge.callee);
            
            let caller_file = caller_node
                .map(|n| n.file_path.to_string_lossy())
                .unwrap_or_default();
            let caller_line = caller_node
                .map(|n| n.line_number.to_string())
                .unwrap_or_default();
                
            let callee_file = callee_node
                .map(|n| n.file_path.to_string_lossy())
                .unwrap_or_default();
            let callee_line = callee_node
                .map(|n| n.line_number.to_string())
                .unwrap_or_default();
            
            let edge_type = match edge.edge_type {
                EdgeType::Direct => "Direct",
                EdgeType::Indirect => "Indirect",
                EdgeType::Virtual => "Virtual",
                EdgeType::Dynamic => "Dynamic",
            };
            
            writeln!(output, "Edge,{},{},{},{},{},{},{}", 
                Self::escape_csv_field(&edge.caller),
                Self::escape_csv_field(&edge.callee),
                Self::escape_csv_field(&caller_file),
                caller_line,
                Self::escape_csv_field(&callee_file),
                callee_line,
                edge_type)?;
        }
        
        // Write nodes
        for (name, node) in &graph.nodes {
            let node_type = match node.node_type {
                crate::call_graph::NodeType::Function => "Function",
                crate::call_graph::NodeType::Method => "Method",
                crate::call_graph::NodeType::Lambda => "Lambda",
                crate::call_graph::NodeType::Constructor => "Constructor",
                crate::call_graph::NodeType::Module => "Module",
                crate::call_graph::NodeType::Closure => "Closure",
            };
            
            writeln!(output, "Node,{},,,{},{},,,{}",
                Self::escape_csv_field(name),
                Self::escape_csv_field(&node.file_path.to_string_lossy()),
                node.line_number,
                node_type)?;
        }
        
        // Write metadata
        writeln!(output, "Metadata,TotalNodes,{},,,,,", graph.metadata.total_nodes)?;
        writeln!(output, "Metadata,TotalEdges,{},,,,,", graph.metadata.total_edges)?;
        writeln!(output, "Metadata,Languages,\"{}\",,,,,", graph.metadata.languages.join(";"))?;
        writeln!(output, "Metadata,RootFunctions,\"{}\",,,,,", graph.metadata.root_functions.join(";"))?;
        
        if !graph.metadata.cycles.is_empty() {
            for (i, cycle) in graph.metadata.cycles.iter().enumerate() {
                writeln!(output, "Metadata,Cycle{},\"{}\",,,,,", i, cycle.join("->"))?;
            }
        }
        
        Ok(output)
    }

    fn escape_dot_string(s: &str) -> String {
        s.replace('\\', "\\\\")
         .replace('"', "\\\"")
         .replace('\n', "\\n")
         .replace('\r', "\\r")
         .replace('\t', "\\t")
    }

    fn escape_csv_field(s: &str) -> String {
        if s.contains(',') || s.contains('"') || s.contains('\n') || s.contains('\r') {
            format!("\"{}\"", s.replace('"', "\"\""))
        } else {
            s.to_string()
        }
    }
}

// Filter utilities for the call graph
pub struct CallGraphFilter;

impl CallGraphFilter {
    pub fn filter_by_depth(graph: &mut CallGraph, start_nodes: &[String], max_depth: usize) -> Result<()> {
        if start_nodes.is_empty() {
            return Ok(());
        }

        let mut visited = HashSet::new();
        let mut nodes_to_keep = HashSet::new();
        let mut edges_to_keep = Vec::new();

        // BFS to find nodes within depth limit
        let mut current_level = start_nodes.iter().cloned().collect::<HashSet<String>>();
        nodes_to_keep.extend(current_level.clone());

        for _ in 0..max_depth {
            let mut next_level = HashSet::new();
            
            for node in &current_level {
                if visited.contains(node) {
                    continue;
                }
                visited.insert(node.clone());

                // Find all edges from this node
                for edge in &graph.edges {
                    if edge.caller == *node && !nodes_to_keep.contains(&edge.callee) {
                        next_level.insert(edge.callee.clone());
                        edges_to_keep.push(edge.clone());
                    }
                }
            }

            if next_level.is_empty() {
                break;
            }

            nodes_to_keep.extend(next_level.clone());
            current_level = next_level;
        }

        // Filter nodes
        graph.nodes.retain(|name, _| nodes_to_keep.contains(name));
        
        // Filter edges
        graph.edges.retain(|edge| {
            nodes_to_keep.contains(&edge.caller) && nodes_to_keep.contains(&edge.callee)
        });

        Ok(())
    }

    pub fn filter_by_pattern(graph: &mut CallGraph, include_patterns: &[String], exclude_patterns: &[String]) -> Result<()> {
        use regex::Regex;

        let include_regexes: Result<Vec<Regex>, _> = include_patterns.iter()
            .map(|pattern| Regex::new(pattern))
            .collect();
        let include_regexes = include_regexes?;

        let exclude_regexes: Result<Vec<Regex>, _> = exclude_patterns.iter()
            .map(|pattern| Regex::new(pattern))
            .collect();
        let exclude_regexes = exclude_regexes?;

        // Filter nodes
        graph.nodes.retain(|name, node| {
            let file_path_str = node.file_path.to_string_lossy();
            
            // Check exclude patterns first
            for regex in &exclude_regexes {
                if regex.is_match(name) || regex.is_match(&file_path_str) {
                    return false;
                }
            }

            // Check include patterns
            if include_regexes.is_empty() {
                return true;
            }

            for regex in &include_regexes {
                if regex.is_match(name) || regex.is_match(&file_path_str) {
                    return true;
                }
            }

            false
        });

        // Filter edges to only include those between remaining nodes
        let remaining_nodes: HashSet<String> = graph.nodes.keys().cloned().collect();
        graph.edges.retain(|edge| {
            remaining_nodes.contains(&edge.caller) && remaining_nodes.contains(&edge.callee)
        });

        Ok(())
    }

    pub fn filter_by_language(graph: &mut CallGraph, languages: &[String]) -> Result<()> {
        if languages.is_empty() {
            return Ok(());
        }

        let target_extensions: HashSet<String> = languages.iter()
            .map(|lang| Self::language_to_extension(lang))
            .flatten()
            .collect();

        // Filter nodes by file extension
        graph.nodes.retain(|_, node| {
            if let Some(ext) = node.file_path.extension() {
                if let Some(ext_str) = ext.to_str() {
                    return target_extensions.contains(ext_str);
                }
            }
            false
        });

        // Filter edges
        let remaining_nodes: HashSet<String> = graph.nodes.keys().cloned().collect();
        graph.edges.retain(|edge| {
            remaining_nodes.contains(&edge.caller) && remaining_nodes.contains(&edge.callee)
        });

        Ok(())
    }

    fn language_to_extension(language: &str) -> Vec<String> {
        match language.to_lowercase().as_str() {
            "rust" => vec!["rs".to_string()],
            "python" => vec!["py".to_string()],
            "javascript" => vec!["js".to_string()],
            "typescript" => vec!["ts".to_string(), "tsx".to_string()],
            "java" => vec!["java".to_string()],
            "c" => vec!["c".to_string(), "h".to_string()],
            "cpp" | "c++" => vec!["cpp".to_string(), "cxx".to_string(), "cc".to_string(), "hpp".to_string(), "hxx".to_string()],
            "go" => vec!["go".to_string()],
            "ruby" => vec!["rb".to_string()],
            "php" => vec!["php".to_string()],
            "terraform" => vec!["tf".to_string(), "hcl".to_string()],
            _ => Vec::new(),
        }
    }
}