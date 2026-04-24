//! Light & Darkness college spells.
//!
//! # GURPS Rules
//!
//! Light & Darkness spells manipulate illumination, shadows, and vision.
//! They include spells for creating light and darkness, enhancing sight,
//! and transforming into shadow forms.
//!
//! # Citations
//!
//! M 110-116 - Light & Darkness college

use crate::{Duration, EnergyCost, ResistanceType, Spell, SpellPrerequisite, SpellType};
use tracing::{debug, instrument};

/// Returns base energy cost for Light & Darkness spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Light & Darkness spell energy cost");
    match spell {
        Spell::Light => EnergyCost::Fixed(1),
        Spell::Darkness => EnergyCost::Fixed(1),
        Spell::ContinualLight => EnergyCost::Fixed(2),
        Spell::Colors => EnergyCost::Fixed(1),
        Spell::RemoveShadow => EnergyCost::Fixed(2),
        Spell::NightVision => EnergyCost::Fixed(2),
        Spell::DarkVision => EnergyCost::Fixed(3),
        Spell::Blur => EnergyCost::Fixed(2),
        Spell::SeeInvisible => EnergyCost::Fixed(2),
        Spell::Mirror => EnergyCost::Fixed(2),
        Spell::LightJet => EnergyCost::Fixed(1),
        Spell::Blackout => EnergyCost::Fixed(3),
        Spell::WallOfLight => EnergyCost::Fixed(2),
        Spell::WallOfDarkness => EnergyCost::Fixed(2),
        Spell::Flash => EnergyCost::Fixed(2),
        Spell::BodyOfShadow => EnergyCost::Fixed(6),
        Spell::ShadowForm => EnergyCost::Fixed(4),
        Spell::Infravision => EnergyCost::Fixed(3),
        _ => panic!("Invalid spell {:?} for Light & Darkness college", spell),
    }
}

/// Returns casting time in seconds for Light & Darkness spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Light & Darkness spell casting time");
    match spell {
        Spell::Light => 1,
        Spell::Darkness => 1,
        Spell::ContinualLight => 3,
        Spell::Colors => 1,
        Spell::RemoveShadow => 1,
        Spell::NightVision => 2,
        Spell::DarkVision => 2,
        Spell::Blur => 1,
        Spell::SeeInvisible => 2,
        Spell::Mirror => 2,
        Spell::LightJet => 1,
        Spell::Blackout => 2,
        Spell::WallOfLight => 2,
        Spell::WallOfDarkness => 2,
        Spell::Flash => 1,
        Spell::BodyOfShadow => 4,
        Spell::ShadowForm => 3,
        Spell::Infravision => 2,
        _ => panic!("Invalid spell {:?} for Light & Darkness college", spell),
    }
}

/// Returns duration for Light & Darkness spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Light & Darkness spell duration");
    match spell {
        Spell::Light => Duration::Minutes(1),
        Spell::Darkness => Duration::Minutes(1),
        Spell::ContinualLight => Duration::Permanent,
        Spell::Colors => Duration::Permanent,
        Spell::RemoveShadow => Duration::Minutes(1),
        Spell::NightVision => Duration::Minutes(10),
        Spell::DarkVision => Duration::Minutes(10),
        Spell::Blur => Duration::Minutes(1),
        Spell::SeeInvisible => Duration::Minutes(1),
        Spell::Mirror => Duration::Permanent,
        Spell::LightJet => Duration::Instant,
        Spell::Blackout => Duration::Minutes(1),
        Spell::WallOfLight => Duration::Minutes(1),
        Spell::WallOfDarkness => Duration::Minutes(1),
        Spell::Flash => Duration::Instant,
        Spell::BodyOfShadow => Duration::Hours(1),
        Spell::ShadowForm => Duration::Minutes(10),
        Spell::Infravision => Duration::Minutes(10),
        _ => panic!("Invalid spell {:?} for Light & Darkness college", spell),
    }
}

/// Returns prerequisites for Light & Darkness spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Light & Darkness spell prerequisites");
    match spell {
        Spell::Light => vec![SpellPrerequisite::Magery(0)],
        Spell::Darkness => vec![SpellPrerequisite::Magery(0)],
        Spell::ContinualLight => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Light),
        ],
        Spell::Colors => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::Light),
        ],
        Spell::RemoveShadow => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Light),
        ],
        Spell::NightVision => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::Light),
        ],
        Spell::DarkVision => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Darkness),
        ],
        Spell::Blur => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Darkness),
        ],
        Spell::SeeInvisible => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Light),
        ],
        Spell::Mirror => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Light),
        ],
        Spell::LightJet => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Light),
        ],
        Spell::Blackout => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Darkness),
        ],
        Spell::WallOfLight => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::ContinualLight),
        ],
        Spell::WallOfDarkness => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Darkness),
        ],
        Spell::Flash => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Light),
        ],
        Spell::BodyOfShadow => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::ShadowForm),
        ],
        Spell::ShadowForm => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Darkness),
        ],
        Spell::Infravision => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::NightVision),
        ],
        _ => panic!("Invalid spell {:?} for Light & Darkness college", spell),
    }
}

/// Returns spell type for Light & Darkness spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Light & Darkness spell type");
    match spell {
        Spell::Light => SpellType::Area,
        Spell::Darkness => SpellType::Area,
        Spell::ContinualLight => SpellType::Regular,
        Spell::Colors => SpellType::Regular,
        Spell::RemoveShadow => SpellType::Area,
        Spell::NightVision => SpellType::Regular,
        Spell::DarkVision => SpellType::Regular,
        Spell::Blur => SpellType::Regular,
        Spell::SeeInvisible => SpellType::Regular,
        Spell::Mirror => SpellType::Regular,
        Spell::LightJet => SpellType::Regular,
        Spell::Blackout => SpellType::Area,
        Spell::WallOfLight => SpellType::Area,
        Spell::WallOfDarkness => SpellType::Area,
        Spell::Flash => SpellType::Regular,
        Spell::BodyOfShadow => SpellType::Regular,
        Spell::ShadowForm => SpellType::Regular,
        Spell::Infravision => SpellType::Regular,
        _ => panic!("Invalid spell {:?} for Light & Darkness college", spell),
    }
}

/// Returns resistance type for Light & Darkness spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Light & Darkness spell resistance");
    match spell {
        Spell::Light => None,
        Spell::Darkness => None,
        Spell::ContinualLight => None,
        Spell::Colors => None,
        Spell::RemoveShadow => None,
        Spell::NightVision => None,
        Spell::DarkVision => None,
        Spell::Blur => None,
        Spell::SeeInvisible => None,
        Spell::Mirror => None,
        Spell::LightJet => Some(ResistanceType::HT),
        Spell::Blackout => None,
        Spell::WallOfLight => None,
        Spell::WallOfDarkness => None,
        Spell::Flash => Some(ResistanceType::HT),
        Spell::BodyOfShadow => None,
        Spell::ShadowForm => None,
        Spell::Infravision => None,
        _ => panic!("Invalid spell {:?} for Light & Darkness college", spell),
    }
}

/// Returns page reference for Light & Darkness spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Light & Darkness spell reference");
    match spell {
        Spell::Light => "M110",
        Spell::Darkness => "M110",
        Spell::ContinualLight => "M110",
        Spell::Colors => "M110",
        Spell::RemoveShadow => "M110",
        Spell::NightVision => "M111",
        Spell::DarkVision => "M111",
        Spell::Blur => "M113",
        Spell::SeeInvisible => "M113",
        Spell::Mirror => "M112",
        Spell::LightJet => "M112",
        Spell::Blackout => "M112",
        Spell::WallOfLight => "M113",
        Spell::WallOfDarkness => "M113",
        Spell::Flash => "M112",
        Spell::BodyOfShadow => "M114",
        Spell::ShadowForm => "M114",
        Spell::Infravision => "M111",
        _ => panic!("Invalid spell {:?} for Light & Darkness college", spell),
    }
}
