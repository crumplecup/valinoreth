//! Survival gear definitions and properties.
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

use crate::{Currency, TechLevel, Weight};
use tracing::{debug, instrument};

/// Survival gear types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, strum::EnumIter)]
pub enum SurvivalGear {
    /// Bedroll, $25. BS 288
    Bedroll,
    /// Blanket, $20. BS 288
    Blanket,
    /// Candle, $0.50. BS 288
    Candle,
    /// Canteen, $10. BS 288
    Canteen,
    /// Cooking pot, $20. BS 288
    CookingPot,
    /// Fishing kit, $5. BS 288
    FishingKit,
    /// Flint and steel, $5. BS 288
    FlintAndSteel,
    /// Lantern, $20. BS 288
    Lantern,
    /// Large tent, $150. BS 288
    LargeTent,
    /// Map, $20. BS 288
    Map,
    /// Matches, $0.50. BS 288
    Matches,
    /// Personal basics (soap, comb, etc.), $5. BS 288
    PersonalBasics,
    /// Rations, $2. BS 288
    Rations,
    /// Signal whistle, $5. BS 288
    SignalWhistle,
    /// Sleeping bag, $40. BS 288
    SleepingBag,
    /// Snare wire, $5. BS 288
    SnareWire,
    /// Tarp, $50. BS 288
    Tarp,
    /// Tent, $50. BS 288
    Tent,
    /// Tinder, $0.50. BS 288
    Tinder,
    /// Torch, $3. BS 288
    Torch,
    /// Waterskin, $10. BS 288
    Waterskin,
}

impl SurvivalGear {
    /// Returns base cost for this survival gear.
    #[instrument]
    pub fn base_cost(&self) -> Currency {
        debug!("Getting survival gear base cost");
        match self {
            Self::Bedroll => Currency::dollars(25.0),
            Self::Blanket => Currency::dollars(20.0),
            Self::Candle => Currency::dollars(0.5),
            Self::Canteen => Currency::dollars(10.0),
            Self::CookingPot => Currency::dollars(20.0),
            Self::FishingKit => Currency::dollars(5.0),
            Self::FlintAndSteel => Currency::dollars(5.0),
            Self::Lantern => Currency::dollars(20.0),
            Self::LargeTent => Currency::dollars(150.0),
            Self::Map => Currency::dollars(20.0),
            Self::Matches => Currency::dollars(0.5),
            Self::PersonalBasics => Currency::dollars(5.0),
            Self::Rations => Currency::dollars(2.0),
            Self::SignalWhistle => Currency::dollars(5.0),
            Self::SleepingBag => Currency::dollars(40.0),
            Self::SnareWire => Currency::dollars(5.0),
            Self::Tarp => Currency::dollars(50.0),
            Self::Tent => Currency::dollars(50.0),
            Self::Tinder => Currency::dollars(0.5),
            Self::Torch => Currency::dollars(3.0),
            Self::Waterskin => Currency::dollars(10.0),
        }
    }

    /// Returns weight for this survival gear.
    #[instrument]
    pub fn weight(&self) -> Weight {
        debug!("Getting survival gear weight");
        match self {
            Self::Bedroll => Weight::pounds(3.0),
            Self::Blanket => Weight::pounds(4.0),
            Self::Candle => Weight::pounds(0.1),
            Self::Canteen => Weight::pounds(0.5), // Empty weight
            Self::CookingPot => Weight::pounds(3.0),
            Self::FishingKit => Weight::pounds(0.5),
            Self::FlintAndSteel => Weight::pounds(0.5),
            Self::Lantern => Weight::pounds(2.0),
            Self::LargeTent => Weight::pounds(30.0),
            Self::Map => Weight::pounds(0.1),
            Self::Matches => Weight::pounds(0.1),
            Self::PersonalBasics => Weight::pounds(1.0),
            Self::Rations => Weight::pounds(0.5),
            Self::SignalWhistle => Weight::pounds(0.1),
            Self::SleepingBag => Weight::pounds(8.0),
            Self::SnareWire => Weight::pounds(0.5),
            Self::Tarp => Weight::pounds(5.0),
            Self::Tent => Weight::pounds(5.0),
            Self::Tinder => Weight::pounds(0.1),
            Self::Torch => Weight::pounds(1.0),
            Self::Waterskin => Weight::pounds(0.25), // Empty weight
        }
    }

    /// Returns tech level for this survival gear.
    #[instrument]
    pub fn tech_level(&self) -> TechLevel {
        debug!("Getting survival gear tech level");
        match self {
            Self::Bedroll => TechLevel::new(0),       // Stone Age
            Self::Blanket => TechLevel::new(0),       // Stone Age
            Self::Candle => TechLevel::new(1),        // Bronze Age
            Self::Canteen => TechLevel::new(3),       // Medieval (metal canteen)
            Self::CookingPot => TechLevel::new(0),    // Stone Age (stone pot)
            Self::FishingKit => TechLevel::new(0),    // Stone Age
            Self::FlintAndSteel => TechLevel::new(0), // Stone Age
            Self::Lantern => TechLevel::new(2),       // Medieval (oil lantern)
            Self::LargeTent => TechLevel::new(1),     // Bronze Age
            Self::Map => TechLevel::new(3),           // Medieval
            Self::Matches => TechLevel::new(5),       // Industrial
            Self::PersonalBasics => TechLevel::new(0), // Stone Age
            Self::Rations => TechLevel::new(0),       // Stone Age (preserved food)
            Self::SignalWhistle => TechLevel::new(2), // Medieval
            Self::SleepingBag => TechLevel::new(5),   // Industrial (synthetic insulation)
            Self::SnareWire => TechLevel::new(1),     // Bronze Age
            Self::Tarp => TechLevel::new(5),          // Industrial (waterproof fabric)
            Self::Tent => TechLevel::new(0),          // Stone Age
            Self::Tinder => TechLevel::new(0),        // Stone Age
            Self::Torch => TechLevel::new(0),         // Stone Age
            Self::Waterskin => TechLevel::new(0),     // Stone Age
        }
    }
}
