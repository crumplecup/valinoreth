//! [`ChatModel`] — shared state for the game chat UI.
//!
//! `ChatModel` owns the message log, scroll position, and VSM state.
//! Its [`to_verified_tree`] method is the single authorised path to an
//! [`elicit_ui::IrSourced`] proof token, ensuring all frontends share the
//! same AccessKit IR source.  No frontend may render without calling it.
//!
//! ## Rendering pipeline
//!
//! ```text
//! ChatModel::to_verified_tree()
//!   → (VerifiedTree, Established<IrSourced>)
//!   → RatatuiBackend::render(&tree)   // or egui / leptos equivalent
//!   → (TuiNode, stats, render_proof)
//!   → render_node(frame, area, &tui_node)
//! ```

use elicit_ui::{VerifiedTree, Viewport};
use elicitation::Established;
use tracing::instrument;

use crate::{
    ChatConsistent, ChatKeyMap, ChatMessage, ChatMessageMode, ChatSender, ChatState, GameDisplay,
};
use crate::ui::vsm::{
    begin_compose, cancel_compose, receive_message, scroll_down, scroll_up, send_message,
};

/// How many message rows to show in the chat area.
const VISIBLE_LINES: usize = 20;

/// Shared model for the game chat screen.
///
/// All frontends (ratatui, egui, leptos) call [`to_verified_tree`] and consume
/// the same AccessKit IR.  State mutation goes through VSM transitions so every
/// change is tracked by a proof token.
///
/// [`to_verified_tree`]: ChatModel::to_verified_tree
pub struct ChatModel {
    messages: Vec<ChatMessage>,
    /// Lines scrolled up from the bottom (0 = newest visible at bottom).
    scroll_offset: usize,
    state: ChatState,
    proof: Established<ChatConsistent>,
}

impl ChatModel {
    /// Create an empty chat model.
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            scroll_offset: 0,
            state: ChatState::Viewing,
            proof: Established::assert(),
        }
    }

    /// Post a GM message to the log.
    #[instrument(skip(self, text))]
    pub fn gm_say(&mut self, text: impl Into<String>) {
        self.post(ChatSender::GameMaster, text);
    }

    /// Post a system event to the log (combat round, dice result, etc.).
    #[instrument(skip(self, text))]
    pub fn system_event(&mut self, text: impl Into<String>) {
        self.post(ChatSender::System, text);
    }

    /// All messages in the log (oldest-first).
    pub fn messages(&self) -> &[ChatMessage] {
        &self.messages
    }

    /// `true` when the player is in the composing state.
    pub fn is_composing(&self) -> bool {
        matches!(self.state, ChatState::Composing { .. })
    }

    /// The composing buffer, or `None` when not composing.
    pub fn composed_text(&self) -> Option<&str> {
        match &self.state {
            ChatState::Composing { buffer } => Some(buffer),
            _ => None,
        }
    }

    /// Append a character to the composing buffer.  No-op when not composing.
    pub fn push_char(&mut self, c: char) {
        if let ChatState::Composing { buffer } = &mut self.state {
            buffer.push(c);
        }
    }

    /// Delete the last character from the composing buffer.  No-op when not composing.
    pub fn pop_char(&mut self) {
        if let ChatState::Composing { buffer } = &mut self.state {
            buffer.pop();
        }
    }

    /// Transition to [`ChatState::Composing`] with a fresh buffer.
    pub fn begin_compose(&mut self) {
        let (s, p) = begin_compose(self.state.clone(), self.proof.clone());
        self.state = s;
        self.proof = p;
    }

    /// Send the composing buffer as a player message and return to viewing.
    ///
    /// Returns the sent text if the buffer was non-empty, `None` otherwise.
    pub fn send_message(&mut self) -> Option<String> {
        let text = match &self.state {
            ChatState::Composing { buffer } if !buffer.trim().is_empty() => Some(buffer.clone()),
            _ => None,
        };
        let (s, p) = send_message(self.state.clone(), self.proof.clone());
        self.state = s;
        self.proof = p;
        if let Some(ref t) = text {
            self.messages.push(ChatMessage::new(ChatSender::Player, t));
        }
        text
    }

    /// Discard the composing buffer and return to viewing.
    pub fn cancel_compose(&mut self) {
        let (s, p) = cancel_compose(self.state.clone(), self.proof.clone());
        self.state = s;
        self.proof = p;
    }

    /// Scroll toward older messages.
    pub fn scroll_up(&mut self) {
        let max = self.messages.len().saturating_sub(VISIBLE_LINES);
        if self.scroll_offset < max {
            self.scroll_offset += 1;
        }
        let (s, p) = scroll_up(self.state.clone(), self.proof.clone());
        self.state = s;
        self.proof = p;
    }

    /// Scroll toward newer messages.
    pub fn scroll_down(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
        let (s, p) = scroll_down(self.state.clone(), self.proof.clone());
        self.state = s;
        self.proof = p;
    }

    fn post(&mut self, sender: ChatSender, text: impl Into<String>) {
        self.messages.push(ChatMessage::new(sender, text));
        let (s, p) = receive_message(self.state.clone(), self.proof.clone());
        self.state = s;
        self.proof = p;
    }

    /// Build a fully-described [`VerifiedTree`] from the current model state.
    ///
    /// This is the **only** authorised way to obtain an
    /// [`elicit_ui::IrSourced`] proof token.  Every frontend renderer must
    /// call this before rendering — the token is the compile-time contract that
    /// all frontends share the same AccessKit IR source.
    ///
    /// ## Tree structure
    ///
    /// ```text
    /// Window
    ///   Banner  — "Valinoreth — GURPS Combat"
    ///   Main
    ///     List  — message log (VISIBLE_LINES rows)
    ///       ListItem × N
    ///     TextInput  — player input (Composing state only)
    ///   Status  — keybinding hints
    /// ```
    #[instrument(skip(self))]
    pub fn to_verified_tree(
        &self,
    ) -> (VerifiedTree, Established<elicit_ui::IrSourced>) {
        use accesskit::{Node as AkNode, NodeId as AkNodeId, Role as AkRole};
        use std::collections::BTreeMap;

        let mut nodes: BTreeMap<AkNodeId, AkNode> = BTreeMap::new();
        let mut counter: u64 = 1; // NodeId(0) reserved for Window root

        // ── banner ────────────────────────────────────────────────────────────
        let banner_id = AkNodeId::from(counter);
        counter += 1;
        let mut banner = AkNode::new(AkRole::Banner);
        banner.set_label("Valinoreth — GURPS Combat".to_string());
        nodes.insert(banner_id, banner);

        // ── message log ───────────────────────────────────────────────────────
        let log_id = AkNodeId::from(counter);
        counter += 1;

        // Compute the slice of messages visible in the current scroll window.
        let end = self.messages.len().saturating_sub(self.scroll_offset);
        let start = end.saturating_sub(VISIBLE_LINES);
        let visible = &self.messages[start..end];

        let mut msg_children: Vec<AkNodeId> = Vec::new();
        for msg in visible {
            let (root_id, node_pairs) = msg.to_ak_nodes(&ChatMessageMode::Row, counter);
            let node_count = node_pairs.len() as u64;
            for (nid, json) in node_pairs {
                nodes.insert(nid.0, accesskit::Node::from(json));
            }
            msg_children.push(root_id.0);
            counter += node_count;
        }

        let mut log_node = AkNode::new(AkRole::List);
        log_node.set_label("message log".to_string());
        log_node.set_description("id=message-log".to_string());
        log_node.set_children(msg_children);
        nodes.insert(log_id, log_node);

        // ── input area (Composing only) ───────────────────────────────────────
        let mut main_children = vec![log_id];
        if let ChatState::Composing { buffer } = &self.state {
            let input_id = AkNodeId::from(counter);
            counter += 1;
            let mut input = AkNode::new(AkRole::TextInput);
            input.set_label("player input".to_string());
            input.set_value(buffer.clone());
            input.set_description(
                "id=player-input;placeholder=Type your response and press Enter...".to_string(),
            );
            nodes.insert(input_id, input);
            main_children.push(input_id);
        }

        // ── main container ────────────────────────────────────────────────────
        let main_id = AkNodeId::from(counter);
        counter += 1;
        let mut main_node = AkNode::new(AkRole::Main);
        main_node.set_children(main_children);
        nodes.insert(main_id, main_node);

        // ── status bar ────────────────────────────────────────────────────────
        let status_id = AkNodeId::from(counter);
        let hints = ChatKeyMap::status_hints()
            .iter()
            .map(|(k, v)| format!("[{k}] {v}"))
            .collect::<Vec<_>>()
            .join("  ");
        let mut status = AkNode::new(AkRole::Status);
        status.set_label(hints);
        nodes.insert(status_id, status);

        // ── window root ───────────────────────────────────────────────────────
        let window_id = AkNodeId::from(0u64);
        let mut window = AkNode::new(AkRole::Window);
        window.set_children(vec![banner_id, main_id, status_id]);
        nodes.insert(window_id, window);

        let viewport = Viewport::new(800, 600);
        let tree = VerifiedTree::from_parts(nodes, window_id, viewport);
        (tree, Established::assert())
    }
}

impl Default for ChatModel {
    fn default() -> Self {
        Self::new()
    }
}
