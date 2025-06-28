#[cfg(test)]
mod tests {
    use parsentry::call_graph::{CallGraphBuilder, CallGraphConfig};
    use parsentry::call_graph_output::{CallGraphRenderer, OutputFormat};
    use parsentry::parser::CodeParser;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_call_graph_basic_functionality() {
        // Create a temporary directory
        let dir = tempdir().unwrap();
        
        // Create a simple Rust file with function calls
        let rust_file = dir.path().join("test.rs");
        std::fs::write(&rust_file, r#"
fn main() {
    println!("Hello");
    foo();
    bar();
}

fn foo() {
    bar();
    baz();
}

fn bar() {
    println!("Bar");
}

fn baz() {
    println!("Baz");
}
"#).unwrap();

        // Test the call graph functionality
        let mut parser = CodeParser::new().unwrap();
        parser.add_file(&rust_file).unwrap();
        
        let mut builder = CallGraphBuilder::new(parser);
        let config = CallGraphConfig::default();
        
        let call_graph = builder.build(&config).unwrap();
        
        // Verify that we have some nodes and edges
        assert!(!call_graph.nodes.is_empty(), "Call graph should have nodes");
        println!("Found {} nodes and {} edges", 
                call_graph.metadata.total_nodes, 
                call_graph.metadata.total_edges);
    }

    #[test]
    fn test_call_graph_output_formats() {
        // Create a temporary directory
        let dir = tempdir().unwrap();
        
        // Create a simple Rust file
        let rust_file = dir.path().join("simple.rs");
        std::fs::write(&rust_file, r#"
fn main() {
    hello();
}

fn hello() {
    println!("Hello World");
}
"#).unwrap();

        // Build call graph
        let mut parser = CodeParser::new().unwrap();
        parser.add_file(&rust_file).unwrap();
        
        let mut builder = CallGraphBuilder::new(parser);
        let config = CallGraphConfig::default();
        let call_graph = builder.build(&config).unwrap();

        // Test different output formats
        let formats = vec![
            OutputFormat::Json,
            OutputFormat::Dot, 
            OutputFormat::Mermaid,
            OutputFormat::Csv,
        ];

        for format in formats {
            let output = CallGraphRenderer::render(call_graph, &format);
            assert!(output.is_ok(), "Rendering should succeed for format: {:?}", format);
            
            let content = output.unwrap();
            assert!(!content.is_empty(), "Output should not be empty for format: {:?}", format);
            
            // Basic format-specific checks
            match format {
                OutputFormat::Json => {
                    assert!(content.contains("{"), "JSON output should contain braces");
                },
                OutputFormat::Dot => {
                    assert!(content.contains("digraph"), "DOT output should contain digraph");
                },
                OutputFormat::Mermaid => {
                    assert!(content.contains("graph"), "Mermaid output should contain graph");
                },
                OutputFormat::Csv => {
                    assert!(content.contains(","), "CSV output should contain commas");
                },
            }
        }
    }

    #[test]
    fn test_call_graph_with_start_functions() {
        // Create a temporary directory
        let dir = tempdir().unwrap();
        
        // Create a file with multiple functions
        let rust_file = dir.path().join("multi.rs");
        std::fs::write(&rust_file, r#"
fn main() {
    process_data();
}

fn process_data() {
    read_file();
    write_file();
}

fn read_file() {
    println!("Reading");
}

fn write_file() {
    println!("Writing");
}

fn unused_function() {
    println!("This won't be called");
}
"#).unwrap();

        // Build call graph with specific start function
        let mut parser = CodeParser::new().unwrap();
        parser.add_file(&rust_file).unwrap();
        
        let mut builder = CallGraphBuilder::new(parser);
        let mut config = CallGraphConfig::default();
        config.start_functions = vec!["main".to_string()];
        config.max_depth = Some(3);
        
        let call_graph = builder.build(&config).unwrap();
        
        // Should have found some functions but not all
        assert!(call_graph.metadata.total_nodes > 0, "Should find some nodes");
        println!("Start function analysis found {} nodes", call_graph.metadata.total_nodes);
    }
}