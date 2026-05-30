//! [`CombatSession`] — shared handle to server-side combat state.
//!
//! The workflow holds the authoritative [`CombatPhase`] and mutates it as the
//! encounter progresses.  UI observers (TUI spectator, REST clients) read a
//! serializable [`CombatStateView`] snapshot without touching proof tokens or
//! live game references.
//!
//! This mirrors the `BlackjackSession` / `BlackjackStateView` pattern from
//! strictly_games: **the server holds the logic; observers only get a view.**

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{CombatState, CombatantState};

// ── CombatPhase ───────────────────────────────────────────────────────────────

/// Current lifecycle phase of a GURPS combat encounter.
///
/// Stored in a [`CombatSession`] mutex.  The workflow driver mutates this
/// after each VSM transition; UI observers read [`CombatStateView`] snapshots.
#[derive(Debug)]
pub enum CombatPhase {
    /// No combat has been initialized yet.
    Unstarted,
    /// Combat is underway.  Holds the authoritative VSM state.
    Active(CombatState),
    /// Combat has concluded.
    Concluded {
        /// Winning team name, or `None` for a draw.
        winner: Option<String>,
    },
}

// ── CombatSession ─────────────────────────────────────────────────────────────

/// Shared, async-safe handle to the per-encounter combat phase state.
///
/// Pass a clone to any UI observer that needs to poll game state.  The
/// workflow is the only writer; observers read [`CombatStateView`] via
/// [`CombatStateView::from_phase`].
pub type CombatSession = Arc<Mutex<CombatPhase>>;

// ── CombatStateView ───────────────────────────────────────────────────────────

/// Serializable snapshot of combat state from a specific combatant's perspective.
///
/// Built from a [`CombatPhase`] lock guard; safe to clone and send across
/// threads.  The [`to_preamble`] method formats it as the knowledge-cache entry
/// that [`ContextualCommunicator`] prepends to every elicitation prompt.
///
/// [`to_preamble`]: CombatStateView::to_preamble
/// [`ContextualCommunicator`]: crate::ContextualCommunicator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatStateView {
    /// Current combat round (0 when not active).
    pub round: u32,
    /// Name of the combatant whose turn it currently is.
    pub whose_turn: String,
    /// Whether it is the viewer's own turn.
    pub is_your_turn: bool,
    /// Phase label: `"unstarted"`, `"active"`, or `"concluded"`.
    pub phase: String,
    /// Snapshot of all combatants visible to the viewer.
    pub combatants: Vec<CombatantView>,
}

/// Per-combatant snapshot included in a [`CombatStateView`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatantView {
    /// Combatant name.
    pub name: String,
    /// Team name.
    pub team: String,
    /// Current hit points.
    pub current_hp: i32,
    /// Maximum hit points.
    pub max_hp: i32,
    /// True when HP ≤ 0.
    pub is_incapacitated: bool,
    /// True when this is the observer's own entry.
    pub is_you: bool,
}

impl CombatStateView {
    /// Build a view from the current combat phase for the given `viewer_name`.
    pub fn from_phase(phase: &CombatPhase, viewer_name: &str) -> Self {
        match phase {
            CombatPhase::Unstarted => Self {
                round: 0,
                whose_turn: String::new(),
                is_your_turn: false,
                phase: "unstarted".to_string(),
                combatants: vec![],
            },

            CombatPhase::Active(CombatState::Active {
                combatants,
                turn_order,
                current_actor,
                round,
            }) => {
                let actor_slot = turn_order[*current_actor];
                let whose_turn = combatants[actor_slot].id.clone();
                let is_your_turn = whose_turn == viewer_name;
                Self {
                    round: *round,
                    whose_turn,
                    is_your_turn,
                    phase: "active".to_string(),
                    combatants: combatant_views(combatants, viewer_name),
                }
            }

            CombatPhase::Active(_) => Self {
                round: 0,
                whose_turn: String::new(),
                is_your_turn: false,
                phase: "active".to_string(),
                combatants: vec![],
            },

            CombatPhase::Concluded { winner } => Self {
                round: 0,
                whose_turn: winner.clone().unwrap_or_default(),
                is_your_turn: false,
                phase: "concluded".to_string(),
                combatants: vec![],
            },
        }
    }

    /// Format the view as a plain-text preamble for elicitation prompts.
    ///
    /// This is the string pushed into a [`KnowledgeCache`] before each
    /// player decision, so the player always sees current combat state.
    ///
    /// [`KnowledgeCache`]: crate::KnowledgeCache
    pub fn to_preamble(&self) -> String {
        match self.phase.as_str() {
            "unstarted" => "Combat has not yet begun.".to_string(),

            "concluded" => {
                if self.whose_turn.is_empty() {
                    "Combat has concluded — draw!".to_string()
                } else {
                    format!("Combat has concluded. {} wins!", self.whose_turn)
                }
            }

            "active" => {
                let turn_note = if self.is_your_turn {
                    " — YOUR TURN"
                } else {
                    ""
                };
                let mut s = format!(
                    "[Combat — Round {}]\nActive combatant: {}{}\n\n",
                    self.round, self.whose_turn, turn_note
                );
                s.push_str("All combatants:\n");
                for c in &self.combatants {
                    let you_marker = if c.is_you { " [YOU]" } else { "" };
                    let status = if c.is_incapacitated {
                        " (INCAPACITATED)"
                    } else {
                        ""
                    };
                    s.push_str(&format!(
                        "  {} (team: {}): HP {}/{}{}{}.\n",
                        c.name, c.team, c.current_hp, c.max_hp, you_marker, status
                    ));
                }
                s
            }

            _ => String::new(),
        }
    }
}

fn combatant_views(combatants: &[CombatantState], viewer_name: &str) -> Vec<CombatantView> {
    combatants
        .iter()
        .map(|c| CombatantView {
            name: c.id.clone(),
            team: c.team.clone(),
            current_hp: c.current_hp,
            max_hp: c.max_hp,
            is_incapacitated: c.incapacitated,
            is_you: c.id == viewer_name,
        })
        .collect()
}
