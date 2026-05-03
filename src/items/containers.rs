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
    /// Amphora, 30 lb capacity. LT (Greek/Roman storage jar, TL1)
    Amphora,
    /// Backpack, 40 lb capacity. BS 288
    Backpack,
    /// Bandolier, 4 lb capacity. LT (Ammunition/potion belt, TL3)
    Bandolier,
    /// Barrel, 150 lb capacity. BS 288
    Barrel,
    /// Basket, 20 lb capacity. BS 288
    Basket,
    /// Belt pouch, 2 lb capacity. BS 288
    BeltPouch,
    /// Bolt case, 5 lb capacity. LT (Crossbow bolt container, TL2)
    BoltCase,
    /// Small box, 15 lb capacity. BS 288
    BoxSmall,
    /// Bucket, 10 lb capacity. LT (Water/grain carrier, TL1)
    Bucket,
    /// Canteen, 2 lb capacity. LT (Water container, TL2)
    Canteen,
    /// Coffer, 50 lb capacity. LT (Valuable storage box, TL2)
    Coffer,
    /// Coin purse, 1 lb capacity. LT (Small money pouch, TL1)
    CoinPurse,
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
    /// Map case, 2 lb capacity. LT (Map/document tube, TL2)
    MapCase,
    /// Money belt, 1 lb capacity. BS 288
    MoneyBelt,
    /// Oil jar, 20 lb capacity. LT (Oil storage jar, TL1)
    OilJar,
    /// Pack frame, 50 lb capacity. LT (Backpack frame, TL0)
    PackFrame,
    /// Pouch, 6 lb capacity. BS 288
    Pouch,
    /// Potion bottle, 0.5 lb capacity. LT (Potion storage, TL2)
    PotionBottle,
    /// Powder horn, 1 lb capacity. LT (Gunpowder container, TL4)
    PowderHorn,
    /// Quiver, 3 lb capacity. BS 288
    Quiver,
    /// Saddlebags, 40 lb capacity. LT (Horse/mount bags, TL1)
    Saddlebags,
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
    /// Strongbox, 30 lb capacity. LT (Reinforced lockbox, TL3)
    Strongbox,
    /// Trunk, 150 lb capacity. BS 288
    Trunk,
    /// Vial, 0.5 lb capacity. BS 288
    Vial,
    /// Water jar, 25 lb capacity. LT (Large water storage, TL1)
    WaterJar,
    /// Wineskin, 2 lb capacity. BS 288
    Wineskin,
}

impl Container {
    /// Returns base cost for this container.
    #[instrument]
    pub fn base_cost(&self) -> Currency {
        debug!("Getting container base cost");
        match self {
            Self::Amphora => Currency::dollars(35.0),
            Self::Backpack => Currency::dollars(60.0),
            Self::Bandolier => Currency::dollars(30.0),
            Self::Barrel => Currency::dollars(50.0),
            Self::Basket => Currency::dollars(20.0),
            Self::BeltPouch => Currency::dollars(15.0),
            Self::BoltCase => Currency::dollars(20.0),
            Self::BoxSmall => Currency::dollars(20.0),
            Self::Bucket => Currency::dollars(8.0),
            Self::Canteen => Currency::dollars(15.0),
            Self::Coffer => Currency::dollars(80.0),
            Self::CoinPurse => Currency::dollars(12.0),
            Self::Crate => Currency::dollars(40.0),
            Self::Flask => Currency::dollars(10.0),
            Self::Haversack => Currency::dollars(75.0),
            Self::LargeBackpack => Currency::dollars(100.0),
            Self::LargeChest => Currency::dollars(300.0),
            Self::LargePouch => Currency::dollars(20.0),
            Self::LargeSack => Currency::dollars(50.0),
            Self::MapCase => Currency::dollars(30.0),
            Self::MoneyBelt => Currency::dollars(45.0),
            Self::OilJar => Currency::dollars(25.0),
            Self::PackFrame => Currency::dollars(40.0),
            Self::Pouch => Currency::dollars(10.0),
            Self::PotionBottle => Currency::dollars(15.0),
            Self::PowderHorn => Currency::dollars(20.0),
            Self::Quiver => Currency::dollars(10.0),
            Self::Saddlebags => Currency::dollars(60.0),
            Self::Scabbard => Currency::dollars(20.0),
            Self::ScrollCase => Currency::dollars(25.0),
            Self::SmallBackpack => Currency::dollars(60.0),
            Self::SmallChest => Currency::dollars(100.0),
            Self::SmallPouch => Currency::dollars(10.0),
            Self::SmallSack => Currency::dollars(30.0),
            Self::Strongbox => Currency::dollars(120.0),
            Self::Trunk => Currency::dollars(150.0),
            Self::Vial => Currency::dollars(10.0),
            Self::WaterJar => Currency::dollars(30.0),
            Self::Wineskin => Currency::dollars(10.0),
        }
    }

    /// Returns weight for this container (empty weight).
    #[instrument]
    pub fn weight(&self) -> Weight {
        debug!("Getting container weight");
        match self {
            Self::Amphora => Weight::pounds(5.0),
            Self::Backpack => Weight::pounds(3.0),
            Self::Bandolier => Weight::pounds(1.0),
            Self::Barrel => Weight::pounds(30.0),
            Self::Basket => Weight::pounds(2.0),
            Self::BeltPouch => Weight::pounds(0.25),
            Self::BoltCase => Weight::pounds(1.5),
            Self::BoxSmall => Weight::pounds(2.0),
            Self::Bucket => Weight::pounds(2.5),
            Self::Canteen => Weight::pounds(1.0),
            Self::Coffer => Weight::pounds(8.0),
            Self::CoinPurse => Weight::pounds(0.1),
            Self::Crate => Weight::pounds(20.0),
            Self::Flask => Weight::pounds(0.5),
            Self::Haversack => Weight::pounds(4.0),
            Self::LargeBackpack => Weight::pounds(6.0),
            Self::LargeChest => Weight::pounds(30.0),
            Self::LargePouch => Weight::pounds(0.5),
            Self::LargeSack => Weight::pounds(6.0),
            Self::MapCase => Weight::pounds(0.5),
            Self::MoneyBelt => Weight::pounds(0.5),
            Self::OilJar => Weight::pounds(4.0),
            Self::PackFrame => Weight::pounds(4.0),
            Self::Pouch => Weight::pounds(0.2),
            Self::PotionBottle => Weight::pounds(0.2),
            Self::PowderHorn => Weight::pounds(0.5),
            Self::Quiver => Weight::pounds(0.5),
            Self::Saddlebags => Weight::pounds(5.0),
            Self::Scabbard => Weight::pounds(0.5),
            Self::ScrollCase => Weight::pounds(0.5),
            Self::SmallBackpack => Weight::pounds(3.0),
            Self::SmallChest => Weight::pounds(10.0),
            Self::SmallPouch => Weight::pounds(0.2),
            Self::SmallSack => Weight::pounds(3.0),
            Self::Strongbox => Weight::pounds(15.0),
            Self::Trunk => Weight::pounds(25.0),
            Self::Vial => Weight::pounds(0.1),
            Self::WaterJar => Weight::pounds(5.0),
            Self::Wineskin => Weight::pounds(0.5),
        }
    }

    /// Returns tech level for this container.
    #[instrument]
    pub fn tech_level(&self) -> TechLevel {
        debug!("Getting container tech level");
        match self {
            Self::Amphora => TechLevel::new(1),       // Greek/Roman
            Self::Backpack => TechLevel::new(1),      // Bronze Age
            Self::Bandolier => TechLevel::new(3),     // Medieval/Renaissance
            Self::Barrel => TechLevel::new(0),        // Stone Age
            Self::Basket => TechLevel::new(0),        // Stone Age
            Self::BeltPouch => TechLevel::new(0),     // Stone Age
            Self::BoltCase => TechLevel::new(2),      // Medieval
            Self::BoxSmall => TechLevel::new(1),      // Bronze Age
            Self::Bucket => TechLevel::new(1),        // Bronze Age
            Self::Canteen => TechLevel::new(2),       // Medieval
            Self::Coffer => TechLevel::new(2),        // Medieval
            Self::CoinPurse => TechLevel::new(1),     // Bronze Age
            Self::Crate => TechLevel::new(1),         // Bronze Age
            Self::Flask => TechLevel::new(1),         // Bronze Age
            Self::Haversack => TechLevel::new(3),     // Age of Sail
            Self::LargeBackpack => TechLevel::new(2), // Medieval
            Self::LargeChest => TechLevel::new(1),    // Bronze Age
            Self::LargePouch => TechLevel::new(0),    // Stone Age
            Self::LargeSack => TechLevel::new(0),     // Stone Age
            Self::MapCase => TechLevel::new(2),       // Medieval
            Self::MoneyBelt => TechLevel::new(2),     // Medieval
            Self::OilJar => TechLevel::new(1),        // Bronze Age
            Self::PackFrame => TechLevel::new(0),     // Stone Age
            Self::Pouch => TechLevel::new(0),         // Stone Age
            Self::PotionBottle => TechLevel::new(2),  // Medieval
            Self::PowderHorn => TechLevel::new(4),    // Renaissance
            Self::Quiver => TechLevel::new(0),        // Stone Age
            Self::Saddlebags => TechLevel::new(1),    // Bronze Age
            Self::Scabbard => TechLevel::new(1),      // Bronze Age
            Self::ScrollCase => TechLevel::new(2),    // Medieval
            Self::SmallBackpack => TechLevel::new(1), // Bronze Age
            Self::SmallChest => TechLevel::new(1),    // Bronze Age
            Self::SmallPouch => TechLevel::new(0),    // Stone Age
            Self::SmallSack => TechLevel::new(0),     // Stone Age
            Self::Strongbox => TechLevel::new(3),     // Medieval/Renaissance
            Self::Trunk => TechLevel::new(2),         // Medieval
            Self::Vial => TechLevel::new(1),          // Bronze Age
            Self::WaterJar => TechLevel::new(1),      // Bronze Age
            Self::Wineskin => TechLevel::new(0),      // Stone Age
        }
    }

    /// Returns capacity for this container.
    #[instrument]
    pub fn capacity(&self) -> Capacity {
        debug!("Getting container capacity");
        match self {
            Self::Amphora => Capacity::pounds(30.0),
            Self::Backpack => Capacity::pounds(40.0),
            Self::Bandolier => Capacity::pounds(4.0),
            Self::Barrel => Capacity::pounds(150.0),
            Self::Basket => Capacity::pounds(20.0),
            Self::BeltPouch => Capacity::pounds(2.0),
            Self::BoltCase => Capacity::pounds(5.0),
            Self::BoxSmall => Capacity::pounds(15.0),
            Self::Bucket => Capacity::pounds(10.0),
            Self::Canteen => Capacity::pounds(2.0),
            Self::Coffer => Capacity::pounds(50.0),
            Self::CoinPurse => Capacity::pounds(1.0),
            Self::Crate => Capacity::pounds(120.0),
            Self::Flask => Capacity::pounds(1.0),
            Self::Haversack => Capacity::pounds(50.0),
            Self::LargeBackpack => Capacity::pounds(60.0),
            Self::LargeChest => Capacity::pounds(200.0),
            Self::LargePouch => Capacity::pounds(12.0),
            Self::LargeSack => Capacity::pounds(80.0),
            Self::MapCase => Capacity::pounds(2.0),
            Self::MoneyBelt => Capacity::pounds(1.0),
            Self::OilJar => Capacity::pounds(20.0),
            Self::PackFrame => Capacity::pounds(50.0),
            Self::Pouch => Capacity::pounds(6.0),
            Self::PotionBottle => Capacity::pounds(0.5),
            Self::PowderHorn => Capacity::pounds(1.0),
            Self::Quiver => Capacity::pounds(3.0),
            Self::Saddlebags => Capacity::pounds(40.0),
            Self::Scabbard => Capacity::pounds(1.0),
            Self::ScrollCase => Capacity::pounds(1.0),
            Self::SmallBackpack => Capacity::pounds(40.0),
            Self::SmallChest => Capacity::pounds(100.0),
            Self::SmallPouch => Capacity::pounds(3.0),
            Self::SmallSack => Capacity::pounds(40.0),
            Self::Strongbox => Capacity::pounds(30.0),
            Self::Trunk => Capacity::pounds(150.0),
            Self::Vial => Capacity::pounds(0.5),
            Self::WaterJar => Capacity::pounds(25.0),
            Self::Wineskin => Capacity::pounds(2.0),
        }
    }
}
