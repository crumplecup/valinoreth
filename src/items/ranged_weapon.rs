//! Ranged weapon definitions and properties.
//!
//! # GURPS Rules
//!
//! Ranged weapons have fixed damage, accuracy, rate of fire, range,
//! shots, and recoil. This module implements the core stats needed for
//! item enumeration.
//!
//! # Citations
//!
//! BS 276-278 - Ranged weapons table

use crate::{Currency, DamageType, DieLevel, Skill, TechLevel, WeaponDamage, Weight};
use tracing::{debug, instrument};

/// Ranged weapon types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, strum::EnumIter)]
pub enum RangedWeapon {
    /// Assault rifle, 5d piercing. BS 278
    AssaultRifle,
    /// Atlatl, thrust+3 impaling. BS 277
    Atlatl,
    /// Blowgun, 1d-3 impaling. BS 277
    Blowgun,
    /// Bow, 1d impaling. BS 276
    Bow,
    /// Composite bow, 1d+3 impaling. BS 276
    CompositeBow,
    /// Chakram, thrust+1 cutting. LT (Indian throwing disc)
    Chakram,
    /// Crossbow, 1d+4 impaling. BS 276
    Crossbow,
    /// Dart, thrust-1 impaling. BS 277
    Dart,
    /// Derringer, 1d+2 piercing. BS 278
    Derringer,
    /// Flamethrower, 3d crushing (burning). BS 278
    Flamethrower,
    /// Grenade, 3d crushing. BS 278
    Grenade,
    /// Gastraphetes, 1d+6 impaling. LT (Greek belly bow)
    Gastraphetes,
    /// Hand cannon, 2d+2 piercing. LT (Early firearm, TL3)
    HandCannon,
    /// Hand crossbow, 1d impaling. BS 276
    HandCrossbow,
    /// Heavy crossbow, 1d+5 impaling. BS 276
    HeavyCrossbow,
    /// Hunting rifle, 7d piercing. BS 278
    HuntingRifle,
    /// Light crossbow, 1d+2 impaling. BS 276
    LightCrossbow,
    /// Longbow, 1d+2 impaling. BS 276
    Longbow,
    /// Machine gun, 7d piercing. BS 278
    MachineGun,
    /// Musket, 4d piercing. BS 278
    Musket,
    /// Pellet bow, 1d+4 impaling. BS 276
    PelletBow,
    /// Pistol, 2d+2 piercing. BS 278
    Pistol,
    /// Plumbata, thrust impaling. LT (Roman weighted dart)
    Plumbata,
    /// Prodd (stone-throwing crossbow), 1d crushing. BS 276
    Prodd,
    /// Revolver, 2d piercing. BS 278
    Revolver,
    /// Repeating crossbow, 1d+1 impaling. LT (Chinese chu-ko-nu)
    RepeatingCrossbow,
    /// Rifle, 5d piercing. BS 278
    Rifle,
    /// Rock (thrown), thrust crushing. BS 277
    Rock,
    /// Rocket launcher, 6d crushing. BS 278
    RocketLauncher,
    /// Recurve bow, 1d+2 impaling. LT (Composite recurve)
    RecurveBow,
    /// Self bow (primitive), 1d-1 impaling. BS 276
    SelfBow,
    /// Shotgun, 1d+1 piercing. BS 278
    Shotgun,
    /// Short bow, 1d-1 impaling. BS 276
    ShortBow,
    /// Shuriken, thrust impaling. BS 277
    Shuriken,
    /// Sling, swing piercing. BS 277
    Sling,
    /// SMG (submachine gun), 2d+2 piercing. BS 278
    SMG,
    /// Sniper rifle, 7d piercing. BS 278
    SniperRifle,
    /// Staff sling, swing+2 piercing. BS 277
    StaffSling,
    /// Boomerang, swing+1 crushing. BS 277
    Boomerang,
    /// Throwing axe, swing+2 cutting. BS 277
    ThrowingAxe,
    /// Throwing knife, thrust-1 impaling. BS 277
    ThrowingKnife,
    /// Throwing stick, swing+1 crushing. BS 277
    ThrowingStick,
    /// Warbow, 1d+3 impaling. LT (Heavy longbow)
    Warbow,
    /// Thrown bola (entangling), swing crushing. BS 277
    ThrownBola,
    /// Thrown net (entangling), special. BS 277
    ThrownNet,
}

impl RangedWeapon {
    /// Returns base cost for this ranged weapon.
    #[instrument]
    pub fn base_cost(&self) -> Currency {
        debug!("Getting ranged weapon base cost");
        match self {
            Self::AssaultRifle => Currency::dollars(900.0),
            Self::Atlatl => Currency::dollars(20.0),
            Self::Blowgun => Currency::dollars(30.0),
            Self::Bow => Currency::dollars(100.0),
            Self::CompositeBow => Currency::dollars(900.0),
            Self::Chakram => Currency::dollars(20.0),
            Self::Crossbow => Currency::dollars(150.0),
            Self::Dart => Currency::dollars(10.0),
            Self::Derringer => Currency::dollars(100.0),
            Self::Flamethrower => Currency::dollars(1000.0),
            Self::Grenade => Currency::dollars(30.0),
            Self::Gastraphetes => Currency::dollars(400.0),
            Self::HandCannon => Currency::dollars(1000.0),
            Self::HandCrossbow => Currency::dollars(150.0),
            Self::HeavyCrossbow => Currency::dollars(200.0),
            Self::HuntingRifle => Currency::dollars(700.0),
            Self::LightCrossbow => Currency::dollars(150.0),
            Self::Longbow => Currency::dollars(200.0),
            Self::MachineGun => Currency::dollars(4000.0),
            Self::Musket => Currency::dollars(300.0),
            Self::PelletBow => Currency::dollars(400.0),
            Self::Pistol => Currency::dollars(350.0),
            Self::Plumbata => Currency::dollars(5.0),
            Self::Prodd => Currency::dollars(100.0),
            Self::Revolver => Currency::dollars(300.0),
            Self::RepeatingCrossbow => Currency::dollars(300.0),
            Self::Rifle => Currency::dollars(500.0),
            Self::Rock => Currency::dollars(0.0),
            Self::RocketLauncher => Currency::dollars(2000.0),
            Self::RecurveBow => Currency::dollars(500.0),
            Self::SelfBow => Currency::dollars(50.0),
            Self::Shotgun => Currency::dollars(500.0),
            Self::ShortBow => Currency::dollars(50.0),
            Self::Shuriken => Currency::dollars(5.0),
            Self::Sling => Currency::dollars(20.0),
            Self::SMG => Currency::dollars(450.0),
            Self::SniperRifle => Currency::dollars(3500.0),
            Self::StaffSling => Currency::dollars(20.0),
            Self::Boomerang => Currency::dollars(15.0),
            Self::ThrowingAxe => Currency::dollars(60.0),
            Self::ThrowingKnife => Currency::dollars(30.0),
            Self::ThrowingStick => Currency::dollars(5.0),
            Self::Warbow => Currency::dollars(600.0),
            Self::ThrownBola => Currency::dollars(20.0),
            Self::ThrownNet => Currency::dollars(40.0),
        }
    }

    /// Returns weight for this ranged weapon.
    #[instrument]
    pub fn weight(&self) -> Weight {
        debug!("Getting ranged weapon weight");
        match self {
            Self::AssaultRifle => Weight::pounds(9.0),
            Self::Atlatl => Weight::pounds(1.0),
            Self::Blowgun => Weight::pounds(1.0),
            Self::Bow => Weight::pounds(2.0),
            Self::CompositeBow => Weight::pounds(2.0),
            Self::Chakram => Weight::pounds(1.0),
            Self::Crossbow => Weight::pounds(6.0),
            Self::Dart => Weight::pounds(0.1),
            Self::Derringer => Weight::pounds(0.5),
            Self::Flamethrower => Weight::pounds(70.0),
            Self::Grenade => Weight::pounds(1.0),
            Self::Gastraphetes => Weight::pounds(14.0),
            Self::HandCannon => Weight::pounds(15.0),
            Self::HandCrossbow => Weight::pounds(3.0),
            Self::HeavyCrossbow => Weight::pounds(8.0),
            Self::HuntingRifle => Weight::pounds(9.0),
            Self::LightCrossbow => Weight::pounds(4.0),
            Self::Longbow => Weight::pounds(3.0),
            Self::MachineGun => Weight::pounds(30.0),
            Self::Musket => Weight::pounds(10.0),
            Self::PelletBow => Weight::pounds(3.0),
            Self::Pistol => Weight::pounds(1.5),
            Self::Plumbata => Weight::pounds(0.5),
            Self::Prodd => Weight::pounds(7.0),
            Self::Revolver => Weight::pounds(2.0),
            Self::RepeatingCrossbow => Weight::pounds(8.0),
            Self::Rifle => Weight::pounds(9.0),
            Self::Rock => Weight::pounds(0.5),
            Self::RocketLauncher => Weight::pounds(15.0),
            Self::RecurveBow => Weight::pounds(3.0),
            Self::SelfBow => Weight::pounds(2.0),
            Self::Shotgun => Weight::pounds(8.0),
            Self::ShortBow => Weight::pounds(1.0),
            Self::Shuriken => Weight::pounds(0.1),
            Self::Sling => Weight::pounds(0.5),
            Self::SMG => Weight::pounds(7.0),
            Self::SniperRifle => Weight::pounds(11.0),
            Self::StaffSling => Weight::pounds(1.0),
            Self::Boomerang => Weight::pounds(1.0),
            Self::ThrowingAxe => Weight::pounds(2.0),
            Self::ThrowingKnife => Weight::pounds(0.5),
            Self::ThrowingStick => Weight::pounds(1.0),
            Self::Warbow => Weight::pounds(5.0),
            Self::ThrownBola => Weight::pounds(1.0),
            Self::ThrownNet => Weight::pounds(5.0),
        }
    }

    /// Returns tech level for this ranged weapon.
    #[instrument]
    pub fn tech_level(&self) -> TechLevel {
        debug!("Getting ranged weapon tech level");
        match self {
            Self::AssaultRifle => TechLevel::new(7),   // Digital Age
            Self::Atlatl => TechLevel::new(0),         // Stone Age
            Self::Blowgun => TechLevel::new(0),        // Stone Age
            Self::Bow => TechLevel::new(0),            // Stone Age
            Self::CompositeBow => TechLevel::new(2),   // Medieval
            Self::Chakram => TechLevel::new(1),        // Bronze Age
            Self::Crossbow => TechLevel::new(2),       // Medieval
            Self::Dart => TechLevel::new(0),           // Stone Age
            Self::Derringer => TechLevel::new(5),      // Mechanized Age
            Self::Flamethrower => TechLevel::new(6),   // Atomic Age
            Self::Grenade => TechLevel::new(6),        // Atomic Age
            Self::Gastraphetes => TechLevel::new(1),   // Greek/Iron Age
            Self::HandCannon => TechLevel::new(3),     // Medieval
            Self::HandCrossbow => TechLevel::new(2),   // Medieval
            Self::HeavyCrossbow => TechLevel::new(2),  // Medieval
            Self::HuntingRifle => TechLevel::new(5),   // Mechanized Age
            Self::LightCrossbow => TechLevel::new(2),  // Medieval
            Self::Longbow => TechLevel::new(0),        // Stone Age
            Self::MachineGun => TechLevel::new(6),     // Atomic Age
            Self::Musket => TechLevel::new(4),         // Age of Sail
            Self::PelletBow => TechLevel::new(7),      // Digital Age
            Self::Pistol => TechLevel::new(6),         // Atomic Age
            Self::Plumbata => TechLevel::new(1),       // Roman
            Self::Prodd => TechLevel::new(2),          // Medieval
            Self::Revolver => TechLevel::new(6),       // Atomic Age
            Self::RepeatingCrossbow => TechLevel::new(2), // Medieval China
            Self::Rifle => TechLevel::new(6),          // Atomic Age
            Self::Rock => TechLevel::new(0),           // Stone Age
            Self::RocketLauncher => TechLevel::new(7), // Digital Age
            Self::RecurveBow => TechLevel::new(1),     // Bronze Age
            Self::SelfBow => TechLevel::new(0),        // Stone Age
            Self::Shotgun => TechLevel::new(5),        // Mechanized Age
            Self::ShortBow => TechLevel::new(0),       // Stone Age
            Self::Shuriken => TechLevel::new(2),       // Medieval Japan
            Self::Sling => TechLevel::new(0),          // Stone Age
            Self::SMG => TechLevel::new(6),            // Atomic Age
            Self::SniperRifle => TechLevel::new(7),    // Digital Age
            Self::StaffSling => TechLevel::new(1),     // Bronze Age
            Self::Boomerang => TechLevel::new(0),      // Stone Age
            Self::ThrowingAxe => TechLevel::new(0),    // Stone Age
            Self::ThrowingKnife => TechLevel::new(0),  // Stone Age
            Self::ThrowingStick => TechLevel::new(0),  // Stone Age
            Self::Warbow => TechLevel::new(2),         // Medieval
            Self::ThrownBola => TechLevel::new(0),     // Stone Age
            Self::ThrownNet => TechLevel::new(1),      // Bronze Age
        }
    }

    /// Returns weapon damage for this ranged weapon.
    #[instrument]
    pub fn damage(&self) -> WeaponDamage {
        debug!("Getting ranged weapon damage");
        match self {
            Self::AssaultRifle => WeaponDamage::Fixed {
                dice: DieLevel::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Atlatl => WeaponDamage::Thrust {
                modifier: 3,
                damage_type: DamageType::Impaling,
            },
            Self::Blowgun => WeaponDamage::Fixed {
                dice: DieLevel::new(1, -3),
                damage_type: DamageType::Impaling,
            },
            Self::Bow => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 0),
                damage_type: DamageType::Impaling,
            },
            Self::CompositeBow => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 3),
                damage_type: DamageType::Impaling,
            },
            Self::Chakram => WeaponDamage::Thrust {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::Crossbow => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 4),
                damage_type: DamageType::Impaling,
            },
            Self::Dart => WeaponDamage::Thrust {
                modifier: -1,
                damage_type: DamageType::Impaling,
            },
            Self::Derringer => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 2),
                damage_type: DamageType::Piercing,
            },
            Self::Flamethrower => WeaponDamage::Fixed {
                dice: DieLevel::new(3, 0),
                damage_type: DamageType::Crushing, // Burning damage
            },
            Self::Grenade => WeaponDamage::Fixed {
                dice: DieLevel::new(3, 0), // 3d×2 fragmentation
                damage_type: DamageType::Crushing,
            },
            Self::Gastraphetes => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 6),
                damage_type: DamageType::Impaling,
            },
            Self::HandCannon => WeaponDamage::Fixed {
                dice: DieLevel::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::HandCrossbow => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 0),
                damage_type: DamageType::Impaling,
            },
            Self::HeavyCrossbow => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 5),
                damage_type: DamageType::Impaling,
            },
            Self::HuntingRifle => WeaponDamage::Fixed {
                dice: DieLevel::new(7, 0),
                damage_type: DamageType::Piercing,
            },
            Self::LightCrossbow => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 2),
                damage_type: DamageType::Impaling,
            },
            Self::Longbow => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 2),
                damage_type: DamageType::Impaling,
            },
            Self::MachineGun => WeaponDamage::Fixed {
                dice: DieLevel::new(7, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Musket => WeaponDamage::Fixed {
                dice: DieLevel::new(4, 0),
                damage_type: DamageType::Piercing,
            },
            Self::PelletBow => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 4),
                damage_type: DamageType::Impaling,
            },
            Self::Pistol => WeaponDamage::Fixed {
                dice: DieLevel::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::Plumbata => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Impaling,
            },
            Self::Prodd => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 2),
                damage_type: DamageType::Crushing,
            },
            Self::Revolver => WeaponDamage::Fixed {
                dice: DieLevel::new(2, 0),
                damage_type: DamageType::Piercing,
            },
            Self::RepeatingCrossbow => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 1),
                damage_type: DamageType::Impaling,
            },
            Self::Rifle => WeaponDamage::Fixed {
                dice: DieLevel::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Rock => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
            Self::RocketLauncher => WeaponDamage::Fixed {
                dice: DieLevel::new(6, 0),
                damage_type: DamageType::Crushing,
            },
            Self::RecurveBow => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 2),
                damage_type: DamageType::Impaling,
            },
            Self::SelfBow => WeaponDamage::Fixed {
                dice: DieLevel::new(1, -1),
                damage_type: DamageType::Impaling,
            },
            Self::Shotgun => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 1),
                damage_type: DamageType::Piercing,
            },
            Self::ShortBow => WeaponDamage::Fixed {
                dice: DieLevel::new(1, -1),
                damage_type: DamageType::Impaling,
            },
            Self::Shuriken => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Impaling,
            },
            Self::Sling => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Piercing,
            },
            Self::SMG => WeaponDamage::Fixed {
                dice: DieLevel::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::SniperRifle => WeaponDamage::Fixed {
                dice: DieLevel::new(7, 0),
                damage_type: DamageType::Piercing,
            },
            Self::StaffSling => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Piercing,
            },
            Self::Boomerang => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
            Self::ThrowingAxe => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
            Self::ThrowingKnife => WeaponDamage::Thrust {
                modifier: -1,
                damage_type: DamageType::Impaling,
            },
            Self::ThrowingStick => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::Warbow => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 3),
                damage_type: DamageType::Impaling,
            },
            Self::ThrownBola => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
            Self::ThrownNet => WeaponDamage::Swing {
                modifier: -2,
                damage_type: DamageType::Crushing, // Entangling, minimal damage
            },
        }
    }

    /// Returns weapon accuracy for this ranged weapon.
    #[instrument]
    pub fn accuracy(&self) -> i32 {
        debug!("Getting ranged weapon accuracy");
        match self {
            Self::AssaultRifle => 5,
            Self::Atlatl => 2,
            Self::Blowgun => 1,
            Self::Bow => 2,
            Self::CompositeBow => 3,
            Self::Chakram => 1,
            Self::Crossbow => 4,
            Self::Dart => 2,
            Self::Derringer => 1,
            Self::Flamethrower => 2,
            Self::Grenade => 1,
            Self::Gastraphetes => 4,
            Self::HandCannon => 1,
            Self::HandCrossbow => 3,
            Self::HeavyCrossbow => 4,
            Self::HuntingRifle => 5,
            Self::LightCrossbow => 4,
            Self::Longbow => 3,
            Self::MachineGun => 4,
            Self::Musket => 3,
            Self::PelletBow => 4,
            Self::Pistol => 2,
            Self::Plumbata => 0,
            Self::Prodd => 3,
            Self::Revolver => 2,
            Self::RepeatingCrossbow => 2,
            Self::Rifle => 5,
            Self::Rock => 0,
            Self::RocketLauncher => 4,
            Self::RecurveBow => 2,
            Self::SelfBow => 1,
            Self::Shotgun => 3,
            Self::ShortBow => 1,
            Self::Shuriken => 1,
            Self::Sling => 0,
            Self::SMG => 4,
            Self::SniperRifle => 6,
            Self::StaffSling => 1,
            Self::Boomerang => 1,
            Self::ThrowingAxe => 2,
            Self::ThrowingKnife => 0,
            Self::ThrowingStick => 1,
            Self::Warbow => 3,
            Self::ThrownBola => 1,
            Self::ThrownNet => 1,
        }
    }

    /// Returns required skill for this ranged weapon.
    #[instrument]
    pub fn required_skill(&self) -> Skill {
        debug!("Getting ranged weapon required skill");
        match self {
            Self::AssaultRifle => Skill::Guns,
            Self::Atlatl => Skill::ThrownWeapon,
            Self::Blowgun => Skill::Blowpipe,
            Self::Bow => Skill::Bow,
            Self::CompositeBow => Skill::Bow,
            Self::Chakram => Skill::ThrownWeapon,
            Self::Crossbow => Skill::Crossbow,
            Self::Dart => Skill::ThrownWeapon,
            Self::Derringer => Skill::Guns,
            Self::Flamethrower => Skill::Guns,
            Self::Grenade => Skill::ThrownWeapon,
            Self::Gastraphetes => Skill::Crossbow,
            Self::HandCannon => Skill::Guns,
            Self::HandCrossbow => Skill::Crossbow,
            Self::HeavyCrossbow => Skill::Crossbow,
            Self::HuntingRifle => Skill::Guns,
            Self::LightCrossbow => Skill::Crossbow,
            Self::Longbow => Skill::Bow,
            Self::MachineGun => Skill::Guns,
            Self::Musket => Skill::Guns,
            Self::PelletBow => Skill::Bow,
            Self::Pistol => Skill::Guns,
            Self::Plumbata => Skill::ThrownWeapon,
            Self::Prodd => Skill::Crossbow,
            Self::Revolver => Skill::Guns,
            Self::RepeatingCrossbow => Skill::Crossbow,
            Self::Rifle => Skill::Guns,
            Self::Rock => Skill::ThrownWeapon,
            Self::RocketLauncher => Skill::Guns,
            Self::RecurveBow => Skill::Bow,
            Self::SelfBow => Skill::Bow,
            Self::Shotgun => Skill::Guns,
            Self::ShortBow => Skill::Bow,
            Self::Shuriken => Skill::ThrownWeapon,
            Self::Sling => Skill::Sling,
            Self::SMG => Skill::Guns,
            Self::SniperRifle => Skill::Guns,
            Self::StaffSling => Skill::Sling,
            Self::Boomerang => Skill::ThrownWeapon,
            Self::ThrowingAxe => Skill::ThrownWeapon,
            Self::ThrowingKnife => Skill::ThrownWeapon,
            Self::ThrowingStick => Skill::ThrownWeapon,
            Self::Warbow => Skill::Bow,
            Self::ThrownBola => Skill::ThrownWeapon,
            Self::ThrownNet => Skill::ThrownWeapon,
        }
    }
}
