//! ACP (Agent Client Protocol) implementation for Claude Code communication.
//!
//! This module provides ACP-compliant communication with Claude Code,
//! replacing the previous CLI-based approach.

mod client;
mod connection;

pub use client::ParsentryAcpClient;
pub use connection::AcpConnection;
