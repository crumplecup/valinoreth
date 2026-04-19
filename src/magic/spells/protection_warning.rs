//! Protection & Warning college spells.
//!
//! # GURPS Rules
//!
//! Protection spells defend against harm through shields, deflection,
//! and resistance. Warning spells alert to danger.
//!
//! # Citations
//!
//! M 162-185 - Protection & Warning college

use crate::{Duration, EnergyCost, ResistanceType, Spell, SpellPrerequisite, SpellType};
use tracing::{debug, instrument};

/// Returns base energy cost for Protection & Warning spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Protection & Warning spell energy cost");
    match spell {
        Spell::Shield => EnergyCost::Fixed(2),
        Spell::Deflect => EnergyCost::Fixed(2),
        Spell::Reflection => EnergyCost::Fixed(3),
        Spell::Absorb => EnergyCost::Fixed(4),
        Spell::Warn => EnergyCost::Fixed(2),
        Spell::Sense => EnergyCost::Fixed(1),
        Spell::ResistFireProtection => EnergyCost::Fixed(2),
        Spell::ResistColdProtection => EnergyCost::Fixed(2),
        Spell::ResistLightning => EnergyCost::Fixed(2),
        Spell::ResistAcid => EnergyCost::Fixed(2),
        Spell::ArmorEnchantment => EnergyCost::Fixed(4),
        Spell::AnchorSpell => EnergyCost::Fixed(3),
        Spell::WatchdogSpell => EnergyCost::Fixed(3),
        _ => panic!("Invalid spell {:?} for Protection & Warning college", spell),
    }
}

/// Returns casting time for Protection & Warning spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Protection & Warning spell casting time");
    match spell {
        Spell::Shield => 1,
        Spell::Deflect => 1,
        Spell::Reflection => 2,
        Spell::Absorb => 2,
        Spell::Warn => 2,
        Spell::Sense => 1,
        Spell::ResistFireProtection => 2,
        Spell::ResistColdProtection => 2,
        Spell::ResistLightning => 2,
        Spell::ResistAcid => 2,
        Spell::ArmorEnchantment => 3,
        Spell::AnchorSpell => 2,
        Spell::WatchdogSpell => 3,
        _ => panic!("Invalid spell {:?} for Protection & Warning college", spell),
    }
}

/// Returns duration for Protection & Warning spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Protection & Warning spell duration");
    match spell {
        Spell::Shield => Duration::Concentration,
        Spell::Deflect => Duration::Concentration,
        Spell::Reflection => Duration::Concentration,
        Spell::Absorb => Duration::Concentration,
        Spell::Warn => Duration::Hours(1),
        Spell::Sense => Duration::Concentration,
        Spell::ResistFireProtection => Duration::Minutes(1),
        Spell::ResistColdProtection => Duration::Minutes(1),
        Spell::ResistLightning => Duration::Minutes(1),
        Spell::ResistAcid => Duration::Minutes(1),
        Spell::ArmorEnchantment => Duration::Permanent,
        Spell::AnchorSpell => Duration::Minutes(1),
        Spell::WatchdogSpell => Duration::Hours(8),
        _ => panic!("Invalid spell {:?} for Protection & Warning college", spell),
    }
}

/// Returns prerequisites for Protection & Warning spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Protection & Warning spell prerequisites");
    match spell {
        Spell::Shield => vec![SpellPrerequisite::Magery(0)],
        Spell::Deflect => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Shield),
        ],
        Spell::Reflection => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Deflect),
        ],
        Spell::Absorb => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(crate::SpellCollege::ProtectionWarning, 4),
        ],
        Spell::Warn => vec![SpellPrerequisite::Magery(1)],
        Spell::Sense => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Warn),
        ],
        Spell::ResistFireProtection => vec![SpellPrerequisite::Magery(0)],
        Spell::ResistColdProtection => vec![SpellPrerequisite::Magery(0)],
        Spell::ResistLightning => vec![SpellPrerequisite::Magery(0)],
        Spell::ResistAcid => vec![SpellPrerequisite::Magery(0)],
        Spell::ArmorEnchantment => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(crate::SpellCollege::ProtectionWarning, 6),
        ],
        Spell::AnchorSpell => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(crate::SpellCollege::ProtectionWarning, 4),
        ],
        Spell::WatchdogSpell => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Warn),
        ],
        _ => panic!("Invalid spell {:?} for Protection & Warning college", spell),
    }
}

/// Returns spell type for Protection & Warning spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Protection & Warning spell type");
    match spell {
        Spell::Shield => SpellType::Blocking,
        Spell::Deflect => SpellType::Blocking,
        Spell::Reflection => SpellType::Blocking,
        Spell::Absorb => SpellType::Blocking,
        Spell::Warn => SpellType::Information,
        Spell::Sense => SpellType::Information,
        Spell::ResistFireProtection => SpellType::Regular,
        Spell::ResistColdProtection => SpellType::Regular,
        Spell::ResistLightning => SpellType::Regular,
        Spell::ResistAcid => SpellType::Regular,
        Spell::ArmorEnchantment => SpellType::Regular,
        Spell::AnchorSpell => SpellType::Area,
        Spell::WatchdogSpell => SpellType::Area,
        _ => panic!("Invalid spell {:?} for Protection & Warning college", spell),
    }
}

/// Returns resistance type for Protection & Warning spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Protection & Warning spell resistance");
    match spell {
        Spell::Shield => None,
        Spell::Deflect => None,
        Spell::Reflection => None,
        Spell::Absorb => None,
        Spell::Warn => None,
        Spell::Sense => None,
        Spell::ResistFireProtection => None,
        Spell::ResistColdProtection => None,
        Spell::ResistLightning => None,
        Spell::ResistAcid => None,
        Spell::ArmorEnchantment => None,
        Spell::AnchorSpell => None,
        Spell::WatchdogSpell => None,
        _ => panic!("Invalid spell {:?} for Protection & Warning college", spell),
    }
}

/// Returns page reference for Protection & Warning spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Protection & Warning spell reference");
    match spell {
        Spell::Shield => "M182",
        Spell::Deflect => "M168",
        Spell::Reflection => "M177",
        Spell::Absorb => "M163",
        Spell::Warn => "M185",
        Spell::Sense => "M164",
        Spell::ResistFireProtection => "M175",
        Spell::ResistColdProtection => "M175",
        Spell::ResistLightning => "M175",
        Spell::ResistAcid => "M175",
        Spell::ArmorEnchantment => "M163",
        Spell::AnchorSpell => "M163",
        Spell::WatchdogSpell => "M185",
        _ => panic!("Invalid spell {:?} for Protection & Warning college", spell),
    }
}
