//! Ranged weapon properties.
//!
//! # GURPS Rules
//!
//! Ranged weapons have fixed damage, accuracy, rate of fire, range,
//! shots, and recoil. This module implements the core stats needed for
//! item enumeration.
//!
//! # Citations
//!
//! BS 276-278 - Ranged weapons table

use crate::{
    Currency, DamageType, DieLevel, Item, Skill, TechLevel, Weight, WeaponDamage,
};
use tracing::{debug, instrument};

/// Returns base cost for ranged weapons.
#[instrument]
pub(super) fn base_cost(item: &Item) -> Currency {
    debug!("Getting ranged weapon base cost");
    match item {
        Item::Bow => Currency::dollars(100.0),
        Item::Crossbow => Currency::dollars(150.0),
        Item::Pistol => Currency::dollars(350.0),
        Item::Rifle => Currency::dollars(500.0),
        Item::Shotgun => Currency::dollars(500.0),
        Item::Sling => Currency::dollars(20.0),
        Item::ThrowingKnife => Currency::dollars(30.0),
        _ => {
            tracing::error!(item = ?item, "Non-ranged item in weapons_ranged::base_cost");
            Currency::dollars(0.0)
        }
    }
}

/// Returns weight for ranged weapons.
#[instrument]
pub(super) fn weight(item: &Item) -> Weight {
    debug!("Getting ranged weapon weight");
    match item {
        Item::Bow => Weight::pounds(2.0),
        Item::Crossbow => Weight::pounds(6.0),
        Item::Pistol => Weight::pounds(1.5),
        Item::Rifle => Weight::pounds(9.0),
        Item::Shotgun => Weight::pounds(8.0),
        Item::Sling => Weight::pounds(0.5),
        Item::ThrowingKnife => Weight::pounds(0.5),
        _ => {
            tracing::error!(item = ?item, "Non-ranged item in weapons_ranged::weight");
            Weight::pounds(0.0)
        }
    }
}

/// Returns tech level for ranged weapons.
#[instrument]
pub(super) fn tech_level(item: &Item) -> TechLevel {
    debug!("Getting ranged weapon tech level");
    match item {
        Item::Bow => TechLevel::new(0),          // Stone Age
        Item::Crossbow => TechLevel::new(2),     // Medieval
        Item::Pistol => TechLevel::new(6),       // Atomic Age
        Item::Rifle => TechLevel::new(6),        // Atomic Age
        Item::Shotgun => TechLevel::new(5),      // Mechanized Age
        Item::Sling => TechLevel::new(0),        // Stone Age
        Item::ThrowingKnife => TechLevel::new(0), // Stone Age
        _ => {
            tracing::error!(item = ?item, "Non-ranged item in weapons_ranged::tech_level");
            TechLevel::new(0)
        }
    }
}

/// Returns weapon damage (migrated from RangedWeapon::damage).
#[instrument]
pub(super) fn damage(item: &Item) -> WeaponDamage {
    debug!("Getting ranged weapon damage");
    match item {
        Item::Bow => WeaponDamage::Fixed {
            dice: DieLevel::new(1, 0),
            damage_type: DamageType::Impaling,
        },
        Item::Crossbow => WeaponDamage::Fixed {
            dice: DieLevel::new(1, 4),
            damage_type: DamageType::Impaling,
        },
        Item::Pistol => WeaponDamage::Fixed {
            dice: DieLevel::new(2, 2),
            damage_type: DamageType::Piercing,
        },
        Item::Rifle => WeaponDamage::Fixed {
            dice: DieLevel::new(5, 0),
            damage_type: DamageType::Piercing,
        },
        Item::Shotgun => WeaponDamage::Fixed {
            dice: DieLevel::new(1, 1),
            damage_type: DamageType::Piercing,
        },
        Item::Sling => WeaponDamage::Swing {
            modifier: 0,
            damage_type: DamageType::Piercing,
        },
        Item::ThrowingKnife => WeaponDamage::Thrust {
            modifier: -1,
            damage_type: DamageType::Impaling,
        },
        _ => {
            tracing::error!(item = ?item, "Non-ranged item in weapons_ranged::damage");
            WeaponDamage::Fixed {
                dice: DieLevel::new(1, 0),
                damage_type: DamageType::Crushing,
            }
        }
    }
}

/// Returns weapon accuracy (migrated from RangedWeapon::accuracy).
#[instrument]
pub(super) fn accuracy(item: &Item) -> i32 {
    debug!("Getting ranged weapon accuracy");
    match item {
        Item::Bow => 2,
        Item::Crossbow => 4,
        Item::Pistol => 2,
        Item::Rifle => 5,
        Item::Shotgun => 3,
        Item::Sling => 0,
        Item::ThrowingKnife => 0,
        _ => {
            tracing::error!(item = ?item, "Non-ranged item in weapons_ranged::accuracy");
            0
        }
    }
}

/// Returns required skill (migrated from RangedWeapon::required_skill).
#[instrument]
pub(super) fn required_skill(item: &Item) -> Skill {
    debug!("Getting ranged weapon required skill");
    match item {
        Item::Bow => Skill::Bow,
        Item::Crossbow => Skill::Crossbow,
        Item::Pistol => Skill::Guns,
        Item::Rifle => Skill::Guns,
        Item::Shotgun => Skill::Guns,
        Item::Sling => Skill::Sling,
        Item::ThrowingKnife => Skill::ThrownWeapon,
        _ => {
            tracing::error!(item = ?item, "Non-ranged item in weapons_ranged::required_skill");
            Skill::Brawling
        }
    }
}
