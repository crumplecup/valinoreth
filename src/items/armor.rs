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
        _ => {
            tracing::error!(item = ?item, "Non-armor item in armor::damage_resistance");
            0
        }
    }
}
