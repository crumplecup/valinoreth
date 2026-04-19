//! Illusion & Creation college spells.
//!
//! # GURPS Rules
//!
//! Illusion & Creation spells create false images and sensory impressions,
//! or manifest temporary (or permanent) matter. These spells can deceive the
//! senses, alter appearance, create objects, or manipulate light and shadow.
//!
//! # Citations
//!
//! M 88-105 - Illusion & Creation college

use crate::{Duration, EnergyCost, ResistanceType, Spell, SpellCollege, SpellPrerequisite, SpellType};
use tracing::{debug, instrument};

/// Returns base energy cost for Illusion & Creation spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Illusion & Creation spell energy cost");
    match spell {
        Spell::SimpleIllusion => EnergyCost::Fixed(1),
        Spell::PerfectIllusion => EnergyCost::Fixed(3),
        Spell::ComplexIllusion => EnergyCost::Fixed(2),
        Spell::MakeRealIllusion => EnergyCost::Fixed(10),
        Spell::Create => EnergyCost::Fixed(4),
        Spell::PermanentCreation => EnergyCost::Fixed(8),
        Spell::Invisibility => EnergyCost::Fixed(3),
        Spell::InvisibilityToAll => EnergyCost::Fixed(5),
        Spell::Disguise => EnergyCost::Fixed(2),
        Spell::ShapeShifting => EnergyCost::Fixed(6),
        Spell::Duplicate => EnergyCost::Fixed(4),
        Spell::Phantasm => EnergyCost::Fixed(3),
        Spell::SilentImage => EnergyCost::Fixed(1),
        Spell::Light => EnergyCost::Fixed(1),
        Spell::Darkness => EnergyCost::Fixed(2),
        Spell::HideAura => EnergyCost::Fixed(2),
        Spell::Mirror => EnergyCost::Fixed(4),
        _ => panic!("Invalid spell {:?} for Illusion & Creation college", spell),
    }
}

/// Returns casting time in seconds for Illusion & Creation spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Illusion & Creation spell casting time");
    match spell {
        Spell::SimpleIllusion => 1,
        Spell::PerfectIllusion => 2,
        Spell::ComplexIllusion => 2,
        Spell::MakeRealIllusion => 5,
        Spell::Create => 2,
        Spell::PermanentCreation => 5,
        Spell::Invisibility => 2,
        Spell::InvisibilityToAll => 3,
        Spell::Disguise => 2,
        Spell::ShapeShifting => 3,
        Spell::Duplicate => 3,
        Spell::Phantasm => 2,
        Spell::SilentImage => 1,
        Spell::Light => 1,
        Spell::Darkness => 1,
        Spell::HideAura => 1,
        Spell::Mirror => 2,
        _ => panic!("Invalid spell {:?} for Illusion & Creation college", spell),
    }
}

/// Returns duration for Illusion & Creation spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Illusion & Creation spell duration");
    match spell {
        Spell::SimpleIllusion => Duration::Concentration,
        Spell::PerfectIllusion => Duration::Concentration,
        Spell::ComplexIllusion => Duration::Concentration,
        Spell::MakeRealIllusion => Duration::Permanent,
        Spell::Create => Duration::Minutes(10),
        Spell::PermanentCreation => Duration::Permanent,
        Spell::Invisibility => Duration::Minutes(1),
        Spell::InvisibilityToAll => Duration::Minutes(1),
        Spell::Disguise => Duration::Minutes(10),
        Spell::ShapeShifting => Duration::Minutes(1),
        Spell::Duplicate => Duration::Minutes(1),
        Spell::Phantasm => Duration::Concentration,
        Spell::SilentImage => Duration::Concentration,
        Spell::Light => Duration::Minutes(1),
        Spell::Darkness => Duration::Minutes(1),
        Spell::HideAura => Duration::Hours(1),
        Spell::Mirror => Duration::Minutes(1),
        _ => panic!("Invalid spell {:?} for Illusion & Creation college", spell),
    }
}

/// Returns prerequisites for Illusion & Creation spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Illusion & Creation spell prerequisites");
    match spell {
        Spell::SimpleIllusion => vec![SpellPrerequisite::Magery(0)],
        Spell::PerfectIllusion => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::SimpleIllusion),
        ],
        Spell::ComplexIllusion => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::PerfectIllusion),
        ],
        Spell::MakeRealIllusion => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::ComplexIllusion),
        ],
        Spell::Create => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::IllusionCreation, 4),
        ],
        Spell::PermanentCreation => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Create),
        ],
        Spell::Invisibility => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::IllusionCreation, 4),
        ],
        Spell::InvisibilityToAll => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Invisibility),
        ],
        Spell::Disguise => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::SimpleIllusion),
        ],
        Spell::ShapeShifting => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::IllusionCreation, 6),
        ],
        Spell::Duplicate => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::IllusionCreation, 6),
        ],
        Spell::Phantasm => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::ComplexIllusion),
        ],
        Spell::SilentImage => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::SimpleIllusion),
        ],
        Spell::Light => vec![SpellPrerequisite::Magery(0)],
        Spell::Darkness => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::Light),
        ],
        Spell::HideAura => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::IllusionCreation, 4),
        ],
        Spell::Mirror => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::IllusionCreation, 8),
        ],
        _ => panic!("Invalid spell {:?} for Illusion & Creation college", spell),
    }
}

/// Returns spell type for Illusion & Creation spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Illusion & Creation spell type");
    match spell {
        Spell::SimpleIllusion => SpellType::Regular,
        Spell::PerfectIllusion => SpellType::Regular,
        Spell::ComplexIllusion => SpellType::Regular,
        Spell::MakeRealIllusion => SpellType::Regular,
        Spell::Create => SpellType::Regular,
        Spell::PermanentCreation => SpellType::Regular,
        Spell::Invisibility => SpellType::Regular,
        Spell::InvisibilityToAll => SpellType::Regular,
        Spell::Disguise => SpellType::Regular,
        Spell::ShapeShifting => SpellType::Regular,
        Spell::Duplicate => SpellType::Regular,
        Spell::Phantasm => SpellType::Area,
        Spell::SilentImage => SpellType::Regular,
        Spell::Light => SpellType::Regular,
        Spell::Darkness => SpellType::Area,
        Spell::HideAura => SpellType::Regular,
        Spell::Mirror => SpellType::Blocking,
        _ => panic!("Invalid spell {:?} for Illusion & Creation college", spell),
    }
}

/// Returns resistance type for Illusion & Creation spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Illusion & Creation spell resistance");
    match spell {
        Spell::SimpleIllusion => Some(ResistanceType::IQ),
        Spell::PerfectIllusion => Some(ResistanceType::IQ),
        Spell::ComplexIllusion => Some(ResistanceType::IQ),
        Spell::MakeRealIllusion => None,
        Spell::Create => None,
        Spell::PermanentCreation => None,
        Spell::Invisibility => None,
        Spell::InvisibilityToAll => None,
        Spell::Disguise => Some(ResistanceType::IQ),
        Spell::ShapeShifting => None,
        Spell::Duplicate => Some(ResistanceType::IQ),
        Spell::Phantasm => Some(ResistanceType::IQ),
        Spell::SilentImage => Some(ResistanceType::IQ),
        Spell::Light => None,
        Spell::Darkness => None,
        Spell::HideAura => None,
        Spell::Mirror => None,
        _ => panic!("Invalid spell {:?} for Illusion & Creation college", spell),
    }
}

/// Returns spell reference for Illusion & Creation spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Illusion & Creation spell reference");
    match spell {
        Spell::SimpleIllusion => "M100",
        Spell::PerfectIllusion => "M97",
        Spell::ComplexIllusion => "M96",
        Spell::MakeRealIllusion => "M102",
        Spell::Create => "M100",
        Spell::PermanentCreation => "M99",
        Spell::Invisibility => "M98",
        Spell::InvisibilityToAll => "M98",
        Spell::Disguise => "M97",
        Spell::ShapeShifting => "M103",
        Spell::Duplicate => "M98",
        Spell::Phantasm => "M100",
        Spell::SilentImage => "M100",
        Spell::Light => "M99",
        Spell::Darkness => "M97",
        Spell::HideAura => "M98",
        Spell::Mirror => "M101",
        _ => panic!("Invalid spell {:?} for Illusion & Creation college", spell),
    }
}
