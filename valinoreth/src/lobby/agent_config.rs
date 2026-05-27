//! Agent configuration for combat participants.
//!
//! [`AgentConfig`] is a stub for now — MCP server configuration will be added
//! when agent communicators are wired up.

use derive_getters::Getters;
use tracing::instrument;

/// Configuration for an AI agent participant in combat.
///
/// Currently a name-only stub.  Future fields will hold MCP server details and
/// LLM provider configuration once agent communicators are implemented.
#[derive(Debug, Clone, Getters)]
pub struct AgentConfig {
    /// Display name for this agent.
    name: String,
}

impl AgentConfig {
    /// Creates an agent configuration with the given display name.
    #[instrument(skip_all)]
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
