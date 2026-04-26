//! Armor properties.
//!
//! # GURPS Rules
//!
//! Armor provides damage resistance (DR) which reduces incoming damage
//! before wound multipliers are applied. Heavier armor provides more
//! protection but weighs more and costs more.
//!
//! # Citations
//!
//! BS 279-282 - Armor table

use crate::{Currency, Item, TechLevel, Weight};
use tracing::{debug, instrument};

/// Returns base cost for armor.
#[instrument]
pub(super) fn base_cost(item: &Item) -> Currency {
    debug!("Getting armor base cost");
    match item {
        Item::NoArmor => Currency::dollars(0.0),
        Item::LeatherArmor => Currency::dollars(100.0),
        Item::Chainmail => Currency::dollars(550.0),
        Item::PlateArmor => Currency::dollars(3000.0),
        Item::HeavyPlate => Currency::dollars(6000.0),
        Item::ClothArmor => Currency::dollars(30.0),
        Item::LightLeather => Currency::dollars(50.0),
        Item::HeavyLeather => Currency::dollars(150.0),
        Item::ScaleMail => Currency::dollars(420.0),
        Item::SplintMail => Currency::dollars(700.0),
        Item::HalfPlate => Currency::dollars(1500.0),
        Item::MailHauberk => Currency::dollars(600.0),
        Item::MailShirt => Currency::dollars(350.0),
        Item::Brigandine => Currency::dollars(500.0),
        Item::LamellarArmor => Currency::dollars(500.0),
        Item::BronzePlate => Currency::dollars(2400.0),
        Item::LightScale => Currency::dollars(280.0),
        Item::BallisticVest => Currency::dollars(400.0),
        Item::TacticalVest => Currency::dollars(900.0),
        Item::FlakJacket => Currency::dollars(500.0),
        Item::SmallShield => Currency::dollars(40.0),
        Item::MediumShield => Currency::dollars(60.0),
        Item::LargeShield => Currency::dollars(90.0),
        Item::Buckler => Currency::dollars(25.0),
        Item::TowerShield => Currency::dollars(150.0),
        _ => {
            tracing::error!(item = ?item, "Non-armor item in armor::base_cost");
            Currency::dollars(0.0)
        }
    }
}

/// Returns weight for armor.
#[instrument]
pub(super) fn weight(item: &Item) -> Weight {
    debug!("Getting armor weight");
    match item {
        Item::NoArmor => Weight::pounds(0.0),
        Item::LeatherArmor => Weight::pounds(10.0),
        Item::Chainmail => Weight::pounds(35.0),
        Item::PlateArmor => Weight::pounds(50.0),
        Item::HeavyPlate => Weight::pounds(60.0),
        Item::ClothArmor => Weight::pounds(6.0),
        Item::LightLeather => Weight::pounds(5.0),
        Item::HeavyLeather => Weight::pounds(15.0),
        Item::ScaleMail => Weight::pounds(42.0),
        Item::SplintMail => Weight::pounds(45.0),
        Item::HalfPlate => Weight::pounds(30.0),
        Item::MailHauberk => Weight::pounds(45.0),
        Item::MailShirt => Weight::pounds(25.0),
        Item::Brigandine => Weight::pounds(25.0),
        Item::LamellarArmor => Weight::pounds(35.0),
        Item::BronzePlate => Weight::pounds(60.0),
        Item::LightScale => Weight::pounds(28.0),
        Item::BallisticVest => Weight::pounds(2.0),
        Item::TacticalVest => Weight::pounds(9.0),
        Item::FlakJacket => Weight::pounds(20.0),
        Item::SmallShield => Weight::pounds(8.0),
        Item::MediumShield => Weight::pounds(15.0),
        Item::LargeShield => Weight::pounds(25.0),
        Item::Buckler => Weight::pounds(5.0),
        Item::TowerShield => Weight::pounds(45.0),
        _ => {
            tracing::error!(item = ?item, "Non-armor item in armor::weight");
            Weight::pounds(0.0)
        }
    }
}

/// Returns tech level for armor.
#[instrument]
pub(super) fn tech_level(item: &Item) -> TechLevel {
    debug!("Getting armor tech level");
    match item {
        Item::NoArmor => TechLevel::new(0),       // Stone Age
        Item::LeatherArmor => TechLevel::new(1),  // Bronze Age
        Item::Chainmail => TechLevel::new(2),     // Medieval
        Item::PlateArmor => TechLevel::new(3),    // Age of Sail
        Item::HeavyPlate => TechLevel::new(3),    // Age of Sail
        Item::ClothArmor => TechLevel::new(1),    // Bronze Age
        Item::LightLeather => TechLevel::new(0),  // Stone Age
        Item::HeavyLeather => TechLevel::new(1),  // Bronze Age
        Item::ScaleMail => TechLevel::new(2),     // Medieval
        Item::SplintMail => TechLevel::new(2),    // Medieval
        Item::HalfPlate => TechLevel::new(3),     // Age of Sail
        Item::MailHauberk => TechLevel::new(2),   // Medieval
        Item::MailShirt => TechLevel::new(2),     // Medieval
        Item::Brigandine => TechLevel::new(3),    // Age of Sail
        Item::LamellarArmor => TechLevel::new(1), // Bronze Age
        Item::BronzePlate => TechLevel::new(1),   // Bronze Age
        Item::LightScale => TechLevel::new(2),    // Medieval
        Item::BallisticVest => TechLevel::new(7), // Digital Age
        Item::TacticalVest => TechLevel::new(8),  // Microtech Age
        Item::FlakJacket => TechLevel::new(6),    // Atomic Age
        Item::SmallShield => TechLevel::new(1),   // Bronze Age
        Item::MediumShield => TechLevel::new(1),  // Bronze Age
        Item::LargeShield => TechLevel::new(1),   // Bronze Age
        Item::Buckler => TechLevel::new(2),       // Medieval
        Item::TowerShield => TechLevel::new(2),   // Medieval
        _ => {
            tracing::error!(item = ?item, "Non-armor item in armor::tech_level");
            TechLevel::new(0)
        }
    }
}

/// Returns damage resistance (armor-specific property).
#[instrument]
pub(super) fn damage_resistance(item: &Item) -> i32 {
    debug!("Getting armor damage resistance");
    match item {
        Item::NoArmor => 0,
        Item::LeatherArmor => 1,
        Item::Chainmail => 4,
        Item::PlateArmor => 6,
        Item::HeavyPlate => 8,
        Item::ClothArmor => 1,
        Item::LightLeather => 1,
        Item::HeavyLeather => 2,
        Item::ScaleMail => 4,
        Item::SplintMail => 5,
        Item::HalfPlate => 5,
        Item::MailHauberk => 4,
        Item::MailShirt => 4,
        Item::Brigandine => 4,
        Item::LamellarArmor => 4,
        Item::BronzePlate => 5,
        Item::LightScale => 3,
        Item::BallisticVest => 10, // DR 10 vs ballistic
        Item::TacticalVest => 18,  // DR 18 vs ballistic
        Item::FlakJacket => 7,     // DR 7 vs ballistic
        Item::SmallShield => 1,    // DB converted to DR approximation
        Item::MediumShield => 2,   // DB converted to DR approximation
        Item::LargeShield => 3,    // DB converted to DR approximation
        Item::Buckler => 1,        // DB converted to DR approximation
        Item::TowerShield => 4,    // DB converted to DR approximation
        _ => {
            tracing::error!(item = ?item, "Non-armor item in armor::damage_resistance");
            0
        }
    }
}
