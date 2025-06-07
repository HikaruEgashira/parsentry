use git2::{Cred, Error, FetchOptions, RemoteCallbacks, Repository};
use std::env;
use std::path::Path;

/// GitHubリポジトリをcloneする
///
/// # 引数
/// - repo: "owner/repo" 形式のGitHubリポジトリ名
/// - dest: clone先ディレクトリ
pub fn clone_github_repo(repo: &str, dest: &Path) -> Result<(), Error> {
    if dest.exists() {
        return Err(Error::from_str("Destination directory already exists"));
    }

    let url = format!("https://github.com/{}.git", repo);

    let token = env::var("GITHUB_TOKEN").ok();

    let mut callbacks = RemoteCallbacks::new();
    if let Some(ref token) = token {
        callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
            Cred::userpass_plaintext("x-access-token", token)
        });
    }

    let mut fetch_options = FetchOptions::new();
    if token.is_some() {
        fetch_options.remote_callbacks(callbacks);
    }

    let repo = Repository::init(dest)?;
    let mut remote = repo.remote("origin", &url)?;

    remote.fetch(
        &["refs/heads/*:refs/remotes/origin/*"],
        Some(&mut fetch_options),
        None,
    )?;

    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = fetch_head.peel_to_commit()?;
    repo.branch("master", &fetch_commit, true)?;
    repo.set_head("refs/heads/master")?;
    repo.checkout_head(None)?;

    Ok(())
}
