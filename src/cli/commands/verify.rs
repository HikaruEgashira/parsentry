//! Hidden `parsentry verify`: run a PoC command in a labelled Docker sandbox.

use anyhow::{Result, bail};

use super::common::{cache_dir_for, locate_repository, repo_name_from_target};
use crate::cli::ui::StatusPrinter;
use crate::sandbox::{self, SandboxSpec, VerifySandbox};

pub async fn run_verify_command(target: &str, image: &str, command: Vec<String>) -> Result<()> {
    let printer = StatusPrinter::with_service(repo_name_from_target(target));
    let (root_dir, _) = locate_repository(target, &printer).await?;
    let docker = sandbox::docker_bin();
    if !sandbox::docker_available(&docker) {
        bail!("docker is not available (set PARSENTRY_DOCKER to override the binary)");
    }
    let spec = SandboxSpec {
        image: image.to_string(),
        source: std::fs::canonicalize(&root_dir)?,
        key: sandbox::label_key(&cache_dir_for(target)),
        command,
    };
    let sb = VerifySandbox::start(&docker, &spec)?;
    printer.status("Sandbox", &format!("started {}", sb.id()));
    let code = tokio::task::spawn_blocking(move || sb.wait()).await??;
    match code {
        Some(0) => Ok(()),
        Some(c) => std::process::exit(c),
        None => bail!("sandbox exit code unavailable"),
    }
}
