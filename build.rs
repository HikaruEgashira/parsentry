use std::path::PathBuf;

fn main() {
    let dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    // Get tree-sitter include path from the tree-sitter crate
    let tree_sitter_dir = PathBuf::from(
        std::env::var("DEP_TREE_SITTER_RUNTIME_INCLUDE").unwrap_or_else(|_| {
            // Fallback to a common location if the env var is not set
            format!("{}/target/debug/build/tree-sitter-*/out", dir.display())
        }),
    );

    // Compile tree-sitter parsers
    println!("cargo:rerun-if-changed=build.rs");

    cc::Build::new()
        .include(&dir)
        .include(&tree_sitter_dir)
        .file(dir.join("tree-sitter-go/src/parser.c"))
        .compile("tree-sitter-go");

    cc::Build::new()
        .include(&dir)
        .include(&tree_sitter_dir)
        .file(dir.join("tree-sitter-java/src/parser.c"))
        .compile("tree-sitter-java");

    cc::Build::new()
        .include(&dir)
        .include(&tree_sitter_dir)
        .file(dir.join("tree-sitter-javascript/src/parser.c"))
        .file(dir.join("tree-sitter-javascript/src/scanner.c"))
        .compile("tree-sitter-javascript");

    cc::Build::new()
        .include(&dir)
        .include(&tree_sitter_dir)
        .file(dir.join("tree-sitter-python/src/parser.c"))
        .file(dir.join("tree-sitter-python/src/scanner.c"))
        .compile("tree-sitter-python");

    cc::Build::new()
        .include(&dir)
        .include(&tree_sitter_dir)
        .file(dir.join("tree-sitter-rust/src/parser.c"))
        .file(dir.join("tree-sitter-rust/src/scanner.c"))
        .compile("tree-sitter-rust");

    cc::Build::new()
        .include(&dir)
        .include(&tree_sitter_dir)
        .include(dir.join("tree-sitter-typescript/typescript/src"))
        .include(dir.join("tree-sitter-typescript/common"))
        .file(dir.join("tree-sitter-typescript/typescript/src/parser.c"))
        .file(dir.join("tree-sitter-typescript/typescript/src/scanner.c"))
        .compile("tree-sitter-typescript");

    // Add library search path
    println!("cargo:rustc-link-search=native={}", out_dir.display());
}
