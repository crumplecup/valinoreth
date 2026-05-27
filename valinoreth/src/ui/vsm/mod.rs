//! Verified State Machines for the game UI.

mod chat;

pub use chat::{
    ChatConsistent, ChatMachine, ChatState, begin_compose, cancel_compose, receive_message,
    scroll_down, scroll_up, send_message,
};
