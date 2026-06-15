//! Main lobby screen — navigation hub.

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};
use tracing::{debug, instrument};

use crate::lobby::screen::{Screen, ScreenTransition};

/// Menu entries on the main lobby screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LobbyOption {
    StartCombat,
    Settings,
    Quit,
}

impl LobbyOption {
    #[instrument]
    fn label(self) -> &'static str {
        match self {
            Self::StartCombat => "Start Combat",
            Self::Settings => "Settings",
            Self::Quit => "Quit",
        }
    }

    #[instrument]
    fn all() -> &'static [LobbyOption] {
        &[Self::StartCombat, Self::Settings, Self::Quit]
    }
}

/// Main lobby hub screen.
#[derive(Debug)]
pub struct MainLobbyScreen {
    list_state: ListState,
}

impl MainLobbyScreen {
    /// Creates a new main lobby screen with the cursor on the first option.
    #[instrument]
    pub fn new() -> Self {
        debug!("Initializing MainLobbyScreen");
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self { list_state }
    }

    #[instrument(skip(self))]
    fn select_previous(&mut self) {
        let count = LobbyOption::all().len();
        let i = match self.list_state.selected() {
            Some(i) if i > 0 => i - 1,
            _ => count - 1,
        };
        self.list_state.select(Some(i));
    }

    #[instrument(skip(self))]
    fn select_next(&mut self) {
        let count = LobbyOption::all().len();
        let i = match self.list_state.selected() {
            Some(i) => (i + 1) % count,
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    #[instrument(skip(self))]
    fn selected_option(&self) -> LobbyOption {
        LobbyOption::all()[self.list_state.selected().unwrap_or(0)]
    }
}

impl Default for MainLobbyScreen {
    fn default() -> Self {
        Self::new()
    }
}

impl Screen for MainLobbyScreen {
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

        let title = Paragraph::new("⚔  Valinoreth — GURPS Combat  ⚔")
            .style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::BOTTOM));
        frame.render_widget(title, chunks[0]);

        let items: Vec<ListItem> = LobbyOption::all()
            .iter()
            .map(|opt| ListItem::new(opt.label()))
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
        let mut state = self.list_state;
        frame.render_stateful_widget(list, chunks[1], &mut state);

        let help = Paragraph::new("[↑↓ / jk] navigate   [Enter] select   [q] quit")
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
            KeyCode::Enter => match self.selected_option() {
                LobbyOption::StartCombat => ScreenTransition::GoToCombatSetup,
                LobbyOption::Settings => ScreenTransition::GoToSettings,
                LobbyOption::Quit => ScreenTransition::Quit,
            },
            KeyCode::Char('q') => ScreenTransition::Quit,
            _ => ScreenTransition::Stay,
        }
    }
}
