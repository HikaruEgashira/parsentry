//! Agent runtimes that drive the scan pipeline in-process.
//!
//! In the default pipe mode `parsentry scan` prints a prompt for an external
//! CLI agent. With `--runtime claude` each phase prompt is instead sent to an
//! [`AgentRuntime`], so the pipeline runs to completion without an external
//! CLI. [`SidecarRuntime`] talks to the TypeScript sidecar in
//! `tools/agent-runtime` (Claude Agent SDK) over line-delimited JSON-RPC 2.0
//! on stdio. The sidecar owns concurrency, timeouts and 429/529 backoff; it
//! only returns text, and parsentry validates and writes every result file.

use anyhow::{Context, Result, anyhow, bail};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use tokio::sync::Mutex;

/// Environment variable overriding the sidecar command (whitespace separated),
/// e.g. `node tools/agent-runtime/dist/index.js`.
pub const SIDECAR_ENV: &str = "PARSENTRY_AGENT_RUNTIME";
/// Sidecar executable looked up on `PATH` when [`SIDECAR_ENV`] is unset.
pub const SIDECAR_BIN: &str = "parsentry-agent-runtime";

/// One surface analysis task.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SurfaceTask {
    pub id: String,
    pub prompt: String,
}

/// Outcome of a surface task: the agent's final text or an error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceOutcome {
    pub id: String,
    pub result: std::result::Result<String, String>,
}

#[async_trait]
pub trait AgentRuntime: Send + Sync {
    /// Run the threat model prompt; returns the agent's final text.
    async fn run_model(&self, prompt: &str, cwd: &Path) -> Result<String>;
    /// Run surface prompts; the runtime decides how many run concurrently.
    async fn run_surface(&self, tasks: &[SurfaceTask], cwd: &Path) -> Result<Vec<SurfaceOutcome>>;
    /// Run a verification prompt for a finding; returns the agent's final text.
    async fn run_verify(&self, prompt: &str, cwd: &Path) -> Result<String>;
}

/// Runtime selected by `parsentry scan --runtime`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, clap::ValueEnum)]
pub enum RuntimeKind {
    /// Print phase prompts for an external CLI agent (default)
    #[default]
    Pipe,
    /// Run phases with the Claude Agent SDK sidecar
    Claude,
}

struct Connection {
    _child: Child,
    stdin: ChildStdin,
    stdout: Lines<BufReader<ChildStdout>>,
    next_id: u64,
}

/// [`AgentRuntime`] backed by a JSON-RPC stdio sidecar process.
pub struct SidecarRuntime {
    backend: String,
    conn: Mutex<Connection>,
}

impl SidecarRuntime {
    /// Spawn the sidecar from [`SIDECAR_ENV`] or [`SIDECAR_BIN`].
    pub fn spawn_default(backend: &str) -> Result<Self> {
        let command = std::env::var(SIDECAR_ENV).unwrap_or_else(|_| SIDECAR_BIN.to_string());
        let parts: Vec<String> = command.split_whitespace().map(String::from).collect();
        let (program, args) = parts
            .split_first()
            .ok_or_else(|| anyhow!("{SIDECAR_ENV} is empty"))?;
        Self::spawn(PathBuf::from(program).as_path(), args, backend).with_context(|| {
            format!(
                "failed to start agent runtime `{command}`. Build tools/agent-runtime \
                 (npm install && npm run build) and set {SIDECAR_ENV}=\"node \
                 tools/agent-runtime/dist/index.js\" or put {SIDECAR_BIN} on PATH"
            )
        })
    }

    pub fn spawn(program: &Path, args: &[String], backend: &str) -> Result<Self> {
        let mut child = Command::new(program)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .kill_on_drop(true)
            .spawn()?;
        let stdin = child.stdin.take().context("sidecar stdin unavailable")?;
        let stdout = child.stdout.take().context("sidecar stdout unavailable")?;
        Ok(Self {
            backend: backend.to_string(),
            conn: Mutex::new(Connection {
                _child: child,
                stdin,
                stdout: BufReader::new(stdout).lines(),
                next_id: 1,
            }),
        })
    }

    async fn call(&self, method: &str, mut params: Value) -> Result<Value> {
        params["backend"] = json!(self.backend);
        let mut conn = self.conn.lock().await;
        let id = conn.next_id;
        conn.next_id += 1;
        let request = json!({"jsonrpc": "2.0", "id": id, "method": method, "params": params});
        let mut line = serde_json::to_string(&request)?;
        line.push('\n');
        conn.stdin.write_all(line.as_bytes()).await?;
        conn.stdin.flush().await?;
        loop {
            let line = conn
                .stdout
                .next_line()
                .await?
                .ok_or_else(|| anyhow!("agent runtime exited before answering `{method}`"))?;
            // Ignore anything that is not a JSON-RPC response (e.g. stray logs).
            let Ok(msg) = serde_json::from_str::<Value>(&line) else {
                continue;
            };
            if msg.get("id").and_then(Value::as_u64) != Some(id) {
                continue;
            }
            if let Some(err) = msg.get("error") {
                let message = err.get("message").and_then(Value::as_str).unwrap_or("");
                bail!("agent runtime `{method}` failed: {message}");
            }
            return msg
                .get("result")
                .cloned()
                .ok_or_else(|| anyhow!("agent runtime `{method}` returned no result"));
        }
    }

    async fn call_text(&self, method: &str, prompt: &str, cwd: &Path) -> Result<String> {
        let result = self
            .call(method, json!({"prompt": prompt, "cwd": cwd}))
            .await?;
        result
            .get("text")
            .and_then(Value::as_str)
            .map(String::from)
            .ok_or_else(|| anyhow!("agent runtime `{method}` returned no text"))
    }
}

#[async_trait]
impl AgentRuntime for SidecarRuntime {
    async fn run_model(&self, prompt: &str, cwd: &Path) -> Result<String> {
        self.call_text("run_model", prompt, cwd).await
    }

    async fn run_surface(&self, tasks: &[SurfaceTask], cwd: &Path) -> Result<Vec<SurfaceOutcome>> {
        let result = self
            .call("run_surface", json!({"tasks": tasks, "cwd": cwd}))
            .await?;
        let results = result
            .get("results")
            .and_then(Value::as_array)
            .ok_or_else(|| anyhow!("agent runtime `run_surface` returned no results"))?;
        Ok(results
            .iter()
            .map(|r| {
                let id = r
                    .get("id")
                    .and_then(Value::as_str)
                    .unwrap_or("")
                    .to_string();
                let result = match r.get("text").and_then(Value::as_str) {
                    Some(text) => Ok(text.to_string()),
                    None => Err(r
                        .get("error")
                        .and_then(Value::as_str)
                        .unwrap_or("no output")
                        .to_string()),
                };
                SurfaceOutcome { id, result }
            })
            .collect())
    }

    async fn run_verify(&self, prompt: &str, cwd: &Path) -> Result<String> {
        self.call_text("run_verify", prompt, cwd).await
    }
}

/// Extract the JSON object from an agent's final answer, tolerating code
/// fences or prose around it.
pub fn extract_json(text: &str) -> Option<&str> {
    let start = text.find('{')?;
    let end = text.rfind('}')?;
    (start < end).then(|| &text[start..=end])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_json_strips_fences_and_prose() {
        assert_eq!(
            extract_json("Here:\n```json\n{\"a\": {\"b\": 1}}\n```\n"),
            Some("{\"a\": {\"b\": 1}}")
        );
        assert_eq!(extract_json("no json"), None);
    }

    /// A shell script standing in for the sidecar exercises the stdio
    /// JSON-RPC framing end to end.
    #[cfg(unix)]
    #[tokio::test]
    async fn sidecar_round_trip() {
        let script = r#"
read -r line
echo 'log line'
echo '{"jsonrpc":"2.0","id":1,"result":{"text":"{\"ok\":true}"}}'
read -r line
echo '{"jsonrpc":"2.0","id":2,"result":{"results":[{"id":"S-1","text":"t"},{"id":"S-2","error":"timeout"}]}}'
read -r line
echo '{"jsonrpc":"2.0","id":3,"error":{"code":-32000,"message":"boom"}}'
"#;
        let rt = SidecarRuntime::spawn(
            Path::new("sh"),
            &["-c".to_string(), script.to_string()],
            "claude",
        )
        .unwrap();
        let cwd = Path::new(".");
        assert_eq!(rt.run_model("p", cwd).await.unwrap(), "{\"ok\":true}");
        let tasks = vec![
            SurfaceTask {
                id: "S-1".into(),
                prompt: "a".into(),
            },
            SurfaceTask {
                id: "S-2".into(),
                prompt: "b".into(),
            },
        ];
        let out = rt.run_surface(&tasks, cwd).await.unwrap();
        assert_eq!(out[0].result, Ok("t".to_string()));
        assert_eq!(out[1].result, Err("timeout".to_string()));
        let err = rt.run_verify("v", cwd).await.unwrap_err();
        assert!(err.to_string().contains("boom"));
    }
}
