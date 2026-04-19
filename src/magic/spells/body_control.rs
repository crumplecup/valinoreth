//! Body Control college spells.
//!
//! # GURPS Rules
//!
//! Body Control spells manipulate the body's functions, enhancing or impairing
//! physical capabilities, causing pain or paralysis, and altering bodily form.
//! These spells affect attributes, senses, movement, and physical structure.
//!
//! # Citations
//!
//! M 36-67 - Body Control college

use crate::{Duration, EnergyCost, ResistanceType, Spell, SpellCollege, SpellPrerequisite, SpellType};
use tracing::{debug, instrument};

/// Returns base energy cost for Body Control spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Body Control spell energy cost");
    match spell {
        Spell::Itch => EnergyCost::Fixed(2),
        Spell::Spasm => EnergyCost::Fixed(3),
        Spell::Pain => EnergyCost::Fixed(3),
        Spell::Paralyze => EnergyCost::Fixed(5),
        Spell::RootedFeet => EnergyCost::Fixed(3),
        Spell::StrikeBlind => EnergyCost::Fixed(4),
        Spell::StrikeDeaf => EnergyCost::Fixed(4),
        Spell::StrikeDumb => EnergyCost::Fixed(4),
        Spell::Haste => EnergyCost::Fixed(2),
        Spell::Slow => EnergyCost::Fixed(3),
        Spell::Strengthen => EnergyCost::Fixed(2),
        Spell::Weaken => EnergyCost::Fixed(2),
        Spell::Grace => EnergyCost::Fixed(2),
        Spell::Clumsiness => EnergyCost::Fixed(2),
        Spell::ResistPain => EnergyCost::Fixed(2),
        Spell::ResistDisease => EnergyCost::Fixed(3),
        Spell::WitherLimb => EnergyCost::Fixed(8),
        Spell::Deathtouch => EnergyCost::PerDie(1),
        Spell::BodyOfStone => EnergyCost::Fixed(5),
        Spell::AlterBody => EnergyCost::Fixed(10),
        Spell::ShapeFlesh => EnergyCost::Fixed(6),
        Spell::Might => EnergyCost::Fixed(2),
        Spell::Vigor => EnergyCost::Fixed(2),
        Spell::Fatigue => EnergyCost::Fixed(2),
        Spell::Stun => EnergyCost::Fixed(4),
        Spell::Tanglefoot => EnergyCost::Fixed(3),
        Spell::TotalParalysis => EnergyCost::Fixed(6),
        Spell::AlterVisage => EnergyCost::Fixed(5),
        Spell::Climbing => EnergyCost::Fixed(2),
        _ => panic!("Invalid spell {:?} for Body Control college", spell),
    }
}

/// Returns casting time in seconds for Body Control spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Body Control spell casting time");
    match spell {
        Spell::Itch => 1,
        Spell::Spasm => 1,
        Spell::Pain => 1,
        Spell::Paralyze => 2,
        Spell::RootedFeet => 1,
        Spell::StrikeBlind => 1,
        Spell::StrikeDeaf => 1,
        Spell::StrikeDumb => 1,
        Spell::Haste => 2,
        Spell::Slow => 1,
        Spell::Strengthen => 2,
        Spell::Weaken => 1,
        Spell::Grace => 2,
        Spell::Clumsiness => 1,
        Spell::ResistPain => 1,
        Spell::ResistDisease => 1,
        Spell::WitherLimb => 3,
        Spell::Deathtouch => 1,
        Spell::BodyOfStone => 3,
        Spell::AlterBody => 5,
        Spell::ShapeFlesh => 3,
        Spell::Might => 2,
        Spell::Vigor => 2,
        Spell::Fatigue => 1,
        Spell::Stun => 1,
        Spell::Tanglefoot => 1,
        Spell::TotalParalysis => 3,
        Spell::AlterVisage => 3,
        Spell::Climbing => 2,
        _ => panic!("Invalid spell {:?} for Body Control college", spell),
    }
}

/// Returns duration for Body Control spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Body Control spell duration");
    match spell {
        Spell::Itch => Duration::Minutes(1),
        Spell::Spasm => Duration::Minutes(1),
        Spell::Pain => Duration::Minutes(1),
        Spell::Paralyze => Duration::Minutes(1),
        Spell::RootedFeet => Duration::Minutes(1),
        Spell::StrikeBlind => Duration::Minutes(1),
        Spell::StrikeDeaf => Duration::Minutes(1),
        Spell::StrikeDumb => Duration::Minutes(1),
        Spell::Haste => Duration::Minutes(1),
        Spell::Slow => Duration::Minutes(1),
        Spell::Strengthen => Duration::Minutes(1),
        Spell::Weaken => Duration::Minutes(1),
        Spell::Grace => Duration::Minutes(1),
        Spell::Clumsiness => Duration::Minutes(1),
        Spell::ResistPain => Duration::Minutes(10),
        Spell::ResistDisease => Duration::Hours(1),
        Spell::WitherLimb => Duration::Permanent,
        Spell::Deathtouch => Duration::Instant,
        Spell::BodyOfStone => Duration::Minutes(1),
        Spell::AlterBody => Duration::Permanent,
        Spell::ShapeFlesh => Duration::Minutes(1),
        Spell::Might => Duration::Minutes(1),
        Spell::Vigor => Duration::Minutes(1),
        Spell::Fatigue => Duration::Minutes(1),
        Spell::Stun => Duration::Minutes(1),
        Spell::Tanglefoot => Duration::Minutes(1),
        Spell::TotalParalysis => Duration::Minutes(1),
        Spell::AlterVisage => Duration::Permanent,
        Spell::Climbing => Duration::Minutes(1),
        _ => panic!("Invalid spell {:?} for Body Control college", spell),
    }
}

/// Returns prerequisites for Body Control spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Body Control spell prerequisites");
    match spell {
        Spell::Itch => vec![SpellPrerequisite::Magery(0)],
        Spell::Spasm => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::Itch),
        ],
        Spell::Pain => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Spasm),
        ],
        Spell::Paralyze => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Pain),
        ],
        Spell::RootedFeet => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 4),
        ],
        Spell::StrikeBlind => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 6),
        ],
        Spell::StrikeDeaf => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 6),
        ],
        Spell::StrikeDumb => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 6),
        ],
        Spell::Haste => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 4),
        ],
        Spell::Slow => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Haste),
        ],
        Spell::Strengthen => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 4),
        ],
        Spell::Weaken => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Strengthen),
        ],
        Spell::Grace => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 4),
        ],
        Spell::Clumsiness => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Grace),
        ],
        Spell::ResistPain => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::Pain),
        ],
        Spell::ResistDisease => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 4),
        ],
        Spell::WitherLimb => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 8),
        ],
        Spell::Deathtouch => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 10),
        ],
        Spell::BodyOfStone => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 6),
        ],
        Spell::AlterBody => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 10),
        ],
        Spell::ShapeFlesh => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 8),
        ],
        Spell::Might => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Strengthen),
        ],
        Spell::Vigor => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 6),
        ],
        Spell::Fatigue => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 4),
        ],
        Spell::Stun => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Pain),
        ],
        Spell::Tanglefoot => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Clumsiness),
        ],
        Spell::TotalParalysis => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Paralyze),
        ],
        Spell::AlterVisage => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 4),
        ],
        Spell::Climbing => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::SpellsInCollege(SpellCollege::BodyControl, 4),
        ],
        _ => panic!("Invalid spell {:?} for Body Control college", spell),
    }
}

/// Returns spell type for Body Control spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Body Control spell type");
    match spell {
        Spell::Itch => SpellType::Regular,
        Spell::Spasm => SpellType::Regular,
        Spell::Pain => SpellType::Regular,
        Spell::Paralyze => SpellType::Regular,
        Spell::RootedFeet => SpellType::Regular,
        Spell::StrikeBlind => SpellType::Regular,
        Spell::StrikeDeaf => SpellType::Regular,
        Spell::StrikeDumb => SpellType::Regular,
        Spell::Haste => SpellType::Regular,
        Spell::Slow => SpellType::Regular,
        Spell::Strengthen => SpellType::Regular,
        Spell::Weaken => SpellType::Regular,
        Spell::Grace => SpellType::Regular,
        Spell::Clumsiness => SpellType::Regular,
        Spell::ResistPain => SpellType::Regular,
        Spell::ResistDisease => SpellType::Regular,
        Spell::WitherLimb => SpellType::Regular,
        Spell::Deathtouch => SpellType::Melee,
        Spell::BodyOfStone => SpellType::Regular,
        Spell::AlterBody => SpellType::Regular,
        Spell::ShapeFlesh => SpellType::Regular,
        Spell::Might => SpellType::Regular,
        Spell::Vigor => SpellType::Regular,
        Spell::Fatigue => SpellType::Regular,
        Spell::Stun => SpellType::Regular,
        Spell::Tanglefoot => SpellType::Regular,
        Spell::TotalParalysis => SpellType::Regular,
        Spell::AlterVisage => SpellType::Regular,
        Spell::Climbing => SpellType::Regular,
        _ => panic!("Invalid spell {:?} for Body Control college", spell),
    }
}

/// Returns resistance type for Body Control spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Body Control spell resistance");
    match spell {
        Spell::Itch => Some(ResistanceType::HT),
        Spell::Spasm => Some(ResistanceType::HT),
        Spell::Pain => Some(ResistanceType::HT),
        Spell::Paralyze => Some(ResistanceType::HT),
        Spell::RootedFeet => Some(ResistanceType::HT),
        Spell::StrikeBlind => Some(ResistanceType::HT),
        Spell::StrikeDeaf => Some(ResistanceType::HT),
        Spell::StrikeDumb => Some(ResistanceType::HT),
        Spell::Haste => None,
        Spell::Slow => Some(ResistanceType::HT),
        Spell::Strengthen => None,
        Spell::Weaken => Some(ResistanceType::HT),
        Spell::Grace => None,
        Spell::Clumsiness => Some(ResistanceType::HT),
        Spell::ResistPain => None,
        Spell::ResistDisease => None,
        Spell::WitherLimb => Some(ResistanceType::HT),
        Spell::Deathtouch => Some(ResistanceType::HT),
        Spell::BodyOfStone => None,
        Spell::AlterBody => Some(ResistanceType::HT),
        Spell::ShapeFlesh => Some(ResistanceType::HT),
        Spell::Might => None,
        Spell::Vigor => None,
        Spell::Fatigue => Some(ResistanceType::HT),
        Spell::Stun => Some(ResistanceType::HT),
        Spell::Tanglefoot => Some(ResistanceType::HT),
        Spell::TotalParalysis => Some(ResistanceType::HT),
        Spell::AlterVisage => Some(ResistanceType::HT),
        Spell::Climbing => None,
        _ => panic!("Invalid spell {:?} for Body Control college", spell),
    }
}

/// Returns spell reference for Body Control spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Body Control spell reference");
    match spell {
        Spell::Itch => "M59",
        Spell::Spasm => "M65",
        Spell::Pain => "M61",
        Spell::Paralyze => "M61",
        Spell::RootedFeet => "M63",
        Spell::StrikeBlind => "M65",
        Spell::StrikeDeaf => "M65",
        Spell::StrikeDumb => "M65",
        Spell::Haste => "M58",
        Spell::Slow => "M64",
        Spell::Strengthen => "M66",
        Spell::Weaken => "M67",
        Spell::Grace => "M58",
        Spell::Clumsiness => "M51",
        Spell::ResistPain => "M63",
        Spell::ResistDisease => "M62",
        Spell::WitherLimb => "M67",
        Spell::Deathtouch => "M54",
        Spell::BodyOfStone => "M49",
        Spell::AlterBody => "M36",
        Spell::ShapeFlesh => "M64",
        Spell::Might => "M60",
        Spell::Vigor => "M66",
        Spell::Fatigue => "M56",
        Spell::Stun => "M65",
        Spell::Tanglefoot => "M66",
        Spell::TotalParalysis => "M67",
        Spell::AlterVisage => "M41",
        Spell::Climbing => "M50",
        _ => panic!("Invalid spell {:?} for Body Control college", spell),
    }
}
