//! `parsentry verify` — run a verify worker inside a Docker sandbox.

use anyhow::{Result, bail};

use crate::cli::ui::StatusPrinter;
use crate::sandbox::{SandboxSpec, VerifySandbox, sweep, target_cache_key};

use super::common::{cache_dir_for, locate_repository, repo_name_from_target};

pub async fn run_verify_command(target: &str, image: &str, command: Vec<String>) -> Result<()> {
    let printer = StatusPrinter::with_service(repo_name_from_target(target));
    let (root_dir, _repo_name) = locate_repository(target, &printer).await?;
    let source = std::fs::canonicalize(&root_dir)?;
    let cache_key = target_cache_key(&cache_dir_for(target));

    // Clear containers from a previous run that exited abnormally.
    let stale = sweep(&cache_key)?;
    if stale > 0 {
        printer.warning(
            "Sandbox",
            &format!("removed {stale} stale verify container(s)"),
        );
    }

    let spec = SandboxSpec {
        image: image.to_string(),
        source,
        cache_key,
        command,
    };
    let sandbox = VerifySandbox::start(&spec)?;
    printer.status(
        "Sandbox",
        &format!("verify container {} started", sandbox.id()),
    );
    match sandbox.wait().await? {
        Some(0) => Ok(()),
        Some(code) => bail!("verify worker exited with status {code}"),
        None => bail!("verify worker interrupted; container removed"),
    }
}
