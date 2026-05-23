//! GURPS magic spells module.
//!
//! # GURPS Rules
//!
//! Spells are organized into colleges, require energy costs (FP),
//! have prerequisites (Magery, other spells), and use skill rolls.
//!
//! # Citations
//!
//! BS 239-253 - Magic system
//! M 10-200 - Individual spells

mod air;
mod animal;
mod body_control;
mod communication_empathy;
mod core;
mod earth;
mod enchantment;
mod fire;
mod food;
mod gate;
mod healing;
mod illusion_creation;
mod knowledge;
mod light_darkness;
mod making_breaking;
mod meta_spells;
mod mind_control;
mod movement;
mod necromantic;
mod plant;
mod protection_warning;
mod sound;
mod technological;
mod types;
mod water;
mod weather;

pub use core::Spell;
pub use types::{Duration, EnergyCost, ResistanceType, SpellPrerequisite, SpellType};

// Re-export SpellCollege from contracts for convenience
pub use crate::contracts::SpellCollege;
