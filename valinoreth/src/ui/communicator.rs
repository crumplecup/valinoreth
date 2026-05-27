//! [`ObservableCommunicator`] — transparent middleware that routes elicitation
//! exchanges into the game chat log.
//!
//! # Design
//!
//! [`ObservableCommunicator`] wraps any inner [`ElicitCommunicator`] and
//! forwards every call unchanged.  The only side-effect is in
//! [`send_prompt`][ElicitCommunicator::send_prompt]: before delegating to the
//! inner communicator it publishes the assembled prompt to a
//! `watch::Sender<Option<String>>` (for in-flight display), then posts a
//! `[GameMaster]` message to the chat log channel.  After the response returns
//! it clears the watch channel and posts a `[Player]` reply.
//!
//! `call_tool` is passed through unchanged so that [`ChoiceSet`] and other
//! tool-based elicitations continue to work with the inner communicator.
//!
//! # Usage
//!
//! ```rust,ignore
//! use tokio::sync::{mpsc, watch};
//! use valinoreth::{ChatMessage, ChatSender, ObservableCommunicator, TuiCommunicator};
//!
//! let (prompt_tx, _prompt_rx) = watch::channel(None);
//! let (chat_tx, chat_rx) = mpsc::unbounded_channel();
//! let comm = ObservableCommunicator::new(
//!     TuiCommunicator::new(),
//!     prompt_tx,
//!     chat_tx,
//!     ChatSender::Player,
//! );
//! // Pass chat_rx to run_chat so the UI drains it each frame.
//! ```
//!
//! ## Why `watch` for the in-flight prompt?
//!
//! `watch` has "latest value" semantics: non-blocking reads, no backpressure,
//! stale values are automatically dropped.
//!
//! ## Why `mpsc` for chat history?
//!
//! The chat log is append-only; every message must survive.  `mpsc` preserves
//! ordering and never drops entries.
//!
//! [`ChoiceSet`]: elicitation::ChoiceSet

use elicitation::{
    ElicitCommunicator, ElicitResult, ElicitationContext, StyleContext, StyleMarker,
};
use tokio::sync::{mpsc, watch};
use tracing::instrument;

use crate::{ChatMessage, ChatSender};

/// Transparent middleware that publishes elicitation exchanges to the game chat.
///
/// Wraps any inner [`ElicitCommunicator`].  `send_prompt` intercepts the
/// assembled prompt (already formatted with numbered options by the elicitation
/// runtime), routes it to the chat log, and clears it when the response arrives.
///
/// Construct with [`ObservableCommunicator::new`].
#[derive(Clone)]
pub struct ObservableCommunicator<C> {
    inner: C,
    prompt_tx: watch::Sender<Option<String>>,
    chat_tx: mpsc::UnboundedSender<ChatMessage>,
    /// Identity used for the player reply in the chat log.
    sender: ChatSender,
}

impl<C> ObservableCommunicator<C> {
    /// Wrap `inner`, publish in-flight prompts to `prompt_tx`, and route
    /// exchanges to `chat_tx` with replies attributed to `sender`.
    pub fn new(
        inner: C,
        prompt_tx: watch::Sender<Option<String>>,
        chat_tx: mpsc::UnboundedSender<ChatMessage>,
        sender: ChatSender,
    ) -> Self {
        Self { inner, prompt_tx, chat_tx, sender }
    }
}

impl<C: ElicitCommunicator> ElicitCommunicator for ObservableCommunicator<C> {
    /// Publish the prompt to the watch channel and chat log, delegate to the
    /// inner communicator, then clear the watch channel on return.
    #[instrument(skip(self), level = "debug", fields(prompt_len = prompt.len()))]
    fn send_prompt(
        &self,
        prompt: &str,
    ) -> impl std::future::Future<Output = ElicitResult<String>> + Send {
        let prompt_owned = prompt.to_string();
        let prompt_tx = self.prompt_tx.clone();
        let chat_tx = self.chat_tx.clone();
        let sender = self.sender;
        let inner_future = self.inner.send_prompt(prompt);

        async move {
            // Publish in-flight prompt for any live display widget.
            prompt_tx.send(Some(prompt_owned.clone())).ok();

            // Append the GM prompt to the chat log.
            chat_tx
                .send(ChatMessage::new(ChatSender::GameMaster, prompt_owned))
                .ok();

            let result = inner_future.await;

            // Clear the in-flight prompt: exchange is complete.
            prompt_tx.send(None).ok();

            // Append the player's reply to the chat log.
            if let Ok(ref response) = result {
                chat_tx
                    .send(ChatMessage::new(sender, response.clone()))
                    .ok();
            }

            result
        }
    }

    /// Pass tool calls through to the inner communicator unchanged.
    #[instrument(skip(self, params), level = "debug", fields(tool = %params.name))]
    fn call_tool(
        &self,
        params: rmcp::model::CallToolRequestParams,
    ) -> impl std::future::Future<
        Output = Result<rmcp::model::CallToolResult, rmcp::service::ServiceError>,
    > + Send {
        self.inner.call_tool(params)
    }

    fn style_context(&self) -> &StyleContext {
        self.inner.style_context()
    }

    fn elicitation_context(&self) -> &ElicitationContext {
        self.inner.elicitation_context()
    }

    fn with_style<
        T: 'static,
        S: StyleMarker + elicitation::style::ElicitationStyle + 'static,
    >(
        &self,
        style: S,
    ) -> Self {
        Self {
            inner: self.inner.with_style::<T, S>(style),
            prompt_tx: self.prompt_tx.clone(),
            chat_tx: self.chat_tx.clone(),
            sender: self.sender,
        }
    }
}
