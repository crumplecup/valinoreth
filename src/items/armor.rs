//! Armor definitions and properties.
//!
//! # GURPS Rules
//!
//! Armor provides damage resistance (DR) which reduces incoming damage
//! before wound multipliers are applied. Heavier armor provides more
//! protection but weighs more and costs more.
//!
//! # Citations
//!
//! BS 279-282 - Armor table

use crate::{Currency, TechLevel, Weight};
use tracing::{debug, instrument};

/// Armor types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, strum::EnumIter)]
pub enum Armor {
    /// Ballistic vest, DR 10. BS 279
    BallisticVest,
    /// Banded mail, DR 4. BS 279
    BandedMail,
    /// Brigandine, DR 4. BS 279
    Brigandine,
    /// Bronze plate, DR 5. BS 279
    BronzePlate,
    /// Buckler, DR 1. BS 282
    Buckler,
    /// Chainmail, DR 4. BS 279
    Chainmail,
    /// Cloth armor, DR 1. BS 279
    ClothArmor,
    /// Flak jacket, DR 7. BS 279
    FlakJacket,
    /// Half plate, DR 5. BS 279
    HalfPlate,
    /// Heavy leather, DR 2. BS 279
    HeavyLeather,
    /// Heavy plate, DR 8. BS 279
    HeavyPlate,
    /// Lamellar armor, DR 4. BS 279
    LamellarArmor,
    /// Large shield, DR 3. BS 282
    LargeShield,
    /// Leather armor, DR 1. BS 279
    LeatherArmor,
    /// Light leather, DR 1. BS 279
    LightLeather,
    /// Light scale, DR 3. BS 279
    LightScale,
    /// Mail hauberk, DR 4. BS 279
    MailHauberk,
    /// Mail shirt, DR 4. BS 279
    MailShirt,
    /// Medium shield, DR 2. BS 282
    MediumShield,
    /// No armor, DR 0. BS 279
    NoArmor,
    /// Plate armor, DR 6. BS 279
    PlateArmor,
    /// Ring mail, DR 3. BS 279
    RingMail,
    /// Scale mail, DR 4. BS 279
    ScaleMail,
    /// Small shield, DR 1. BS 282
    SmallShield,
    /// Splint mail, DR 5. BS 279
    SplintMail,
    /// Tactical vest, DR 18. BS 279
    TacticalVest,
    /// Tower shield, DR 4. BS 282
    TowerShield,
}

impl Armor {
    /// Returns base cost for this armor.
    #[instrument]
    pub fn base_cost(&self) -> Currency {
        debug!("Getting armor base cost");
        match self {
            Self::BallisticVest => Currency::dollars(400.0),
            Self::BandedMail => Currency::dollars(500.0),
            Self::Brigandine => Currency::dollars(500.0),
            Self::BronzePlate => Currency::dollars(2400.0),
            Self::Buckler => Currency::dollars(25.0),
            Self::Chainmail => Currency::dollars(550.0),
            Self::ClothArmor => Currency::dollars(30.0),
            Self::FlakJacket => Currency::dollars(500.0),
            Self::HalfPlate => Currency::dollars(1500.0),
            Self::HeavyLeather => Currency::dollars(150.0),
            Self::HeavyPlate => Currency::dollars(6000.0),
            Self::LamellarArmor => Currency::dollars(500.0),
            Self::LargeShield => Currency::dollars(90.0),
            Self::LeatherArmor => Currency::dollars(100.0),
            Self::LightLeather => Currency::dollars(50.0),
            Self::LightScale => Currency::dollars(280.0),
            Self::MailHauberk => Currency::dollars(600.0),
            Self::MailShirt => Currency::dollars(350.0),
            Self::MediumShield => Currency::dollars(60.0),
            Self::NoArmor => Currency::dollars(0.0),
            Self::PlateArmor => Currency::dollars(3000.0),
            Self::RingMail => Currency::dollars(300.0),
            Self::ScaleMail => Currency::dollars(420.0),
            Self::SmallShield => Currency::dollars(40.0),
            Self::SplintMail => Currency::dollars(700.0),
            Self::TacticalVest => Currency::dollars(900.0),
            Self::TowerShield => Currency::dollars(150.0),
        }
    }

    /// Returns weight for this armor.
    #[instrument]
    pub fn weight(&self) -> Weight {
        debug!("Getting armor weight");
        match self {
            Self::BallisticVest => Weight::pounds(2.0),
            Self::BandedMail => Weight::pounds(35.0),
            Self::Brigandine => Weight::pounds(25.0),
            Self::BronzePlate => Weight::pounds(60.0),
            Self::Buckler => Weight::pounds(5.0),
            Self::Chainmail => Weight::pounds(35.0),
            Self::ClothArmor => Weight::pounds(6.0),
            Self::FlakJacket => Weight::pounds(20.0),
            Self::HalfPlate => Weight::pounds(30.0),
            Self::HeavyLeather => Weight::pounds(15.0),
            Self::HeavyPlate => Weight::pounds(60.0),
            Self::LamellarArmor => Weight::pounds(35.0),
            Self::LargeShield => Weight::pounds(25.0),
            Self::LeatherArmor => Weight::pounds(10.0),
            Self::LightLeather => Weight::pounds(5.0),
            Self::LightScale => Weight::pounds(28.0),
            Self::MailHauberk => Weight::pounds(45.0),
            Self::MailShirt => Weight::pounds(25.0),
            Self::MediumShield => Weight::pounds(15.0),
            Self::NoArmor => Weight::pounds(0.0),
            Self::PlateArmor => Weight::pounds(50.0),
            Self::RingMail => Weight::pounds(25.0),
            Self::ScaleMail => Weight::pounds(42.0),
            Self::SmallShield => Weight::pounds(8.0),
            Self::SplintMail => Weight::pounds(45.0),
            Self::TacticalVest => Weight::pounds(9.0),
            Self::TowerShield => Weight::pounds(45.0),
        }
    }

    /// Returns tech level for this armor.
    #[instrument]
    pub fn tech_level(&self) -> TechLevel {
        debug!("Getting armor tech level");
        match self {
            Self::BallisticVest => TechLevel::new(7), // Digital Age
            Self::BandedMail => TechLevel::new(3),    // Medieval
            Self::Brigandine => TechLevel::new(3),    // Age of Sail
            Self::BronzePlate => TechLevel::new(1),   // Bronze Age
            Self::Buckler => TechLevel::new(2),       // Medieval
            Self::Chainmail => TechLevel::new(2),     // Medieval
            Self::ClothArmor => TechLevel::new(1),    // Bronze Age
            Self::FlakJacket => TechLevel::new(6),    // Atomic Age
            Self::HalfPlate => TechLevel::new(3),     // Age of Sail
            Self::HeavyLeather => TechLevel::new(1),  // Bronze Age
            Self::HeavyPlate => TechLevel::new(3),    // Age of Sail
            Self::LamellarArmor => TechLevel::new(1), // Bronze Age
            Self::LargeShield => TechLevel::new(1),   // Bronze Age
            Self::LeatherArmor => TechLevel::new(1),  // Bronze Age
            Self::LightLeather => TechLevel::new(0),  // Stone Age
            Self::LightScale => TechLevel::new(2),    // Medieval
            Self::MailHauberk => TechLevel::new(2),   // Medieval
            Self::MailShirt => TechLevel::new(2),     // Medieval
            Self::MediumShield => TechLevel::new(1),  // Bronze Age
            Self::NoArmor => TechLevel::new(0),       // Stone Age
            Self::PlateArmor => TechLevel::new(3),    // Age of Sail
            Self::RingMail => TechLevel::new(2),      // Iron Age
            Self::ScaleMail => TechLevel::new(2),     // Medieval
            Self::SmallShield => TechLevel::new(1),   // Bronze Age
            Self::SplintMail => TechLevel::new(2),    // Medieval
            Self::TacticalVest => TechLevel::new(8),  // Microtech Age
            Self::TowerShield => TechLevel::new(2),   // Medieval
        }
    }

    /// Returns damage resistance for this armor.
    #[instrument]
    pub fn damage_resistance(&self) -> i32 {
        debug!("Getting armor damage resistance");
        match self {
            Self::BallisticVest => 10, // DR 10 vs ballistic
            Self::BandedMail => 4,
            Self::Brigandine => 4,
            Self::BronzePlate => 5,
            Self::Buckler => 1,        // DB converted to DR approximation
            Self::Chainmail => 4,
            Self::ClothArmor => 1,
            Self::FlakJacket => 7,     // DR 7 vs ballistic
            Self::HalfPlate => 5,
            Self::HeavyLeather => 2,
            Self::HeavyPlate => 8,
            Self::LamellarArmor => 4,
            Self::LargeShield => 3,    // DB converted to DR approximation
            Self::LeatherArmor => 1,
            Self::LightLeather => 1,
            Self::LightScale => 3,
            Self::MailHauberk => 4,
            Self::MailShirt => 4,
            Self::MediumShield => 2,   // DB converted to DR approximation
            Self::NoArmor => 0,
            Self::PlateArmor => 6,
            Self::RingMail => 3,
            Self::ScaleMail => 4,
            Self::SmallShield => 1,    // DB converted to DR approximation
            Self::SplintMail => 5,
            Self::TacticalVest => 18,  // DR 18 vs ballistic
            Self::TowerShield => 4,    // DB converted to DR approximation
        }
    }
}
