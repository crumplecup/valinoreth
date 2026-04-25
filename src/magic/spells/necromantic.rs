//! Necromantic college spells.
//!
//! # GURPS Rules
//!
//! Necromantic spells manipulate life force, death, and the undead.
//! They include spells for animating corpses, summoning spirits,
//! draining vitality, and controlling undead creatures.
//!
//! # Citations
//!
//! M 149-161 - Necromantic college

use crate::{Duration, EnergyCost, ResistanceType, Spell, SpellPrerequisite, SpellType};
use tracing::{debug, instrument};

/// Returns base energy cost for Necromantic spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Necromantic spell energy cost");
    match spell {
        Spell::DeathVision => EnergyCost::Fixed(2),
        Spell::Zombie => EnergyCost::Fixed(8),
        Spell::Skeleton => EnergyCost::Fixed(6),
        Spell::ControlZombie => EnergyCost::Fixed(3),
        Spell::TurnZombie => EnergyCost::Fixed(2),
        Spell::PermanentZombie => EnergyCost::Fixed(20),
        Spell::SummonSpirit => EnergyCost::Fixed(4),
        Spell::ControlSpirit => EnergyCost::Fixed(3),
        Spell::BanishSpirit => EnergyCost::Fixed(3),
        Spell::SpeakWithDead => EnergyCost::Fixed(4),
        Spell::SpiritVision => EnergyCost::Fixed(2),
        Spell::PreventResurrection => EnergyCost::Fixed(5),
        Spell::StealHealth => EnergyCost::Fixed(3),
        Spell::AgeSpell => EnergyCost::Fixed(5),
        Spell::Youth => EnergyCost::Fixed(10),
        Spell::PreserveCorpse => EnergyCost::Fixed(2),
        Spell::Pestilence => EnergyCost::Fixed(4),
        Spell::LichForm => EnergyCost::Fixed(20),
        Spell::SummonShade => EnergyCost::Fixed(5),
        Spell::FearOfDeath => EnergyCost::Fixed(2),
        Spell::SenseDeath => EnergyCost::Fixed(2),
        Spell::DrainVitality => EnergyCost::Fixed(4),
        Spell::RestoreVitality => EnergyCost::Fixed(5),
        Spell::CreateGhost => EnergyCost::Fixed(10),
        Spell::BindSpirit => EnergyCost::Fixed(4),
        Spell::FreeSpirit => EnergyCost::Fixed(3),
        Spell::CreateVampire => EnergyCost::Fixed(30),
        Spell::FinalDeath => EnergyCost::Fixed(10),
        Spell::WardAgainstUndead => EnergyCost::Fixed(3),
        Spell::NegativeEnergy => EnergyCost::Fixed(2),
        Spell::PositiveEnergy => EnergyCost::Fixed(3),
        Spell::SenseSoul => EnergyCost::Fixed(2),
        Spell::SoulTear => EnergyCost::Fixed(15),
        Spell::RestoreSoul => EnergyCost::Fixed(20),
        Spell::NecromanticBlast => EnergyCost::Fixed(3),
        Spell::Wither => EnergyCost::Fixed(4),
        Spell::RestoreBody => EnergyCost::Fixed(5),
        Spell::AnimateDead => EnergyCost::Fixed(10),
        _ => panic!("Invalid spell {:?} for Necromantic college", spell),
    }
}

/// Returns casting time in seconds for Necromantic spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Necromantic spell casting time");
    match spell {
        Spell::DeathVision => 2,
        Spell::Zombie => 5,
        Spell::Skeleton => 4,
        Spell::ControlZombie => 2,
        Spell::TurnZombie => 1,
        Spell::PermanentZombie => 10,
        Spell::SummonSpirit => 3,
        Spell::ControlSpirit => 2,
        Spell::BanishSpirit => 2,
        Spell::SpeakWithDead => 3,
        Spell::SpiritVision => 2,
        Spell::PreventResurrection => 3,
        Spell::StealHealth => 2,
        Spell::AgeSpell => 3,
        Spell::Youth => 5,
        Spell::PreserveCorpse => 2,
        Spell::Pestilence => 3,
        Spell::LichForm => 10,
        Spell::SummonShade => 4,
        Spell::FearOfDeath => 1,
        Spell::SenseDeath => 2,
        Spell::DrainVitality => 3,
        Spell::RestoreVitality => 3,
        Spell::CreateGhost => 5,
        Spell::BindSpirit => 3,
        Spell::FreeSpirit => 2,
        Spell::CreateVampire => 10,
        Spell::FinalDeath => 5,
        Spell::WardAgainstUndead => 2,
        Spell::NegativeEnergy => 1,
        Spell::PositiveEnergy => 2,
        Spell::SenseSoul => 2,
        Spell::SoulTear => 5,
        Spell::RestoreSoul => 10,
        Spell::NecromanticBlast => 2,
        Spell::Wither => 3,
        Spell::RestoreBody => 4,
        Spell::AnimateDead => 5,
        _ => panic!("Invalid spell {:?} for Necromantic college", spell),
    }
}

/// Returns duration for Necromantic spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Necromantic spell duration");
    match spell {
        Spell::DeathVision => Duration::Minutes(1),
        Spell::Zombie => Duration::Concentration,
        Spell::Skeleton => Duration::Concentration,
        Spell::ControlZombie => Duration::Minutes(10),
        Spell::TurnZombie => Duration::Instant,
        Spell::PermanentZombie => Duration::Permanent,
        Spell::SummonSpirit => Duration::Minutes(10),
        Spell::ControlSpirit => Duration::Minutes(10),
        Spell::BanishSpirit => Duration::Instant,
        Spell::SpeakWithDead => Duration::Minutes(10),
        Spell::SpiritVision => Duration::Minutes(10),
        Spell::PreventResurrection => Duration::Permanent,
        Spell::StealHealth => Duration::Instant,
        Spell::AgeSpell => Duration::Permanent,
        Spell::Youth => Duration::Permanent,
        Spell::PreserveCorpse => Duration::Permanent,
        Spell::Pestilence => Duration::Permanent,
        Spell::LichForm => Duration::Hours(1),
        Spell::SummonShade => Duration::Minutes(10),
        Spell::FearOfDeath => Duration::Minutes(1),
        Spell::SenseDeath => Duration::Concentration,
        Spell::DrainVitality => Duration::Instant,
        Spell::RestoreVitality => Duration::Instant,
        Spell::CreateGhost => Duration::Permanent,
        Spell::BindSpirit => Duration::Permanent,
        Spell::FreeSpirit => Duration::Instant,
        Spell::CreateVampire => Duration::Permanent,
        Spell::FinalDeath => Duration::Instant,
        Spell::WardAgainstUndead => Duration::Minutes(10),
        Spell::NegativeEnergy => Duration::Instant,
        Spell::PositiveEnergy => Duration::Instant,
        Spell::SenseSoul => Duration::Concentration,
        Spell::SoulTear => Duration::Instant,
        Spell::RestoreSoul => Duration::Instant,
        Spell::NecromanticBlast => Duration::Instant,
        Spell::Wither => Duration::Permanent,
        Spell::RestoreBody => Duration::Instant,
        Spell::AnimateDead => Duration::Concentration,
        _ => panic!("Invalid spell {:?} for Necromantic college", spell),
    }
}

/// Returns prerequisites for Necromantic spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Necromantic spell prerequisites");
    match spell {
        Spell::DeathVision => vec![SpellPrerequisite::Magery(0)],
        Spell::Zombie => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::DeathVision),
        ],
        Spell::Skeleton => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Zombie),
        ],
        Spell::ControlZombie => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Zombie),
        ],
        Spell::TurnZombie => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::ControlZombie),
        ],
        Spell::PermanentZombie => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::Zombie),
        ],
        Spell::SummonSpirit => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::DeathVision),
        ],
        Spell::ControlSpirit => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::SummonSpirit),
        ],
        Spell::BanishSpirit => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::SummonSpirit),
        ],
        Spell::SpeakWithDead => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::SummonSpirit),
        ],
        Spell::SpiritVision => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::DeathVision),
        ],
        Spell::PreventResurrection => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::DeathVision),
        ],
        Spell::StealHealth => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::DeathVision),
        ],
        Spell::AgeSpell => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::DeathVision),
        ],
        Spell::Youth => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::AgeSpell),
        ],
        Spell::PreserveCorpse => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::DeathVision),
        ],
        Spell::Pestilence => vec![SpellPrerequisite::Magery(2)],
        Spell::LichForm => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::Zombie),
        ],
        Spell::SummonShade => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::SummonSpirit),
        ],
        Spell::FearOfDeath => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::DeathVision),
        ],
        Spell::SenseDeath => vec![SpellPrerequisite::Magery(0)],
        Spell::DrainVitality => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::StealHealth),
        ],
        Spell::RestoreVitality => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::DrainVitality),
        ],
        Spell::CreateGhost => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::SummonSpirit),
        ],
        Spell::BindSpirit => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::ControlSpirit),
        ],
        Spell::FreeSpirit => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::BindSpirit),
        ],
        Spell::CreateVampire => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::PermanentZombie),
        ],
        Spell::FinalDeath => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::TurnZombie),
        ],
        Spell::WardAgainstUndead => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::TurnZombie),
        ],
        Spell::NegativeEnergy => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::DeathVision),
        ],
        Spell::PositiveEnergy => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::NegativeEnergy),
        ],
        Spell::SenseSoul => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::SpiritVision),
        ],
        Spell::SoulTear => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::SenseSoul),
        ],
        Spell::RestoreSoul => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::SoulTear),
        ],
        Spell::NecromanticBlast => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::NegativeEnergy),
        ],
        Spell::Wither => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Decay),
        ],
        Spell::RestoreBody => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::Wither),
        ],
        Spell::AnimateDead => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::Zombie),
        ],
        _ => panic!("Invalid spell {:?} for Necromantic college", spell),
    }
}

/// Returns spell type for Necromantic spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Necromantic spell type");
    match spell {
        Spell::DeathVision => SpellType::Information,
        Spell::Zombie => SpellType::Regular,
        Spell::Skeleton => SpellType::Regular,
        Spell::ControlZombie => SpellType::Regular,
        Spell::TurnZombie => SpellType::Area,
        Spell::PermanentZombie => SpellType::Regular,
        Spell::SummonSpirit => SpellType::Regular,
        Spell::ControlSpirit => SpellType::Regular,
        Spell::BanishSpirit => SpellType::Regular,
        Spell::SpeakWithDead => SpellType::Regular,
        Spell::SpiritVision => SpellType::Regular,
        Spell::PreventResurrection => SpellType::Regular,
        Spell::StealHealth => SpellType::Regular,
        Spell::AgeSpell => SpellType::Regular,
        Spell::Youth => SpellType::Regular,
        Spell::PreserveCorpse => SpellType::Regular,
        Spell::Pestilence => SpellType::Regular,
        Spell::LichForm => SpellType::Regular,
        Spell::SummonShade => SpellType::Regular,
        Spell::FearOfDeath => SpellType::Regular,
        Spell::SenseDeath => SpellType::Information,
        Spell::DrainVitality => SpellType::Regular,
        Spell::RestoreVitality => SpellType::Regular,
        Spell::CreateGhost => SpellType::Regular,
        Spell::BindSpirit => SpellType::Regular,
        Spell::FreeSpirit => SpellType::Regular,
        Spell::CreateVampire => SpellType::Regular,
        Spell::FinalDeath => SpellType::Regular,
        Spell::WardAgainstUndead => SpellType::Area,
        Spell::NegativeEnergy => SpellType::Regular,
        Spell::PositiveEnergy => SpellType::Regular,
        Spell::SenseSoul => SpellType::Information,
        Spell::SoulTear => SpellType::Regular,
        Spell::RestoreSoul => SpellType::Regular,
        Spell::NecromanticBlast => SpellType::Regular,
        Spell::Wither => SpellType::Regular,
        Spell::RestoreBody => SpellType::Regular,
        Spell::AnimateDead => SpellType::Area,
        _ => panic!("Invalid spell {:?} for Necromantic college", spell),
    }
}

/// Returns resistance type for Necromantic spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Necromantic spell resistance");
    match spell {
        Spell::DeathVision => None,
        Spell::Zombie => None,
        Spell::Skeleton => None,
        Spell::ControlZombie => Some(ResistanceType::Will),
        Spell::TurnZombie => Some(ResistanceType::Will),
        Spell::PermanentZombie => None,
        Spell::SummonSpirit => Some(ResistanceType::Will),
        Spell::ControlSpirit => Some(ResistanceType::Will),
        Spell::BanishSpirit => Some(ResistanceType::Will),
        Spell::SpeakWithDead => None,
        Spell::SpiritVision => None,
        Spell::PreventResurrection => None,
        Spell::StealHealth => Some(ResistanceType::HT),
        Spell::AgeSpell => Some(ResistanceType::HT),
        Spell::Youth => None,
        Spell::PreserveCorpse => None,
        Spell::Pestilence => Some(ResistanceType::HT),
        Spell::LichForm => None,
        Spell::SummonShade => Some(ResistanceType::Will),
        Spell::FearOfDeath => Some(ResistanceType::Will),
        Spell::SenseDeath => None,
        Spell::DrainVitality => Some(ResistanceType::HT),
        Spell::RestoreVitality => None,
        Spell::CreateGhost => None,
        Spell::BindSpirit => Some(ResistanceType::Will),
        Spell::FreeSpirit => None,
        Spell::CreateVampire => None,
        Spell::FinalDeath => Some(ResistanceType::Will),
        Spell::WardAgainstUndead => None,
        Spell::NegativeEnergy => None,
        Spell::PositiveEnergy => None,
        Spell::SenseSoul => None,
        Spell::SoulTear => Some(ResistanceType::Will),
        Spell::RestoreSoul => None,
        Spell::NecromanticBlast => Some(ResistanceType::HT),
        Spell::Wither => Some(ResistanceType::HT),
        Spell::RestoreBody => None,
        Spell::AnimateDead => None,
        _ => panic!("Invalid spell {:?} for Necromantic college", spell),
    }
}

/// Returns page reference for Necromantic spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Necromantic spell reference");
    match spell {
        Spell::DeathVision => "M149",
        Spell::Zombie => "M149",
        Spell::Skeleton => "M159",
        Spell::ControlZombie => "M151",
        Spell::TurnZombie => "M161",
        Spell::PermanentZombie => "M149",
        Spell::SummonSpirit => "M160",
        Spell::ControlSpirit => "M160",
        Spell::BanishSpirit => "M160",
        Spell::SpeakWithDead => "M160",
        Spell::SpiritVision => "M159",
        Spell::PreventResurrection => "M157",
        Spell::StealHealth => "M151",
        Spell::AgeSpell => "M149",
        Spell::Youth => "M161",
        Spell::PreserveCorpse => "M157",
        Spell::Pestilence => "M149",
        Spell::LichForm => "M150",
        Spell::SummonShade => "M159",
        Spell::FearOfDeath => "M150",
        Spell::SenseDeath => "M152",
        Spell::DrainVitality => "M151",
        Spell::RestoreVitality => "M158",
        Spell::CreateGhost => "M150",
        Spell::BindSpirit => "M150",
        Spell::FreeSpirit => "M150",
        Spell::CreateVampire => "M158",
        Spell::FinalDeath => "M149",
        Spell::WardAgainstUndead => "M157",
        Spell::NegativeEnergy => "M149",
        Spell::PositiveEnergy => "M157",
        Spell::SenseSoul => "M159",
        Spell::SoulTear => "M160",
        Spell::RestoreSoul => "M158",
        Spell::NecromanticBlast => "M152",
        Spell::Wither => "M161",
        Spell::RestoreBody => "M158",
        Spell::AnimateDead => "M152",
        _ => panic!("Invalid spell {:?} for Necromantic college", spell),
    }
}
