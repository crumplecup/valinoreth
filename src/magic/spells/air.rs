//! Air college spells.
//!
//! # GURPS Rules
//!
//! Air spells manipulate air, wind, sound, and odors. They control gases,
//! create blasts of wind, manipulate sound, and allow movement through air.
//!
//! # Citations
//!
//! M 25-35 - Air college

use crate::{Duration, EnergyCost, ResistanceType, Spell, SpellCollege, SpellPrerequisite, SpellType};
use tracing::{debug, instrument};

/// Returns base energy cost for Air spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Air spell energy cost");
    match spell {
        Spell::PurifyAirAir => EnergyCost::Fixed(1),
        Spell::CreateAir => EnergyCost::Fixed(2),
        Spell::DestroyAir => EnergyCost::Fixed(2),
        Spell::ShapeAir => EnergyCost::Fixed(2),
        Spell::NoSmell => EnergyCost::Fixed(1),
        Spell::Stench => EnergyCost::Fixed(2),
        Spell::WalkOnAir => EnergyCost::Fixed(3),
        Spell::Lightning => EnergyCost::PerDie(1),
        Spell::Windstorm => EnergyCost::Fixed(4),
        Spell::AirJet => EnergyCost::PerDie(1),
        Spell::BreatheWater => EnergyCost::Fixed(3),
        Spell::BodyOfAir => EnergyCost::Fixed(5),
        Spell::Concussion => EnergyCost::Fixed(2),
        Spell::Sound => EnergyCost::Fixed(1),
        Spell::Silence => EnergyCost::Fixed(2),
        Spell::Thunderclap => EnergyCost::Fixed(3),
        Spell::AirVortex => EnergyCost::Fixed(3),
        Spell::EssentialAir => EnergyCost::Fixed(3),
        _ => panic!("Invalid spell {:?} for Air college", spell),
    }
}

/// Returns casting time in seconds for Air spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Air spell casting time");
    match spell {
        Spell::PurifyAirAir => 1,
        Spell::CreateAir => 1,
        Spell::DestroyAir => 1,
        Spell::ShapeAir => 1,
        Spell::NoSmell => 1,
        Spell::Stench => 1,
        Spell::WalkOnAir => 2,
        Spell::Lightning => 1,
        Spell::Windstorm => 2,
        Spell::AirJet => 1,
        Spell::BreatheWater => 2,
        Spell::BodyOfAir => 3,
        Spell::Concussion => 1,
        Spell::Sound => 1,
        Spell::Silence => 1,
        Spell::Thunderclap => 1,
        Spell::AirVortex => 2,
        Spell::EssentialAir => 2,
        _ => panic!("Invalid spell {:?} for Air college", spell),
    }
}

/// Returns duration for Air spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Air spell duration");
    match spell {
        Spell::PurifyAirAir => Duration::Instant,
        Spell::CreateAir => Duration::Concentration,
        Spell::DestroyAir => Duration::Instant,
        Spell::ShapeAir => Duration::Concentration,
        Spell::NoSmell => Duration::Minutes(1),
        Spell::Stench => Duration::Minutes(1),
        Spell::WalkOnAir => Duration::Minutes(1),
        Spell::Lightning => Duration::Instant,
        Spell::Windstorm => Duration::Minutes(1),
        Spell::AirJet => Duration::Instant,
        Spell::BreatheWater => Duration::Minutes(10),
        Spell::BodyOfAir => Duration::Minutes(1),
        Spell::Concussion => Duration::Instant,
        Spell::Sound => Duration::Minutes(1),
        Spell::Silence => Duration::Minutes(1),
        Spell::Thunderclap => Duration::Instant,
        Spell::AirVortex => Duration::Minutes(1),
        Spell::EssentialAir => Duration::Hours(1),
        _ => panic!("Invalid spell {:?} for Air college", spell),
    }
}

/// Returns prerequisites for Air spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Air spell prerequisites");
    match spell {
        Spell::PurifyAirAir => vec![SpellPrerequisite::Magery(0)],
        Spell::CreateAir => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::PurifyAirAir),
        ],
        Spell::DestroyAir => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::CreateAir),
        ],
        Spell::ShapeAir => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::CreateAir),
        ],
        Spell::NoSmell => vec![SpellPrerequisite::Magery(0)],
        Spell::Stench => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::NoSmell),
        ],
        Spell::WalkOnAir => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Air, 4),
        ],
        Spell::Lightning => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Air, 4),
        ],
        Spell::Windstorm => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::ShapeAir),
        ],
        Spell::AirJet => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::ShapeAir),
        ],
        Spell::BreatheWater => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Air, 4),
        ],
        Spell::BodyOfAir => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Air, 6),
        ],
        Spell::Concussion => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Air, 4),
        ],
        Spell::Sound => vec![SpellPrerequisite::Magery(0)],
        Spell::Silence => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::Sound),
        ],
        Spell::Thunderclap => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Sound),
        ],
        Spell::AirVortex => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::ShapeAir),
        ],
        Spell::EssentialAir => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Air, 6),
        ],
        _ => panic!("Invalid spell {:?} for Air college", spell),
    }
}

/// Returns spell type for Air spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Air spell type");
    match spell {
        Spell::PurifyAirAir => SpellType::Area,
        Spell::CreateAir => SpellType::Regular,
        Spell::DestroyAir => SpellType::Area,
        Spell::ShapeAir => SpellType::Regular,
        Spell::NoSmell => SpellType::Area,
        Spell::Stench => SpellType::Area,
        Spell::WalkOnAir => SpellType::Regular,
        Spell::Lightning => SpellType::Missile,
        Spell::Windstorm => SpellType::Area,
        Spell::AirJet => SpellType::Missile,
        Spell::BreatheWater => SpellType::Regular,
        Spell::BodyOfAir => SpellType::Regular,
        Spell::Concussion => SpellType::Regular,
        Spell::Sound => SpellType::Regular,
        Spell::Silence => SpellType::Area,
        Spell::Thunderclap => SpellType::Regular,
        Spell::AirVortex => SpellType::Area,
        Spell::EssentialAir => SpellType::Regular,
        _ => panic!("Invalid spell {:?} for Air college", spell),
    }
}

/// Returns resistance type for Air spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Air spell resistance");
    match spell {
        Spell::PurifyAirAir => None,
        Spell::CreateAir => None,
        Spell::DestroyAir => None,
        Spell::ShapeAir => None,
        Spell::NoSmell => None,
        Spell::Stench => None,
        Spell::WalkOnAir => None,
        Spell::Lightning => None,
        Spell::Windstorm => None,
        Spell::AirJet => None,
        Spell::BreatheWater => None,
        Spell::BodyOfAir => None,
        Spell::Concussion => Some(ResistanceType::HT),
        Spell::Sound => None,
        Spell::Silence => None,
        Spell::Thunderclap => Some(ResistanceType::HT),
        Spell::AirVortex => None,
        Spell::EssentialAir => None,
        _ => panic!("Invalid spell {:?} for Air college", spell),
    }
}

/// Returns spell reference for Air spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Air spell reference");
    match spell {
        Spell::PurifyAirAir => "M33",
        Spell::CreateAir => "M28",
        Spell::DestroyAir => "M28",
        Spell::ShapeAir => "M34",
        Spell::NoSmell => "M32",
        Spell::Stench => "M34",
        Spell::WalkOnAir => "M35",
        Spell::Lightning => "M30",
        Spell::Windstorm => "M35",
        Spell::AirJet => "M25",
        Spell::BreatheWater => "M27",
        Spell::BodyOfAir => "M26",
        Spell::Concussion => "M28",
        Spell::Sound => "M34",
        Spell::Silence => "M34",
        Spell::Thunderclap => "M35",
        Spell::AirVortex => "M26",
        Spell::EssentialAir => "M29",
        _ => panic!("Invalid spell {:?} for Air college", spell),
    }
}
