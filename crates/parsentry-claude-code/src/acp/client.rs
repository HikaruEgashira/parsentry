//! ACP Client implementation for Parsentry.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use agent_client_protocol as acp;
use tracing::{debug, info, warn};

/// ACP Client implementation for Parsentry.
///
/// This client handles requests from the Claude Code agent,
/// such as file reads/writes and permission requests.
pub struct ParsentryAcpClient {
    /// Working directory for file operations.
    working_dir: PathBuf,
    /// Output directory for SARIF files (optional).
    output_dir: Option<PathBuf>,
    /// Collected response text from agent messages.
    response_buffer: Arc<Mutex<String>>,
}

impl ParsentryAcpClient {
    /// Create a new Parsentry ACP client.
    pub fn new(working_dir: PathBuf) -> Self {
        Self {
            working_dir,
            output_dir: None,
            response_buffer: Arc::new(Mutex::new(String::new())),
        }
    }

    /// Set the output directory for write operations.
    pub fn with_output_dir(mut self, output_dir: PathBuf) -> Self {
        self.output_dir = Some(output_dir);
        self
    }

    /// Get the collected response and clear the buffer.
    pub fn take_response(&self) -> String {
        let mut buffer = self.response_buffer.lock().unwrap();
        std::mem::take(&mut *buffer)
    }

    /// Get the response buffer for sharing.
    pub fn response_buffer(&self) -> Arc<Mutex<String>> {
        Arc::clone(&self.response_buffer)
    }
}

#[async_trait::async_trait(?Send)]
impl acp::Client for ParsentryAcpClient {
    /// Handle permission requests from the agent.
    async fn request_permission(
        &self,
        request: acp::RequestPermissionRequest,
    ) -> acp::Result<acp::RequestPermissionResponse> {
        debug!("Agent requested permission: {:?}", request);

        // Auto-approve by selecting the first option
        if let Some(option) = request.options.first() {
            Ok(acp::RequestPermissionResponse::new(
                acp::RequestPermissionOutcome::Selected(
                    acp::SelectedPermissionOutcome::new(option.option_id.clone()),
                ),
            ))
        } else {
            // No options provided, cannot grant permission
            Err(acp::Error::method_not_found())
        }
    }

    /// Handle file read requests from the agent.
    async fn read_text_file(
        &self,
        request: acp::ReadTextFileRequest,
    ) -> acp::Result<acp::ReadTextFileResponse> {
        let path = &request.path;
        debug!("Agent requested to read file: {}", path.display());

        if !path.starts_with(&self.working_dir) {
            warn!(
                "File read denied (outside working directory): {}",
                path.display()
            );
            return Ok(acp::ReadTextFileResponse::new(String::new()));
        }

        match tokio::fs::read_to_string(path).await {
            Ok(content) => {
                debug!(
                    "File read successful: {} ({} bytes)",
                    path.display(),
                    content.len()
                );
                Ok(acp::ReadTextFileResponse::new(content))
            }
            Err(e) => {
                warn!("File read failed: {} - {}", path.display(), e);
                Ok(acp::ReadTextFileResponse::new(String::new()))
            }
        }
    }

    /// Handle file write requests from the agent.
    async fn write_text_file(
        &self,
        request: acp::WriteTextFileRequest,
    ) -> acp::Result<acp::WriteTextFileResponse> {
        let path = &request.path;
        debug!("Agent requested to write file: {}", path.display());

        // Only allow writes to output directory
        let allowed = if let Some(ref output_dir) = self.output_dir {
            path.starts_with(output_dir)
        } else {
            // If no output_dir, allow writes in working_dir
            path.starts_with(&self.working_dir)
        };

        if !allowed {
            warn!(
                "File write denied (outside allowed directories): {}",
                path.display()
            );
            return Ok(acp::WriteTextFileResponse::new());
        }

        // Create parent directories if needed
        if let Some(parent) = path.parent() {
            if let Err(e) = tokio::fs::create_dir_all(parent).await {
                warn!("Failed to create parent directories: {}", e);
                return Ok(acp::WriteTextFileResponse::new());
            }
        }

        match tokio::fs::write(path, &request.content).await {
            Ok(()) => {
                info!("File written successfully: {}", path.display());
                Ok(acp::WriteTextFileResponse::new())
            }
            Err(e) => {
                warn!("File write failed: {} - {}", path.display(), e);
                Ok(acp::WriteTextFileResponse::new())
            }
        }
    }

    /// Handle terminal creation requests (not supported).
    async fn create_terminal(
        &self,
        _request: acp::CreateTerminalRequest,
    ) -> acp::Result<acp::CreateTerminalResponse> {
        Err(acp::Error::method_not_found())
    }

    /// Handle terminal output requests (not supported).
    async fn terminal_output(
        &self,
        _request: acp::TerminalOutputRequest,
    ) -> acp::Result<acp::TerminalOutputResponse> {
        Err(acp::Error::method_not_found())
    }

    /// Handle terminal release requests (not supported).
    async fn release_terminal(
        &self,
        _request: acp::ReleaseTerminalRequest,
    ) -> acp::Result<acp::ReleaseTerminalResponse> {
        Err(acp::Error::method_not_found())
    }

    /// Handle wait for terminal exit requests (not supported).
    async fn wait_for_terminal_exit(
        &self,
        _request: acp::WaitForTerminalExitRequest,
    ) -> acp::Result<acp::WaitForTerminalExitResponse> {
        Err(acp::Error::method_not_found())
    }

    /// Handle terminal kill requests (not supported).
    async fn kill_terminal_command(
        &self,
        _request: acp::KillTerminalCommandRequest,
    ) -> acp::Result<acp::KillTerminalCommandResponse> {
        Err(acp::Error::method_not_found())
    }

    /// Handle session notifications from the agent.
    /// This is where we collect agent response text.
    async fn session_notification(
        &self,
        notification: acp::SessionNotification,
    ) -> acp::Result<()> {
        match notification.update {
            acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk { content, .. }) => {
                let text = match content {
                    acp::ContentBlock::Text(text_content) => text_content.text,
                    acp::ContentBlock::Image(_) => "<image>".into(),
                    acp::ContentBlock::Audio(_) => "<audio>".into(),
                    acp::ContentBlock::ResourceLink(link) => link.uri,
                    acp::ContentBlock::Resource(_) => "<resource>".into(),
                    _ => "<unknown>".into(),
                };
                // Accumulate text in buffer
                let mut buffer = self.response_buffer.lock().unwrap();
                buffer.push_str(&text);
            }
            _ => {
                debug!("Session update: {:?}", notification.update);
            }
        }
        Ok(())
    }

    /// Handle extension method calls (not supported).
    async fn ext_method(&self, _request: acp::ExtRequest) -> acp::Result<acp::ExtResponse> {
        Err(acp::Error::method_not_found())
    }

    /// Handle extension notifications (not supported).
    async fn ext_notification(&self, _notification: acp::ExtNotification) -> acp::Result<()> {
        Err(acp::Error::method_not_found())
    }
}
