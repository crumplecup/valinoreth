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
use tracing::debug;
#[cfg(not(creusot))]
use tracing::instrument;

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
    /// Burnoose, $40. LT (North African hooded cloak, TL2)
    Burnoose,
    /// Cape, $20. BS 266
    Cape,
    /// Cassock, $45. LT (Religious vestment, TL2)
    Cassock,
    /// Changshan, $50. LT (Chinese long gown, TL2)
    Changshan,
    /// Chiton, $25. LT (Greek tunic, TL1)
    Chiton,
    /// Cloak, $50. BS 266
    Cloak,
    /// Coat, $75. BS 266
    Coat,
    /// Cowl, $12. LT (Monk's hood, TL2)
    Cowl,
    /// Generic clothing, $120. BS 266
    Clothing,
    /// Dhoti, $15. LT (Indian wrapped lower garment, TL1)
    Dhoti,
    /// Doublet, $40. BS 266
    Doublet,
    /// Dress, $40. BS 266
    Dress,
    /// Fine gloves, $40. BS 266
    FineGloves,
    /// Gloves, $30. BS 266
    Gloves,
    /// Gi, $35. LT (Martial arts uniform, TL2)
    Gi,
    /// Hakama, $40. LT (Japanese wide-legged pants, TL2)
    Hakama,
    /// Hanfu, $60. LT (Chinese traditional dress, TL1)
    Hanfu,
    /// Hat, $10. BS 266
    Hat,
    /// Heavy boots, $100. BS 266
    HeavyBoots,
    /// Hood, $5. BS 266
    Hood,
    /// Jacket, $50. BS 266
    Jacket,
    /// Jerkin, $40. LT (Leather vest/jacket, TL2)
    Jerkin,
    /// Kaftan, $70. LT (Ottoman/Persian coat, TL2)
    Kaftan,
    /// Keffiyeh, $8. LT (Middle Eastern headscarf, TL1)
    Keffiyeh,
    /// Kimono, $80. LT (Japanese traditional robe, TL2)
    Kimono,
    /// Loincloth, $5. LT (Simple undergarment, TL0)
    Loincloth,
    /// Mantle, $60. BS 266
    Mantle,
    /// Obi, $20. LT (Japanese wide sash/belt, TL2)
    Obi,
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
    /// Sari, $30. LT (Indian draped garment, TL1)
    Sari,
    /// Shirt, $20. BS 266
    Shirt,
    /// Stockings, $5. BS 266
    Stockings,
    /// Surcoat, $50. LT (Medieval overgarment, TL2)
    Surcoat,
    /// Tabard, $25. BS 266
    Tabard,
    /// Thawb, $35. LT (Arab long robe, TL1)
    Thawb,
    /// Toga, $30. LT (Roman draped garment, TL1)
    Toga,
    /// Tunic, $30. BS 266
    Tunic,
    /// Turban, $10. LT (Middle Eastern/Asian headwear, TL1)
    Turban,
    /// Vest, $25. BS 266
    Vest,
    /// Wimple, $5. BS 266
    Wimple,
}

impl Clothing {
    /// Returns base cost for this clothing.
    #[cfg_attr(not(creusot), instrument)]
    pub fn base_cost(&self) -> Currency {
        debug!("Getting clothing base cost");
        match self {
            Self::Apron => Currency::dollars(15.0),
            Self::Belt => Currency::dollars(15.0),
            Self::Boots => Currency::dollars(80.0),
            Self::Breeches => Currency::dollars(35.0),
            Self::Burnoose => Currency::dollars(40.0),
            Self::Cape => Currency::dollars(20.0),
            Self::Cassock => Currency::dollars(45.0),
            Self::Changshan => Currency::dollars(50.0),
            Self::Chiton => Currency::dollars(25.0),
            Self::Cloak => Currency::dollars(50.0),
            Self::Coat => Currency::dollars(75.0),
            Self::Cowl => Currency::dollars(12.0),
            Self::Clothing => Currency::dollars(120.0),
            Self::Dhoti => Currency::dollars(15.0),
            Self::Doublet => Currency::dollars(40.0),
            Self::Dress => Currency::dollars(40.0),
            Self::FineGloves => Currency::dollars(40.0),
            Self::Gloves => Currency::dollars(30.0),
            Self::Gi => Currency::dollars(35.0),
            Self::Hakama => Currency::dollars(40.0),
            Self::Hanfu => Currency::dollars(60.0),
            Self::Hat => Currency::dollars(10.0),
            Self::HeavyBoots => Currency::dollars(100.0),
            Self::Hood => Currency::dollars(5.0),
            Self::Jacket => Currency::dollars(50.0),
            Self::Jerkin => Currency::dollars(40.0),
            Self::Kaftan => Currency::dollars(70.0),
            Self::Keffiyeh => Currency::dollars(8.0),
            Self::Kimono => Currency::dollars(80.0),
            Self::Loincloth => Currency::dollars(5.0),
            Self::Mantle => Currency::dollars(60.0),
            Self::Obi => Currency::dollars(20.0),
            Self::Pants => Currency::dollars(30.0),
            Self::ReinforcedGloves => Currency::dollars(50.0),
            Self::RidingBoots => Currency::dollars(120.0),
            Self::Robe => Currency::dollars(20.0),
            Self::Sandals => Currency::dollars(25.0),
            Self::Sash => Currency::dollars(10.0),
            Self::Scarf => Currency::dollars(5.0),
            Self::Sari => Currency::dollars(30.0),
            Self::Shirt => Currency::dollars(20.0),
            Self::Stockings => Currency::dollars(5.0),
            Self::Surcoat => Currency::dollars(50.0),
            Self::Tabard => Currency::dollars(25.0),
            Self::Thawb => Currency::dollars(35.0),
            Self::Toga => Currency::dollars(30.0),
            Self::Tunic => Currency::dollars(30.0),
            Self::Turban => Currency::dollars(10.0),
            Self::Vest => Currency::dollars(25.0),
            Self::Wimple => Currency::dollars(5.0),
        }
    }

    /// Returns weight for this clothing.
    #[cfg_attr(not(creusot), instrument)]
    pub fn weight(&self) -> Weight {
        debug!("Getting clothing weight");
        match self {
            Self::Apron => Weight::pounds(0.75),
            Self::Belt => Weight::pounds(0.25),
            Self::Boots => Weight::pounds(2.0),
            Self::Breeches => Weight::pounds(1.5),
            Self::Burnoose => Weight::pounds(3.0),
            Self::Cape => Weight::pounds(1.0),
            Self::Cassock => Weight::pounds(2.5),
            Self::Changshan => Weight::pounds(2.0),
            Self::Chiton => Weight::pounds(1.0),
            Self::Cloak => Weight::pounds(4.0),
            Self::Coat => Weight::pounds(4.0),
            Self::Cowl => Weight::pounds(0.5),
            Self::Clothing => Weight::pounds(2.0),
            Self::Dhoti => Weight::pounds(0.5),
            Self::Doublet => Weight::pounds(1.5),
            Self::Dress => Weight::pounds(2.0),
            Self::FineGloves => Weight::pounds(0.25),
            Self::Gloves => Weight::pounds(0.5),
            Self::Gi => Weight::pounds(1.5),
            Self::Hakama => Weight::pounds(1.5),
            Self::Hanfu => Weight::pounds(2.5),
            Self::Hat => Weight::pounds(0.5),
            Self::HeavyBoots => Weight::pounds(3.0),
            Self::Hood => Weight::pounds(0.25),
            Self::Jacket => Weight::pounds(2.0),
            Self::Jerkin => Weight::pounds(2.0),
            Self::Kaftan => Weight::pounds(2.5),
            Self::Keffiyeh => Weight::pounds(0.25),
            Self::Kimono => Weight::pounds(3.0),
            Self::Loincloth => Weight::pounds(0.1),
            Self::Mantle => Weight::pounds(3.0),
            Self::Obi => Weight::pounds(0.75),
            Self::Pants => Weight::pounds(1.0),
            Self::ReinforcedGloves => Weight::pounds(0.75),
            Self::RidingBoots => Weight::pounds(3.0),
            Self::Robe => Weight::pounds(2.0),
            Self::Sandals => Weight::pounds(0.5),
            Self::Sash => Weight::pounds(0.5),
            Self::Scarf => Weight::pounds(0.1),
            Self::Sari => Weight::pounds(1.5),
            Self::Shirt => Weight::pounds(0.5),
            Self::Stockings => Weight::pounds(0.25),
            Self::Surcoat => Weight::pounds(2.5),
            Self::Tabard => Weight::pounds(1.0),
            Self::Thawb => Weight::pounds(2.0),
            Self::Toga => Weight::pounds(1.5),
            Self::Tunic => Weight::pounds(1.0),
            Self::Turban => Weight::pounds(0.5),
            Self::Vest => Weight::pounds(0.5),
            Self::Wimple => Weight::pounds(0.25),
        }
    }

    /// Returns tech level for this clothing.
    #[cfg_attr(not(creusot), instrument)]
    pub fn tech_level(&self) -> TechLevel {
        debug!("Getting clothing tech level");
        match self {
            Self::Apron => TechLevel::new(1),            // Bronze Age
            Self::Belt => TechLevel::new(0),             // Stone Age
            Self::Boots => TechLevel::new(1),            // Bronze Age
            Self::Breeches => TechLevel::new(2),         // Medieval
            Self::Burnoose => TechLevel::new(2),         // North African/Medieval
            Self::Cape => TechLevel::new(0),             // Stone Age
            Self::Cassock => TechLevel::new(2),          // Medieval
            Self::Changshan => TechLevel::new(2),        // Chinese Medieval
            Self::Chiton => TechLevel::new(1),           // Greek Bronze Age
            Self::Cloak => TechLevel::new(1),            // Bronze Age
            Self::Coat => TechLevel::new(4),             // Renaissance
            Self::Cowl => TechLevel::new(2),             // Medieval
            Self::Clothing => TechLevel::new(1),         // Bronze Age
            Self::Dhoti => TechLevel::new(1),            // Indian Bronze Age
            Self::Doublet => TechLevel::new(3),          // Age of Sail
            Self::Dress => TechLevel::new(1),            // Bronze Age
            Self::FineGloves => TechLevel::new(2),       // Medieval
            Self::Gloves => TechLevel::new(1),           // Bronze Age
            Self::Gi => TechLevel::new(2),               // Japanese/Chinese Medieval
            Self::Hakama => TechLevel::new(2),           // Japanese Medieval
            Self::Hanfu => TechLevel::new(1),            // Chinese Bronze Age
            Self::Hat => TechLevel::new(1),              // Bronze Age
            Self::HeavyBoots => TechLevel::new(2),       // Medieval
            Self::Hood => TechLevel::new(0),             // Stone Age
            Self::Jacket => TechLevel::new(3),           // Age of Sail
            Self::Jerkin => TechLevel::new(2),           // Medieval
            Self::Kaftan => TechLevel::new(2),           // Ottoman/Persian
            Self::Keffiyeh => TechLevel::new(1),         // Middle Eastern Bronze Age
            Self::Kimono => TechLevel::new(2),           // Japanese Medieval
            Self::Loincloth => TechLevel::new(0),        // Stone Age
            Self::Mantle => TechLevel::new(2),           // Medieval
            Self::Obi => TechLevel::new(2),              // Japanese Medieval
            Self::Pants => TechLevel::new(1),            // Bronze Age
            Self::ReinforcedGloves => TechLevel::new(2), // Medieval
            Self::RidingBoots => TechLevel::new(2),      // Medieval
            Self::Robe => TechLevel::new(1),             // Bronze Age
            Self::Sandals => TechLevel::new(0),          // Stone Age
            Self::Sash => TechLevel::new(1),             // Bronze Age
            Self::Scarf => TechLevel::new(0),            // Stone Age
            Self::Sari => TechLevel::new(1),             // Indian Bronze Age
            Self::Shirt => TechLevel::new(1),            // Bronze Age
            Self::Stockings => TechLevel::new(1),        // Bronze Age
            Self::Surcoat => TechLevel::new(2),          // Medieval
            Self::Tabard => TechLevel::new(2),           // Medieval
            Self::Thawb => TechLevel::new(1),            // Arab Bronze Age
            Self::Toga => TechLevel::new(1),             // Roman Bronze Age
            Self::Tunic => TechLevel::new(0),            // Stone Age
            Self::Turban => TechLevel::new(1),           // Middle Eastern/Asian Bronze Age
            Self::Vest => TechLevel::new(1),             // Bronze Age
            Self::Wimple => TechLevel::new(2),           // Medieval
        }
    }
}
