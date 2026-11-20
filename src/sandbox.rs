//! Docker sandbox for the verify phase.
//!
//! Every container is labelled `parsentry.target=<cache key>` so leftovers
//! from crashed runs can be swept by label. A [`VerifySandbox`] force-removes
//! its container on wait, on drop, and (via [`kill_all`]) on SIGTERM/SIGINT.

use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Mutex;

use anyhow::{Context, Result, bail};

pub const LABEL_KEY: &str = "parsentry.target";

/// Containers that are still running, removed by [`kill_all`] on signals.
static LIVE: Mutex<Vec<(OsString, String)>> = Mutex::new(Vec::new());

/// Docker binary, overridable with `PARSENTRY_DOCKER`.
pub fn docker_bin() -> OsString {
    std::env::var_os("PARSENTRY_DOCKER")
        .filter(|v| !v.is_empty())
        .unwrap_or_else(|| "docker".into())
}

/// Whether the Docker daemon answers.
pub fn docker_available(docker: &OsString) -> bool {
    Command::new(docker)
        .args(["version", "--format", "{{.Server.Version}}"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

/// Label-safe form of a cache directory name: `[A-Za-z0-9._-]` only.
pub fn label_key(cache_dir: &Path) -> String {
    let name = cache_dir
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let key: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-') {
                c
            } else {
                '_'
            }
        })
        .collect();
    if key.is_empty() { "_".into() } else { key }
}

pub struct SandboxSpec {
    pub image: String,
    pub source: PathBuf,
    pub key: String,
    pub command: Vec<String>,
}

impl SandboxSpec {
    pub fn run_args(&self) -> Vec<OsString> {
        let mut volume = OsString::from(self.source.as_os_str());
        volume.push(":/src:ro");
        let mut args: Vec<OsString> = ["run", "--detach", "--rm", "--init", "--label"]
            .iter()
            .map(OsString::from)
            .collect();
        args.push(format!("{LABEL_KEY}={}", self.key).into());
        args.push("--add-host".into());
        args.push("host.docker.internal:host-gateway".into());
        args.push("--volume".into());
        args.push(volume);
        args.push(self.image.clone().into());
        args.extend(self.command.iter().map(OsString::from));
        args
    }
}

pub struct VerifySandbox {
    docker: OsString,
    id: String,
    removed: bool,
}

impl VerifySandbox {
    pub fn start(docker: &OsString, spec: &SandboxSpec) -> Result<Self> {
        let out = Command::new(docker)
            .args(spec.run_args())
            .stdin(Stdio::null())
            .output()
            .context("failed to run docker")?;
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
        LIVE.lock()
            .unwrap_or_else(|e| e.into_inner())
            .push((docker.clone(), id.clone()));
        Ok(Self {
            docker: docker.clone(),
            id,
            removed: false,
        })
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    /// Stream logs until the container exits, then force-remove it.
    /// Returns the container exit code when Docker still reports it.
    pub fn wait(mut self) -> Result<Option<i32>> {
        let _ = Command::new(&self.docker)
            .args(["logs", "--follow", &self.id])
            .stdin(Stdio::null())
            .status();
        let out = Command::new(&self.docker)
            .args(["wait", &self.id])
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .output();
        self.remove();
        Ok(out
            .ok()
            .filter(|o| o.status.success())
            .and_then(|o| String::from_utf8_lossy(&o.stdout).trim().parse().ok()))
    }

    fn remove(&mut self) {
        if self.removed {
            return;
        }
        self.removed = true;
        force_remove(&self.docker, &self.id);
        LIVE.lock()
            .unwrap_or_else(|e| e.into_inner())
            .retain(|(_, id)| id != &self.id);
    }
}

impl Drop for VerifySandbox {
    fn drop(&mut self) {
        self.remove();
    }
}

fn force_remove(docker: &OsString, id: &str) {
    let _ = Command::new(docker)
        .args(["rm", "--force", id])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}

/// Force-remove every live sandbox; called from the signal handler.
pub fn kill_all() {
    let live = std::mem::take(&mut *LIVE.lock().unwrap_or_else(|e| e.into_inner()));
    for (docker, id) in live {
        force_remove(&docker, &id);
    }
}

/// Remove all containers labelled with `key`. Returns how many were removed.
pub fn sweep(docker: &OsString, key: &str) -> Result<usize> {
    let out = Command::new(docker)
        .args(["ps", "--all", "--quiet", "--filter"])
        .arg(format!("label={LABEL_KEY}={key}"))
        .stdin(Stdio::null())
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
    for id in &ids {
        force_remove(docker, id);
    }
    Ok(ids.len())
}

/// Sweep `key` when Docker is available; silently skip otherwise.
pub fn sweep_if_available(key: &str) -> Option<usize> {
    let docker = docker_bin();
    if !docker_available(&docker) {
        return None;
    }
    sweep(&docker, key).ok()
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;
    use tempfile::TempDir;

    /// `kill_all` touches every live sandbox, so tests that start one run serially.
    static SERIAL: Mutex<()> = Mutex::new(());

    /// A fake docker that logs each invocation to `calls.log`.
    fn fake_docker(dir: &TempDir) -> (OsString, PathBuf) {
        let log = dir.path().join("calls.log");
        let script = dir.path().join("docker");
        std::fs::write(
            &script,
            format!(
                r#"#!/bin/sh
echo "$*" >> '{log}'
case "$1" in
  run) echo cid123 ;;
  ps) printf 'old1\nold2\n' ;;
  wait) echo 7 ;;
esac
"#,
                log = log.display()
            ),
        )
        .unwrap();
        std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
        (script.into_os_string(), log)
    }

    fn calls(log: &Path) -> Vec<String> {
        std::fs::read_to_string(log)
            .unwrap_or_default()
            .lines()
            .map(str::to_string)
            .collect()
    }

    fn spec() -> SandboxSpec {
        SandboxSpec {
            image: "alpine:3".into(),
            source: "/repo".into(),
            key: "owner__repo".into(),
            command: vec!["sh".into(), "-c".into(), "true".into()],
        }
    }

    #[test]
    fn run_args_are_labelled_and_read_only() {
        let args: Vec<String> = spec()
            .run_args()
            .into_iter()
            .map(|a| a.into_string().unwrap())
            .collect();
        assert_eq!(
            args,
            [
                "run",
                "--detach",
                "--rm",
                "--init",
                "--label",
                "parsentry.target=owner__repo",
                "--add-host",
                "host.docker.internal:host-gateway",
                "--volume",
                "/repo:/src:ro",
                "alpine:3",
                "sh",
                "-c",
                "true",
            ]
        );
    }

    #[test]
    fn label_key_is_label_safe() {
        assert_eq!(label_key(Path::new("/c/owner__repo")), "owner__repo");
        assert_eq!(label_key(Path::new("/c/a b=c,d\"e")), "a_b_c_d_e");
        assert_eq!(label_key(Path::new("/")), "_");
    }

    #[test]
    fn sweep_removes_labelled_containers() {
        let dir = TempDir::new().unwrap();
        let (docker, log) = fake_docker(&dir);
        assert_eq!(sweep(&docker, "k").unwrap(), 2);
        assert_eq!(
            calls(&log),
            [
                "ps --all --quiet --filter label=parsentry.target=k",
                "rm --force old1",
                "rm --force old2",
            ]
        );
    }

    #[test]
    fn wait_removes_container_once() {
        let _guard = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let dir = TempDir::new().unwrap();
        let (docker, log) = fake_docker(&dir);
        let sb = VerifySandbox::start(&docker, &spec()).unwrap();
        assert_eq!(sb.id(), "cid123");
        assert_eq!(sb.wait().unwrap(), Some(7));
        let c = calls(&log);
        assert_eq!(
            &c[1..],
            ["logs --follow cid123", "wait cid123", "rm --force cid123"]
        );
    }

    #[test]
    fn drop_and_kill_all_remove_container() {
        let _guard = SERIAL.lock().unwrap_or_else(|e| e.into_inner());
        let dir = TempDir::new().unwrap();
        let (docker, log) = fake_docker(&dir);
        drop(VerifySandbox::start(&docker, &spec()).unwrap());
        assert_eq!(calls(&log).last().unwrap(), "rm --force cid123");

        let sb = VerifySandbox::start(&docker, &spec()).unwrap();
        kill_all();
        assert_eq!(calls(&log).last().unwrap(), "rm --force cid123");
        std::mem::forget(sb);
    }
}
