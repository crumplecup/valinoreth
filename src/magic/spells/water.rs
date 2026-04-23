//! Water college spells.
//!
//! # GURPS Rules
//!
//! Water spells manipulate water, ice, and liquids. They control water's form and
//! state, allow movement through water, and summon water elementals.
//!
//! # Citations
//!
//! M 186-200 - Water college

use crate::{Duration, EnergyCost, ResistanceType, Spell, SpellCollege, SpellPrerequisite, SpellType};
use tracing::{debug, instrument};

/// Returns base energy cost for Water spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Water spell energy cost");
    match spell {
        Spell::PurifyWater => EnergyCost::Fixed(1),
        Spell::CreateWater => EnergyCost::Fixed(2),
        Spell::DestroyWater => EnergyCost::Fixed(2),
        Spell::ShapeWater => EnergyCost::Fixed(2),
        Spell::Freeze => EnergyCost::Fixed(3),
        Spell::Boil => EnergyCost::Fixed(2),
        Spell::WalkOnWater => EnergyCost::Fixed(3),
        Spell::WaterJet => EnergyCost::PerDie(1),
        Spell::IceSphere => EnergyCost::PerDie(1),
        Spell::IceSlick => EnergyCost::Fixed(2),
        Spell::BodyOfWater => EnergyCost::Fixed(5),
        Spell::Dehydrate => EnergyCost::Fixed(3),
        Spell::CreateIce => EnergyCost::Fixed(3),
        Spell::CondenseSteam => EnergyCost::Fixed(2),
        Spell::IceDagger => EnergyCost::Fixed(2),
        Spell::Swim => EnergyCost::Fixed(2),
        Spell::SummonWaterElemental => EnergyCost::Fixed(10),
        Spell::BreatheWaterWater => EnergyCost::Fixed(3),
        Spell::EssentialWater => EnergyCost::Fixed(4),
        Spell::WaterVision => EnergyCost::Fixed(2),
        Spell::IceVision => EnergyCost::Fixed(2),
        Spell::Fog => EnergyCost::Fixed(2),
        Spell::Frost => EnergyCost::Fixed(2),
        Spell::Geyser => EnergyCost::Fixed(3),
        Spell::Hail => EnergyCost::PerDie(1),
        Spell::Whirlpool => EnergyCost::Fixed(4),
        Spell::CreateSteam => EnergyCost::Fixed(2),
        Spell::ResistWater => EnergyCost::Fixed(2),
        Spell::BreatheAir => EnergyCost::Fixed(3),
        _ => panic!("Invalid spell {:?} for Water college", spell),
    }
}

/// Returns casting time in seconds for Water spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Water spell casting time");
    match spell {
        Spell::PurifyWater => 1,
        Spell::CreateWater => 1,
        Spell::DestroyWater => 1,
        Spell::ShapeWater => 1,
        Spell::Freeze => 2,
        Spell::Boil => 1,
        Spell::WalkOnWater => 2,
        Spell::WaterJet => 1,
        Spell::IceSphere => 1,
        Spell::IceSlick => 1,
        Spell::BodyOfWater => 3,
        Spell::Dehydrate => 2,
        Spell::CreateIce => 2,
        Spell::CondenseSteam => 1,
        Spell::IceDagger => 1,
        Spell::Swim => 2,
        Spell::SummonWaterElemental => 5,
        Spell::BreatheWaterWater => 2,
        Spell::EssentialWater => 3,
        Spell::WaterVision => 2,
        Spell::IceVision => 1,
        Spell::Fog => 2,
        Spell::Frost => 1,
        Spell::Geyser => 2,
        Spell::Hail => 2,
        Spell::Whirlpool => 3,
        Spell::CreateSteam => 1,
        Spell::ResistWater => 2,
        Spell::BreatheAir => 2,
        _ => panic!("Invalid spell {:?} for Water college", spell),
    }
}

/// Returns duration for Water spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Water spell duration");
    match spell {
        Spell::PurifyWater => Duration::Instant,
        Spell::CreateWater => Duration::Permanent,
        Spell::DestroyWater => Duration::Instant,
        Spell::ShapeWater => Duration::Concentration,
        Spell::Freeze => Duration::Permanent,
        Spell::Boil => Duration::Instant,
        Spell::WalkOnWater => Duration::Minutes(1),
        Spell::WaterJet => Duration::Instant,
        Spell::IceSphere => Duration::Instant,
        Spell::IceSlick => Duration::Permanent,
        Spell::BodyOfWater => Duration::Minutes(1),
        Spell::Dehydrate => Duration::Instant,
        Spell::CreateIce => Duration::Permanent,
        Spell::CondenseSteam => Duration::Permanent,
        Spell::IceDagger => Duration::Minutes(1),
        Spell::Swim => Duration::Minutes(10),
        Spell::SummonWaterElemental => Duration::Minutes(1),
        Spell::BreatheWaterWater => Duration::Minutes(10),
        Spell::EssentialWater => Duration::Instant,
        Spell::WaterVision => Duration::Minutes(1),
        Spell::IceVision => Duration::Minutes(1),
        Spell::Fog => Duration::Minutes(1),
        Spell::Frost => Duration::Instant,
        Spell::Geyser => Duration::Instant,
        Spell::Hail => Duration::Instant,
        Spell::Whirlpool => Duration::Concentration,
        Spell::CreateSteam => Duration::Permanent,
        Spell::ResistWater => Duration::Minutes(1),
        Spell::BreatheAir => Duration::Minutes(10),
        _ => panic!("Invalid spell {:?} for Water college", spell),
    }
}

/// Returns prerequisites for Water spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Water spell prerequisites");
    match spell {
        Spell::PurifyWater => vec![SpellPrerequisite::Magery(0)],
        Spell::CreateWater => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::PurifyWater)
            ],
        Spell::DestroyWater => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::CreateWater)
            ],
        Spell::ShapeWater => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::CreateWater)
            ],
        Spell::Freeze => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::CreateWater)
            ],
        Spell::Boil => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::CreateWater)
            ],
        Spell::WalkOnWater => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Water, 4)
            ],
        Spell::WaterJet => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::ShapeWater)
            ],
        Spell::IceSphere => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Freeze)
            ],
        Spell::IceSlick => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::Freeze)
            ],
        Spell::BodyOfWater => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Water, 6)
            ],
        Spell::Dehydrate => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::DestroyWater)
            ],
        Spell::CreateIce => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Freeze)
            ],
        Spell::CondenseSteam => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::CreateWater)
            ],
        Spell::IceDagger => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::CreateIce)
            ],
        Spell::Swim => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::ShapeWater)
            ],
        Spell::SummonWaterElemental => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Water, 8)
            ],
        Spell::BreatheWaterWater => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Water, 4)
            ],
        Spell::EssentialWater => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Water, 8)
            ],
        Spell::WaterVision => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Water, 4)
            ],
        Spell::IceVision => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Freeze)
            ],
        Spell::Fog => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::CreateWater)
            ],
        Spell::Frost => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::Freeze)
            ],
        Spell::Geyser => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::WaterJet)
            ],
        Spell::Hail => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::IceSphere)
            ],
        Spell::Whirlpool => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Water, 6)
            ],
        Spell::CreateSteam => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::Boil)
            ],
        Spell::ResistWater => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Water, 4)
            ],
        Spell::BreatheAir => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::BreatheWaterWater)
            ],
        _ => panic!("Invalid spell {:?} for Water college", spell),
    }
}

/// Returns spell type for Water spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Water spell type");
    match spell {
        Spell::PurifyWater => SpellType::Area,
        Spell::CreateWater => SpellType::Regular,
        Spell::DestroyWater => SpellType::Area,
        Spell::ShapeWater => SpellType::Regular,
        Spell::Freeze => SpellType::Area,
        Spell::Boil => SpellType::Area,
        Spell::WalkOnWater => SpellType::Regular,
        Spell::WaterJet => SpellType::Missile,
        Spell::IceSphere => SpellType::Missile,
        Spell::IceSlick => SpellType::Area,
        Spell::BodyOfWater => SpellType::Regular,
        Spell::Dehydrate => SpellType::Regular,
        Spell::CreateIce => SpellType::Regular,
        Spell::CondenseSteam => SpellType::Regular,
        Spell::IceDagger => SpellType::Melee,
        Spell::Swim => SpellType::Regular,
        Spell::SummonWaterElemental => SpellType::Regular,
        Spell::BreatheWaterWater => SpellType::Regular,
        Spell::EssentialWater => SpellType::Information,
        Spell::WaterVision => SpellType::Regular,
        Spell::IceVision => SpellType::Regular,
        Spell::Fog => SpellType::Area,
        Spell::Frost => SpellType::Regular,
        Spell::Geyser => SpellType::Area,
        Spell::Hail => SpellType::Area,
        Spell::Whirlpool => SpellType::Area,
        Spell::CreateSteam => SpellType::Regular,
        Spell::ResistWater => SpellType::Regular,
        Spell::BreatheAir => SpellType::Regular,
        _ => panic!("Invalid spell {:?} for Water college", spell),
    }
}

/// Returns resistance type for Water spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Water spell resistance");
    match spell {
        Spell::PurifyWater => None,
        Spell::CreateWater => None,
        Spell::DestroyWater => None,
        Spell::ShapeWater => None,
        Spell::Freeze => None,
        Spell::Boil => None,
        Spell::WalkOnWater => None,
        Spell::WaterJet => None,
        Spell::IceSphere => None,
        Spell::IceSlick => None,
        Spell::BodyOfWater => None,
        Spell::Dehydrate => Some(ResistanceType::HT),
        Spell::CreateIce => None,
        Spell::CondenseSteam => None,
        Spell::IceDagger => None,
        Spell::Swim => None,
        Spell::SummonWaterElemental => None,
        Spell::BreatheWaterWater => None,
        Spell::EssentialWater => None,
        Spell::WaterVision => None,
        Spell::IceVision => None,
        Spell::Fog => None,
        Spell::Frost => None,
        Spell::Geyser => None,
        Spell::Hail => None,
        Spell::Whirlpool => None,
        Spell::CreateSteam => None,
        Spell::ResistWater => None,
        Spell::BreatheAir => None,
        _ => panic!("Invalid spell {:?} for Water college", spell),
    }
}

/// Returns spell reference for Water spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Water spell reference");
    match spell {
        Spell::PurifyWater => "M196",
        Spell::CreateWater => "M191",
        Spell::DestroyWater => "M192",
        Spell::ShapeWater => "M199",
        Spell::Freeze => "M193",
        Spell::Boil => "M189",
        Spell::WalkOnWater => "M200",
        Spell::WaterJet => "M200",
        Spell::IceSphere => "M194",
        Spell::IceSlick => "M194",
        Spell::BodyOfWater => "M189",
        Spell::Dehydrate => "M192",
        Spell::CreateIce => "M191",
        Spell::CondenseSteam => "M191",
        Spell::IceDagger => "M194",
        Spell::Swim => "M199",
        Spell::SummonWaterElemental => "M199",
        Spell::BreatheWaterWater => "M189",
        Spell::EssentialWater => "M192",
        Spell::WaterVision => "M200",
        Spell::IceVision => "M194",
        Spell::Fog => "M193",
        Spell::Frost => "M193",
        Spell::Geyser => "M193",
        Spell::Hail => "M193",
        Spell::Whirlpool => "M200",
        Spell::CreateSteam => "M191",
        Spell::ResistWater => "M198",
        Spell::BreatheAir => "M189",
        _ => panic!("Invalid spell {:?} for Water college", spell),
    }
}
