//! Earth college spells.
//!
//! # GURPS Rules
//!
//! Earth spells manipulate earth, stone, and minerals. They control soil and rock,
//! transform materials, create earthquakes, and allow movement through solid matter.
//!
//! # Citations
//!
//! M 36-45 - Earth college

use crate::{
    Duration, EnergyCost, ResistanceType, Spell, SpellCollege, SpellPrerequisite, SpellType,
};
use tracing::{debug, instrument};

/// Returns base energy cost for Earth spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Earth spell energy cost");
    match spell {
        Spell::ShapeEarth => EnergyCost::Fixed(2),
        Spell::EarthToStone => EnergyCost::Fixed(3),
        Spell::StoneToEarth => EnergyCost::Fixed(3),
        Spell::CreateEarth => EnergyCost::Fixed(3),
        Spell::DestroyEarth => EnergyCost::Fixed(3),
        Spell::EarthVision => EnergyCost::Fixed(2),
        Spell::WalkThroughEarth => EnergyCost::Fixed(4),
        Spell::Earthquake => EnergyCost::Fixed(6),
        Spell::ShapeStone => EnergyCost::Fixed(2),
        Spell::EarthToAir => EnergyCost::Fixed(5),
        Spell::StoneMissile => EnergyCost::PerDie(1),
        Spell::CreateStone => EnergyCost::Fixed(4),
        Spell::SandJet => EnergyCost::PerDie(1),
        Spell::FleshToStone => EnergyCost::Fixed(10),
        Spell::StoneToFlesh => EnergyCost::Fixed(10),
        Spell::IronArm => EnergyCost::Fixed(3),
        Spell::Entombment => EnergyCost::Fixed(5),
        Spell::SummonEarthElemental => EnergyCost::Fixed(10),
        Spell::PurifyEarth => EnergyCost::Fixed(1),
        Spell::EssentialEarth => EnergyCost::Fixed(3),
        Spell::BodyOfEarth => EnergyCost::Fixed(5),
        Spell::EarthToWater => EnergyCost::Fixed(4),
        Spell::IdentifyMetal => EnergyCost::Fixed(1),
        Spell::ShapeMetal => EnergyCost::Fixed(3),
        _ => panic!("Invalid spell {:?} for Earth college", spell),
    }
}

/// Returns casting time in seconds for Earth spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Earth spell casting time");
    match spell {
        Spell::ShapeEarth => 1,
        Spell::EarthToStone => 2,
        Spell::StoneToEarth => 2,
        Spell::CreateEarth => 1,
        Spell::DestroyEarth => 1,
        Spell::EarthVision => 1,
        Spell::WalkThroughEarth => 2,
        Spell::Earthquake => 3,
        Spell::ShapeStone => 2,
        Spell::EarthToAir => 3,
        Spell::StoneMissile => 1,
        Spell::CreateStone => 2,
        Spell::SandJet => 1,
        Spell::FleshToStone => 5,
        Spell::StoneToFlesh => 5,
        Spell::IronArm => 2,
        Spell::Entombment => 2,
        Spell::SummonEarthElemental => 5,
        Spell::PurifyEarth => 1,
        Spell::EssentialEarth => 2,
        Spell::BodyOfEarth => 3,
        Spell::EarthToWater => 2,
        Spell::IdentifyMetal => 1,
        Spell::ShapeMetal => 2,
        _ => panic!("Invalid spell {:?} for Earth college", spell),
    }
}

/// Returns duration for Earth spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Earth spell duration");
    match spell {
        Spell::ShapeEarth => Duration::Concentration,
        Spell::EarthToStone => Duration::Permanent,
        Spell::StoneToEarth => Duration::Permanent,
        Spell::CreateEarth => Duration::Permanent,
        Spell::DestroyEarth => Duration::Instant,
        Spell::EarthVision => Duration::Concentration,
        Spell::WalkThroughEarth => Duration::Minutes(1),
        Spell::Earthquake => Duration::Instant,
        Spell::ShapeStone => Duration::Permanent,
        Spell::EarthToAir => Duration::Permanent,
        Spell::StoneMissile => Duration::Instant,
        Spell::CreateStone => Duration::Permanent,
        Spell::SandJet => Duration::Instant,
        Spell::FleshToStone => Duration::Permanent,
        Spell::StoneToFlesh => Duration::Permanent,
        Spell::IronArm => Duration::Minutes(1),
        Spell::Entombment => Duration::Minutes(1),
        Spell::SummonEarthElemental => Duration::Minutes(1),
        Spell::PurifyEarth => Duration::Instant,
        Spell::EssentialEarth => Duration::Hours(1),
        Spell::BodyOfEarth => Duration::Minutes(1),
        Spell::EarthToWater => Duration::Permanent,
        Spell::IdentifyMetal => Duration::Instant,
        Spell::ShapeMetal => Duration::Permanent,
        _ => panic!("Invalid spell {:?} for Earth college", spell),
    }
}

/// Returns prerequisites for Earth spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Earth spell prerequisites");
    match spell {
        Spell::ShapeEarth => vec![SpellPrerequisite::Magery(0)],
        Spell::EarthToStone => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::ShapeEarth),
        ],
        Spell::StoneToEarth => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::EarthToStone),
        ],
        Spell::CreateEarth => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::ShapeEarth),
        ],
        Spell::DestroyEarth => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::CreateEarth),
        ],
        Spell::EarthVision => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::ShapeEarth),
        ],
        Spell::WalkThroughEarth => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Earth, 4),
        ],
        Spell::Earthquake => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Earth, 8),
        ],
        Spell::ShapeStone => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::EarthToStone),
        ],
        Spell::EarthToAir => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Earth, 6),
        ],
        Spell::StoneMissile => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::ShapeStone),
        ],
        Spell::CreateStone => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::EarthToStone),
        ],
        Spell::SandJet => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::ShapeEarth),
        ],
        Spell::FleshToStone => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Earth, 6),
        ],
        Spell::StoneToFlesh => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::FleshToStone),
        ],
        Spell::IronArm => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Earth, 4),
        ],
        Spell::Entombment => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Earth, 4),
        ],
        Spell::SummonEarthElemental => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Earth, 8),
        ],
        Spell::PurifyEarth => vec![SpellPrerequisite::Magery(0)],
        Spell::EssentialEarth => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Earth, 6),
        ],
        Spell::BodyOfEarth => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Earth, 6),
        ],
        Spell::EarthToWater => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::EarthToAir),
        ],
        Spell::IdentifyMetal => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::ShapeEarth),
        ],
        Spell::ShapeMetal => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::ShapeStone),
        ],
        _ => panic!("Invalid spell {:?} for Earth college", spell),
    }
}

/// Returns spell type for Earth spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Earth spell type");
    match spell {
        Spell::ShapeEarth => SpellType::Regular,
        Spell::EarthToStone => SpellType::Area,
        Spell::StoneToEarth => SpellType::Area,
        Spell::CreateEarth => SpellType::Regular,
        Spell::DestroyEarth => SpellType::Area,
        Spell::EarthVision => SpellType::Information,
        Spell::WalkThroughEarth => SpellType::Regular,
        Spell::Earthquake => SpellType::Area,
        Spell::ShapeStone => SpellType::Regular,
        Spell::EarthToAir => SpellType::Regular,
        Spell::StoneMissile => SpellType::Missile,
        Spell::CreateStone => SpellType::Regular,
        Spell::SandJet => SpellType::Missile,
        Spell::FleshToStone => SpellType::Regular,
        Spell::StoneToFlesh => SpellType::Regular,
        Spell::IronArm => SpellType::Regular,
        Spell::Entombment => SpellType::Regular,
        Spell::SummonEarthElemental => SpellType::Regular,
        Spell::PurifyEarth => SpellType::Area,
        Spell::EssentialEarth => SpellType::Regular,
        Spell::BodyOfEarth => SpellType::Regular,
        Spell::EarthToWater => SpellType::Regular,
        Spell::IdentifyMetal => SpellType::Information,
        Spell::ShapeMetal => SpellType::Regular,
        _ => panic!("Invalid spell {:?} for Earth college", spell),
    }
}

/// Returns resistance type for Earth spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Earth spell resistance");
    match spell {
        Spell::ShapeEarth => None,
        Spell::EarthToStone => None,
        Spell::StoneToEarth => None,
        Spell::CreateEarth => None,
        Spell::DestroyEarth => None,
        Spell::EarthVision => None,
        Spell::WalkThroughEarth => None,
        Spell::Earthquake => None,
        Spell::ShapeStone => None,
        Spell::EarthToAir => None,
        Spell::StoneMissile => None,
        Spell::CreateStone => None,
        Spell::SandJet => None,
        Spell::FleshToStone => Some(ResistanceType::HT),
        Spell::StoneToFlesh => None,
        Spell::IronArm => None,
        Spell::Entombment => Some(ResistanceType::HT),
        Spell::SummonEarthElemental => None,
        Spell::PurifyEarth => None,
        Spell::EssentialEarth => None,
        Spell::BodyOfEarth => None,
        Spell::EarthToWater => None,
        Spell::IdentifyMetal => None,
        Spell::ShapeMetal => None,
        _ => panic!("Invalid spell {:?} for Earth college", spell),
    }
}

/// Returns spell reference for Earth spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Earth spell reference");
    match spell {
        Spell::ShapeEarth => "M64",
        Spell::EarthToStone => "M59",
        Spell::StoneToEarth => "M66",
        Spell::CreateEarth => "M57",
        Spell::DestroyEarth => "M58",
        Spell::EarthVision => "M59",
        Spell::WalkThroughEarth => "M67",
        Spell::Earthquake => "M59",
        Spell::ShapeStone => "M64",
        Spell::EarthToAir => "M58",
        Spell::StoneMissile => "M66",
        Spell::CreateStone => "M57",
        Spell::SandJet => "M64",
        Spell::FleshToStone => "M60",
        Spell::StoneToFlesh => "M66",
        Spell::IronArm => "M62",
        Spell::Entombment => "M59",
        Spell::SummonEarthElemental => "M58",
        Spell::PurifyEarth => "M63",
        Spell::EssentialEarth => "M60",
        Spell::BodyOfEarth => "M48",
        Spell::EarthToWater => "M59",
        Spell::IdentifyMetal => "M62",
        Spell::ShapeMetal => "M64",
        _ => panic!("Invalid spell {:?} for Earth college", spell),
    }
}
