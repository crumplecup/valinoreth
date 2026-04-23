//! Animal college spells.
//!
//! # GURPS Rules
//!
//! Animal spells control, communicate with, and transform into animals.
//! They include beast control, shapeshifting, and summoning spells.
//!
//! # Citations
//!
//! M 16-23 - Animal college

use crate::{Duration, EnergyCost, ResistanceType, Spell, SpellCollege, SpellPrerequisite, SpellType};
use tracing::{debug, instrument};

/// Returns base energy cost for Animal spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Animal spell energy cost");
    match spell {
        Spell::BeastSoother => EnergyCost::Fixed(2),
        Spell::BeastSpeech => EnergyCost::Fixed(2),
        Spell::MammalControl => EnergyCost::Fixed(2),
        Spell::BirdControl => EnergyCost::Fixed(2),
        Spell::ReptileControl => EnergyCost::Fixed(2),
        Spell::FishControl => EnergyCost::Fixed(2),
        Spell::InsectControl => EnergyCost::Fixed(2),
        Spell::BeastSummoning => EnergyCost::Fixed(3),
        Spell::RepelAnimal => EnergyCost::Fixed(2),
        Spell::RepelMammal => EnergyCost::Fixed(2),
        Spell::RepelBird => EnergyCost::Fixed(2),
        Spell::RepelReptile => EnergyCost::Fixed(2),
        Spell::RepelFish => EnergyCost::Fixed(2),
        Spell::RepelInsect => EnergyCost::Fixed(2),
        Spell::Master => EnergyCost::Fixed(3),
        Spell::Rider => EnergyCost::Fixed(2),
        Spell::Shapeshifting => EnergyCost::Fixed(6),
        Spell::BirdShapeshifting => EnergyCost::Fixed(6),
        Spell::ReptileShapeshifting => EnergyCost::Fixed(6),
        Spell::FishShapeshifting => EnergyCost::Fixed(6),
        Spell::InsectShapeshifting => EnergyCost::Fixed(6),
        Spell::GreatShapeshifting => EnergyCost::Fixed(10),
        _ => panic!("Invalid spell {:?} for Animal college", spell),
    }
}

/// Returns casting time in seconds for Animal spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Animal spell casting time");
    match spell {
        Spell::BeastSoother => 1,
        Spell::BeastSpeech => 2,
        Spell::MammalControl => 2,
        Spell::BirdControl => 2,
        Spell::ReptileControl => 2,
        Spell::FishControl => 2,
        Spell::InsectControl => 2,
        Spell::BeastSummoning => 3,
        Spell::RepelAnimal => 1,
        Spell::RepelMammal => 1,
        Spell::RepelBird => 1,
        Spell::RepelReptile => 1,
        Spell::RepelFish => 1,
        Spell::RepelInsect => 1,
        Spell::Master => 2,
        Spell::Rider => 2,
        Spell::Shapeshifting => 4,
        Spell::BirdShapeshifting => 4,
        Spell::ReptileShapeshifting => 4,
        Spell::FishShapeshifting => 4,
        Spell::InsectShapeshifting => 4,
        Spell::GreatShapeshifting => 5,
        _ => panic!("Invalid spell {:?} for Animal college", spell),
    }
}

/// Returns duration for Animal spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Animal spell duration");
    match spell {
        Spell::BeastSoother => Duration::Permanent,
        Spell::BeastSpeech => Duration::Minutes(1),
        Spell::MammalControl => Duration::Concentration,
        Spell::BirdControl => Duration::Concentration,
        Spell::ReptileControl => Duration::Concentration,
        Spell::FishControl => Duration::Concentration,
        Spell::InsectControl => Duration::Concentration,
        Spell::BeastSummoning => Duration::Minutes(10),
        Spell::RepelAnimal => Duration::Minutes(1),
        Spell::RepelMammal => Duration::Minutes(1),
        Spell::RepelBird => Duration::Minutes(1),
        Spell::RepelReptile => Duration::Minutes(1),
        Spell::RepelFish => Duration::Minutes(1),
        Spell::RepelInsect => Duration::Minutes(1),
        Spell::Master => Duration::Concentration,
        Spell::Rider => Duration::Minutes(5),
        Spell::Shapeshifting => Duration::Hours(1),
        Spell::BirdShapeshifting => Duration::Hours(1),
        Spell::ReptileShapeshifting => Duration::Hours(1),
        Spell::FishShapeshifting => Duration::Hours(1),
        Spell::InsectShapeshifting => Duration::Hours(1),
        Spell::GreatShapeshifting => Duration::Hours(1),
        _ => panic!("Invalid spell {:?} for Animal college", spell),
    }
}

/// Returns prerequisites for Animal spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Animal spell prerequisites");
    match spell {
        Spell::BeastSoother => vec![SpellPrerequisite::Magery(0)],
        Spell::BeastSpeech => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::BeastSoother),
        ],
        Spell::MammalControl => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::BeastSoother),
        ],
        Spell::BirdControl => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::BeastSoother),
        ],
        Spell::ReptileControl => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::BeastSoother),
        ],
        Spell::FishControl => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::BeastSoother),
        ],
        Spell::InsectControl => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::BeastSoother),
        ],
        Spell::BeastSummoning => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::BeastSoother),
        ],
        Spell::RepelAnimal => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::BeastSoother),
        ],
        Spell::RepelMammal => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::MammalControl),
        ],
        Spell::RepelBird => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::BirdControl),
        ],
        Spell::RepelReptile => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::ReptileControl),
        ],
        Spell::RepelFish => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::FishControl),
        ],
        Spell::RepelInsect => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::InsectControl),
        ],
        Spell::Master => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::BeastSoother),
        ],
        Spell::Rider => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Animal, 1),
        ],
        Spell::Shapeshifting => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Animal, 6),
        ],
        Spell::BirdShapeshifting => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::BirdControl),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Animal, 6),
        ],
        Spell::ReptileShapeshifting => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::ReptileControl),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Animal, 6),
        ],
        Spell::FishShapeshifting => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::FishControl),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Animal, 6),
        ],
        Spell::InsectShapeshifting => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::InsectControl),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Animal, 6),
        ],
        Spell::GreatShapeshifting => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Animal, 10),
        ],
        _ => panic!("Invalid spell {:?} for Animal college", spell),
    }
}

/// Returns spell type for Animal spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Animal spell type");
    match spell {
        Spell::BeastSoother => SpellType::Regular,
        Spell::BeastSpeech => SpellType::Regular,
        Spell::MammalControl => SpellType::Regular,
        Spell::BirdControl => SpellType::Regular,
        Spell::ReptileControl => SpellType::Regular,
        Spell::FishControl => SpellType::Regular,
        Spell::InsectControl => SpellType::Regular,
        Spell::BeastSummoning => SpellType::Regular,
        Spell::RepelAnimal => SpellType::Area,
        Spell::RepelMammal => SpellType::Area,
        Spell::RepelBird => SpellType::Area,
        Spell::RepelReptile => SpellType::Area,
        Spell::RepelFish => SpellType::Area,
        Spell::RepelInsect => SpellType::Area,
        Spell::Master => SpellType::Regular,
        Spell::Rider => SpellType::Regular,
        Spell::Shapeshifting => SpellType::Regular,
        Spell::BirdShapeshifting => SpellType::Regular,
        Spell::ReptileShapeshifting => SpellType::Regular,
        Spell::FishShapeshifting => SpellType::Regular,
        Spell::InsectShapeshifting => SpellType::Regular,
        Spell::GreatShapeshifting => SpellType::Regular,
        _ => panic!("Invalid spell {:?} for Animal college", spell),
    }
}

/// Returns resistance type for Animal spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Animal spell resistance");
    match spell {
        Spell::BeastSoother => None,
        Spell::BeastSpeech => None,
        Spell::MammalControl => Some(ResistanceType::Will),
        Spell::BirdControl => Some(ResistanceType::Will),
        Spell::ReptileControl => Some(ResistanceType::Will),
        Spell::FishControl => Some(ResistanceType::Will),
        Spell::InsectControl => Some(ResistanceType::Will),
        Spell::BeastSummoning => None,
        Spell::RepelAnimal => None,
        Spell::RepelMammal => None,
        Spell::RepelBird => None,
        Spell::RepelReptile => None,
        Spell::RepelFish => None,
        Spell::RepelInsect => None,
        Spell::Master => Some(ResistanceType::Will),
        Spell::Rider => None,
        Spell::Shapeshifting => None,
        Spell::BirdShapeshifting => None,
        Spell::ReptileShapeshifting => None,
        Spell::FishShapeshifting => None,
        Spell::InsectShapeshifting => None,
        Spell::GreatShapeshifting => None,
        _ => panic!("Invalid spell {:?} for Animal college", spell),
    }
}

/// Returns page reference for Animal spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Animal spell reference");
    match spell {
        Spell::BeastSoother => "M30",
        Spell::BeastSpeech => "M29",
        Spell::MammalControl => "M30",
        Spell::BirdControl => "M29",
        Spell::ReptileControl => "M33",
        Spell::FishControl => "M30",
        Spell::InsectControl => "M31",
        Spell::BeastSummoning => "M29",
        Spell::RepelAnimal => "M32",
        Spell::RepelMammal => "M32",
        Spell::RepelBird => "M32",
        Spell::RepelReptile => "M32",
        Spell::RepelFish => "M32",
        Spell::RepelInsect => "M32",
        Spell::Master => "M31",
        Spell::Rider => "M33",
        Spell::Shapeshifting => "M34",
        Spell::BirdShapeshifting => "M34",
        Spell::ReptileShapeshifting => "M34",
        Spell::FishShapeshifting => "M34",
        Spell::InsectShapeshifting => "M34",
        Spell::GreatShapeshifting => "M31",
        _ => panic!("Invalid spell {:?} for Animal college", spell),
    }
}
