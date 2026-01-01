//! ACP connection management for Claude Code.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use agent_client_protocol::{self as acp, Agent as _};
use anyhow::{Context, Result};
use tokio::process::{Child, Command};
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};
use tracing::{debug, info};

use super::ParsentryAcpClient;

/// ACP connection to Claude Code agent.
pub struct AcpConnection {
    /// The child process running Claude Code.
    child: Child,
    /// The client-side connection for ACP communication.
    connection: acp::ClientSideConnection,
    /// Current session ID (if session is active).
    session_id: Option<acp::SessionId>,
    /// Working directory for the connection.
    working_dir: PathBuf,
    /// Shared response buffer for collecting agent messages.
    response_buffer: Arc<Mutex<String>>,
}

impl AcpConnection {
    /// Spawn a new Claude Code ACP process.
    pub async fn spawn(
        claude_path: &Path,
        working_dir: &Path,
        model: Option<&str>,
    ) -> Result<Self> {
        let mut cmd = Command::new(claude_path);

        // claude-code-acp does not require --acp flag (it's already ACP mode)
        // Only pass model if specified
        if let Some(model) = model {
            cmd.arg("--model").arg(model);
        }

        cmd.current_dir(working_dir)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .kill_on_drop(true);

        debug!("Spawning Claude Code process with ACP: {:?}", cmd);

        let mut child = cmd.spawn().context("Failed to spawn Claude Code process")?;

        let stdin = child.stdin.take().context("Failed to get stdin")?;
        let stdout = child.stdout.take().context("Failed to get stdout")?;

        // Create ACP client and get response buffer reference
        let client = ParsentryAcpClient::new(working_dir.to_path_buf());
        let response_buffer = client.response_buffer();

        // Create client-side connection using tokio-util compat layer
        let outgoing = stdin.compat_write();
        let incoming = stdout.compat();

        let (connection, handle_io) =
            acp::ClientSideConnection::new(client, outgoing, incoming, |fut| {
                tokio::task::spawn_local(fut);
            });

        // Handle I/O in the background
        tokio::task::spawn_local(handle_io);

        info!("Claude Code ACP process spawned successfully");

        Ok(Self {
            child,
            connection,
            session_id: None,
            working_dir: working_dir.to_path_buf(),
            response_buffer,
        })
    }

    /// Initialize the ACP connection.
    pub async fn initialize(&mut self) -> Result<()> {
        let mut request = acp::InitializeRequest::new(1.into());
        request.client_info = Some(acp::Implementation::new(
            "parsentry",
            env!("CARGO_PKG_VERSION"),
        ));

        let response = self
            .connection
            .initialize(request)
            .await
            .context("Failed to initialize ACP connection")?;

        info!(
            "ACP initialized with protocol version {}",
            response.protocol_version
        );

        Ok(())
    }

    /// Create a new session.
    pub async fn new_session(&mut self) -> Result<acp::SessionId> {
        let request = acp::NewSessionRequest::new(self.working_dir.to_string_lossy().to_string());

        let response = self
            .connection
            .new_session(request)
            .await
            .context("Failed to create new session")?;

        let session_id = response.session_id;
        self.session_id = Some(session_id.clone());

        info!("ACP session created: {:?}", session_id);

        Ok(session_id)
    }

    /// Send a prompt and get a response.
    /// Returns the accumulated text from agent messages.
    pub async fn prompt(&mut self, message: &str) -> Result<String> {
        let session_id = self
            .session_id
            .clone()
            .context("No active session. Call new_session() first.")?;

        // Clear the response buffer before sending prompt
        {
            let mut buffer = self.response_buffer.lock().unwrap();
            buffer.clear();
        }

        let content = vec![acp::ContentBlock::Text(acp::TextContent::new(message))];
        let request = acp::PromptRequest::new(session_id, content);

        // Send prompt and wait for completion
        let _response = self
            .connection
            .prompt(request)
            .await
            .context("Failed to send prompt")?;

        // Get accumulated response from buffer
        let response_text = {
            let mut buffer = self.response_buffer.lock().unwrap();
            std::mem::take(&mut *buffer)
        };

        Ok(response_text)
    }

    /// Get the current session ID.
    pub fn session_id(&self) -> Option<&acp::SessionId> {
        self.session_id.as_ref()
    }

    /// Close the connection and terminate the process.
    pub async fn close(&mut self) -> Result<()> {
        self.session_id = None;
        self.child.kill().await.ok();
        info!("ACP connection closed");
        Ok(())
    }
}
