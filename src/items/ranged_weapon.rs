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
    /// Arbalest, 1d+7 impaling. LT (Heavy siege crossbow)
    Arbalest,
    /// Arquebus, 3d piercing. LT (Spanish matchlock, TL4)
    Arquebus,
    /// Assault rifle, 5d piercing. BS 278
    AssaultRifle,
    /// Arisaka Type 38, 4d piercing. HT (Japanese bolt-action rifle, TL6)
    ArisakaType38,
    /// Atlatl, thrust+3 impaling. BS 277
    Atlatl,
    /// Ballista, 3d+4 impaling. LT (Siege crossbow, TL1)
    Ballista,
    /// Blowgun, 1d-3 impaling. BS 277
    Blowgun,
    /// Blunderbuss, 1d+2 piercing. LT (Early shotgun, TL4)
    Blunderbuss,
    /// Bola, swing crushing. LT (Whirled bola, TL0)
    Bola,
    /// Bow, 1d impaling. BS 276
    Bow,
    /// Browning Hi-Power, 2d+2 piercing. HT (9mm semi-auto pistol, TL6)
    BrowningHiPower,
    /// Carbine, 4d piercing. LT (Short rifle, TL5)
    Carbine,
    /// Caliver, 3d piercing. LT (Light arquebus, TL4)
    Caliver,
    /// Cap-lock pistol, 1d+2 piercing. HT (Single-shot percussion pistol, TL5)
    CapLockPistol,
    /// Catapult, 3d crushing. LT (Torsion stone thrower, TL1)
    Catapult,
    /// Composite bow, 1d+3 impaling. BS 276
    CompositeBow,
    /// Chakram, thrust+1 cutting. LT (Indian throwing disc)
    Chakram,
    /// Chassepot Rifle, 4d piercing. HT (French bolt-action rifle, TL5)
    ChassepotRifle,
    /// Chu-ko-nu, 1d impaling. LT (Chinese repeating crossbow variant)
    Chukonu,
    /// Colt Walker, 2d+2 piercing. HT (Powerful cap-and-ball revolver, TL5)
    ColtWalker,
    /// Colt M1911, 2d+2 piercing. HT (.45 ACP semi-auto pistol, TL6)
    ColtM1911,
    /// Crossbow, 1d+4 impaling. BS 276
    Crossbow,
    /// Daikyu, 1d+3 impaling. LT (Japanese great bow)
    Daikyu,
    /// Dart, thrust-1 impaling. BS 277
    Dart,
    /// Dynamite, 6d crushing. HT (High explosive, TL5)
    Dynamite,
    /// Dreyse Needle Rifle, 3d+2 piercing. HT (Prussian breech-loader, TL5)
    DreyseNeedleRifle,
    /// Derringer, 1d+2 piercing. BS 278
    Derringer,
    /// Double-action revolver, 2d piercing. HT (DA revolver .38, TL5)
    DoubleActionRevolver,
    /// Double-barrel shotgun, 1d+1 piercing. HT (Break-action shotgun, TL5)
    DoubleBarrelShotgun,
    /// Espringal, 2d impaling. LT (Medieval dart-thrower, TL2)
    Espringal,
    /// Enfield Pattern 1853, 4d piercing. HT (Rifled musket, TL5)
    EnfieldPattern1853,
    /// Flamethrower, 3d crushing (burning). BS 278
    Flamethrower,
    /// Field Gun, 6d crushing. HT (Artillery piece, TL5)
    FieldGun,
    /// Flintlock, 1d+2 piercing. LT (Flintlock pistol, TL4)
    Flintlock,
    /// Fusil, 4d piercing. LT (Light musket, TL4)
    Fusil,
    /// Gatling gun, 5d piercing. HT (Hand-cranked machine gun, TL5)
    GatlingGun,
    /// Gardner Gun, 5d piercing. HT (Hand-cranked machine gun, TL5)
    GardnerGun,
    /// Grenade, 3d crushing. BS 278
    Grenade,
    /// Hackbut, 3d piercing. LT (Heavy hand cannon, TL4)
    Hackbut,
    /// Gastraphetes, 1d+6 impaling. LT (Greek belly bow)
    Gastraphetes,
    /// Great bow, 1d+4 impaling. LT (Extra-heavy war bow, TL2)
    GreatBow,
    /// Horn bow, 1d+2 impaling. LT (Horn composite bow, TL1)
    HornBow,
    /// Hand cannon, 2d+2 piercing. LT (Early firearm, TL3)
    HandCannon,
    /// Hankyu, 1d impaling. LT (Japanese short bow)
    Hankyu,
    /// Hand mortar, 2d crushing. LT (Portable mortar, TL4)
    HandMortar,
    /// Henry rifle, 3d+1 piercing. HT (Lever-action repeater, TL5)
    HenryRifle,
    /// Hand crossbow, 1d impaling. BS 276
    HandCrossbow,
    /// Heavy crossbow, 1d+5 impaling. BS 276
    HeavyCrossbow,
    /// Hurlbat, swing+2 cutting. LT (Indian throwing weapon)
    Hurlbat,
    /// Hunting rifle, 7d piercing. BS 278
    HuntingRifle,
    /// Jezail, 5d piercing. LT (Afghan long gun, TL4)
    Jezail,
    /// LeMat revolver, 2d+1 piercing. HT (9-shot + shotgun revolver, TL5)
    LematRevolver,
    /// Lee-Enfield SMLE, 4d piercing. HT (British .303 bolt-action, TL6)
    LeeEnfieldSMLE,
    /// Lever-action rifle, 3d piercing. HT (Repeating rifle, TL5)
    LeverActionRifle,
    /// Light crossbow, 1d+2 impaling. BS 276
    LightCrossbow,
    /// Longbow, 1d+2 impaling. BS 276
    Longbow,
    /// Luger P08, 2d piercing. HT (German 9mm semi-auto pistol, TL6)
    LugerP08,
    /// M1 Garand, 5d piercing. HT (American semi-auto rifle, TL6)
    M1Garand,
    /// Mangonel, 3d crushing. LT (Torsion catapult, TL1)
    Mangonel,
    /// Matchlock, 3d+2 piercing. LT (Early musket, TL4)
    Matchlock,
    /// Martini-Henry rifle, 4d piercing. HT (Breech-loading rifle, TL5)
    MartiniHenryRifle,
    /// Mauser Model 1871, 4d+1 piercing. HT (Bolt-action rifle, TL5)
    MauserModel1871,
    /// Mauser C96, 2d piercing. HT ("Broomhandle" semi-auto pistol, TL6)
    MauserC96,
    /// Mauser Gewehr 98, 4d+1 piercing. HT (German WWI bolt-action, TL6)
    MauserGewehr98,
    /// Maxim gun, 6d piercing. HT (Early recoil-operated MG, TL5)
    MaximGun,
    /// Machine gun, 7d piercing. BS 278
    MachineGun,
    /// Mitrailleuse, 5d piercing. HT (Volley gun, TL5)
    Mitrailleuse,
    /// Mosin-Nagant, 4d+1 piercing. HT (Russian bolt-action rifle, TL6)
    MosinNagant,
    /// Mountain Gun, 5d crushing. HT (Light artillery, TL5)
    MountainGun,
    /// Musket, 4d piercing. BS 278
    Musket,
    /// Musketoon, 3d+2 piercing. LT (Short musket, TL4)
    Musketoon,
    /// Naval Gun, 8d crushing. HT (Ship-mounted artillery, TL5)
    NavalGun,
    /// Nordenfelt Gun, 5d piercing. HT (Multi-barrel volley gun, TL5)
    NordenfeltGun,
    /// Pellet bow, 1d+4 impaling. BS 276
    PelletBow,
    /// Pistol, 2d+2 piercing. BS 278
    Pistol,
    /// Pellet sling, swing+1 crushing. LT (Bullet sling, TL0)
    PelletSling,
    /// Petronel, 2d+2 piercing. LT (Cavalry firearm, TL4)
    Petronel,
    /// Pepperbox revolver, 1d+1 piercing. HT (4-6 barrel pistol, TL5)
    PepperboxRevolver,
    /// Percussion revolver, 2d piercing. HT (Cap-and-ball revolver, TL5)
    PercussionRevolver,
    /// Plumbata, thrust impaling. LT (Roman weighted dart)
    Plumbata,
    /// Prodd (stone-throwing crossbow), 1d crushing. BS 276
    Prodd,
    /// Pump-action shotgun, 1d+1 piercing. HT (Early pump shotgun, TL5)
    PumpActionShotgun,
    /// Remington Rolling Block Pistol, 2d piercing. HT (Single-shot pistol, TL5)
    RemingtonRollingBlockPistol,
    /// Remington Rolling Block Rifle, 4d piercing. HT (Single-shot rifle, TL5)
    RemingtonRollingBlockRifle,
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
    /// Scorpion, 2d+2 impaling. LT (Roman light ballista, TL1)
    Scorpion,
    /// Recurve bow, 1d+2 impaling. LT (Composite recurve)
    RecurveBow,
    /// Self bow (primitive), 1d-1 impaling. BS 276
    SelfBow,
    /// Shotgun, 1d+1 piercing. BS 278
    Shotgun,
    /// Short bow, 1d-1 impaling. BS 276
    ShortBow,
    /// Sharps rifle, 4d piercing. HT (Single-shot breech-loader, TL5)
    SharpsRifle,
    /// Shuriken, thrust impaling. BS 277
    Shuriken,
    /// Siege crossbow, 3d+6 impaling. LT (Extra-heavy crossbow, TL2)
    SiegeCrossbow,
    /// Single-action revolver, 2d+1 piercing. HT (Colt SAA .45, TL5)
    SingleActionRevolver,
    /// Sling, swing piercing. BS 277
    Sling,
    /// Smith & Wesson Model 1, 1d piercing. HT (Tip-up .22 revolver, TL5)
    SmithWessonModel1,
    /// Smith & Wesson Model 3 (Schofield), 2d piercing. HT (Break-top .45 revolver, TL5)
    SmithWessonModel3,
    /// SMG (submachine gun), 2d+2 piercing. BS 278
    SMG,
    /// Sniper rifle, 7d piercing. BS 278
    SniperRifle,
    /// Spencer Repeating Rifle, 3d+2 piercing. HT (Lever-action repeater, TL5)
    SpencerRepeatingRifle,
    /// Springfield Model 1861, 4d piercing. HT (Rifled musket, TL5)
    SpringfieldModel1861,
    /// Springfield M1903, 4d+1 piercing. HT (American bolt-action, TL6)
    SpringfieldM1903,
    /// Trapdoor Springfield, 4d piercing. HT (Breech-loading rifle, TL5)
    TrapdoorSpringfield,
    /// Staff sling, swing+2 piercing. BS 277
    StaffSling,
    /// Stonebow, 1d+2 crushing. LT (Stone/pellet crossbow)
    Stonebow,
    /// Boomerang, swing+1 crushing. BS 277
    Boomerang,
    /// Throwing axe, swing+2 cutting. BS 277
    ThrowingAxe,
    /// Throwing hammer, swing+2 crushing. LT (Thrown warhammer, TL0)
    ThrowingHammer,
    /// Throwing knife, thrust-1 impaling. BS 277
    ThrowingKnife,
    /// Throwing spear, thrust+3 impaling. LT (Light javelin, TL0)
    ThrowingSpear,
    /// Throwing stick, swing+1 crushing. BS 277
    ThrowingStick,
    /// Trebuchet, 4d crushing. LT (Counterweight catapult, TL2)
    Trebuchet,
    /// TNT, 8d crushing. HT (Trinitrotoluene explosive, TL5)
    TNT,
    /// Volcanic pistol, 1d+1 piercing. HT (Lever-action pistol, TL5)
    VolcanicPistol,
    /// Warbow, 1d+3 impaling. LT (Heavy longbow)
    Warbow,
    /// Walther PPK, 2d-1 piercing. HT (Compact .32 ACP pistol, TL6)
    WaltherPPK,
    /// Webley RIC Revolver, 2d piercing. HT (British service revolver, TL5)
    WebleyRIC,
    /// Webley Mk VI, 2d piercing. HT (British service revolver .455, TL6)
    WebleyMkVI,
    /// Wheellock, 1d+2 piercing. LT (Wheellock pistol, TL4)
    Wheellock,
    /// Winchester Model 1866, 3d piercing. HT ("Yellow Boy" lever-action, TL5)
    Winchester1866,
    /// Winchester Model 1873, 3d+1 piercing. HT ("Gun that Won the West", TL5)
    Winchester1873,
    /// Winchester Model 1887, 1d+1 piercing. HT (Lever-action shotgun, TL5)
    Winchester1887,
    /// Winchester repeater, 3d+1 piercing. HT (Lever-action rifle, TL5)
    WinchesterRepeater,
    /// Yumi, 1d+1 impaling. LT (Japanese asymmetric bow, TL2)
    Yumi,
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
            Self::Arbalest => Currency::dollars(300.0),
            Self::Arquebus => Currency::dollars(500.0),
            Self::AssaultRifle => Currency::dollars(900.0),
            Self::ArisakaType38 => Currency::dollars(400.0),
                        Self::Atlatl => Currency::dollars(20.0),
            Self::Ballista => Currency::dollars(2000.0),
                        Self::Blowgun => Currency::dollars(30.0),
                        Self::Blunderbuss => Currency::dollars(300.0),
            Self::Bola => Currency::dollars(20.0),
                        Self::Bow => Currency::dollars(100.0),
            Self::BrowningHiPower => Currency::dollars(350.0),
                                    Self::Carbine => Currency::dollars(300.0),
            Self::Caliver => Currency::dollars(300.0),
            Self::CapLockPistol => Currency::dollars(150.0),
            Self::Catapult => Currency::dollars(3000.0),
            Self::CompositeBow => Currency::dollars(900.0),
                        Self::Chakram => Currency::dollars(20.0),
            Self::ChassepotRifle => Currency::dollars(400.0),
            Self::Chukonu => Currency::dollars(250.0),
            Self::ColtWalker => Currency::dollars(300.0),
            Self::ColtM1911 => Currency::dollars(300.0),
            Self::Crossbow => Currency::dollars(150.0),
            Self::Daikyu => Currency::dollars(450.0),
            Self::Dart => Currency::dollars(10.0),
            Self::Dynamite => Currency::dollars(20.0),
            Self::DreyseNeedleRifle => Currency::dollars(450.0),
            Self::Derringer => Currency::dollars(100.0),
            Self::DoubleActionRevolver => Currency::dollars(250.0),
            Self::DoubleBarrelShotgun => Currency::dollars(150.0),
            Self::Espringal => Currency::dollars(1500.0),
            Self::EnfieldPattern1853 => Currency::dollars(375.0),
                        Self::Flamethrower => Currency::dollars(1000.0),
            Self::FieldGun => Currency::dollars(5000.0),
            Self::Flintlock => Currency::dollars(200.0),
            Self::Fusil => Currency::dollars(350.0),
            Self::GatlingGun => Currency::dollars(3000.0),
            Self::GardnerGun => Currency::dollars(2800.0),
            Self::Grenade => Currency::dollars(30.0),
            Self::Hackbut => Currency::dollars(500.0),
                        Self::Gastraphetes => Currency::dollars(400.0),
            Self::GreatBow => Currency::dollars(700.0),
            Self::HornBow => Currency::dollars(800.0),
            Self::HandCannon => Currency::dollars(1000.0),
                        Self::Hankyu => Currency::dollars(250.0),
            Self::HandMortar => Currency::dollars(800.0),
            Self::HenryRifle => Currency::dollars(400.0),
            Self::HandCrossbow => Currency::dollars(150.0),
                        Self::HeavyCrossbow => Currency::dollars(200.0),
            Self::Hurlbat => Currency::dollars(30.0),
                        Self::HuntingRifle => Currency::dollars(700.0),
            Self::Jezail => Currency::dollars(400.0),
            Self::LematRevolver => Currency::dollars(350.0),
            Self::LeeEnfieldSMLE => Currency::dollars(425.0),
            Self::LeverActionRifle => Currency::dollars(350.0),
            Self::LightCrossbow => Currency::dollars(150.0),
                                    Self::Longbow => Currency::dollars(200.0),
            Self::LugerP08 => Currency::dollars(350.0),
            Self::M1Garand => Currency::dollars(500.0),
            Self::Mangonel => Currency::dollars(3500.0),
            Self::Matchlock => Currency::dollars(400.0),
            Self::MartiniHenryRifle => Currency::dollars(450.0),
            Self::MauserModel1871 => Currency::dollars(500.0),
            Self::MauserC96 => Currency::dollars(325.0),
            Self::MauserGewehr98 => Currency::dollars(450.0),
            Self::MaximGun => Currency::dollars(4500.0),
            Self::MachineGun => Currency::dollars(4000.0),
            Self::Mitrailleuse => Currency::dollars(3500.0),
            Self::MosinNagant => Currency::dollars(400.0),
            Self::MountainGun => Currency::dollars(3500.0),
                        Self::Musket => Currency::dollars(300.0),
            Self::Musketoon => Currency::dollars(250.0),
            Self::NavalGun => Currency::dollars(8000.0),
            Self::NordenfeltGun => Currency::dollars(3200.0),
            Self::PelletBow => Currency::dollars(400.0),
                        Self::Pistol => Currency::dollars(350.0),
                        Self::PelletSling => Currency::dollars(15.0),
            Self::Petronel => Currency::dollars(300.0),
            Self::PepperboxRevolver => Currency::dollars(200.0),
            Self::PercussionRevolver => Currency::dollars(250.0),
            Self::Plumbata => Currency::dollars(5.0),
            Self::Prodd => Currency::dollars(100.0),
            Self::PumpActionShotgun => Currency::dollars(300.0),
            Self::RemingtonRollingBlockPistol => Currency::dollars(200.0),
            Self::RemingtonRollingBlockRifle => Currency::dollars(425.0),
            Self::Revolver => Currency::dollars(300.0),
            Self::RepeatingCrossbow => Currency::dollars(300.0),
            Self::Rifle => Currency::dollars(500.0),
            Self::Rock => Currency::dollars(0.0),
                        Self::RocketLauncher => Currency::dollars(2000.0),
            Self::Scorpion => Currency::dollars(1000.0),
            Self::RecurveBow => Currency::dollars(500.0),
            Self::SelfBow => Currency::dollars(50.0),
            Self::Shotgun => Currency::dollars(500.0),
                        Self::ShortBow => Currency::dollars(50.0),
            Self::SharpsRifle => Currency::dollars(425.0),
            Self::SiegeCrossbow => Currency::dollars(500.0),
            Self::SingleActionRevolver => Currency::dollars(300.0),
            Self::Shuriken => Currency::dollars(5.0),
            Self::Sling => Currency::dollars(20.0),
            Self::SmithWessonModel1 => Currency::dollars(100.0),
            Self::SmithWessonModel3 => Currency::dollars(250.0),
            Self::SMG => Currency::dollars(450.0),
            Self::SniperRifle => Currency::dollars(3500.0),
            Self::SpencerRepeatingRifle => Currency::dollars(400.0),
            Self::SpringfieldModel1861 => Currency::dollars(375.0),
            Self::SpringfieldM1903 => Currency::dollars(475.0),
            Self::TrapdoorSpringfield => Currency::dollars(425.0),
                        Self::StaffSling => Currency::dollars(20.0),
            Self::Stonebow => Currency::dollars(120.0),
            Self::Boomerang => Currency::dollars(15.0),
                        Self::ThrowingAxe => Currency::dollars(60.0),
            Self::ThrowingHammer => Currency::dollars(70.0),
                        Self::ThrowingKnife => Currency::dollars(30.0),
            Self::ThrowingSpear => Currency::dollars(50.0),
                        Self::ThrowingStick => Currency::dollars(5.0),
            Self::Trebuchet => Currency::dollars(5000.0),
            Self::TNT => Currency::dollars(30.0),
            Self::VolcanicPistol => Currency::dollars(275.0),
                        Self::Warbow => Currency::dollars(600.0),
            Self::WaltherPPK => Currency::dollars(250.0),
            Self::WebleyRIC => Currency::dollars(200.0),
            Self::WebleyMkVI => Currency::dollars(225.0),
            Self::Wheellock => Currency::dollars(250.0),
            Self::Winchester1866 => Currency::dollars(400.0),
            Self::Winchester1873 => Currency::dollars(425.0),
            Self::Winchester1887 => Currency::dollars(350.0),
            Self::WinchesterRepeater => Currency::dollars(450.0),
            Self::Yumi => Currency::dollars(300.0),
            Self::ThrownBola => Currency::dollars(20.0),
            Self::ThrownNet => Currency::dollars(40.0),
        }
    }

    /// Returns weight for this ranged weapon.
    #[instrument]
    pub fn weight(&self) -> Weight {
        debug!("Getting ranged weapon weight");
        match self {
            Self::Arbalest => Weight::pounds(12.0),
            Self::Arquebus => Weight::pounds(11.0),
            Self::AssaultRifle => Weight::pounds(9.0),
            Self::ArisakaType38 => Weight::pounds(9.0),
                        Self::Atlatl => Weight::pounds(1.0),
            Self::Ballista => Weight::pounds(350.0),
                        Self::Blowgun => Weight::pounds(1.0),
                        Self::Blunderbuss => Weight::pounds(8.0),
            Self::Bola => Weight::pounds(1.0),
                        Self::Bow => Weight::pounds(2.0),
            Self::BrowningHiPower => Weight::pounds(2.0),
                                    Self::Carbine => Weight::pounds(7.0),
            Self::Caliver => Weight::pounds(8.0),
            Self::CapLockPistol => Weight::pounds(2.5),
            Self::Catapult => Weight::pounds(500.0),
            Self::CompositeBow => Weight::pounds(2.0),
                        Self::Chakram => Weight::pounds(1.0),
            Self::ChassepotRifle => Weight::pounds(9.5),
            Self::Chukonu => Weight::pounds(7.0),
            Self::ColtWalker => Weight::pounds(4.5),
            Self::ColtM1911 => Weight::pounds(2.5),
            Self::Crossbow => Weight::pounds(6.0),
            Self::Daikyu => Weight::pounds(3.5),
            Self::Dart => Weight::pounds(0.1),
            Self::Dynamite => Weight::pounds(0.5),
            Self::DreyseNeedleRifle => Weight::pounds(10.5),
                        Self::Derringer => Weight::pounds(0.5),
            Self::DoubleActionRevolver => Weight::pounds(2.0),
            Self::DoubleBarrelShotgun => Weight::pounds(7.5),
            Self::Espringal => Weight::pounds(300.0),
            Self::EnfieldPattern1853 => Weight::pounds(9.5),
                        Self::Flamethrower => Weight::pounds(70.0),
            Self::FieldGun => Weight::pounds(2000.0),
            Self::Flintlock => Weight::pounds(2.5),
            Self::Fusil => Weight::pounds(8.0),
            Self::GatlingGun => Weight::pounds(200.0),
            Self::GardnerGun => Weight::pounds(180.0),
                        Self::Grenade => Weight::pounds(1.0),
            Self::Hackbut => Weight::pounds(25.0),
                        Self::Gastraphetes => Weight::pounds(14.0),
            Self::GreatBow => Weight::pounds(6.0),
            Self::HornBow => Weight::pounds(3.0),
            Self::HandCannon => Weight::pounds(15.0),
                        Self::Hankyu => Weight::pounds(1.5),
            Self::HandMortar => Weight::pounds(20.0),
            Self::HenryRifle => Weight::pounds(9.5),
            Self::HandCrossbow => Weight::pounds(3.0),
                        Self::HeavyCrossbow => Weight::pounds(8.0),
            Self::Hurlbat => Weight::pounds(1.5),
                        Self::HuntingRifle => Weight::pounds(9.0),
            Self::Jezail => Weight::pounds(11.0),
            Self::LematRevolver => Weight::pounds(3.0),
            Self::LeeEnfieldSMLE => Weight::pounds(8.75),
            Self::LeverActionRifle => Weight::pounds(8.0),
            Self::LightCrossbow => Weight::pounds(4.0),
                                    Self::Longbow => Weight::pounds(3.0),
            Self::LugerP08 => Weight::pounds(1.75),
            Self::M1Garand => Weight::pounds(9.5),
            Self::Mangonel => Weight::pounds(600.0),
            Self::Matchlock => Weight::pounds(12.0),
            Self::MartiniHenryRifle => Weight::pounds(9.0),
            Self::MauserModel1871 => Weight::pounds(10.0),
            Self::MauserC96 => Weight::pounds(2.5),
            Self::MauserGewehr98 => Weight::pounds(9.0),
            Self::MaximGun => Weight::pounds(60.0),
            Self::MachineGun => Weight::pounds(30.0),
            Self::Mitrailleuse => Weight::pounds(300.0),
            Self::MosinNagant => Weight::pounds(8.75),
            Self::MountainGun => Weight::pounds(800.0),
                        Self::Musket => Weight::pounds(10.0),
            Self::Musketoon => Weight::pounds(6.0),
            Self::NavalGun => Weight::pounds(5000.0),
            Self::NordenfeltGun => Weight::pounds(250.0),
            Self::PelletBow => Weight::pounds(3.0),
                        Self::Pistol => Weight::pounds(1.5),
                        Self::PelletSling => Weight::pounds(0.5),
            Self::Petronel => Weight::pounds(5.0),
            Self::PepperboxRevolver => Weight::pounds(2.0),
            Self::PercussionRevolver => Weight::pounds(2.5),
            Self::Plumbata => Weight::pounds(0.5),
            Self::Prodd => Weight::pounds(7.0),
            Self::PumpActionShotgun => Weight::pounds(8.0),
            Self::RemingtonRollingBlockPistol => Weight::pounds(2.5),
            Self::RemingtonRollingBlockRifle => Weight::pounds(9.0),
            Self::Revolver => Weight::pounds(2.0),
            Self::RepeatingCrossbow => Weight::pounds(8.0),
            Self::Rifle => Weight::pounds(9.0),
            Self::Rock => Weight::pounds(0.5),
                        Self::RocketLauncher => Weight::pounds(15.0),
            Self::Scorpion => Weight::pounds(200.0),
            Self::RecurveBow => Weight::pounds(3.0),
            Self::SelfBow => Weight::pounds(2.0),
            Self::Shotgun => Weight::pounds(8.0),
                        Self::ShortBow => Weight::pounds(1.0),
            Self::SharpsRifle => Weight::pounds(10.5),
            Self::SiegeCrossbow => Weight::pounds(15.0),
            Self::SingleActionRevolver => Weight::pounds(2.5),
            Self::Shuriken => Weight::pounds(0.1),
            Self::Sling => Weight::pounds(0.5),
            Self::SmithWessonModel1 => Weight::pounds(1.0),
            Self::SmithWessonModel3 => Weight::pounds(2.5),
            Self::SMG => Weight::pounds(7.0),
            Self::SniperRifle => Weight::pounds(11.0),
            Self::SpencerRepeatingRifle => Weight::pounds(10.0),
            Self::SpringfieldModel1861 => Weight::pounds(9.5),
            Self::SpringfieldM1903 => Weight::pounds(8.75),
            Self::TrapdoorSpringfield => Weight::pounds(9.0),
                        Self::StaffSling => Weight::pounds(1.0),
            Self::Stonebow => Weight::pounds(6.0),
            Self::Boomerang => Weight::pounds(1.0),
                        Self::ThrowingAxe => Weight::pounds(2.0),
            Self::ThrowingHammer => Weight::pounds(3.0),
                        Self::ThrowingKnife => Weight::pounds(0.5),
            Self::ThrowingSpear => Weight::pounds(3.0),
                        Self::ThrowingStick => Weight::pounds(1.0),
            Self::Trebuchet => Weight::pounds(1000.0),
            Self::TNT => Weight::pounds(1.0),
            Self::VolcanicPistol => Weight::pounds(3.5),
                        Self::Warbow => Weight::pounds(5.0),
            Self::WaltherPPK => Weight::pounds(1.25),
            Self::WebleyRIC => Weight::pounds(2.0),
            Self::WebleyMkVI => Weight::pounds(2.5),
            Self::Wheellock => Weight::pounds(3.0),
            Self::Winchester1866 => Weight::pounds(9.0),
            Self::Winchester1873 => Weight::pounds(9.5),
            Self::Winchester1887 => Weight::pounds(9.5),
            Self::WinchesterRepeater => Weight::pounds(9.0),
            Self::Yumi => Weight::pounds(2.5),
            Self::ThrownBola => Weight::pounds(1.0),
            Self::ThrownNet => Weight::pounds(5.0),
        }
    }

    /// Returns tech level for this ranged weapon.
    #[instrument]
    pub fn tech_level(&self) -> TechLevel {
        debug!("Getting ranged weapon tech level");
        match self {
            Self::Arbalest => TechLevel::new(3),       // Medieval
            Self::Arquebus => TechLevel::new(4),       // Renaissance
            Self::AssaultRifle => TechLevel::new(7),   // Digital Age
            Self::ArisakaType38 => TechLevel::new(6),  // Atomic Age
                        Self::Atlatl => TechLevel::new(0),         // Stone Age
            Self::Ballista => TechLevel::new(1),       // Roman/Iron Age
                        Self::Blowgun => TechLevel::new(0),        // Stone Age
                        Self::Blunderbuss => TechLevel::new(4),    // Renaissance
            Self::Bola => TechLevel::new(0),           // Stone Age
                        Self::Bow => TechLevel::new(0),            // Stone Age
            Self::BrowningHiPower => TechLevel::new(6), // Atomic Age
                                    Self::Carbine => TechLevel::new(5),        // Mechanized Age
            Self::Caliver => TechLevel::new(4),        // Renaissance
            Self::CapLockPistol => TechLevel::new(5),  // Industrial Revolution
            Self::Catapult => TechLevel::new(1),       // Roman/Iron Age
            Self::CompositeBow => TechLevel::new(2),   // Medieval
                        Self::Chakram => TechLevel::new(1),        // Bronze Age
            Self::ChassepotRifle => TechLevel::new(5), // Industrial Revolution
            Self::Chukonu => TechLevel::new(2),        // Medieval China
            Self::ColtWalker => TechLevel::new(5),     // Industrial Revolution
            Self::ColtM1911 => TechLevel::new(6),      // Atomic Age
            Self::Crossbow => TechLevel::new(2),       // Medieval
            Self::Daikyu => TechLevel::new(3),        // Japanese
            Self::Dart => TechLevel::new(0),           // Stone Age
            Self::Dynamite => TechLevel::new(5),       // Industrial Revolution
            Self::DreyseNeedleRifle => TechLevel::new(5), // Industrial Revolution
            Self::Derringer => TechLevel::new(5),      // Mechanized Age
            Self::DoubleActionRevolver => TechLevel::new(5), // Industrial Revolution
            Self::DoubleBarrelShotgun => TechLevel::new(5), // Industrial Revolution
            Self::Espringal => TechLevel::new(2),      // Medieval
            Self::EnfieldPattern1853 => TechLevel::new(5), // Industrial Revolution
                        Self::Flamethrower => TechLevel::new(6),   // Atomic Age
            Self::FieldGun => TechLevel::new(5),       // Industrial Revolution
            Self::Flintlock => TechLevel::new(4),      // Renaissance
            Self::Fusil => TechLevel::new(4),          // Renaissance
            Self::GatlingGun => TechLevel::new(5),     // Industrial Revolution
            Self::GardnerGun => TechLevel::new(5),     // Industrial Revolution
                        Self::Grenade => TechLevel::new(6),        // Atomic Age
            Self::Hackbut => TechLevel::new(4),        // Renaissance
                        Self::Gastraphetes => TechLevel::new(1),   // Greek/Iron Age
            Self::GreatBow => TechLevel::new(2),       // Medieval
            Self::HornBow => TechLevel::new(1),        // Bronze Age
            Self::HandCannon => TechLevel::new(3),     // Medieval
                        Self::Hankyu => TechLevel::new(3),        // Japanese
            Self::HandMortar => TechLevel::new(4),     // Renaissance
            Self::HenryRifle => TechLevel::new(5),     // Industrial Revolution
            Self::HandCrossbow => TechLevel::new(2),   // Medieval
                        Self::HeavyCrossbow => TechLevel::new(2),  // Medieval
            Self::Hurlbat => TechLevel::new(1),        // Bronze Age
                        Self::HuntingRifle => TechLevel::new(5),   // Mechanized Age
            Self::Jezail => TechLevel::new(4),         // Renaissance
            Self::LematRevolver => TechLevel::new(5),  // Industrial Revolution
            Self::LeeEnfieldSMLE => TechLevel::new(6), // Atomic Age
            Self::LeverActionRifle => TechLevel::new(5), // Industrial Revolution
            Self::LightCrossbow => TechLevel::new(2),  // Medieval
                        Self::Longbow => TechLevel::new(0),        // Stone Age
            Self::LugerP08 => TechLevel::new(6),       // Atomic Age
            Self::M1Garand => TechLevel::new(6),       // Atomic Age
            Self::Mangonel => TechLevel::new(1),       // Roman/Iron Age
            Self::Matchlock => TechLevel::new(4),      // Renaissance
            Self::MartiniHenryRifle => TechLevel::new(5), // Industrial Revolution
            Self::MauserModel1871 => TechLevel::new(5), // Industrial Revolution
            Self::MauserC96 => TechLevel::new(6),      // Atomic Age
            Self::MauserGewehr98 => TechLevel::new(6), // Atomic Age
            Self::MaximGun => TechLevel::new(5),       // Industrial Revolution
            Self::MachineGun => TechLevel::new(6),     // Atomic Age
            Self::Mitrailleuse => TechLevel::new(5),   // Industrial Revolution
            Self::MosinNagant => TechLevel::new(6),    // Atomic Age
            Self::MountainGun => TechLevel::new(5),    // Industrial Revolution
            Self::Musket => TechLevel::new(4),         // Age of Sail
            Self::Musketoon => TechLevel::new(4),      // Renaissance
            Self::NavalGun => TechLevel::new(5),       // Industrial Revolution
            Self::NordenfeltGun => TechLevel::new(5),  // Industrial Revolution
            Self::PelletBow => TechLevel::new(7),      // Digital Age
                        Self::Pistol => TechLevel::new(6),         // Atomic Age
                        Self::PelletSling => TechLevel::new(0),    // Stone Age
            Self::Petronel => TechLevel::new(4),       // Renaissance
            Self::PepperboxRevolver => TechLevel::new(5), // Industrial Revolution
            Self::PercussionRevolver => TechLevel::new(5), // Industrial Revolution
            Self::Plumbata => TechLevel::new(1),       // Roman
            Self::Prodd => TechLevel::new(2),          // Medieval
            Self::PumpActionShotgun => TechLevel::new(5), // Industrial Revolution
            Self::RemingtonRollingBlockPistol => TechLevel::new(5), // Industrial Revolution
            Self::RemingtonRollingBlockRifle => TechLevel::new(5), // Industrial Revolution
            Self::Revolver => TechLevel::new(6),       // Atomic Age
            Self::RepeatingCrossbow => TechLevel::new(2), // Medieval China
            Self::Rifle => TechLevel::new(6),          // Atomic Age
            Self::Rock => TechLevel::new(0),           // Stone Age
                        Self::RocketLauncher => TechLevel::new(7), // Digital Age
            Self::Scorpion => TechLevel::new(1),       // Roman
            Self::RecurveBow => TechLevel::new(1),     // Bronze Age
            Self::SelfBow => TechLevel::new(0),        // Stone Age
            Self::Shotgun => TechLevel::new(5),        // Mechanized Age
                        Self::ShortBow => TechLevel::new(0),       // Stone Age
            Self::SharpsRifle => TechLevel::new(5),    // Industrial Revolution
            Self::SiegeCrossbow => TechLevel::new(2),  // Medieval
            Self::SingleActionRevolver => TechLevel::new(5), // Industrial Revolution
            Self::Shuriken => TechLevel::new(2),       // Medieval Japan
            Self::Sling => TechLevel::new(0),          // Stone Age
            Self::SmithWessonModel1 => TechLevel::new(5), // Industrial Revolution
            Self::SmithWessonModel3 => TechLevel::new(5), // Industrial Revolution
            Self::SMG => TechLevel::new(6),            // Atomic Age
            Self::SniperRifle => TechLevel::new(7),    // Digital Age
            Self::SpencerRepeatingRifle => TechLevel::new(5), // Industrial Revolution
            Self::SpringfieldModel1861 => TechLevel::new(5), // Industrial Revolution
            Self::SpringfieldM1903 => TechLevel::new(6), // Atomic Age
            Self::TrapdoorSpringfield => TechLevel::new(5), // Industrial Revolution
                        Self::StaffSling => TechLevel::new(1),     // Bronze Age
            Self::Stonebow => TechLevel::new(2),       // Medieval
            Self::Boomerang => TechLevel::new(0),      // Stone Age
                        Self::ThrowingAxe => TechLevel::new(0),    // Stone Age
            Self::ThrowingHammer => TechLevel::new(0), // Stone Age
                        Self::ThrowingKnife => TechLevel::new(0),  // Stone Age
            Self::ThrowingSpear => TechLevel::new(0),  // Stone Age
                        Self::ThrowingStick => TechLevel::new(0),  // Stone Age
            Self::Trebuchet => TechLevel::new(2),      // Medieval
            Self::TNT => TechLevel::new(5),            // Industrial Revolution
            Self::VolcanicPistol => TechLevel::new(5), // Industrial Revolution
                        Self::Warbow => TechLevel::new(2),         // Medieval
            Self::WaltherPPK => TechLevel::new(6),     // Atomic Age
            Self::WebleyRIC => TechLevel::new(5),      // Industrial Revolution
            Self::WebleyMkVI => TechLevel::new(6),     // Atomic Age
            Self::Wheellock => TechLevel::new(4),      // Renaissance
            Self::Winchester1866 => TechLevel::new(5), // Industrial Revolution
            Self::Winchester1873 => TechLevel::new(5), // Industrial Revolution
            Self::Winchester1887 => TechLevel::new(5), // Industrial Revolution
            Self::WinchesterRepeater => TechLevel::new(5), // Industrial Revolution
            Self::Yumi => TechLevel::new(2),           // Medieval Japan
            Self::ThrownBola => TechLevel::new(0),     // Stone Age
            Self::ThrownNet => TechLevel::new(1),      // Bronze Age
        }
    }

    /// Returns weapon damage for this ranged weapon.
    #[instrument]
    pub fn damage(&self) -> WeaponDamage {
        debug!("Getting ranged weapon damage");
        match self {
            Self::Arbalest => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 7),
                damage_type: DamageType::Impaling,
            },
            Self::Arquebus => WeaponDamage::Fixed {
                dice: DieLevel::new(3, 0),
                damage_type: DamageType::Piercing,
            },
            Self::AssaultRifle => WeaponDamage::Fixed {
                dice: DieLevel::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::ArisakaType38 => WeaponDamage::Fixed {
                dice: DieLevel::new(4, 0),
                damage_type: DamageType::Piercing,
            },
                        Self::Atlatl => WeaponDamage::Thrust {
                modifier: 3,
                damage_type: DamageType::Impaling,
            },
            Self::Ballista => WeaponDamage::Fixed {
                dice: DieLevel::new(3, 4),
                damage_type: DamageType::Impaling,
            },
                        Self::Blowgun => WeaponDamage::Fixed {
                dice: DieLevel::new(1, -3),
                damage_type: DamageType::Impaling,
            },
                        Self::Blunderbuss => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 2),
                damage_type: DamageType::Piercing,
            },
            Self::Bola => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
                        Self::Bow => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 0),
                damage_type: DamageType::Impaling,
            },
            Self::BrowningHiPower => WeaponDamage::Fixed {
                dice: DieLevel::new(2, 2),
                damage_type: DamageType::Piercing,
            },
                                    Self::Carbine => WeaponDamage::Fixed {
                dice: DieLevel::new(4, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Caliver => WeaponDamage::Fixed {
                dice: DieLevel::new(3, 0),
                damage_type: DamageType::Piercing,
            },
            Self::CapLockPistol => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 2),
                damage_type: DamageType::Piercing,
            },
            Self::Catapult => WeaponDamage::Fixed {
                dice: DieLevel::new(3, 0),
                damage_type: DamageType::Crushing,
            },
            Self::CompositeBow => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 3),
                damage_type: DamageType::Impaling,
            },
                        Self::Chakram => WeaponDamage::Thrust {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::ChassepotRifle => WeaponDamage::Fixed {
                dice: DieLevel::new(4, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Chukonu => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 0),
                damage_type: DamageType::Impaling,
            },
            Self::ColtWalker => WeaponDamage::Fixed {
                dice: DieLevel::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::ColtM1911 => WeaponDamage::Fixed {
                dice: DieLevel::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::Crossbow => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 4),
                damage_type: DamageType::Impaling,
            },
            Self::Daikyu => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 3),
                damage_type: DamageType::Impaling,
            },
            Self::Dart => WeaponDamage::Thrust {
                modifier: -1,
                damage_type: DamageType::Impaling,
            },
            Self::Dynamite => WeaponDamage::Fixed {
                dice: DieLevel::new(6, 0),
                damage_type: DamageType::Crushing,
            },
            Self::DreyseNeedleRifle => WeaponDamage::Fixed {
                dice: DieLevel::new(3, 2),
                damage_type: DamageType::Piercing,
            },
                        Self::Derringer => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 2),
                damage_type: DamageType::Piercing,
            },
            Self::DoubleActionRevolver => WeaponDamage::Fixed {
                dice: DieLevel::new(2, 0),
                damage_type: DamageType::Piercing,
            },
            Self::DoubleBarrelShotgun => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 1),
                damage_type: DamageType::Piercing,
            },
            Self::Espringal => WeaponDamage::Fixed {
                dice: DieLevel::new(2, 0),
                damage_type: DamageType::Impaling,
            },
            Self::EnfieldPattern1853 => WeaponDamage::Fixed {
                dice: DieLevel::new(4, 0),
                damage_type: DamageType::Piercing,
            },
                        Self::Flamethrower => WeaponDamage::Fixed {
                dice: DieLevel::new(3, 0),
                damage_type: DamageType::Crushing, // Burning damage
            },
            Self::FieldGun => WeaponDamage::Fixed {
                dice: DieLevel::new(6, 0),
                damage_type: DamageType::Crushing,
            },
            Self::Flintlock => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 2),
                damage_type: DamageType::Piercing,
            },
            Self::Fusil => WeaponDamage::Fixed {
                dice: DieLevel::new(4, 0),
                damage_type: DamageType::Piercing,
            },
            Self::GatlingGun => WeaponDamage::Fixed {
                dice: DieLevel::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::GardnerGun => WeaponDamage::Fixed {
                dice: DieLevel::new(5, 0),
                damage_type: DamageType::Piercing,
            },
                        Self::Grenade => WeaponDamage::Fixed {
                dice: DieLevel::new(3, 0), // 3d×2 fragmentation
                damage_type: DamageType::Crushing,
            },
            Self::Hackbut => WeaponDamage::Fixed {
                dice: DieLevel::new(3, 0),
                damage_type: DamageType::Piercing,
            },
                        Self::Gastraphetes => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 6),
                damage_type: DamageType::Impaling,
            },
            Self::GreatBow => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 4),
                damage_type: DamageType::Impaling,
            },
            Self::HornBow => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 2),
                damage_type: DamageType::Impaling,
            },
            Self::HandCannon => WeaponDamage::Fixed {
                dice: DieLevel::new(2, 2),
                damage_type: DamageType::Piercing,
            },
                        Self::Hankyu => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 0),
                damage_type: DamageType::Impaling,
            },
            Self::HandMortar => WeaponDamage::Fixed {
                dice: DieLevel::new(2, 0),
                damage_type: DamageType::Crushing,
            },
            Self::HenryRifle => WeaponDamage::Fixed {
                dice: DieLevel::new(3, 1),
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
            Self::Hurlbat => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
                        Self::HuntingRifle => WeaponDamage::Fixed {
                dice: DieLevel::new(7, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Jezail => WeaponDamage::Fixed {
                dice: DieLevel::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::LematRevolver => WeaponDamage::Fixed {
                dice: DieLevel::new(2, 1),
                damage_type: DamageType::Piercing,
            },
            Self::LeeEnfieldSMLE => WeaponDamage::Fixed {
                dice: DieLevel::new(4, 0),
                damage_type: DamageType::Piercing,
            },
            Self::LeverActionRifle => WeaponDamage::Fixed {
                dice: DieLevel::new(3, 0),
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
            Self::LugerP08 => WeaponDamage::Fixed {
                dice: DieLevel::new(2, 0),
                damage_type: DamageType::Piercing,
            },
            Self::M1Garand => WeaponDamage::Fixed {
                dice: DieLevel::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Mangonel => WeaponDamage::Fixed {
                dice: DieLevel::new(3, 0),
                damage_type: DamageType::Crushing,
            },
            Self::Matchlock => WeaponDamage::Fixed {
                dice: DieLevel::new(3, 2),
                damage_type: DamageType::Piercing,
            },
            Self::MartiniHenryRifle => WeaponDamage::Fixed {
                dice: DieLevel::new(4, 0),
                damage_type: DamageType::Piercing,
            },
            Self::MauserModel1871 => WeaponDamage::Fixed {
                dice: DieLevel::new(4, 1),
                damage_type: DamageType::Piercing,
            },
            Self::MauserC96 => WeaponDamage::Fixed {
                dice: DieLevel::new(2, 0),
                damage_type: DamageType::Piercing,
            },
            Self::MauserGewehr98 => WeaponDamage::Fixed {
                dice: DieLevel::new(4, 1),
                damage_type: DamageType::Piercing,
            },
            Self::MaximGun => WeaponDamage::Fixed {
                dice: DieLevel::new(6, 0),
                damage_type: DamageType::Piercing,
            },
            Self::MachineGun => WeaponDamage::Fixed {
                dice: DieLevel::new(7, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Mitrailleuse => WeaponDamage::Fixed {
                dice: DieLevel::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::MosinNagant => WeaponDamage::Fixed {
                dice: DieLevel::new(4, 1),
                damage_type: DamageType::Piercing,
            },
            Self::MountainGun => WeaponDamage::Fixed {
                dice: DieLevel::new(5, 0),
                damage_type: DamageType::Crushing,
            },
                        Self::Musket => WeaponDamage::Fixed {
                dice: DieLevel::new(4, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Musketoon => WeaponDamage::Fixed {
                dice: DieLevel::new(3, 2),
                damage_type: DamageType::Piercing,
            },
            Self::NavalGun => WeaponDamage::Fixed {
                dice: DieLevel::new(8, 0),
                damage_type: DamageType::Crushing,
            },
            Self::NordenfeltGun => WeaponDamage::Fixed {
                dice: DieLevel::new(5, 0),
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
                        Self::PelletSling => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::Petronel => WeaponDamage::Fixed {
                dice: DieLevel::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::PepperboxRevolver => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 1),
                damage_type: DamageType::Piercing,
            },
            Self::PercussionRevolver => WeaponDamage::Fixed {
                dice: DieLevel::new(2, 0),
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
            Self::PumpActionShotgun => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 1),
                damage_type: DamageType::Piercing,
            },
            Self::RemingtonRollingBlockPistol => WeaponDamage::Fixed {
                dice: DieLevel::new(2, 0),
                damage_type: DamageType::Piercing,
            },
            Self::RemingtonRollingBlockRifle => WeaponDamage::Fixed {
                dice: DieLevel::new(4, 0),
                damage_type: DamageType::Piercing,
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
            Self::Scorpion => WeaponDamage::Fixed {
                dice: DieLevel::new(2, 2),
                damage_type: DamageType::Impaling,
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
            Self::SharpsRifle => WeaponDamage::Fixed {
                dice: DieLevel::new(4, 0),
                damage_type: DamageType::Piercing,
            },
            Self::SiegeCrossbow => WeaponDamage::Fixed {
                dice: DieLevel::new(3, 6),
                damage_type: DamageType::Impaling,
            },
            Self::SingleActionRevolver => WeaponDamage::Fixed {
                dice: DieLevel::new(2, 1),
                damage_type: DamageType::Piercing,
            },
            Self::Shuriken => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Impaling,
            },
            Self::Sling => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Piercing,
            },
            Self::SmithWessonModel1 => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 0),
                damage_type: DamageType::Piercing,
            },
            Self::SmithWessonModel3 => WeaponDamage::Fixed {
                dice: DieLevel::new(2, 0),
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
            Self::SpencerRepeatingRifle => WeaponDamage::Fixed {
                dice: DieLevel::new(3, 2),
                damage_type: DamageType::Piercing,
            },
            Self::SpringfieldModel1861 => WeaponDamage::Fixed {
                dice: DieLevel::new(4, 0),
                damage_type: DamageType::Piercing,
            },
            Self::SpringfieldM1903 => WeaponDamage::Fixed {
                dice: DieLevel::new(4, 1),
                damage_type: DamageType::Piercing,
            },
            Self::TrapdoorSpringfield => WeaponDamage::Fixed {
                dice: DieLevel::new(4, 0),
                damage_type: DamageType::Piercing,
            },
                        Self::StaffSling => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Piercing,
            },
            Self::Stonebow => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 2),
                damage_type: DamageType::Crushing,
            },
            Self::Boomerang => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
                        Self::ThrowingAxe => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
            Self::ThrowingHammer => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Crushing,
            },
                        Self::ThrowingKnife => WeaponDamage::Thrust {
                modifier: -1,
                damage_type: DamageType::Impaling,
            },
            Self::ThrowingSpear => WeaponDamage::Thrust {
                modifier: 3,
                damage_type: DamageType::Impaling,
            },
                        Self::ThrowingStick => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::Trebuchet => WeaponDamage::Fixed {
                dice: DieLevel::new(4, 0),
                damage_type: DamageType::Crushing,
            },
            Self::TNT => WeaponDamage::Fixed {
                dice: DieLevel::new(8, 0),
                damage_type: DamageType::Crushing,
            },
            Self::VolcanicPistol => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 1),
                damage_type: DamageType::Piercing,
            },
                        Self::Warbow => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 3),
                damage_type: DamageType::Impaling,
            },
            Self::WaltherPPK => WeaponDamage::Fixed {
                dice: DieLevel::new(2, -1),
                damage_type: DamageType::Piercing,
            },
            Self::WebleyRIC => WeaponDamage::Fixed {
                dice: DieLevel::new(2, 0),
                damage_type: DamageType::Piercing,
            },
            Self::WebleyMkVI => WeaponDamage::Fixed {
                dice: DieLevel::new(2, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Wheellock => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 2),
                damage_type: DamageType::Piercing,
            },
            Self::Winchester1866 => WeaponDamage::Fixed {
                dice: DieLevel::new(3, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Winchester1873 => WeaponDamage::Fixed {
                dice: DieLevel::new(3, 1),
                damage_type: DamageType::Piercing,
            },
            Self::Winchester1887 => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 1),
                damage_type: DamageType::Piercing,
            },
            Self::WinchesterRepeater => WeaponDamage::Fixed {
                dice: DieLevel::new(3, 1),
                damage_type: DamageType::Piercing,
            },
            Self::Yumi => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 1),
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
            Self::Arbalest => 5,
            Self::Arquebus => 2,
            Self::AssaultRifle => 5,
            Self::ArisakaType38 => 4,
                        Self::Atlatl => 2,
            Self::Ballista => 5,
                        Self::Blowgun => 1,
                        Self::Blunderbuss => 1,
            Self::Bola => 1,
                        Self::Bow => 2,
            Self::BrowningHiPower => 2,
                                    Self::Carbine => 4,
            Self::Caliver => 3,
            Self::CapLockPistol => 1,
            Self::Catapult => 1,
            Self::CompositeBow => 3,
                        Self::Chakram => 1,
            Self::ChassepotRifle => 4,
            Self::Chukonu => 2,
            Self::ColtWalker => 1,
            Self::ColtM1911 => 2,
            Self::Crossbow => 4,
            Self::Daikyu => 3,
            Self::Dart => 2,
            Self::Dynamite => 1,
            Self::DreyseNeedleRifle => 3,
                        Self::Derringer => 1,
            Self::DoubleActionRevolver => 2,
            Self::DoubleBarrelShotgun => 3,
            Self::Espringal => 3,
            Self::EnfieldPattern1853 => 4,
                        Self::Flamethrower => 2,
            Self::FieldGun => 2,
            Self::Flintlock => 1,
            Self::Fusil => 3,
            Self::GatlingGun => 4,
            Self::GardnerGun => 4,
                        Self::Grenade => 1,
            Self::Hackbut => 2,
                        Self::Gastraphetes => 4,
            Self::GreatBow => 3,
            Self::HornBow => 3,
            Self::HandCannon => 1,
                        Self::Hankyu => 2,
            Self::HandMortar => 1,
            Self::HenryRifle => 4,
            Self::HandCrossbow => 3,
                        Self::HeavyCrossbow => 4,
            Self::Hurlbat => 1,
                        Self::HuntingRifle => 5,
            Self::Jezail => 4,
            Self::LematRevolver => 2,
            Self::LeeEnfieldSMLE => 4,
            Self::LeverActionRifle => 4,
            Self::LightCrossbow => 4,
                                    Self::Longbow => 3,
            Self::LugerP08 => 2,
            Self::M1Garand => 4,
            Self::Mangonel => 1,
            Self::Matchlock => 2,
            Self::MartiniHenryRifle => 4,
            Self::MauserModel1871 => 5,
            Self::MauserC96 => 2,
            Self::MauserGewehr98 => 5,
            Self::MaximGun => 5,
            Self::MachineGun => 4,
            Self::Mitrailleuse => 3,
            Self::MosinNagant => 4,
            Self::MountainGun => 2,
                        Self::Musket => 3,
            Self::Musketoon => 2,
            Self::NavalGun => 1,
            Self::NordenfeltGun => 3,
            Self::PelletBow => 4,
                        Self::Pistol => 2,
                        Self::PelletSling => 0,
            Self::Petronel => 1,
            Self::PepperboxRevolver => 1,
            Self::PercussionRevolver => 2,
            Self::Plumbata => 0,
            Self::Prodd => 3,
            Self::PumpActionShotgun => 3,
            Self::RemingtonRollingBlockPistol => 2,
            Self::RemingtonRollingBlockRifle => 4,
            Self::Revolver => 2,
            Self::RepeatingCrossbow => 2,
            Self::Rifle => 5,
            Self::Rock => 0,
                        Self::RocketLauncher => 4,
            Self::Scorpion => 4,
            Self::RecurveBow => 2,
            Self::SelfBow => 1,
            Self::Shotgun => 3,
                        Self::ShortBow => 1,
            Self::SharpsRifle => 4,
            Self::SiegeCrossbow => 5,
            Self::SingleActionRevolver => 2,
            Self::Shuriken => 1,
            Self::Sling => 0,
            Self::SmithWessonModel1 => 1,
            Self::SmithWessonModel3 => 2,
            Self::SMG => 4,
            Self::SniperRifle => 6,
            Self::SpencerRepeatingRifle => 3,
            Self::SpringfieldModel1861 => 4,
            Self::SpringfieldM1903 => 5,
            Self::TrapdoorSpringfield => 4,
                        Self::StaffSling => 1,
            Self::Stonebow => 3,
            Self::Boomerang => 1,
                        Self::ThrowingAxe => 2,
            Self::ThrowingHammer => 2,
                        Self::ThrowingKnife => 0,
            Self::ThrowingSpear => 2,
                        Self::ThrowingStick => 1,
            Self::Trebuchet => 1,
            Self::TNT => 1,
            Self::VolcanicPistol => 1,
                        Self::Warbow => 3,
            Self::WaltherPPK => 2,
            Self::WebleyRIC => 2,
            Self::WebleyMkVI => 2,
            Self::Wheellock => 1,
            Self::Winchester1866 => 4,
            Self::Winchester1873 => 4,
            Self::Winchester1887 => 3,
            Self::WinchesterRepeater => 4,
            Self::Yumi => 2,
            Self::ThrownBola => 1,
            Self::ThrownNet => 1,
        }
    }

    /// Returns required skill for this ranged weapon.
    #[instrument]
    pub fn required_skill(&self) -> Skill {
        debug!("Getting ranged weapon required skill");
        match self {
            Self::Arbalest => Skill::Crossbow,
            Self::Arquebus => Skill::Guns,
            Self::AssaultRifle => Skill::Guns,
            Self::ArisakaType38 => Skill::Guns,
                        Self::Atlatl => Skill::ThrownWeapon,
            Self::Ballista => Skill::Crossbow,
                        Self::Blowgun => Skill::Blowpipe,
                        Self::Blunderbuss => Skill::Guns,
            Self::Bola => Skill::ThrownWeapon,
                        Self::Bow => Skill::Bow,
            Self::BrowningHiPower => Skill::Guns,
                                    Self::Carbine => Skill::Guns,
            Self::Caliver => Skill::Guns,
            Self::CapLockPistol => Skill::Guns,
            Self::Catapult => Skill::Artillery,
            Self::CompositeBow => Skill::Bow,
                        Self::Chakram => Skill::ThrownWeapon,
            Self::ChassepotRifle => Skill::Guns,
            Self::Chukonu => Skill::Crossbow,
            Self::ColtWalker => Skill::Guns,
            Self::ColtM1911 => Skill::Guns,
            Self::Crossbow => Skill::Crossbow,
            Self::Daikyu => Skill::Bow,
            Self::Dart => Skill::ThrownWeapon,
            Self::Dynamite => Skill::ThrownWeapon,
            Self::DreyseNeedleRifle => Skill::Guns,
                        Self::Derringer => Skill::Guns,
            Self::DoubleActionRevolver => Skill::Guns,
            Self::DoubleBarrelShotgun => Skill::Guns,
            Self::Espringal => Skill::Artillery,
            Self::EnfieldPattern1853 => Skill::Guns,
                        Self::Flamethrower => Skill::Guns,
            Self::FieldGun => Skill::Artillery,
            Self::Flintlock => Skill::Guns,
            Self::Fusil => Skill::Guns,
            Self::GatlingGun => Skill::Guns,
            Self::GardnerGun => Skill::Guns,
                        Self::Grenade => Skill::ThrownWeapon,
            Self::Hackbut => Skill::Guns,
                        Self::Gastraphetes => Skill::Crossbow,
            Self::GreatBow => Skill::Bow,
            Self::HornBow => Skill::Bow,
            Self::HandCannon => Skill::Guns,
                        Self::Hankyu => Skill::Bow,
            Self::HandMortar => Skill::Artillery,
            Self::HenryRifle => Skill::Guns,
            Self::HandCrossbow => Skill::Crossbow,
                        Self::HeavyCrossbow => Skill::Crossbow,
            Self::Hurlbat => Skill::ThrownWeapon,
                        Self::HuntingRifle => Skill::Guns,
            Self::Jezail => Skill::Guns,
            Self::LematRevolver => Skill::Guns,
            Self::LeeEnfieldSMLE => Skill::Guns,
            Self::LeverActionRifle => Skill::Guns,
            Self::LightCrossbow => Skill::Crossbow,
                                    Self::Longbow => Skill::Bow,
            Self::LugerP08 => Skill::Guns,
            Self::M1Garand => Skill::Guns,
            Self::Mangonel => Skill::Artillery,
            Self::Matchlock => Skill::Guns,
            Self::MartiniHenryRifle => Skill::Guns,
            Self::MauserModel1871 => Skill::Guns,
            Self::MauserC96 => Skill::Guns,
            Self::MauserGewehr98 => Skill::Guns,
            Self::MaximGun => Skill::Guns,
            Self::MachineGun => Skill::Guns,
            Self::Mitrailleuse => Skill::Guns,
            Self::MosinNagant => Skill::Guns,
            Self::MountainGun => Skill::Artillery,
                        Self::Musket => Skill::Guns,
            Self::Musketoon => Skill::Guns,
            Self::NavalGun => Skill::Artillery,
            Self::NordenfeltGun => Skill::Guns,
            Self::PelletBow => Skill::Bow,
                        Self::Pistol => Skill::Guns,
                        Self::PelletSling => Skill::Sling,
            Self::Petronel => Skill::Guns,
            Self::PepperboxRevolver => Skill::Guns,
            Self::PercussionRevolver => Skill::Guns,
            Self::Plumbata => Skill::ThrownWeapon,
            Self::Prodd => Skill::Crossbow,
            Self::PumpActionShotgun => Skill::Guns,
            Self::RemingtonRollingBlockPistol => Skill::Guns,
            Self::RemingtonRollingBlockRifle => Skill::Guns,
            Self::Revolver => Skill::Guns,
            Self::RepeatingCrossbow => Skill::Crossbow,
            Self::Rifle => Skill::Guns,
            Self::Rock => Skill::ThrownWeapon,
                        Self::RocketLauncher => Skill::Guns,
            Self::Scorpion => Skill::Artillery,
            Self::RecurveBow => Skill::Bow,
            Self::SelfBow => Skill::Bow,
            Self::Shotgun => Skill::Guns,
                        Self::ShortBow => Skill::Bow,
            Self::SharpsRifle => Skill::Guns,
            Self::SiegeCrossbow => Skill::Crossbow,
            Self::SingleActionRevolver => Skill::Guns,
            Self::Shuriken => Skill::ThrownWeapon,
            Self::Sling => Skill::Sling,
            Self::SmithWessonModel1 => Skill::Guns,
            Self::SmithWessonModel3 => Skill::Guns,
            Self::SMG => Skill::Guns,
            Self::SniperRifle => Skill::Guns,
            Self::SpencerRepeatingRifle => Skill::Guns,
            Self::SpringfieldModel1861 => Skill::Guns,
            Self::SpringfieldM1903 => Skill::Guns,
            Self::TrapdoorSpringfield => Skill::Guns,
                        Self::StaffSling => Skill::Sling,
            Self::Stonebow => Skill::Crossbow,
            Self::Boomerang => Skill::ThrownWeapon,
                        Self::ThrowingAxe => Skill::ThrownWeapon,
            Self::ThrowingHammer => Skill::ThrownWeapon,
                        Self::ThrowingKnife => Skill::ThrownWeapon,
            Self::ThrowingSpear => Skill::ThrownWeapon,
                        Self::ThrowingStick => Skill::ThrownWeapon,
            Self::Trebuchet => Skill::Artillery,
            Self::TNT => Skill::ThrownWeapon,
            Self::VolcanicPistol => Skill::Guns,
                        Self::Warbow => Skill::Bow,
            Self::WaltherPPK => Skill::Guns,
            Self::WebleyRIC => Skill::Guns,
            Self::WebleyMkVI => Skill::Guns,
            Self::Wheellock => Skill::Guns,
            Self::Winchester1866 => Skill::Guns,
            Self::Winchester1873 => Skill::Guns,
            Self::Winchester1887 => Skill::Guns,
            Self::WinchesterRepeater => Skill::Guns,
            Self::Yumi => Skill::Bow,
            Self::ThrownBola => Skill::ThrownWeapon,
            Self::ThrownNet => Skill::ThrownWeapon,
        }
    }
}
