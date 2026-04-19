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
        _ => panic!("Invalid spell {:?} for Water college", spell),
    }
}
