//! Docker sandbox for the verify phase.
//!
//! The verify worker runs inside a container that mounts the source tree
//! read-only and reaches services on the host through `host.docker.internal`.
//! Every container carries the label `parsentry.target=<cache key>` so that
//! containers leaked by an abnormal exit can be swept by label.

use anyhow::{Context, Result, bail};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

/// Label key attached to every sandbox container.
pub const TARGET_LABEL: &str = "parsentry.target";
/// Mount point of the read-only source tree inside the container.
pub const SOURCE_MOUNT: &str = "/src";
/// Default image for the verify worker.
pub const DEFAULT_IMAGE: &str = "debian:stable-slim";

/// Label-safe cache key for a target, derived from its cache directory.
pub fn target_cache_key(cache_dir: &Path) -> String {
    format!(
        "{:x}",
        Sha256::digest(cache_dir.to_string_lossy().as_bytes())
    )
}

/// `key=value` form of the target label.
pub fn target_label(cache_key: &str) -> String {
    format!("{TARGET_LABEL}={cache_key}")
}

/// Configuration of a verify container.
#[derive(Debug, Clone)]
pub struct SandboxSpec {
    pub image: String,
    pub source: PathBuf,
    pub cache_key: String,
    pub command: Vec<String>,
}

impl SandboxSpec {
    /// Arguments for `docker run` (detached; the guard owns the lifetime).
    pub fn run_args(&self) -> Vec<String> {
        let mut args: Vec<String> = vec![
            "run".into(),
            "--detach".into(),
            "--rm".into(),
            "--init".into(),
            "--label".into(),
            target_label(&self.cache_key),
            "--add-host".into(),
            "host.docker.internal:host-gateway".into(),
            "--volume".into(),
            format!("{}:{SOURCE_MOUNT}:ro", self.source.display()),
            "--workdir".into(),
            SOURCE_MOUNT.into(),
            self.image.clone(),
        ];
        args.extend(self.command.iter().cloned());
        args
    }
}

fn docker() -> Command {
    Command::new(std::env::var("PARSENTRY_DOCKER").unwrap_or_else(|_| "docker".into()))
}

/// Whether a Docker daemon is reachable.
pub fn docker_available() -> bool {
    docker()
        .args(["info", "--format", "{{.ServerVersion}}"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

/// Remove every container labelled for `cache_key`. Returns how many were removed.
pub fn sweep(cache_key: &str) -> Result<usize> {
    let out = docker()
        .args(["ps", "--all", "--quiet", "--filter"])
        .arg(format!("label={}", target_label(cache_key)))
        .output()
        .context("failed to run docker ps")?;
    if !out.status.success() {
        bail!(
            "docker ps failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    let ids: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .split_whitespace()
        .map(str::to_string)
        .collect();
    if ids.is_empty() {
        return Ok(0);
    }
    let status = docker()
        .args(["rm", "--force"])
        .args(&ids)
        .stdout(Stdio::null())
        .status()
        .context("failed to run docker rm")?;
    if !status.success() {
        bail!("docker rm failed for {} container(s)", ids.len());
    }
    Ok(ids.len())
}

/// Best-effort sweep used at scan boundaries; skipped when Docker is absent.
pub fn sweep_if_available(cache_key: &str) -> Option<usize> {
    if !docker_available() {
        return None;
    }
    sweep(cache_key).ok()
}

/// A running verify container. Killed when dropped.
#[derive(Debug)]
pub struct VerifySandbox {
    id: String,
    killed: bool,
}

impl VerifySandbox {
    pub fn start(spec: &SandboxSpec) -> Result<Self> {
        let out = docker()
            .args(spec.run_args())
            .output()
            .context("failed to run docker run")?;
        if !out.status.success() {
            bail!(
                "docker run failed: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }
        let id = String::from_utf8_lossy(&out.stdout).trim().to_string();
        if id.is_empty() {
            bail!("docker run returned no container id");
        }
        Ok(Self { id, killed: false })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    /// Wait for the container to exit, killing it on SIGINT/SIGTERM.
    /// Returns the container exit code (or `None` when interrupted).
    pub async fn wait(mut self) -> Result<Option<i32>> {
        let mut child = tokio::process::Command::new(
            std::env::var("PARSENTRY_DOCKER").unwrap_or_else(|_| "docker".into()),
        )
        .args(["wait", &self.id])
        .stdout(Stdio::piped())
        .kill_on_drop(true)
        .spawn()
        .context("failed to run docker wait")?;
        let stdout = child.stdout.take();

        tokio::select! {
            status = child.wait() => {
                status?;
                let mut code = String::new();
                if let Some(mut s) = stdout {
                    use tokio::io::AsyncReadExt;
                    s.read_to_string(&mut code).await.ok();
                }
                self.kill();
                Ok(code.trim().parse().ok())
            }
            _ = shutdown_signal() => {
                self.kill();
                Ok(None)
            }
        }
    }

    /// Force-remove the container. Idempotent.
    pub fn kill(&mut self) {
        if self.killed {
            return;
        }
        self.killed = true;
        let _ = docker()
            .args(["rm", "--force", &self.id])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
}

impl Drop for VerifySandbox {
    fn drop(&mut self) {
        self.kill();
    }
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{SignalKind, signal};
        match signal(SignalKind::terminate()) {
            Ok(mut term) => {
                tokio::select! {
                    _ = term.recv() => {}
                    _ = tokio::signal::ctrl_c() => {}
                }
            }
            Err(_) => {
                let _ = tokio::signal::ctrl_c().await;
            }
        }
    }
    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec() -> SandboxSpec {
        SandboxSpec {
            image: "img:1".into(),
            source: PathBuf::from("/repo"),
            cache_key: "abc".into(),
            command: vec!["sh".into(), "-c".into(), "true".into()],
        }
    }

    #[test]
    fn run_args_mount_source_read_only_with_label_and_host_gateway() {
        let args = spec().run_args();
        let joined = args.join(" ");
        assert!(joined.contains("--label parsentry.target=abc"));
        assert!(joined.contains("--add-host host.docker.internal:host-gateway"));
        assert!(joined.contains("--volume /repo:/src:ro"));
        assert!(joined.contains("--rm"));
        assert!(joined.ends_with("img:1 sh -c true"));
    }

    #[test]
    fn cache_key_is_stable_and_label_safe() {
        let a = target_cache_key(Path::new("/c/owner__repo"));
        assert_eq!(a, target_cache_key(Path::new("/c/owner__repo")));
        assert_ne!(a, target_cache_key(Path::new("/c/other")));
        assert_eq!(a.len(), 64);
        assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[cfg(unix)]
    #[test]
    fn sweep_and_kill_use_label_and_force_remove() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let log = dir.path().join("log");
        let fake = dir.path().join("docker");
        std::fs::write(
            &fake,
            format!(
                "#!/bin/sh\necho \"$@\" >> {}\ncase \"$1\" in ps) echo c1; echo c2;; run) echo cid;; esac\n",
                log.display()
            ),
        )
        .unwrap();
        std::fs::set_permissions(&fake, std::fs::Permissions::from_mode(0o755)).unwrap();
        // SAFETY: only this test reads PARSENTRY_DOCKER.
        unsafe { std::env::set_var("PARSENTRY_DOCKER", &fake) };

        assert_eq!(sweep("abc").unwrap(), 2);
        let sb = VerifySandbox::start(&spec()).unwrap();
        assert_eq!(sb.id(), "cid");
        drop(sb);

        unsafe { std::env::remove_var("PARSENTRY_DOCKER") };
        let calls = std::fs::read_to_string(&log).unwrap();
        assert!(calls.contains("ps --all --quiet --filter label=parsentry.target=abc"));
        assert!(calls.contains("rm --force c1 c2"));
        assert!(calls.contains("rm --force cid"));
    }
}
