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
mod enchantment;
mod earth;
mod food;
mod gate;
mod light_darkness;
mod making_breaking;
mod meta_spells;
mod necromantic;
mod plant;
mod sound;
mod technological;
mod weather;
mod fire;
mod healing;
mod illusion_creation;
mod knowledge;
mod mind_control;
mod movement;
mod protection_warning;
mod types;
mod water;

pub use core::{Spell, SpellCollege};
pub use types::{Duration, EnergyCost, ResistanceType, SpellPrerequisite, SpellType};
