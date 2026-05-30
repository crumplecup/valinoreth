//! Lobby settings and pre-built character roster for combat setup.

use tracing::instrument;

use crate::{
    AttributeType, CharacterDescriptor, CharacterDescriptorBuilder, DerivedStatsDescriptor,
    SkillDescriptorBuilder, SkillDifficulty,
};

use super::AgentConfig;

/// Who controls a combat slot.
#[derive(Debug, Clone)]
pub enum PlayerKind {
    /// The human at the keyboard.
    Human,
    /// An AI agent (uses [`TuiCommunicator`] until MCP is wired).
    ///
    /// [`TuiCommunicator`]: crate::TuiCommunicator
    Agent(AgentConfig),
}

impl PlayerKind {
    /// Display label used in the setup screen.
    #[instrument(skip(self))]
    pub fn label(&self) -> &str {
        match self {
            Self::Human => "Human",
            Self::Agent(_) => "Agent",
        }
    }

    /// Toggles between `Human` and a default `Agent`.
    #[instrument(skip(self))]
    pub fn toggle(self) -> Self {
        match self {
            Self::Human => Self::Agent(AgentConfig::new("Agent")),
            Self::Agent(_) => Self::Human,
        }
    }
}

/// One participant slot in a combat encounter.
#[derive(Debug, Clone)]
pub struct CombatSlot {
    /// Display name shown in the TUI.
    pub name: String,
    /// The GURPS character descriptor for this slot.
    pub character: CharacterDescriptor,
    /// Team assignment (e.g. `"A"` or `"B"`).
    pub team: String,
    /// Human or agent controller.
    pub kind: PlayerKind,
}

/// User-adjustable settings for the lobby.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LobbySettings {
    /// Whether to display combat state information during encounters.
    pub show_combat_state: bool,
}

impl Default for LobbySettings {
    #[instrument]
    fn default() -> Self {
        Self {
            show_combat_state: true,
        }
    }
}

impl LobbySettings {
    /// Creates lobby settings with defaults.
    #[instrument]
    pub fn new() -> Self {
        Self::default()
    }
}

/// A named pre-built GURPS character available in the combat setup screen.
#[derive(Debug, Clone)]
pub struct RosterEntry {
    /// Short label shown in the menu (e.g. `"Fighter"`).
    pub label: &'static str,
    /// One-line description (e.g. `"Sturdy melee combatant, Broadsword-12"`).
    pub description: &'static str,
    /// The full character descriptor.
    pub character: CharacterDescriptor,
}

/// Returns the default roster of pre-built characters.
///
/// Each entry is a simple GURPS archetype with pre-filled derived stats and one
/// melee skill.  The workflow's `melee_skill` helper picks up the first
/// DX/ST-based skill automatically.
#[instrument]
pub fn default_roster() -> Vec<RosterEntry> {
    vec![
        RosterEntry {
            label: "Fighter",
            description: "Sturdy melee combatant — Broadsword-12, HP 12",
            character: CharacterDescriptorBuilder::default()
                .name("Fighter".to_string())
                .total_points(100)
                .points_spent(100)
                .attributes(vec![])
                .skills(vec![
                    SkillDescriptorBuilder::default()
                        .name("Broadsword".to_string())
                        .difficulty(SkillDifficulty::Average)
                        .base_attribute(AttributeType::DX)
                        .points(4)
                        .level(12)
                        .build()
                        .expect("valid skill"),
                ])
                .derived_stats(DerivedStatsDescriptor {
                    basic_speed: 5.5,
                    basic_move: 5,
                    dodge: 8,
                    hp: 12,
                    will: 10,
                    perception: 10,
                    fp: 12,
                })
                .build()
                .expect("valid Fighter"),
        },
        RosterEntry {
            label: "Scout",
            description: "Fast and evasive — Shortsword-10, Dodge 10",
            character: CharacterDescriptorBuilder::default()
                .name("Scout".to_string())
                .total_points(100)
                .points_spent(100)
                .attributes(vec![])
                .skills(vec![
                    SkillDescriptorBuilder::default()
                        .name("Shortsword".to_string())
                        .difficulty(SkillDifficulty::Average)
                        .base_attribute(AttributeType::DX)
                        .points(2)
                        .level(10)
                        .build()
                        .expect("valid skill"),
                ])
                .derived_stats(DerivedStatsDescriptor {
                    basic_speed: 6.5,
                    basic_move: 6,
                    dodge: 10,
                    hp: 10,
                    will: 10,
                    perception: 11,
                    fp: 10,
                })
                .build()
                .expect("valid Scout"),
        },
        RosterEntry {
            label: "Brute",
            description: "Tough brawler — Axe/Mace-13, HP 15",
            character: CharacterDescriptorBuilder::default()
                .name("Brute".to_string())
                .total_points(100)
                .points_spent(100)
                .attributes(vec![])
                .skills(vec![
                    SkillDescriptorBuilder::default()
                        .name("Axe/Mace".to_string())
                        .difficulty(SkillDifficulty::Average)
                        .base_attribute(AttributeType::DX)
                        .points(8)
                        .level(13)
                        .build()
                        .expect("valid skill"),
                ])
                .derived_stats(DerivedStatsDescriptor {
                    basic_speed: 4.0,
                    basic_move: 4,
                    dodge: 7,
                    hp: 15,
                    will: 10,
                    perception: 9,
                    fp: 15,
                })
                .build()
                .expect("valid Brute"),
        },
        RosterEntry {
            label: "Duelist",
            description: "Precise fencer — Rapier-14, good parry",
            character: CharacterDescriptorBuilder::default()
                .name("Duelist".to_string())
                .total_points(100)
                .points_spent(100)
                .attributes(vec![])
                .skills(vec![
                    SkillDescriptorBuilder::default()
                        .name("Rapier".to_string())
                        .difficulty(SkillDifficulty::Average)
                        .base_attribute(AttributeType::DX)
                        .points(8)
                        .level(14)
                        .build()
                        .expect("valid skill"),
                ])
                .derived_stats(DerivedStatsDescriptor {
                    basic_speed: 6.0,
                    basic_move: 6,
                    dodge: 9,
                    hp: 10,
                    will: 10,
                    perception: 10,
                    fp: 10,
                })
                .build()
                .expect("valid Duelist"),
        },
    ]
}
