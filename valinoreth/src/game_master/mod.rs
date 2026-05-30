//! GameMaster - concrete implementation of all trait interfaces.
//!
//! The GameMaster acts as the "lead actor" in state machine plays,
//! implementing all GURPS rules through the trait contracts.

mod attack;
mod character_advancement;
mod character_builder;
mod combat;
mod damage;
mod defense;
mod maneuver;
mod skill_check;
mod skill_manager;
mod spell_caster;
mod spell_effect_resolver;
mod spell_executor;
mod spell_manager;

use crate::ThreeDiceRoll;
use elicitation::Generator;

/// Configuration for the GameMaster.
#[derive(Debug, Clone)]
pub struct GameMasterConfig {
    /// Mana level for the game world.
    pub mana_level: ManaLevel,
    /// Random seed for deterministic dice generation.
    pub seed: u64,
}

impl Default for GameMasterConfig {
    fn default() -> Self {
        Self {
            mana_level: ManaLevel::Normal,
            seed: 42, // Default seed for reproducibility
        }
    }
}

/// Mana levels for spell casting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManaLevel {
    /// No magic works
    NoMana,
    /// Magic at -5
    Low,
    /// Magic works normally
    Normal,
    /// Magic at +5
    High,
    /// Magic always succeeds
    VeryHigh,
}

/// GameMaster implementation.
///
/// Serves as the concrete backend for all GURPS trait interfaces.
/// Uses elicitation's Generator system for deterministic, seeded dice rolls.
#[derive(Debug)]
pub struct GameMaster {
    /// Configuration
    config: GameMasterConfig,
}

impl GameMaster {
    /// Create a new GameMaster with default configuration.
    pub fn new() -> Self {
        Self::with_config(GameMasterConfig::default())
    }

    /// Create a new GameMaster with custom configuration.
    pub fn with_config(config: GameMasterConfig) -> Self {
        Self { config }
    }

    /// Roll 3d6 for GURPS checks.
    ///
    /// Returns a ThreeDiceRoll with sum between 3 and 18.
    ///
    /// Creates a fresh generator each time using the configured seed.
    /// This ensures deterministic rolls for testing and replay.
    pub(crate) fn roll_3d6(&self) -> ThreeDiceRoll {
        ThreeDiceRoll::random_generator(self.config.seed).generate()
    }
}

impl Default for GameMaster {
    fn default() -> Self {
        Self::new()
    }
}
