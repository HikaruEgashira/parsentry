//! Streaming message types for Codex CLI JSONL output.
//!
//! This module provides types and traits for handling real-time streaming
//! output from Codex CLI using `--json` format.

use serde::{Deserialize, Serialize};

/// Types of streaming messages from Codex CLI (JSONL format)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StreamMessage {
    /// Initial session information
    #[serde(rename = "system")]
    System(SystemMessage),
    /// Assistant response (may be partial)
    #[serde(rename = "assistant")]
    Assistant(AssistantMessage),
    /// Final result
    #[serde(rename = "result")]
    Result(ResultMessage),
}

/// System/progress message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMessage {
    #[serde(default)]
    pub subtype: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub session_id: Option<String>,
}

/// Assistant response message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantMessage {
    #[serde(default)]
    pub message: Option<MessageContent>,
}

/// Message content with role and content blocks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageContent {
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub content: Vec<ContentBlock>,
    #[serde(default)]
    pub stop_reason: Option<String>,
}

/// Content block types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentBlock {
    /// Text content
    Text {
        text: String,
    },
    /// Tool use request
    ToolUse {
        id: String,
        name: String,
        #[serde(default)]
        input: serde_json::Value,
    },
    /// Tool result
    ToolResult {
        #[serde(default)]
        tool_use_id: Option<String>,
        #[serde(default)]
        content: Option<String>,
        #[serde(default)]
        is_error: Option<bool>,
    },
}

/// Final result message (matching Claude Code structure)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResultMessage {
    #[serde(default)]
    pub subtype: Option<String>,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default, rename = "cost_usd")]
    pub cost_usd: Option<f64>,
    #[serde(default)]
    pub duration_ms: Option<u64>,
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub duration_api_ms: Option<u64>,
    #[serde(default)]
    pub is_error: Option<bool>,
    #[serde(default)]
    pub total_cost_usd: Option<f64>,
    #[serde(default)]
    pub num_turns: Option<u32>,
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
    fn test_parse_system_message() {
        let json = r#"{"type":"system","subtype":"init","session_id":"abc123"}"#;
        let msg: StreamMessage = serde_json::from_str(json).unwrap();
        match msg {
            StreamMessage::System(system) => {
                assert_eq!(system.session_id, Some("abc123".to_string()));
                assert_eq!(system.subtype, Some("init".to_string()));
            }
            _ => panic!("Expected System message"),
        }
    }

    #[test]
    fn test_parse_assistant_message_with_text() {
        let json = r#"{"type":"assistant","message":{"role":"assistant","content":[{"type":"text","text":"Hello"}]}}"#;
        let msg: StreamMessage = serde_json::from_str(json).unwrap();
        match msg {
            StreamMessage::Assistant(assistant) => {
                let message = assistant.message.unwrap();
                assert_eq!(message.content.len(), 1);
                match &message.content[0] {
                    ContentBlock::Text { text } => assert_eq!(text, "Hello"),
                    _ => panic!("Expected Text content"),
                }
            }
            _ => panic!("Expected Assistant message"),
        }
    }

    #[test]
    fn test_parse_result_message() {
        let json = r#"{"type":"result","subtype":"success","result":"{\"analysis\":\"test\"}","cost_usd":0.01,"duration_ms":1500,"total_cost_usd":0.02}"#;
        let msg: StreamMessage = serde_json::from_str(json).unwrap();
        match msg {
            StreamMessage::Result(result) => {
                assert_eq!(result.cost_usd, Some(0.01));
                assert_eq!(result.duration_ms, Some(1500));
                assert_eq!(result.total_cost_usd, Some(0.02));
                assert_eq!(result.subtype, Some("success".to_string()));
            }
            _ => panic!("Expected Result message"),
        }
    }

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
