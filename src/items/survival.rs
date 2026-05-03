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
    /// Bronze mirror, $25. LT (Polished bronze, TL1)
    BronzeMirror,
    /// Candle, $0.50. BS 288
    Candle,
    /// Camp bed, $50. LT (Portable cot, TL2)
    CampBed,
    /// Camp stool, $15. LT (Folding stool, TL1)
    CampStool,
    /// Canteen, $10. BS 288
    Canteen,
    /// Cooking pot, $20. BS 288
    CookingPot,
    /// Dried meat (1 lb), $3. LT (Preserved meat, TL0)
    DriedMeat,
    /// Fire drill, $3. LT (Bow drill kit, TL0)
    FireDrill,
    /// Fishing kit, $5. BS 288
    FishingKit,
    /// Flint and steel, $5. BS 288
    FlintAndSteel,
    /// Fur blanket, $40. LT (Animal fur, TL0)
    FurBlanket,
    /// Hammock, $20. LT (Portable bed, TL0)
    Hammock,
    /// Lantern, $20. BS 288
    Lantern,
    /// Large tent, $150. BS 288
    LargeTent,
    /// Map, $20. BS 288
    Map,
    /// Matches, $0.50. BS 288
    Matches,
    /// Mess kit, $15. LT (Portable eating utensils, TL1)
    MessKit,
    /// Oil lamp, $10. LT (Simple clay/bronze lamp, TL1)
    OilLamp,
    /// Personal basics (soap, comb, etc.), $5. BS 288
    PersonalBasics,
    /// Pilgrim staff, $10. LT (Walking staff, TL0)
    PilgrimStaff,
    /// Rations, $2. BS 288
    Rations,
    /// Signal horn, $15. LT (Signaling horn, TL0)
    SignalHorn,
    /// Signal whistle, $5. BS 288
    SignalWhistle,
    /// Sleeping bag, $40. BS 288
    SleepingBag,
    /// Storm lantern, $30. LT (Weather-resistant lantern, TL3)
    StormLantern,
    /// Snare wire, $5. BS 288
    SnareWire,
    /// Tarp, $50. BS 288
    Tarp,
    /// Tent, $50. BS 288
    Tent,
    /// Tinderbox, $8. LT (Fire starting kit, TL1)
    Tinderbox,
    /// Tinder, $0.50. BS 288
    Tinder,
    /// Torch, $3. BS 288
    Torch,
    /// Travel rations (1 day), $3. LT (Trail food, TL0)
    TravelRations,
    /// Water bag (large), $15. LT (Large water carrier, TL0)
    WaterBag,
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
            Self::BronzeMirror => Currency::dollars(25.0),
            Self::CampBed => Currency::dollars(50.0),
            Self::CampStool => Currency::dollars(15.0),
            Self::Candle => Currency::dollars(0.5),
            Self::Canteen => Currency::dollars(10.0),
            Self::CookingPot => Currency::dollars(20.0),
            Self::DriedMeat => Currency::dollars(3.0),
            Self::FireDrill => Currency::dollars(3.0),
            Self::FishingKit => Currency::dollars(5.0),
            Self::FlintAndSteel => Currency::dollars(5.0),
            Self::FurBlanket => Currency::dollars(40.0),
            Self::Hammock => Currency::dollars(20.0),
            Self::Lantern => Currency::dollars(20.0),
            Self::LargeTent => Currency::dollars(150.0),
            Self::Map => Currency::dollars(20.0),
            Self::Matches => Currency::dollars(0.5),
            Self::MessKit => Currency::dollars(15.0),
            Self::OilLamp => Currency::dollars(10.0),
            Self::PersonalBasics => Currency::dollars(5.0),
            Self::PilgrimStaff => Currency::dollars(10.0),
            Self::Rations => Currency::dollars(2.0),
            Self::SignalHorn => Currency::dollars(15.0),
            Self::SignalWhistle => Currency::dollars(5.0),
            Self::SleepingBag => Currency::dollars(40.0),
            Self::StormLantern => Currency::dollars(30.0),
            Self::SnareWire => Currency::dollars(5.0),
            Self::Tarp => Currency::dollars(50.0),
            Self::Tent => Currency::dollars(50.0),
            Self::Tinder => Currency::dollars(0.5),
            Self::Tinderbox => Currency::dollars(8.0),
            Self::Torch => Currency::dollars(3.0),
            Self::TravelRations => Currency::dollars(3.0),
            Self::WaterBag => Currency::dollars(15.0),
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
            Self::BronzeMirror => Weight::pounds(1.0),
            Self::CampBed => Weight::pounds(15.0),
            Self::CampStool => Weight::pounds(3.0),
            Self::Candle => Weight::pounds(0.1),
            Self::Canteen => Weight::pounds(0.5), // Empty weight
            Self::CookingPot => Weight::pounds(3.0),
            Self::DriedMeat => Weight::pounds(1.0),
            Self::FireDrill => Weight::pounds(1.0),
            Self::FishingKit => Weight::pounds(0.5),
            Self::FlintAndSteel => Weight::pounds(0.5),
            Self::FurBlanket => Weight::pounds(8.0),
            Self::Hammock => Weight::pounds(2.0),
            Self::Lantern => Weight::pounds(2.0),
            Self::LargeTent => Weight::pounds(30.0),
            Self::Map => Weight::pounds(0.1),
            Self::Matches => Weight::pounds(0.1),
            Self::MessKit => Weight::pounds(2.0),
            Self::OilLamp => Weight::pounds(2.0),
            Self::PersonalBasics => Weight::pounds(1.0),
            Self::PilgrimStaff => Weight::pounds(3.0),
            Self::Rations => Weight::pounds(0.5),
            Self::SignalHorn => Weight::pounds(2.0),
            Self::SignalWhistle => Weight::pounds(0.1),
            Self::SleepingBag => Weight::pounds(8.0),
            Self::StormLantern => Weight::pounds(3.0),
            Self::SnareWire => Weight::pounds(0.5),
            Self::Tarp => Weight::pounds(5.0),
            Self::Tent => Weight::pounds(5.0),
            Self::Tinder => Weight::pounds(0.1),
            Self::Tinderbox => Weight::pounds(1.0),
            Self::Torch => Weight::pounds(1.0),
            Self::TravelRations => Weight::pounds(1.0),
            Self::WaterBag => Weight::pounds(1.0), // Empty weight
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
            Self::BronzeMirror => TechLevel::new(1),  // Bronze Age
            Self::CampBed => TechLevel::new(2),       // Medieval
            Self::CampStool => TechLevel::new(1),     // Bronze Age
            Self::Candle => TechLevel::new(1),        // Bronze Age
            Self::Canteen => TechLevel::new(3),       // Medieval (metal canteen)
            Self::CookingPot => TechLevel::new(0),    // Stone Age (stone pot)
            Self::DriedMeat => TechLevel::new(0),     // Stone Age
            Self::FireDrill => TechLevel::new(0),     // Stone Age
            Self::FishingKit => TechLevel::new(0),    // Stone Age
            Self::FlintAndSteel => TechLevel::new(0), // Stone Age
            Self::FurBlanket => TechLevel::new(0),    // Stone Age
            Self::Hammock => TechLevel::new(0),       // Stone Age
            Self::Lantern => TechLevel::new(2),       // Medieval (oil lantern)
            Self::LargeTent => TechLevel::new(1),     // Bronze Age
            Self::Map => TechLevel::new(3),           // Medieval
            Self::Matches => TechLevel::new(5),       // Industrial
            Self::MessKit => TechLevel::new(1),       // Bronze Age
            Self::OilLamp => TechLevel::new(1),       // Bronze Age
            Self::PersonalBasics => TechLevel::new(0), // Stone Age
            Self::PilgrimStaff => TechLevel::new(0),  // Stone Age
            Self::Rations => TechLevel::new(0),       // Stone Age (preserved food)
            Self::SignalHorn => TechLevel::new(0),    // Stone Age
            Self::SignalWhistle => TechLevel::new(2), // Medieval
            Self::SleepingBag => TechLevel::new(5),   // Industrial (synthetic insulation)
            Self::StormLantern => TechLevel::new(3),  // Medieval
            Self::SnareWire => TechLevel::new(1),     // Bronze Age
            Self::Tarp => TechLevel::new(5),          // Industrial (waterproof fabric)
            Self::Tent => TechLevel::new(0),          // Stone Age
            Self::Tinderbox => TechLevel::new(1),     // Bronze Age
            Self::Tinder => TechLevel::new(0),        // Stone Age
            Self::Torch => TechLevel::new(0),         // Stone Age
            Self::TravelRations => TechLevel::new(0), // Stone Age
            Self::WaterBag => TechLevel::new(0),      // Stone Age
            Self::Waterskin => TechLevel::new(0),     // Stone Age
        }
    }
}
