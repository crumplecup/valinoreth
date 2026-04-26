//! Tool properties.
//!
//! # GURPS Rules
//!
//! Tools provide bonuses to specific tasks or enable certain actions.
//! Common tools include lockpicks, first aid kits, toolkits, rope,
//! grapnels, crowbars, and specialized equipment.
//!
//! # Citations
//!
//! BS 289 - Tools and equipment

use crate::{Currency, Item, TechLevel, Weight};
use tracing::{debug, instrument};

/// Returns base cost for tools.
#[instrument]
pub(super) fn base_cost(item: &Item) -> Currency {
    debug!("Getting tool base cost");
    match item {
        Item::Lockpicks => Currency::dollars(50.0),
        Item::FirstAidKit => Currency::dollars(50.0),
        Item::Toolkit => Currency::dollars(200.0),
        Item::Rope => Currency::dollars(5.0),
        Item::Grapnel => Currency::dollars(20.0),
        Item::Crowbar => Currency::dollars(20.0),
        Item::Hammer => Currency::dollars(15.0),
        Item::Saw => Currency::dollars(150.0),
        Item::Shovel => Currency::dollars(12.0),
        Item::MagnifyingGlass => Currency::dollars(100.0),
        _ => {
            tracing::error!(item = ?item, "Non-tool item in tools::base_cost");
            Currency::dollars(0.0)
        }
    }
}

/// Returns weight for tools.
#[instrument]
pub(super) fn weight(item: &Item) -> Weight {
    debug!("Getting tool weight");
    match item {
        Item::Lockpicks => Weight::pounds(0.1),
        Item::FirstAidKit => Weight::pounds(2.0),
        Item::Toolkit => Weight::pounds(10.0),
        Item::Rope => Weight::pounds(1.5),
        Item::Grapnel => Weight::pounds(2.0),
        Item::Crowbar => Weight::pounds(3.0),
        Item::Hammer => Weight::pounds(2.0),
        Item::Saw => Weight::pounds(3.0),
        Item::Shovel => Weight::pounds(6.0),
        Item::MagnifyingGlass => Weight::pounds(0.25),
        _ => {
            tracing::error!(item = ?item, "Non-tool item in tools::weight");
            Weight::pounds(0.0)
        }
    }
}

/// Returns tech level for tools.
#[instrument]
pub(super) fn tech_level(item: &Item) -> TechLevel {
    debug!("Getting tool tech level");
    match item {
        Item::Lockpicks => TechLevel::new(3),       // Medieval/Renaissance
        Item::FirstAidKit => TechLevel::new(5),     // Industrial
        Item::Toolkit => TechLevel::new(1),         // Bronze Age (basic tools)
        Item::Rope => TechLevel::new(0),            // Stone Age
        Item::Grapnel => TechLevel::new(2),         // Medieval
        Item::Crowbar => TechLevel::new(1),         // Bronze Age
        Item::Hammer => TechLevel::new(0),          // Stone Age
        Item::Saw => TechLevel::new(2),             // Medieval
        Item::Shovel => TechLevel::new(1),          // Bronze Age
        Item::MagnifyingGlass => TechLevel::new(4), // Renaissance
        _ => {
            tracing::error!(item = ?item, "Non-tool item in tools::tech_level");
            TechLevel::new(0)
        }
    }
}
