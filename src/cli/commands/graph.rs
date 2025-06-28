use anyhow::Result;
use std::path::PathBuf;

use crate::call_graph::{CallGraphBuilder, CallGraphConfig};
use crate::call_graph_output::{CallGraphRenderer, OutputFormat, CallGraphFilter};
use crate::cli::args::GraphArgs;
use crate::parser::CodeParser;
use crate::repo::{RepoOps, clone_github_repo};

pub async fn run_graph_command(args: GraphArgs) -> Result<()> {
    println!("📊 Starting call graph generation...");
    
    // Handle repository cloning or use provided root
    let (root_dir, _repo_name) = if let Some(repo) = &args.repo {
        let dest = PathBuf::from("repo");
        if dest.exists() {
            std::fs::remove_dir_all(&dest).map_err(|e| {
                anyhow::anyhow!("Failed to delete clone directory: {}", e)
            })?;
        }
        println!("🛠️  Cloning GitHub repository: {} → {}", repo, dest.display());
        clone_github_repo(repo, &dest).map_err(|e| {
            anyhow::anyhow!("Failed to clone GitHub repository: {}", e)
        })?;
        
        let repo_name = if repo.contains('/') {
            repo.split('/')
                .last()
                .unwrap_or("unknown-repo")
                .replace(".git", "")
        } else {
            repo.replace(".git", "")
        };
        
        (dest, Some(repo_name))
    } else if let Some(root) = &args.root {
        (root.clone(), None)
    } else {
        return Err(anyhow::anyhow!("Either --root or --repo must be specified"));
    };
    
    let repo = RepoOps::new(root_dir.clone());
    let files = repo.get_relevant_files();
    
    // Initialize the code parser
    let mut parser = CodeParser::new()?;
    
    // Add all relevant files to the parser
    println!("📂 Loading {} files for call graph analysis...", files.len());
    for file_path in files {
        if let Err(e) = parser.add_file(&file_path) {
            if args.verbosity > 0 {
                println!("⚠️  Failed to load {}: {}", file_path.display(), e);
            }
        }
    }

    // Create call graph builder
    let mut builder = CallGraphBuilder::new(parser);

    // Configure call graph generation
    let mut config = CallGraphConfig::default();
    
    if let Some(max_depth) = args.max_depth {
        config.max_depth = Some(max_depth);
    }

    if let Some(start_functions_str) = &args.start_functions {
        config.start_functions = start_functions_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
    }

    if let Some(include_patterns_str) = &args.include {
        config.include_patterns = include_patterns_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
    }

    if let Some(exclude_patterns_str) = &args.exclude {
        config.exclude_patterns = exclude_patterns_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
    }

    config.detect_cycles = args.detect_cycles;
    config.security_focus = args.security_focus;

    // Build the call graph
    println!("🔗 Building call graph...");
    let call_graph = builder.build(&config)?;

    // Apply filters if needed
    let mut filtered_graph = call_graph.clone();
    
    if !config.include_patterns.is_empty() || !config.exclude_patterns.is_empty() {
        CallGraphFilter::filter_by_pattern(
            &mut filtered_graph,
            &config.include_patterns,
            &config.exclude_patterns,
        )?;
    }

    // Parse output format
    let output_format: OutputFormat = args.format.parse()?;

    // Generate output
    let output_content = CallGraphRenderer::render(&filtered_graph, &output_format)?;

    // Determine output path
    let output_path = if let Some(explicit_path) = &args.output {
        explicit_path.clone()
    } else {
        let extension = match output_format {
            OutputFormat::Dot => "dot",
            OutputFormat::Json => "json",
            OutputFormat::Mermaid => "md",
            OutputFormat::Csv => "csv",
        };
        PathBuf::from(format!("call_graph.{}", extension))
    };

    // Write output
    std::fs::write(&output_path, output_content)?;

    // Print summary
    println!("📊 Call Graph Summary:");
    println!("  • Total nodes: {}", filtered_graph.metadata.total_nodes);
    println!("  • Total edges: {}", filtered_graph.metadata.total_edges);
    println!("  • Languages: {}", filtered_graph.metadata.languages.join(", "));
    println!("  • Root functions: {}", filtered_graph.metadata.root_functions.len());
    
    if !filtered_graph.metadata.cycles.is_empty() {
        println!("  • Cycles detected: {}", filtered_graph.metadata.cycles.len());
        for (i, cycle) in filtered_graph.metadata.cycles.iter().enumerate() {
            println!("    Cycle {}: {}", i + 1, cycle.join(" -> "));
        }
    }

    println!(
        "📁 Call graph saved to: {}",
        output_path.display()
    );
    
    println!("✅ Call graph generation completed");

    Ok(())
}