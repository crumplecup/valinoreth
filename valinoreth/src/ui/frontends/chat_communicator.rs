//! [`ChatCommunicator`] — [`ElicitCommunicator`] for human players in the ratatui TUI.
//!
//! When the elicitation runtime calls [`send_prompt`], this communicator:
//!
//! 1. Posts the prompt text to the shared chat log (via `prompt_tx`).
//! 2. Signals the event loop to enter compose mode (via `compose_tx`).
//! 3. Awaits the user's reply on `reply_rx`.
//!
//! The event loop drives the other end: it drains `prompt_tx` into [`ChatModel`],
//! begins compose mode on the compose signal, and sends the user's typed text
//! back through `reply_tx`.
//!
//! All human slots in a session share the same `reply_rx` — turn-based combat
//! guarantees only one slot awaits at a time.
//!
//! [`send_prompt`]: ElicitCommunicator::send_prompt
//! [`ChatModel`]: crate::ChatModel

use std::sync::Arc;

use elicitation::{
    ElicitCommunicator, ElicitError, ElicitErrorKind, ElicitResult, ElicitationContext,
    StyleContext, StyleMarker,
};
use tokio::sync::{mpsc, Mutex};
use tracing::{debug, instrument, warn};

use crate::{ChatMessage, ChatSender};

/// [`ElicitCommunicator`] for human players that routes through the ratatui chat UI.
#[derive(Clone)]
pub struct ChatCommunicator {
    prompt_tx: mpsc::UnboundedSender<ChatMessage>,
    compose_tx: mpsc::UnboundedSender<()>,
    reply_rx: Arc<Mutex<mpsc::UnboundedReceiver<String>>>,
    style_ctx: StyleContext,
    elicit_ctx: ElicitationContext,
}

impl ChatCommunicator {
    /// Creates a new chat communicator.
    ///
    /// - `prompt_tx` — sender to the chat log channel (shared with GM narration).
    /// - `compose_tx` — signals the event loop to enter compose mode.
    /// - `reply_rx` — receives the user's typed reply from the event loop.
    pub fn new(
        prompt_tx: mpsc::UnboundedSender<ChatMessage>,
        compose_tx: mpsc::UnboundedSender<()>,
        reply_rx: Arc<Mutex<mpsc::UnboundedReceiver<String>>>,
    ) -> Self {
        Self {
            prompt_tx,
            compose_tx,
            reply_rx,
            style_ctx: StyleContext::default(),
            elicit_ctx: ElicitationContext::default(),
        }
    }
}

impl ElicitCommunicator for ChatCommunicator {
    /// Post the prompt to the chat log, trigger compose mode, and await a reply.
    ///
    /// # Errors
    ///
    /// Returns [`ElicitError`] if the chat or reply channel is closed.
    #[instrument(skip(self), level = "debug", fields(prompt_len = prompt.len()))]
    fn send_prompt(
        &self,
        prompt: &str,
    ) -> impl std::future::Future<Output = ElicitResult<String>> + Send {
        let prompt_tx = self.prompt_tx.clone();
        let compose_tx = self.compose_tx.clone();
        let reply_rx = self.reply_rx.clone();
        let prompt_text = prompt.to_string();

        async move {
            debug!("ChatCommunicator: posting prompt to chat log");
            if prompt_tx
                .send(ChatMessage::new(ChatSender::GameMaster, &prompt_text))
                .is_err()
            {
                warn!("ChatCommunicator: chat channel closed before prompt could be sent");
                return Err(ElicitError::new(ElicitErrorKind::ParseError(
                    "Chat channel closed".to_string(),
                )));
            }

            if compose_tx.send(()).is_err() {
                warn!("ChatCommunicator: compose signal channel closed");
            }

            debug!("ChatCommunicator: awaiting user reply");
            let reply = reply_rx.lock().await.recv().await;
            match reply {
                Some(text) => {
                    debug!(reply = %text, "ChatCommunicator: reply received");
                    Ok(text)
                }
                None => {
                    warn!("ChatCommunicator: reply channel closed while awaiting response");
                    Err(ElicitError::new(ElicitErrorKind::ParseError(
                        "Reply channel closed".to_string(),
                    )))
                }
            }
        }
    }

    #[instrument(skip(self, _params), level = "debug")]
    fn call_tool(
        &self,
        _params: rmcp::model::CallToolRequestParams,
    ) -> impl std::future::Future<
        Output = Result<rmcp::model::CallToolResult, rmcp::service::ServiceError>,
    > + Send {
        async move {
            warn!("call_tool not supported on ChatCommunicator");
            Err(rmcp::service::ServiceError::Cancelled {
                reason: Some("ChatCommunicator does not support MCP tool calls".to_string()),
            })
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
