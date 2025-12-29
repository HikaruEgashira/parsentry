//! Streaming message types for Codex CLI JSONL output.
//!
//! This module provides types and traits for handling real-time streaming
//! output from Codex CLI using `--json` format.

use serde::{Deserialize, Serialize};

/// Types of streaming messages from Codex CLI (JSONL format)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum StreamMessage {
    /// Message event with content
    Message(MessageEvent),
    /// Generic JSON event
    Generic(serde_json::Value),
}

/// Message event from Codex
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageEvent {
    #[serde(default, rename = "type")]
    pub event_type: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub text: Option<String>,
    #[serde(default)]
    pub message: Option<MessageContent>,
}

/// Message content with role
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageContent {
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub content: Option<String>,
}

/// Result message from Codex execution
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResultMessage {
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub duration_ms: Option<u64>,
    #[serde(default)]
    pub is_error: Option<bool>,
}

/// Event types for streaming callbacks
#[derive(Debug, Clone)]
pub enum StreamEvent {
    /// New text content (partial or complete)
    Text(String),
    /// Tool being used
    ToolUse {
        name: String,
        input: serde_json::Value,
    },
    /// Tool finished
    ToolComplete {
        name: String,
        success: bool,
    },
    /// Progress/system message
    Progress(String),
    /// Final result available
    Complete(ResultMessage),
    /// Error occurred
    Error(String),
}

/// Callback trait for streaming events
pub trait StreamCallback: Send + Sync {
    /// Called when a streaming event occurs
    fn on_event(&self, event: StreamEvent);
}

/// Default no-op callback for backward compatibility
pub struct NoOpCallback;

impl StreamCallback for NoOpCallback {
    fn on_event(&self, _event: StreamEvent) {}
}

/// Callback that collects events into a channel
pub struct ChannelCallback {
    sender: tokio::sync::mpsc::UnboundedSender<StreamEvent>,
}

impl ChannelCallback {
    /// Create a new channel callback and receiver pair
    pub fn new() -> (Self, tokio::sync::mpsc::UnboundedReceiver<StreamEvent>) {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        (Self { sender }, receiver)
    }
}

impl Default for ChannelCallback {
    fn default() -> Self {
        Self::new().0
    }
}

impl StreamCallback for ChannelCallback {
    fn on_event(&self, event: StreamEvent) {
        let _ = self.sender.send(event);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_callback() {
        let (callback, mut receiver) = ChannelCallback::new();

        callback.on_event(StreamEvent::Text("Hello".to_string()));
        callback.on_event(StreamEvent::Progress("Working...".to_string()));

        // Events should be received
        assert!(receiver.try_recv().is_ok());
        assert!(receiver.try_recv().is_ok());
    }
}
