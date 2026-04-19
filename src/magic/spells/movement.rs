//! Movement college spells.
//!
//! # GURPS Rules
//!
//! Movement spells affect physical motion, including levitation,
//! teleportation, and telekinesis.
//!
//! # Citations
//!
//! M 146-161 - Movement college

use crate::{Duration, EnergyCost, ResistanceType, Spell, SpellPrerequisite, SpellType};
use tracing::{debug, instrument};

/// Returns base energy cost for Movement spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Movement spell energy cost");
    match spell {
        Spell::Levitation => EnergyCost::Fixed(2),
        Spell::Flight => EnergyCost::Fixed(4),
        Spell::Blink => EnergyCost::Fixed(2),
        Spell::Teleport => EnergyCost::Fixed(5),
        Spell::Teleportation => EnergyCost::Fixed(8),
        Spell::HasteMovement => EnergyCost::Fixed(3),
        Spell::SlowMovement => EnergyCost::Fixed(3),
        Spell::Cling => EnergyCost::Fixed(2),
        Spell::Jump => EnergyCost::Fixed(2),
        Spell::Apportation => EnergyCost::Fixed(3),
        Spell::Poltergeist => EnergyCost::Fixed(1),
        Spell::Lockmaster => EnergyCost::Fixed(2),
        Spell::TelekineticBlow => EnergyCost::PerDie(1),
        Spell::HaltMovement => EnergyCost::Fixed(3),
        _ => panic!("Invalid spell {:?} for Movement college", spell),
    }
}

/// Returns casting time for Movement spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Movement spell casting time");
    match spell {
        Spell::Levitation => 2,
        Spell::Flight => 3,
        Spell::Blink => 1,
        Spell::Teleport => 3,
        Spell::Teleportation => 5,
        Spell::HasteMovement => 2,
        Spell::SlowMovement => 2,
        Spell::Cling => 1,
        Spell::Jump => 1,
        Spell::Apportation => 2,
        Spell::Poltergeist => 1,
        Spell::Lockmaster => 1,
        Spell::TelekineticBlow => 1,
        Spell::HaltMovement => 2,
        _ => panic!("Invalid spell {:?} for Movement college", spell),
    }
}

/// Returns duration for Movement spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Movement spell duration");
    match spell {
        Spell::Levitation => Duration::Minutes(1),
        Spell::Flight => Duration::Minutes(1),
        Spell::Blink => Duration::Instant,
        Spell::Teleport => Duration::Instant,
        Spell::Teleportation => Duration::Permanent,
        Spell::HasteMovement => Duration::Minutes(1),
        Spell::SlowMovement => Duration::Minutes(1),
        Spell::Cling => Duration::Minutes(1),
        Spell::Jump => Duration::Instant,
        Spell::Apportation => Duration::Instant,
        Spell::Poltergeist => Duration::Concentration,
        Spell::Lockmaster => Duration::Instant,
        Spell::TelekineticBlow => Duration::Instant,
        Spell::HaltMovement => Duration::Minutes(1),
        _ => panic!("Invalid spell {:?} for Movement college", spell),
    }
}

/// Returns prerequisites for Movement spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Movement spell prerequisites");
    match spell {
        Spell::Levitation => vec![SpellPrerequisite::Magery(1)],
        Spell::Flight => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Levitation),
        ],
        Spell::Blink => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Levitation),
            SpellPrerequisite::Spell(Spell::Apportation),
        ],
        Spell::Teleport => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Blink),
        ],
        Spell::Teleportation => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Teleport),
        ],
        Spell::HasteMovement => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Apportation),
        ],
        Spell::SlowMovement => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Apportation),
        ],
        Spell::Cling => vec![SpellPrerequisite::Magery(1)],
        Spell::Jump => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Apportation),
        ],
        Spell::Apportation => vec![SpellPrerequisite::Magery(1)],
        Spell::Poltergeist => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Apportation),
        ],
        Spell::Lockmaster => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Apportation),
        ],
        Spell::TelekineticBlow => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Poltergeist),
        ],
        Spell::HaltMovement => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::SlowMovement),
        ],
        _ => panic!("Invalid spell {:?} for Movement college", spell),
    }
}

/// Returns spell type for Movement spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Movement spell type");
    match spell {
        Spell::Levitation => SpellType::Regular,
        Spell::Flight => SpellType::Regular,
        Spell::Blink => SpellType::Regular,
        Spell::Teleport => SpellType::Regular,
        Spell::Teleportation => SpellType::Area,
        Spell::HasteMovement => SpellType::Regular,
        Spell::SlowMovement => SpellType::Regular,
        Spell::Cling => SpellType::Regular,
        Spell::Jump => SpellType::Regular,
        Spell::Apportation => SpellType::Regular,
        Spell::Poltergeist => SpellType::Regular,
        Spell::Lockmaster => SpellType::Regular,
        Spell::TelekineticBlow => SpellType::Missile,
        Spell::HaltMovement => SpellType::Regular,
        _ => panic!("Invalid spell {:?} for Movement college", spell),
    }
}

/// Returns resistance type for Movement spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Movement spell resistance");
    match spell {
        Spell::Levitation => None,
        Spell::Flight => None,
        Spell::Blink => None,
        Spell::Teleport => None,
        Spell::Teleportation => None,
        Spell::HasteMovement => None,
        Spell::SlowMovement => Some(ResistanceType::HT),
        Spell::Cling => None,
        Spell::Jump => None,
        Spell::Apportation => Some(ResistanceType::Will),
        Spell::Poltergeist => None,
        Spell::Lockmaster => None,
        Spell::TelekineticBlow => None,
        Spell::HaltMovement => Some(ResistanceType::HT),
        _ => panic!("Invalid spell {:?} for Movement college", spell),
    }
}

/// Returns page reference for Movement spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Movement spell reference");
    match spell {
        Spell::Levitation => "M154",
        Spell::Flight => "M152",
        Spell::Blink => "M159",
        Spell::Teleport => "M160",
        Spell::Teleportation => "M159",
        Spell::HasteMovement => "M153",
        Spell::SlowMovement => "M158",
        Spell::Cling => "M151",
        Spell::Jump => "M154",
        Spell::Apportation => "M160",
        Spell::Poltergeist => "M147",
        Spell::Lockmaster => "M155",
        Spell::TelekineticBlow => "M156",
        Spell::HaltMovement => "M153",
        _ => panic!("Invalid spell {:?} for Movement college", spell),
    }
}
