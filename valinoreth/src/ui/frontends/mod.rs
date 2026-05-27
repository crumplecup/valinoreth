//! Frontend renderers for the game chat UI.

#[cfg(feature = "frontend-ratatui")]
mod ratatui;

#[cfg(feature = "frontend-ratatui")]
pub use ratatui::run_chat;
