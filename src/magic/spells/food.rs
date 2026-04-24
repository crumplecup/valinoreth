//! Food college spells.
//!
//! # GURPS Rules
//!
//! Food spells create, preserve, and manipulate food and drink. They include
//! spells for cooking, detecting contamination, and preventing spoilage.
//! The Food College has many Magery 0 spells, making it accessible to non-mages.
//!
//! # Citations
//!
//! M 72-80 - Food college

use crate::{Duration, EnergyCost, ResistanceType, Spell, SpellCollege, SpellPrerequisite, SpellType};
use tracing::{debug, instrument};

/// Returns base energy cost for Food spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Food spell energy cost");
    match spell {
        Spell::Decay => EnergyCost::Fixed(1),
        Spell::TestFood => EnergyCost::Fixed(1),
        Spell::PurifyFood => EnergyCost::Fixed(1),
        Spell::CreateFood => EnergyCost::Fixed(4),
        Spell::PreserveFood => EnergyCost::Fixed(1),
        Spell::Cook => EnergyCost::Fixed(1),
        Spell::Season => EnergyCost::Fixed(1),
        Spell::Flavor => EnergyCost::Fixed(1),
        Spell::PrepareGame => EnergyCost::Fixed(2),
        Spell::PoisonFood => EnergyCost::Fixed(2),
        Spell::KnowRecipe => EnergyCost::Fixed(2),
        Spell::Mature => EnergyCost::Fixed(1),
        Spell::Banquet => EnergyCost::Fixed(10),
        Spell::EssentialFood => EnergyCost::Fixed(2),
        Spell::Distill => EnergyCost::Fixed(2),
        Spell::Ferment => EnergyCost::Fixed(1),
        Spell::WaterToWine => EnergyCost::Fixed(3),
        Spell::Hunger => EnergyCost::Fixed(2),
        Spell::Thirst => EnergyCost::Fixed(2),
        Spell::Rotproof => EnergyCost::Fixed(3),
        _ => panic!("Invalid spell {:?} for Food college", spell),
    }
}

/// Returns casting time in seconds for Food spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Food spell casting time");
    match spell {
        Spell::Decay => 1,
        Spell::TestFood => 2,
        Spell::PurifyFood => 1,
        Spell::CreateFood => 3,
        Spell::PreserveFood => 1,
        Spell::Cook => 1,
        Spell::Season => 1,
        Spell::Flavor => 1,
        Spell::PrepareGame => 2,
        Spell::PoisonFood => 2,
        Spell::KnowRecipe => 3,
        Spell::Mature => 2,
        Spell::Banquet => 10,
        Spell::EssentialFood => 2,
        Spell::Distill => 2,
        Spell::Ferment => 3,
        Spell::WaterToWine => 2,
        Spell::Hunger => 1,
        Spell::Thirst => 1,
        Spell::Rotproof => 2,
        _ => panic!("Invalid spell {:?} for Food college", spell),
    }
}

/// Returns duration for Food spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Food spell duration");
    match spell {
        Spell::Decay => Duration::Permanent,
        Spell::TestFood => Duration::Instant,
        Spell::PurifyFood => Duration::Permanent,
        Spell::CreateFood => Duration::Permanent,
        Spell::PreserveFood => Duration::Permanent,
        Spell::Cook => Duration::Permanent,
        Spell::Season => Duration::Permanent,
        Spell::Flavor => Duration::Permanent,
        Spell::PrepareGame => Duration::Permanent,
        Spell::PoisonFood => Duration::Permanent,
        Spell::KnowRecipe => Duration::Instant,
        Spell::Mature => Duration::Permanent,
        Spell::Banquet => Duration::Permanent,
        Spell::EssentialFood => Duration::Permanent,
        Spell::Distill => Duration::Permanent,
        Spell::Ferment => Duration::Permanent,
        Spell::WaterToWine => Duration::Permanent,
        Spell::Hunger => Duration::Minutes(10),
        Spell::Thirst => Duration::Minutes(10),
        Spell::Rotproof => Duration::Permanent,
        _ => panic!("Invalid spell {:?} for Food college", spell),
    }
}

/// Returns prerequisites for Food spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Food spell prerequisites");
    match spell {
        Spell::Decay => vec![SpellPrerequisite::Magery(0)],
        Spell::TestFood => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::Decay),
        ],
        Spell::PurifyFood => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::Decay),
        ],
        Spell::CreateFood => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::Decay),
        ],
        Spell::PreserveFood => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::Decay),
        ],
        Spell::Cook => vec![SpellPrerequisite::Magery(0)],
        Spell::Season => vec![SpellPrerequisite::Magery(0)],
        Spell::Flavor => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::Season),
        ],
        Spell::PrepareGame => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::PurifyFood),
        ],
        Spell::PoisonFood => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::PurifyFood),
            SpellPrerequisite::Spell(Spell::Decay),
        ],
        Spell::KnowRecipe => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Season),
        ],
        Spell::Mature => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Decay),
        ],
        Spell::Banquet => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Food, 6),
        ],
        Spell::EssentialFood => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::CreateFood),
        ],
        Spell::Distill => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::PurifyFood),
        ],
        Spell::Ferment => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::Decay),
        ],
        Spell::WaterToWine => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Ferment),
        ],
        Spell::Hunger => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::CreateFood),
        ],
        Spell::Thirst => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::CreateFood),
        ],
        Spell::Rotproof => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::PreserveFood),
        ],
        _ => panic!("Invalid spell {:?} for Food college", spell),
    }
}

/// Returns spell type for Food spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Food spell type");
    match spell {
        Spell::Decay => SpellType::Regular,
        Spell::TestFood => SpellType::Information,
        Spell::PurifyFood => SpellType::Regular,
        Spell::CreateFood => SpellType::Regular,
        Spell::PreserveFood => SpellType::Regular,
        Spell::Cook => SpellType::Regular,
        Spell::Season => SpellType::Regular,
        Spell::Flavor => SpellType::Regular,
        Spell::PrepareGame => SpellType::Regular,
        Spell::PoisonFood => SpellType::Regular,
        Spell::KnowRecipe => SpellType::Information,
        Spell::Mature => SpellType::Regular,
        Spell::Banquet => SpellType::Regular,
        Spell::EssentialFood => SpellType::Regular,
        Spell::Distill => SpellType::Regular,
        Spell::Ferment => SpellType::Regular,
        Spell::WaterToWine => SpellType::Regular,
        Spell::Hunger => SpellType::Regular,
        Spell::Thirst => SpellType::Regular,
        Spell::Rotproof => SpellType::Regular,
        _ => panic!("Invalid spell {:?} for Food college", spell),
    }
}

/// Returns resistance type for Food spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Food spell resistance");
    match spell {
        Spell::Decay => None,
        Spell::TestFood => None,
        Spell::PurifyFood => None,
        Spell::CreateFood => None,
        Spell::PreserveFood => None,
        Spell::Cook => None,
        Spell::Season => None,
        Spell::Flavor => None,
        Spell::PrepareGame => None,
        Spell::PoisonFood => None,
        Spell::KnowRecipe => None,
        Spell::Mature => None,
        Spell::Banquet => None,
        Spell::EssentialFood => None,
        Spell::Distill => None,
        Spell::Ferment => None,
        Spell::WaterToWine => None,
        Spell::Hunger => Some(ResistanceType::HT),
        Spell::Thirst => Some(ResistanceType::HT),
        Spell::Rotproof => None,
        _ => panic!("Invalid spell {:?} for Food college", spell),
    }
}

/// Returns page reference for Food spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Food spell reference");
    match spell {
        Spell::Decay => "M77",
        Spell::TestFood => "M78",
        Spell::PurifyFood => "M78",
        Spell::CreateFood => "M77",
        Spell::PreserveFood => "M78",
        Spell::Cook => "M77",
        Spell::Season => "M78",
        Spell::Flavor => "M77",
        Spell::PrepareGame => "M78",
        Spell::PoisonFood => "M78",
        Spell::KnowRecipe => "M77",
        Spell::Mature => "M77",
        Spell::Banquet => "M77",
        Spell::EssentialFood => "M77",
        Spell::Distill => "M77",
        Spell::Ferment => "M77",
        Spell::WaterToWine => "M79",
        Spell::Hunger => "M77",
        Spell::Thirst => "M79",
        Spell::Rotproof => "M78",
        _ => panic!("Invalid spell {:?} for Food college", spell),
    }
}
