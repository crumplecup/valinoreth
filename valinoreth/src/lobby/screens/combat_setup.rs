//! Combat setup screen — configure participants before starting an encounter.
//!
//! Shows two slots (Team A vs Team B) in a side-by-side layout.  The player
//! can cycle through pre-built characters and toggle Human/Agent for each slot.
//!
//! ## Key bindings
//!
//! | Key | Action |
//! |-----|--------|
//! | ↑ / k | previous slot |
//! | ↓ / j | next slot |
//! | ← / h | previous character |
//! | → / l | next character |
//! | t | toggle Human / Agent |
//! | Enter | start combat |
//! | Esc | back to main lobby |

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
};
use tracing::{debug, instrument};

use crate::lobby::screen::{Screen, ScreenTransition};
use crate::lobby::settings::{CombatSlot, PlayerKind, RosterEntry, default_roster};

/// Combat setup screen state.
#[derive(Debug)]
pub struct CombatSetupScreen {
    /// All available characters.
    roster: Vec<RosterEntry>,
    /// Currently selected roster index per slot.
    roster_indices: [usize; 2],
    /// Which slot has keyboard focus (0 = Team A, 1 = Team B).
    focused: usize,
    /// Controller kind for each slot.
    kinds: [PlayerKind; 2],
}

impl CombatSetupScreen {
    /// Creates a new combat setup screen with default slot configuration.
    ///
    /// Team A starts as Human/Fighter; Team B starts as Agent/Scout.
    #[instrument]
    pub fn new() -> Self {
        debug!("Initializing CombatSetupScreen");
        let roster = default_roster();
        let kinds = [PlayerKind::Human, PlayerKind::Agent(crate::AgentConfig::new("Agent"))];
        Self { roster, roster_indices: [0, 1], focused: 0, kinds }
    }

    /// Cycles the character selection for the focused slot forward.
    #[instrument(skip(self))]
    fn next_character(&mut self) {
        let len = self.roster.len();
        self.roster_indices[self.focused] = (self.roster_indices[self.focused] + 1) % len;
    }

    /// Cycles the character selection for the focused slot backward.
    #[instrument(skip(self))]
    fn prev_character(&mut self) {
        let len = self.roster.len();
        let idx = self.roster_indices[self.focused];
        self.roster_indices[self.focused] = if idx == 0 { len - 1 } else { idx - 1 };
    }

    /// Toggles the controller kind for the focused slot.
    #[instrument(skip(self))]
    fn toggle_kind(&mut self) {
        let kind = std::mem::replace(&mut self.kinds[self.focused], PlayerKind::Human);
        self.kinds[self.focused] = kind.toggle();
    }

    /// Builds the two [`CombatSlot`]s from the current configuration.
    #[instrument(skip(self))]
    fn build_slots(&self) -> Vec<CombatSlot> {
        let teams = ["A", "B"];
        self.roster_indices
            .iter()
            .enumerate()
            .map(|(i, &ri)| {
                let entry = &self.roster[ri];
                CombatSlot {
                    name: entry.character.name.clone(),
                    character: entry.character.clone(),
                    team: teams[i].to_string(),
                    kind: self.kinds[i].clone(),
                }
            })
            .collect()
    }

    /// Renders a single slot card.
    #[instrument(skip(frame))]
    fn render_slot(
        &self,
        frame: &mut Frame,
        area: Rect,
        slot_idx: usize,
        focused: bool,
    ) {
        let entry = &self.roster[self.roster_indices[slot_idx]];
        let kind = &self.kinds[slot_idx];
        let team = if slot_idx == 0 { "A" } else { "B" };

        let border_style = if focused {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::default()
            .title(format!(" Team {} ", team))
            .borders(Borders::ALL)
            .border_style(border_style);

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let lines: Vec<ratatui::text::Line> = vec![
            ratatui::text::Line::from(vec![
                ratatui::text::Span::styled("Character: ", Style::default().fg(Color::Gray)),
                ratatui::text::Span::styled(
                    entry.label,
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                ),
            ]),
            ratatui::text::Line::from(vec![ratatui::text::Span::styled(
                entry.description,
                Style::default().fg(Color::DarkGray),
            )]),
            ratatui::text::Line::from(""),
            ratatui::text::Line::from(vec![
                ratatui::text::Span::styled("Controller: ", Style::default().fg(Color::Gray)),
                ratatui::text::Span::styled(
                    kind.label(),
                    Style::default()
                        .fg(if matches!(kind, PlayerKind::Human) {
                            Color::Cyan
                        } else {
                            Color::Magenta
                        })
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
        ];

        let para = Paragraph::new(lines);
        frame.render_widget(para, inner);
    }
}

impl Default for CombatSetupScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl Screen for CombatSetupScreen {
    fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(6),
                Constraint::Length(3),
            ])
            .split(area);

        let title = Paragraph::new("Combat Setup")
            .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::BOTTOM));
        frame.render_widget(title, chunks[0]);

        let slots_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[1]);

        self.render_slot(frame, slots_chunks[0], 0, self.focused == 0);
        self.render_slot(frame, slots_chunks[1], 1, self.focused == 1);

        let help = Paragraph::new(
            "[↑↓/jk] select slot   [←→/hl] character   [t] toggle kind   [Enter] start   [Esc] back",
        )
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
        frame.render_widget(help, chunks[2]);
    }

    fn handle_key(&mut self, key: KeyEvent) -> ScreenTransition {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.focused = if self.focused == 0 { 1 } else { 0 };
                ScreenTransition::Stay
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.focused = (self.focused + 1) % 2;
                ScreenTransition::Stay
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.next_character();
                ScreenTransition::Stay
            }
            KeyCode::Left | KeyCode::Char('h') => {
                self.prev_character();
                ScreenTransition::Stay
            }
            KeyCode::Char('t') => {
                self.toggle_kind();
                ScreenTransition::Stay
            }
            KeyCode::Enter => ScreenTransition::StartCombat { slots: self.build_slots() },
            KeyCode::Esc => ScreenTransition::GoToMainLobby,
            _ => ScreenTransition::Stay,
        }
    }
}
