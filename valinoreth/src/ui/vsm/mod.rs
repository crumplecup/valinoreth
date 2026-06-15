//! Verified State Machines for the game UI.

mod chat;

pub use chat::{
    begin_compose, cancel_compose, receive_message, scroll_down, scroll_up, send_message,
    ChatConsistent, ChatMachine, ChatState,
};
// BEGIN ELICITATION KANI REEXPORTS — DO NOT EDIT
pub use chat::begin_compose_kani_contracted;
pub use chat::cancel_compose_kani_contracted;
pub use chat::chat_consistent;
pub use chat::receive_message_kani_contracted;
pub use chat::scroll_down_kani_contracted;
pub use chat::scroll_up_kani_contracted;
pub use chat::send_message_kani_contracted;
// END ELICITATION KANI REEXPORTS
