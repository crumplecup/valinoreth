//! Verified State Machine for the game chat screen.
//!
//! [`ChatMachine`] has two states:
//!
//! - [`ChatState::Viewing`] — player is reading the message log
//! - [`ChatState::Composing`] — player is typing a response
//!
//! The invariant [`ChatConsistent`] guarantees the state machine is in a
//! coherent configuration: exactly one chat mode is active at a time and
//! the composing buffer is valid when present.
//!
//! ## State diagram
//!
//! ```text
//! Viewing ──begin_compose──► Composing
//!    ▲                           │
//!    │◄──cancel_compose──────────┤
//!    │◄──send_message────────────┘
//!    │
//!    └── receive_message, scroll_up, scroll_down (self-loops)
//! ```

use elicitation::{Elicit, Established, KaniCompose, KaniVariantState, Prop, VerifiedStateMachine};
use elicitation::formal_method;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── ChatState ─────────────────────────────────────────────────────────────────

/// State of the chat UI screen.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    Elicit,
    KaniCompose,
    KaniVariantState,
)]
pub enum ChatState {
    /// Player is reading the message log.
    #[default]
    Viewing,
    /// Player is composing a response; `buffer` holds the current input text.
    Composing {
        /// Text being composed by the player.
        buffer: String,
    },
}

// ── ChatConsistent ────────────────────────────────────────────────────────────

/// Invariant: the chat state machine is in a consistent state.
///
/// Holds whenever the state is a valid `ChatState` variant — one mode
/// active at a time, no undefined intermediate configurations.
///
/// All `ChatState` variants are structurally valid by construction, so
/// the invariant holds trivially for any well-formed value.
#[derive(Prop)]
#[prop(
    kani_invariant_fn = "chat_consistent",
    creusot_invariant_fn = "chat_consistent",
    verus_inv_body = "true",
    creusot_inv_body = "pearlite! { true }"
)]
pub struct ChatConsistent;

/// Structural invariant predicate for [`ChatState`].
///
/// All `ChatState` variants are valid by construction; the invariant
/// holds trivially for any well-formed value.
pub fn chat_consistent(_state: &ChatState) -> bool {
    true
}

// ── ChatMachine ───────────────────────────────────────────────────────────────

/// Verified State Machine for the game chat screen.
///
/// Generates Kani/Creusot/Verus proof harnesses for every registered
/// transition, guaranteeing [`ChatConsistent`] is preserved across all
/// state changes.
#[derive(VerifiedStateMachine)]
#[vsm(transitions = [
    begin_compose,
    send_message,
    cancel_compose,
    receive_message,
    scroll_up,
    scroll_down,
])]
pub struct ChatMachine;

// ── Transitions ───────────────────────────────────────────────────────────────

/// Player opens the input line to compose a message.
///
/// Transitions from any state to [`ChatState::Composing`] with a fresh buffer.
#[formal_method(contracts = [ChatConsistent])]
#[instrument(skip(proof))]
pub fn begin_compose(
    _state: ChatState,
    proof: Established<ChatConsistent>,
) -> (ChatState, Established<ChatConsistent>) {
    (ChatState::Composing { buffer: String::new() }, proof)
}

/// Player sends the composed message, returning to [`ChatState::Viewing`].
///
/// No-op if already in `Viewing` state.
#[formal_method(contracts = [ChatConsistent])]
#[instrument(skip(proof))]
pub fn send_message(
    state: ChatState,
    proof: Established<ChatConsistent>,
) -> (ChatState, Established<ChatConsistent>) {
    match state {
        ChatState::Composing { .. } => (ChatState::Viewing, proof),
        other => (other, proof),
    }
}

/// Player cancels composing and returns to [`ChatState::Viewing`].
#[formal_method(contracts = [ChatConsistent])]
#[instrument(skip(proof))]
pub fn cancel_compose(
    _state: ChatState,
    proof: Established<ChatConsistent>,
) -> (ChatState, Established<ChatConsistent>) {
    (ChatState::Viewing, proof)
}

/// A new message arrives in the log. State is unchanged; this is a self-loop
/// that the VSM records as a valid event transition.
#[formal_method(contracts = [ChatConsistent])]
#[instrument(skip(proof))]
pub fn receive_message(
    state: ChatState,
    proof: Established<ChatConsistent>,
) -> (ChatState, Established<ChatConsistent>) {
    (state, proof)
}

/// User scrolls the message log toward older entries. State unchanged.
#[formal_method(contracts = [ChatConsistent])]
#[instrument(skip(proof))]
pub fn scroll_up(
    state: ChatState,
    proof: Established<ChatConsistent>,
) -> (ChatState, Established<ChatConsistent>) {
    (state, proof)
}

/// User scrolls the message log toward newer entries. State unchanged.
#[formal_method(contracts = [ChatConsistent])]
#[instrument(skip(proof))]
pub fn scroll_down(
    state: ChatState,
    proof: Established<ChatConsistent>,
) -> (ChatState, Established<ChatConsistent>) {
    (state, proof)
}
