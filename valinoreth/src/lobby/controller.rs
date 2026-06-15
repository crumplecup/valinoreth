//! Lobby controller — the state machine driving the multi-screen TUI.
//!
//! Call [`run_lobby`] to start the full lobby event loop from `main`.

use std::sync::Arc;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use elicit_ratatui::{render_node, RatatuiBackend};
use elicit_ui::UiTreeRenderer as _;
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::{mpsc, Mutex};
use tracing::{debug, info, instrument, warn};

use crate::lobby::screen::{Screen, ScreenTransition};
use crate::lobby::screens::{CombatSetupScreen, MainLobbyScreen, SettingsScreen};
use crate::lobby::settings::{CombatSlot, LobbySettings, PlayerKind};
use crate::{
    ChatCommunicator, ChatMessage, ChatModel, CombatCommunicator, CombatWorkflow, GameMaster,
    LlmElicitCommunicator, Player,
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
        Self {
            settings: LobbySettings::new(),
        }
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

            if !event::poll(Duration::from_millis(50))? {
                continue;
            }

            let Event::Key(key) = event::read()? else {
                continue;
            };
            if key.kind == KeyEventKind::Release {
                continue;
            }

            let transition = match &mut screen {
                ActiveScreen::MainLobby(s) => s.handle_key(key),
                ActiveScreen::CombatSetup(s) => s.handle_key(key),
                ActiveScreen::Settings(s) => s.handle_key(key),
            };

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

    /// Executes a combat encounter inside the existing ratatui context.
    ///
    /// Stays in the alternate screen throughout.  GM narration and human
    /// elicitation prompts flow through a [`ChatModel`] rendered via the
    /// AccessKit IR pipeline.  Human players type replies in compose mode;
    /// agent players call the LLM directly.
    ///
    /// Returns to the lobby when the workflow completes and the player presses
    /// `q` or `Esc`.
    ///
    /// # Errors
    ///
    /// Returns `Err` on terminal I/O failures or if the workflow task panics.
    #[instrument(skip(self, terminal, slots), fields(num_slots = slots.len()))]
    async fn execute_combat(
        &self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
        slots: Vec<CombatSlot>,
    ) -> anyhow::Result<()> {
        info!("Starting combat encounter");

        // ── Channels ──────────────────────────────────────────────────────────
        // All chat messages (GM narration + human prompts) arrive on chat_rx.
        let (chat_tx, mut chat_rx) = mpsc::unbounded_channel::<ChatMessage>();
        // compose_rx fires when a ChatCommunicator wants compose mode enabled.
        let (compose_tx, mut compose_rx) = mpsc::unbounded_channel::<()>();
        // reply_tx is the event loop end; reply_rx is held by ChatCommunicator.
        let (reply_tx, reply_rx) = mpsc::unbounded_channel::<String>();
        let reply_rx = Arc::new(Mutex::new(reply_rx));

        // ── Build communicators ───────────────────────────────────────────────
        let chat_comm = ChatCommunicator::new(chat_tx.clone(), compose_tx, reply_rx);

        let mut model = ChatModel::new();
        model.system_event("Combat begins — good luck!");

        let gm = GameMaster::new();
        let mut players: Vec<(Player<CombatCommunicator>, String)> = Vec::new();
        for slot in &slots {
            let comm = match &slot.kind {
                PlayerKind::Human => CombatCommunicator::Human(chat_comm.clone()),
                PlayerKind::Agent(config) => match LlmElicitCommunicator::new(config) {
                    Ok(llm) => CombatCommunicator::Agent(llm),
                    Err(e) => {
                        warn!(error = %e, "Failed to create LLM communicator; falling back to Human");
                        model.system_event(format!(
                            "Agent init failed ({e}); slot falling back to Human"
                        ));
                        CombatCommunicator::Human(chat_comm.clone())
                    }
                },
            };
            players.push((Player::new(slot.character.clone(), comm), slot.team.clone()));
        }

        // ── Spawn workflow ────────────────────────────────────────────────────
        let mut workflow = CombatWorkflow::new(gm, players, chat_tx);
        let workflow_handle = tokio::spawn(async move { workflow.run().await });

        // ── Render loop ───────────────────────────────────────────────────────
        let render_backend = RatatuiBackend::new();
        let combat_result = 'combat: loop {
            // Drain GM narration and player prompts into the model.
            while let Ok(msg) = chat_rx.try_recv() {
                model.receive(msg);
            }

            // Enter compose mode if a ChatCommunicator requested it.
            while compose_rx.try_recv().is_ok() {
                if !model.is_composing() {
                    model.begin_compose();
                }
            }

            // Check if the workflow finished.
            if workflow_handle.is_finished() {
                break 'combat workflow_handle.await?;
            }

            // Render via IR pipeline.
            let (tree, _ir_proof) = model.to_verified_tree();
            let (tui_node, _stats, _render_proof) = render_backend
                .render(&tree)
                .map_err(|e| anyhow::anyhow!("IR render error: {e}"))?;
            terminal.draw(|frame| render_node(frame, frame.area(), &tui_node))?;

            // Poll for key events.
            if !event::poll(Duration::from_millis(50))? {
                continue;
            }
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc if !model.is_composing() => {
                        info!("Combat aborted by user");
                        workflow_handle.abort();
                        return Ok(());
                    }
                    KeyCode::Up | KeyCode::Char('k') if !model.is_composing() => {
                        model.scroll_up();
                    }
                    KeyCode::Down | KeyCode::Char('j') if !model.is_composing() => {
                        model.scroll_down();
                    }
                    KeyCode::Char('i') if !model.is_composing() => model.begin_compose(),
                    KeyCode::Enter if model.is_composing() => {
                        if let Some(text) = model.send_message() {
                            if reply_tx.send(text).is_err() {
                                warn!("Reply channel closed — workflow may have ended");
                            }
                        }
                    }
                    KeyCode::Esc if model.is_composing() => model.cancel_compose(),
                    KeyCode::Backspace if model.is_composing() => model.pop_char(),
                    KeyCode::Char(c) if model.is_composing() => model.push_char(c),
                    _ => {}
                }
            }
        };

        // ── Show result ───────────────────────────────────────────────────────
        // Drain any final messages from the channel.
        while let Ok(msg) = chat_rx.try_recv() {
            model.receive(msg);
        }
        match &combat_result {
            Ok(Some(winner)) => model.gm_say(format!("=== Team {winner} wins! ===")),
            Ok(None) => {
                model.gm_say("=== Draw — all combatants incapacitated ===".to_string());
            }
            Err(e) => model.gm_say(format!("=== Combat error: {e} ===")),
        }
        model.system_event("Press q or Esc to return to the lobby.");

        // ── Wait for acknowledgement ──────────────────────────────────────────
        loop {
            let (tree, _ir_proof) = model.to_verified_tree();
            let (tui_node, _stats, _render_proof) = render_backend
                .render(&tree)
                .map_err(|e| anyhow::anyhow!("IR render error: {e}"))?;
            terminal.draw(|frame| render_node(frame, frame.area(), &tui_node))?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press
                        && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
                    {
                        break;
                    }
                }
            }
        }

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
/// Loads `.env` via dotenvy (if present), handles terminal setup, drives
/// [`LobbyController::run`], and restores the terminal on exit regardless of
/// how the loop ends.
///
/// # Errors
///
/// Returns `Err` on terminal I/O failures or unrecoverable workflow errors.
#[instrument]
pub async fn run_lobby() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = LobbyController::new().run(&mut terminal).await;

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}
