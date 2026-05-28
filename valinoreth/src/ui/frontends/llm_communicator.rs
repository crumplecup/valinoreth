//! [`LlmElicitCommunicator`] — [`ElicitCommunicator`] backed by a language-model.
//!
//! Converts every [`ElicitCommunicator::send_prompt`] call into a direct LLM
//! completion request via [`LlmClient`].  The agent replies to GURPS combat
//! prompts with a bare option number or label; the elicitation runtime validates
//! the response against the expected options.

use elicitation::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitResult, ElicitationContext,
    StyleContext, StyleMarker,
};
use rmcp::model::{CallToolResult, Content};
use tracing::{debug, info, instrument, warn};

use crate::{AgentConfig, LlmClient};

/// Fixed system prompt instructing the agent to act as a GURPS combat participant.
const DEFAULT_SYSTEM_PROMPT: &str = "\
You are an AI agent in a GURPS tactical combat simulation. You will be asked to make \
combat decisions (e.g. choose a maneuver, attack an opponent, or defend against an attack). \
Reply with ONLY the option number or option label — no explanation, no punctuation, nothing else. \
For example, if asked to choose between Attack and Defend, reply with '1' or 'Attack'.";

/// An [`ElicitCommunicator`] that sends prompts directly to an LLM.
///
/// Constructed from an [`AgentConfig`] which provides the LLM provider, model,
/// and API key (read from the environment).  Each [`send_prompt`] call translates
/// to one LLM completion request; the agent's response is returned as-is.
///
/// [`send_prompt`]: ElicitCommunicator::send_prompt
#[derive(Clone)]
pub struct LlmElicitCommunicator {
    client: LlmClient,
    agent_name: String,
    system_prompt: String,
    style_ctx: StyleContext,
    elicit_ctx: ElicitationContext,
}

impl LlmElicitCommunicator {
    /// Creates a new communicator from the given agent configuration.
    ///
    /// # Errors
    ///
    /// Returns an error string if the LLM config cannot be built (e.g. missing
    /// API key environment variable).
    #[instrument(skip(config), fields(agent_name = %config.name()))]
    pub fn new(config: &AgentConfig) -> Result<Self, String> {
        info!("Creating LlmElicitCommunicator");
        let llm_config = config.create_llm_config().map_err(|e| e.to_string())?;
        let client = LlmClient::new(llm_config);
        Ok(Self {
            client,
            agent_name: config.name().clone(),
            system_prompt: DEFAULT_SYSTEM_PROMPT.to_string(),
            style_ctx: StyleContext::default(),
            elicit_ctx: ElicitationContext::default(),
        })
    }

    /// Returns a clone with a custom system prompt.
    pub fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = prompt.into();
        self
    }
}

impl ElicitCommunicator for LlmElicitCommunicator {
    /// Send the prompt to the LLM and return its trimmed response.
    #[instrument(skip(self), fields(agent = %self.agent_name, prompt_len = prompt.len()))]
    fn send_prompt(
        &self,
        prompt: &str,
    ) -> impl std::future::Future<Output = ElicitResult<String>> + Send {
        let client = self.client.clone();
        let prompt_owned = prompt.to_string();
        let agent_name = self.agent_name.clone();
        let system_prompt = self.system_prompt.clone();

        async move {
            debug!(agent = %agent_name, "Sending prompt to LLM");
            let response = client
                .generate(&system_prompt, &prompt_owned)
                .await
                .map_err(|e| {
                    warn!(error = %e, agent = %agent_name, "LLM generation failed");
                    ElicitError::new(ElicitErrorKind::ParseError(format!(
                        "LLM error for agent {agent_name}: {e}"
                    )))
                })?;
            let trimmed = response.trim().to_string();
            info!(agent = %agent_name, response = %trimmed, "LLM response received");
            Ok(trimmed)
        }
    }

    /// Handle `elicit_select` tool calls by forwarding the choice to the LLM.
    ///
    /// Formats the options as a numbered list, sends it to the LLM via
    /// [`send_prompt`], then normalises the response back to an exact option
    /// label (handling both numeric `"1"` and label `"Dodge"` replies, case-
    /// insensitively).  All other tool names return `Cancelled`.
    ///
    /// [`send_prompt`]: ElicitCommunicator::send_prompt
    #[instrument(skip(self, params), level = "debug", fields(tool = %params.name))]
    fn call_tool(
        &self,
        params: rmcp::model::CallToolRequestParams,
    ) -> impl std::future::Future<
        Output = Result<rmcp::model::CallToolResult, rmcp::service::ServiceError>,
    > + Send {
        let client = self.client.clone();
        let agent_name = self.agent_name.clone();
        let system_prompt = self.system_prompt.clone();

        async move {
            if params.name.as_ref() != "elicit_select" {
                warn!(agent = %agent_name, tool = %params.name, "unsupported tool call");
                return Err(rmcp::service::ServiceError::Cancelled {
                    reason: Some(format!(
                        "LLM communicator does not support tool: {}",
                        params.name
                    )),
                });
            }

            let args = params.arguments.unwrap_or_default();

            let options: Vec<String> = args
                .get("options")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default();

            if options.is_empty() {
                return Err(rmcp::service::ServiceError::Cancelled {
                    reason: Some("elicit_select: no options provided".to_string()),
                });
            }

            let prompt_text = args
                .get("prompt")
                .and_then(|v| v.as_str())
                .unwrap_or("Choose an option:");

            let mut formatted = format!("{prompt_text}\n\nOptions:\n");
            for (i, opt) in options.iter().enumerate() {
                formatted.push_str(&format!("{}. {opt}\n", i + 1));
            }
            formatted.push_str("\nReply with ONLY the option label, nothing else.");

            debug!(agent = %agent_name, option_count = options.len(), "sending elicit_select to LLM");

            let response = client
                .generate(&system_prompt, &formatted)
                .await
                .map_err(|e| rmcp::service::ServiceError::Cancelled {
                    reason: Some(format!("LLM error for agent {agent_name}: {e}")),
                })?;

            let raw = response.trim().to_string();

            // Normalise: numeric index ("1") or label ("Dodge"), case-insensitive.
            let selected = if let Ok(n) = raw.parse::<usize>() {
                options
                    .get(n.saturating_sub(1))
                    .cloned()
                    .unwrap_or_else(|| raw.clone())
            } else {
                options
                    .iter()
                    .find(|o| o.to_lowercase() == raw.to_lowercase())
                    .cloned()
                    .unwrap_or_else(|| raw.clone())
            };

            info!(agent = %agent_name, selected = %selected, "LLM chose option via elicit_select");
            Ok(CallToolResult::success(vec![Content::text(selected)]))
        }
    }

    fn style_context(&self) -> &StyleContext {
        &self.style_ctx
    }

    fn elicitation_context(&self) -> &ElicitationContext {
        &self.elicit_ctx
    }

    fn with_style<T: 'static, S: StyleMarker + elicitation::style::ElicitationStyle + 'static>(
        &self,
        style: S,
    ) -> Self {
        let mut new = self.clone();
        new.style_ctx.set_style::<T, S>(style).ok();
        new
    }
}
