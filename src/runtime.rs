//! Agent runtimes that execute phase prompts in-process instead of piping
//! them to an external CLI agent.
//!
//! [`SidecarRuntime`] launches a sidecar executable and speaks line-delimited
//! JSON-RPC 2.0 over its stdio. Every request carries the prompt for one
//! phase; the sidecar answers with the raw artifact (threat model JSON or
//! SARIF). The runtime never writes cache files itself — the caller
//! validates the returned text and owns the cache layout.

use anyhow::{Context, Result, anyhow, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

/// Environment variable naming the sidecar executable.
pub const RUNTIME_ENV: &str = "PARSENTRY_AGENT_RUNTIME";
/// Sidecar looked up on `PATH` when [`RUNTIME_ENV`] is unset.
pub const DEFAULT_SIDECAR: &str = "parsentry-agent-runtime";
/// Environment variable selecting the sidecar backend (`claude` or `openai`).
pub const BACKEND_ENV: &str = "PARSENTRY_AGENT_BACKEND";

/// Executes phase prompts and returns the produced artifact as text.
pub trait AgentRuntime {
    /// Run the threat model prompt; returns threat model JSON.
    fn run_model(&mut self, prompt: &str) -> Result<String>;
    /// Run one surface prompt; returns SARIF JSON.
    fn run_surface(&mut self, surface_id: &str, prompt: &str) -> Result<String>;
    /// Verify a surface's SARIF findings; returns (possibly revised) SARIF JSON.
    fn run_verify(&mut self, surface_id: &str, sarif: &str) -> Result<String>;
}

#[derive(Serialize)]
struct Request<'a> {
    jsonrpc: &'static str,
    id: u64,
    method: &'a str,
    params: Value,
}

#[derive(Deserialize)]
struct Response {
    #[serde(default)]
    id: Option<Value>,
    #[serde(default)]
    result: Option<Value>,
    #[serde(default)]
    error: Option<RpcError>,
}

#[derive(Deserialize)]
struct RpcError {
    code: i64,
    message: String,
}

/// JSON-RPC 2.0 client for a sidecar process over stdio.
pub struct SidecarRuntime {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    next_id: u64,
}

impl SidecarRuntime {
    /// Launch the sidecar from `PARSENTRY_AGENT_RUNTIME`, else `parsentry-agent-runtime` on PATH,
    /// selecting `backend` via `PARSENTRY_AGENT_BACKEND`.
    pub fn from_env(backend: &str) -> Result<Self> {
        let program = std::env::var_os(RUNTIME_ENV)
            .filter(|v| !v.is_empty())
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(DEFAULT_SIDECAR));
        let mut command = Command::new(&program);
        command.env(BACKEND_ENV, backend);
        Self::spawn(command)
            .with_context(|| format!("failed to launch agent runtime {}", program.display()))
    }

    /// Launch `command` as a sidecar with piped stdin/stdout.
    pub fn spawn(mut command: Command) -> Result<Self> {
        let mut child = command
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()?;
        let stdin = child.stdin.take().context("sidecar stdin unavailable")?;
        let stdout = BufReader::new(child.stdout.take().context("sidecar stdout unavailable")?);
        Ok(Self {
            child,
            stdin,
            stdout,
            next_id: 1,
        })
    }

    /// Send one request and wait for the response with the matching id.
    pub fn call(&mut self, method: &str, params: Value) -> Result<Value> {
        let id = self.next_id;
        self.next_id += 1;
        let mut line = serde_json::to_string(&Request {
            jsonrpc: "2.0",
            id,
            method,
            params,
        })?;
        line.push('\n');
        self.stdin
            .write_all(line.as_bytes())
            .and_then(|_| self.stdin.flush())
            .context("failed to write to agent runtime")?;

        loop {
            let mut buf = String::new();
            if self.stdout.read_line(&mut buf)? == 0 {
                bail!("agent runtime exited before answering {method}");
            }
            if buf.trim().is_empty() {
                continue;
            }
            let resp: Response = serde_json::from_str(buf.trim())
                .with_context(|| format!("invalid JSON-RPC response: {}", buf.trim()))?;
            // Skip notifications and responses to other requests.
            if resp.id.as_ref().and_then(Value::as_u64) != Some(id) {
                continue;
            }
            if let Some(err) = resp.error {
                bail!(
                    "agent runtime {method} failed ({}): {}",
                    err.code,
                    err.message
                );
            }
            return resp
                .result
                .ok_or_else(|| anyhow!("agent runtime {method} returned no result"));
        }
    }

    /// Call `method` and extract the artifact from `result.output` (string or JSON value).
    fn call_output(&mut self, method: &str, params: Value) -> Result<String> {
        let result = self.call(method, params)?;
        match result.get("output").unwrap_or(&result) {
            Value::String(s) => Ok(s.clone()),
            other => Ok(serde_json::to_string(other)?),
        }
    }
}

impl AgentRuntime for SidecarRuntime {
    fn run_model(&mut self, prompt: &str) -> Result<String> {
        self.call_output("run_model", serde_json::json!({ "prompt": prompt }))
    }

    fn run_surface(&mut self, surface_id: &str, prompt: &str) -> Result<String> {
        self.call_output(
            "run_surface",
            serde_json::json!({ "surface_id": surface_id, "prompt": prompt }),
        )
    }

    fn run_verify(&mut self, surface_id: &str, sarif: &str) -> Result<String> {
        self.call_output(
            "run_verify",
            serde_json::json!({ "surface_id": surface_id, "sarif": sarif }),
        )
    }
}

impl Drop for SidecarRuntime {
    fn drop(&mut self) {
        // Sidecars are per-scan; never leave one running behind.
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fake_sidecar(script: &str) -> SidecarRuntime {
        let mut cmd = Command::new("sh");
        cmd.arg("-c").arg(script);
        SidecarRuntime::spawn(cmd).unwrap()
    }

    /// Echoes the method name back as the output, preceded by a notification.
    const ECHO: &str = r#"while IFS= read -r line; do
  id=$(printf '%s' "$line" | sed -n 's/.*"id":\([0-9]*\).*/\1/p')
  method=$(printf '%s' "$line" | sed -n 's/.*"method":"\([a-z_]*\)".*/\1/p')
  echo '{"jsonrpc":"2.0","method":"log","params":{}}'
  printf '{"jsonrpc":"2.0","id":%s,"result":{"output":"%s"}}\n' "$id" "$method"
done"#;

    #[test]
    fn round_trip_with_sh_sidecar() {
        let mut rt = fake_sidecar(ECHO);
        assert_eq!(rt.run_model("p").unwrap(), "run_model");
        assert_eq!(rt.run_surface("S-1", "p").unwrap(), "run_surface");
        assert_eq!(rt.run_verify("S-1", "{}").unwrap(), "run_verify");
    }

    #[test]
    fn json_output_value_is_serialized() {
        let mut rt = fake_sidecar(
            r#"read -r line; echo '{"jsonrpc":"2.0","id":1,"result":{"output":{"a":1}}}'"#,
        );
        assert_eq!(rt.run_model("p").unwrap(), r#"{"a":1}"#);
    }

    #[test]
    fn rpc_error_is_reported() {
        let mut rt = fake_sidecar(
            r#"read -r line; echo '{"jsonrpc":"2.0","id":1,"error":{"code":-32000,"message":"boom"}}'"#,
        );
        let err = rt.run_model("p").unwrap_err().to_string();
        assert!(err.contains("boom"), "{err}");
    }

    #[test]
    fn early_exit_is_an_error() {
        let mut rt = fake_sidecar("exit 0");
        assert!(rt.run_model("p").is_err());
    }
}
