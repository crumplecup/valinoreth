//! [`CombatCommunicator`] — unified [`ElicitCommunicator`] for combat participants.
//!
//! [`CombatWorkflow`] is generic over a single communicator type `C`.  A combat
//! session may have both human players (driven by [`ChatCommunicator`]) and AI
//! agents (driven by [`LlmElicitCommunicator`]).  This enum satisfies the
//! single-type constraint by dispatching to the appropriate variant at runtime.
//!
//! [`CombatWorkflow`]: crate::CombatWorkflow

use std::future::Future;

use elicitation::{ElicitResult, ElicitationContext, StyleContext, StyleMarker};
use elicitation::ElicitCommunicator;

use crate::{ChatCommunicator, LlmElicitCommunicator};

/// A combat communicator that dispatches to either a human TUI or an LLM agent.
#[derive(Clone)]
pub enum CombatCommunicator {
    /// Human player controlled through the ratatui chat UI.
    Human(ChatCommunicator),
    /// AI agent controlled through direct LLM calls.
    Agent(LlmElicitCommunicator),
}

impl ElicitCommunicator for CombatCommunicator {
    fn send_prompt(
        &self,
        prompt: &str,
    ) -> impl Future<Output = ElicitResult<String>> + Send {
        let this = self.clone();
        let prompt = prompt.to_string();
        async move {
            match this {
                CombatCommunicator::Human(c) => c.send_prompt(&prompt).await,
                CombatCommunicator::Agent(c) => c.send_prompt(&prompt).await,
            }
        }
    }

    fn call_tool(
        &self,
        params: rmcp::model::CallToolRequestParams,
    ) -> impl Future<
        Output = Result<rmcp::model::CallToolResult, rmcp::service::ServiceError>,
    > + Send {
        let this = self.clone();
        async move {
            match this {
                CombatCommunicator::Human(c) => c.call_tool(params).await,
                CombatCommunicator::Agent(c) => c.call_tool(params).await,
            }
        }
    }

    fn style_context(&self) -> &StyleContext {
        match self {
            Self::Human(c) => c.style_context(),
            Self::Agent(c) => c.style_context(),
        }
    }

    fn elicitation_context(&self) -> &ElicitationContext {
        match self {
            Self::Human(c) => c.elicitation_context(),
            Self::Agent(c) => c.elicitation_context(),
        }
    }

    fn with_style<T: 'static, S: StyleMarker + elicitation::style::ElicitationStyle + 'static>(
        &self,
        style: S,
    ) -> Self {
        match self {
            Self::Human(c) => Self::Human(c.with_style::<T, S>(style)),
            Self::Agent(c) => Self::Agent(c.with_style::<T, S>(style)),
        }
    }
}
