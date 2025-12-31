//! Context optimizer for reducing LLM context size.
//!
//! This module provides functionality to filter definitions based on call graph
//! relationships, reducing the amount of context passed to LLM agents.

use anyhow::Result;
use std::collections::HashSet;
use std::path::PathBuf;

use crate::call_graph::{CallGraph, CallGraphBuilder, CallGraphConfig};
use parsentry_core::Language;
use parsentry_parser::{CodeParser, Definition};

/// Reference to a function with location information.
#[derive(Debug, Clone)]
pub struct FunctionReference {
    /// Function name.
    pub name: String,
    /// Absolute file path.
    pub file_path: PathBuf,
    /// Line number (1-based).
    pub line_number: usize,
}

impl FunctionReference {
    /// Format as "path:line name" for compact representation.
    pub fn to_location_string(&self) -> String {
        format!("{}:{} {}", self.file_path.display(), self.line_number, self.name)
    }
}

/// Filter definitions to only those in the call graph starting from security-relevant functions.
///
/// # Arguments
/// * `parser` - CodeParser with loaded files (consumed)
/// * `definitions` - All definitions extracted from the codebase
/// * `security_functions` - Function names that are security-relevant (Principal/Resource)
/// * `max_depth` - Maximum depth to traverse in the call graph
///
/// # Returns
/// Filtered definitions that are within the call graph of security functions.
pub fn filter_by_call_graph(
    parser: CodeParser,
    definitions: &[(Definition, Language)],
    security_functions: &[String],
    max_depth: usize,
) -> Result<Vec<(Definition, Language)>> {
    if security_functions.is_empty() {
        // If no security functions specified, return all definitions
        return Ok(definitions.to_vec());
    }

    // Build call graph starting from security functions
    let config = CallGraphConfig {
        start_functions: security_functions.to_vec(),
        max_depth: Some(max_depth),
        detect_cycles: false,
        security_focus: true,
        ..Default::default()
    };

    let mut builder = CallGraphBuilder::new(parser);
    let graph = builder.build(&config)?;

    // Collect function names that are in the call graph
    let graph_functions: HashSet<&str> = graph.nodes.keys().map(|s| s.as_str()).collect();

    // Filter definitions to only those in the call graph
    let filtered: Vec<_> = definitions
        .iter()
        .filter(|(def, _)| graph_functions.contains(def.name.as_str()))
        .cloned()
        .collect();

    Ok(filtered)
}

/// Convert definitions to file references (path + line only).
///
/// This is useful for agents that can read files themselves,
/// as it reduces context size while still providing location information.
pub fn to_file_references(definitions: &[(Definition, Language)]) -> Vec<FunctionReference> {
    definitions
        .iter()
        .filter_map(|(def, _)| {
            def.file_path.as_ref().map(|path| FunctionReference {
                name: def.name.clone(),
                file_path: path.clone(),
                line_number: def.line_number.unwrap_or(0),
            })
        })
        .collect()
}

/// Build a call graph for a set of security functions.
///
/// # Arguments
/// * `parser` - CodeParser with loaded files (consumed)
/// * `security_functions` - Function names to start traversal from
/// * `max_depth` - Maximum depth to traverse
///
/// # Returns
/// The constructed call graph.
pub fn build_call_graph(
    parser: CodeParser,
    security_functions: &[String],
    max_depth: usize,
) -> Result<CallGraph> {
    let config = CallGraphConfig {
        start_functions: security_functions.to_vec(),
        max_depth: Some(max_depth),
        detect_cycles: false,
        security_focus: true,
        ..Default::default()
    };

    let mut builder = CallGraphBuilder::new(parser);
    let graph = builder.build(&config)?;
    Ok(graph.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_function_reference_to_location_string() {
        let reference = FunctionReference {
            name: "validate_input".to_string(),
            file_path: PathBuf::from("/src/validator.rs"),
            line_number: 42,
        };
        assert_eq!(reference.to_location_string(), "/src/validator.rs:42 validate_input");
    }

    #[test]
    fn test_to_file_references_empty() {
        let definitions: Vec<(Definition, Language)> = vec![];
        let references = to_file_references(&definitions);
        assert!(references.is_empty());
    }
}
