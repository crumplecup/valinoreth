//! Frontend renderers for the game chat UI.

#[cfg(feature = "frontend-ratatui")]
mod ratatui;
#[cfg(feature = "frontend-ratatui")]
mod tui_communicator;

#[cfg(feature = "frontend-ratatui")]
pub use ratatui::run_chat;
#[cfg(feature = "frontend-ratatui")]
pub use tui_communicator::TuiCommunicator;
