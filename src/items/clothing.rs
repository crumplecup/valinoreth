//! Clothing definitions and properties.
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

use crate::{Currency, TechLevel, Weight};
use tracing::{debug, instrument};

/// Clothing types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, strum::EnumIter)]
pub enum Clothing {
    /// Apron, $15. BS 266
    Apron,
    /// Belt, $15. BS 266
    Belt,
    /// Boots, $80. BS 266
    Boots,
    /// Breeches, $35. BS 266
    Breeches,
    /// Cape, $20. BS 266
    Cape,
    /// Cloak, $50. BS 266
    Cloak,
    /// Coat, $75. BS 266
    Coat,
    /// Generic clothing, $120. BS 266
    Clothing,
    /// Doublet, $40. BS 266
    Doublet,
    /// Dress, $40. BS 266
    Dress,
    /// Fine gloves, $40. BS 266
    FineGloves,
    /// Gloves, $30. BS 266
    Gloves,
    /// Hat, $10. BS 266
    Hat,
    /// Heavy boots, $100. BS 266
    HeavyBoots,
    /// Hood, $5. BS 266
    Hood,
    /// Jacket, $50. BS 266
    Jacket,
    /// Mantle, $60. BS 266
    Mantle,
    /// Pants, $30. BS 266
    Pants,
    /// Reinforced gloves, $50. BS 266
    ReinforcedGloves,
    /// Riding boots, $120. BS 266
    RidingBoots,
    /// Robe, $20. BS 266
    Robe,
    /// Sandals, $25. BS 266
    Sandals,
    /// Sash, $10. BS 266
    Sash,
    /// Scarf, $5. BS 266
    Scarf,
    /// Shirt, $20. BS 266
    Shirt,
    /// Stockings, $5. BS 266
    Stockings,
    /// Tabard, $25. BS 266
    Tabard,
    /// Tunic, $30. BS 266
    Tunic,
    /// Vest, $25. BS 266
    Vest,
    /// Wimple, $5. BS 266
    Wimple,
}

impl Clothing {
    /// Returns base cost for this clothing.
    #[instrument]
    pub fn base_cost(&self) -> Currency {
        debug!("Getting clothing base cost");
        match self {
            Self::Apron => Currency::dollars(15.0),
            Self::Belt => Currency::dollars(15.0),
            Self::Boots => Currency::dollars(80.0),
            Self::Breeches => Currency::dollars(35.0),
            Self::Cape => Currency::dollars(20.0),
            Self::Cloak => Currency::dollars(50.0),
            Self::Coat => Currency::dollars(75.0),
            Self::Clothing => Currency::dollars(120.0),
            Self::Doublet => Currency::dollars(40.0),
            Self::Dress => Currency::dollars(40.0),
            Self::FineGloves => Currency::dollars(40.0),
            Self::Gloves => Currency::dollars(30.0),
            Self::Hat => Currency::dollars(10.0),
            Self::HeavyBoots => Currency::dollars(100.0),
            Self::Hood => Currency::dollars(5.0),
            Self::Jacket => Currency::dollars(50.0),
            Self::Mantle => Currency::dollars(60.0),
            Self::Pants => Currency::dollars(30.0),
            Self::ReinforcedGloves => Currency::dollars(50.0),
            Self::RidingBoots => Currency::dollars(120.0),
            Self::Robe => Currency::dollars(20.0),
            Self::Sandals => Currency::dollars(25.0),
            Self::Sash => Currency::dollars(10.0),
            Self::Scarf => Currency::dollars(5.0),
            Self::Shirt => Currency::dollars(20.0),
            Self::Stockings => Currency::dollars(5.0),
            Self::Tabard => Currency::dollars(25.0),
            Self::Tunic => Currency::dollars(30.0),
            Self::Vest => Currency::dollars(25.0),
            Self::Wimple => Currency::dollars(5.0),
        }
    }

    /// Returns weight for this clothing.
    #[instrument]
    pub fn weight(&self) -> Weight {
        debug!("Getting clothing weight");
        match self {
            Self::Apron => Weight::pounds(0.75),
            Self::Belt => Weight::pounds(0.25),
            Self::Boots => Weight::pounds(2.0),
            Self::Breeches => Weight::pounds(1.5),
            Self::Cape => Weight::pounds(1.0),
            Self::Cloak => Weight::pounds(4.0),
            Self::Coat => Weight::pounds(4.0),
            Self::Clothing => Weight::pounds(2.0),
            Self::Doublet => Weight::pounds(1.5),
            Self::Dress => Weight::pounds(2.0),
            Self::FineGloves => Weight::pounds(0.25),
            Self::Gloves => Weight::pounds(0.5),
            Self::Hat => Weight::pounds(0.5),
            Self::HeavyBoots => Weight::pounds(3.0),
            Self::Hood => Weight::pounds(0.25),
            Self::Jacket => Weight::pounds(2.0),
            Self::Mantle => Weight::pounds(3.0),
            Self::Pants => Weight::pounds(1.0),
            Self::ReinforcedGloves => Weight::pounds(0.75),
            Self::RidingBoots => Weight::pounds(3.0),
            Self::Robe => Weight::pounds(2.0),
            Self::Sandals => Weight::pounds(0.5),
            Self::Sash => Weight::pounds(0.5),
            Self::Scarf => Weight::pounds(0.1),
            Self::Shirt => Weight::pounds(0.5),
            Self::Stockings => Weight::pounds(0.25),
            Self::Tabard => Weight::pounds(1.0),
            Self::Tunic => Weight::pounds(1.0),
            Self::Vest => Weight::pounds(0.5),
            Self::Wimple => Weight::pounds(0.25),
        }
    }

    /// Returns tech level for this clothing.
    #[instrument]
    pub fn tech_level(&self) -> TechLevel {
        debug!("Getting clothing tech level");
        match self {
            Self::Apron => TechLevel::new(1),            // Bronze Age
            Self::Belt => TechLevel::new(0),             // Stone Age
            Self::Boots => TechLevel::new(1),            // Bronze Age
            Self::Breeches => TechLevel::new(2),         // Medieval
            Self::Cape => TechLevel::new(0),             // Stone Age
            Self::Cloak => TechLevel::new(1),            // Bronze Age
            Self::Coat => TechLevel::new(4),             // Renaissance
            Self::Clothing => TechLevel::new(1),         // Bronze Age
            Self::Doublet => TechLevel::new(3),          // Age of Sail
            Self::Dress => TechLevel::new(1),            // Bronze Age
            Self::FineGloves => TechLevel::new(2),       // Medieval
            Self::Gloves => TechLevel::new(1),           // Bronze Age
            Self::Hat => TechLevel::new(1),              // Bronze Age
            Self::HeavyBoots => TechLevel::new(2),       // Medieval
            Self::Hood => TechLevel::new(0),             // Stone Age
            Self::Jacket => TechLevel::new(3),           // Age of Sail
            Self::Mantle => TechLevel::new(2),           // Medieval
            Self::Pants => TechLevel::new(1),            // Bronze Age
            Self::ReinforcedGloves => TechLevel::new(2), // Medieval
            Self::RidingBoots => TechLevel::new(2),      // Medieval
            Self::Robe => TechLevel::new(1),             // Bronze Age
            Self::Sandals => TechLevel::new(0),          // Stone Age
            Self::Sash => TechLevel::new(1),             // Bronze Age
            Self::Scarf => TechLevel::new(0),            // Stone Age
            Self::Shirt => TechLevel::new(1),            // Bronze Age
            Self::Stockings => TechLevel::new(1),        // Bronze Age
            Self::Tabard => TechLevel::new(2),           // Medieval
            Self::Tunic => TechLevel::new(0),            // Stone Age
            Self::Vest => TechLevel::new(1),             // Bronze Age
            Self::Wimple => TechLevel::new(2),           // Medieval
        }
    }
}
