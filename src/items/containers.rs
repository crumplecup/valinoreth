//! Container definitions and properties.
//!
//! # GURPS Rules
//!
//! Containers hold other items and have a capacity measured in pounds.
//! Common containers include pouches, backpacks, sacks, and chests.
//!
//! # Citations
//!
//! BS 288 - Containers

use crate::{Capacity, Currency, TechLevel, Weight};
use tracing::{debug, instrument};

/// Container types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, strum::EnumIter)]
pub enum Container {
    /// Backpack, 40 lb capacity. BS 288
    Backpack,
    /// Barrel, 150 lb capacity. BS 288
    Barrel,
    /// Basket, 20 lb capacity. BS 288
    Basket,
    /// Belt pouch, 2 lb capacity. BS 288
    BeltPouch,
    /// Small box, 15 lb capacity. BS 288
    BoxSmall,
    /// Crate, 120 lb capacity. BS 288
    Crate,
    /// Flask, 1 lb capacity. BS 288
    Flask,
    /// Haversack, 50 lb capacity. BS 288
    Haversack,
    /// Large backpack, 60 lb capacity. BS 288
    LargeBackpack,
    /// Large chest, 200 lb capacity. BS 288
    LargeChest,
    /// Large pouch, 12 lb capacity. BS 288
    LargePouch,
    /// Large sack, 80 lb capacity. BS 288
    LargeSack,
    /// Money belt, 1 lb capacity. BS 288
    MoneyBelt,
    /// Pouch, 6 lb capacity. BS 288
    Pouch,
    /// Quiver, 3 lb capacity. BS 288
    Quiver,
    /// Scabbard, 1 lb capacity. BS 288
    Scabbard,
    /// Scroll case, 1 lb capacity. BS 288
    ScrollCase,
    /// Small backpack, 40 lb capacity. BS 288
    SmallBackpack,
    /// Small chest, 100 lb capacity. BS 288
    SmallChest,
    /// Small pouch, 3 lb capacity. BS 288
    SmallPouch,
    /// Small sack, 40 lb capacity. BS 288
    SmallSack,
    /// Trunk, 150 lb capacity. BS 288
    Trunk,
    /// Vial, 0.5 lb capacity. BS 288
    Vial,
    /// Wineskin, 2 lb capacity. BS 288
    Wineskin,
}

impl Container {
    /// Returns base cost for this container.
    #[instrument]
    pub fn base_cost(&self) -> Currency {
        debug!("Getting container base cost");
        match self {
            Self::Backpack => Currency::dollars(60.0),
            Self::Barrel => Currency::dollars(50.0),
            Self::Basket => Currency::dollars(20.0),
            Self::BeltPouch => Currency::dollars(15.0),
            Self::BoxSmall => Currency::dollars(20.0),
            Self::Crate => Currency::dollars(40.0),
            Self::Flask => Currency::dollars(10.0),
            Self::Haversack => Currency::dollars(75.0),
            Self::LargeBackpack => Currency::dollars(100.0),
            Self::LargeChest => Currency::dollars(300.0),
            Self::LargePouch => Currency::dollars(20.0),
            Self::LargeSack => Currency::dollars(50.0),
            Self::MoneyBelt => Currency::dollars(45.0),
            Self::Pouch => Currency::dollars(10.0),
            Self::Quiver => Currency::dollars(10.0),
            Self::Scabbard => Currency::dollars(20.0),
            Self::ScrollCase => Currency::dollars(25.0),
            Self::SmallBackpack => Currency::dollars(60.0),
            Self::SmallChest => Currency::dollars(100.0),
            Self::SmallPouch => Currency::dollars(10.0),
            Self::SmallSack => Currency::dollars(30.0),
            Self::Trunk => Currency::dollars(150.0),
            Self::Vial => Currency::dollars(10.0),
            Self::Wineskin => Currency::dollars(10.0),
        }
    }

    /// Returns weight for this container (empty weight).
    #[instrument]
    pub fn weight(&self) -> Weight {
        debug!("Getting container weight");
        match self {
            Self::Backpack => Weight::pounds(3.0),
            Self::Barrel => Weight::pounds(30.0),
            Self::Basket => Weight::pounds(2.0),
            Self::BeltPouch => Weight::pounds(0.25),
            Self::BoxSmall => Weight::pounds(2.0),
            Self::Crate => Weight::pounds(20.0),
            Self::Flask => Weight::pounds(0.5),
            Self::Haversack => Weight::pounds(4.0),
            Self::LargeBackpack => Weight::pounds(6.0),
            Self::LargeChest => Weight::pounds(30.0),
            Self::LargePouch => Weight::pounds(0.5),
            Self::LargeSack => Weight::pounds(6.0),
            Self::MoneyBelt => Weight::pounds(0.5),
            Self::Pouch => Weight::pounds(0.2),
            Self::Quiver => Weight::pounds(0.5),
            Self::Scabbard => Weight::pounds(0.5),
            Self::ScrollCase => Weight::pounds(0.5),
            Self::SmallBackpack => Weight::pounds(3.0),
            Self::SmallChest => Weight::pounds(10.0),
            Self::SmallPouch => Weight::pounds(0.2),
            Self::SmallSack => Weight::pounds(3.0),
            Self::Trunk => Weight::pounds(25.0),
            Self::Vial => Weight::pounds(0.1),
            Self::Wineskin => Weight::pounds(0.5),
        }
    }

    /// Returns tech level for this container.
    #[instrument]
    pub fn tech_level(&self) -> TechLevel {
        debug!("Getting container tech level");
        match self {
            Self::Backpack => TechLevel::new(1),      // Bronze Age
            Self::Barrel => TechLevel::new(0),        // Stone Age
            Self::Basket => TechLevel::new(0),        // Stone Age
            Self::BeltPouch => TechLevel::new(0),     // Stone Age
            Self::BoxSmall => TechLevel::new(1),      // Bronze Age
            Self::Crate => TechLevel::new(1),         // Bronze Age
            Self::Flask => TechLevel::new(1),         // Bronze Age
            Self::Haversack => TechLevel::new(3),     // Age of Sail
            Self::LargeBackpack => TechLevel::new(2), // Medieval
            Self::LargeChest => TechLevel::new(1),    // Bronze Age
            Self::LargePouch => TechLevel::new(0),    // Stone Age
            Self::LargeSack => TechLevel::new(0),     // Stone Age
            Self::MoneyBelt => TechLevel::new(2),     // Medieval
            Self::Pouch => TechLevel::new(0),         // Stone Age
            Self::Quiver => TechLevel::new(0),        // Stone Age
            Self::Scabbard => TechLevel::new(1),      // Bronze Age
            Self::ScrollCase => TechLevel::new(2),    // Medieval
            Self::SmallBackpack => TechLevel::new(1), // Bronze Age
            Self::SmallChest => TechLevel::new(1),    // Bronze Age
            Self::SmallPouch => TechLevel::new(0),    // Stone Age
            Self::SmallSack => TechLevel::new(0),     // Stone Age
            Self::Trunk => TechLevel::new(2),         // Medieval
            Self::Vial => TechLevel::new(1),          // Bronze Age
            Self::Wineskin => TechLevel::new(0),      // Stone Age
        }
    }

    /// Returns capacity for this container.
    #[instrument]
    pub fn capacity(&self) -> Capacity {
        debug!("Getting container capacity");
        match self {
            Self::Backpack => Capacity::pounds(40.0),
            Self::Barrel => Capacity::pounds(150.0),
            Self::Basket => Capacity::pounds(20.0),
            Self::BeltPouch => Capacity::pounds(2.0),
            Self::BoxSmall => Capacity::pounds(15.0),
            Self::Crate => Capacity::pounds(120.0),
            Self::Flask => Capacity::pounds(1.0),
            Self::Haversack => Capacity::pounds(50.0),
            Self::LargeBackpack => Capacity::pounds(60.0),
            Self::LargeChest => Capacity::pounds(200.0),
            Self::LargePouch => Capacity::pounds(12.0),
            Self::LargeSack => Capacity::pounds(80.0),
            Self::MoneyBelt => Capacity::pounds(1.0),
            Self::Pouch => Capacity::pounds(6.0),
            Self::Quiver => Capacity::pounds(3.0),
            Self::Scabbard => Capacity::pounds(1.0),
            Self::ScrollCase => Capacity::pounds(1.0),
            Self::SmallBackpack => Capacity::pounds(40.0),
            Self::SmallChest => Capacity::pounds(100.0),
            Self::SmallPouch => Capacity::pounds(3.0),
            Self::SmallSack => Capacity::pounds(40.0),
            Self::Trunk => Capacity::pounds(150.0),
            Self::Vial => Capacity::pounds(0.5),
            Self::Wineskin => Capacity::pounds(2.0),
        }
    }
}
