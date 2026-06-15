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
mod combat_communicator;
mod controller;
mod screen;
mod screens;
mod settings;

pub use agent_config::{AgentConfig, ConfigError};
pub use combat_communicator::CombatCommunicator;
pub use controller::{run_lobby, LobbyController};
pub use screen::{Screen, ScreenTransition};
pub use screens::{CombatSetupScreen, MainLobbyScreen, SettingsScreen};
pub use settings::{default_roster, CombatSlot, LobbySettings, PlayerKind, RosterEntry};
