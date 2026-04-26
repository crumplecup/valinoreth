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
        Item::Longbow => Currency::dollars(200.0),
        Item::CompositeBow => Currency::dollars(900.0),
        Item::ShortBow => Currency::dollars(50.0),
        Item::LightCrossbow => Currency::dollars(150.0),
        Item::HeavyCrossbow => Currency::dollars(200.0),
        Item::ThrowingAxe => Currency::dollars(60.0),
        Item::Shuriken => Currency::dollars(5.0),
        Item::Dart => Currency::dollars(10.0),
        Item::Revolver => Currency::dollars(300.0),
        Item::SMG => Currency::dollars(450.0),
        Item::AssaultRifle => Currency::dollars(900.0),
        Item::SniperRifle => Currency::dollars(3500.0),
        Item::Blowgun => Currency::dollars(30.0),
        Item::Atlatl => Currency::dollars(20.0),
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
        Item::Longbow => Weight::pounds(3.0),
        Item::CompositeBow => Weight::pounds(2.0),
        Item::ShortBow => Weight::pounds(1.0),
        Item::LightCrossbow => Weight::pounds(4.0),
        Item::HeavyCrossbow => Weight::pounds(8.0),
        Item::ThrowingAxe => Weight::pounds(2.0),
        Item::Shuriken => Weight::pounds(0.1),
        Item::Dart => Weight::pounds(0.1),
        Item::Revolver => Weight::pounds(2.0),
        Item::SMG => Weight::pounds(7.0),
        Item::AssaultRifle => Weight::pounds(9.0),
        Item::SniperRifle => Weight::pounds(11.0),
        Item::Blowgun => Weight::pounds(1.0),
        Item::Atlatl => Weight::pounds(1.0),
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
        Item::Longbow => TechLevel::new(0),      // Stone Age
        Item::CompositeBow => TechLevel::new(2), // Medieval
        Item::ShortBow => TechLevel::new(0),     // Stone Age
        Item::LightCrossbow => TechLevel::new(2), // Medieval
        Item::HeavyCrossbow => TechLevel::new(2), // Medieval
        Item::ThrowingAxe => TechLevel::new(0),  // Stone Age
        Item::Shuriken => TechLevel::new(2),     // Medieval Japan
        Item::Dart => TechLevel::new(0),         // Stone Age
        Item::Revolver => TechLevel::new(6),     // Atomic Age
        Item::SMG => TechLevel::new(6),          // Atomic Age
        Item::AssaultRifle => TechLevel::new(7), // Digital Age
        Item::SniperRifle => TechLevel::new(7),  // Digital Age
        Item::Blowgun => TechLevel::new(0),      // Stone Age
        Item::Atlatl => TechLevel::new(0),       // Stone Age
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
        Item::Longbow => WeaponDamage::Fixed {
            dice: DieLevel::new(1, 2),
            damage_type: DamageType::Impaling,
        },
        Item::CompositeBow => WeaponDamage::Fixed {
            dice: DieLevel::new(1, 3),
            damage_type: DamageType::Impaling,
        },
        Item::ShortBow => WeaponDamage::Fixed {
            dice: DieLevel::new(1, -1),
            damage_type: DamageType::Impaling,
        },
        Item::LightCrossbow => WeaponDamage::Fixed {
            dice: DieLevel::new(1, 2),
            damage_type: DamageType::Impaling,
        },
        Item::HeavyCrossbow => WeaponDamage::Fixed {
            dice: DieLevel::new(1, 5),
            damage_type: DamageType::Impaling,
        },
        Item::ThrowingAxe => WeaponDamage::Swing {
            modifier: 2,
            damage_type: DamageType::Cutting,
        },
        Item::Shuriken => WeaponDamage::Thrust {
            modifier: 0,
            damage_type: DamageType::Impaling,
        },
        Item::Dart => WeaponDamage::Thrust {
            modifier: -1,
            damage_type: DamageType::Impaling,
        },
        Item::Revolver => WeaponDamage::Fixed {
            dice: DieLevel::new(2, 0),
            damage_type: DamageType::Piercing,
        },
        Item::SMG => WeaponDamage::Fixed {
            dice: DieLevel::new(2, 2),
            damage_type: DamageType::Piercing,
        },
        Item::AssaultRifle => WeaponDamage::Fixed {
            dice: DieLevel::new(5, 0),
            damage_type: DamageType::Piercing,
        },
        Item::SniperRifle => WeaponDamage::Fixed {
            dice: DieLevel::new(7, 0),
            damage_type: DamageType::Piercing,
        },
        Item::Blowgun => WeaponDamage::Fixed {
            dice: DieLevel::new(1, -3),
            damage_type: DamageType::Impaling,
        },
        Item::Atlatl => WeaponDamage::Thrust {
            modifier: 3,
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
        Item::Longbow => 3,
        Item::CompositeBow => 3,
        Item::ShortBow => 1,
        Item::LightCrossbow => 4,
        Item::HeavyCrossbow => 4,
        Item::ThrowingAxe => 2,
        Item::Shuriken => 1,
        Item::Dart => 2,
        Item::Revolver => 2,
        Item::SMG => 4,
        Item::AssaultRifle => 5,
        Item::SniperRifle => 6,
        Item::Blowgun => 1,
        Item::Atlatl => 2,
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
        Item::Longbow => Skill::Bow,
        Item::CompositeBow => Skill::Bow,
        Item::ShortBow => Skill::Bow,
        Item::LightCrossbow => Skill::Crossbow,
        Item::HeavyCrossbow => Skill::Crossbow,
        Item::ThrowingAxe => Skill::ThrownWeapon,
        Item::Shuriken => Skill::ThrownWeapon,
        Item::Dart => Skill::ThrownWeapon,
        Item::Revolver => Skill::Guns,
        Item::SMG => Skill::Guns,
        Item::AssaultRifle => Skill::Guns,
        Item::SniperRifle => Skill::Guns,
        Item::Blowgun => Skill::Blowpipe,
        Item::Atlatl => Skill::ThrownWeapon,
        _ => {
            tracing::error!(item = ?item, "Non-ranged item in weapons_ranged::required_skill");
            Skill::Brawling
        }
    }
}
