//! Container properties.
//!
//! # GURPS Rules
//!
//! Containers hold other items and have a capacity measured in pounds.
//! Common containers include pouches, backpacks, sacks, and chests.
//!
//! # Citations
//!
//! BS 288 - Containers

use crate::{Capacity, Currency, Item, TechLevel, Weight};
use tracing::{debug, instrument};

/// Returns base cost for containers.
#[instrument]
pub(super) fn base_cost(item: &Item) -> Currency {
    debug!("Getting container base cost");
    match item {
        Item::SmallPouch => Currency::dollars(10.0),
        Item::Pouch => Currency::dollars(10.0),
        Item::LargePouch => Currency::dollars(20.0),
        Item::SmallBackpack => Currency::dollars(60.0),
        Item::Backpack => Currency::dollars(60.0),
        Item::LargeBackpack => Currency::dollars(100.0),
        Item::SmallSack => Currency::dollars(30.0),
        Item::LargeSack => Currency::dollars(50.0),
        Item::SmallChest => Currency::dollars(100.0),
        Item::LargeChest => Currency::dollars(300.0),
        Item::BeltPouch => Currency::dollars(15.0),
        Item::Barrel => Currency::dollars(50.0),
        Item::Crate => Currency::dollars(40.0),
        Item::Trunk => Currency::dollars(150.0),
        Item::MoneyBelt => Currency::dollars(45.0),
        Item::Quiver => Currency::dollars(10.0),
        Item::Flask => Currency::dollars(10.0),
        Item::Basket => Currency::dollars(20.0),
        Item::BoxSmall => Currency::dollars(20.0),
        Item::Haversack => Currency::dollars(75.0),
        _ => {
            tracing::error!(item = ?item, "Non-container item in containers::base_cost");
            Currency::dollars(0.0)
        }
    }
}

/// Returns weight for containers (empty weight).
#[instrument]
pub(super) fn weight(item: &Item) -> Weight {
    debug!("Getting container weight");
    match item {
        Item::SmallPouch => Weight::pounds(0.2),
        Item::Pouch => Weight::pounds(0.2),
        Item::LargePouch => Weight::pounds(0.5),
        Item::SmallBackpack => Weight::pounds(3.0),
        Item::Backpack => Weight::pounds(3.0),
        Item::LargeBackpack => Weight::pounds(6.0),
        Item::SmallSack => Weight::pounds(3.0),
        Item::LargeSack => Weight::pounds(6.0),
        Item::SmallChest => Weight::pounds(10.0),
        Item::LargeChest => Weight::pounds(30.0),
        Item::BeltPouch => Weight::pounds(0.25),
        Item::Barrel => Weight::pounds(30.0),
        Item::Crate => Weight::pounds(20.0),
        Item::Trunk => Weight::pounds(25.0),
        Item::MoneyBelt => Weight::pounds(0.5),
        Item::Quiver => Weight::pounds(0.5),
        Item::Flask => Weight::pounds(0.5),
        Item::Basket => Weight::pounds(2.0),
        Item::BoxSmall => Weight::pounds(2.0),
        Item::Haversack => Weight::pounds(4.0),
        _ => {
            tracing::error!(item = ?item, "Non-container item in containers::weight");
            Weight::pounds(0.0)
        }
    }
}

/// Returns tech level for containers.
#[instrument]
pub(super) fn tech_level(item: &Item) -> TechLevel {
    debug!("Getting container tech level");
    match item {
        Item::SmallPouch => TechLevel::new(0),    // Stone Age
        Item::Pouch => TechLevel::new(0),         // Stone Age
        Item::LargePouch => TechLevel::new(0),    // Stone Age
        Item::SmallBackpack => TechLevel::new(1), // Bronze Age
        Item::Backpack => TechLevel::new(1),      // Bronze Age
        Item::LargeBackpack => TechLevel::new(2), // Medieval
        Item::SmallSack => TechLevel::new(0),     // Stone Age
        Item::LargeSack => TechLevel::new(0),     // Stone Age
        Item::SmallChest => TechLevel::new(1),    // Bronze Age
        Item::LargeChest => TechLevel::new(1),    // Bronze Age
        Item::BeltPouch => TechLevel::new(0),     // Stone Age
        Item::Barrel => TechLevel::new(0),        // Stone Age
        Item::Crate => TechLevel::new(1),         // Bronze Age
        Item::Trunk => TechLevel::new(2),         // Medieval
        Item::MoneyBelt => TechLevel::new(2),     // Medieval
        Item::Quiver => TechLevel::new(0),        // Stone Age
        Item::Flask => TechLevel::new(1),         // Bronze Age
        Item::Basket => TechLevel::new(0),        // Stone Age
        Item::BoxSmall => TechLevel::new(1),      // Bronze Age
        Item::Haversack => TechLevel::new(3),     // Age of Sail
        _ => {
            tracing::error!(item = ?item, "Non-container item in containers::tech_level");
            TechLevel::new(0)
        }
    }
}

/// Returns capacity for containers.
#[instrument]
pub(super) fn capacity(item: &Item) -> Capacity {
    debug!("Getting container capacity");
    match item {
        Item::SmallPouch => Capacity::pounds(3.0),
        Item::Pouch => Capacity::pounds(6.0),
        Item::LargePouch => Capacity::pounds(12.0),
        Item::SmallBackpack => Capacity::pounds(40.0),
        Item::Backpack => Capacity::pounds(40.0),
        Item::LargeBackpack => Capacity::pounds(60.0),
        Item::SmallSack => Capacity::pounds(40.0),
        Item::LargeSack => Capacity::pounds(80.0),
        Item::SmallChest => Capacity::pounds(100.0),
        Item::LargeChest => Capacity::pounds(200.0),
        Item::BeltPouch => Capacity::pounds(2.0),
        Item::Barrel => Capacity::pounds(150.0),
        Item::Crate => Capacity::pounds(120.0),
        Item::Trunk => Capacity::pounds(150.0),
        Item::MoneyBelt => Capacity::pounds(1.0),
        Item::Quiver => Capacity::pounds(3.0),
        Item::Flask => Capacity::pounds(1.0),
        Item::Basket => Capacity::pounds(20.0),
        Item::BoxSmall => Capacity::pounds(15.0),
        Item::Haversack => Capacity::pounds(50.0),
        _ => {
            tracing::error!(item = ?item, "Non-container item in containers::capacity");
            Capacity::pounds(0.0)
        }
    }
}
