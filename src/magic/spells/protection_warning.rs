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
        Spell::Armor => EnergyCost::Fixed(4), // Variable: 2 per DR point, max DR 5
        Spell::ForceWall => EnergyCost::Fixed(2),
        Spell::ForceDome => EnergyCost::Fixed(3),
        Spell::UtterWall => EnergyCost::Fixed(4),
        Spell::UtterDome => EnergyCost::Fixed(6),
        Spell::SpellWall => EnergyCost::Fixed(3),
        Spell::SpellShield => EnergyCost::Fixed(3),
        Spell::Magelock => EnergyCost::Fixed(2),
        Spell::Alarm => EnergyCost::Fixed(2),
        Spell::MysticMist => EnergyCost::Fixed(3),
        Spell::TeleportShield => EnergyCost::Fixed(4),
        Spell::WeatherDome => EnergyCost::Fixed(5),
        Spell::Block => EnergyCost::Fixed(2),
        Spell::Nightingale => EnergyCost::Fixed(3),
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
        Spell::Armor => 2,
        Spell::ForceWall => 2,
        Spell::ForceDome => 3,
        Spell::UtterWall => 3,
        Spell::UtterDome => 4,
        Spell::SpellWall => 2,
        Spell::SpellShield => 1,
        Spell::Magelock => 2,
        Spell::Alarm => 2,
        Spell::MysticMist => 2,
        Spell::TeleportShield => 2,
        Spell::WeatherDome => 3,
        Spell::Block => 1,
        Spell::Nightingale => 2,
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
        Spell::Armor => Duration::Minutes(1),
        Spell::ForceWall => Duration::Concentration,
        Spell::ForceDome => Duration::Concentration,
        Spell::UtterWall => Duration::Concentration,
        Spell::UtterDome => Duration::Concentration,
        Spell::SpellWall => Duration::Concentration,
        Spell::SpellShield => Duration::Minutes(1),
        Spell::Magelock => Duration::Permanent,
        Spell::Alarm => Duration::Hours(8),
        Spell::MysticMist => Duration::Minutes(10),
        Spell::TeleportShield => Duration::Hours(1),
        Spell::WeatherDome => Duration::Hours(1),
        Spell::Block => Duration::Minutes(1),
        Spell::Nightingale => Duration::Hours(8),
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
        Spell::Armor => vec![SpellPrerequisite::Magery(1)],
        Spell::ForceWall => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Shield),
        ],
        Spell::ForceDome => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::ForceWall),
        ],
        Spell::UtterWall => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::ForceWall),
            SpellPrerequisite::Spell(Spell::SpellWall),
        ],
        Spell::UtterDome => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::ForceDome),
            SpellPrerequisite::Spell(Spell::SpellWall),
        ],
        Spell::SpellWall => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(crate::SpellCollege::ProtectionWarning, 4),
        ],
        Spell::SpellShield => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Shield),
        ],
        Spell::Magelock => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Armor),
        ],
        Spell::Alarm => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Warn),
        ],
        Spell::MysticMist => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(crate::SpellCollege::ProtectionWarning, 4),
        ],
        Spell::TeleportShield => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(crate::SpellCollege::ProtectionWarning, 6),
        ],
        Spell::WeatherDome => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::ForceDome),
        ],
        Spell::Block => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Shield),
        ],
        Spell::Nightingale => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(crate::SpellCollege::ProtectionWarning, 4),
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
        Spell::Armor => SpellType::Regular,
        Spell::ForceWall => SpellType::Area,
        Spell::ForceDome => SpellType::Area,
        Spell::UtterWall => SpellType::Area,
        Spell::UtterDome => SpellType::Area,
        Spell::SpellWall => SpellType::Area,
        Spell::SpellShield => SpellType::Blocking,
        Spell::Magelock => SpellType::Regular,
        Spell::Alarm => SpellType::Area,
        Spell::MysticMist => SpellType::Area,
        Spell::TeleportShield => SpellType::Area,
        Spell::WeatherDome => SpellType::Area,
        Spell::Block => SpellType::Regular,
        Spell::Nightingale => SpellType::Area,
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
        Spell::Armor => None,
        Spell::ForceWall => None,
        Spell::ForceDome => None,
        Spell::UtterWall => None,
        Spell::UtterDome => None,
        Spell::SpellWall => None,
        Spell::SpellShield => None,
        Spell::Magelock => None,
        Spell::Alarm => None,
        Spell::MysticMist => Some(ResistanceType::IQ), // Confusion effect
        Spell::TeleportShield => None,
        Spell::WeatherDome => None,
        Spell::Block => None,
        Spell::Nightingale => None,
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
        Spell::Armor => "M163",
        Spell::ForceWall => "M169",
        Spell::ForceDome => "M169",
        Spell::UtterWall => "M183",
        Spell::UtterDome => "M184",
        Spell::SpellWall => "M183",
        Spell::SpellShield => "M181",
        Spell::Magelock => "M170",
        Spell::Alarm => "M165",
        Spell::MysticMist => "M171",
        Spell::TeleportShield => "M182",
        Spell::WeatherDome => "M185",
        Spell::Block => "M166",
        Spell::Nightingale => "M171",
        _ => panic!("Invalid spell {:?} for Protection & Warning college", spell),
    }
}
