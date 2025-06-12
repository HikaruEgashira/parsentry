use parsentry::repo::RepoOps;
use tempfile::tempdir;

#[test]
fn test_ruby_files_are_recognized() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let file_path = dir.path().join("test.rb");
    std::fs::write(&file_path, "puts 'hello'")?;

    let repo = RepoOps::new(dir.path().to_path_buf());
    let files = repo.get_relevant_files();

    assert!(files.contains(&file_path));
    Ok(())
}

#[test]
fn test_matches_gitignore_leading_star() {
    assert!(RepoOps::matches_gitignore_pattern("error.log", "*.log"));
    assert!(RepoOps::matches_gitignore_pattern(
        "logs/error.log",
        "*.log"
    ));
    assert!(!RepoOps::matches_gitignore_pattern("error.txt", "*.log"));
}

#[test]
fn test_matches_gitignore_trailing_star() {
    assert!(RepoOps::matches_gitignore_pattern(
        "build/output.o",
        "build/*"
    ));
    assert!(RepoOps::matches_gitignore_pattern(
        "build/sub/obj.o",
        "build/*"
    ));
    assert!(!RepoOps::matches_gitignore_pattern(
        "target/output.o",
        "build/*"
    ));
}

#[test]
fn test_matches_gitignore_exact_match() {
    assert!(RepoOps::matches_gitignore_pattern(
        "src/main.rs",
        "src/main.rs"
    ));
    assert!(!RepoOps::matches_gitignore_pattern(
        "src/lib.rs",
        "src/main.rs"
    ));
}

#[test]
fn test_matches_gitignore_nested_directory() {
    assert!(RepoOps::matches_gitignore_pattern(
        "app/node_modules/package.json",
        "node_modules"
    ));
    assert!(!RepoOps::matches_gitignore_pattern(
        "app/modules/package.json",
        "node_modules"
    ));
}
