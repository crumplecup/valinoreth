//! Knowledge college spells.
//!
//! # GURPS Rules
//!
//! Knowledge spells provide information through divination, detection,
//! and analysis of magic, objects, and history.
//!
//! # Citations
//!
//! M 106-113 - Knowledge college

use crate::{Duration, EnergyCost, ResistanceType, Spell, SpellCollege, SpellPrerequisite, SpellType};
use tracing::{debug, instrument};

/// Returns base energy cost for Knowledge spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Knowledge spell energy cost");
    match spell {
        Spell::DetectMagic => EnergyCost::Fixed(2),
        Spell::AnalyzeMagic => EnergyCost::Fixed(8),
        Spell::IdentifySpell => EnergyCost::Fixed(2),
        Spell::Seeker => EnergyCost::Fixed(3),
        Spell::SeekAir => EnergyCost::Fixed(1),
        Spell::SeekEarth => EnergyCost::Fixed(1),
        Spell::SeekFire => EnergyCost::Fixed(1),
        Spell::SeekWater => EnergyCost::Fixed(1),
        Spell::SeekPlant => EnergyCost::Fixed(1),
        Spell::SeekFood => EnergyCost::Fixed(1),
        Spell::SeekMachine => EnergyCost::Fixed(1),
        Spell::History => EnergyCost::Fixed(4),
        Spell::Wisdom => EnergyCost::Fixed(8),
        Spell::AncientHistory => EnergyCost::Fixed(5),
        Spell::RecoverMemory => EnergyCost::Fixed(3),
        Spell::Divination => EnergyCost::Fixed(4),
        Spell::Pathfinder => EnergyCost::Fixed(3),
        Spell::GlassWall => EnergyCost::Fixed(2),
        Spell::Trace => EnergyCost::Fixed(2),
        Spell::Aura => EnergyCost::Fixed(2),
        _ => panic!("Invalid spell {:?} for Knowledge college", spell),
    }
}

/// Returns casting time for Knowledge spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Knowledge spell casting time");
    match spell {
        Spell::DetectMagic => 1,
        Spell::AnalyzeMagic => 1,
        Spell::IdentifySpell => 1,
        Spell::Seeker => 2,
        Spell::SeekAir => 1,
        Spell::SeekEarth => 1,
        Spell::SeekFire => 1,
        Spell::SeekWater => 1,
        Spell::SeekPlant => 1,
        Spell::SeekFood => 1,
        Spell::SeekMachine => 1,
        Spell::History => 3,
        Spell::Wisdom => 5,
        Spell::AncientHistory => 4,
        Spell::RecoverMemory => 2,
        Spell::Divination => 5,
        Spell::Pathfinder => 2,
        Spell::GlassWall => 1,
        Spell::Trace => 2,
        Spell::Aura => 2,
        _ => panic!("Invalid spell {:?} for Knowledge college", spell),
    }
}

/// Returns duration for Knowledge spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Knowledge spell duration");
    match spell {
        Spell::DetectMagic => Duration::Concentration,
        Spell::AnalyzeMagic => Duration::Instant,
        Spell::IdentifySpell => Duration::Instant,
        Spell::Seeker => Duration::Instant,
        Spell::SeekAir => Duration::Concentration,
        Spell::SeekEarth => Duration::Concentration,
        Spell::SeekFire => Duration::Concentration,
        Spell::SeekWater => Duration::Concentration,
        Spell::SeekPlant => Duration::Concentration,
        Spell::SeekFood => Duration::Concentration,
        Spell::SeekMachine => Duration::Concentration,
        Spell::History => Duration::Instant,
        Spell::Wisdom => Duration::Instant,
        Spell::AncientHistory => Duration::Instant,
        Spell::RecoverMemory => Duration::Instant,
        Spell::Divination => Duration::Instant,
        Spell::Pathfinder => Duration::Instant,
        Spell::GlassWall => Duration::Minutes(1),
        Spell::Trace => Duration::Concentration,
        Spell::Aura => Duration::Instant,
        _ => panic!("Invalid spell {:?} for Knowledge college", spell),
    }
}

/// Returns prerequisites for Knowledge spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Knowledge spell prerequisites");
    match spell {
        Spell::DetectMagic => vec![SpellPrerequisite::Magery(0)],
        Spell::AnalyzeMagic => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Knowledge, 6),
        ],
        Spell::IdentifySpell => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::DetectMagic),
        ],
        Spell::Seeker => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::DetectMagic),
        ],
        Spell::SeekAir => vec![SpellPrerequisite::Magery(0)],
        Spell::SeekEarth => vec![SpellPrerequisite::Magery(0)],
        Spell::SeekFire => vec![SpellPrerequisite::Magery(0)],
        Spell::SeekWater => vec![SpellPrerequisite::Magery(0)],
        Spell::SeekPlant => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::SeekEarth),
        ],
        Spell::SeekFood => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::SeekWater),
        ],
        Spell::SeekMachine => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::SeekEarth),
        ],
        Spell::History => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Knowledge, 4),
        ],
        Spell::Wisdom => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Knowledge, 12),
            SpellPrerequisite::IQ(14),
        ],
        Spell::AncientHistory => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::History),
        ],
        Spell::RecoverMemory => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Knowledge, 4),
        ],
        Spell::Divination => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Knowledge, 6),
        ],
        Spell::Pathfinder => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Seeker),
        ],
        Spell::GlassWall => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Knowledge, 4),
        ],
        Spell::Trace => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Seeker),
        ],
        Spell::Aura => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::DetectMagic),
        ],
        _ => panic!("Invalid spell {:?} for Knowledge college", spell),
    }
}

/// Returns spell type for Knowledge spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Knowledge spell type");
    match spell {
        Spell::DetectMagic => SpellType::Information,
        Spell::AnalyzeMagic => SpellType::Information,
        Spell::IdentifySpell => SpellType::Information,
        Spell::Seeker => SpellType::Information,
        Spell::SeekAir => SpellType::Information,
        Spell::SeekEarth => SpellType::Information,
        Spell::SeekFire => SpellType::Information,
        Spell::SeekWater => SpellType::Information,
        Spell::SeekPlant => SpellType::Information,
        Spell::SeekFood => SpellType::Information,
        Spell::SeekMachine => SpellType::Information,
        Spell::History => SpellType::Information,
        Spell::Wisdom => SpellType::Information,
        Spell::AncientHistory => SpellType::Information,
        Spell::RecoverMemory => SpellType::Regular,
        Spell::Divination => SpellType::Information,
        Spell::Pathfinder => SpellType::Information,
        Spell::GlassWall => SpellType::Area,
        Spell::Trace => SpellType::Information,
        Spell::Aura => SpellType::Information,
        _ => panic!("Invalid spell {:?} for Knowledge college", spell),
    }
}

/// Returns resistance type for Knowledge spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Knowledge spell resistance");
    match spell {
        Spell::DetectMagic => None,
        Spell::AnalyzeMagic => None,
        Spell::IdentifySpell => None,
        Spell::Seeker => None,
        Spell::SeekAir => None,
        Spell::SeekEarth => None,
        Spell::SeekFire => None,
        Spell::SeekWater => None,
        Spell::SeekPlant => None,
        Spell::SeekFood => None,
        Spell::SeekMachine => None,
        Spell::History => None,
        Spell::Wisdom => None,
        Spell::AncientHistory => None,
        Spell::RecoverMemory => Some(ResistanceType::IQ),
        Spell::Divination => None,
        Spell::Pathfinder => None,
        Spell::GlassWall => None,
        Spell::Trace => None,
        Spell::Aura => None,
        _ => panic!("Invalid spell {:?} for Knowledge college", spell),
    }
}

/// Returns page reference for Knowledge spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Knowledge spell reference");
    match spell {
        Spell::DetectMagic => "M107",
        Spell::AnalyzeMagic => "M106",
        Spell::IdentifySpell => "M109",
        Spell::Seeker => "M112",
        Spell::SeekAir => "M111",
        Spell::SeekEarth => "M111",
        Spell::SeekFire => "M112",
        Spell::SeekWater => "M112",
        Spell::SeekPlant => "M112",
        Spell::SeekFood => "M111",
        Spell::SeekMachine => "M111",
        Spell::History => "M108",
        Spell::Wisdom => "M113",
        Spell::AncientHistory => "M106",
        Spell::RecoverMemory => "M110",
        Spell::Divination => "M107",
        Spell::Pathfinder => "M110",
        Spell::GlassWall => "M108",
        Spell::Trace => "M113",
        Spell::Aura => "M107",
        _ => panic!("Invalid spell {:?} for Knowledge college", spell),
    }
}
