//! Combat setup screen — configure participants before starting an encounter.
//!
//! Players can set the number of combat slots (2–4), cycle through pre-built
//! characters, toggle Human/Agent per slot, and start the encounter.
//!
//! ## Key bindings
//!
//! | Key | Action |
//! |-----|--------|
//! | ← / h | previous slot |
//! | → / l | next slot |
//! | ↑ / k | previous character |
//! | ↓ / j | next character |
//! | t | toggle Human / Agent |
//! | + | add slot (max 4) |
//! | - | remove slot (min 2) |
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

/// Minimum number of combat slots.
const MIN_SLOTS: usize = 2;

/// Team labels cycled round-robin across slots.
const TEAMS: &[&str] = &["A", "B", "C", "D", "E", "F", "G", "H"];

/// Combat setup screen state.
#[derive(Debug)]
pub struct CombatSetupScreen {
    roster: Vec<RosterEntry>,
    /// Roster index chosen for each slot.
    roster_indices: Vec<usize>,
    /// Which slot has keyboard focus.
    focused: usize,
    /// Controller kind for each slot.
    kinds: Vec<PlayerKind>,
    /// Default agent config loaded from `agent_config.toml`, if present.
    default_agent: crate::AgentConfig,
}

impl CombatSetupScreen {
    /// Creates a new combat setup screen with 2 default slots.
    ///
    /// Tries to load `agent_config.toml` from the working directory.  Falls
    /// back to default Anthropic Haiku settings if the file is absent or
    /// unreadable.  Slot 0 → Human/Fighter; Slot 1 → Agent (loaded config).
    #[instrument]
    pub fn new() -> Self {
        debug!("Initializing CombatSetupScreen");
        let roster = default_roster();
        let default_agent = crate::AgentConfig::from_file("agent_config.toml")
            .unwrap_or_else(|_| crate::AgentConfig::new("Agent"));
        Self {
            roster_indices: vec![0, 1],
            focused: 0,
            kinds: vec![PlayerKind::Human, PlayerKind::Agent(default_agent.clone())],
            default_agent,
            roster,
        }
    }

    fn num_slots(&self) -> usize {
        self.roster_indices.len()
    }

    #[instrument(skip(self))]
    fn add_slot(&mut self) {
        let new_idx = self.num_slots() % self.roster.len();
        self.roster_indices.push(new_idx);
        self.kinds.push(PlayerKind::Human);
    }

    #[instrument(skip(self))]
    fn remove_slot(&mut self) {
        if self.num_slots() <= MIN_SLOTS {
            return;
        }
        self.roster_indices.pop();
        self.kinds.pop();
        if self.focused >= self.num_slots() {
            self.focused = self.num_slots() - 1;
        }
    }

    #[instrument(skip(self))]
    fn next_character(&mut self) {
        let len = self.roster.len();
        self.roster_indices[self.focused] = (self.roster_indices[self.focused] + 1) % len;
    }

    #[instrument(skip(self))]
    fn prev_character(&mut self) {
        let len = self.roster.len();
        let idx = self.roster_indices[self.focused];
        self.roster_indices[self.focused] = if idx == 0 { len - 1 } else { idx - 1 };
    }

    #[instrument(skip(self))]
    fn toggle_kind(&mut self) {
        self.kinds[self.focused] = match &self.kinds[self.focused] {
            PlayerKind::Human => PlayerKind::Agent(self.default_agent.clone()),
            PlayerKind::Agent(_) => PlayerKind::Human,
        };
    }

    #[instrument(skip(self))]
    fn build_slots(&self) -> Vec<CombatSlot> {
        self.roster_indices
            .iter()
            .enumerate()
            .map(|(i, &ri)| {
                let entry = &self.roster[ri];
                CombatSlot {
                    name: entry.character.name.clone(),
                    character: entry.character.clone(),
                    team: TEAMS[i % TEAMS.len()].to_string(),
                    kind: self.kinds[i].clone(),
                }
            })
            .collect()
    }

    fn render_slot(&self, frame: &mut Frame, area: Rect, slot_idx: usize, focused: bool) {
        let entry = &self.roster[self.roster_indices[slot_idx]];
        let kind = &self.kinds[slot_idx];
        let team = TEAMS[slot_idx % TEAMS.len()];

        let border_style = if focused {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        let block = Block::default()
            .title(format!(" Team {team} "))
            .borders(Borders::ALL)
            .border_style(border_style);

        let inner = block.inner(area);
        frame.render_widget(block, area);

        let lines: Vec<ratatui::text::Line> = vec![
            ratatui::text::Line::from(vec![
                ratatui::text::Span::styled("Character: ", Style::default().fg(Color::Gray)),
                ratatui::text::Span::styled(
                    entry.label,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            ratatui::text::Line::from(ratatui::text::Span::styled(
                entry.description,
                Style::default().fg(Color::DarkGray),
            )),
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

        frame.render_widget(Paragraph::new(lines), inner);
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

        let title = Paragraph::new(format!("Combat Setup  ({} participants)", self.num_slots()))
            .style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::BOTTOM));
        frame.render_widget(title, chunks[0]);

        // Split the slots area into equal columns.
        let col_constraints: Vec<Constraint> = (0..self.num_slots())
            .map(|_| Constraint::Ratio(1, self.num_slots() as u32))
            .collect();

        let slot_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(col_constraints)
            .split(chunks[1]);

        for i in 0..self.num_slots() {
            self.render_slot(frame, slot_chunks[i], i, self.focused == i);
        }

        let help = Paragraph::new(
            "[←→/hl] focus  [↑↓/jk] character  [t] Human/Agent  [+/-] add/remove slot  [Enter] start  [Esc] back",
        )
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);
        frame.render_widget(help, chunks[2]);
    }

    fn handle_key(&mut self, key: KeyEvent) -> ScreenTransition {
        match key.code {
            KeyCode::Left | KeyCode::Char('h') => {
                self.focused = if self.focused == 0 {
                    self.num_slots() - 1
                } else {
                    self.focused - 1
                };
                ScreenTransition::Stay
            }
            KeyCode::Right | KeyCode::Char('l') => {
                self.focused = (self.focused + 1) % self.num_slots();
                ScreenTransition::Stay
            }
            KeyCode::Up | KeyCode::Char('k') => {
                self.prev_character();
                ScreenTransition::Stay
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.next_character();
                ScreenTransition::Stay
            }
            KeyCode::Char('t') => {
                self.toggle_kind();
                ScreenTransition::Stay
            }
            KeyCode::Char('+') => {
                self.add_slot();
                ScreenTransition::Stay
            }
            KeyCode::Char('-') => {
                self.remove_slot();
                ScreenTransition::Stay
            }
            KeyCode::Enter => ScreenTransition::StartCombat {
                slots: self.build_slots(),
            },
            KeyCode::Esc => ScreenTransition::GoToMainLobby,
            _ => ScreenTransition::Stay,
        }
    }
}
