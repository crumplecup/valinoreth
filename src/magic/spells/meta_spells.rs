//! Meta-Spells college spells.
//!
//! # GURPS Rules
//!
//! Meta-Spells manipulate other spells and magical energy, allowing
//! counterspelling, dispelling, energy manipulation, and spell modification.
//!
//! # Citations
//!
//! M 126-133 - Meta-Spells college

use crate::{Duration, EnergyCost, ResistanceType, Spell, SpellPrerequisite, SpellType};
use tracing::{debug, instrument};

/// Returns base energy cost for Meta-Spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Meta-Spells energy cost");
    match spell {
        Spell::Counterspell => EnergyCost::Fixed(3),
        Spell::DispelMagic => EnergyCost::Fixed(3),
        Spell::SuspendSpell => EnergyCost::Fixed(2),
        Spell::MaintainSpell => EnergyCost::Fixed(1),
        Spell::ExtendSpell => EnergyCost::Fixed(2),
        Spell::HasteSpell => EnergyCost::Fixed(3),
        Spell::Link => EnergyCost::Fixed(4),
        Spell::Permanency => EnergyCost::Fixed(10),
        Spell::StealEnergy => EnergyCost::Fixed(2),
        Spell::DrawPower => EnergyCost::Fixed(1),
        Spell::SpellStone => EnergyCost::Fixed(5),
        Spell::Reflect => EnergyCost::Fixed(4),
        Spell::Ward => EnergyCost::Fixed(3),
        Spell::Augment => EnergyCost::Fixed(2),
        Spell::Diminish => EnergyCost::Fixed(2),
        Spell::Delay => EnergyCost::Fixed(2),
        _ => panic!("Invalid spell {:?} for Meta-Spells college", spell),
    }
}

/// Returns casting time in seconds for Meta-Spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Meta-Spells casting time");
    match spell {
        Spell::Counterspell => 1,
        Spell::DispelMagic => 2,
        Spell::SuspendSpell => 2,
        Spell::MaintainSpell => 1,
        Spell::ExtendSpell => 2,
        Spell::HasteSpell => 2,
        Spell::Link => 3,
        Spell::Permanency => 10,
        Spell::StealEnergy => 2,
        Spell::DrawPower => 1,
        Spell::SpellStone => 5,
        Spell::Reflect => 2,
        Spell::Ward => 3,
        Spell::Augment => 1,
        Spell::Diminish => 1,
        Spell::Delay => 2,
        _ => panic!("Invalid spell {:?} for Meta-Spells college", spell),
    }
}

/// Returns duration for Meta-Spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Meta-Spells duration");
    match spell {
        Spell::Counterspell => Duration::Instant,
        Spell::DispelMagic => Duration::Instant,
        Spell::SuspendSpell => Duration::Minutes(10),
        Spell::MaintainSpell => Duration::Concentration,
        Spell::ExtendSpell => Duration::Instant,
        Spell::HasteSpell => Duration::Instant,
        Spell::Link => Duration::Permanent,
        Spell::Permanency => Duration::Permanent,
        Spell::StealEnergy => Duration::Instant,
        Spell::DrawPower => Duration::Instant,
        Spell::SpellStone => Duration::Permanent,
        Spell::Reflect => Duration::Minutes(1),
        Spell::Ward => Duration::Hours(1),
        Spell::Augment => Duration::Instant,
        Spell::Diminish => Duration::Instant,
        Spell::Delay => Duration::Instant,
        _ => panic!("Invalid spell {:?} for Meta-Spells college", spell),
    }
}

/// Returns prerequisites for Meta-Spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Meta-Spells prerequisites");
    match spell {
        Spell::Counterspell => vec![SpellPrerequisite::Magery(1)],
        Spell::DispelMagic => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Counterspell),
        ],
        Spell::SuspendSpell => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::DispelMagic),
        ],
        Spell::MaintainSpell => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Counterspell),
        ],
        Spell::ExtendSpell => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::MaintainSpell),
        ],
        Spell::HasteSpell => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Counterspell),
        ],
        Spell::Link => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::MaintainSpell),
        ],
        Spell::Permanency => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::ExtendSpell),
        ],
        Spell::StealEnergy => vec![SpellPrerequisite::Magery(2)],
        Spell::DrawPower => vec![SpellPrerequisite::Magery(1)],
        Spell::SpellStone => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::MaintainSpell),
        ],
        Spell::Reflect => vec![SpellPrerequisite::Magery(3)],
        Spell::Ward => vec![SpellPrerequisite::Magery(2)],
        Spell::Augment => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Counterspell),
        ],
        Spell::Diminish => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Counterspell),
        ],
        Spell::Delay => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::MaintainSpell),
        ],
        _ => panic!("Invalid spell {:?} for Meta-Spells college", spell),
    }
}

/// Returns spell type for Meta-Spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Meta-Spells type");
    match spell {
        Spell::Counterspell => SpellType::Regular,
        Spell::DispelMagic => SpellType::Regular,
        Spell::SuspendSpell => SpellType::Regular,
        Spell::MaintainSpell => SpellType::Regular,
        Spell::ExtendSpell => SpellType::Regular,
        Spell::HasteSpell => SpellType::Regular,
        Spell::Link => SpellType::Regular,
        Spell::Permanency => SpellType::Regular,
        Spell::StealEnergy => SpellType::Regular,
        Spell::DrawPower => SpellType::Regular,
        Spell::SpellStone => SpellType::Regular,
        Spell::Reflect => SpellType::Regular,
        Spell::Ward => SpellType::Area,
        Spell::Augment => SpellType::Regular,
        Spell::Diminish => SpellType::Regular,
        Spell::Delay => SpellType::Regular,
        _ => panic!("Invalid spell {:?} for Meta-Spells college", spell),
    }
}

/// Returns resistance type for Meta-Spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Meta-Spells resistance");
    match spell {
        Spell::Counterspell => None,
        Spell::DispelMagic => None,
        Spell::SuspendSpell => None,
        Spell::MaintainSpell => None,
        Spell::ExtendSpell => None,
        Spell::HasteSpell => None,
        Spell::Link => None,
        Spell::Permanency => None,
        Spell::StealEnergy => Some(ResistanceType::Will),
        Spell::DrawPower => None,
        Spell::SpellStone => None,
        Spell::Reflect => None,
        Spell::Ward => None,
        Spell::Augment => None,
        Spell::Diminish => None,
        Spell::Delay => None,
        _ => panic!("Invalid spell {:?} for Meta-Spells college", spell),
    }
}

/// Returns page reference for Meta-Spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Meta-Spells reference");
    match spell {
        Spell::Counterspell => "M126",
        Spell::DispelMagic => "M127",
        Spell::SuspendSpell => "M132",
        Spell::MaintainSpell => "M130",
        Spell::ExtendSpell => "M128",
        Spell::HasteSpell => "M129",
        Spell::Link => "M130",
        Spell::Permanency => "M131",
        Spell::StealEnergy => "M132",
        Spell::DrawPower => "M127",
        Spell::SpellStone => "M132",
        Spell::Reflect => "M131",
        Spell::Ward => "M133",
        Spell::Augment => "M126",
        Spell::Diminish => "M127",
        Spell::Delay => "M128",
        _ => panic!("Invalid spell {:?} for Meta-Spells college", spell),
    }
}
