//! Melee weapon definitions and properties.
//!
//! # GURPS Rules
//!
//! Melee weapons use ST-based damage (thrust or swing) with modifiers.
//! Each weapon has reach, parry modifier, and required skill. Cost, weight,
//! and tech level vary by weapon type.
//!
//! # Citations
//!
//! BS 271-276 - Melee weapons table

use crate::{Currency, DamageType, Reach, Skill, TechLevel, WeaponDamage, Weight};
use tracing::{debug, instrument};

/// Melee weapon types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, strum::EnumIter)]
pub enum MeleeWeapon {
    /// Axe, swing+2 cutting, reach 1. BS 271
    Axe,
    /// Baton, swing crushing, reach 1. BS 271
    Baton,
    /// Battle axe, swing+3 cutting, reach 1. BS 271
    BattleAxe,
    /// Blackjack (sap), swing crushing, reach C. BS 271
    Blackjack,
    /// Bastard sword, swing+2 cutting, reach 1-2. BS 271
    BastardSword,
    /// Bardiche, swing+3 cutting, reach 2-3. LT
    Bardiche,
    /// Bill, swing+3 cutting, reach 2-3. LT
    Bill,
    /// Billhook, swing+2 cutting, reach 2. LT
    Billhook,
    /// Bola, swing crushing, reach 1. BS 271
    Bola,
    /// Brass knuckles, thrust crushing, reach C. BS 271
    BrassKnuckles,
    /// Broadsword, swing+1 cutting, reach 1. BS 271
    Broadsword,
    /// Chakram, swing cutting, reach 1. BS 271
    Chakram,
    /// Cestus, thrust crushing, reach C. BS 271
    Cestus,
    /// Chain, swing+1 crushing, reach 1-3. BS 271
    Chain,
    /// Claymore, swing+2 cutting, reach 1-2. BS 271
    Claymore,
    /// Club, swing+1 crushing, reach 1. BS 271
    Club,
    /// Combat net, swing-2 crushing, reach 1. BS 271
    CombatNet,
    /// Crowbar, swing+2 crushing, reach 1. BS 289
    Crowbar,
    /// Cutlass, swing+1 cutting, reach 1. BS 271
    Cutlass,
    /// Dagger, thrust-1 impaling, reach C. BS 271
    Dagger,
    /// Estoc, thrust+2 impaling, reach 1. BS 271
    Estoc,
    /// Falchion, swing+2 cutting, reach 1. BS 271
    Falchion,
    /// Fauchard, swing+3 cutting, reach 2-3. LT
    Fauchard,
    /// Fist, thrust-1 crushing, reach C. BS 271
    Fist,
    /// Flail, swing+2 crushing, reach 1-2. BS 271
    Flail,
    /// Gladius, swing cutting, reach 1. BS 271
    Gladius,
    /// Garrote, special, reach C. BS 271
    Garrote,
    /// Glaive, swing+2 cutting, reach 2-3. BS 271
    Glaive,
    /// Guisarme, swing+2 cutting, reach 2-3. LT
    Guisarme,
    /// Great axe, swing+3 cutting, reach 1-2. BS 271
    GreatAxe,
    /// Halberd, swing+3 cutting, reach 2-3. BS 271
    Halberd,
    /// Hatchet, swing+1 cutting, reach 1. BS 271
    Hatchet,
    /// Javelin, thrust+1 impaling, reach 1. BS 271
    Javelin,
    /// Jitte, thrust-1 impaling, reach 1. BS 271
    Jitte,
    /// Katana, swing+1 cutting, reach 1. BS 271
    Katana,
    /// Katar, thrust+1 impaling, reach C. BS 271
    Katar,
    /// Kick, thrust crushing, reach C-1. BS 271
    Kick,
    /// Knife, thrust-1 impaling, reach C. BS 271
    Knife,
    /// Kopesh, swing+1 cutting, reach 1. BS 271
    Kopesh,
    /// Kusari, swing+1 crushing, reach 1-3. BS 271
    Kusari,
    /// Large knife, swing-1 cutting, reach C-1. BS 271
    LargeKnife,
    /// Lasso, special entangling, reach 1-2. BS 271
    Lasso,
    /// Lance, thrust+3 impaling, reach 3. BS 271
    Lance,
    /// Long spear, thrust+2 impaling, reach 2-3. BS 271
    LongSpear,
    /// Longsword, swing+1 cutting, reach 1. BS 271
    Longsword,
    /// Mace, swing+2 crushing, reach 1. BS 271
    Mace,
    /// Main gauche, thrust-1 impaling, reach 1. BS 271
    MainGauche,
    /// Maul, swing+4 crushing, reach 1-2. BS 271
    Maul,
    /// Mattock, swing+2 impaling, reach 1. BS 289
    Mattock,
    /// Morningstar, swing+3 crushing, reach 1. BS 271
    Morningstar,
    /// Naginata, swing+2 cutting, reach 2-3. BS 271
    Naginata,
    /// Nunchaku, swing+1 crushing, reach 1. BS 271
    Nunchaku,
    /// Pick, swing+2 impaling, reach 1. BS 271
    Pick,
    /// Pike, thrust+2 impaling, reach 4-5. BS 271
    Pike,
    /// Poleax, swing+3 cutting, reach 2-3. BS 271
    Poleax,
    /// Partisan, thrust+2 impaling, reach 2-3. LT
    Partisan,
    /// Quarterstaff, swing+2 crushing, reach 1-2. BS 271
    Quarterstaff,
    /// Rapier, thrust+1 impaling, reach 1. BS 271
    Rapier,
    /// Saber, swing+1 cutting, reach 1. BS 271
    Saber,
    /// Sai, thrust-1 impaling, reach C. BS 271
    Sai,
    /// Scimitar, swing+1 cutting, reach 1. BS 271
    Scimitar,
    /// Shortsword, swing cutting, reach 1. BS 271
    Shortsword,
    /// Shovel, swing+2 crushing, reach 1. BS 289
    Shovel,
    /// Smallsword, thrust impaling, reach 1. BS 271
    Smallsword,
    /// Spear, thrust+2 impaling, reach 1-2. BS 271
    Spear,
    /// Staff, swing+1 crushing, reach 1-2. BS 271
    Staff,
    /// Stick (walking stick/cane), swing crushing, reach 1. BS 271
    Stick,
    /// Tanto, thrust impaling, reach C. BS 271
    Tanto,
    /// Tonfa, swing+1 crushing, reach 1. BS 271
    Tonfa,
    /// Trident, thrust+2 impaling, reach 1-2. BS 271
    Trident,
    /// Two-handed sword, swing+2 cutting, reach 1-2. BS 271
    TwoHandedSword,
    /// Wakizashi, swing cutting, reach 1. BS 271
    Wakizashi,
    /// Voulge, swing+3 cutting, reach 2-3. LT
    Voulge,
    /// War fan, swing crushing, reach C. BS 271
    WarFan,
    /// Warhammer, swing+3 impaling, reach 1-2. BS 271
    Warhammer,
    /// Whip, swing-1 cutting, reach 1-2. BS 271
    Whip,
}

impl MeleeWeapon {
    /// Returns base cost for this melee weapon.
    #[instrument]
    pub fn base_cost(&self) -> Currency {
        debug!("Getting melee weapon base cost");
        match self {
            Self::Axe => Currency::dollars(50.0),
            Self::Baton => Currency::dollars(20.0),
            Self::BattleAxe => Currency::dollars(50.0),
            Self::BastardSword => Currency::dollars(650.0),
            Self::Bardiche => Currency::dollars(120.0),
            Self::Bill => Currency::dollars(100.0),
            Self::Billhook => Currency::dollars(40.0),
            Self::Blackjack => Currency::dollars(20.0),
            Self::Bola => Currency::dollars(20.0),
            Self::BrassKnuckles => Currency::dollars(10.0),
            Self::Broadsword => Currency::dollars(500.0),
            Self::Chakram => Currency::dollars(15.0),
            Self::Cestus => Currency::dollars(15.0),
            Self::Chain => Currency::dollars(30.0),
            Self::Claymore => Currency::dollars(500.0),
            Self::Club => Currency::dollars(10.0),
            Self::CombatNet => Currency::dollars(40.0),
            Self::Crowbar => Currency::dollars(20.0),
            Self::Cutlass => Currency::dollars(400.0),
            Self::Dagger => Currency::dollars(20.0),
            Self::Estoc => Currency::dollars(600.0),
            Self::Falchion => Currency::dollars(400.0),
            Self::Fauchard => Currency::dollars(90.0),
            Self::Fist => Currency::dollars(0.0),
            Self::Flail => Currency::dollars(60.0),
            Self::Gladius => Currency::dollars(200.0),
            Self::Garrote => Currency::dollars(15.0),
            Self::Glaive => Currency::dollars(100.0),
            Self::Guisarme => Currency::dollars(100.0),
            Self::GreatAxe => Currency::dollars(100.0),
            Self::Halberd => Currency::dollars(150.0),
            Self::Hatchet => Currency::dollars(40.0),
            Self::Javelin => Currency::dollars(30.0),
            Self::Jitte => Currency::dollars(50.0),
            Self::Katana => Currency::dollars(650.0),
            Self::Katar => Currency::dollars(40.0),
            Self::Kick => Currency::dollars(0.0),
            Self::Knife => Currency::dollars(40.0),
            Self::Kopesh => Currency::dollars(200.0),
            Self::Kusari => Currency::dollars(70.0),
            Self::LargeKnife => Currency::dollars(60.0),
            Self::Lasso => Currency::dollars(20.0),
            Self::Lance => Currency::dollars(60.0),
            Self::LongSpear => Currency::dollars(60.0),
            Self::Longsword => Currency::dollars(500.0),
            Self::Mace => Currency::dollars(50.0),
            Self::MainGauche => Currency::dollars(50.0),
            Self::Maul => Currency::dollars(80.0),
            Self::Mattock => Currency::dollars(25.0),
            Self::Morningstar => Currency::dollars(80.0),
            Self::Naginata => Currency::dollars(100.0),
            Self::Nunchaku => Currency::dollars(20.0),
            Self::Pick => Currency::dollars(70.0),
            Self::Pike => Currency::dollars(80.0),
            Self::Poleax => Currency::dollars(120.0),
            Self::Partisan => Currency::dollars(100.0),
            Self::Quarterstaff => Currency::dollars(10.0),
            Self::Rapier => Currency::dollars(500.0),
            Self::Saber => Currency::dollars(500.0),
            Self::Sai => Currency::dollars(20.0),
            Self::Scimitar => Currency::dollars(500.0),
            Self::Shortsword => Currency::dollars(400.0),
            Self::Shovel => Currency::dollars(15.0),
            Self::Smallsword => Currency::dollars(400.0),
            Self::Spear => Currency::dollars(40.0),
            Self::Staff => Currency::dollars(5.0),
            Self::Stick => Currency::dollars(5.0),
            Self::Tanto => Currency::dollars(30.0),
            Self::Tonfa => Currency::dollars(20.0),
            Self::Trident => Currency::dollars(100.0),
            Self::TwoHandedSword => Currency::dollars(900.0),
            Self::Wakizashi => Currency::dollars(400.0),
            Self::Voulge => Currency::dollars(110.0),
            Self::WarFan => Currency::dollars(50.0),
            Self::Warhammer => Currency::dollars(100.0),
            Self::Whip => Currency::dollars(20.0),
        }
    }

    /// Returns weight for this melee weapon.
    #[instrument]
    pub fn weight(&self) -> Weight {
        debug!("Getting melee weapon weight");
        match self {
            Self::Axe => Weight::pounds(4.0),
            Self::Baton => Weight::pounds(1.0),
            Self::BattleAxe => Weight::pounds(6.0),
            Self::Blackjack => Weight::pounds(1.0),
            Self::BastardSword => Weight::pounds(5.0),
            Self::Bardiche => Weight::pounds(10.0),
            Self::Bill => Weight::pounds(9.0),
            Self::Billhook => Weight::pounds(5.0),
            Self::Bola => Weight::pounds(1.0),
            Self::BrassKnuckles => Weight::pounds(0.25),
            Self::Broadsword => Weight::pounds(3.0),
            Self::Chakram => Weight::pounds(0.5),
            Self::Cestus => Weight::pounds(0.5),
            Self::Chain => Weight::pounds(3.0),
            Self::Claymore => Weight::pounds(7.0),
            Self::Club => Weight::pounds(3.0),
            Self::CombatNet => Weight::pounds(5.0),
            Self::Crowbar => Weight::pounds(3.0),
            Self::Cutlass => Weight::pounds(2.0),
            Self::Dagger => Weight::pounds(0.25),
            Self::Estoc => Weight::pounds(3.5),
            Self::Falchion => Weight::pounds(3.5),
            Self::Fauchard => Weight::pounds(8.0),
            Self::Fist => Weight::pounds(0.0),
            Self::Flail => Weight::pounds(8.0),
            Self::Gladius => Weight::pounds(2.0),
            Self::Garrote => Weight::pounds(0.25),
            Self::Glaive => Weight::pounds(8.0),
            Self::Guisarme => Weight::pounds(9.0),
            Self::GreatAxe => Weight::pounds(8.0),
            Self::Halberd => Weight::pounds(12.0),
            Self::Hatchet => Weight::pounds(2.0),
            Self::Javelin => Weight::pounds(2.0),
            Self::Jitte => Weight::pounds(1.5),
            Self::Katana => Weight::pounds(2.5),
            Self::Katar => Weight::pounds(1.0),
            Self::Kick => Weight::pounds(0.0),
            Self::Knife => Weight::pounds(1.0),
            Self::Kopesh => Weight::pounds(3.5),
            Self::Kusari => Weight::pounds(5.0),
            Self::LargeKnife => Weight::pounds(1.5),
            Self::Lasso => Weight::pounds(3.0),
            Self::Lance => Weight::pounds(6.0),
            Self::LongSpear => Weight::pounds(5.0),
            Self::Longsword => Weight::pounds(3.0),
            Self::Mace => Weight::pounds(5.0),
            Self::MainGauche => Weight::pounds(1.25),
            Self::Maul => Weight::pounds(12.0),
            Self::Mattock => Weight::pounds(6.0),
            Self::Morningstar => Weight::pounds(6.0),
            Self::Naginata => Weight::pounds(9.0),
            Self::Nunchaku => Weight::pounds(1.5),
            Self::Pick => Weight::pounds(3.0),
            Self::Pike => Weight::pounds(13.0),
            Self::Poleax => Weight::pounds(10.0),
            Self::Partisan => Weight::pounds(7.0),
            Self::Quarterstaff => Weight::pounds(4.0),
            Self::Rapier => Weight::pounds(2.75),
            Self::Saber => Weight::pounds(2.0),
            Self::Sai => Weight::pounds(1.0),
            Self::Scimitar => Weight::pounds(3.0),
            Self::Shortsword => Weight::pounds(2.0),
            Self::Shovel => Weight::pounds(6.0),
            Self::Smallsword => Weight::pounds(1.5),
            Self::Spear => Weight::pounds(4.0),
            Self::Staff => Weight::pounds(4.0),
            Self::Stick => Weight::pounds(2.0),
            Self::Tanto => Weight::pounds(0.5),
            Self::Tonfa => Weight::pounds(1.5),
            Self::Trident => Weight::pounds(4.0),
            Self::TwoHandedSword => Weight::pounds(7.0),
            Self::Wakizashi => Weight::pounds(1.5),
            Self::Voulge => Weight::pounds(10.0),
            Self::WarFan => Weight::pounds(1.0),
            Self::Warhammer => Weight::pounds(7.0),
            Self::Whip => Weight::pounds(2.0),
        }
    }

    /// Returns tech level for this melee weapon.
    #[instrument]
    pub fn tech_level(&self) -> TechLevel {
        debug!("Getting melee weapon tech level");
        match self {
            Self::Axe => TechLevel::new(0),            // Stone Age
            Self::Baton => TechLevel::new(5),          // Modern (police baton)
            Self::BattleAxe => TechLevel::new(1),      // Bronze/Iron Age
            Self::Blackjack => TechLevel::new(1),
            Self::BastardSword => TechLevel::new(2),   // Medieval
            Self::Bardiche => TechLevel::new(3),    // Medieval
            Self::Bill => TechLevel::new(3),        // Medieval
            Self::Billhook => TechLevel::new(2),    // Medieval
            Self::Bola => TechLevel::new(0),           // Stone Age
            Self::BrassKnuckles => TechLevel::new(3),  // Industrial
            Self::Broadsword => TechLevel::new(2),     // Medieval
            Self::Chakram => TechLevel::new(2),        // Medieval Indian
            Self::Cestus => TechLevel::new(1),          // Roman
            Self::Chain => TechLevel::new(1),           // Ancient
            Self::Claymore => TechLevel::new(3),       // Scottish Renaissance
            Self::Club => TechLevel::new(0),
            Self::CombatNet => TechLevel::new(1),      // Bronze Age
            Self::Crowbar => TechLevel::new(3),        // Industrial
            Self::Cutlass => TechLevel::new(4),        // Age of Sail
            Self::Dagger => TechLevel::new(1),         // Bronze Age
            Self::Estoc => TechLevel::new(3),          // Late Medieval
            Self::Falchion => TechLevel::new(2),       // Medieval
            Self::Fauchard => TechLevel::new(3),    // Medieval
            Self::Fist => TechLevel::new(0),           // Stone Age
            Self::Flail => TechLevel::new(2),          // Medieval
            Self::Gladius => TechLevel::new(1),        // Roman/Iron Age
            Self::Garrote => TechLevel::new(1),        // Ancient
            Self::Glaive => TechLevel::new(2),
            Self::Guisarme => TechLevel::new(3),    // Medieval
            Self::GreatAxe => TechLevel::new(1),       // Bronze/Iron Age
            Self::Halberd => TechLevel::new(2),        // Medieval
            Self::Hatchet => TechLevel::new(0),        // Stone Age
            Self::Javelin => TechLevel::new(0),        // Stone Age
            Self::Jitte => TechLevel::new(3),          // Japanese feudal
            Self::Katana => TechLevel::new(3),         // Medieval Japan
            Self::Katar => TechLevel::new(2),          // Medieval India
            Self::Kick => TechLevel::new(0),           // Stone Age
            Self::Knife => TechLevel::new(0),          // Stone Age
            Self::Kopesh => TechLevel::new(1),         // Egyptian Bronze Age
            Self::Kusari => TechLevel::new(2),         // Medieval Japan
            Self::LargeKnife => TechLevel::new(1),     // Bronze Age
            Self::Lasso => TechLevel::new(0),          // Stone Age
            Self::Lance => TechLevel::new(2),          // Medieval
            Self::LongSpear => TechLevel::new(1),      // Bronze Age
            Self::Longsword => TechLevel::new(2),      // Medieval
            Self::Mace => TechLevel::new(1),           // Bronze Age
            Self::MainGauche => TechLevel::new(4),     // Renaissance
            Self::Maul => TechLevel::new(1),           // Bronze/Iron Age
            Self::Mattock => TechLevel::new(1),        // Bronze/Iron Age
            Self::Morningstar => TechLevel::new(2),    // Medieval
            Self::Naginata => TechLevel::new(3),
            Self::Nunchaku => TechLevel::new(2),       // Medieval
            Self::Pick => TechLevel::new(2),           // Medieval
            Self::Pike => TechLevel::new(2),
            Self::Poleax => TechLevel::new(2),
            Self::Partisan => TechLevel::new(3),    // Medieval
            Self::Quarterstaff => TechLevel::new(0),   // Stone Age
            Self::Rapier => TechLevel::new(4),         // Renaissance
            Self::Saber => TechLevel::new(4),          // Age of Sail
            Self::Sai => TechLevel::new(2),            // Medieval
            Self::Scimitar => TechLevel::new(2),       // Medieval
            Self::Shortsword => TechLevel::new(1),     // Bronze Age
            Self::Shovel => TechLevel::new(1),         // Ancient tool
            Self::Smallsword => TechLevel::new(4),     // Renaissance
            Self::Spear => TechLevel::new(0),          // Stone Age
            Self::Staff => TechLevel::new(0),          // Stone Age
            Self::Stick => TechLevel::new(0),
            Self::Tanto => TechLevel::new(3),          // Japanese
            Self::Tonfa => TechLevel::new(0),          // Stone Age
            Self::Trident => TechLevel::new(0),        // Stone Age
            Self::TwoHandedSword => TechLevel::new(2), // Medieval
            Self::Wakizashi => TechLevel::new(3),      // Medieval Japan
            Self::Voulge => TechLevel::new(3),      // Medieval
            Self::WarFan => TechLevel::new(3),         // Japanese Renaissance
            Self::Warhammer => TechLevel::new(2),      // Medieval
            Self::Whip => TechLevel::new(1),           // Bronze Age
        }
    }

    /// Returns weapon damage for this melee weapon.
    #[instrument]
    pub fn damage(&self) -> WeaponDamage {
        debug!("Getting melee weapon damage");
        match self {
            Self::Axe => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
            Self::Baton => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
            Self::BattleAxe => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Cutting,
            },
            Self::Blackjack => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
            Self::BastardSword => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
            Self::Bardiche => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Cutting,
            },
            Self::Bill => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Cutting,
            },
            Self::Billhook => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
            Self::Bola => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
            Self::BrassKnuckles => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
            Self::Broadsword => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::Chakram => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Cutting,
            },
            Self::Cestus => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
            Self::Chain => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::Claymore => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
            Self::Club => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::CombatNet => WeaponDamage::Swing {
                modifier: -2,
                damage_type: DamageType::Crushing, // Entangling, minimal damage
            },
            Self::Crowbar => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Crushing,
            },
            Self::Cutlass => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::Dagger => WeaponDamage::Thrust {
                modifier: -1,
                damage_type: DamageType::Impaling,
            },
            Self::Estoc => WeaponDamage::Thrust {
                modifier: 2,
                damage_type: DamageType::Impaling,
            },
            Self::Falchion => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
            Self::Fauchard => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Cutting,
            },
            Self::Fist => WeaponDamage::Thrust {
                modifier: -1,
                damage_type: DamageType::Crushing,
            },
            Self::Flail => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Crushing,
            },
            Self::Gladius => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Cutting,
            },
            Self::Garrote => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Crushing, // Strangling attack
            },
            Self::Glaive => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
            Self::Guisarme => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
            Self::GreatAxe => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Cutting,
            },
            Self::Halberd => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Cutting,
            },
            Self::Hatchet => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::Javelin => WeaponDamage::Thrust {
                modifier: 1,
                damage_type: DamageType::Impaling,
            },
            Self::Jitte => WeaponDamage::Thrust {
                modifier: -1,
                damage_type: DamageType::Impaling,
            },
            Self::Katana => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::Katar => WeaponDamage::Thrust {
                modifier: 1,
                damage_type: DamageType::Impaling,
            },
            Self::Kick => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
            Self::Knife => WeaponDamage::Thrust {
                modifier: -1,
                damage_type: DamageType::Impaling,
            },
            Self::Kopesh => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::Kusari => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::LargeKnife => WeaponDamage::Swing {
                modifier: -1,
                damage_type: DamageType::Cutting,
            },
            Self::Lasso => WeaponDamage::Swing {
                modifier: -2,
                damage_type: DamageType::Crushing, // Entangling
            },
            Self::Lance => WeaponDamage::Thrust {
                modifier: 3,
                damage_type: DamageType::Impaling,
            },
            Self::LongSpear => WeaponDamage::Thrust {
                modifier: 2,
                damage_type: DamageType::Impaling,
            },
            Self::Longsword => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::Mace => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Crushing,
            },
            Self::MainGauche => WeaponDamage::Thrust {
                modifier: -1,
                damage_type: DamageType::Impaling,
            },
            Self::Maul => WeaponDamage::Swing {
                modifier: 4,
                damage_type: DamageType::Crushing,
            },
            Self::Mattock => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Impaling,
            },
            Self::Morningstar => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Crushing,
            },
            Self::Naginata => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
            Self::Nunchaku => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::Pick => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Impaling,
            },
            Self::Pike => WeaponDamage::Thrust {
                modifier: 2,
                damage_type: DamageType::Impaling,
            },
            Self::Poleax => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Cutting,
            },
            Self::Partisan => WeaponDamage::Thrust {
                modifier: 2,
                damage_type: DamageType::Impaling,
            },
            Self::Quarterstaff => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Crushing,
            },
            Self::Rapier => WeaponDamage::Thrust {
                modifier: 1,
                damage_type: DamageType::Impaling,
            },
            Self::Saber => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::Sai => WeaponDamage::Thrust {
                modifier: -1,
                damage_type: DamageType::Impaling,
            },
            Self::Scimitar => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::Shortsword => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Cutting,
            },
            Self::Shovel => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Crushing,
            },
            Self::Smallsword => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Impaling,
            },
            Self::Spear => WeaponDamage::Thrust {
                modifier: 2,
                damage_type: DamageType::Impaling,
            },
            Self::Staff => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::Stick => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
            Self::Tanto => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Impaling,
            },
            Self::Tonfa => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::Trident => WeaponDamage::Thrust {
                modifier: 2,
                damage_type: DamageType::Impaling,
            },
            Self::TwoHandedSword => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
            Self::Wakizashi => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Cutting,
            },
            Self::Voulge => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Cutting,
            },
            Self::WarFan => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
            Self::Warhammer => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Impaling,
            },
            Self::Whip => WeaponDamage::Swing {
                modifier: -1,
                damage_type: DamageType::Cutting,
            },
        }
    }

    /// Returns weapon reach for this melee weapon.
    #[instrument]
    pub fn reach(&self) -> Reach {
        debug!("Getting melee weapon reach");
        match self {
            Self::Axe => Reach::One,
            Self::Baton => Reach::One,
            Self::BattleAxe => Reach::One,
            Self::Blackjack => Reach::Close,
            Self::BastardSword => Reach::OneTwo,
            Self::Bardiche => Reach::TwoThree,
            Self::Bill => Reach::TwoThree,
            Self::Billhook => Reach::OneTwo,
            Self::Bola => Reach::One,
            Self::BrassKnuckles => Reach::Close,
            Self::Broadsword => Reach::One,
            Self::Chakram => Reach::One,
            Self::Cestus => Reach::Close,
            Self::Chain => Reach::OneThree,
            Self::Claymore => Reach::OneTwo,
            Self::Club => Reach::One,
            Self::CombatNet => Reach::One,
            Self::Crowbar => Reach::One,
            Self::Cutlass => Reach::One,
            Self::Dagger => Reach::Close,
            Self::Estoc => Reach::One,
            Self::Falchion => Reach::One,
            Self::Fauchard => Reach::TwoThree,
            Self::Fist => Reach::Close,
            Self::Flail => Reach::OneTwo,
            Self::Gladius => Reach::One,
            Self::Garrote => Reach::Close,
            Self::Glaive => Reach::TwoThree,
            Self::Guisarme => Reach::TwoThree,
            Self::GreatAxe => Reach::OneTwo,
            Self::Halberd => Reach::TwoThree,
            Self::Hatchet => Reach::One,
            Self::Javelin => Reach::One,
            Self::Jitte => Reach::One,
            Self::Katana => Reach::One,
            Self::Katar => Reach::Close,
            Self::Kick => Reach::CloseOne,
            Self::Knife => Reach::Close,
            Self::Kopesh => Reach::One,
            Self::Kusari => Reach::OneThree,
            Self::LargeKnife => Reach::CloseOne,
            Self::Lasso => Reach::OneTwo,
            Self::Lance => Reach::Three,
            Self::LongSpear => Reach::TwoThree,
            Self::Longsword => Reach::One,
            Self::Mace => Reach::One,
            Self::MainGauche => Reach::One,
            Self::Maul => Reach::OneTwo,
            Self::Mattock => Reach::One,
            Self::Morningstar => Reach::One,
            Self::Naginata => Reach::TwoThree,
            Self::Nunchaku => Reach::One,
            Self::Pick => Reach::One,
            Self::Pike => Reach::TwoThree,
            Self::Poleax => Reach::TwoThree,
            Self::Partisan => Reach::TwoThree,
            Self::Quarterstaff => Reach::OneTwo,
            Self::Rapier => Reach::One,
            Self::Saber => Reach::One,
            Self::Sai => Reach::Close,
            Self::Scimitar => Reach::One,
            Self::Shortsword => Reach::One,
            Self::Shovel => Reach::One,
            Self::Smallsword => Reach::One,
            Self::Spear => Reach::OneTwo,
            Self::Staff => Reach::OneTwo,
            Self::Stick => Reach::One,
            Self::Tanto => Reach::Close,
            Self::Tonfa => Reach::One,
            Self::Trident => Reach::OneTwo,
            Self::TwoHandedSword => Reach::OneTwo,
            Self::Wakizashi => Reach::One,
            Self::Voulge => Reach::TwoThree,
            Self::WarFan => Reach::Close,
            Self::Warhammer => Reach::OneTwo,
            Self::Whip => Reach::OneTwo,
        }
    }

    /// Returns parry modifier for this melee weapon.
    #[instrument]
    pub fn parry_modifier(&self) -> i32 {
        debug!("Getting melee weapon parry modifier");
        match self {
            Self::Axe => -1,
            Self::Baton => 0,
            Self::BattleAxe => -1,
            Self::Blackjack => 0,
            Self::BastardSword => 0,
            Self::Bardiche => 0,
            Self::Bill => 0,
            Self::Billhook => 0,
            Self::Bola => -2,
            Self::BrassKnuckles => 0,
            Self::Broadsword => 0,
            Self::Chakram => -2,
            Self::Cestus => 0,
            Self::Chain => -2,
            Self::Claymore => 0,
            Self::Club => 0,
            Self::CombatNet => -2,
            Self::Crowbar => 0,
            Self::Cutlass => 0,
            Self::Dagger => -1,
            Self::Estoc => 1,
            Self::Falchion => 0,
            Self::Fauchard => 0,
            Self::Fist => 0,
            Self::Flail => -2,
            Self::Gladius => 0,
            Self::Garrote => -4,
            Self::Glaive => 0,
            Self::Guisarme => 0,
            Self::GreatAxe => -2,
            Self::Halberd => 0,
            Self::Hatchet => -1,
            Self::Javelin => 0,
            Self::Jitte => 1,
            Self::Katana => 0,
            Self::Katar => 0,
            Self::Kick => -2,
            Self::Knife => -1,
            Self::Kopesh => 0,
            Self::Kusari => -2,
            Self::LargeKnife => -1,
            Self::Lasso => -2,
            Self::Lance => -2,
            Self::LongSpear => 0,
            Self::Longsword => 0,
            Self::Mace => 0,
            Self::MainGauche => 1,
            Self::Maul => -2,
            Self::Mattock => -1,
            Self::Morningstar => 0,
            Self::Naginata => 1,
            Self::Nunchaku => -2,
            Self::Pick => -1,
            Self::Pike => 0,
            Self::Poleax => 0,
            Self::Partisan => 0,
            Self::Quarterstaff => 2,
            Self::Rapier => 1,
            Self::Saber => 0,
            Self::Sai => 1,
            Self::Scimitar => 0,
            Self::Shortsword => 0,
            Self::Shovel => -1,
            Self::Smallsword => 1,
            Self::Spear => 0,
            Self::Staff => 2,
            Self::Stick => 1,
            Self::Tanto => -1,
            Self::Tonfa => 1,
            Self::Trident => 0,
            Self::TwoHandedSword => 0,
            Self::Wakizashi => 0,
            Self::Voulge => 0,
            Self::WarFan => 1,
            Self::Warhammer => -1,
            Self::Whip => -1,
        }
    }

    /// Returns required skill for this melee weapon.
    #[instrument]
    pub fn required_skill(&self) -> Skill {
        debug!("Getting melee weapon required skill");
        match self {
            Self::Axe => Skill::AxeMace,
            Self::Baton => Skill::Shortsword,
            Self::BattleAxe => Skill::AxeMace,
            Self::Blackjack => Skill::Brawling,
            Self::BastardSword => Skill::Broadsword,
            Self::Bardiche => Skill::Polearm,
            Self::Bill => Skill::Polearm,
            Self::Billhook => Skill::Polearm,
            Self::Bola => Skill::ThrownWeapon,
            Self::BrassKnuckles => Skill::Brawling,
            Self::Broadsword => Skill::Broadsword,
            Self::Chakram => Skill::ThrownWeapon,
            Self::Cestus => Skill::Brawling,
            Self::Chain => Skill::Flail,
            Self::Claymore => Skill::TwoHandedSword,
            Self::Club => Skill::AxeMace,
            Self::CombatNet => Skill::ThrownWeapon,
            Self::Crowbar => Skill::AxeMace,
            Self::Cutlass => Skill::Broadsword,
            Self::Dagger => Skill::Knife,
            Self::Estoc => Skill::Rapier,
            Self::Falchion => Skill::Broadsword,
            Self::Fauchard => Skill::Polearm,
            Self::Fist => Skill::Brawling,
            Self::Flail => Skill::Flail,
            Self::Gladius => Skill::Shortsword,
            Self::Garrote => Skill::Brawling,
            Self::Glaive => Skill::Polearm,
            Self::Guisarme => Skill::Polearm,
            Self::GreatAxe => Skill::TwoHandedAxeMace,
            Self::Halberd => Skill::Polearm,
            Self::Hatchet => Skill::AxeMace,
            Self::Javelin => Skill::Spear,
            Self::Jitte => Skill::MainGauche,
            Self::Katana => Skill::TwoHandedSword,
            Self::Katar => Skill::Knife,
            Self::Kick => Skill::Brawling,
            Self::Knife => Skill::Knife,
            Self::Kopesh => Skill::Broadsword,
            Self::Kusari => Skill::Kusari,
            Self::LargeKnife => Skill::Knife,
            Self::Lasso => Skill::ThrownWeapon,
            Self::Lance => Skill::Lance,
            Self::LongSpear => Skill::Spear,
            Self::Longsword => Skill::Broadsword,
            Self::Mace => Skill::AxeMace,
            Self::MainGauche => Skill::MainGauche,
            Self::Maul => Skill::TwoHandedAxeMace,
            Self::Mattock => Skill::AxeMace,
            Self::Morningstar => Skill::Flail,
            Self::Naginata => Skill::Polearm,
            Self::Nunchaku => Skill::Flail,
            Self::Pick => Skill::AxeMace,
            Self::Pike => Skill::Spear,
            Self::Poleax => Skill::Polearm,
            Self::Partisan => Skill::Polearm,
            Self::Quarterstaff => Skill::Staff,
            Self::Rapier => Skill::Rapier,
            Self::Saber => Skill::Saber,
            Self::Sai => Skill::Knife,
            Self::Scimitar => Skill::Broadsword,
            Self::Shortsword => Skill::Shortsword,
            Self::Shovel => Skill::AxeMace,
            Self::Smallsword => Skill::Smallsword,
            Self::Spear => Skill::Spear,
            Self::Staff => Skill::Staff,
            Self::Stick => Skill::Staff,
            Self::Tanto => Skill::Knife,
            Self::Tonfa => Skill::Shortsword,
            Self::Trident => Skill::Spear,
            Self::TwoHandedSword => Skill::TwoHandedSword,
            Self::Wakizashi => Skill::Shortsword,
            Self::Voulge => Skill::Polearm,
            Self::WarFan => Skill::Shortsword,
            Self::Warhammer => Skill::TwoHandedAxeMace,
            Self::Whip => Skill::Shortsword,
        }
    }
}
