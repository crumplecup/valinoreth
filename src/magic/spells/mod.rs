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
mod body_control;
mod core;
mod earth;
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
