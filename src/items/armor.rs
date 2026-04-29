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
    /// Aketon, DR 2. LT (Quilted coat, TL2)
    Aketon,
    /// Ballistic vest, DR 10. BS 279
    BallisticVest,
    /// Banded mail, DR 4. BS 279
    BandedMail,
    /// Bascinet, DR 5. LT (Medieval helmet, TL3)
    Bascinet,
    /// Brigandine, DR 4. BS 279
    Brigandine,
    /// Bronze plate, DR 5. BS 279
    BronzePlate,
    /// Bronze breastplate, DR 4. LT (Partial bronze plate, TL1)
    BronzeBreastplate,
    /// Bronze corselet, DR 4. LT (Light bronze armor, TL1)
    BronzeCorselet,
    /// Buckler, DR 1. BS 282
    Buckler,
    /// Buff coat, DR 2. LT (Heavy leather coat, TL4)
    BuffCoat,
    /// Chainmail, DR 4. BS 279
    Chainmail,
    /// Chainmail shirt, DR 3. LT (Light mail shirt, TL2)
    ChainmailShirt,
    /// Close helm, DR 7. LT (Full face helmet, TL4)
    CloseHelm,
    /// Cloth armor, DR 1. BS 279
    ClothArmor,
    /// Coat of plates, DR 5. LT (Transitional armor, TL3)
    CoatOfPlates,
    /// Cuirass, DR 5. LT (Breastplate only, TL3)
    Cuirass,
    /// Do-maru, DR 4. LT (Japanese wrap armor, TL3)
    DoMaru,
    /// Flak jacket, DR 7. BS 279
    FlakJacket,
    /// Gambeson, DR 2. LT (Quilted armor, TL2)
    Gambeson,
    /// Gothic plate, DR 7. LT (Fluted full plate, TL4)
    GothicPlate,
    /// Great helm, DR 7. LT (Crusader helmet, TL3)
    GreatHelm,
    /// Half plate, DR 5. BS 279
    HalfPlate,
    /// Heavy leather, DR 2. BS 279
    HeavyLeather,
    /// Heavy leather sleeves, DR 2. LT (Arm protection, TL1)
    HeavyLeatherSleeves,
    /// Heavy plate, DR 8. BS 279
    HeavyPlate,
    /// Jack of plates, DR 4. LT (Plate-lined jack, TL3)
    JackOfPlates,
    /// Kikko armor, DR 4. LT (Japanese hexagonal plate, TL3)
    KikkoArmor,
    /// Lamellar armor, DR 4. BS 279
    LamellarArmor,
    /// Large shield, DR 3. BS 282
    LargeShield,
    /// Leather armor, DR 1. BS 279
    LeatherArmor,
    /// Leather cap, DR 1. LT (Simple leather helmet, TL1)
    LeatherCap,
    /// Light leather, DR 1. BS 279
    LightLeather,
    /// Light scale, DR 3. BS 279
    LightScale,
    /// Linothorax, DR 2. LT (Greek linen armor, TL1)
    Linothorax,
    /// Lorica segmentata, DR 5. LT (Roman segmented armor, TL1)
    LoricaSegmentata,
    /// Lorica hamata, DR 4. LT (Roman mail, TL1)
    LoricaHamata,
    /// Mountain pattern armor, DR 5. LT (Chinese brigandine, TL3)
    MountainPatternArmor,
    /// Mail hauberk, DR 4. BS 279
    MailHauberk,
    /// Mail shirt, DR 4. BS 279
    MailShirt,
    /// Mail leggings, DR 3. LT (Leg protection, TL2)
    MailLeggings,
    /// Medium shield, DR 2. BS 282
    MediumShield,
    /// No armor, DR 0. BS 279
    NoArmor,
    /// O-yoroi, DR 5. LT (Japanese great armor, TL3)
    OYoroi,
    /// Plate armor, DR 6. BS 279
    PlateArmor,
    /// Pot helm, DR 4. LT (Simple iron helmet, TL2)
    PotHelm,
    /// Ring mail, DR 3. BS 279
    RingMail,
    /// Scale corselet, DR 3. LT (Light scale armor, TL1)
    ScaleCorselet,
    /// Scale mail, DR 4. BS 279
    ScaleMail,
    /// Sallet, DR 5. LT (Late medieval helmet, TL4)
    Sallet,
    /// Small shield, DR 1. BS 282
    SmallShield,
    /// Splint mail, DR 5. BS 279
    SplintMail,
    /// Studded leather, DR 2. LT (Reinforced leather, TL1)
    StuddedLeather,
    /// Three-quarter plate, DR 6. LT (Partial full plate, TL4)
    ThreeQuarterPlate,
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
                        Self::Aketon => Currency::dollars(130.0),
            Self::BallisticVest => Currency::dollars(400.0),
                        Self::BandedMail => Currency::dollars(500.0),
            Self::Bascinet => Currency::dollars(200.0),
            Self::Brigandine => Currency::dollars(500.0),
                        Self::BronzePlate => Currency::dollars(2400.0),
                        Self::BronzeBreastplate => Currency::dollars(1200.0),
            Self::BronzeCorselet => Currency::dollars(1800.0),
                        Self::Buckler => Currency::dollars(25.0),
            Self::BuffCoat => Currency::dollars(210.0),
                        Self::Chainmail => Currency::dollars(550.0),
                        Self::ChainmailShirt => Currency::dollars(350.0),
            Self::CloseHelm => Currency::dollars(300.0),
                        Self::ClothArmor => Currency::dollars(30.0),
            Self::CoatOfPlates => Currency::dollars(3000.0),
                        Self::Cuirass => Currency::dollars(1500.0),
            Self::DoMaru => Currency::dollars(2400.0),
                        Self::FlakJacket => Currency::dollars(500.0),
                        Self::Gambeson => Currency::dollars(150.0),
                        Self::GothicPlate => Currency::dollars(6000.0),
            Self::GreatHelm => Currency::dollars(150.0),
            Self::HalfPlate => Currency::dollars(1500.0),
                        Self::HeavyLeather => Currency::dollars(150.0),
            Self::HeavyLeatherSleeves => Currency::dollars(100.0),
            Self::HeavyPlate => Currency::dollars(6000.0),
                        Self::JackOfPlates => Currency::dollars(400.0),
            Self::KikkoArmor => Currency::dollars(600.0),
            Self::LamellarArmor => Currency::dollars(500.0),
            Self::LargeShield => Currency::dollars(90.0),
                        Self::LeatherArmor => Currency::dollars(100.0),
            Self::LeatherCap => Currency::dollars(20.0),
            Self::LightLeather => Currency::dollars(50.0),
            Self::LightScale => Currency::dollars(280.0),
            Self::Linothorax => Currency::dollars(250.0),
                                    Self::LoricaSegmentata => Currency::dollars(1500.0),
            Self::LoricaHamata => Currency::dollars(600.0),
            Self::MountainPatternArmor => Currency::dollars(2000.0),
            Self::MailHauberk => Currency::dollars(600.0),
                        Self::MailShirt => Currency::dollars(350.0),
            Self::MailLeggings => Currency::dollars(200.0),
            Self::MediumShield => Currency::dollars(60.0),
                        Self::NoArmor => Currency::dollars(0.0),
            Self::OYoroi => Currency::dollars(3000.0),
                        Self::PlateArmor => Currency::dollars(3000.0),
            Self::PotHelm => Currency::dollars(100.0),
                        Self::RingMail => Currency::dollars(300.0),
            Self::ScaleCorselet => Currency::dollars(350.0),
                        Self::ScaleMail => Currency::dollars(420.0),
            Self::Sallet => Currency::dollars(200.0),
            Self::SmallShield => Currency::dollars(40.0),
                                    Self::SplintMail => Currency::dollars(700.0),
            Self::StuddedLeather => Currency::dollars(180.0),
            Self::ThreeQuarterPlate => Currency::dollars(4500.0),
            Self::TacticalVest => Currency::dollars(900.0),
            Self::TowerShield => Currency::dollars(150.0),
        }
    }

    /// Returns weight for this armor.
    #[instrument]
    pub fn weight(&self) -> Weight {
        debug!("Getting armor weight");
        match self {
                        Self::Aketon => Weight::pounds(10.0),
            Self::BallisticVest => Weight::pounds(2.0),
                        Self::BandedMail => Weight::pounds(35.0),
            Self::Bascinet => Weight::pounds(5.0),
            Self::Brigandine => Weight::pounds(25.0),
            Self::BronzePlate => Weight::pounds(60.0),
                        Self::BronzeBreastplate => Weight::pounds(25.0),
            Self::BronzeCorselet => Weight::pounds(30.0),
                        Self::Buckler => Weight::pounds(5.0),
            Self::BuffCoat => Weight::pounds(10.0),
                        Self::Chainmail => Weight::pounds(35.0),
                        Self::ChainmailShirt => Weight::pounds(20.0),
            Self::CloseHelm => Weight::pounds(8.0),
                        Self::ClothArmor => Weight::pounds(6.0),
            Self::CoatOfPlates => Weight::pounds(40.0),
                        Self::Cuirass => Weight::pounds(25.0),
            Self::DoMaru => Weight::pounds(40.0),
                        Self::FlakJacket => Weight::pounds(20.0),
                        Self::Gambeson => Weight::pounds(12.0),
                        Self::GothicPlate => Weight::pounds(65.0),
            Self::GreatHelm => Weight::pounds(10.0),
            Self::HalfPlate => Weight::pounds(30.0),
                        Self::HeavyLeather => Weight::pounds(15.0),
            Self::HeavyLeatherSleeves => Weight::pounds(8.0),
            Self::HeavyPlate => Weight::pounds(60.0),
                        Self::JackOfPlates => Weight::pounds(25.0),
            Self::KikkoArmor => Weight::pounds(30.0),
            Self::LamellarArmor => Weight::pounds(35.0),
            Self::LargeShield => Weight::pounds(25.0),
                        Self::LeatherArmor => Weight::pounds(10.0),
            Self::LeatherCap => Weight::pounds(1.0),
            Self::LightLeather => Weight::pounds(5.0),
            Self::LightScale => Weight::pounds(28.0),
            Self::Linothorax => Weight::pounds(15.0),
                                    Self::LoricaSegmentata => Weight::pounds(35.0),
            Self::LoricaHamata => Weight::pounds(30.0),
            Self::MountainPatternArmor => Weight::pounds(35.0),
            Self::MailHauberk => Weight::pounds(45.0),
                        Self::MailShirt => Weight::pounds(25.0),
            Self::MailLeggings => Weight::pounds(15.0),
            Self::MediumShield => Weight::pounds(15.0),
                        Self::NoArmor => Weight::pounds(0.0),
            Self::OYoroi => Weight::pounds(50.0),
                        Self::PlateArmor => Weight::pounds(50.0),
            Self::PotHelm => Weight::pounds(5.0),
                        Self::RingMail => Weight::pounds(25.0),
            Self::ScaleCorselet => Weight::pounds(22.0),
                        Self::ScaleMail => Weight::pounds(42.0),
            Self::Sallet => Weight::pounds(6.0),
            Self::SmallShield => Weight::pounds(8.0),
                                    Self::SplintMail => Weight::pounds(45.0),
            Self::StuddedLeather => Weight::pounds(18.0),
            Self::ThreeQuarterPlate => Weight::pounds(55.0),
            Self::TacticalVest => Weight::pounds(9.0),
            Self::TowerShield => Weight::pounds(45.0),
        }
    }

    /// Returns tech level for this armor.
    #[instrument]
    pub fn tech_level(&self) -> TechLevel {
        debug!("Getting armor tech level");
        match self {
                        Self::Aketon => TechLevel::new(2),        // Medieval
            Self::BallisticVest => TechLevel::new(7), // Digital Age
                        Self::BandedMail => TechLevel::new(3),    // Medieval
            Self::Bascinet => TechLevel::new(3),       // Medieval
            Self::Brigandine => TechLevel::new(3),    // Age of Sail
            Self::BronzePlate => TechLevel::new(1),   // Bronze Age
                        Self::BronzeBreastplate => TechLevel::new(1), // Bronze Age
            Self::BronzeCorselet => TechLevel::new(1),  // Bronze Age
                        Self::Buckler => TechLevel::new(2),       // Medieval
            Self::BuffCoat => TechLevel::new(4),       // Renaissance
                        Self::Chainmail => TechLevel::new(2),     // Medieval
                        Self::ChainmailShirt => TechLevel::new(2), // Medieval
            Self::CloseHelm => TechLevel::new(4),      // Renaissance
            Self::ClothArmor => TechLevel::new(1),    // Bronze Age
            Self::CoatOfPlates => TechLevel::new(3),  // Medieval
            Self::Cuirass => TechLevel::new(3),       // Medieval
            Self::DoMaru => TechLevel::new(3),        // Japanese Medieval
            Self::FlakJacket => TechLevel::new(6),    // Atomic Age
                        Self::Gambeson => TechLevel::new(2),      // Medieval
                        Self::GothicPlate => TechLevel::new(4),    // Renaissance
            Self::GreatHelm => TechLevel::new(3),      // Crusader/Medieval
            Self::HalfPlate => TechLevel::new(3),     // Age of Sail
                        Self::HeavyLeather => TechLevel::new(1),  // Bronze Age
            Self::HeavyLeatherSleeves => TechLevel::new(1), // Bronze Age
            Self::HeavyPlate => TechLevel::new(3),    // Age of Sail
                        Self::JackOfPlates => TechLevel::new(3),  // Medieval
            Self::KikkoArmor => TechLevel::new(3),     // Japanese Medieval
            Self::LamellarArmor => TechLevel::new(1), // Bronze Age
            Self::LargeShield => TechLevel::new(1),   // Bronze Age
                        Self::LeatherArmor => TechLevel::new(1),  // Bronze Age
            Self::LeatherCap => TechLevel::new(1),     // Bronze Age
            Self::LightLeather => TechLevel::new(0),  // Stone Age
            Self::LightScale => TechLevel::new(2),    // Medieval
            Self::Linothorax => TechLevel::new(1),    // Greek/Bronze
                                    Self::LoricaSegmentata => TechLevel::new(1), // Roman
            Self::LoricaHamata => TechLevel::new(1),   // Roman
            Self::MountainPatternArmor => TechLevel::new(3), // Chinese Medieval
            Self::MailHauberk => TechLevel::new(2),   // Medieval
                        Self::MailShirt => TechLevel::new(2),     // Medieval
            Self::MailLeggings => TechLevel::new(2),   // Medieval
            Self::MediumShield => TechLevel::new(1),  // Bronze Age
                        Self::NoArmor => TechLevel::new(0),       // Stone Age
            Self::OYoroi => TechLevel::new(3),         // Japanese Medieval
                        Self::PlateArmor => TechLevel::new(3),    // Age of Sail
            Self::PotHelm => TechLevel::new(2),        // Iron Age/Medieval
                        Self::RingMail => TechLevel::new(2),      // Iron Age
            Self::ScaleCorselet => TechLevel::new(1),  // Bronze Age
                        Self::ScaleMail => TechLevel::new(2),     // Medieval
            Self::Sallet => TechLevel::new(4),         // Late Medieval/Renaissance
            Self::SmallShield => TechLevel::new(1),   // Bronze Age
                                    Self::SplintMail => TechLevel::new(2),    // Medieval
            Self::StuddedLeather => TechLevel::new(1), // Bronze Age
            Self::ThreeQuarterPlate => TechLevel::new(4), // Renaissance
            Self::TacticalVest => TechLevel::new(8),  // Microtech Age
            Self::TowerShield => TechLevel::new(2),   // Medieval
        }
    }

    /// Returns damage resistance for this armor.
    #[instrument]
    pub fn damage_resistance(&self) -> i32 {
        debug!("Getting armor damage resistance");
        match self {
                        Self::Aketon => 2,
            Self::BallisticVest => 10, // DR 10 vs ballistic
                        Self::BandedMail => 4,
            Self::Bascinet => 5,
            Self::Brigandine => 4,
                        Self::BronzePlate => 5,
                        Self::BronzeBreastplate => 4,
            Self::BronzeCorselet => 4,
                        Self::Buckler => 1,        // DB converted to DR approximation
            Self::BuffCoat => 2,
                        Self::Chainmail => 4,
                        Self::ChainmailShirt => 3,
            Self::CloseHelm => 7,
                        Self::ClothArmor => 1,
            Self::CoatOfPlates => 5,
                        Self::Cuirass => 5,
            Self::DoMaru => 4,
                        Self::FlakJacket => 7,
                        Self::Gambeson => 2,
                        Self::GothicPlate => 7,
            Self::GreatHelm => 7,     // DR 7 vs ballistic
            Self::HalfPlate => 5,
                        Self::HeavyLeather => 2,
            Self::HeavyLeatherSleeves => 2,
                        Self::HeavyPlate => 8,
                        Self::JackOfPlates => 4,
            Self::KikkoArmor => 4,
            Self::LamellarArmor => 4,
            Self::LargeShield => 3,    // DB converted to DR approximation
                        Self::LeatherArmor => 1,
            Self::LeatherCap => 1,
            Self::LightLeather => 1,
                        Self::LightScale => 3,
            Self::Linothorax => 2,
                                    Self::LoricaSegmentata => 5,
            Self::LoricaHamata => 4,
            Self::MountainPatternArmor => 5,
            Self::MailHauberk => 4,
                        Self::MailShirt => 4,
            Self::MailLeggings => 3,
            Self::MediumShield => 2,   // DB converted to DR approximation
                        Self::NoArmor => 0,
            Self::OYoroi => 5,
                        Self::PlateArmor => 6,
            Self::PotHelm => 4,
                        Self::RingMail => 3,
            Self::ScaleCorselet => 3,
                        Self::ScaleMail => 4,
            Self::Sallet => 5,
            Self::SmallShield => 1,    // DB converted to DR approximation
                                    Self::SplintMail => 5,
            Self::StuddedLeather => 2,
            Self::ThreeQuarterPlate => 6,
            Self::TacticalVest => 18,  // DR 18 vs ballistic
            Self::TowerShield => 4,    // DB converted to DR approximation
        }
    }
}
