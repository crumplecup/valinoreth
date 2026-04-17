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

mod attack;
mod damage;
mod defense;
mod error;
mod weapons;

pub use attack::{AttackResult, AttackRoll, CombatModifiers, CombatModifiersBuilder, Modifier};
pub use damage::{Armor, ArmorBuilder, DamageResolution};
pub use defense::{
    calculate_block, calculate_dodge, calculate_parry, defense_succeeds, ActiveDefense,
    DefenseResult, RETREAT_BONUS,
};
pub use error::{CombatError, CombatErrorKind};
pub use weapons::{DamageType, MeleeWeapon, RangedWeapon, Reach, Weapon, WeaponDamage};
