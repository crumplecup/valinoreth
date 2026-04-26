//! Melee weapon properties.
//!
//! # GURPS Rules
//!
//! Melee weapons use ST-based damage (thrust or swing) with modifiers.
//! Each weapon has reach, parry modifier, and required skill. Cost, weight,
//! and tech level vary by weapon type.
//!
//! # Citations
//!
//! BS 271-276 - Melee weapons table

use crate::{Currency, DamageType, Item, Reach, Skill, TechLevel, Weight, WeaponDamage};
use tracing::{debug, instrument};

/// Returns base cost for melee weapons.
#[instrument]
pub(super) fn base_cost(item: &Item) -> Currency {
    debug!("Getting melee weapon base cost");
    match item {
        Item::Axe => Currency::dollars(50.0),
        Item::Baton => Currency::dollars(20.0),
        Item::BrassKnuckles => Currency::dollars(10.0),
        Item::Broadsword => Currency::dollars(500.0),
        Item::Dagger => Currency::dollars(20.0),
        Item::Fist => Currency::dollars(0.0),
        Item::Flail => Currency::dollars(60.0),
        Item::GreatAxe => Currency::dollars(100.0),
        Item::Halberd => Currency::dollars(150.0),
        Item::Hatchet => Currency::dollars(40.0),
        Item::Javelin => Currency::dollars(30.0),
        Item::Kick => Currency::dollars(0.0),
        Item::Knife => Currency::dollars(40.0),
        Item::Kusari => Currency::dollars(70.0),
        Item::Lance => Currency::dollars(60.0),
        Item::LongSpear => Currency::dollars(60.0),
        Item::Mace => Currency::dollars(50.0),
        Item::MainGauche => Currency::dollars(50.0),
        Item::Morningstar => Currency::dollars(80.0),
        Item::Quarterstaff => Currency::dollars(10.0),
        Item::Rapier => Currency::dollars(500.0),
        Item::Saber => Currency::dollars(500.0),
        Item::Shortsword => Currency::dollars(400.0),
        Item::Smallsword => Currency::dollars(400.0),
        Item::Spear => Currency::dollars(40.0),
        Item::Staff => Currency::dollars(5.0),
        Item::TwoHandedSword => Currency::dollars(900.0),
        Item::Warhammer => Currency::dollars(100.0),
        Item::Katana => Currency::dollars(650.0),
        Item::Scimitar => Currency::dollars(500.0),
        Item::Cutlass => Currency::dollars(400.0),
        Item::Longsword => Currency::dollars(500.0),
        Item::BastardSword => Currency::dollars(650.0),
        Item::Wakizashi => Currency::dollars(400.0),
        Item::Nunchaku => Currency::dollars(20.0),
        Item::Sai => Currency::dollars(20.0),
        Item::Katar => Currency::dollars(40.0),
        Item::Tonfa => Currency::dollars(20.0),
        Item::Estoc => Currency::dollars(600.0),
        Item::Falchion => Currency::dollars(400.0),
        Item::Gladius => Currency::dollars(200.0),
        Item::Maul => Currency::dollars(80.0),
        Item::Pick => Currency::dollars(70.0),
        _ => {
            tracing::error!(item = ?item, "Non-melee item in weapons_melee::base_cost");
            Currency::dollars(0.0)
        }
    }
}

/// Returns weight for melee weapons.
#[instrument]
pub(super) fn weight(item: &Item) -> Weight {
    debug!("Getting melee weapon weight");
    match item {
        Item::Axe => Weight::pounds(4.0),
        Item::Baton => Weight::pounds(1.0),
        Item::BrassKnuckles => Weight::pounds(0.25),
        Item::Broadsword => Weight::pounds(3.0),
        Item::Dagger => Weight::pounds(0.25),
        Item::Fist => Weight::pounds(0.0),
        Item::Flail => Weight::pounds(8.0),
        Item::GreatAxe => Weight::pounds(8.0),
        Item::Halberd => Weight::pounds(12.0),
        Item::Hatchet => Weight::pounds(2.0),
        Item::Javelin => Weight::pounds(2.0),
        Item::Kick => Weight::pounds(0.0),
        Item::Knife => Weight::pounds(1.0),
        Item::Kusari => Weight::pounds(5.0),
        Item::Lance => Weight::pounds(6.0),
        Item::LongSpear => Weight::pounds(5.0),
        Item::Mace => Weight::pounds(5.0),
        Item::MainGauche => Weight::pounds(1.25),
        Item::Morningstar => Weight::pounds(6.0),
        Item::Quarterstaff => Weight::pounds(4.0),
        Item::Rapier => Weight::pounds(2.75),
        Item::Saber => Weight::pounds(2.0),
        Item::Shortsword => Weight::pounds(2.0),
        Item::Smallsword => Weight::pounds(1.5),
        Item::Spear => Weight::pounds(4.0),
        Item::Staff => Weight::pounds(4.0),
        Item::TwoHandedSword => Weight::pounds(7.0),
        Item::Warhammer => Weight::pounds(7.0),
        Item::Katana => Weight::pounds(2.5),
        Item::Scimitar => Weight::pounds(3.0),
        Item::Cutlass => Weight::pounds(2.0),
        Item::Longsword => Weight::pounds(3.0),
        Item::BastardSword => Weight::pounds(5.0),
        Item::Wakizashi => Weight::pounds(1.5),
        Item::Nunchaku => Weight::pounds(1.5),
        Item::Sai => Weight::pounds(1.0),
        Item::Katar => Weight::pounds(1.0),
        Item::Tonfa => Weight::pounds(1.5),
        Item::Estoc => Weight::pounds(3.5),
        Item::Falchion => Weight::pounds(3.5),
        Item::Gladius => Weight::pounds(2.0),
        Item::Maul => Weight::pounds(12.0),
        Item::Pick => Weight::pounds(3.0),
        _ => {
            tracing::error!(item = ?item, "Non-melee item in weapons_melee::weight");
            Weight::pounds(0.0)
        }
    }
}

/// Returns tech level for melee weapons.
#[instrument]
pub(super) fn tech_level(item: &Item) -> TechLevel {
    debug!("Getting melee weapon tech level");
    match item {
        Item::Axe => TechLevel::new(0),          // Stone Age
        Item::Baton => TechLevel::new(5),        // Modern (police baton)
        Item::BrassKnuckles => TechLevel::new(3), // Industrial
        Item::Broadsword => TechLevel::new(2),   // Medieval
        Item::Dagger => TechLevel::new(1),       // Bronze Age
        Item::Fist => TechLevel::new(0),         // Stone Age
        Item::Flail => TechLevel::new(2),        // Medieval
        Item::GreatAxe => TechLevel::new(1),     // Bronze/Iron Age
        Item::Halberd => TechLevel::new(2),      // Medieval
        Item::Hatchet => TechLevel::new(0),      // Stone Age
        Item::Javelin => TechLevel::new(0),      // Stone Age
        Item::Kick => TechLevel::new(0),         // Stone Age
        Item::Knife => TechLevel::new(0),        // Stone Age
        Item::Kusari => TechLevel::new(2),       // Medieval Japan
        Item::Lance => TechLevel::new(2),        // Medieval
        Item::LongSpear => TechLevel::new(1),    // Bronze Age
        Item::Mace => TechLevel::new(1),         // Bronze Age
        Item::MainGauche => TechLevel::new(4),   // Renaissance
        Item::Morningstar => TechLevel::new(2),  // Medieval
        Item::Quarterstaff => TechLevel::new(0), // Stone Age
        Item::Rapier => TechLevel::new(4),       // Renaissance
        Item::Saber => TechLevel::new(4),        // Age of Sail
        Item::Shortsword => TechLevel::new(1),   // Bronze Age
        Item::Smallsword => TechLevel::new(4),   // Renaissance
        Item::Spear => TechLevel::new(0),        // Stone Age
        Item::Staff => TechLevel::new(0),        // Stone Age
        Item::TwoHandedSword => TechLevel::new(2), // Medieval
        Item::Warhammer => TechLevel::new(2),    // Medieval
        Item::Katana => TechLevel::new(3),       // Medieval Japan
        Item::Scimitar => TechLevel::new(2),     // Medieval
        Item::Cutlass => TechLevel::new(4),      // Age of Sail
        Item::Longsword => TechLevel::new(2),    // Medieval
        Item::BastardSword => TechLevel::new(2), // Medieval
        Item::Wakizashi => TechLevel::new(3),    // Medieval Japan
        Item::Nunchaku => TechLevel::new(2),     // Medieval
        Item::Sai => TechLevel::new(2),          // Medieval
        Item::Katar => TechLevel::new(2),        // Medieval India
        Item::Tonfa => TechLevel::new(0),        // Stone Age
        Item::Estoc => TechLevel::new(3),        // Late Medieval
        Item::Falchion => TechLevel::new(2),     // Medieval
        Item::Gladius => TechLevel::new(1),      // Roman/Iron Age
        Item::Maul => TechLevel::new(1),         // Bronze/Iron Age
        Item::Pick => TechLevel::new(2),         // Medieval
        _ => {
            tracing::error!(item = ?item, "Non-melee item in weapons_melee::tech_level");
            TechLevel::new(0)
        }
    }
}

/// Returns weapon damage (migrated from MeleeWeapon::damage).
#[instrument]
pub(super) fn damage(item: &Item) -> WeaponDamage {
    debug!("Getting melee weapon damage");
    match item {
        Item::Axe => WeaponDamage::Swing {
            modifier: 2,
            damage_type: DamageType::Cutting,
        },
        Item::Baton => WeaponDamage::Swing {
            modifier: 0,
            damage_type: DamageType::Crushing,
        },
        Item::BrassKnuckles => WeaponDamage::Thrust {
            modifier: 0,
            damage_type: DamageType::Crushing,
        },
        Item::Broadsword => WeaponDamage::Swing {
            modifier: 1,
            damage_type: DamageType::Cutting,
        },
        Item::Dagger => WeaponDamage::Thrust {
            modifier: -1,
            damage_type: DamageType::Impaling,
        },
        Item::Fist => WeaponDamage::Thrust {
            modifier: -1,
            damage_type: DamageType::Crushing,
        },
        Item::Flail => WeaponDamage::Swing {
            modifier: 2,
            damage_type: DamageType::Crushing,
        },
        Item::GreatAxe => WeaponDamage::Swing {
            modifier: 3,
            damage_type: DamageType::Cutting,
        },
        Item::Halberd => WeaponDamage::Swing {
            modifier: 3,
            damage_type: DamageType::Cutting,
        },
        Item::Hatchet => WeaponDamage::Swing {
            modifier: 1,
            damage_type: DamageType::Cutting,
        },
        Item::Javelin => WeaponDamage::Thrust {
            modifier: 1,
            damage_type: DamageType::Impaling,
        },
        Item::Kick => WeaponDamage::Thrust {
            modifier: 0,
            damage_type: DamageType::Crushing,
        },
        Item::Knife => WeaponDamage::Thrust {
            modifier: -1,
            damage_type: DamageType::Impaling,
        },
        Item::Kusari => WeaponDamage::Swing {
            modifier: 1,
            damage_type: DamageType::Crushing,
        },
        Item::Lance => WeaponDamage::Thrust {
            modifier: 3,
            damage_type: DamageType::Impaling,
        },
        Item::LongSpear => WeaponDamage::Thrust {
            modifier: 2,
            damage_type: DamageType::Impaling,
        },
        Item::Mace => WeaponDamage::Swing {
            modifier: 2,
            damage_type: DamageType::Crushing,
        },
        Item::MainGauche => WeaponDamage::Thrust {
            modifier: -1,
            damage_type: DamageType::Impaling,
        },
        Item::Morningstar => WeaponDamage::Swing {
            modifier: 3,
            damage_type: DamageType::Crushing,
        },
        Item::Quarterstaff => WeaponDamage::Swing {
            modifier: 2,
            damage_type: DamageType::Crushing,
        },
        Item::Rapier => WeaponDamage::Thrust {
            modifier: 1,
            damage_type: DamageType::Impaling,
        },
        Item::Saber => WeaponDamage::Swing {
            modifier: 1,
            damage_type: DamageType::Cutting,
        },
        Item::Shortsword => WeaponDamage::Swing {
            modifier: 0,
            damage_type: DamageType::Cutting,
        },
        Item::Smallsword => WeaponDamage::Thrust {
            modifier: 0,
            damage_type: DamageType::Impaling,
        },
        Item::Spear => WeaponDamage::Thrust {
            modifier: 2,
            damage_type: DamageType::Impaling,
        },
        Item::Staff => WeaponDamage::Swing {
            modifier: 1,
            damage_type: DamageType::Crushing,
        },
        Item::TwoHandedSword => WeaponDamage::Swing {
            modifier: 2,
            damage_type: DamageType::Cutting,
        },
        Item::Warhammer => WeaponDamage::Swing {
            modifier: 3,
            damage_type: DamageType::Impaling,
        },
        Item::Katana => WeaponDamage::Swing {
            modifier: 1,
            damage_type: DamageType::Cutting,
        },
        Item::Scimitar => WeaponDamage::Swing {
            modifier: 1,
            damage_type: DamageType::Cutting,
        },
        Item::Cutlass => WeaponDamage::Swing {
            modifier: 1,
            damage_type: DamageType::Cutting,
        },
        Item::Longsword => WeaponDamage::Swing {
            modifier: 1,
            damage_type: DamageType::Cutting,
        },
        Item::BastardSword => WeaponDamage::Swing {
            modifier: 2,
            damage_type: DamageType::Cutting,
        },
        Item::Wakizashi => WeaponDamage::Swing {
            modifier: 0,
            damage_type: DamageType::Cutting,
        },
        Item::Nunchaku => WeaponDamage::Swing {
            modifier: 1,
            damage_type: DamageType::Crushing,
        },
        Item::Sai => WeaponDamage::Thrust {
            modifier: -1,
            damage_type: DamageType::Impaling,
        },
        Item::Katar => WeaponDamage::Thrust {
            modifier: 1,
            damage_type: DamageType::Impaling,
        },
        Item::Tonfa => WeaponDamage::Swing {
            modifier: 1,
            damage_type: DamageType::Crushing,
        },
        Item::Estoc => WeaponDamage::Thrust {
            modifier: 2,
            damage_type: DamageType::Impaling,
        },
        Item::Falchion => WeaponDamage::Swing {
            modifier: 2,
            damage_type: DamageType::Cutting,
        },
        Item::Gladius => WeaponDamage::Swing {
            modifier: 0,
            damage_type: DamageType::Cutting,
        },
        Item::Maul => WeaponDamage::Swing {
            modifier: 4,
            damage_type: DamageType::Crushing,
        },
        Item::Pick => WeaponDamage::Swing {
            modifier: 2,
            damage_type: DamageType::Impaling,
        },
        _ => {
            tracing::error!(item = ?item, "Non-melee item in weapons_melee::damage");
            WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Crushing,
            }
        }
    }
}

/// Returns weapon reach (migrated from MeleeWeapon::reach).
#[instrument]
pub(super) fn reach(item: &Item) -> Reach {
    debug!("Getting melee weapon reach");
    match item {
        Item::Axe => Reach::One,
        Item::Baton => Reach::One,
        Item::BrassKnuckles => Reach::Close,
        Item::Broadsword => Reach::One,
        Item::Dagger => Reach::Close,
        Item::Fist => Reach::Close,
        Item::Flail => Reach::OneTwo,
        Item::GreatAxe => Reach::OneTwo,
        Item::Halberd => Reach::TwoThree,
        Item::Hatchet => Reach::One,
        Item::Javelin => Reach::One,
        Item::Kick => Reach::CloseOne,
        Item::Knife => Reach::Close,
        Item::Kusari => Reach::OneThree,
        Item::Lance => Reach::Three,
        Item::LongSpear => Reach::TwoThree,
        Item::Mace => Reach::One,
        Item::MainGauche => Reach::One,
        Item::Morningstar => Reach::One,
        Item::Quarterstaff => Reach::OneTwo,
        Item::Rapier => Reach::One,
        Item::Saber => Reach::One,
        Item::Shortsword => Reach::One,
        Item::Smallsword => Reach::One,
        Item::Spear => Reach::OneTwo,
        Item::Staff => Reach::OneTwo,
        Item::TwoHandedSword => Reach::OneTwo,
        Item::Warhammer => Reach::OneTwo,
        Item::Katana => Reach::One,
        Item::Scimitar => Reach::One,
        Item::Cutlass => Reach::One,
        Item::Longsword => Reach::One,
        Item::BastardSword => Reach::OneTwo,
        Item::Wakizashi => Reach::One,
        Item::Nunchaku => Reach::One,
        Item::Sai => Reach::Close,
        Item::Katar => Reach::Close,
        Item::Tonfa => Reach::One,
        Item::Estoc => Reach::One,
        Item::Falchion => Reach::One,
        Item::Gladius => Reach::One,
        Item::Maul => Reach::OneTwo,
        Item::Pick => Reach::One,
        _ => {
            tracing::error!(item = ?item, "Non-melee item in weapons_melee::reach");
            Reach::Close
        }
    }
}

/// Returns parry modifier (migrated from MeleeWeapon::parry_modifier).
#[instrument]
pub(super) fn parry_modifier(item: &Item) -> i32 {
    debug!("Getting melee weapon parry modifier");
    match item {
        Item::Axe => -1,
        Item::Baton => 0,
        Item::BrassKnuckles => 0,
        Item::Broadsword => 0,
        Item::Dagger => -1,
        Item::Fist => 0,
        Item::Flail => -2,
        Item::GreatAxe => -2,
        Item::Halberd => 0,
        Item::Hatchet => -1,
        Item::Javelin => 0,
        Item::Kick => -2,
        Item::Knife => -1,
        Item::Kusari => -2,
        Item::Lance => -2,
        Item::LongSpear => 0,
        Item::Mace => 0,
        Item::MainGauche => 1,
        Item::Morningstar => 0,
        Item::Quarterstaff => 2,
        Item::Rapier => 1,
        Item::Saber => 0,
        Item::Shortsword => 0,
        Item::Smallsword => 1,
        Item::Spear => 0,
        Item::Staff => 2,
        Item::TwoHandedSword => 0,
        Item::Warhammer => -1,
        Item::Katana => 0,
        Item::Scimitar => 0,
        Item::Cutlass => 0,
        Item::Longsword => 0,
        Item::BastardSword => 0,
        Item::Wakizashi => 0,
        Item::Nunchaku => -2,
        Item::Sai => 1,
        Item::Katar => 0,
        Item::Tonfa => 1,
        Item::Estoc => 1,
        Item::Falchion => 0,
        Item::Gladius => 0,
        Item::Maul => -2,
        Item::Pick => -1,
        _ => {
            tracing::error!(item = ?item, "Non-melee item in weapons_melee::parry_modifier");
            0
        }
    }
}

/// Returns required skill (migrated from MeleeWeapon::required_skill).
#[instrument]
pub(super) fn required_skill(item: &Item) -> Skill {
    debug!("Getting melee weapon required skill");
    match item {
        Item::Axe => Skill::AxeMace,
        Item::Baton => Skill::Shortsword,
        Item::BrassKnuckles => Skill::Brawling,
        Item::Broadsword => Skill::Broadsword,
        Item::Dagger => Skill::Knife,
        Item::Fist => Skill::Brawling,
        Item::Flail => Skill::Flail,
        Item::GreatAxe => Skill::TwoHandedAxeMace,
        Item::Halberd => Skill::Polearm,
        Item::Hatchet => Skill::AxeMace,
        Item::Javelin => Skill::Spear,
        Item::Kick => Skill::Brawling,
        Item::Knife => Skill::Knife,
        Item::Kusari => Skill::Kusari,
        Item::Lance => Skill::Lance,
        Item::LongSpear => Skill::Spear,
        Item::Mace => Skill::AxeMace,
        Item::MainGauche => Skill::MainGauche,
        Item::Morningstar => Skill::Flail,
        Item::Quarterstaff => Skill::Staff,
        Item::Rapier => Skill::Rapier,
        Item::Saber => Skill::Saber,
        Item::Shortsword => Skill::Shortsword,
        Item::Smallsword => Skill::Smallsword,
        Item::Spear => Skill::Spear,
        Item::Staff => Skill::Staff,
        Item::TwoHandedSword => Skill::TwoHandedSword,
        Item::Warhammer => Skill::TwoHandedAxeMace,
        Item::Katana => Skill::TwoHandedSword,
        Item::Scimitar => Skill::Broadsword,
        Item::Cutlass => Skill::Broadsword,
        Item::Longsword => Skill::Broadsword,
        Item::BastardSword => Skill::Broadsword,
        Item::Wakizashi => Skill::Shortsword,
        Item::Nunchaku => Skill::Flail,
        Item::Sai => Skill::Knife,
        Item::Katar => Skill::Knife,
        Item::Tonfa => Skill::Shortsword,
        Item::Estoc => Skill::Rapier,
        Item::Falchion => Skill::Broadsword,
        Item::Gladius => Skill::Shortsword,
        Item::Maul => Skill::TwoHandedAxeMace,
        Item::Pick => Skill::AxeMace,
        _ => {
            tracing::error!(item = ?item, "Non-melee item in weapons_melee::required_skill");
            Skill::Brawling
        }
    }
}
