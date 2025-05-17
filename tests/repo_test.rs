use vulnhuntrs::repo::RepoOps;

#[test]
fn test_matches_gitignore_leading_star() {
    assert!(RepoOps::matches_gitignore_pattern("error.log", "*.log"));
    assert!(RepoOps::matches_gitignore_pattern("logs/error.log", "*.log"));
    assert!(!RepoOps::matches_gitignore_pattern("error.txt", "*.log"));
}

#[test]
fn test_matches_gitignore_trailing_star() {
    assert!(RepoOps::matches_gitignore_pattern("build/output.o", "build/*"));
    assert!(RepoOps::matches_gitignore_pattern("build/sub/obj.o", "build/*"));
    assert!(!RepoOps::matches_gitignore_pattern("target/output.o", "build/*"));
}

#[test]
fn test_matches_gitignore_exact_match() {
    assert!(RepoOps::matches_gitignore_pattern("src/main.rs", "src/main.rs"));
    assert!(!RepoOps::matches_gitignore_pattern("src/lib.rs", "src/main.rs"));
}
