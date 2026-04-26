//! Clothing properties.
//!
//! # GURPS Rules
//!
//! Clothing provides protection from the elements and can include
//! boots, gloves, cloaks, hats, and other garments. Some clothing
//! items may provide minimal protection but generally don't have
//! significant DR unless specifically armored.
//!
//! # Citations
//!
//! BS 266-270 - Clothing and personal items

use crate::{Currency, Item, TechLevel, Weight};
use tracing::{debug, instrument};

/// Returns base cost for clothing.
#[instrument]
pub(super) fn base_cost(item: &Item) -> Currency {
    debug!("Getting clothing base cost");
    match item {
        Item::Clothing => Currency::dollars(120.0),
        Item::Boots => Currency::dollars(80.0),
        Item::Gloves => Currency::dollars(30.0),
        Item::Cloak => Currency::dollars(50.0),
        Item::Hat => Currency::dollars(10.0),
        Item::Belt => Currency::dollars(15.0),
        Item::Robe => Currency::dollars(20.0),
        Item::Sandals => Currency::dollars(25.0),
        Item::HeavyBoots => Currency::dollars(100.0),
        Item::ReinforcedGloves => Currency::dollars(50.0),
        Item::Tunic => Currency::dollars(30.0),
        Item::Shirt => Currency::dollars(20.0),
        Item::Pants => Currency::dollars(30.0),
        Item::Dress => Currency::dollars(40.0),
        Item::Cape => Currency::dollars(20.0),
        Item::Hood => Currency::dollars(5.0),
        Item::Scarf => Currency::dollars(5.0),
        Item::Vest => Currency::dollars(25.0),
        Item::Apron => Currency::dollars(15.0),
        Item::Tabard => Currency::dollars(25.0),
        _ => {
            tracing::error!(item = ?item, "Non-clothing item in clothing::base_cost");
            Currency::dollars(0.0)
        }
    }
}

/// Returns weight for clothing.
#[instrument]
pub(super) fn weight(item: &Item) -> Weight {
    debug!("Getting clothing weight");
    match item {
        Item::Clothing => Weight::pounds(2.0),
        Item::Boots => Weight::pounds(2.0),
        Item::Gloves => Weight::pounds(0.5),
        Item::Cloak => Weight::pounds(4.0),
        Item::Hat => Weight::pounds(0.5),
        Item::Belt => Weight::pounds(0.25),
        Item::Robe => Weight::pounds(2.0),
        Item::Sandals => Weight::pounds(0.5),
        Item::HeavyBoots => Weight::pounds(3.0),
        Item::ReinforcedGloves => Weight::pounds(0.75),
        Item::Tunic => Weight::pounds(1.0),
        Item::Shirt => Weight::pounds(0.5),
        Item::Pants => Weight::pounds(1.0),
        Item::Dress => Weight::pounds(2.0),
        Item::Cape => Weight::pounds(1.0),
        Item::Hood => Weight::pounds(0.25),
        Item::Scarf => Weight::pounds(0.1),
        Item::Vest => Weight::pounds(0.5),
        Item::Apron => Weight::pounds(0.75),
        Item::Tabard => Weight::pounds(1.0),
        _ => {
            tracing::error!(item = ?item, "Non-clothing item in clothing::weight");
            Weight::pounds(0.0)
        }
    }
}

/// Returns tech level for clothing.
#[instrument]
pub(super) fn tech_level(item: &Item) -> TechLevel {
    debug!("Getting clothing tech level");
    match item {
        Item::Clothing => TechLevel::new(1),         // Bronze Age
        Item::Boots => TechLevel::new(1),            // Bronze Age
        Item::Gloves => TechLevel::new(1),           // Bronze Age
        Item::Cloak => TechLevel::new(1),            // Bronze Age
        Item::Hat => TechLevel::new(1),              // Bronze Age
        Item::Belt => TechLevel::new(0),             // Stone Age
        Item::Robe => TechLevel::new(1),             // Bronze Age
        Item::Sandals => TechLevel::new(0),          // Stone Age
        Item::HeavyBoots => TechLevel::new(2),       // Medieval
        Item::ReinforcedGloves => TechLevel::new(2), // Medieval
        Item::Tunic => TechLevel::new(0),            // Stone Age
        Item::Shirt => TechLevel::new(1),            // Bronze Age
        Item::Pants => TechLevel::new(1),            // Bronze Age
        Item::Dress => TechLevel::new(1),            // Bronze Age
        Item::Cape => TechLevel::new(0),             // Stone Age
        Item::Hood => TechLevel::new(0),             // Stone Age
        Item::Scarf => TechLevel::new(0),            // Stone Age
        Item::Vest => TechLevel::new(1),             // Bronze Age
        Item::Apron => TechLevel::new(1),            // Bronze Age
        Item::Tabard => TechLevel::new(2),           // Medieval
        _ => {
            tracing::error!(item = ?item, "Non-clothing item in clothing::tech_level");
            TechLevel::new(0)
        }
    }
}
