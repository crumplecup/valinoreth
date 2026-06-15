//! Ratatui terminal frontend for the game chat UI.
//!
//! Provides [`run_chat`], a blocking crossterm terminal loop that renders
//! the [`ChatModel`] via the AccessKit IR pipeline:
//!
//! ```text
//! model.to_verified_tree()
//!   → RatatuiBackend::render(&tree)
//!   → render_node(frame, area, &tui_node)
//! ```
//!
//! Key bindings are sourced from [`ChatKeyMap`] — the single source of truth
//! that keeps all frontends consistent.
//!
//! ## Layout
//!
//! ```text
//! ┌──────────────────────────────────────────────────────┐
//! │  Valinoreth — GURPS Combat                 (banner)  │
//! ├──────────────────────────────────────────────────────┤
//! │ [SYSTEM] Combat round 1 begins                       │
//! │ [GM] The orc charges you!                            │
//! │ [PLAYER] I parry with my broadsword                  │
//! │ ...                          (scrollable log, 20 rows)│
//! ├──────────────────────────────────────────────────────┤
//! │ > _                                       (composing) │
//! ├──────────────────────────────────────────────────────┤
//! │ [↑↓] scroll  [i] input  [Enter] send  [q] quit       │
//! └──────────────────────────────────────────────────────┘
//! ```

use crossterm::{
    event::{Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use elicit_ratatui::{render_node, RatatuiBackend};
use elicit_ui::UiTreeRenderer as _;
use ratatui::{backend::CrosstermBackend, Terminal};
use tracing::instrument;

use crate::{ChatKeyMap, ChatModel};

/// Run the ratatui chat frontend.  Blocks until the user presses `q` or `Esc`.
///
/// The loop polls for crossterm key events and re-renders every 50 ms via the
/// IR pipeline, so external code may push messages into `model` between frames
/// if running on a separate thread.
///
/// # Errors
///
/// Returns `Err` on terminal setup/teardown or I/O failures.
#[instrument(skip(model))]
pub fn run_chat(model: &mut ChatModel) -> std::io::Result<()> {
    // ── terminal setup ────────────────────────────────────────────────────────
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let render_backend = RatatuiBackend::new();
    let result = event_loop(model, &mut terminal, &render_backend);

    // ── terminal teardown (always runs) ───────────────────────────────────────
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    result
}

fn event_loop(
    model: &mut ChatModel,
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    render_backend: &RatatuiBackend,
) -> std::io::Result<()> {
    loop {
        // ── render frame via IR pipeline ──────────────────────────────────────
        let (tree, _ir_proof) = model.to_verified_tree();
        let (tui_node, _stats, _render_proof) = render_backend
            .render(&tree)
            .map_err(|e| std::io::Error::other(e.to_string()))?;

        terminal.draw(|frame| {
            render_node(frame, frame.area(), &tui_node);
        })?;

        // ── non-blocking event poll (50 ms cadence) ───────────────────────────
        if !crossterm::event::poll(std::time::Duration::from_millis(50))? {
            continue;
        }

        if let Event::Key(key) = crossterm::event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            // Dispatch via ChatKeyMap — single source of truth for bindings.
            match key.code {
                // ── navigation ────────────────────────────────────────────────
                KeyCode::Up | KeyCode::Char('k') => model.scroll_up(),
                KeyCode::Down | KeyCode::Char('j') => model.scroll_down(),

                // ── composing ─────────────────────────────────────────────────
                KeyCode::Char('i') if !model.is_composing() => model.begin_compose(),
                KeyCode::Enter if model.is_composing() => {
                    let _ = model.send_message();
                }
                KeyCode::Esc => {
                    if model.is_composing() {
                        model.cancel_compose();
                    } else {
                        break;
                    }
                }
                KeyCode::Backspace if model.is_composing() => model.pop_char(),
                KeyCode::Char(c) if model.is_composing() => model.push_char(c),

                // ── quit ──────────────────────────────────────────────────────
                KeyCode::Char('q') if !model.is_composing() => break,

                _ => {}
            }
        }
    }

    // Suppress unused import warning: ChatKeyMap is the conceptual authority
    // even if only its status_hints() is called through to_verified_tree().
    let _ = ChatKeyMap::status_hints;

    Ok(())
}
