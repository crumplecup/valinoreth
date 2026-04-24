//! Enchantment college spells.
//!
//! # GURPS Rules
//!
//! Enchantment spells create permanent magical items, enchant objects with
//! magical properties, and manipulate reality through wishes. These spells
//! are among the most powerful and expensive in the magic system.
//!
//! # Citations
//!
//! M 56-71 - Enchantment college

use crate::{Duration, EnergyCost, ResistanceType, Spell, SpellCollege, SpellPrerequisite, SpellType};
use tracing::{debug, instrument};

/// Returns base energy cost for Enchantment spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Enchantment spell energy cost");
    match spell {
        Spell::Enchant => EnergyCost::Fixed(100),
        Spell::TemporaryEnchantment => EnergyCost::Fixed(2),
        Spell::Hex => EnergyCost::Fixed(10),
        Spell::Scroll => EnergyCost::Fixed(2),
        Spell::Power => EnergyCost::Fixed(50),
        Spell::Powerstone => EnergyCost::Fixed(100),
        Spell::RemoveEnchantment => EnergyCost::Fixed(10),
        Spell::ResistEnchantment => EnergyCost::Fixed(2),
        Spell::Golem => EnergyCost::Fixed(50),
        Spell::Malefice => EnergyCost::Fixed(5),
        Spell::Ensorcel => EnergyCost::Fixed(10),
        Spell::ImpressionBlocker => EnergyCost::Fixed(20),
        Spell::LesserWish => EnergyCost::Fixed(20),
        Spell::Wish => EnergyCost::Fixed(100),
        Spell::GreatWish => EnergyCost::Fixed(500),
        Spell::Manastone => EnergyCost::Fixed(150),
        Spell::Staff => EnergyCost::Fixed(75),
        Spell::Wand => EnergyCost::Fixed(50),
        Spell::CrystalBall => EnergyCost::Fixed(100),
        Spell::SoulStone => EnergyCost::Fixed(200),
        _ => panic!("Invalid spell {:?} for Enchantment college", spell),
    }
}

/// Returns casting time in seconds for Enchantment spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Enchantment spell casting time");
    match spell {
        Spell::Enchant => 86400, // 1 day
        Spell::TemporaryEnchantment => 10,
        Spell::Hex => 300, // 5 minutes
        Spell::Scroll => 60, // 1 minute
        Spell::Power => 3600, // 1 hour
        Spell::Powerstone => 86400, // 1 day
        Spell::RemoveEnchantment => 60,
        Spell::ResistEnchantment => 2,
        Spell::Golem => 28800, // 8 hours
        Spell::Malefice => 300, // 5 minutes
        Spell::Ensorcel => 600, // 10 minutes
        Spell::ImpressionBlocker => 300, // 5 minutes
        Spell::LesserWish => 60,
        Spell::Wish => 600,
        Spell::GreatWish => 3600,
        Spell::Manastone => 172800, // 2 days
        Spell::Staff => 43200, // 12 hours
        Spell::Wand => 21600, // 6 hours
        Spell::CrystalBall => 86400, // 1 day
        Spell::SoulStone => 259200, // 3 days
        _ => panic!("Invalid spell {:?} for Enchantment college", spell),
    }
}

/// Returns duration for Enchantment spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Enchantment spell duration");
    match spell {
        Spell::Enchant => Duration::Permanent,
        Spell::TemporaryEnchantment => Duration::Hours(1),
        Spell::Hex => Duration::Permanent,
        Spell::Scroll => Duration::Permanent,
        Spell::Power => Duration::Permanent,
        Spell::Powerstone => Duration::Permanent,
        Spell::RemoveEnchantment => Duration::Permanent,
        Spell::ResistEnchantment => Duration::Minutes(10),
        Spell::Golem => Duration::Permanent,
        Spell::Malefice => Duration::Permanent,
        Spell::Ensorcel => Duration::Permanent,
        Spell::ImpressionBlocker => Duration::Permanent,
        Spell::LesserWish => Duration::Instant,
        Spell::Wish => Duration::Instant,
        Spell::GreatWish => Duration::Instant,
        Spell::Manastone => Duration::Permanent,
        Spell::Staff => Duration::Permanent,
        Spell::Wand => Duration::Permanent,
        Spell::CrystalBall => Duration::Permanent,
        Spell::SoulStone => Duration::Permanent,
        _ => panic!("Invalid spell {:?} for Enchantment college", spell),
    }
}

/// Returns prerequisites for Enchantment spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Enchantment spell prerequisites");
    match spell {
        Spell::Enchant => vec![SpellPrerequisite::Magery(2)],
        Spell::TemporaryEnchantment => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Enchant),
        ],
        Spell::Hex => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Enchant),
        ],
        Spell::Scroll => vec![SpellPrerequisite::Magery(1)],
        Spell::Power => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Enchant),
        ],
        Spell::Powerstone => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Enchant),
        ],
        Spell::RemoveEnchantment => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Enchant),
        ],
        Spell::ResistEnchantment => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Enchantment, 3),
        ],
        Spell::Golem => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Enchant),
        ],
        Spell::Malefice => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Enchant),
        ],
        Spell::Ensorcel => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Malefice),
        ],
        Spell::ImpressionBlocker => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Enchant),
        ],
        Spell::LesserWish => vec![SpellPrerequisite::Magery(3)],
        Spell::Wish => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::LesserWish),
        ],
        Spell::GreatWish => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::Wish),
        ],
        Spell::Manastone => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::Enchant),
        ],
        Spell::Staff => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Enchant),
        ],
        Spell::Wand => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Enchant),
        ],
        Spell::CrystalBall => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Enchant),
        ],
        Spell::SoulStone => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::Enchant),
        ],
        _ => panic!("Invalid spell {:?} for Enchantment college", spell),
    }
}

/// Returns spell type for Enchantment spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Enchantment spell type");
    match spell {
        Spell::Enchant => SpellType::Regular,
        Spell::TemporaryEnchantment => SpellType::Regular,
        Spell::Hex => SpellType::Regular,
        Spell::Scroll => SpellType::Regular,
        Spell::Power => SpellType::Regular,
        Spell::Powerstone => SpellType::Regular,
        Spell::RemoveEnchantment => SpellType::Regular,
        Spell::ResistEnchantment => SpellType::Regular,
        Spell::Golem => SpellType::Regular,
        Spell::Malefice => SpellType::Regular,
        Spell::Ensorcel => SpellType::Regular,
        Spell::ImpressionBlocker => SpellType::Regular,
        Spell::LesserWish => SpellType::Regular,
        Spell::Wish => SpellType::Regular,
        Spell::GreatWish => SpellType::Regular,
        Spell::Manastone => SpellType::Regular,
        Spell::Staff => SpellType::Regular,
        Spell::Wand => SpellType::Regular,
        Spell::CrystalBall => SpellType::Regular,
        Spell::SoulStone => SpellType::Regular,
        _ => panic!("Invalid spell {:?} for Enchantment college", spell),
    }
}

/// Returns resistance type for Enchantment spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Enchantment spell resistance");
    match spell {
        Spell::Enchant => None,
        Spell::TemporaryEnchantment => None,
        Spell::Hex => Some(ResistanceType::Will),
        Spell::Scroll => None,
        Spell::Power => None,
        Spell::Powerstone => None,
        Spell::RemoveEnchantment => None,
        Spell::ResistEnchantment => None,
        Spell::Golem => None,
        Spell::Malefice => Some(ResistanceType::Will),
        Spell::Ensorcel => Some(ResistanceType::Will),
        Spell::ImpressionBlocker => None,
        Spell::LesserWish => None,
        Spell::Wish => None,
        Spell::GreatWish => None,
        Spell::Manastone => None,
        Spell::Staff => None,
        Spell::Wand => None,
        Spell::CrystalBall => None,
        Spell::SoulStone => Some(ResistanceType::Will),
        _ => panic!("Invalid spell {:?} for Enchantment college", spell),
    }
}

/// Returns page reference for Enchantment spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Enchantment spell reference");
    match spell {
        Spell::Enchant => "M56",
        Spell::TemporaryEnchantment => "M56",
        Spell::Hex => "M57",
        Spell::Scroll => "M57",
        Spell::Power => "M57",
        Spell::Powerstone => "M69",
        Spell::RemoveEnchantment => "M58",
        Spell::ResistEnchantment => "M58",
        Spell::Golem => "M59",
        Spell::Malefice => "M60",
        Spell::Ensorcel => "M60",
        Spell::ImpressionBlocker => "M60",
        Spell::LesserWish => "M61",
        Spell::Wish => "M61",
        Spell::GreatWish => "M61",
        Spell::Manastone => "M70",
        Spell::Staff => "M70",
        Spell::Wand => "M70",
        Spell::CrystalBall => "M71",
        Spell::SoulStone => "M71",
        _ => panic!("Invalid spell {:?} for Enchantment college", spell),
    }
}
