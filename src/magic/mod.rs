//! Magic system implementation.
//!
//! # GURPS Rules
//!
//! Magic in GURPS uses:
//! - Spell casting (3d6 vs skill)
//! - Energy costs (FP expenditure)
//! - Prerequisites (Magery, other spells)
//! - Colleges (spell categories)
//!
//! # Citations
//!
//! - BS 239-253 - Magic overview
//! - M 10-200 - Spell descriptions

mod spells;

pub use spells::{
    Duration, EnergyCost, ResistanceType, Spell, SpellPrerequisite, SpellType,
};
