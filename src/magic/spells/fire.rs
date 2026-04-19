//! Fire college spells.
//!
//! # GURPS Rules
//!
//! Fire spells manipulate heat, flames, and temperature. They create and control
//! fire, resist extreme temperatures, and summon fire elementals.
//!
//! # Citations
//!
//! M 59-75 - Fire college

use crate::{Duration, EnergyCost, ResistanceType, Spell, SpellCollege, SpellPrerequisite, SpellType};
use tracing::{debug, instrument};

/// Returns base energy cost for Fire spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Fire spell energy cost");
    match spell {
        Spell::IgniteFlame => EnergyCost::Fixed(1),
        Spell::CreateFire => EnergyCost::Fixed(4),
        Spell::ExtinguishFire => EnergyCost::Fixed(3),
        Spell::ShapeFire => EnergyCost::Fixed(2),
        Spell::Fireball => EnergyCost::PerDie(1),
        Spell::ExplosiveFireball => EnergyCost::PerDie(2),
        Spell::FlameJet => EnergyCost::PerDie(1),
        Spell::Heat => EnergyCost::Fixed(2),
        Spell::Cold => EnergyCost::Fixed(2),
        Spell::ResistFire => EnergyCost::Fixed(2),
        Spell::ResistCold => EnergyCost::Fixed(2),
        Spell::PurifyAir => EnergyCost::Fixed(1),
        Spell::Frostbite => EnergyCost::PerDie(1),
        Spell::WallOfFire => EnergyCost::Fixed(3),
        Spell::SummonFireElemental => EnergyCost::Fixed(8),
        Spell::PermanentFlame => EnergyCost::Fixed(10),
        Spell::SeekFireFire => EnergyCost::Fixed(2),
        Spell::Ignition => EnergyCost::Fixed(3),
        Spell::EssentialFlame => EnergyCost::Fixed(3),
        Spell::BodyOfFire => EnergyCost::Fixed(5),
        Spell::FireVision => EnergyCost::Fixed(2),
        Spell::Warmth => EnergyCost::Fixed(1),
        Spell::DeflectEnergy => EnergyCost::Fixed(2),
        Spell::ControlFireElemental => EnergyCost::Fixed(8),
        Spell::CreateFireElemental => EnergyCost::Fixed(20),
        _ => panic!("Invalid spell {:?} for Fire college", spell),
    }
}

/// Returns casting time in seconds for Fire spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Fire spell casting time");
    match spell {
        Spell::IgniteFlame => 1,
        Spell::CreateFire => 1,
        Spell::ExtinguishFire => 1,
        Spell::ShapeFire => 1,
        Spell::Fireball => 1,
        Spell::ExplosiveFireball => 2,
        Spell::FlameJet => 1,
        Spell::Heat => 1,
        Spell::Cold => 1,
        Spell::ResistFire => 1,
        Spell::ResistCold => 1,
        Spell::PurifyAir => 1,
        Spell::Frostbite => 1,
        Spell::WallOfFire => 2,
        Spell::SummonFireElemental => 5,
        Spell::PermanentFlame => 10,
        Spell::SeekFireFire => 1,
        Spell::Ignition => 1,
        Spell::EssentialFlame => 2,
        Spell::BodyOfFire => 3,
        Spell::FireVision => 1,
        Spell::Warmth => 1,
        Spell::DeflectEnergy => 1,
        Spell::ControlFireElemental => 3,
        Spell::CreateFireElemental => 10,
        _ => panic!("Invalid spell {:?} for Fire college", spell),
    }
}

/// Returns duration for Fire spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Fire spell duration");
    match spell {
        Spell::IgniteFlame => Duration::Instant,
        Spell::CreateFire => Duration::Concentration,
        Spell::ExtinguishFire => Duration::Instant,
        Spell::ShapeFire => Duration::Concentration,
        Spell::Fireball => Duration::Instant,
        Spell::ExplosiveFireball => Duration::Instant,
        Spell::FlameJet => Duration::Instant,
        Spell::Heat => Duration::Minutes(1),
        Spell::Cold => Duration::Minutes(1),
        Spell::ResistFire => Duration::Minutes(1),
        Spell::ResistCold => Duration::Minutes(1),
        Spell::PurifyAir => Duration::Instant,
        Spell::Frostbite => Duration::Instant,
        Spell::WallOfFire => Duration::Minutes(1),
        Spell::SummonFireElemental => Duration::Minutes(1),
        Spell::PermanentFlame => Duration::Permanent,
        Spell::SeekFireFire => Duration::Concentration,
        Spell::Ignition => Duration::Instant,
        Spell::EssentialFlame => Duration::Hours(1),
        Spell::BodyOfFire => Duration::Minutes(1),
        Spell::FireVision => Duration::Concentration,
        Spell::Warmth => Duration::Minutes(10),
        Spell::DeflectEnergy => Duration::Minutes(1),
        Spell::ControlFireElemental => Duration::Minutes(1),
        Spell::CreateFireElemental => Duration::Permanent,
        _ => panic!("Invalid spell {:?} for Fire college", spell),
    }
}

/// Returns prerequisites for Fire spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Fire spell prerequisites");
    match spell {
        Spell::IgniteFlame => vec![SpellPrerequisite::Magery(0)],
        Spell::CreateFire => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::IgniteFlame)
            ],
        Spell::ExtinguishFire => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::IgniteFlame)
            ],
        Spell::ShapeFire => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::IgniteFlame)
            ],
        Spell::Fireball => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::CreateFire)
            ],
        Spell::ExplosiveFireball => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Fireball)
            ],
        Spell::FlameJet => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::ShapeFire)
            ],
        Spell::Heat => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::IgniteFlame)
            ],
        Spell::Cold => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::Heat)
            ],
        Spell::ResistFire => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::Heat)
            ],
        Spell::ResistCold => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::Cold)
            ],
        Spell::PurifyAir => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::CreateFire)
            ],
        Spell::Frostbite => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Cold)
            ],
        Spell::WallOfFire => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::ShapeFire)
            ],
        Spell::SummonFireElemental => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Fire, 8)
            ],
        Spell::PermanentFlame => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Fire, 4)
            ],
        Spell::SeekFireFire => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::IgniteFlame)
            ],
        Spell::Ignition => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Heat)
            ],
        Spell::EssentialFlame => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Fire, 6)
            ],
        Spell::BodyOfFire => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Fire, 6)
            ],
        Spell::FireVision => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::ShapeFire)
            ],
        Spell::Warmth => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::Heat)
            ],
        Spell::DeflectEnergy => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::ShapeFire)
            ],
        Spell::ControlFireElemental => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::SummonFireElemental)
            ],
        Spell::CreateFireElemental => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::ControlFireElemental)
            ],
        _ => panic!("Invalid spell {:?} for Fire college", spell),
    }
}

/// Returns spell type for Fire spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Fire spell type");
    match spell {
        Spell::IgniteFlame => SpellType::Regular,
        Spell::CreateFire => SpellType::Regular,
        Spell::ExtinguishFire => SpellType::Area,
        Spell::ShapeFire => SpellType::Regular,
        Spell::Fireball => SpellType::Missile,
        Spell::ExplosiveFireball => SpellType::Missile,
        Spell::FlameJet => SpellType::Missile,
        Spell::Heat => SpellType::Regular,
        Spell::Cold => SpellType::Regular,
        Spell::ResistFire => SpellType::Regular,
        Spell::ResistCold => SpellType::Regular,
        Spell::PurifyAir => SpellType::Area,
        Spell::Frostbite => SpellType::Missile,
        Spell::WallOfFire => SpellType::Area,
        Spell::SummonFireElemental => SpellType::Regular,
        Spell::PermanentFlame => SpellType::Regular,
        Spell::SeekFireFire => SpellType::Information,
        Spell::Ignition => SpellType::Regular,
        Spell::EssentialFlame => SpellType::Regular,
        Spell::BodyOfFire => SpellType::Regular,
        Spell::FireVision => SpellType::Information,
        Spell::Warmth => SpellType::Area,
        Spell::DeflectEnergy => SpellType::Blocking,
        Spell::ControlFireElemental => SpellType::Regular,
        Spell::CreateFireElemental => SpellType::Regular,
        _ => panic!("Invalid spell {:?} for Fire college", spell),
    }
}

/// Returns resistance type for Fire spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Fire spell resistance");
    match spell {
        Spell::IgniteFlame => None,
        Spell::CreateFire => None,
        Spell::ExtinguishFire => None,
        Spell::ShapeFire => None,
        Spell::Fireball => None,
        Spell::ExplosiveFireball => None,
        Spell::FlameJet => None,
        Spell::Heat => Some(ResistanceType::HT),
        Spell::Cold => Some(ResistanceType::HT),
        Spell::ResistFire => None,
        Spell::ResistCold => None,
        Spell::PurifyAir => None,
        Spell::Frostbite => None,
        Spell::WallOfFire => None,
        Spell::SummonFireElemental => None,
        Spell::PermanentFlame => None,
        Spell::SeekFireFire => None,
        Spell::Ignition => None,
        Spell::EssentialFlame => None,
        Spell::BodyOfFire => None,
        Spell::FireVision => None,
        Spell::Warmth => None,
        Spell::DeflectEnergy => None,
        Spell::ControlFireElemental => None,
        Spell::CreateFireElemental => None,
        _ => panic!("Invalid spell {:?} for Fire college", spell),
    }
}

/// Returns spell reference for Fire spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Fire spell reference");
    match spell {
        Spell::IgniteFlame => "M68",
        Spell::CreateFire => "M69",
        Spell::ExtinguishFire => "M71",
        Spell::ShapeFire => "M76",
        Spell::Fireball => "M70",
        Spell::ExplosiveFireball => "M71",
        Spell::FlameJet => "M70",
        Spell::Heat => "M72",
        Spell::Cold => "M69",
        Spell::ResistFire => "M75",
        Spell::ResistCold => "M75",
        Spell::PurifyAir => "M74",
        Spell::Frostbite => "M69",
        Spell::WallOfFire => "M76",
        Spell::SummonFireElemental => "M76",
        Spell::PermanentFlame => "M74",
        Spell::SeekFireFire => "M76",
        Spell::Ignition => "M72",
        Spell::EssentialFlame => "M71",
        Spell::BodyOfFire => "M48",
        Spell::FireVision => "M70",
        Spell::Warmth => "M77",
        Spell::DeflectEnergy => "M69",
        Spell::ControlFireElemental => "M69",
        Spell::CreateFireElemental => "M69",
        _ => panic!("Invalid spell {:?} for Fire college", spell),
    }
}
