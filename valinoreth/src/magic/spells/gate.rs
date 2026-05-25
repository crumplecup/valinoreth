//! Gate college spells.
//!
//! # GURPS Rules
//!
//! Gate spells enable teleportation, planar travel, and dimensional manipulation.
//! They include spells for instant transportation, creating portals, and accessing
//! other planes of existence.
//!
//! # Citations
//!
//! M 80-86 - Gate college

use crate::{Duration, EnergyCost, ResistanceType, Spell, SpellPrerequisite, SpellType};
use tracing::{debug, instrument};

/// Returns base energy cost for Gate spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Gate spell energy cost");
    match spell {
        Spell::Teleport => EnergyCost::Fixed(3),
        Spell::Blink => EnergyCost::Fixed(2),
        Spell::Apportation => EnergyCost::Fixed(1),
        Spell::Levitation => EnergyCost::Fixed(2),
        Spell::TeleportOther => EnergyCost::Fixed(4),
        Spell::Beacon => EnergyCost::Fixed(2),
        Spell::TraceTeleport => EnergyCost::Fixed(2),
        Spell::DivertTeleport => EnergyCost::Fixed(3),
        Spell::PlanarSummons => EnergyCost::Fixed(10),
        Spell::PlanarVisit => EnergyCost::Fixed(20),
        Spell::PlaneShift => EnergyCost::Fixed(10),
        Spell::PlaneShiftOther => EnergyCost::Fixed(15),
        Spell::Phase => EnergyCost::Fixed(5),
        Spell::PhaseOther => EnergyCost::Fixed(8),
        Spell::SeekGate => EnergyCost::Fixed(3),
        Spell::ControlGate => EnergyCost::Fixed(10),
        Spell::CreateGate => EnergyCost::Fixed(20),
        Spell::ScryGate => EnergyCost::Fixed(2),
        Spell::HideObject => EnergyCost::Fixed(5),
        Spell::Sanctuary => EnergyCost::Fixed(20),
        _ => panic!("Invalid spell {:?} for Gate college", spell),
    }
}

/// Returns casting time in seconds for Gate spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Gate spell casting time");
    match spell {
        Spell::Teleport => 3,
        Spell::Blink => 1,
        Spell::Apportation => 1,
        Spell::Levitation => 1,
        Spell::TeleportOther => 3,
        Spell::Beacon => 2,
        Spell::TraceTeleport => 2,
        Spell::DivertTeleport => 3,
        Spell::PlanarSummons => 10,
        Spell::PlanarVisit => 10,
        Spell::PlaneShift => 5,
        Spell::PlaneShiftOther => 5,
        Spell::Phase => 3,
        Spell::PhaseOther => 3,
        Spell::SeekGate => 3,
        Spell::ControlGate => 5,
        Spell::CreateGate => 10,
        Spell::ScryGate => 2,
        Spell::HideObject => 3,
        Spell::Sanctuary => 10,
        _ => panic!("Invalid spell {:?} for Gate college", spell),
    }
}

/// Returns duration for Gate spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Gate spell duration");
    match spell {
        Spell::Teleport => Duration::Instant,
        Spell::Blink => Duration::Instant,
        Spell::Apportation => Duration::Concentration,
        Spell::Levitation => Duration::Concentration,
        Spell::TeleportOther => Duration::Instant,
        Spell::Beacon => Duration::Permanent,
        Spell::TraceTeleport => Duration::Instant,
        Spell::DivertTeleport => Duration::Minutes(1),
        Spell::PlanarSummons => Duration::Minutes(10),
        Spell::PlanarVisit => Duration::Hours(1),
        Spell::PlaneShift => Duration::Permanent,
        Spell::PlaneShiftOther => Duration::Permanent,
        Spell::Phase => Duration::Minutes(1),
        Spell::PhaseOther => Duration::Minutes(1),
        Spell::SeekGate => Duration::Instant,
        Spell::ControlGate => Duration::Concentration,
        Spell::CreateGate => Duration::Minutes(10),
        Spell::ScryGate => Duration::Concentration,
        Spell::HideObject => Duration::Permanent,
        Spell::Sanctuary => Duration::Permanent,
        _ => panic!("Invalid spell {:?} for Gate college", spell),
    }
}

/// Returns prerequisites for Gate spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Gate spell prerequisites");
    match spell {
        Spell::Teleport => vec![SpellPrerequisite::Magery(1)],
        Spell::Blink => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Teleport),
        ],
        Spell::Apportation => vec![SpellPrerequisite::Magery(0)],
        Spell::Levitation => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Apportation),
        ],
        Spell::TeleportOther => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Teleport),
        ],
        Spell::Beacon => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Teleport),
        ],
        Spell::TraceTeleport => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Teleport),
        ],
        Spell::DivertTeleport => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::TraceTeleport),
        ],
        Spell::PlanarSummons => vec![SpellPrerequisite::Magery(1)],
        Spell::PlanarVisit => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::PlanarSummons),
        ],
        Spell::PlaneShift => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::PlanarSummons),
        ],
        Spell::PlaneShiftOther => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::PlaneShift),
        ],
        Spell::Phase => vec![SpellPrerequisite::Magery(3)],
        Spell::PhaseOther => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::Phase),
        ],
        Spell::SeekGate => vec![SpellPrerequisite::Magery(2)],
        Spell::ControlGate => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::SeekGate),
        ],
        Spell::CreateGate => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::ControlGate),
            SpellPrerequisite::Spell(Spell::Teleport),
        ],
        Spell::ScryGate => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::SeekGate),
        ],
        Spell::HideObject => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Teleport),
        ],
        Spell::Sanctuary => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::HideObject),
        ],
        _ => panic!("Invalid spell {:?} for Gate college", spell),
    }
}

/// Returns spell type for Gate spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Gate spell type");
    match spell {
        Spell::Teleport => SpellType::Regular,
        Spell::Blink => SpellType::Regular,
        Spell::Apportation => SpellType::Regular,
        Spell::Levitation => SpellType::Regular,
        Spell::TeleportOther => SpellType::Regular,
        Spell::Beacon => SpellType::Regular,
        Spell::TraceTeleport => SpellType::Information,
        Spell::DivertTeleport => SpellType::Regular,
        Spell::PlanarSummons => SpellType::Regular,
        Spell::PlanarVisit => SpellType::Regular,
        Spell::PlaneShift => SpellType::Regular,
        Spell::PlaneShiftOther => SpellType::Regular,
        Spell::Phase => SpellType::Regular,
        Spell::PhaseOther => SpellType::Regular,
        Spell::SeekGate => SpellType::Information,
        Spell::ControlGate => SpellType::Regular,
        Spell::CreateGate => SpellType::Regular,
        Spell::ScryGate => SpellType::Information,
        Spell::HideObject => SpellType::Regular,
        Spell::Sanctuary => SpellType::Regular,
        _ => panic!("Invalid spell {:?} for Gate college", spell),
    }
}

/// Returns resistance type for Gate spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Gate spell resistance");
    match spell {
        Spell::Teleport => None,
        Spell::Blink => None,
        Spell::Apportation => None,
        Spell::Levitation => None,
        Spell::TeleportOther => Some(ResistanceType::Will),
        Spell::Beacon => None,
        Spell::TraceTeleport => None,
        Spell::DivertTeleport => None,
        Spell::PlanarSummons => Some(ResistanceType::Will),
        Spell::PlanarVisit => None,
        Spell::PlaneShift => None,
        Spell::PlaneShiftOther => Some(ResistanceType::Will),
        Spell::Phase => None,
        Spell::PhaseOther => Some(ResistanceType::Will),
        Spell::SeekGate => None,
        Spell::ControlGate => None,
        Spell::CreateGate => None,
        Spell::ScryGate => None,
        Spell::HideObject => None,
        Spell::Sanctuary => None,
        _ => panic!("Invalid spell {:?} for Gate college", spell),
    }
}

/// Returns page reference for Gate spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Gate spell reference");
    match spell {
        Spell::Teleport => "M147",
        Spell::Blink => "M80",
        Spell::Apportation => "M80",
        Spell::Levitation => "M80",
        Spell::TeleportOther => "M147",
        Spell::Beacon => "M83",
        Spell::TraceTeleport => "M84",
        Spell::DivertTeleport => "M84",
        Spell::PlanarSummons => "M82",
        Spell::PlanarVisit => "M82",
        Spell::PlaneShift => "M83",
        Spell::PlaneShiftOther => "M83",
        Spell::Phase => "M83",
        Spell::PhaseOther => "M83",
        Spell::SeekGate => "M85",
        Spell::ControlGate => "M85",
        Spell::CreateGate => "M85",
        Spell::ScryGate => "M85",
        Spell::HideObject => "M86",
        Spell::Sanctuary => "M86",
        _ => panic!("Invalid spell {:?} for Gate college", spell),
    }
}
