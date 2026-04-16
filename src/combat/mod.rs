//! Combat system implementation.
//!
//! # GURPS Rules
//!
//! Combat in GURPS is resolved through:
//! - Attack rolls (3d6 vs skill)
//! - Active defense (Dodge/Parry/Block)
//! - Damage resolution (roll dice, apply DR, location multipliers)
//!
//! # Citations
//!
//! - BS 356-358 - Attack rolls
//! - BS 374-377 - Active defenses
//! - BS 378-380 - Damage and injury

mod error;

pub use error::{CombatError, CombatErrorKind};
