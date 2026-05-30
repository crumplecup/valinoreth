//! Settings screen — toggle lobby preferences.

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use tracing::{debug, instrument};

use crate::lobby::screen::{Screen, ScreenTransition};
use crate::lobby::settings::LobbySettings;

/// Toggleable setting entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SettingItem {
    ShowCombatState,
}

impl SettingItem {
    #[instrument]
    fn label(self) -> &'static str {
        match self {
            Self::ShowCombatState => "Show Combat State Panel",
        }
    }

    #[instrument]
    fn all() -> &'static [SettingItem] {
        &[Self::ShowCombatState]
    }
}

/// Settings screen.
#[derive(Debug)]
pub struct SettingsScreen {
    settings: LobbySettings,
    list_state: ListState,
}

impl SettingsScreen {
    /// Creates a settings screen reflecting the current [`LobbySettings`].
    #[instrument(skip(settings))]
    pub fn new(settings: LobbySettings) -> Self {
        debug!("Initializing SettingsScreen");
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            settings,
            list_state,
        }
    }

    /// Returns the settings as modified by the user.
    #[instrument(skip(self))]
    pub fn settings(&self) -> LobbySettings {
        self.settings
    }

    #[instrument(skip(self))]
    fn selected_item(&self) -> SettingItem {
        SettingItem::all()[self.list_state.selected().unwrap_or(0)]
    }

    #[instrument(skip(self))]
    fn value_label(&self, item: SettingItem) -> &'static str {
        match item {
            SettingItem::ShowCombatState => {
                if self.settings.show_combat_state {
                    "On"
                } else {
                    "Off"
                }
            }
        }
    }

    #[instrument(skip(self))]
    fn toggle_selected(&mut self) {
        match self.selected_item() {
            SettingItem::ShowCombatState => {
                self.settings.show_combat_state = !self.settings.show_combat_state;
            }
        }
    }

    #[instrument(skip(self))]
    fn select_previous(&mut self) {
        let count = SettingItem::all().len();
        let i = match self.list_state.selected() {
            Some(i) if i > 0 => i - 1,
            _ => count - 1,
        };
        self.list_state.select(Some(i));
    }

    #[instrument(skip(self))]
    fn select_next(&mut self) {
        let count = SettingItem::all().len();
        let i = match self.list_state.selected() {
            Some(i) => (i + 1) % count,
            None => 0,
        };
        self.list_state.select(Some(i));
    }
}

impl Screen for SettingsScreen {
    fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(3),
                Constraint::Length(1),
            ])
            .split(area);

        let title = Paragraph::new("Settings")
            .style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::BOTTOM));
        frame.render_widget(title, chunks[0]);

        let items: Vec<ListItem> = SettingItem::all()
            .iter()
            .map(|item| {
                let value = self.value_label(*item);
                let value_style = if value == "On" {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::Red)
                };
                ListItem::new(ratatui::text::Line::from(vec![
                    ratatui::text::Span::raw(format!("{}: ", item.label())),
                    ratatui::text::Span::styled(value, value_style),
                ]))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::NONE)
                    .padding(ratatui::widgets::Padding::horizontal(2)),
            )
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ ");
        let mut state = self.list_state.clone();
        frame.render_stateful_widget(list, chunks[1], &mut state);

        let help = Paragraph::new("[↑↓] navigate   [Enter/Space] toggle   [Esc] back")
            .style(Style::default().fg(Color::DarkGray))
            .alignment(Alignment::Center);
        frame.render_widget(help, chunks[2]);
    }

    fn handle_key(&mut self, key: KeyEvent) -> ScreenTransition {
        match key.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.select_previous();
                ScreenTransition::Stay
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.select_next();
                ScreenTransition::Stay
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                self.toggle_selected();
                ScreenTransition::Stay
            }
            KeyCode::Esc => ScreenTransition::GoToMainLobby,
            _ => ScreenTransition::Stay,
        }
    }
}
