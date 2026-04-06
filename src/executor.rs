//! Parallel CLI agent executor with semaphore-controlled concurrency.
//!
//! Passes prompts to a CLI agent (e.g. `claude -p "<prompt>"`) and reads
//! results from output files specified in the prompt. The agent is expected
//! to write its output to a file, not stdout.

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{Context, Result};
use futures::stream::{self, StreamExt};
use tokio::sync::Semaphore;

use crate::cli::ui::StatusPrinter;

/// Result of a single agent execution.
#[derive(Debug)]
pub struct ExecutionResult {
    /// Identifier for the attack surface that was analyzed.
    pub surface_id: String,
    /// Path to the output file written by the agent.
    pub output_path: PathBuf,
    /// The content read from the output file (empty if file not found).
    pub output: String,
    /// Whether the execution completed successfully and produced output.
    pub success: bool,
    /// Whether the result was served from cache.
    pub cached: bool,
}

/// Executor that dispatches prompts to a CLI agent in parallel.
pub struct AgentExecutor {
    agent_path: PathBuf,
    semaphore: Arc<Semaphore>,
    timeout: Duration,
}

impl AgentExecutor {
    /// Create a new executor.
    ///
    /// * `agent_path` - Path to the agent binary. Defaults to `"claude"` if `None`.
    /// * `max_concurrent` - Maximum number of concurrent agent processes.
    /// * `timeout_secs` - Per-task timeout in seconds.
    pub fn new(agent_path: Option<PathBuf>, max_concurrent: usize, timeout_secs: u64) -> Self {
        let path = agent_path.unwrap_or_else(|| PathBuf::from("claude"));
        Self {
            agent_path: path,
            semaphore: Arc::new(Semaphore::new(max_concurrent)),
            timeout: Duration::from_secs(timeout_secs),
        }
    }

    /// Execute a single prompt by passing it to the agent CLI via `-p` flag.
    ///
    /// The agent is expected to write its output to `output_path` (specified
    /// in the prompt text). After execution, the result is read from that file.
    pub async fn execute_one(
        &self,
        surface_id: &str,
        prompt: &str,
        output_path: &std::path::Path,
    ) -> Result<ExecutionResult> {
        let _permit = self
            .semaphore
            .acquire()
            .await
            .context("semaphore closed unexpectedly")?;

        // Ensure output directory exists
        if let Some(parent) = output_path.parent() {
            tokio::fs::create_dir_all(parent).await.ok();
        }

        let child = tokio::process::Command::new(&self.agent_path)
            .arg("-p")
            .arg(prompt)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .with_context(|| {
                format!(
                    "failed to spawn agent binary '{}'. Is it installed and on PATH?",
                    self.agent_path.display()
                )
            })?;

        // Wait for process with timeout.
        let proc_output = tokio::time::timeout(self.timeout, child.wait_with_output())
            .await
            .with_context(|| {
                format!(
                    "agent timed out after {}s for surface '{}'",
                    self.timeout.as_secs(),
                    surface_id
                )
            })?
            .with_context(|| {
                format!("agent process failed for surface '{}'", surface_id)
            })?;

        let exit_success = proc_output.status.success();

        if !exit_success {
            let stderr = String::from_utf8_lossy(&proc_output.stderr);
            tracing::warn!(
                surface_id = surface_id,
                exit_code = ?proc_output.status.code(),
                stderr = %stderr,
                "agent returned non-zero exit code"
            );
        }

        // Read the output file written by the agent
        let output = if output_path.exists() {
            tokio::fs::read_to_string(output_path)
                .await
                .unwrap_or_default()
        } else {
            tracing::warn!(
                surface_id = surface_id,
                path = %output_path.display(),
                "agent did not write output file"
            );
            String::new()
        };

        let has_output = output_path.exists() && !output.is_empty();

        Ok(ExecutionResult {
            surface_id: surface_id.to_string(),
            output_path: output_path.to_path_buf(),
            output,
            success: exit_success && has_output,
            cached: false,
        })
    }

    /// Execute multiple prompts in parallel with semaphore-controlled concurrency.
    ///
    /// Each task is a `(surface_id, prompt, output_path)` tuple. Progress is
    /// reported to stderr via [`StatusPrinter`].
    pub async fn execute_all(
        &self,
        tasks: Vec<(String, String, PathBuf)>,
    ) -> Vec<ExecutionResult> {
        let total = tasks.len();
        let printer = StatusPrinter::new();
        let counter = Arc::new(std::sync::atomic::AtomicUsize::new(0));

        printer.status("Executing", &format!("{} tasks with agent", total));

        let results: Vec<ExecutionResult> = stream::iter(tasks)
            .map(|(surface_id, prompt, output_path)| {
                let counter = Arc::clone(&counter);
                let printer = StatusPrinter::new();
                let total = total;
                async move {
                    let idx = counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
                    printer.info(
                        "Analyzing",
                        &format!("[{}/{}] {}", idx, total, &surface_id),
                    );

                    match self.execute_one(&surface_id, &prompt, &output_path).await {
                        Ok(result) => {
                            if result.success {
                                printer.success("Done", &surface_id);
                            } else {
                                printer.warning("Failed", &format!("{} (no output)", &surface_id));
                            }
                            result
                        }
                        Err(e) => {
                            printer.error("Error", &format!("{}: {}", &surface_id, e));
                            ExecutionResult {
                                surface_id,
                                output_path,
                                output: String::new(),
                                success: false,
                                cached: false,
                            }
                        }
                    }
                }
            })
            .buffer_unordered(total.max(1))
            .collect()
            .await;

        let succeeded = results.iter().filter(|r| r.success).count();
        let failed = results.len() - succeeded;

        printer.success(
            "Finished",
            &format!("{} succeeded, {} failed", succeeded, failed),
        );

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_execute_one_with_file_output() {
        let tmp = TempDir::new().unwrap();
        let output_path = tmp.path().join("test.sarif.json");

        // Pre-create the output file to simulate agent writing it
        std::fs::write(&output_path, r#"{"version":"2.1.0"}"#).unwrap();

        // Use `echo` as a no-op agent (exits 0, ignores -p flag)
        let executor = AgentExecutor::new(Some(PathBuf::from("echo")), 2, 10);
        let result = executor.execute_one("test-surface", "analyze this", &output_path).await.unwrap();

        assert!(result.success);
        assert_eq!(result.surface_id, "test-surface");
        assert!(!result.cached);
        assert!(result.output.contains("2.1.0"));
    }

    #[tokio::test]
    async fn test_execute_one_not_found() {
        let tmp = TempDir::new().unwrap();
        let output_path = tmp.path().join("out.json");
        let executor = AgentExecutor::new(
            Some(PathBuf::from("nonexistent-binary-xyz")),
            1,
            5,
        );
        let result = executor.execute_one("test", "prompt", &output_path).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_default_agent_path() {
        let executor = AgentExecutor::new(None, 1, 10);
        assert_eq!(executor.agent_path, PathBuf::from("claude"));
    }

    #[tokio::test]
    async fn test_execute_all_empty() {
        let executor = AgentExecutor::new(Some(PathBuf::from("echo")), 2, 10);
        let results = executor.execute_all(vec![]).await;
        assert!(results.is_empty());
    }

    #[tokio::test]
    async fn test_execute_all_with_failures() {
        let tmp = TempDir::new().unwrap();
        let executor = AgentExecutor::new(
            Some(PathBuf::from("nonexistent-binary-xyz")),
            2,
            5,
        );
        let tasks = vec![
            ("fail-1".to_string(), "prompt".to_string(), tmp.path().join("f1.json")),
            ("fail-2".to_string(), "prompt".to_string(), tmp.path().join("f2.json")),
        ];

        let results = executor.execute_all(tasks).await;
        assert_eq!(results.len(), 2);
        assert!(results.iter().all(|r| !r.success));
    }
}
