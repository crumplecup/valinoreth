//! Plant college spells.
//!
//! # GURPS Rules
//!
//! Plant spells manipulate vegetation, control plant growth, and animate
//! plant life. They include spells for healing plants, shaping wood,
//! summoning plant elementals, and commanding forests.
//!
//! # Citations
//!
//! M 155-170 - Plant college

use crate::{Duration, EnergyCost, ResistanceType, Spell, SpellPrerequisite, SpellType};
use tracing::{debug, instrument};

/// Returns base energy cost for Plant spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Plant spell energy cost");
    match spell {
        Spell::IdentifyPlant => EnergyCost::Fixed(1),
        Spell::SeekPlantPlant => EnergyCost::Fixed(2),
        Spell::PlantGrowth => EnergyCost::Fixed(3),
        Spell::HealPlant => EnergyCost::Fixed(2),
        Spell::WitherPlant => EnergyCost::Fixed(2),
        Spell::ShapeWood => EnergyCost::Fixed(2),
        Spell::ControlPlant => EnergyCost::Fixed(2),
        Spell::AnimatePlant => EnergyCost::Fixed(4),
        Spell::Entangle => EnergyCost::Fixed(3),
        Spell::ThornWall => EnergyCost::Fixed(3),
        Spell::FertileSoil => EnergyCost::Fixed(4),
        Spell::Blight => EnergyCost::Fixed(4),
        Spell::FruitBearing => EnergyCost::Fixed(3),
        Spell::SummonPlantElemental => EnergyCost::Fixed(8),
        Spell::ControlPlantElemental => EnergyCost::Fixed(3),
        Spell::CreatePlantElemental => EnergyCost::Fixed(20),
        Spell::PlantForm => EnergyCost::Fixed(6),
        Spell::SpeakWithPlants => EnergyCost::Fixed(2),
        Spell::PlantVision => EnergyCost::Fixed(3),
        Spell::Treant => EnergyCost::Fixed(10),
        Spell::AwakenPlant => EnergyCost::Fixed(8),
        Spell::ProtectPlant => EnergyCost::Fixed(2),
        Spell::PoisonExtract => EnergyCost::Fixed(3),
        Spell::PurifyPlant => EnergyCost::Fixed(2),
        Spell::HerbLore => EnergyCost::Fixed(2),
        Spell::ForestControl => EnergyCost::Fixed(10),
        Spell::HedgeWall => EnergyCost::Fixed(4),
        Spell::Decompose => EnergyCost::Fixed(2),
        Spell::PreservePlant => EnergyCost::Fixed(2),
        Spell::VineServant => EnergyCost::Fixed(5),
        Spell::LifeToPlant => EnergyCost::Fixed(4),
        Spell::LifeFromPlant => EnergyCost::Fixed(3),
        _ => panic!("Invalid spell {:?} for Plant college", spell),
    }
}

/// Returns casting time in seconds for Plant spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Plant spell casting time");
    match spell {
        Spell::IdentifyPlant => 1,
        Spell::SeekPlantPlant => 2,
        Spell::PlantGrowth => 2,
        Spell::HealPlant => 2,
        Spell::WitherPlant => 2,
        Spell::ShapeWood => 2,
        Spell::ControlPlant => 2,
        Spell::AnimatePlant => 3,
        Spell::Entangle => 2,
        Spell::ThornWall => 3,
        Spell::FertileSoil => 3,
        Spell::Blight => 3,
        Spell::FruitBearing => 2,
        Spell::SummonPlantElemental => 5,
        Spell::ControlPlantElemental => 2,
        Spell::CreatePlantElemental => 10,
        Spell::PlantForm => 4,
        Spell::SpeakWithPlants => 2,
        Spell::PlantVision => 2,
        Spell::Treant => 5,
        Spell::AwakenPlant => 5,
        Spell::ProtectPlant => 2,
        Spell::PoisonExtract => 3,
        Spell::PurifyPlant => 2,
        Spell::HerbLore => 2,
        Spell::ForestControl => 5,
        Spell::HedgeWall => 3,
        Spell::Decompose => 2,
        Spell::PreservePlant => 2,
        Spell::VineServant => 4,
        Spell::LifeToPlant => 3,
        Spell::LifeFromPlant => 2,
        _ => panic!("Invalid spell {:?} for Plant college", spell),
    }
}

/// Returns duration for Plant spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Plant spell duration");
    match spell {
        Spell::IdentifyPlant => Duration::Instant,
        Spell::SeekPlantPlant => Duration::Concentration,
        Spell::PlantGrowth => Duration::Permanent,
        Spell::HealPlant => Duration::Instant,
        Spell::WitherPlant => Duration::Instant,
        Spell::ShapeWood => Duration::Permanent,
        Spell::ControlPlant => Duration::Minutes(10),
        Spell::AnimatePlant => Duration::Concentration,
        Spell::Entangle => Duration::Minutes(1),
        Spell::ThornWall => Duration::Minutes(10),
        Spell::FertileSoil => Duration::Permanent,
        Spell::Blight => Duration::Permanent,
        Spell::FruitBearing => Duration::Permanent,
        Spell::SummonPlantElemental => Duration::Minutes(10),
        Spell::ControlPlantElemental => Duration::Minutes(10),
        Spell::CreatePlantElemental => Duration::Permanent,
        Spell::PlantForm => Duration::Hours(1),
        Spell::SpeakWithPlants => Duration::Minutes(10),
        Spell::PlantVision => Duration::Concentration,
        Spell::Treant => Duration::Permanent,
        Spell::AwakenPlant => Duration::Permanent,
        Spell::ProtectPlant => Duration::Minutes(10),
        Spell::PoisonExtract => Duration::Permanent,
        Spell::PurifyPlant => Duration::Instant,
        Spell::HerbLore => Duration::Instant,
        Spell::ForestControl => Duration::Concentration,
        Spell::HedgeWall => Duration::Permanent,
        Spell::Decompose => Duration::Instant,
        Spell::PreservePlant => Duration::Permanent,
        Spell::VineServant => Duration::Concentration,
        Spell::LifeToPlant => Duration::Instant,
        Spell::LifeFromPlant => Duration::Instant,
        _ => panic!("Invalid spell {:?} for Plant college", spell),
    }
}

/// Returns prerequisites for Plant spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Plant spell prerequisites");
    match spell {
        Spell::IdentifyPlant => vec![SpellPrerequisite::Magery(0)],
        Spell::SeekPlantPlant => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::IdentifyPlant),
        ],
        Spell::PlantGrowth => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::IdentifyPlant),
        ],
        Spell::HealPlant => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::IdentifyPlant),
        ],
        Spell::WitherPlant => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::HealPlant),
        ],
        Spell::ShapeWood => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::IdentifyPlant),
        ],
        Spell::ControlPlant => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::IdentifyPlant),
        ],
        Spell::AnimatePlant => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::ControlPlant),
        ],
        Spell::Entangle => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::PlantGrowth),
        ],
        Spell::ThornWall => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Entangle),
        ],
        Spell::FertileSoil => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::PlantGrowth),
        ],
        Spell::Blight => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::WitherPlant),
        ],
        Spell::FruitBearing => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::PlantGrowth),
        ],
        Spell::SummonPlantElemental => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::AnimatePlant),
        ],
        Spell::ControlPlantElemental => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::SummonPlantElemental),
        ],
        Spell::CreatePlantElemental => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::SummonPlantElemental),
        ],
        Spell::PlantForm => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::AnimatePlant),
        ],
        Spell::SpeakWithPlants => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::ControlPlant),
        ],
        Spell::PlantVision => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::SpeakWithPlants),
        ],
        Spell::Treant => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::AnimatePlant),
        ],
        Spell::AwakenPlant => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::ControlPlant),
        ],
        Spell::ProtectPlant => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::HealPlant),
        ],
        Spell::PoisonExtract => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::IdentifyPlant),
        ],
        Spell::PurifyPlant => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::PoisonExtract),
        ],
        Spell::HerbLore => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::IdentifyPlant),
        ],
        Spell::ForestControl => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::ControlPlant),
        ],
        Spell::HedgeWall => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::ThornWall),
        ],
        Spell::Decompose => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::WitherPlant),
        ],
        Spell::PreservePlant => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::HealPlant),
        ],
        Spell::VineServant => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::AnimatePlant),
        ],
        Spell::LifeToPlant => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::PlantGrowth),
        ],
        Spell::LifeFromPlant => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::WitherPlant),
        ],
        _ => panic!("Invalid spell {:?} for Plant college", spell),
    }
}

/// Returns spell type for Plant spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Plant spell type");
    match spell {
        Spell::IdentifyPlant => SpellType::Information,
        Spell::SeekPlantPlant => SpellType::Information,
        Spell::PlantGrowth => SpellType::Regular,
        Spell::HealPlant => SpellType::Regular,
        Spell::WitherPlant => SpellType::Regular,
        Spell::ShapeWood => SpellType::Regular,
        Spell::ControlPlant => SpellType::Regular,
        Spell::AnimatePlant => SpellType::Regular,
        Spell::Entangle => SpellType::Area,
        Spell::ThornWall => SpellType::Area,
        Spell::FertileSoil => SpellType::Area,
        Spell::Blight => SpellType::Area,
        Spell::FruitBearing => SpellType::Regular,
        Spell::SummonPlantElemental => SpellType::Regular,
        Spell::ControlPlantElemental => SpellType::Regular,
        Spell::CreatePlantElemental => SpellType::Regular,
        Spell::PlantForm => SpellType::Regular,
        Spell::SpeakWithPlants => SpellType::Regular,
        Spell::PlantVision => SpellType::Regular,
        Spell::Treant => SpellType::Regular,
        Spell::AwakenPlant => SpellType::Regular,
        Spell::ProtectPlant => SpellType::Regular,
        Spell::PoisonExtract => SpellType::Regular,
        Spell::PurifyPlant => SpellType::Regular,
        Spell::HerbLore => SpellType::Information,
        Spell::ForestControl => SpellType::Area,
        Spell::HedgeWall => SpellType::Area,
        Spell::Decompose => SpellType::Regular,
        Spell::PreservePlant => SpellType::Regular,
        Spell::VineServant => SpellType::Regular,
        Spell::LifeToPlant => SpellType::Regular,
        Spell::LifeFromPlant => SpellType::Regular,
        _ => panic!("Invalid spell {:?} for Plant college", spell),
    }
}

/// Returns resistance type for Plant spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Plant spell resistance");
    match spell {
        Spell::IdentifyPlant => None,
        Spell::SeekPlantPlant => None,
        Spell::PlantGrowth => None,
        Spell::HealPlant => None,
        Spell::WitherPlant => None,
        Spell::ShapeWood => None,
        Spell::ControlPlant => None,
        Spell::AnimatePlant => None,
        Spell::Entangle => Some(ResistanceType::HT),
        Spell::ThornWall => None,
        Spell::FertileSoil => None,
        Spell::Blight => None,
        Spell::FruitBearing => None,
        Spell::SummonPlantElemental => Some(ResistanceType::Will),
        Spell::ControlPlantElemental => Some(ResistanceType::Will),
        Spell::CreatePlantElemental => None,
        Spell::PlantForm => None,
        Spell::SpeakWithPlants => None,
        Spell::PlantVision => None,
        Spell::Treant => None,
        Spell::AwakenPlant => None,
        Spell::ProtectPlant => None,
        Spell::PoisonExtract => None,
        Spell::PurifyPlant => None,
        Spell::HerbLore => None,
        Spell::ForestControl => None,
        Spell::HedgeWall => None,
        Spell::Decompose => None,
        Spell::PreservePlant => None,
        Spell::VineServant => None,
        Spell::LifeToPlant => None,
        Spell::LifeFromPlant => None,
        _ => panic!("Invalid spell {:?} for Plant college", spell),
    }
}

/// Returns page reference for Plant spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Plant spell reference");
    match spell {
        Spell::IdentifyPlant => "M155",
        Spell::SeekPlantPlant => "M156",
        Spell::PlantGrowth => "M156",
        Spell::HealPlant => "M155",
        Spell::WitherPlant => "M156",
        Spell::ShapeWood => "M159",
        Spell::ControlPlant => "M155",
        Spell::AnimatePlant => "M155",
        Spell::Entangle => "M156",
        Spell::ThornWall => "M159",
        Spell::FertileSoil => "M156",
        Spell::Blight => "M156",
        Spell::FruitBearing => "M156",
        Spell::SummonPlantElemental => "M160",
        Spell::ControlPlantElemental => "M155",
        Spell::CreatePlantElemental => "M155",
        Spell::PlantForm => "M160",
        Spell::SpeakWithPlants => "M159",
        Spell::PlantVision => "M159",
        Spell::Treant => "M160",
        Spell::AwakenPlant => "M155",
        Spell::ProtectPlant => "M158",
        Spell::PoisonExtract => "M158",
        Spell::PurifyPlant => "M158",
        Spell::HerbLore => "M157",
        Spell::ForestControl => "M156",
        Spell::HedgeWall => "M157",
        Spell::Decompose => "M155",
        Spell::PreservePlant => "M158",
        Spell::VineServant => "M160",
        Spell::LifeToPlant => "M160",
        Spell::LifeFromPlant => "M157",
        _ => panic!("Invalid spell {:?} for Plant college", spell),
    }
}
