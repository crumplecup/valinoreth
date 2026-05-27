//! Chat message type and its AccessKit display implementation.

use accesskit::Role as AkRole;
use derive_more::Display;
use elicit_accesskit::{NodeId, NodeJson, Role};
use elicitation::{Elicit, KaniCompose};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::GameDisplay;

/// Who sent a [`ChatMessage`].
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
    Elicit,
    KaniCompose,
    Display,
)]
pub enum ChatSender {
    /// The Game Master narrating events or issuing prompts.
    #[display("GM")]
    GameMaster,
    /// A player responding to the GM.
    #[display("PLAYER")]
    Player,
    /// Automatic system/game event announcements (combat round, dice roll, etc.).
    #[display("SYSTEM")]
    System,
}

/// A single message in the game chat log.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit, KaniCompose)]
pub struct ChatMessage {
    /// Who sent this message.
    pub sender: ChatSender,
    /// Message text.
    pub text: String,
}

impl ChatMessage {
    /// Construct a new chat message.
    pub fn new(sender: ChatSender, text: impl Into<String>) -> Self {
        Self {
            sender,
            text: text.into(),
        }
    }
}

/// Display strategies for a [`ChatMessage`].
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Default,
    Serialize,
    Deserialize,
    JsonSchema,
    Elicit,
    KaniCompose,
)]
pub enum ChatMessageMode {
    /// A compact row: `[SENDER] message text`.
    #[default]
    Row,
}

impl GameDisplay for ChatMessage {
    type Mode = ChatMessageMode;

    fn root_role(_mode: &Self::Mode) -> Role {
        Role(AkRole::ListItem)
    }

    fn to_ak_nodes(&self, _mode: &Self::Mode, id_base: u64) -> (NodeId, Vec<(NodeId, NodeJson)>) {
        let root_id = NodeId(accesskit::NodeId::from(id_base));
        let label = format!("[{}] {}", self.sender, self.text);
        let node = NodeJson::new(Role(AkRole::ListItem)).with_label(label);
        (root_id, vec![(root_id, node)])
    }
}
