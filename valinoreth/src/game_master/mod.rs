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

use crate::{DiceGenerator, ThreeDiceRoll};
use elicitation::Generator;
use std::sync::Mutex;

/// Configuration for the GameMaster.
#[derive(Debug, Clone)]
pub struct GameMasterConfig {
    /// Mana level for the game world.
    pub mana_level: ManaLevel,
    /// Optional random seed for deterministic dice generation.
    pub seed: Option<u64>,
}

impl Default for GameMasterConfig {
    fn default() -> Self {
        Self {
            mana_level: ManaLevel::Normal,
            seed: None,
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
#[derive(Debug)]
pub struct GameMaster {
    /// Configuration
    _config: GameMasterConfig,
    /// Shared random stream consumed by all dice rolls.
    dice: Mutex<DiceGenerator>,
}

impl GameMaster {
    /// Create a new GameMaster with default configuration.
    pub fn new() -> Self {
        Self::with_config(GameMasterConfig::default())
    }

    /// Create a new GameMaster with custom configuration.
    pub fn with_config(config: GameMasterConfig) -> Self {
        let dice = match config.seed {
            Some(seed) => ThreeDiceRoll::random_generator(seed),
            None => DiceGenerator::from_entropy(),
        };

        Self {
            _config: config,
            dice: Mutex::new(dice),
        }
    }

    /// Create a new GameMaster with deterministic dice generation.
    pub fn with_seed(seed: u64) -> Self {
        Self::with_config(GameMasterConfig {
            seed: Some(seed),
            ..GameMasterConfig::default()
        })
    }

    pub(crate) fn roll_die(&self, sides: i32) -> i32 {
        let dice = self.dice.lock().expect("GameMaster dice mutex poisoned");
        dice.roll_die(sides)
    }

    /// Roll 3d6 for GURPS checks.
    ///
    /// Returns a ThreeDiceRoll with sum between 3 and 18.
    pub(crate) fn roll_3d6(&self) -> ThreeDiceRoll {
        let dice = self.dice.lock().expect("GameMaster dice mutex poisoned");
        dice.generate()
    }
}

impl Default for GameMaster {
    fn default() -> Self {
        Self::new()
    }
}
