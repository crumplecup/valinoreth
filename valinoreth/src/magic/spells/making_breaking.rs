//! Making & Breaking college spells.
//!
//! # GURPS Rules
//!
//! Making & Breaking spells manipulate physical objects, allowing creation,
//! destruction, repair, transformation, and enhancement of materials and items.
//!
//! # Citations
//!
//! M 116-125 - Making & Breaking college

use crate::{Duration, EnergyCost, ResistanceType, Spell, SpellPrerequisite, SpellType};
use tracing::{debug, instrument};

/// Returns base energy cost for Making & Breaking spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Making & Breaking spell energy cost");
    match spell {
        Spell::Repair => EnergyCost::Fixed(1),
        Spell::Shatter => EnergyCost::Fixed(2),
        Spell::Stiffen => EnergyCost::Fixed(1),
        Spell::Soften => EnergyCost::Fixed(1),
        Spell::Restore => EnergyCost::Fixed(2),
        Spell::Rust => EnergyCost::Fixed(2),
        Spell::TransformObject => EnergyCost::Fixed(5),
        Spell::Copy => EnergyCost::Fixed(2),
        Spell::Dye => EnergyCost::Fixed(1),
        Spell::Knot => EnergyCost::Fixed(1),
        Spell::Fasten => EnergyCost::Fixed(2),
        Spell::ExtendObject => EnergyCost::Fixed(3),
        Spell::ShrinkObject => EnergyCost::Fixed(3),
        Spell::Polish => EnergyCost::Fixed(1),
        Spell::Explode => EnergyCost::Fixed(3),
        Spell::FindWeakness => EnergyCost::Fixed(2),
        Spell::CreateObject => EnergyCost::Fixed(4),
        Spell::DestroyObject => EnergyCost::Fixed(3),
        Spell::Rebuild => EnergyCost::Fixed(5),
        Spell::Inscribe => EnergyCost::Fixed(2),
        Spell::AnimateObject => EnergyCost::Fixed(4),
        _ => panic!("Invalid spell {:?} for Making & Breaking college", spell),
    }
}

/// Returns casting time in seconds for Making & Breaking spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Making & Breaking spell casting time");
    match spell {
        Spell::Repair => 1,
        Spell::Shatter => 1,
        Spell::Stiffen => 1,
        Spell::Soften => 1,
        Spell::Restore => 2,
        Spell::Rust => 2,
        Spell::TransformObject => 5,
        Spell::Copy => 2,
        Spell::Dye => 1,
        Spell::Knot => 1,
        Spell::Fasten => 2,
        Spell::ExtendObject => 3,
        Spell::ShrinkObject => 3,
        Spell::Polish => 1,
        Spell::Explode => 2,
        Spell::FindWeakness => 2,
        Spell::CreateObject => 4,
        Spell::DestroyObject => 2,
        Spell::Rebuild => 5,
        Spell::Inscribe => 2,
        Spell::AnimateObject => 3,
        _ => panic!("Invalid spell {:?} for Making & Breaking college", spell),
    }
}

/// Returns duration for Making & Breaking spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Making & Breaking spell duration");
    match spell {
        Spell::Repair => Duration::Permanent,
        Spell::Shatter => Duration::Instant,
        Spell::Stiffen => Duration::Minutes(1),
        Spell::Soften => Duration::Minutes(1),
        Spell::Restore => Duration::Permanent,
        Spell::Rust => Duration::Permanent,
        Spell::TransformObject => Duration::Permanent,
        Spell::Copy => Duration::Permanent,
        Spell::Dye => Duration::Permanent,
        Spell::Knot => Duration::Minutes(10),
        Spell::Fasten => Duration::Minutes(10),
        Spell::ExtendObject => Duration::Minutes(10),
        Spell::ShrinkObject => Duration::Minutes(10),
        Spell::Polish => Duration::Permanent,
        Spell::Explode => Duration::Instant,
        Spell::FindWeakness => Duration::Instant,
        Spell::CreateObject => Duration::Minutes(1),
        Spell::DestroyObject => Duration::Instant,
        Spell::Rebuild => Duration::Permanent,
        Spell::Inscribe => Duration::Permanent,
        Spell::AnimateObject => Duration::Concentration,
        _ => panic!("Invalid spell {:?} for Making & Breaking college", spell),
    }
}

/// Returns prerequisites for Making & Breaking spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Making & Breaking spell prerequisites");
    match spell {
        Spell::Repair => vec![SpellPrerequisite::Magery(0)],
        Spell::Shatter => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Repair),
        ],
        Spell::Stiffen => vec![SpellPrerequisite::Magery(0)],
        Spell::Soften => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::Stiffen),
        ],
        Spell::Restore => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Repair),
        ],
        Spell::Rust => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Restore),
        ],
        Spell::TransformObject => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::CreateObject),
        ],
        Spell::Copy => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Repair),
        ],
        Spell::Dye => vec![SpellPrerequisite::Magery(0)],
        Spell::Knot => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::Stiffen),
        ],
        Spell::Fasten => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Knot),
        ],
        Spell::ExtendObject => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::TransformObject),
        ],
        Spell::ShrinkObject => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::ExtendObject),
        ],
        Spell::Polish => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::Dye),
        ],
        Spell::Explode => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Shatter),
        ],
        Spell::FindWeakness => vec![SpellPrerequisite::Magery(1)],
        Spell::CreateObject => vec![SpellPrerequisite::Magery(2)],
        Spell::DestroyObject => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Shatter),
        ],
        Spell::Rebuild => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::Repair),
            SpellPrerequisite::Spell(Spell::CreateObject),
        ],
        Spell::Inscribe => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Dye),
        ],
        Spell::AnimateObject => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::CreateObject),
        ],
        _ => panic!("Invalid spell {:?} for Making & Breaking college", spell),
    }
}

/// Returns spell type for Making & Breaking spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Making & Breaking spell type");
    match spell {
        Spell::Repair => SpellType::Regular,
        Spell::Shatter => SpellType::Regular,
        Spell::Stiffen => SpellType::Regular,
        Spell::Soften => SpellType::Regular,
        Spell::Restore => SpellType::Regular,
        Spell::Rust => SpellType::Regular,
        Spell::TransformObject => SpellType::Regular,
        Spell::Copy => SpellType::Regular,
        Spell::Dye => SpellType::Regular,
        Spell::Knot => SpellType::Regular,
        Spell::Fasten => SpellType::Regular,
        Spell::ExtendObject => SpellType::Regular,
        Spell::ShrinkObject => SpellType::Regular,
        Spell::Polish => SpellType::Regular,
        Spell::Explode => SpellType::Regular,
        Spell::FindWeakness => SpellType::Information,
        Spell::CreateObject => SpellType::Regular,
        Spell::DestroyObject => SpellType::Regular,
        Spell::Rebuild => SpellType::Regular,
        Spell::Inscribe => SpellType::Regular,
        Spell::AnimateObject => SpellType::Regular,
        _ => panic!("Invalid spell {:?} for Making & Breaking college", spell),
    }
}

/// Returns resistance type for Making & Breaking spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Making & Breaking spell resistance");
    match spell {
        Spell::Repair => None,
        Spell::Shatter => None,
        Spell::Stiffen => None,
        Spell::Soften => None,
        Spell::Restore => None,
        Spell::Rust => None,
        Spell::TransformObject => None,
        Spell::Copy => None,
        Spell::Dye => None,
        Spell::Knot => None,
        Spell::Fasten => None,
        Spell::ExtendObject => None,
        Spell::ShrinkObject => None,
        Spell::Polish => None,
        Spell::Explode => None,
        Spell::FindWeakness => None,
        Spell::CreateObject => None,
        Spell::DestroyObject => None,
        Spell::Rebuild => None,
        Spell::Inscribe => None,
        Spell::AnimateObject => None,
        _ => panic!("Invalid spell {:?} for Making & Breaking college", spell),
    }
}

/// Returns page reference for Making & Breaking spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Making & Breaking spell reference");
    match spell {
        Spell::Repair => "M118",
        Spell::Shatter => "M122",
        Spell::Stiffen => "M123",
        Spell::Soften => "M123",
        Spell::Restore => "M121",
        Spell::Rust => "M121",
        Spell::TransformObject => "M124",
        Spell::Copy => "M117",
        Spell::Dye => "M117",
        Spell::Knot => "M119",
        Spell::Fasten => "M118",
        Spell::ExtendObject => "M118",
        Spell::ShrinkObject => "M122",
        Spell::Polish => "M120",
        Spell::Explode => "M118",
        Spell::FindWeakness => "M119",
        Spell::CreateObject => "M116",
        Spell::DestroyObject => "M117",
        Spell::Rebuild => "M120",
        Spell::Inscribe => "M119",
        Spell::AnimateObject => "M116",
        _ => panic!("Invalid spell {:?} for Making & Breaking college", spell),
    }
}
