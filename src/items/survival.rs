//! Survival gear properties.
//!
//! # GURPS Rules
//!
//! Survival gear includes items essential for wilderness travel and
//! camping: torches, tents, blankets, rations, waterskins, bedrolls,
//! lanterns, and candles. These items enable extended expeditions.
//!
//! # Citations
//!
//! BS 288 - Camping and survival equipment

use crate::{Currency, Item, TechLevel, Weight};
use tracing::{debug, instrument};

/// Returns base cost for survival gear.
#[instrument]
pub(super) fn base_cost(item: &Item) -> Currency {
    debug!("Getting survival gear base cost");
    match item {
        Item::Torch => Currency::dollars(3.0),
        Item::Tent => Currency::dollars(50.0),
        Item::LargeTent => Currency::dollars(150.0),
        Item::Blanket => Currency::dollars(20.0),
        Item::Rations => Currency::dollars(2.0),
        Item::Waterskin => Currency::dollars(10.0),
        Item::Bedroll => Currency::dollars(25.0),
        Item::Canteen => Currency::dollars(10.0),
        Item::Lantern => Currency::dollars(20.0),
        Item::Candle => Currency::dollars(0.5),
        Item::Tinder => Currency::dollars(0.5),
        Item::FlintAndSteel => Currency::dollars(5.0),
        Item::SleepingBag => Currency::dollars(40.0),
        Item::CookingPot => Currency::dollars(20.0),
        Item::Matches => Currency::dollars(0.5),
        Item::Map => Currency::dollars(20.0),
        Item::FishingKit => Currency::dollars(5.0),
        Item::SnareWire => Currency::dollars(5.0),
        Item::SignalWhistle => Currency::dollars(5.0),
        Item::Tarp => Currency::dollars(50.0),
        _ => {
            tracing::error!(item = ?item, "Non-survival item in survival::base_cost");
            Currency::dollars(0.0)
        }
    }
}

/// Returns weight for survival gear.
#[instrument]
pub(super) fn weight(item: &Item) -> Weight {
    debug!("Getting survival gear weight");
    match item {
        Item::Torch => Weight::pounds(1.0),
        Item::Tent => Weight::pounds(5.0),
        Item::LargeTent => Weight::pounds(30.0),
        Item::Blanket => Weight::pounds(4.0),
        Item::Rations => Weight::pounds(0.5),
        Item::Waterskin => Weight::pounds(0.25), // Empty weight
        Item::Bedroll => Weight::pounds(3.0),
        Item::Canteen => Weight::pounds(0.5), // Empty weight
        Item::Lantern => Weight::pounds(2.0),
        Item::Candle => Weight::pounds(0.1),
        Item::Tinder => Weight::pounds(0.1),
        Item::FlintAndSteel => Weight::pounds(0.5),
        Item::SleepingBag => Weight::pounds(8.0),
        Item::CookingPot => Weight::pounds(3.0),
        Item::Matches => Weight::pounds(0.1),
        Item::Map => Weight::pounds(0.1),
        Item::FishingKit => Weight::pounds(0.5),
        Item::SnareWire => Weight::pounds(0.5),
        Item::SignalWhistle => Weight::pounds(0.1),
        Item::Tarp => Weight::pounds(5.0),
        _ => {
            tracing::error!(item = ?item, "Non-survival item in survival::weight");
            Weight::pounds(0.0)
        }
    }
}

/// Returns tech level for survival gear.
#[instrument]
pub(super) fn tech_level(item: &Item) -> TechLevel {
    debug!("Getting survival gear tech level");
    match item {
        Item::Torch => TechLevel::new(0),         // Stone Age
        Item::Tent => TechLevel::new(0),          // Stone Age
        Item::LargeTent => TechLevel::new(1),     // Bronze Age
        Item::Blanket => TechLevel::new(0),       // Stone Age
        Item::Rations => TechLevel::new(0),       // Stone Age (preserved food)
        Item::Waterskin => TechLevel::new(0),     // Stone Age
        Item::Bedroll => TechLevel::new(0),       // Stone Age
        Item::Canteen => TechLevel::new(3),       // Medieval (metal canteen)
        Item::Lantern => TechLevel::new(2),       // Medieval (oil lantern)
        Item::Candle => TechLevel::new(1),        // Bronze Age
        Item::Tinder => TechLevel::new(0),        // Stone Age
        Item::FlintAndSteel => TechLevel::new(0), // Stone Age
        Item::SleepingBag => TechLevel::new(5),   // Industrial (synthetic insulation)
        Item::CookingPot => TechLevel::new(0),    // Stone Age (stone pot)
        Item::Matches => TechLevel::new(5),       // Industrial
        Item::Map => TechLevel::new(3),           // Medieval
        Item::FishingKit => TechLevel::new(0),    // Stone Age
        Item::SnareWire => TechLevel::new(1),     // Bronze Age
        Item::SignalWhistle => TechLevel::new(2), // Medieval
        Item::Tarp => TechLevel::new(5),          // Industrial (waterproof fabric)
        _ => {
            tracing::error!(item = ?item, "Non-survival item in survival::tech_level");
            TechLevel::new(0)
        }
    }
}
