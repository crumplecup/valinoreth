//! [`Screen`] trait and [`ScreenTransition`] for the lobby state machine.

use crossterm::event::KeyEvent;
use ratatui::Frame;

use crate::CombatSlot;

/// Result of handling an input event on a lobby screen.
///
/// Screens return this from [`Screen::handle_key`] to drive the
/// [`LobbyController`](crate::LobbyController) state machine.
#[derive(Debug, Clone)]
pub enum ScreenTransition {
    /// Stay on the current screen — no state change.
    Stay,
    /// Navigate to the main lobby.
    GoToMainLobby,
    /// Navigate to the combat setup screen.
    GoToCombatSetup,
    /// Navigate to the settings screen.
    GoToSettings,
    /// Launch a combat encounter with the configured participant slots.
    StartCombat {
        /// One slot per participant (Team A slots first, then Team B).
        slots: Vec<CombatSlot>,
    },
    /// Exit the application cleanly.
    Quit,
}

/// Trait implemented by each screen in the lobby state machine.
///
/// The controller calls `render` each frame and `handle_key` on every key event.
/// Screens own their own state and return [`ScreenTransition`] values to drive
/// navigation.
pub trait Screen {
    /// Renders the screen into the provided [`Frame`].
    fn render(&self, frame: &mut Frame);

    /// Handles a key event and returns the resulting [`ScreenTransition`].
    fn handle_key(&mut self, key: KeyEvent) -> ScreenTransition;
}
