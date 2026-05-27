//! Lobby — the start-screen TUI for GURPS combat encounters.
//!
//! Provides a multi-screen ratatui interface where players can configure
//! a combat encounter (characters, teams, Human vs Agent per slot) before
//! launching it.
//!
//! ## Screen flow
//!
//! ```text
//! MainLobbyScreen
//!   ├── Start Combat → CombatSetupScreen → [launches CombatWorkflow]
//!   ├── Settings     → SettingsScreen
//!   └── Quit
//! ```
//!
//! ## Entry point
//!
//! Call [`run_lobby`] from `main` to start the full TUI loop.

mod agent_config;
mod controller;
mod screen;
mod screens;
mod settings;

pub use agent_config::AgentConfig;
pub use controller::{LobbyController, run_lobby};
pub use screen::{Screen, ScreenTransition};
pub use screens::{CombatSetupScreen, MainLobbyScreen, SettingsScreen};
pub use settings::{CombatSlot, LobbySettings, PlayerKind, RosterEntry, default_roster};
