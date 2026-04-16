//! # Valinoreth
//!
//! A Rust implementation of GURPS (Generic Universal RolePlaying System) game mechanics.
//!
//! ## Attribution
//!
//! GURPS is a trademark of Steve Jackson Games, and its rules and contents are copyrighted
//! by Steve Jackson Games. All rights are reserved by Steve Jackson Games.
//!
//! This is an unofficial, non-commercial implementation for personal use only.
//! This material is not official and is not endorsed by Steve Jackson Games.
//!
//! For official GURPS products, visit <http://www.sjgames.com/>

#![warn(missing_docs)]

mod advantages;
mod body;
mod character;
mod cli;
mod dice;
mod free;
mod movement;
mod players;
mod skills;
mod special_features;

pub use advantages::{
    AbsoluteDirection, Advantage, Disadvantage, Duty, EiditicMemory, Luck, Perk, SenseOfDuty,
    Wealth,
};
pub use body::BodyArea;
pub use character::{AttributeType, Attributes, BaseDamage, CombatStats, Encumbrance, Stats};
pub use cli::Cli;
pub use dice::{DieLevel, Random};
pub use free::trace_init;
pub use movement::{AllOutMeleeAttack, AllOutRangedAttack, FreeAction, Manuever, Posture, Success};
pub use players::Players;
pub use skills::Skill;
pub use special_features::SpecialFeatures;
