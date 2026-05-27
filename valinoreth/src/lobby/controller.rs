//! Lobby controller — the state machine driving the multi-screen TUI.
//!
//! Call [`run_lobby`] to start the full lobby event loop from `main`.

use std::io::Write as _;

use crossterm::{
    event::{self, Event, KeyEventKind},
    execute,
    terminal::{
        EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
    },
};
use ratatui::{Terminal, backend::CrosstermBackend};
use tokio::sync::mpsc;
use tracing::{debug, info, instrument, warn};

use crate::lobby::screen::{Screen, ScreenTransition};
use crate::lobby::screens::{CombatSetupScreen, MainLobbyScreen, SettingsScreen};
use crate::lobby::settings::{CombatSlot, LobbySettings};
use crate::{
    ChatMessage, ChatSender, CombatWorkflow, GameMaster, Player, TuiCommunicator,
};

/// Active screen in the lobby state machine.
#[derive(Debug)]
enum ActiveScreen {
    MainLobby(MainLobbyScreen),
    CombatSetup(CombatSetupScreen),
    Settings(SettingsScreen),
}

/// Controls the lobby TUI state machine.
///
/// Holds current [`LobbySettings`] and drives screen transitions in
/// [`LobbyController::run`].
#[derive(Debug)]
pub struct LobbyController {
    settings: LobbySettings,
}

impl LobbyController {
    /// Creates a new lobby controller with default settings.
    #[instrument]
    pub fn new() -> Self {
        info!("Creating LobbyController");
        Self { settings: LobbySettings::new() }
    }

    /// Runs the lobby event loop until the user quits.
    ///
    /// Drives screen transitions, handles key events, and executes combat
    /// sessions.  The `terminal` must already be in ratatui mode (alternate
    /// screen, raw mode).
    ///
    /// # Errors
    ///
    /// Returns `Err` on I/O failures from the terminal or combat workflow.
    #[instrument(skip(self, terminal))]
    pub async fn run(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    ) -> anyhow::Result<()> {
        info!("Starting lobby event loop");
        let mut screen = ActiveScreen::MainLobby(MainLobbyScreen::new());

        loop {
            terminal.draw(|f| match &screen {
                ActiveScreen::MainLobby(s) => s.render(f),
                ActiveScreen::CombatSetup(s) => s.render(f),
                ActiveScreen::Settings(s) => s.render(f),
            })?;

            if !event::poll(std::time::Duration::from_millis(50))? {
                continue;
            }

            let Event::Key(key) = event::read()? else { continue };
            if key.kind == KeyEventKind::Release {
                continue;
            }

            let transition = match &mut screen {
                ActiveScreen::MainLobby(s) => s.handle_key(key),
                ActiveScreen::CombatSetup(s) => s.handle_key(key),
                ActiveScreen::Settings(s) => s.handle_key(key),
            };

            // StartCombat leaves the ratatui context, runs combat, then returns.
            if let ScreenTransition::StartCombat { ref slots } = transition {
                let slots = slots.clone();
                self.execute_combat(terminal, slots).await?;
                screen = ActiveScreen::MainLobby(MainLobbyScreen::new());
                continue;
            }

            screen = match self.apply_transition(transition, screen) {
                Some(next) => next,
                None => {
                    info!("Lobby quitting");
                    return Ok(());
                }
            };
        }
    }

    /// Applies a non-combat screen transition.
    #[instrument(skip(self, current))]
    fn apply_transition(
        &mut self,
        transition: ScreenTransition,
        current: ActiveScreen,
    ) -> Option<ActiveScreen> {
        debug!(transition = ?transition, "Applying screen transition");
        match transition {
            ScreenTransition::Stay => Some(current),

            ScreenTransition::GoToMainLobby => {
                if let ActiveScreen::Settings(s) = &current {
                    self.settings = s.settings();
                }
                info!("Navigating to MainLobby");
                Some(ActiveScreen::MainLobby(MainLobbyScreen::new()))
            }

            ScreenTransition::GoToCombatSetup => {
                info!("Navigating to CombatSetup");
                Some(ActiveScreen::CombatSetup(CombatSetupScreen::new()))
            }

            ScreenTransition::GoToSettings => {
                info!("Navigating to Settings");
                Some(ActiveScreen::Settings(SettingsScreen::new(self.settings)))
            }

            ScreenTransition::StartCombat { .. } => {
                warn!("StartCombat reached apply_transition — should have been intercepted");
                Some(current)
            }

            ScreenTransition::Quit => None,
        }
    }

    /// Executes a combat encounter and returns to the lobby when done.
    ///
    /// Leaves the ratatui alternate screen, runs the [`CombatWorkflow`] in
    /// normal terminal mode (GM narration via background task, player choices
    /// via [`TuiCommunicator`]), then re-enters the alternate screen.
    #[instrument(skip(self, terminal, slots), fields(num_slots = slots.len()))]
    async fn execute_combat(
        &self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
        slots: Vec<CombatSlot>,
    ) -> anyhow::Result<()> {
        info!("Executing combat encounter");

        // Leave ratatui alternate screen so we get a normal scrolling terminal.
        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

        println!("\n╔══════════════════════════════╗");
        println!("║   ⚔  GURPS Combat Starting  ⚔ ║");
        println!("╚══════════════════════════════╝\n");
        for slot in &slots {
            println!(
                "  Team {} — {} ({})",
                slot.team,
                slot.character.name,
                slot.kind.label()
            );
        }
        println!();

        // Build workflow.
        let gm = GameMaster::new();
        let (chat_tx, mut chat_rx) = mpsc::unbounded_channel::<ChatMessage>();

        // Background task: print GM narration as it arrives.
        let print_task = tokio::spawn(async move {
            let mut stdout = std::io::stdout();
            while let Some(msg) = chat_rx.recv().await {
                let prefix = match msg.sender {
                    ChatSender::GameMaster => "[GM]    ",
                    ChatSender::Player => "[PLAYER]",
                    ChatSender::System => "[SYS]   ",
                };
                let _ = write!(stdout, "{} {}\r\n", prefix, msg.text);
                let _ = stdout.flush();
            }
        });

        // Build players — both use TuiCommunicator; agent kind is cosmetic for now.
        let players: Vec<(Player<TuiCommunicator>, String)> = slots
            .into_iter()
            .map(|slot| (Player::new(slot.character, TuiCommunicator::new()), slot.team))
            .collect();

        enable_raw_mode()?;
        let result = CombatWorkflow::new(gm, players, chat_tx).run().await;
        disable_raw_mode()?;

        // Drain remaining messages from the channel.
        print_task.await?;

        // Show outcome.
        println!();
        match &result {
            Ok(Some(winner)) => println!("═══  Team {} wins!  ═══", winner),
            Ok(None) => println!("═══  Draw — all combatants incapacitated  ═══"),
            Err(e) => println!("═══  Combat error: {}  ═══", e),
        }
        println!("\nPress any key to return to lobby…");

        // Wait for one keypress before returning.
        enable_raw_mode()?;
        loop {
            if event::poll(std::time::Duration::from_millis(500))? {
                let _ = event::read();
                break;
            }
        }
        disable_raw_mode()?;

        // Re-enter ratatui.
        execute!(terminal.backend_mut(), EnterAlternateScreen)?;
        enable_raw_mode()?;
        terminal.clear()?;

        info!("Combat complete, returning to lobby");
        Ok(())
    }
}

impl Default for LobbyController {
    fn default() -> Self {
        Self::new()
    }
}

/// Sets up the terminal and runs the lobby until the user quits.
///
/// Handles terminal setup, drives [`LobbyController::run`], and restores the
/// terminal on exit regardless of how the loop ends.
///
/// # Errors
///
/// Returns `Err` on terminal I/O failures or unrecoverable workflow errors.
#[instrument]
pub async fn run_lobby() -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = LobbyController::new().run(&mut terminal).await;

    // Always restore terminal even if we got an error.
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}
