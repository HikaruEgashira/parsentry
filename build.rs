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
        .file(dir.join("tree-sitter-c/src/parser.c"))
        .flag("-Wno-unused-parameter")
        .compile("tree-sitter-c");

    cc::Build::new()
        .include(&dir)
        .include(&tree_sitter_dir)
        .file(dir.join("tree-sitter-cpp/src/parser.c"))
        .file(dir.join("tree-sitter-cpp/src/scanner.c"))
        .flag("-Wno-unused-parameter")
        .compile("tree-sitter-cpp");

    cc::Build::new()
        .include(&dir)
        .include(&tree_sitter_dir)
        .file(dir.join("tree-sitter-go/src/parser.c"))
        .flag("-Wno-unused-parameter")
        .compile("tree-sitter-go");

    cc::Build::new()
        .include(&dir)
        .include(&tree_sitter_dir)
        .file(dir.join("tree-sitter-java/src/parser.c"))
        .flag("-Wno-unused-parameter")
        .compile("tree-sitter-java");

    cc::Build::new()
        .include(&dir)
        .include(&tree_sitter_dir)
        .file(dir.join("tree-sitter-javascript/src/parser.c"))
        .file(dir.join("tree-sitter-javascript/src/scanner.c"))
        .flag("-Wno-unused-parameter")
        .compile("tree-sitter-javascript");

    cc::Build::new()
        .include(&dir)
        .include(&tree_sitter_dir)
        .file(dir.join("tree-sitter-python/src/parser.c"))
        .file(dir.join("tree-sitter-python/src/scanner.c"))
        .flag("-Wno-unused-parameter")
        .compile("tree-sitter-python");

    cc::Build::new()
        .include(&dir)
        .include(&tree_sitter_dir)
        .file(dir.join("tree-sitter-rust/src/parser.c"))
        .file(dir.join("tree-sitter-rust/src/scanner.c"))
        .flag("-Wno-unused-parameter")
        .compile("tree-sitter-rust");

    cc::Build::new()
        .include(&dir)
        .include(&tree_sitter_dir)
        .include(dir.join("tree-sitter-typescript/typescript/src"))
        .include(dir.join("tree-sitter-typescript/common"))
        .file(dir.join("tree-sitter-typescript/typescript/src/parser.c"))
        .file(dir.join("tree-sitter-typescript/typescript/src/scanner.c"))
        .flag("-Wno-unused-parameter")
        .compile("tree-sitter-typescript");

    // Add build step for TSX parser
    cc::Build::new()
        .include(&dir)
        .include(&tree_sitter_dir)
        .include(dir.join("tree-sitter-typescript/tsx/src"))
        .include(dir.join("tree-sitter-typescript/common"))
        .file(dir.join("tree-sitter-typescript/tsx/src/parser.c"))
        .file(dir.join("tree-sitter-typescript/tsx/src/scanner.c"))
        .flag("-Wno-unused-parameter")
        .compile("tree-sitter-tsx");

    cc::Build::new()
        .include(&dir)
        .include(&tree_sitter_dir)
        .file(dir.join("tree-sitter-ruby/src/parser.c"))
        .file(dir.join("tree-sitter-ruby/src/scanner.c"))
        .flag("-Wno-unused-parameter")
        .compile("tree-sitter-ruby");

    // Add HCL/Terraform parser
    cc::Build::new()
        .include(&dir)
        .include(&tree_sitter_dir)
        .file(dir.join("tree-sitter-terraform/src/parser.c"))
        .file(dir.join("tree-sitter-terraform/src/scanner.c"))
        .flag("-Wno-unused-parameter")
        .compile("tree-sitter-hcl");

    // TODO: Add YAML and Bash support once tree-sitter submodules are properly set up
    // cc::Build::new()
    //     .include(&dir)
    //     .include(&tree_sitter_dir)
    //     .include(dir.join("tree-sitter-yaml/src"))
    //     .file(dir.join("tree-sitter-yaml/src/parser.c"))
    //     .file(dir.join("tree-sitter-yaml/src/scanner.cc"))
    //     .flag("-Wno-unused-parameter")
    //     .cpp(true)
    //     .compile("tree-sitter-yaml");

    // cc::Build::new()
    //     .include(&dir)
    //     .include(&tree_sitter_dir)
    //     .include(dir.join("tree-sitter-bash/src"))
    //     .file(dir.join("tree-sitter-bash/src/parser.c"))
    //     .file(dir.join("tree-sitter-bash/src/scanner.c"))
    //     .flag("-Wno-unused-parameter")
    //     .compile("tree-sitter-bash");

    // Add library search path
    println!("cargo:rustc-link-search=native={}", out_dir.display());
}
