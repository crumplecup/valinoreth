//! Game UI — WCAG-compliant AccessKit IR driving multiple frontends.
//!
//! Follows the same pattern as `elicit_server`'s archive UI:
//!
//! 1. [`GameDisplay`] trait — every displayable type produces an AccessKit IR
//!    subtree.
//! 2. [`ChatModel`] — shared state whose [`to_verified_tree`] is the single
//!    authorised source of an [`IrSourced`] proof token.
//! 3. [`ChatMachine`] VSM — formally verified state transitions for the chat
//!    screen, each carrying an [`Established<ChatConsistent>`] proof token.
//! 4. Frontend modules — ratatui / egui / leptos all consume the same
//!    [`VerifiedTree`]; no per-frontend state can diverge.
//!
//! [`to_verified_tree`]: ChatModel::to_verified_tree
//! [`IrSourced`]: elicit_ui::IrSourced
//! [`VerifiedTree`]: elicit_ui::VerifiedTree

mod chat;
mod display;
mod frontends;
mod keymap;
mod model;
mod vsm;

pub use chat::{ChatMessage, ChatMessageMode, ChatSender};
pub use elicitation::middleware::{
    ContextualCommunicator, KnowledgeCache, ObservableCommunicator, Participant, SharedKnowledge,
    knowledge_cache,
};
pub use display::GameDisplay;
pub use keymap::{ChatAction, ChatKeyMap};
pub use model::ChatModel;
pub use vsm::{
    ChatConsistent, ChatMachine, ChatState, begin_compose, cancel_compose, receive_message,
    scroll_down, scroll_up, send_message,
};

#[cfg(feature = "frontend-ratatui")]
pub use frontends::run_chat;
