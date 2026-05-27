//! Key bindings for the game chat UI.
//!
//! [`ChatKeyMap`] is the single source of truth for all key bindings.
//! Every frontend derives its runtime dispatch and status-bar hints from it,
//! so adding a binding here propagates to all renderers automatically.

/// An action the user can take in the chat UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChatAction {
    /// Scroll the message log toward older messages.
    ScrollUp,
    /// Scroll the message log toward newer messages.
    ScrollDown,
    /// Open the player input line.
    BeginCompose,
    /// Send the composed message.
    SendMessage,
    /// Discard the current compose buffer and return to viewing.
    CancelCompose,
    /// Quit the frontend.
    Quit,
}

/// Key binding declarations for the game chat UI.
///
/// All frontends (ratatui, egui, leptos) derive their key-to-action mapping
/// from this map.  The status-bar hint list is the single description of what
/// keys do — no per-frontend duplication.
pub struct ChatKeyMap;

impl ChatKeyMap {
    /// Human-readable `(key, description)` pairs for the status bar.
    pub fn status_hints() -> &'static [(&'static str, &'static str)] {
        &[
            ("↑↓", "scroll"),
            ("i", "input"),
            ("Enter", "send"),
            ("Esc", "cancel/back"),
            ("q", "quit"),
        ]
    }
}
