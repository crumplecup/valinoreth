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

use crate::{Currency, DamageDice, DamageType, Skill, TechLevel, WeaponDamage, Weight};
use tracing::{debug, instrument};

/// Ranged weapon types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, strum::EnumIter)]
pub enum RangedWeapon {
    /// Arbalest, 1d+7 impaling. LT (Heavy siege crossbow)
    Arbalest,
    /// Accuracy International AWM, 7d piercing. HT (British precision rifle, TL8)
    AccuracyInternationalAWM,
    /// AA-12, 1d+1 piercing. HT (American automatic shotgun, TL8)
    AA12,
    /// Arquebus, 3d piercing. LT (Spanish matchlock, TL4)
    Arquebus,
    /// AK-47, 5d piercing. HT (Soviet 7.62mm assault rifle, TL7)
    AK47,
    /// AKM, 5d piercing. HT (Modernized AK-47, TL7)
    AKM,
    /// AK-12, 5d piercing. HT (Modern Russian 5.45mm rifle, TL8)
    AK12,
    /// AS Val, 4d piercing. HT (Russian suppressed 9x39mm assault rifle, TL8)
    ASVal,
    /// Assault rifle, 5d piercing. BS 278
    AssaultRifle,
    /// Arisaka Type 38, 4d piercing. HT (Japanese bolt-action rifle, TL6)
    ArisakaType38,
    /// AT4, 8d crushing. HT (Swedish 84mm anti-tank rocket, TL7)
    AT4,
    /// Atlatl, thrust+3 impaling. BS 277
    Atlatl,
    /// Ballista, 3d+4 impaling. LT (Siege crossbow, TL1)
    Ballista,
    /// Barrett M107, 9d piercing. HT (American .50 BMG semi-auto anti-materiel rifle, TL8)
    BarrettM107,
    /// Blowgun, 1d-3 impaling. BS 277
    Blowgun,
    /// Blunderbuss, 1d+2 piercing. LT (Early shotgun, TL4)
    Blunderbuss,
    /// Bola, swing crushing. LT (Whirled bola, TL0)
    Bola,
    /// Bow, 1d impaling. BS 276
    Bow,
    /// Boys Anti-Tank Rifle, 7d piercing. HT (British .55 anti-tank rifle, TL6)
    BoysAntiTankRifle,
    /// BAR (Browning Automatic Rifle), 5d piercing. HT (American .30-06 LMG, TL6)
    BAR,
    /// Bren Gun, 5d piercing. HT (British .303 LMG, TL6)
    BrenGun,
    /// Barrett M82, 9d piercing. HT (American .50 BMG anti-materiel rifle, TL7)
    BarrettM82,
    /// Benelli M3, 1d+1 piercing. HT (Italian 12-gauge convertible shotgun, TL7)
    BenelliM3,
    /// Beretta M92, 2d+2 piercing. HT (9mm semi-auto pistol, TL7)
    BerettaM92,
    /// Browning Hi-Power, 2d+2 piercing. HT (9mm semi-auto pistol, TL6)
    BrowningHiPower,
    /// Browning M1917, 6d piercing. HT (American .30-06 HMG, TL6)
    BrowningM1917,
    /// Browning M2, 7d piercing. HT (American .50 cal HMG, TL6)
    BrowningM2,
    /// Browning Auto-5, 1d+1 piercing. HT (Semi-auto shotgun, TL6)
    BrowningAuto5,
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
    /// Claymore Mine, 8d crushing. HT (Directional anti-personnel mine, TL7)
    ClaymoreMine,
    /// Chassepot Rifle, 4d piercing. HT (French bolt-action rifle, TL5)
    ChassepotRifle,
    /// CheyTac M200 Intervention, 8d piercing. HT (American long-range precision rifle, TL8)
    CheyTacM200,
    /// Chiappa Rhino, 3d-1 piercing. HT (Italian low bore-axis .357 Magnum revolver, TL8)
    ChiappaRhino,
    /// Chu-ko-nu, 1d impaling. LT (Chinese repeating crossbow variant)
    Chukonu,
    /// Colt Walker, 2d+2 piercing. HT (Powerful cap-and-ball revolver, TL5)
    ColtWalker,
    /// Colt M1911, 2d+2 piercing. HT (.45 ACP semi-auto pistol, TL6)
    ColtM1911,
    /// Coilgun, 6d piercing. HT (Electromagnetic projectile accelerator, experimental TL8)
    Coilgun,
    /// Crossbow, 1d+4 impaling. BS 276
    Crossbow,
    /// CZ 75, 2d+2 piercing. HT (9mm semi-auto pistol, TL7)
    CZ75,
    /// CZ P-10, 2d+2 piercing. HT (Czech striker-fired 9mm pistol, TL8)
    CZP10,
    /// C4 Explosive, 8d crushing. HT (Plastic explosive, TL7)
    C4Explosive,
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
    /// Desert Eagle, 3d piercing. HT (.50 AE semi-auto pistol, TL7)
    DesertEagle,
    /// Double-action revolver, 2d piercing. HT (DA revolver .38, TL5)
    DoubleActionRevolver,
    /// Double-barrel shotgun, 1d+1 piercing. HT (Break-action shotgun, TL5)
    DoubleBarrelShotgun,
    /// Dragunov SVD, 6d piercing. HT (Soviet 7.62mm sniper rifle, TL7)
    DragunovSVD,
    /// Espringal, 2d impaling. LT (Medieval dart-thrower, TL2)
    Espringal,
    /// Enfield Pattern 1853, 4d piercing. HT (Rifled musket, TL5)
    EnfieldPattern1853,
    /// Flamethrower, 3d crushing (burning). BS 278
    Flamethrower,
    /// Flare Gun, special. HT (Signaling pistol, TL6)
    FlareGun,
    /// Field Gun, 6d crushing. HT (Artillery piece, TL5)
    FieldGun,
    /// Flintlock, 1d+2 piercing. LT (Flintlock pistol, TL4)
    Flintlock,
    /// FAMAS, 5d piercing. HT (French 5.56mm bullpup rifle, TL8)
    FAMAS,
    /// FN FAL, 5d+1 piercing. HT (7.62mm battle rifle, TL7)
    FNFAL,
    /// FN MAG, 6d piercing. HT (Belgian 7.62mm GPMG, TL7)
    FNMAG,
    /// FN P90, 3d+1 piercing. HT (Belgian 5.7mm PDW, TL8)
    FNP90,
    /// FN SCAR-L, 5d piercing. HT (Belgian 5.56mm modular rifle, TL8)
    FNSCARL,
    /// FN Five-seveN, 2d piercing. HT (Belgian 5.7mm pistol, TL8)
    FNFiveSeveN,
    /// FN Minimi Para, 5d piercing. HT (Belgian 5.56mm compact LMG, TL8)
    FNMinimiPara,
    /// Fostech Origin-12, 1d+1 piercing. HT (American semi-auto shotgun, TL8)
    FostechOrigin12,
    /// Fusil, 4d piercing. LT (Light musket, TL4)
    Fusil,
    /// Gatling gun, 5d piercing. HT (Hand-cranked machine gun, TL5)
    GatlingGun,
    /// Gardner Gun, 5d piercing. HT (Hand-cranked machine gun, TL5)
    GardnerGun,
    /// Grenade, 3d crushing. BS 278
    Grenade,
    /// Grease Gun M3, 2d+2 piercing. HT (American .45 ACP SMG, TL6)
    GreaseGunM3,
    /// Hackbut, 3d piercing. LT (Heavy hand cannon, TL4)
    Hackbut,
    /// Gastraphetes, 1d+6 impaling. LT (Greek belly bow)
    Gastraphetes,
    /// Great bow, 1d+4 impaling. LT (Extra-heavy war bow, TL2)
    GreatBow,
    /// Glock 17, 2d+2 piercing. HT (9mm semi-auto pistol, TL7)
    Glock17,
    /// Glock 43X, 2d piercing. HT (Subcompact 9mm pistol, TL8)
    Glock43X,
    /// G3, 5d+1 piercing. HT (German 7.62mm battle rifle, TL7)
    G3,
    /// GP-25, 6d crushing. HT (Soviet 40mm underbarrel grenade launcher, TL7)
    GP25,
    /// Gauss Rifle, 7d piercing. HT (Magnetic accelerator rifle, experimental TL8)
    GaussRifle,
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
    /// HK416, 5d piercing. HT (German 5.56mm improved AR, TL8)
    HK416,
    /// HK MP7, 3d piercing. HT (German 4.6mm PDW, TL8)
    HKMP7,
    /// HK Mark 23, 2d+2 piercing. HT (German .45 ACP SOCOM pistol, TL8)
    HKMark23,
    /// HK MG4, 5d piercing. HT (German 5.56mm LMG, TL8)
    HKMG4,
    /// Jezail, 5d piercing. LT (Afghan long gun, TL4)
    Jezail,
    /// Kel-Tec KSG, 1d+1 piercing. HT (American bullpup pump shotgun, TL8)
    KelTecKSG,
    /// Kriss Vector, 2d+2 piercing. HT (American .45 ACP low-recoil SMG, TL8)
    KrissVector,
    /// LeMat revolver, 2d+1 piercing. HT (9-shot + shotgun revolver, TL5)
    LematRevolver,
    /// Lee-Enfield SMLE, 4d piercing. HT (British .303 bolt-action, TL6)
    LeeEnfieldSMLE,
    /// Lever-action rifle, 3d piercing. HT (Repeating rifle, TL5)
    LeverActionRifle,
    /// Lewis Gun, 5d piercing. HT (British/American .303 LMG, TL6)
    LewisGun,
    /// Light crossbow, 1d+2 impaling. BS 276
    LightCrossbow,
    /// Longbow, 1d+2 impaling. BS 276
    Longbow,
    /// Luger P08, 2d piercing. HT (German 9mm semi-auto pistol, TL6)
    LugerP08,
    /// L96A1, 7d piercing. HT (British .338 Lapua sniper rifle, TL7)
    L96A1,
    /// LAW M72, 8d crushing. HT (American 66mm anti-tank rocket, TL7)
    LAWM72,
    /// Laser Rifle, 5d burning. HT (Directed energy weapon, experimental TL8)
    LaserRifle,
    /// M1 Garand, 5d piercing. HT (American semi-auto rifle, TL6)
    M1Garand,
    /// Makarov PM, 2d piercing. HT (9x18mm semi-auto pistol, TL7)
    MakarovPM,
    /// M14, 5d+1 piercing. HT (American 7.62mm battle rifle, TL7)
    M14,
    /// M16, 5d piercing. HT (American 5.56mm assault rifle, TL7)
    M16,
    /// M203, 6d crushing. HT (American 40mm underbarrel grenade launcher, TL7)
    M203,
    /// M21 SWS, 6d piercing. HT (American 7.62mm sniper rifle, TL7)
    M21SWS,
    /// M249 SAW, 5d piercing. HT (American 5.56mm squad automatic weapon, TL7)
    M249SAW,
    /// M60, 6d piercing. HT (American 7.62mm GPMG, TL7)
    M60,
    /// M4 Carbine, 5d piercing. HT (American 5.56mm carbine, TL8)
    M4Carbine,
    /// M79, 6d crushing. HT (American 40mm grenade launcher, TL7)
    M79,
    /// MAC-10, 2d+2 piercing. HT (American .45 ACP SMG, TL7)
    MAC10,
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
    /// MG34, 6d piercing. HT (German 7.92mm GPMG, TL6)
    MG34,
    /// MG42, 6d piercing. HT (German 7.92mm GPMG, TL6)
    MG42,
    /// MG3, 6d piercing. HT (German 7.62mm GPMG, TL7)
    MG3,
    /// Maxim gun, 6d piercing. HT (Early recoil-operated MG, TL5)
    MaximGun,
    /// Machine gun, 7d piercing. BS 278
    MachineGun,
    /// McMillan TAC-50, 9d piercing. HT (American .50 BMG anti-materiel rifle, TL8)
    McMillanTAC50,
    /// Milkor MGL, 6d crushing. HT (South African 40mm revolver grenade launcher, TL8)
    MilkorMGL,
    /// Mitrailleuse, 5d piercing. HT (Volley gun, TL5)
    Mitrailleuse,
    /// Mk 19, 6d crushing. HT (American 40mm automatic grenade launcher, TL7)
    Mk19,
    /// Mosin-Nagant, 4d+1 piercing. HT (Russian bolt-action rifle, TL6)
    MosinNagant,
    /// Mossberg 500, 1d+1 piercing. HT (American pump-action shotgun, TL7)
    Mossberg500,
    /// MP18, 2d+1 piercing. HT (German 9mm SMG, TL6)
    MP18,
    /// MP40, 2d+2 piercing. HT (German 9mm SMG, TL6)
    MP40,
    /// MP5, 2d+2 piercing. HT (German 9mm SMG, TL7)
    MP5,
    /// Mountain Gun, 5d crushing. HT (Light artillery, TL5)
    MountainGun,
    /// MTS-255, 1d+1 piercing. HT (Russian revolving shotgun, TL8)
    MTS255,
    /// Musket, 4d piercing. BS 278
    Musket,
    /// Musketoon, 3d+2 piercing. LT (Short musket, TL4)
    Musketoon,
    /// Naval Gun, 8d crushing. HT (Ship-mounted artillery, TL5)
    NavalGun,
    /// Negev NG7, 6d piercing. HT (Israeli 7.62mm GPMG, TL8)
    NegevNG7,
    /// Nordenfelt Gun, 5d piercing. HT (Multi-barrel volley gun, TL5)
    NordenfeltGun,
    /// Pecheneg, 6d piercing. HT (Russian 7.62mm GPMG, TL8)
    Pecheneg,
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
    /// PPS-43, 2d+2 piercing. HT (Soviet 7.62mm SMG, TL6)
    PPS43,
    /// PPSh-41, 2d+2 piercing. HT (Soviet 7.62mm SMG, TL6)
    PPSh41,
    /// PKM, 6d piercing. HT (Soviet 7.62mm GPMG, TL7)
    PKM,
    /// PSG1, 6d piercing. HT (German 7.62mm precision rifle, TL7)
    PSG1,
    /// PGM Hecate II, 9d piercing. HT (French .50 BMG anti-materiel rifle, TL8)
    PGMHecateII,
    /// Plasma Rifle, 8d burning. HT (Superheated plasma projector, experimental TL8)
    PlasmaRifle,
    /// PTRD Anti-Tank Rifle, 7d+1 piercing. HT (Soviet 14.5mm anti-tank rifle, TL6)
    PTRDAntiTankRifle,
    /// Prodd (stone-throwing crossbow), 1d crushing. BS 276
    Prodd,
    /// Pump-action shotgun, 1d+1 piercing. HT (Early pump shotgun, TL5)
    PumpActionShotgun,
    /// QBZ-95, 5d piercing. HT (Chinese 5.8mm bullpup rifle, TL8)
    QBZ95,
    /// Remington 870, 1d+1 piercing. HT (American pump-action shotgun, TL7)
    Remington870,
    /// Remington Rolling Block Pistol, 2d piercing. HT (Single-shot pistol, TL5)
    RemingtonRollingBlockPistol,
    /// Remington Rolling Block Rifle, 4d piercing. HT (Single-shot rifle, TL5)
    RemingtonRollingBlockRifle,
    /// Remington Model 31, 1d+1 piercing. HT (Pump-action shotgun, TL6)
    RemingtonModel31,
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
    /// RPG-7, 8d crushing. HT (Soviet 40mm anti-tank rocket launcher, TL7)
    RPG7,
    /// Railgun, 10d piercing. HT (Electromagnetic rail accelerator, experimental TL8)
    Railgun,
    /// Scorpion, 2d+2 impaling. LT (Roman light ballista, TL1)
    Scorpion,
    /// Recurve bow, 1d+2 impaling. LT (Composite recurve)
    RecurveBow,
    /// Saiga-12, 1d+1 piercing. HT (Russian semi-auto shotgun, TL7)
    Saiga12,
    /// Sako TRG-42, 7d piercing. HT (Finnish .338 Lapua precision rifle, TL8)
    SakoTRG42,
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
    /// Skorpion vz. 61, 2d piercing. HT (Czech .32 ACP SMG, TL7)
    Skorpion,
    /// SIG P220, 2d+2 piercing. HT (9mm semi-auto pistol, TL7)
    SIGP220,
    /// SIG P320, 2d+2 piercing. HT (Modular striker-fired 9mm pistol, TL8)
    SIGP320,
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
    /// SPAS-12, 1d+1 piercing. HT (Italian combat shotgun, TL7)
    SPAS12,
    /// Spencer Repeating Rifle, 3d+2 piercing. HT (Lever-action repeater, TL5)
    SpencerRepeatingRifle,
    /// Springfield Model 1861, 4d piercing. HT (Rifled musket, TL5)
    SpringfieldModel1861,
    /// Springfield M1903, 4d+1 piercing. HT (American bolt-action, TL6)
    SpringfieldM1903,
    /// Trapdoor Springfield, 4d piercing. HT (Breech-loading rifle, TL5)
    TrapdoorSpringfield,
    /// Sten Gun, 2d+1 piercing. HT (British 9mm SMG, TL6)
    StenGun,
    /// Sterling, 2d+2 piercing. HT (British 9mm SMG, TL7)
    Sterling,
    /// Steyr AUG, 5d piercing. HT (Austrian 5.56mm bullpup rifle, TL8)
    SteyrAUG,
    /// Stinger Missile, 12d crushing. HT (American surface-to-air missile, TL7)
    StingerMissile,
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
    /// Thompson M1928, 2d+2 piercing. HT (American .45 ACP SMG, TL6)
    ThompsonM1928,
    /// Trebuchet, 4d crushing. LT (Counterweight catapult, TL2)
    Trebuchet,
    /// TNT, 8d crushing. HT (Trinitrotoluene explosive, TL5)
    TNT,
    /// TOW Missile, 12d crushing. HT (American anti-tank guided missile, TL7)
    TOWMissile,
    /// Uzi, 2d+2 piercing. HT (Israeli 9mm SMG, TL7)
    Uzi,
    /// Ultimax 100, 5d piercing. HT (Singaporean 5.56mm LMG, TL8)
    Ultimax100,
    /// USAS-12, 1d+1 piercing. HT (American automatic shotgun, TL8)
    USAS12,
    /// Volcanic pistol, 1d+1 piercing. HT (Lever-action pistol, TL5)
    VolcanicPistol,
    /// Vickers Machine Gun, 6d piercing. HT (British .303 HMG, TL6)
    VickersMachineGun,
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
    /// Winchester Model 1897, 1d+1 piercing. HT (Pump-action "Trench Gun", TL6)
    Winchester1897,
    /// Winchester repeater, 3d+1 piercing. HT (Lever-action rifle, TL5)
    WinchesterRepeater,
    /// XM8, 5d piercing. HT (American 5.56mm experimental rifle, TL8)
    XM8,
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
            Self::AccuracyInternationalAWM => Currency::dollars(10000.0),
            Self::AA12 => Currency::dollars(3000.0),
            Self::Arquebus => Currency::dollars(500.0),
            Self::AK47 => Currency::dollars(600.0),
            Self::AKM => Currency::dollars(650.0),
            Self::AK12 => Currency::dollars(2500.0),
            Self::ASVal => Currency::dollars(2500.0),
            Self::AssaultRifle => Currency::dollars(900.0),
            Self::ArisakaType38 => Currency::dollars(400.0),
            Self::AT4 => Currency::dollars(1500.0),
                        Self::Atlatl => Currency::dollars(20.0),
            Self::Ballista => Currency::dollars(2000.0),
            Self::BarrettM107 => Currency::dollars(13000.0),
                        Self::Blowgun => Currency::dollars(30.0),
                        Self::Blunderbuss => Currency::dollars(300.0),
            Self::Bola => Currency::dollars(20.0),
                        Self::Bow => Currency::dollars(100.0),
            Self::BoysAntiTankRifle => Currency::dollars(1500.0),
            Self::BAR => Currency::dollars(1200.0),
            Self::BrenGun => Currency::dollars(1100.0),
            Self::BarrettM82 => Currency::dollars(8000.0),
            Self::BenelliM3 => Currency::dollars(1300.0),
            Self::BerettaM92 => Currency::dollars(450.0),
            Self::BrowningHiPower => Currency::dollars(350.0),
            Self::BrowningM1917 => Currency::dollars(2000.0),
            Self::BrowningM2 => Currency::dollars(3500.0),
            Self::BrowningAuto5 => Currency::dollars(400.0),
                                    Self::Carbine => Currency::dollars(300.0),
            Self::Caliver => Currency::dollars(300.0),
            Self::CapLockPistol => Currency::dollars(150.0),
            Self::Catapult => Currency::dollars(3000.0),
            Self::CompositeBow => Currency::dollars(900.0),
                        Self::Chakram => Currency::dollars(20.0),
            Self::ClaymoreMine => Currency::dollars(120.0),
            Self::ChassepotRifle => Currency::dollars(400.0),
            Self::CheyTacM200 => Currency::dollars(12000.0),
            Self::ChiappaRhino => Currency::dollars(1000.0),
            Self::Chukonu => Currency::dollars(250.0),
            Self::ColtWalker => Currency::dollars(300.0),
            Self::ColtM1911 => Currency::dollars(300.0),
            Self::Coilgun => Currency::dollars(25000.0),
            Self::Crossbow => Currency::dollars(150.0),
            Self::CZ75 => Currency::dollars(500.0),
            Self::CZP10 => Currency::dollars(500.0),
            Self::C4Explosive => Currency::dollars(50.0),
            Self::Daikyu => Currency::dollars(450.0),
            Self::Dart => Currency::dollars(10.0),
            Self::Dynamite => Currency::dollars(20.0),
            Self::DreyseNeedleRifle => Currency::dollars(450.0),
            Self::Derringer => Currency::dollars(100.0),
            Self::DesertEagle => Currency::dollars(750.0),
            Self::DoubleActionRevolver => Currency::dollars(250.0),
            Self::DoubleBarrelShotgun => Currency::dollars(150.0),
            Self::DragunovSVD => Currency::dollars(3500.0),
            Self::Espringal => Currency::dollars(1500.0),
            Self::EnfieldPattern1853 => Currency::dollars(375.0),
                        Self::Flamethrower => Currency::dollars(1000.0),
            Self::FlareGun => Currency::dollars(50.0),
            Self::FieldGun => Currency::dollars(5000.0),
            Self::Flintlock => Currency::dollars(200.0),
            Self::FAMAS => Currency::dollars(2200.0),
            Self::FNFAL => Currency::dollars(700.0),
            Self::FNMAG => Currency::dollars(3000.0),
            Self::FNP90 => Currency::dollars(1800.0),
            Self::FNSCARL => Currency::dollars(2800.0),
            Self::FNFiveSeveN => Currency::dollars(1000.0),
            Self::FNMinimiPara => Currency::dollars(4500.0),
            Self::FostechOrigin12 => Currency::dollars(1800.0),
            Self::Fusil => Currency::dollars(350.0),
            Self::GatlingGun => Currency::dollars(3000.0),
            Self::GardnerGun => Currency::dollars(2800.0),
            Self::Grenade => Currency::dollars(30.0),
            Self::GreaseGunM3 => Currency::dollars(225.0),
            Self::Hackbut => Currency::dollars(500.0),
                        Self::Gastraphetes => Currency::dollars(400.0),
            Self::GreatBow => Currency::dollars(700.0),
            Self::Glock17 => Currency::dollars(400.0),
            Self::Glock43X => Currency::dollars(550.0),
            Self::G3 => Currency::dollars(750.0),
            Self::GP25 => Currency::dollars(500.0),
            Self::GaussRifle => Currency::dollars(30000.0),
            Self::HornBow => Currency::dollars(800.0),
            Self::HandCannon => Currency::dollars(1000.0),
                        Self::Hankyu => Currency::dollars(250.0),
            Self::HandMortar => Currency::dollars(800.0),
            Self::HenryRifle => Currency::dollars(400.0),
            Self::HandCrossbow => Currency::dollars(150.0),
                        Self::HeavyCrossbow => Currency::dollars(200.0),
            Self::Hurlbat => Currency::dollars(30.0),
                        Self::HuntingRifle => Currency::dollars(700.0),
            Self::HK416 => Currency::dollars(2800.0),
            Self::HKMP7 => Currency::dollars(1800.0),
            Self::HKMark23 => Currency::dollars(2400.0),
            Self::HKMG4 => Currency::dollars(4200.0),
            Self::Jezail => Currency::dollars(400.0),
            Self::KelTecKSG => Currency::dollars(1000.0),
            Self::KrissVector => Currency::dollars(1600.0),
            Self::LematRevolver => Currency::dollars(350.0),
            Self::LeeEnfieldSMLE => Currency::dollars(425.0),
            Self::LeverActionRifle => Currency::dollars(350.0),
            Self::LewisGun => Currency::dollars(1800.0),
            Self::LightCrossbow => Currency::dollars(150.0),
                                    Self::Longbow => Currency::dollars(200.0),
            Self::LugerP08 => Currency::dollars(350.0),
            Self::L96A1 => Currency::dollars(6000.0),
            Self::LAWM72 => Currency::dollars(1000.0),
            Self::LaserRifle => Currency::dollars(50000.0),
            Self::M1Garand => Currency::dollars(500.0),
            Self::MakarovPM => Currency::dollars(300.0),
            Self::M14 => Currency::dollars(750.0),
            Self::M16 => Currency::dollars(800.0),
            Self::M203 => Currency::dollars(600.0),
            Self::M21SWS => Currency::dollars(4500.0),
            Self::M249SAW => Currency::dollars(3500.0),
            Self::M60 => Currency::dollars(3200.0),
            Self::M4Carbine => Currency::dollars(1400.0),
            Self::M79 => Currency::dollars(1000.0),
            Self::MAC10 => Currency::dollars(350.0),
            Self::Mangonel => Currency::dollars(3500.0),
            Self::Matchlock => Currency::dollars(400.0),
            Self::MartiniHenryRifle => Currency::dollars(450.0),
            Self::MauserModel1871 => Currency::dollars(500.0),
            Self::MauserC96 => Currency::dollars(325.0),
            Self::MauserGewehr98 => Currency::dollars(450.0),
            Self::MG34 => Currency::dollars(2500.0),
            Self::MG42 => Currency::dollars(2200.0),
            Self::MG3 => Currency::dollars(2800.0),
            Self::MaximGun => Currency::dollars(4500.0),
            Self::MachineGun => Currency::dollars(4000.0),
            Self::McMillanTAC50 => Currency::dollars(11000.0),
            Self::MilkorMGL => Currency::dollars(8000.0),
            Self::Mitrailleuse => Currency::dollars(3500.0),
            Self::Mk19 => Currency::dollars(15000.0),
            Self::MosinNagant => Currency::dollars(400.0),
            Self::Mossberg500 => Currency::dollars(400.0),
            Self::MP18 => Currency::dollars(400.0),
            Self::MP40 => Currency::dollars(350.0),
            Self::MP5 => Currency::dollars(450.0),
            Self::MountainGun => Currency::dollars(3500.0),
            Self::MTS255 => Currency::dollars(1500.0),
                        Self::Musket => Currency::dollars(300.0),
            Self::Musketoon => Currency::dollars(250.0),
            Self::NavalGun => Currency::dollars(8000.0),
            Self::NegevNG7 => Currency::dollars(5000.0),
            Self::NordenfeltGun => Currency::dollars(3200.0),
            Self::Pecheneg => Currency::dollars(4800.0),
            Self::PelletBow => Currency::dollars(400.0),
                        Self::Pistol => Currency::dollars(350.0),
                        Self::PelletSling => Currency::dollars(15.0),
            Self::Petronel => Currency::dollars(300.0),
            Self::PepperboxRevolver => Currency::dollars(200.0),
            Self::PercussionRevolver => Currency::dollars(250.0),
            Self::Plumbata => Currency::dollars(5.0),
            Self::PPS43 => Currency::dollars(200.0),
            Self::PPSh41 => Currency::dollars(250.0),
            Self::PKM => Currency::dollars(3000.0),
            Self::PSG1 => Currency::dollars(7000.0),
            Self::PGMHecateII => Currency::dollars(12000.0),
            Self::PlasmaRifle => Currency::dollars(60000.0),
            Self::PTRDAntiTankRifle => Currency::dollars(1600.0),
            Self::Prodd => Currency::dollars(100.0),
            Self::PumpActionShotgun => Currency::dollars(300.0),
            Self::QBZ95 => Currency::dollars(2000.0),
            Self::Remington870 => Currency::dollars(450.0),
            Self::RemingtonRollingBlockPistol => Currency::dollars(200.0),
            Self::RemingtonRollingBlockRifle => Currency::dollars(425.0),
            Self::RemingtonModel31 => Currency::dollars(325.0),
            Self::Revolver => Currency::dollars(300.0),
            Self::RepeatingCrossbow => Currency::dollars(300.0),
            Self::Rifle => Currency::dollars(500.0),
            Self::Rock => Currency::dollars(0.0),
                        Self::RocketLauncher => Currency::dollars(2000.0),
            Self::RPG7 => Currency::dollars(500.0),
            Self::Railgun => Currency::dollars(75000.0),
            Self::Scorpion => Currency::dollars(1000.0),
            Self::RecurveBow => Currency::dollars(500.0),
            Self::Saiga12 => Currency::dollars(900.0),
            Self::SakoTRG42 => Currency::dollars(9000.0),
            Self::SelfBow => Currency::dollars(50.0),
            Self::Shotgun => Currency::dollars(500.0),
                        Self::ShortBow => Currency::dollars(50.0),
            Self::SharpsRifle => Currency::dollars(425.0),
            Self::SiegeCrossbow => Currency::dollars(500.0),
            Self::SIGP220 => Currency::dollars(600.0),
            Self::SIGP320 => Currency::dollars(600.0),
            Self::SingleActionRevolver => Currency::dollars(300.0),
            Self::Shuriken => Currency::dollars(5.0),
            Self::Skorpion => Currency::dollars(300.0),
            Self::Sling => Currency::dollars(20.0),
            Self::SmithWessonModel1 => Currency::dollars(100.0),
            Self::SmithWessonModel3 => Currency::dollars(250.0),
            Self::SMG => Currency::dollars(450.0),
            Self::SniperRifle => Currency::dollars(3500.0),
            Self::SPAS12 => Currency::dollars(1500.0),
            Self::SpencerRepeatingRifle => Currency::dollars(400.0),
            Self::SpringfieldModel1861 => Currency::dollars(375.0),
            Self::SpringfieldM1903 => Currency::dollars(475.0),
            Self::TrapdoorSpringfield => Currency::dollars(425.0),
            Self::StenGun => Currency::dollars(150.0),
            Self::Sterling => Currency::dollars(400.0),
            Self::SteyrAUG => Currency::dollars(2400.0),
            Self::StingerMissile => Currency::dollars(40000.0),
                        Self::StaffSling => Currency::dollars(20.0),
            Self::Stonebow => Currency::dollars(120.0),
            Self::Boomerang => Currency::dollars(15.0),
                        Self::ThrowingAxe => Currency::dollars(60.0),
            Self::ThrowingHammer => Currency::dollars(70.0),
                        Self::ThrowingKnife => Currency::dollars(30.0),
            Self::ThrowingSpear => Currency::dollars(50.0),
                        Self::ThrowingStick => Currency::dollars(5.0),
            Self::ThompsonM1928 => Currency::dollars(600.0),
            Self::Trebuchet => Currency::dollars(5000.0),
            Self::TNT => Currency::dollars(30.0),
            Self::TOWMissile => Currency::dollars(60000.0),
            Self::Uzi => Currency::dollars(400.0),
            Self::Ultimax100 => Currency::dollars(3800.0),
            Self::USAS12 => Currency::dollars(2500.0),
            Self::VolcanicPistol => Currency::dollars(275.0),
            Self::VickersMachineGun => Currency::dollars(2500.0),
                        Self::Warbow => Currency::dollars(600.0),
            Self::WaltherPPK => Currency::dollars(250.0),
            Self::WebleyRIC => Currency::dollars(200.0),
            Self::WebleyMkVI => Currency::dollars(225.0),
            Self::Wheellock => Currency::dollars(250.0),
            Self::Winchester1866 => Currency::dollars(400.0),
            Self::Winchester1873 => Currency::dollars(425.0),
            Self::Winchester1887 => Currency::dollars(350.0),
            Self::Winchester1897 => Currency::dollars(325.0),
            Self::WinchesterRepeater => Currency::dollars(450.0),
            Self::XM8 => Currency::dollars(2600.0),
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
            Self::AccuracyInternationalAWM => Weight::pounds(15.0),
            Self::AA12 => Weight::pounds(10.0),
            Self::Arquebus => Weight::pounds(11.0),
            Self::AK47 => Weight::pounds(9.5),
            Self::AKM => Weight::pounds(8.0),
            Self::AK12 => Weight::pounds(7.8),
            Self::ASVal => Weight::pounds(5.5),
            Self::AssaultRifle => Weight::pounds(9.0),
            Self::ArisakaType38 => Weight::pounds(9.0),
            Self::AT4 => Weight::pounds(14.8),
                        Self::Atlatl => Weight::pounds(1.0),
            Self::Ballista => Weight::pounds(350.0),
            Self::BarrettM107 => Weight::pounds(30.9),
                        Self::Blowgun => Weight::pounds(1.0),
                        Self::Blunderbuss => Weight::pounds(8.0),
            Self::Bola => Weight::pounds(1.0),
                        Self::Bow => Weight::pounds(2.0),
            Self::BoysAntiTankRifle => Weight::pounds(36.0),
            Self::BAR => Weight::pounds(19.0),
            Self::BrenGun => Weight::pounds(22.0),
            Self::BarrettM82 => Weight::pounds(30.0),
            Self::BenelliM3 => Weight::pounds(7.5),
            Self::BerettaM92 => Weight::pounds(2.1),
            Self::BrowningHiPower => Weight::pounds(2.0),
            Self::BrowningM1917 => Weight::pounds(40.0),
            Self::BrowningM2 => Weight::pounds(84.0),
            Self::BrowningAuto5 => Weight::pounds(8.5),
                                    Self::Carbine => Weight::pounds(7.0),
            Self::Caliver => Weight::pounds(8.0),
            Self::CapLockPistol => Weight::pounds(2.5),
            Self::Catapult => Weight::pounds(500.0),
            Self::CompositeBow => Weight::pounds(2.0),
                        Self::Chakram => Weight::pounds(1.0),
            Self::ClaymoreMine => Weight::pounds(3.5),
            Self::ChassepotRifle => Weight::pounds(9.5),
            Self::CheyTacM200 => Weight::pounds(31.0),
            Self::ChiappaRhino => Weight::pounds(1.8),
            Self::Chukonu => Weight::pounds(7.0),
            Self::ColtWalker => Weight::pounds(4.5),
            Self::ColtM1911 => Weight::pounds(2.5),
            Self::Coilgun => Weight::pounds(12.0),
            Self::Crossbow => Weight::pounds(6.0),
            Self::CZ75 => Weight::pounds(2.2),
            Self::CZP10 => Weight::pounds(1.6),
            Self::C4Explosive => Weight::pounds(1.25),
            Self::Daikyu => Weight::pounds(3.5),
            Self::Dart => Weight::pounds(0.1),
            Self::Dynamite => Weight::pounds(0.5),
            Self::DreyseNeedleRifle => Weight::pounds(10.5),
                        Self::Derringer => Weight::pounds(0.5),
            Self::DesertEagle => Weight::pounds(4.4),
            Self::DoubleActionRevolver => Weight::pounds(2.0),
            Self::DoubleBarrelShotgun => Weight::pounds(7.5),
            Self::DragunovSVD => Weight::pounds(9.5),
            Self::Espringal => Weight::pounds(300.0),
            Self::EnfieldPattern1853 => Weight::pounds(9.5),
                        Self::Flamethrower => Weight::pounds(70.0),
            Self::FlareGun => Weight::pounds(2.0),
            Self::FieldGun => Weight::pounds(2000.0),
            Self::Flintlock => Weight::pounds(2.5),
            Self::FAMAS => Weight::pounds(8.0),
            Self::FNFAL => Weight::pounds(10.0),
            Self::FNMAG => Weight::pounds(24.5),
            Self::FNP90 => Weight::pounds(5.7),
            Self::FNSCARL => Weight::pounds(7.3),
            Self::FNFiveSeveN => Weight::pounds(1.5),
            Self::FNMinimiPara => Weight::pounds(15.0),
            Self::FostechOrigin12 => Weight::pounds(9.0),
            Self::Fusil => Weight::pounds(8.0),
            Self::GatlingGun => Weight::pounds(200.0),
            Self::GardnerGun => Weight::pounds(180.0),
                        Self::Grenade => Weight::pounds(1.0),
            Self::GreaseGunM3 => Weight::pounds(8.0),
            Self::Hackbut => Weight::pounds(25.0),
                        Self::Gastraphetes => Weight::pounds(14.0),
            Self::GreatBow => Weight::pounds(6.0),
            Self::Glock17 => Weight::pounds(1.6),
            Self::Glock43X => Weight::pounds(1.2),
            Self::G3 => Weight::pounds(9.7),
            Self::GP25 => Weight::pounds(3.5),
            Self::GaussRifle => Weight::pounds(11.0),
            Self::HornBow => Weight::pounds(3.0),
            Self::HandCannon => Weight::pounds(15.0),
                        Self::Hankyu => Weight::pounds(1.5),
            Self::HandMortar => Weight::pounds(20.0),
            Self::HenryRifle => Weight::pounds(9.5),
            Self::HandCrossbow => Weight::pounds(3.0),
                        Self::HeavyCrossbow => Weight::pounds(8.0),
            Self::Hurlbat => Weight::pounds(1.5),
                        Self::HuntingRifle => Weight::pounds(9.0),
            Self::HK416 => Weight::pounds(7.5),
            Self::HKMP7 => Weight::pounds(4.2),
            Self::HKMark23 => Weight::pounds(2.8),
            Self::HKMG4 => Weight::pounds(18.0),
            Self::Jezail => Weight::pounds(11.0),
            Self::KelTecKSG => Weight::pounds(6.9),
            Self::KrissVector => Weight::pounds(5.5),
            Self::LematRevolver => Weight::pounds(3.0),
            Self::LeeEnfieldSMLE => Weight::pounds(8.75),
            Self::LeverActionRifle => Weight::pounds(8.0),
            Self::LewisGun => Weight::pounds(28.0),
            Self::LightCrossbow => Weight::pounds(4.0),
                                    Self::Longbow => Weight::pounds(3.0),
            Self::LugerP08 => Weight::pounds(1.75),
            Self::L96A1 => Weight::pounds(15.0),
            Self::LAWM72 => Weight::pounds(5.2),
            Self::LaserRifle => Weight::pounds(10.0),
            Self::M1Garand => Weight::pounds(9.5),
            Self::MakarovPM => Weight::pounds(1.7),
            Self::M14 => Weight::pounds(9.2),
            Self::M16 => Weight::pounds(7.2),
            Self::M203 => Weight::pounds(3.0),
            Self::M21SWS => Weight::pounds(11.0),
            Self::M249SAW => Weight::pounds(17.0),
            Self::M60 => Weight::pounds(23.0),
            Self::M4Carbine => Weight::pounds(6.5),
            Self::M79 => Weight::pounds(6.0),
            Self::MAC10 => Weight::pounds(6.3),
            Self::Mangonel => Weight::pounds(600.0),
            Self::Matchlock => Weight::pounds(12.0),
            Self::MartiniHenryRifle => Weight::pounds(9.0),
            Self::MauserModel1871 => Weight::pounds(10.0),
            Self::MauserC96 => Weight::pounds(2.5),
            Self::MauserGewehr98 => Weight::pounds(9.0),
            Self::MG34 => Weight::pounds(26.0),
            Self::MG42 => Weight::pounds(25.0),
            Self::MG3 => Weight::pounds(24.5),
            Self::MaximGun => Weight::pounds(60.0),
            Self::MachineGun => Weight::pounds(30.0),
            Self::McMillanTAC50 => Weight::pounds(26.0),
            Self::MilkorMGL => Weight::pounds(12.0),
            Self::Mitrailleuse => Weight::pounds(300.0),
            Self::Mk19 => Weight::pounds(75.0),
            Self::MosinNagant => Weight::pounds(8.75),
            Self::Mossberg500 => Weight::pounds(7.25),
            Self::MP18 => Weight::pounds(9.0),
            Self::MP40 => Weight::pounds(8.0),
            Self::MP5 => Weight::pounds(5.5),
            Self::MountainGun => Weight::pounds(800.0),
            Self::MTS255 => Weight::pounds(9.0),
                        Self::Musket => Weight::pounds(10.0),
            Self::Musketoon => Weight::pounds(6.0),
            Self::NavalGun => Weight::pounds(5000.0),
            Self::NegevNG7 => Weight::pounds(16.5),
            Self::NordenfeltGun => Weight::pounds(250.0),
            Self::Pecheneg => Weight::pounds(19.0),
            Self::PelletBow => Weight::pounds(3.0),
                        Self::Pistol => Weight::pounds(1.5),
                        Self::PelletSling => Weight::pounds(0.5),
            Self::Petronel => Weight::pounds(5.0),
            Self::PepperboxRevolver => Weight::pounds(2.0),
            Self::PercussionRevolver => Weight::pounds(2.5),
            Self::Plumbata => Weight::pounds(0.5),
            Self::PPS43 => Weight::pounds(7.0),
            Self::PPSh41 => Weight::pounds(8.0),
            Self::PKM => Weight::pounds(16.5),
            Self::PSG1 => Weight::pounds(17.8),
            Self::PGMHecateII => Weight::pounds(30.0),
            Self::PlasmaRifle => Weight::pounds(13.0),
            Self::PTRDAntiTankRifle => Weight::pounds(38.0),
            Self::Prodd => Weight::pounds(7.0),
            Self::PumpActionShotgun => Weight::pounds(8.0),
            Self::QBZ95 => Weight::pounds(7.3),
            Self::Remington870 => Weight::pounds(7.5),
            Self::RemingtonRollingBlockPistol => Weight::pounds(2.5),
            Self::RemingtonRollingBlockRifle => Weight::pounds(9.0),
            Self::RemingtonModel31 => Weight::pounds(7.5),
            Self::Revolver => Weight::pounds(2.0),
            Self::RepeatingCrossbow => Weight::pounds(8.0),
            Self::Rifle => Weight::pounds(9.0),
            Self::Rock => Weight::pounds(0.5),
                        Self::RocketLauncher => Weight::pounds(15.0),
            Self::RPG7 => Weight::pounds(15.0),
            Self::Railgun => Weight::pounds(15.0),
            Self::Scorpion => Weight::pounds(200.0),
            Self::RecurveBow => Weight::pounds(3.0),
            Self::Saiga12 => Weight::pounds(8.5),
            Self::SakoTRG42 => Weight::pounds(11.0),
            Self::SelfBow => Weight::pounds(2.0),
            Self::Shotgun => Weight::pounds(8.0),
                        Self::ShortBow => Weight::pounds(1.0),
            Self::SharpsRifle => Weight::pounds(10.5),
            Self::SiegeCrossbow => Weight::pounds(15.0),
            Self::SIGP220 => Weight::pounds(1.9),
            Self::SIGP320 => Weight::pounds(1.7),
            Self::SingleActionRevolver => Weight::pounds(2.5),
            Self::Shuriken => Weight::pounds(0.1),
            Self::Skorpion => Weight::pounds(3.0),
            Self::Sling => Weight::pounds(0.5),
            Self::SmithWessonModel1 => Weight::pounds(1.0),
            Self::SmithWessonModel3 => Weight::pounds(2.5),
            Self::SMG => Weight::pounds(7.0),
            Self::SniperRifle => Weight::pounds(11.0),
            Self::SPAS12 => Weight::pounds(9.5),
            Self::SpencerRepeatingRifle => Weight::pounds(10.0),
            Self::SpringfieldModel1861 => Weight::pounds(9.5),
            Self::SpringfieldM1903 => Weight::pounds(8.75),
            Self::TrapdoorSpringfield => Weight::pounds(9.0),
            Self::StenGun => Weight::pounds(6.5),
            Self::Sterling => Weight::pounds(6.0),
            Self::SteyrAUG => Weight::pounds(7.9),
            Self::StingerMissile => Weight::pounds(34.5),
                        Self::StaffSling => Weight::pounds(1.0),
            Self::Stonebow => Weight::pounds(6.0),
            Self::Boomerang => Weight::pounds(1.0),
                        Self::ThrowingAxe => Weight::pounds(2.0),
            Self::ThrowingHammer => Weight::pounds(3.0),
                        Self::ThrowingKnife => Weight::pounds(0.5),
            Self::ThrowingSpear => Weight::pounds(3.0),
                        Self::ThrowingStick => Weight::pounds(1.0),
            Self::ThompsonM1928 => Weight::pounds(10.5),
            Self::Trebuchet => Weight::pounds(1000.0),
            Self::TNT => Weight::pounds(1.0),
            Self::TOWMissile => Weight::pounds(50.0),
            Self::Uzi => Weight::pounds(7.7),
            Self::Ultimax100 => Weight::pounds(10.5),
            Self::USAS12 => Weight::pounds(12.0),
            Self::VolcanicPistol => Weight::pounds(3.5),
            Self::VickersMachineGun => Weight::pounds(40.0),
                        Self::Warbow => Weight::pounds(5.0),
            Self::WaltherPPK => Weight::pounds(1.25),
            Self::WebleyRIC => Weight::pounds(2.0),
            Self::WebleyMkVI => Weight::pounds(2.5),
            Self::Wheellock => Weight::pounds(3.0),
            Self::Winchester1866 => Weight::pounds(9.0),
            Self::Winchester1873 => Weight::pounds(9.5),
            Self::Winchester1887 => Weight::pounds(9.5),
            Self::Winchester1897 => Weight::pounds(8.0),
            Self::WinchesterRepeater => Weight::pounds(9.0),
            Self::XM8 => Weight::pounds(6.2),
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
            Self::AccuracyInternationalAWM => TechLevel::new(8), // Information Age
            Self::AA12 => TechLevel::new(8),           // Information Age
            Self::Arquebus => TechLevel::new(4),       // Renaissance
            Self::AK47 => TechLevel::new(7),           // Digital Age
            Self::AKM => TechLevel::new(7),            // Digital Age
            Self::AK12 => TechLevel::new(8),           // Information Age
            Self::ASVal => TechLevel::new(8),          // Information Age
            Self::AssaultRifle => TechLevel::new(7),   // Digital Age
            Self::ArisakaType38 => TechLevel::new(6),  // Atomic Age
            Self::AT4 => TechLevel::new(7),            // Digital Age
                        Self::Atlatl => TechLevel::new(0),         // Stone Age
            Self::Ballista => TechLevel::new(1),       // Roman/Iron Age
            Self::BarrettM107 => TechLevel::new(8),    // Information Age
                        Self::Blowgun => TechLevel::new(0),        // Stone Age
                        Self::Blunderbuss => TechLevel::new(4),    // Renaissance
            Self::Bola => TechLevel::new(0),           // Stone Age
                        Self::Bow => TechLevel::new(0),            // Stone Age
            Self::BoysAntiTankRifle => TechLevel::new(6), // Atomic Age
            Self::BAR => TechLevel::new(6),            // Atomic Age
            Self::BrenGun => TechLevel::new(6),        // Atomic Age
            Self::BarrettM82 => TechLevel::new(7),     // Digital Age
            Self::BenelliM3 => TechLevel::new(7),      // Digital Age
            Self::BerettaM92 => TechLevel::new(7),     // Digital Age
            Self::BrowningHiPower => TechLevel::new(6), // Atomic Age
            Self::BrowningM1917 => TechLevel::new(6),  // Atomic Age
            Self::BrowningM2 => TechLevel::new(6),     // Atomic Age
            Self::BrowningAuto5 => TechLevel::new(6),  // Atomic Age
                                    Self::Carbine => TechLevel::new(5),        // Mechanized Age
            Self::Caliver => TechLevel::new(4),        // Renaissance
            Self::CapLockPistol => TechLevel::new(5),  // Industrial Revolution
            Self::Catapult => TechLevel::new(1),       // Roman/Iron Age
            Self::CompositeBow => TechLevel::new(2),   // Medieval
                        Self::Chakram => TechLevel::new(1),        // Bronze Age
            Self::ClaymoreMine => TechLevel::new(7),   // Digital Age
            Self::ChassepotRifle => TechLevel::new(5), // Industrial Revolution
            Self::CheyTacM200 => TechLevel::new(8),    // Information Age
            Self::ChiappaRhino => TechLevel::new(8),   // Information Age
            Self::Chukonu => TechLevel::new(2),        // Medieval China
            Self::ColtWalker => TechLevel::new(5),     // Industrial Revolution
            Self::ColtM1911 => TechLevel::new(6),      // Atomic Age
            Self::Coilgun => TechLevel::new(8),        // Information Age (experimental)
            Self::Crossbow => TechLevel::new(2),       // Medieval
            Self::CZ75 => TechLevel::new(7),           // Digital Age
            Self::CZP10 => TechLevel::new(8),          // Information Age
            Self::C4Explosive => TechLevel::new(7),    // Digital Age
            Self::Daikyu => TechLevel::new(3),        // Japanese
            Self::Dart => TechLevel::new(0),           // Stone Age
            Self::Dynamite => TechLevel::new(5),       // Industrial Revolution
            Self::DreyseNeedleRifle => TechLevel::new(5), // Industrial Revolution
            Self::Derringer => TechLevel::new(5),      // Mechanized Age
            Self::DesertEagle => TechLevel::new(7),    // Digital Age
            Self::DoubleActionRevolver => TechLevel::new(5), // Industrial Revolution
            Self::DoubleBarrelShotgun => TechLevel::new(5), // Industrial Revolution
            Self::DragunovSVD => TechLevel::new(7),    // Digital Age
            Self::Espringal => TechLevel::new(2),      // Medieval
            Self::EnfieldPattern1853 => TechLevel::new(5), // Industrial Revolution
                        Self::Flamethrower => TechLevel::new(6),   // Atomic Age
            Self::FlareGun => TechLevel::new(6),       // Atomic Age
            Self::FieldGun => TechLevel::new(5),       // Industrial Revolution
            Self::Flintlock => TechLevel::new(4),      // Renaissance
            Self::FAMAS => TechLevel::new(8),          // Information Age
            Self::FNFAL => TechLevel::new(7),          // Digital Age
            Self::FNMAG => TechLevel::new(7),          // Digital Age
            Self::FNP90 => TechLevel::new(8),          // Information Age
            Self::FNSCARL => TechLevel::new(8),        // Information Age
            Self::FNFiveSeveN => TechLevel::new(8),    // Information Age
            Self::FNMinimiPara => TechLevel::new(8),   // Information Age
            Self::FostechOrigin12 => TechLevel::new(8), // Information Age
            Self::Fusil => TechLevel::new(4),          // Renaissance
            Self::GatlingGun => TechLevel::new(5),     // Industrial Revolution
            Self::GardnerGun => TechLevel::new(5),     // Industrial Revolution
                        Self::Grenade => TechLevel::new(6),        // Atomic Age
            Self::GreaseGunM3 => TechLevel::new(6),    // Atomic Age
            Self::Hackbut => TechLevel::new(4),        // Renaissance
                        Self::Gastraphetes => TechLevel::new(1),   // Greek/Iron Age
            Self::GreatBow => TechLevel::new(2),       // Medieval
            Self::Glock17 => TechLevel::new(7),        // Digital Age
            Self::Glock43X => TechLevel::new(8),       // Information Age
            Self::G3 => TechLevel::new(7),             // Digital Age
            Self::GP25 => TechLevel::new(7),           // Digital Age
            Self::GaussRifle => TechLevel::new(8),     // Information Age (experimental)
            Self::HornBow => TechLevel::new(1),        // Bronze Age
            Self::HandCannon => TechLevel::new(3),     // Medieval
                        Self::Hankyu => TechLevel::new(3),        // Japanese
            Self::HandMortar => TechLevel::new(4),     // Renaissance
            Self::HenryRifle => TechLevel::new(5),     // Industrial Revolution
            Self::HandCrossbow => TechLevel::new(2),   // Medieval
                        Self::HeavyCrossbow => TechLevel::new(2),  // Medieval
            Self::Hurlbat => TechLevel::new(1),        // Bronze Age
                        Self::HuntingRifle => TechLevel::new(5),   // Mechanized Age
            Self::HK416 => TechLevel::new(8),          // Information Age
            Self::HKMP7 => TechLevel::new(8),          // Information Age
            Self::HKMark23 => TechLevel::new(8),       // Information Age
            Self::HKMG4 => TechLevel::new(8),          // Information Age
            Self::Jezail => TechLevel::new(4),         // Renaissance
            Self::KelTecKSG => TechLevel::new(8),      // Information Age
            Self::KrissVector => TechLevel::new(8),    // Information Age
            Self::LematRevolver => TechLevel::new(5),  // Industrial Revolution
            Self::LeeEnfieldSMLE => TechLevel::new(6), // Atomic Age
            Self::LeverActionRifle => TechLevel::new(5), // Industrial Revolution
            Self::LewisGun => TechLevel::new(6),       // Atomic Age
            Self::LightCrossbow => TechLevel::new(2),  // Medieval
                        Self::Longbow => TechLevel::new(0),        // Stone Age
            Self::LugerP08 => TechLevel::new(6),       // Atomic Age
            Self::L96A1 => TechLevel::new(7),          // Digital Age
            Self::LAWM72 => TechLevel::new(7),         // Digital Age
            Self::LaserRifle => TechLevel::new(8),     // Information Age (experimental)
            Self::M1Garand => TechLevel::new(6),       // Atomic Age
            Self::MakarovPM => TechLevel::new(7),      // Digital Age
            Self::M14 => TechLevel::new(7),            // Digital Age
            Self::M16 => TechLevel::new(7),            // Digital Age
            Self::M203 => TechLevel::new(7),           // Digital Age
            Self::M21SWS => TechLevel::new(7),          // Digital Age
            Self::M249SAW => TechLevel::new(7),        // Digital Age
            Self::M4Carbine => TechLevel::new(8),      // Information Age
            Self::M60 => TechLevel::new(7),            // Digital Age
            Self::M79 => TechLevel::new(7),            // Digital Age
            Self::MAC10 => TechLevel::new(7),          // Digital Age
            Self::Mangonel => TechLevel::new(1),       // Roman/Iron Age
            Self::Matchlock => TechLevel::new(4),      // Renaissance
            Self::MartiniHenryRifle => TechLevel::new(5), // Industrial Revolution
            Self::MauserModel1871 => TechLevel::new(5), // Industrial Revolution
            Self::MauserC96 => TechLevel::new(6),      // Atomic Age
            Self::MauserGewehr98 => TechLevel::new(6), // Atomic Age
            Self::MG34 => TechLevel::new(6),           // Atomic Age
            Self::MG42 => TechLevel::new(6),           // Atomic Age
            Self::MG3 => TechLevel::new(7),            // Digital Age
            Self::MaximGun => TechLevel::new(5),       // Industrial Revolution
            Self::MachineGun => TechLevel::new(6),     // Atomic Age
            Self::McMillanTAC50 => TechLevel::new(8),  // Information Age
            Self::Mitrailleuse => TechLevel::new(5),   // Industrial Revolution
            Self::Mk19 => TechLevel::new(7),           // Digital Age
            Self::MilkorMGL => TechLevel::new(8),      // Information Age
            Self::MosinNagant => TechLevel::new(6),    // Atomic Age
            Self::Mossberg500 => TechLevel::new(7),    // Digital Age
            Self::MP18 => TechLevel::new(6),           // Atomic Age
            Self::MP40 => TechLevel::new(6),           // Atomic Age
            Self::MP5 => TechLevel::new(7),            // Digital Age
            Self::MountainGun => TechLevel::new(5),    // Industrial Revolution
            Self::MTS255 => TechLevel::new(8),         // Information Age
            Self::Musket => TechLevel::new(4),         // Age of Sail
            Self::Musketoon => TechLevel::new(4),      // Renaissance
            Self::NavalGun => TechLevel::new(5),       // Industrial Revolution
            Self::NegevNG7 => TechLevel::new(8),       // Information Age
            Self::NordenfeltGun => TechLevel::new(5),  // Industrial Revolution
            Self::Pecheneg => TechLevel::new(8),       // Information Age
            Self::PelletBow => TechLevel::new(7),      // Digital Age
                        Self::Pistol => TechLevel::new(6),         // Atomic Age
                        Self::PelletSling => TechLevel::new(0),    // Stone Age
            Self::Petronel => TechLevel::new(4),       // Renaissance
            Self::PepperboxRevolver => TechLevel::new(5), // Industrial Revolution
            Self::PercussionRevolver => TechLevel::new(5), // Industrial Revolution
            Self::Plumbata => TechLevel::new(1),       // Roman
            Self::PPS43 => TechLevel::new(6),          // Atomic Age
            Self::PPSh41 => TechLevel::new(6),         // Atomic Age
            Self::PKM => TechLevel::new(7),            // Digital Age
            Self::PSG1 => TechLevel::new(7),          // Digital Age
            Self::PGMHecateII => TechLevel::new(8),    // Information Age
            Self::PlasmaRifle => TechLevel::new(8),    // Information Age (experimental)
            Self::PTRDAntiTankRifle => TechLevel::new(6), // Atomic Age
            Self::Prodd => TechLevel::new(2),          // Medieval
            Self::PumpActionShotgun => TechLevel::new(5), // Industrial Revolution
            Self::QBZ95 => TechLevel::new(8),              // Information Age
            Self::Remington870 => TechLevel::new(7),       // Digital Age
            Self::RemingtonRollingBlockPistol => TechLevel::new(5), // Industrial Revolution
            Self::RemingtonRollingBlockRifle => TechLevel::new(5), // Industrial Revolution
            Self::RemingtonModel31 => TechLevel::new(6), // Atomic Age
            Self::Revolver => TechLevel::new(6),       // Atomic Age
            Self::RepeatingCrossbow => TechLevel::new(2), // Medieval China
            Self::Rifle => TechLevel::new(6),          // Atomic Age
            Self::Rock => TechLevel::new(0),           // Stone Age
                        Self::RocketLauncher => TechLevel::new(7), // Digital Age
            Self::RPG7 => TechLevel::new(7),           // Digital Age
            Self::Railgun => TechLevel::new(8),        // Information Age (experimental)
            Self::Scorpion => TechLevel::new(1),       // Roman
            Self::RecurveBow => TechLevel::new(1),     // Bronze Age
            Self::Saiga12 => TechLevel::new(7),        // Digital Age
            Self::SakoTRG42 => TechLevel::new(8),      // Information Age
            Self::SelfBow => TechLevel::new(0),        // Stone Age
            Self::Shotgun => TechLevel::new(5),        // Mechanized Age
                        Self::ShortBow => TechLevel::new(0),       // Stone Age
            Self::SharpsRifle => TechLevel::new(5),    // Industrial Revolution
            Self::SiegeCrossbow => TechLevel::new(2),  // Medieval
            Self::SIGP220 => TechLevel::new(7),        // Digital Age
            Self::SIGP320 => TechLevel::new(8),        // Information Age
            Self::SingleActionRevolver => TechLevel::new(5), // Industrial Revolution
            Self::Shuriken => TechLevel::new(2),       // Medieval Japan
            Self::Skorpion => TechLevel::new(7),       // Digital Age
            Self::Sling => TechLevel::new(0),          // Stone Age
            Self::SmithWessonModel1 => TechLevel::new(5), // Industrial Revolution
            Self::SmithWessonModel3 => TechLevel::new(5), // Industrial Revolution
            Self::SMG => TechLevel::new(6),            // Atomic Age
            Self::SniperRifle => TechLevel::new(7),    // Digital Age
            Self::SPAS12 => TechLevel::new(7),         // Digital Age
            Self::SpencerRepeatingRifle => TechLevel::new(5), // Industrial Revolution
            Self::SpringfieldModel1861 => TechLevel::new(5), // Industrial Revolution
            Self::SpringfieldM1903 => TechLevel::new(6), // Atomic Age
            Self::TrapdoorSpringfield => TechLevel::new(5), // Industrial Revolution
            Self::StenGun => TechLevel::new(6),        // Atomic Age
            Self::Sterling => TechLevel::new(7),       // Digital Age
            Self::SteyrAUG => TechLevel::new(8),        // Information Age
            Self::StingerMissile => TechLevel::new(7), // Digital Age
                        Self::StaffSling => TechLevel::new(1),     // Bronze Age
            Self::Stonebow => TechLevel::new(2),       // Medieval
            Self::Boomerang => TechLevel::new(0),      // Stone Age
                        Self::ThrowingAxe => TechLevel::new(0),    // Stone Age
            Self::ThrowingHammer => TechLevel::new(0), // Stone Age
                        Self::ThrowingKnife => TechLevel::new(0),  // Stone Age
            Self::ThrowingSpear => TechLevel::new(0),  // Stone Age
                        Self::ThrowingStick => TechLevel::new(0),  // Stone Age
            Self::ThompsonM1928 => TechLevel::new(6),  // Atomic Age
            Self::Trebuchet => TechLevel::new(2),      // Medieval
            Self::TNT => TechLevel::new(5),            // Industrial Revolution
            Self::TOWMissile => TechLevel::new(7),     // Digital Age
            Self::Uzi => TechLevel::new(7),            // Digital Age
            Self::Ultimax100 => TechLevel::new(8),     // Information Age
            Self::USAS12 => TechLevel::new(8),         // Information Age
            Self::VolcanicPistol => TechLevel::new(5), // Industrial Revolution
            Self::VickersMachineGun => TechLevel::new(6), // Atomic Age
                        Self::Warbow => TechLevel::new(2),         // Medieval
            Self::WaltherPPK => TechLevel::new(6),     // Atomic Age
            Self::WebleyRIC => TechLevel::new(5),      // Industrial Revolution
            Self::WebleyMkVI => TechLevel::new(6),     // Atomic Age
            Self::Wheellock => TechLevel::new(4),      // Renaissance
            Self::Winchester1866 => TechLevel::new(5), // Industrial Revolution
            Self::Winchester1873 => TechLevel::new(5), // Industrial Revolution
            Self::Winchester1887 => TechLevel::new(5), // Industrial Revolution
            Self::Winchester1897 => TechLevel::new(6), // Atomic Age
            Self::WinchesterRepeater => TechLevel::new(5), // Industrial Revolution
            Self::XM8 => TechLevel::new(8),            // Information Age
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
                dice: DamageDice::new(1, 7),
                damage_type: DamageType::Impaling,
            },
            Self::AccuracyInternationalAWM => WeaponDamage::Fixed {
                dice: DamageDice::new(7, 0),
                damage_type: DamageType::Piercing,
            },
            Self::AA12 => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 1),
                damage_type: DamageType::Piercing,
            },
            Self::Arquebus => WeaponDamage::Fixed {
                dice: DamageDice::new(3, 0),
                damage_type: DamageType::Piercing,
            },
            Self::AK47 => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::AKM => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::AK12 => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::ASVal => WeaponDamage::Fixed {
                dice: DamageDice::new(4, 0),
                damage_type: DamageType::Piercing,
            },
            Self::AssaultRifle => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::ArisakaType38 => WeaponDamage::Fixed {
                dice: DamageDice::new(4, 0),
                damage_type: DamageType::Piercing,
            },
            Self::AT4 => WeaponDamage::Fixed {
                dice: DamageDice::new(8, 0),
                damage_type: DamageType::Crushing,
            },
                        Self::Atlatl => WeaponDamage::Thrust {
                modifier: 3,
                damage_type: DamageType::Impaling,
            },
            Self::Ballista => WeaponDamage::Fixed {
                dice: DamageDice::new(3, 4),
                damage_type: DamageType::Impaling,
            },
                        Self::Blowgun => WeaponDamage::Fixed {
                dice: DamageDice::new(1, -3),
                damage_type: DamageType::Impaling,
            },
                        Self::Blunderbuss => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 2),
                damage_type: DamageType::Piercing,
            },
            Self::Bola => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
                        Self::Bow => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 0),
                damage_type: DamageType::Impaling,
            },
            Self::BoysAntiTankRifle => WeaponDamage::Fixed {
                dice: DamageDice::new(7, 0),
                damage_type: DamageType::Piercing,
            },
            Self::BAR => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::BrenGun => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::BarrettM82 => WeaponDamage::Fixed {
                dice: DamageDice::new(9, 0),
                damage_type: DamageType::Piercing,
            },
            Self::BarrettM107 => WeaponDamage::Fixed {
                dice: DamageDice::new(9, 0),
                damage_type: DamageType::Piercing,
            },
            Self::BenelliM3 => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 1),
                damage_type: DamageType::Piercing,
            },
            Self::BerettaM92 => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::BrowningHiPower => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::BrowningM1917 => WeaponDamage::Fixed {
                dice: DamageDice::new(6, 0),
                damage_type: DamageType::Piercing,
            },
            Self::BrowningM2 => WeaponDamage::Fixed {
                dice: DamageDice::new(7, 0),
                damage_type: DamageType::Piercing,
            },
            Self::BrowningAuto5 => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 1),
                damage_type: DamageType::Piercing,
            },
                                    Self::Carbine => WeaponDamage::Fixed {
                dice: DamageDice::new(4, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Caliver => WeaponDamage::Fixed {
                dice: DamageDice::new(3, 0),
                damage_type: DamageType::Piercing,
            },
            Self::CapLockPistol => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 2),
                damage_type: DamageType::Piercing,
            },
            Self::Catapult => WeaponDamage::Fixed {
                dice: DamageDice::new(3, 0),
                damage_type: DamageType::Crushing,
            },
            Self::CompositeBow => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 3),
                damage_type: DamageType::Impaling,
            },
                        Self::Chakram => WeaponDamage::Thrust {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::ClaymoreMine => WeaponDamage::Fixed {
                dice: DamageDice::new(8, 0),
                damage_type: DamageType::Crushing,
            },
            Self::ChassepotRifle => WeaponDamage::Fixed {
                dice: DamageDice::new(4, 0),
                damage_type: DamageType::Piercing,
            },
            Self::CheyTacM200 => WeaponDamage::Fixed {
                dice: DamageDice::new(8, 0),
                damage_type: DamageType::Piercing,
            },
            Self::ChiappaRhino => WeaponDamage::Fixed {
                dice: DamageDice::new(3, -1),
                damage_type: DamageType::Piercing,
            },
            Self::Chukonu => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 0),
                damage_type: DamageType::Impaling,
            },
            Self::ColtWalker => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::ColtM1911 => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::Coilgun => WeaponDamage::Fixed {
                dice: DamageDice::new(6, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Crossbow => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 4),
                damage_type: DamageType::Impaling,
            },
            Self::CZ75 => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::CZP10 => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::C4Explosive => WeaponDamage::Fixed {
                dice: DamageDice::new(8, 0),
                damage_type: DamageType::Crushing,
            },
            Self::Daikyu => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 3),
                damage_type: DamageType::Impaling,
            },
            Self::Dart => WeaponDamage::Thrust {
                modifier: -1,
                damage_type: DamageType::Impaling,
            },
            Self::Dynamite => WeaponDamage::Fixed {
                dice: DamageDice::new(6, 0),
                damage_type: DamageType::Crushing,
            },
            Self::DreyseNeedleRifle => WeaponDamage::Fixed {
                dice: DamageDice::new(3, 2),
                damage_type: DamageType::Piercing,
            },
                        Self::Derringer => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 2),
                damage_type: DamageType::Piercing,
            },
            Self::DesertEagle => WeaponDamage::Fixed {
                dice: DamageDice::new(3, 0),
                damage_type: DamageType::Piercing,
            },
            Self::DoubleActionRevolver => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 0),
                damage_type: DamageType::Piercing,
            },
            Self::DoubleBarrelShotgun => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 1),
                damage_type: DamageType::Piercing,
            },
            Self::DragunovSVD => WeaponDamage::Fixed {
                dice: DamageDice::new(6, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Espringal => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 0),
                damage_type: DamageType::Impaling,
            },
            Self::EnfieldPattern1853 => WeaponDamage::Fixed {
                dice: DamageDice::new(4, 0),
                damage_type: DamageType::Piercing,
            },
                        Self::Flamethrower => WeaponDamage::Fixed {
                dice: DamageDice::new(3, 0),
                damage_type: DamageType::Crushing, // Burning damage
            },
            Self::FlareGun => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 0),
                damage_type: DamageType::Crushing, // Signaling, minimal damage
            },
            Self::FieldGun => WeaponDamage::Fixed {
                dice: DamageDice::new(6, 0),
                damage_type: DamageType::Crushing,
            },
            Self::Flintlock => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 2),
                damage_type: DamageType::Piercing,
            },
            Self::FAMAS => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::FNFAL => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 1),
                damage_type: DamageType::Piercing,
            },
            Self::FNMAG => WeaponDamage::Fixed {
                dice: DamageDice::new(6, 0),
                damage_type: DamageType::Piercing,
            },
            Self::FNP90 => WeaponDamage::Fixed {
                dice: DamageDice::new(3, 1),
                damage_type: DamageType::Piercing,
            },
            Self::FNSCARL => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::FNFiveSeveN => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 0),
                damage_type: DamageType::Piercing,
            },
            Self::FNMinimiPara => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::FostechOrigin12 => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 1),
                damage_type: DamageType::Piercing,
            },
            Self::Fusil => WeaponDamage::Fixed {
                dice: DamageDice::new(4, 0),
                damage_type: DamageType::Piercing,
            },
            Self::GatlingGun => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::GardnerGun => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
                        Self::Grenade => WeaponDamage::Fixed {
                dice: DamageDice::new(3, 0), // 3d×2 fragmentation
                damage_type: DamageType::Crushing,
            },
            Self::GreaseGunM3 => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::Hackbut => WeaponDamage::Fixed {
                dice: DamageDice::new(3, 0),
                damage_type: DamageType::Piercing,
            },
                        Self::Gastraphetes => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 6),
                damage_type: DamageType::Impaling,
            },
            Self::GreatBow => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 4),
                damage_type: DamageType::Impaling,
            },
            Self::Glock17 => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::Glock43X => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 0),
                damage_type: DamageType::Piercing,
            },
            Self::G3 => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 1),
                damage_type: DamageType::Piercing,
            },
            Self::GP25 => WeaponDamage::Fixed {
                dice: DamageDice::new(6, 0),
                damage_type: DamageType::Crushing,
            },
            Self::GaussRifle => WeaponDamage::Fixed {
                dice: DamageDice::new(7, 0),
                damage_type: DamageType::Piercing,
            },
            Self::HornBow => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 2),
                damage_type: DamageType::Impaling,
            },
            Self::HandCannon => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
                        Self::Hankyu => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 0),
                damage_type: DamageType::Impaling,
            },
            Self::HandMortar => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 0),
                damage_type: DamageType::Crushing,
            },
            Self::HenryRifle => WeaponDamage::Fixed {
                dice: DamageDice::new(3, 1),
                damage_type: DamageType::Piercing,
            },
            Self::HandCrossbow => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 0),
                damage_type: DamageType::Impaling,
            },
                        Self::HeavyCrossbow => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 5),
                damage_type: DamageType::Impaling,
            },
            Self::Hurlbat => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
                        Self::HuntingRifle => WeaponDamage::Fixed {
                dice: DamageDice::new(7, 0),
                damage_type: DamageType::Piercing,
            },
            Self::HK416 => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::HKMP7 => WeaponDamage::Fixed {
                dice: DamageDice::new(3, 0),
                damage_type: DamageType::Piercing,
            },
            Self::HKMark23 => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::HKMG4 => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Jezail => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::KelTecKSG => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 1),
                damage_type: DamageType::Piercing,
            },
            Self::KrissVector => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::LematRevolver => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 1),
                damage_type: DamageType::Piercing,
            },
            Self::LeeEnfieldSMLE => WeaponDamage::Fixed {
                dice: DamageDice::new(4, 0),
                damage_type: DamageType::Piercing,
            },
            Self::LeverActionRifle => WeaponDamage::Fixed {
                dice: DamageDice::new(3, 0),
                damage_type: DamageType::Piercing,
            },
            Self::LewisGun => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::LightCrossbow => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 2),
                damage_type: DamageType::Impaling,
            },
                                    Self::Longbow => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 2),
                damage_type: DamageType::Impaling,
            },
            Self::LugerP08 => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 0),
                damage_type: DamageType::Piercing,
            },
            Self::L96A1 => WeaponDamage::Fixed {
                dice: DamageDice::new(7, 0),
                damage_type: DamageType::Piercing,
            },
            Self::LAWM72 => WeaponDamage::Fixed {
                dice: DamageDice::new(8, 0),
                damage_type: DamageType::Crushing,
            },
            Self::LaserRifle => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::MAC10 => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::M1Garand => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::MakarovPM => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 0),
                damage_type: DamageType::Piercing,
            },
            Self::M14 => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 1),
                damage_type: DamageType::Piercing,
            },
            Self::M16 => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::M203 => WeaponDamage::Fixed {
                dice: DamageDice::new(6, 0),
                damage_type: DamageType::Crushing,
            },
            Self::M21SWS => WeaponDamage::Fixed {
                dice: DamageDice::new(6, 0),
                damage_type: DamageType::Piercing,
            },
            Self::M249SAW => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::M4Carbine => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::M60 => WeaponDamage::Fixed {
                dice: DamageDice::new(6, 0),
                damage_type: DamageType::Piercing,
            },
            Self::M79 => WeaponDamage::Fixed {
                dice: DamageDice::new(6, 0),
                damage_type: DamageType::Crushing,
            },
            Self::Mangonel => WeaponDamage::Fixed {
                dice: DamageDice::new(3, 0),
                damage_type: DamageType::Crushing,
            },
            Self::Matchlock => WeaponDamage::Fixed {
                dice: DamageDice::new(3, 2),
                damage_type: DamageType::Piercing,
            },
            Self::MartiniHenryRifle => WeaponDamage::Fixed {
                dice: DamageDice::new(4, 0),
                damage_type: DamageType::Piercing,
            },
            Self::MauserModel1871 => WeaponDamage::Fixed {
                dice: DamageDice::new(4, 1),
                damage_type: DamageType::Piercing,
            },
            Self::MauserC96 => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 0),
                damage_type: DamageType::Piercing,
            },
            Self::MauserGewehr98 => WeaponDamage::Fixed {
                dice: DamageDice::new(4, 1),
                damage_type: DamageType::Piercing,
            },
            Self::MG34 => WeaponDamage::Fixed {
                dice: DamageDice::new(6, 0),
                damage_type: DamageType::Piercing,
            },
            Self::MG42 => WeaponDamage::Fixed {
                dice: DamageDice::new(6, 0),
                damage_type: DamageType::Piercing,
            },
            Self::MG3 => WeaponDamage::Fixed {
                dice: DamageDice::new(6, 0),
                damage_type: DamageType::Piercing,
            },
            Self::MaximGun => WeaponDamage::Fixed {
                dice: DamageDice::new(6, 0),
                damage_type: DamageType::Piercing,
            },
            Self::MachineGun => WeaponDamage::Fixed {
                dice: DamageDice::new(7, 0),
                damage_type: DamageType::Piercing,
            },
            Self::McMillanTAC50 => WeaponDamage::Fixed {
                dice: DamageDice::new(9, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Mitrailleuse => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Mk19 => WeaponDamage::Fixed {
                dice: DamageDice::new(6, 0),
                damage_type: DamageType::Crushing,
            },
            Self::MilkorMGL => WeaponDamage::Fixed {
                dice: DamageDice::new(6, 0),
                damage_type: DamageType::Crushing,
            },
            Self::MosinNagant => WeaponDamage::Fixed {
                dice: DamageDice::new(4, 1),
                damage_type: DamageType::Piercing,
            },
            Self::Mossberg500 => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 1),
                damage_type: DamageType::Piercing,
            },
            Self::MP18 => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 1),
                damage_type: DamageType::Piercing,
            },
            Self::MP40 => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::MP5 => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::MountainGun => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Crushing,
            },
            Self::MTS255 => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 1),
                damage_type: DamageType::Piercing,
            },
                        Self::Musket => WeaponDamage::Fixed {
                dice: DamageDice::new(4, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Musketoon => WeaponDamage::Fixed {
                dice: DamageDice::new(3, 2),
                damage_type: DamageType::Piercing,
            },
            Self::NavalGun => WeaponDamage::Fixed {
                dice: DamageDice::new(8, 0),
                damage_type: DamageType::Crushing,
            },
            Self::NegevNG7 => WeaponDamage::Fixed {
                dice: DamageDice::new(6, 0),
                damage_type: DamageType::Piercing,
            },
            Self::NordenfeltGun => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Pecheneg => WeaponDamage::Fixed {
                dice: DamageDice::new(6, 0),
                damage_type: DamageType::Piercing,
            },
            Self::PelletBow => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 4),
                damage_type: DamageType::Impaling,
            },
                        Self::Pistol => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
                        Self::PelletSling => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::Petronel => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::PepperboxRevolver => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 1),
                damage_type: DamageType::Piercing,
            },
            Self::PercussionRevolver => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Plumbata => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Impaling,
            },
            Self::PPS43 => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::PPSh41 => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::PKM => WeaponDamage::Fixed {
                dice: DamageDice::new(6, 0),
                damage_type: DamageType::Piercing,
            },
            Self::PSG1 => WeaponDamage::Fixed {
                dice: DamageDice::new(6, 0),
                damage_type: DamageType::Piercing,
            },
            Self::PGMHecateII => WeaponDamage::Fixed {
                dice: DamageDice::new(9, 0),
                damage_type: DamageType::Piercing,
            },
            Self::PlasmaRifle => WeaponDamage::Fixed {
                dice: DamageDice::new(8, 0),
                damage_type: DamageType::Crushing,
            },
            Self::PTRDAntiTankRifle => WeaponDamage::Fixed {
                dice: DamageDice::new(7, 1),
                damage_type: DamageType::Piercing,
            },
            Self::Prodd => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 2),
                damage_type: DamageType::Crushing,
            },
            Self::PumpActionShotgun => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 1),
                damage_type: DamageType::Piercing,
            },
            Self::QBZ95 => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Remington870 => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 1),
                damage_type: DamageType::Piercing,
            },
            Self::RemingtonRollingBlockPistol => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 0),
                damage_type: DamageType::Piercing,
            },
            Self::RemingtonRollingBlockRifle => WeaponDamage::Fixed {
                dice: DamageDice::new(4, 0),
                damage_type: DamageType::Piercing,
            },
            Self::RemingtonModel31 => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 1),
                damage_type: DamageType::Piercing,
            },
            Self::Revolver => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 0),
                damage_type: DamageType::Piercing,
            },
            Self::RepeatingCrossbow => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 1),
                damage_type: DamageType::Impaling,
            },
            Self::Rifle => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Rock => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
                        Self::RocketLauncher => WeaponDamage::Fixed {
                dice: DamageDice::new(6, 0),
                damage_type: DamageType::Crushing,
            },
            Self::RPG7 => WeaponDamage::Fixed {
                dice: DamageDice::new(8, 0),
                damage_type: DamageType::Crushing,
            },
            Self::Railgun => WeaponDamage::Fixed {
                dice: DamageDice::new(10, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Scorpion => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Impaling,
            },
            Self::RecurveBow => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 2),
                damage_type: DamageType::Impaling,
            },
            Self::Saiga12 => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 1),
                damage_type: DamageType::Piercing,
            },
            Self::SakoTRG42 => WeaponDamage::Fixed {
                dice: DamageDice::new(7, 0),
                damage_type: DamageType::Piercing,
            },
            Self::SelfBow => WeaponDamage::Fixed {
                dice: DamageDice::new(1, -1),
                damage_type: DamageType::Impaling,
            },
            Self::Shotgun => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 1),
                damage_type: DamageType::Piercing,
            },
                        Self::ShortBow => WeaponDamage::Fixed {
                dice: DamageDice::new(1, -1),
                damage_type: DamageType::Impaling,
            },
            Self::SharpsRifle => WeaponDamage::Fixed {
                dice: DamageDice::new(4, 0),
                damage_type: DamageType::Piercing,
            },
            Self::SiegeCrossbow => WeaponDamage::Fixed {
                dice: DamageDice::new(3, 6),
                damage_type: DamageType::Impaling,
            },
            Self::SIGP220 => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::SIGP320 => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::SingleActionRevolver => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 1),
                damage_type: DamageType::Piercing,
            },
            Self::Shuriken => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Impaling,
            },
            Self::Skorpion => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Sling => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Piercing,
            },
            Self::SmithWessonModel1 => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 0),
                damage_type: DamageType::Piercing,
            },
            Self::SmithWessonModel3 => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 0),
                damage_type: DamageType::Piercing,
            },
            Self::SMG => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::SniperRifle => WeaponDamage::Fixed {
                dice: DamageDice::new(7, 0),
                damage_type: DamageType::Piercing,
            },
            Self::SPAS12 => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 1),
                damage_type: DamageType::Piercing,
            },
            Self::SpencerRepeatingRifle => WeaponDamage::Fixed {
                dice: DamageDice::new(3, 2),
                damage_type: DamageType::Piercing,
            },
            Self::SpringfieldModel1861 => WeaponDamage::Fixed {
                dice: DamageDice::new(4, 0),
                damage_type: DamageType::Piercing,
            },
            Self::SpringfieldM1903 => WeaponDamage::Fixed {
                dice: DamageDice::new(4, 1),
                damage_type: DamageType::Piercing,
            },
            Self::TrapdoorSpringfield => WeaponDamage::Fixed {
                dice: DamageDice::new(4, 0),
                damage_type: DamageType::Piercing,
            },
            Self::StenGun => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 1),
                damage_type: DamageType::Piercing,
            },
            Self::Sterling => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::SteyrAUG => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::StingerMissile => WeaponDamage::Fixed {
                dice: DamageDice::new(12, 0),
                damage_type: DamageType::Crushing,
            },
                        Self::StaffSling => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Piercing,
            },
            Self::Stonebow => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 2),
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
            Self::ThompsonM1928 => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::Trebuchet => WeaponDamage::Fixed {
                dice: DamageDice::new(4, 0),
                damage_type: DamageType::Crushing,
            },
            Self::TNT => WeaponDamage::Fixed {
                dice: DamageDice::new(8, 0),
                damage_type: DamageType::Crushing,
            },
            Self::TOWMissile => WeaponDamage::Fixed {
                dice: DamageDice::new(12, 0),
                damage_type: DamageType::Crushing,
            },
            Self::Uzi => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::Ultimax100 => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::USAS12 => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 1),
                damage_type: DamageType::Piercing,
            },
            Self::VolcanicPistol => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 1),
                damage_type: DamageType::Piercing,
            },
            Self::VickersMachineGun => WeaponDamage::Fixed {
                dice: DamageDice::new(6, 0),
                damage_type: DamageType::Piercing,
            },
                        Self::Warbow => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 3),
                damage_type: DamageType::Impaling,
            },
            Self::WaltherPPK => WeaponDamage::Fixed {
                dice: DamageDice::new(2, -1),
                damage_type: DamageType::Piercing,
            },
            Self::WebleyRIC => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 0),
                damage_type: DamageType::Piercing,
            },
            Self::WebleyMkVI => WeaponDamage::Fixed {
                dice: DamageDice::new(2, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Wheellock => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 2),
                damage_type: DamageType::Piercing,
            },
            Self::Winchester1866 => WeaponDamage::Fixed {
                dice: DamageDice::new(3, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Winchester1873 => WeaponDamage::Fixed {
                dice: DamageDice::new(3, 1),
                damage_type: DamageType::Piercing,
            },
            Self::Winchester1887 => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 1),
                damage_type: DamageType::Piercing,
            },
            Self::Winchester1897 => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 1),
                damage_type: DamageType::Piercing,
            },
            Self::WinchesterRepeater => WeaponDamage::Fixed {
                dice: DamageDice::new(3, 1),
                damage_type: DamageType::Piercing,
            },
            Self::XM8 => WeaponDamage::Fixed {
                dice: DamageDice::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Yumi => WeaponDamage::Fixed {
                dice: DamageDice::new(1, 1),
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
            Self::AccuracyInternationalAWM => 6,
            Self::AA12 => 3,
            Self::Arquebus => 2,
            Self::AK47 => 4,
            Self::AKM => 4,
            Self::AK12 => 5,
            Self::ASVal => 4,
            Self::AssaultRifle => 5,
            Self::ArisakaType38 => 4,
            Self::AT4 => 3,
                        Self::Atlatl => 2,
            Self::Ballista => 5,
                        Self::Blowgun => 1,
                        Self::Blunderbuss => 1,
            Self::Bola => 1,
                        Self::Bow => 2,
            Self::BoysAntiTankRifle => 5,
            Self::BAR => 4,
            Self::BrenGun => 4,
            Self::BarrettM82 => 6,
            Self::BarrettM107 => 6,
            Self::BenelliM3 => 3,
            Self::BerettaM92 => 2,
            Self::BrowningHiPower => 2,
            Self::BrowningM1917 => 5,
            Self::BrowningM2 => 5,
            Self::BrowningAuto5 => 3,
                                    Self::Carbine => 4,
            Self::Caliver => 3,
            Self::CapLockPistol => 1,
            Self::Catapult => 1,
            Self::CompositeBow => 3,
                        Self::Chakram => 1,
            Self::ClaymoreMine => 1,
            Self::ChassepotRifle => 4,
            Self::CheyTacM200 => 7,
            Self::ChiappaRhino => 2,
            Self::Chukonu => 2,
            Self::ColtWalker => 1,
            Self::ColtM1911 => 2,
            Self::Coilgun => 6,
            Self::Crossbow => 4,
            Self::CZ75 => 2,
            Self::CZP10 => 2,
            Self::C4Explosive => 1,
            Self::Daikyu => 3,
            Self::Dart => 2,
            Self::Dynamite => 1,
            Self::DreyseNeedleRifle => 3,
                        Self::Derringer => 1,
            Self::DesertEagle => 2,
            Self::DoubleActionRevolver => 2,
            Self::DoubleBarrelShotgun => 3,
            Self::DragunovSVD => 6,
            Self::Espringal => 3,
            Self::EnfieldPattern1853 => 4,
                        Self::Flamethrower => 2,
            Self::FlareGun => 1,
            Self::FieldGun => 2,
            Self::Flintlock => 1,
            Self::FAMAS => 5,
            Self::FNFAL => 5,
            Self::FNMAG => 5,
            Self::FNP90 => 4,
            Self::FNSCARL => 5,
            Self::FNFiveSeveN => 2,
            Self::FNMinimiPara => 5,
            Self::FostechOrigin12 => 3,
            Self::Fusil => 3,
            Self::GatlingGun => 4,
            Self::GardnerGun => 4,
                        Self::Grenade => 1,
            Self::GreaseGunM3 => 3,
            Self::Hackbut => 2,
                        Self::Gastraphetes => 4,
            Self::GreatBow => 3,
            Self::Glock17 => 2,
            Self::Glock43X => 2,
            Self::G3 => 5,
            Self::GP25 => 3,
            Self::GaussRifle => 6,
            Self::HornBow => 3,
            Self::HandCannon => 1,
                        Self::Hankyu => 2,
            Self::HandMortar => 1,
            Self::HenryRifle => 4,
            Self::HandCrossbow => 3,
                        Self::HeavyCrossbow => 4,
            Self::Hurlbat => 1,
                        Self::HuntingRifle => 5,
            Self::HK416 => 5,
            Self::HKMP7 => 4,
            Self::HKMark23 => 3,
            Self::HKMG4 => 5,
            Self::Jezail => 4,
            Self::KelTecKSG => 3,
            Self::KrissVector => 3,
            Self::LematRevolver => 2,
            Self::LeeEnfieldSMLE => 4,
            Self::LeverActionRifle => 4,
            Self::LewisGun => 4,
            Self::LightCrossbow => 4,
                                    Self::Longbow => 3,
            Self::LugerP08 => 2,
            Self::L96A1 => 6,
            Self::LAWM72 => 3,
            Self::LaserRifle => 7,
            Self::MAC10 => 3,
            Self::M1Garand => 4,
            Self::MakarovPM => 2,
            Self::M14 => 5,
            Self::M16 => 5,
            Self::M203 => 3,
            Self::M21SWS => 6,
            Self::M249SAW => 5,
            Self::M4Carbine => 5,
            Self::M60 => 5,
            Self::M79 => 4,
            Self::Mangonel => 1,
            Self::Matchlock => 2,
            Self::MartiniHenryRifle => 4,
            Self::MauserModel1871 => 5,
            Self::MauserC96 => 2,
            Self::MauserGewehr98 => 5,
            Self::MG34 => 5,
            Self::MG42 => 5,
            Self::MG3 => 5,
            Self::MaximGun => 5,
            Self::MachineGun => 4,
            Self::McMillanTAC50 => 6,
            Self::Mitrailleuse => 3,
            Self::Mk19 => 4,
            Self::MilkorMGL => 4,
            Self::MosinNagant => 4,
            Self::Mossberg500 => 3,
            Self::MP18 => 2,
            Self::MP40 => 3,
            Self::MP5 => 4,
            Self::MountainGun => 2,
            Self::MTS255 => 3,
                        Self::Musket => 3,
            Self::Musketoon => 2,
            Self::NavalGun => 1,
            Self::NegevNG7 => 5,
            Self::NordenfeltGun => 3,
            Self::Pecheneg => 5,
            Self::PelletBow => 4,
                        Self::Pistol => 2,
                        Self::PelletSling => 0,
            Self::Petronel => 1,
            Self::PepperboxRevolver => 1,
            Self::PercussionRevolver => 2,
            Self::Plumbata => 0,
            Self::PPS43 => 3,
            Self::PPSh41 => 3,
            Self::PKM => 5,
            Self::PSG1 => 7,
            Self::PGMHecateII => 6,
            Self::PlasmaRifle => 5,
            Self::PTRDAntiTankRifle => 5,
            Self::Prodd => 3,
            Self::PumpActionShotgun => 3,
            Self::QBZ95 => 5,
            Self::Remington870 => 3,
            Self::RemingtonRollingBlockPistol => 2,
            Self::RemingtonRollingBlockRifle => 4,
            Self::RemingtonModel31 => 3,
            Self::Revolver => 2,
            Self::RepeatingCrossbow => 2,
            Self::Rifle => 5,
            Self::Rock => 0,
                        Self::RocketLauncher => 4,
            Self::RPG7 => 3,
            Self::Railgun => 7,
            Self::Scorpion => 4,
            Self::RecurveBow => 2,
            Self::Saiga12 => 3,
            Self::SakoTRG42 => 6,
            Self::SelfBow => 1,
            Self::Shotgun => 3,
                        Self::ShortBow => 1,
            Self::SharpsRifle => 4,
            Self::SiegeCrossbow => 5,
            Self::SIGP220 => 2,
            Self::SIGP320 => 2,
            Self::SingleActionRevolver => 2,
            Self::Shuriken => 1,
            Self::Skorpion => 2,
            Self::Sling => 0,
            Self::SmithWessonModel1 => 1,
            Self::SmithWessonModel3 => 2,
            Self::SMG => 4,
            Self::SniperRifle => 6,
            Self::SPAS12 => 3,
            Self::SpencerRepeatingRifle => 3,
            Self::SpringfieldModel1861 => 4,
            Self::SpringfieldM1903 => 5,
            Self::TrapdoorSpringfield => 4,
            Self::StenGun => 2,
            Self::Sterling => 3,
            Self::SteyrAUG => 5,
            Self::StingerMissile => 5,
                        Self::StaffSling => 1,
            Self::Stonebow => 3,
            Self::Boomerang => 1,
                        Self::ThrowingAxe => 2,
            Self::ThrowingHammer => 2,
                        Self::ThrowingKnife => 0,
            Self::ThrowingSpear => 2,
                        Self::ThrowingStick => 1,
            Self::ThompsonM1928 => 3,
            Self::Trebuchet => 1,
            Self::TNT => 1,
            Self::TOWMissile => 5,
            Self::Uzi => 3,
            Self::Ultimax100 => 5,
            Self::USAS12 => 3,
            Self::VolcanicPistol => 1,
            Self::VickersMachineGun => 5,
                        Self::Warbow => 3,
            Self::WaltherPPK => 2,
            Self::WebleyRIC => 2,
            Self::WebleyMkVI => 2,
            Self::Wheellock => 1,
            Self::Winchester1866 => 4,
            Self::Winchester1873 => 4,
            Self::Winchester1887 => 3,
            Self::Winchester1897 => 3,
            Self::WinchesterRepeater => 4,
            Self::XM8 => 5,
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
            Self::AccuracyInternationalAWM => Skill::Guns,
            Self::AA12 => Skill::Guns,
            Self::Arquebus => Skill::Guns,
            Self::AK47 => Skill::Guns,
            Self::AKM => Skill::Guns,
            Self::AK12 => Skill::Guns,
            Self::ASVal => Skill::Guns,
            Self::AssaultRifle => Skill::Guns,
            Self::ArisakaType38 => Skill::Guns,
            Self::AT4 => Skill::Guns,
                        Self::Atlatl => Skill::ThrownWeapon,
            Self::Ballista => Skill::Crossbow,
                        Self::Blowgun => Skill::Blowpipe,
                        Self::Blunderbuss => Skill::Guns,
            Self::Bola => Skill::ThrownWeapon,
                        Self::Bow => Skill::Bow,
            Self::BoysAntiTankRifle => Skill::Guns,
            Self::BAR => Skill::Guns,
            Self::BrenGun => Skill::Guns,
            Self::BarrettM82 => Skill::Guns,
            Self::BarrettM107 => Skill::Guns,
            Self::BenelliM3 => Skill::Guns,
            Self::BerettaM92 => Skill::Guns,
            Self::BrowningHiPower => Skill::Guns,
            Self::BrowningM1917 => Skill::Guns,
            Self::BrowningM2 => Skill::Guns,
            Self::BrowningAuto5 => Skill::Guns,
                                    Self::Carbine => Skill::Guns,
            Self::Caliver => Skill::Guns,
            Self::CapLockPistol => Skill::Guns,
            Self::Catapult => Skill::Artillery,
            Self::CompositeBow => Skill::Bow,
                        Self::Chakram => Skill::ThrownWeapon,
            Self::ClaymoreMine => Skill::Guns,
            Self::ChassepotRifle => Skill::Guns,
            Self::CheyTacM200 => Skill::Guns,
            Self::ChiappaRhino => Skill::Guns,
            Self::Chukonu => Skill::Crossbow,
            Self::ColtWalker => Skill::Guns,
            Self::ColtM1911 => Skill::Guns,
            Self::Coilgun => Skill::Guns,
            Self::Crossbow => Skill::Crossbow,
            Self::CZ75 => Skill::Guns,
            Self::CZP10 => Skill::Guns,
            Self::C4Explosive => Skill::Guns,
            Self::Daikyu => Skill::Bow,
            Self::Dart => Skill::ThrownWeapon,
            Self::Dynamite => Skill::ThrownWeapon,
            Self::DreyseNeedleRifle => Skill::Guns,
                        Self::Derringer => Skill::Guns,
            Self::DesertEagle => Skill::Guns,
            Self::DoubleActionRevolver => Skill::Guns,
            Self::DoubleBarrelShotgun => Skill::Guns,
            Self::DragunovSVD => Skill::Guns,
            Self::Espringal => Skill::Artillery,
            Self::EnfieldPattern1853 => Skill::Guns,
                        Self::Flamethrower => Skill::Guns,
            Self::FlareGun => Skill::Guns,
            Self::FieldGun => Skill::Artillery,
            Self::Flintlock => Skill::Guns,
            Self::FAMAS => Skill::Guns,
            Self::FNFAL => Skill::Guns,
            Self::FNMAG => Skill::Guns,
            Self::FNP90 => Skill::Guns,
            Self::FNSCARL => Skill::Guns,
            Self::FNFiveSeveN => Skill::Guns,
            Self::FNMinimiPara => Skill::Guns,
            Self::FostechOrigin12 => Skill::Guns,
            Self::Fusil => Skill::Guns,
            Self::GatlingGun => Skill::Guns,
            Self::GardnerGun => Skill::Guns,
                        Self::Grenade => Skill::ThrownWeapon,
            Self::GreaseGunM3 => Skill::Guns,
            Self::Hackbut => Skill::Guns,
                        Self::Gastraphetes => Skill::Crossbow,
            Self::GreatBow => Skill::Bow,
            Self::Glock17 => Skill::Guns,
            Self::Glock43X => Skill::Guns,
            Self::G3 => Skill::Guns,
            Self::GP25 => Skill::Guns,
            Self::GaussRifle => Skill::Guns,
            Self::HornBow => Skill::Bow,
            Self::HandCannon => Skill::Guns,
                        Self::Hankyu => Skill::Bow,
            Self::HandMortar => Skill::Artillery,
            Self::HenryRifle => Skill::Guns,
            Self::HandCrossbow => Skill::Crossbow,
                        Self::HeavyCrossbow => Skill::Crossbow,
            Self::Hurlbat => Skill::ThrownWeapon,
                        Self::HuntingRifle => Skill::Guns,
            Self::HK416 => Skill::Guns,
            Self::HKMP7 => Skill::Guns,
            Self::HKMark23 => Skill::Guns,
            Self::HKMG4 => Skill::Guns,
            Self::Jezail => Skill::Guns,
            Self::KelTecKSG => Skill::Guns,
            Self::KrissVector => Skill::Guns,
            Self::LematRevolver => Skill::Guns,
            Self::LeeEnfieldSMLE => Skill::Guns,
            Self::LeverActionRifle => Skill::Guns,
            Self::LewisGun => Skill::Guns,
            Self::LightCrossbow => Skill::Crossbow,
                                    Self::Longbow => Skill::Bow,
            Self::LugerP08 => Skill::Guns,
            Self::L96A1 => Skill::Guns,
            Self::LAWM72 => Skill::Guns,
            Self::LaserRifle => Skill::Guns,
            Self::MAC10 => Skill::Guns,
            Self::M1Garand => Skill::Guns,
            Self::MakarovPM => Skill::Guns,
            Self::M14 => Skill::Guns,
            Self::M16 => Skill::Guns,
            Self::M203 => Skill::Guns,
            Self::M21SWS => Skill::Guns,
            Self::M249SAW => Skill::Guns,
            Self::M4Carbine => Skill::Guns,
            Self::M60 => Skill::Guns,
            Self::M79 => Skill::Guns,
            Self::Mangonel => Skill::Artillery,
            Self::Matchlock => Skill::Guns,
            Self::MartiniHenryRifle => Skill::Guns,
            Self::MauserModel1871 => Skill::Guns,
            Self::MauserC96 => Skill::Guns,
            Self::MauserGewehr98 => Skill::Guns,
            Self::MG34 => Skill::Guns,
            Self::MG42 => Skill::Guns,
            Self::MG3 => Skill::Guns,
            Self::MaximGun => Skill::Guns,
            Self::MachineGun => Skill::Guns,
            Self::McMillanTAC50 => Skill::Guns,
            Self::Mitrailleuse => Skill::Guns,
            Self::Mk19 => Skill::Guns,
            Self::MilkorMGL => Skill::Guns,
            Self::MosinNagant => Skill::Guns,
            Self::Mossberg500 => Skill::Guns,
            Self::MP18 => Skill::Guns,
            Self::MP40 => Skill::Guns,
            Self::MP5 => Skill::Guns,
            Self::MountainGun => Skill::Artillery,
            Self::MTS255 => Skill::Guns,
                        Self::Musket => Skill::Guns,
            Self::Musketoon => Skill::Guns,
            Self::NavalGun => Skill::Artillery,
            Self::NegevNG7 => Skill::Guns,
            Self::NordenfeltGun => Skill::Guns,
            Self::Pecheneg => Skill::Guns,
            Self::PelletBow => Skill::Bow,
                        Self::Pistol => Skill::Guns,
                        Self::PelletSling => Skill::Sling,
            Self::Petronel => Skill::Guns,
            Self::PepperboxRevolver => Skill::Guns,
            Self::PercussionRevolver => Skill::Guns,
            Self::Plumbata => Skill::ThrownWeapon,
            Self::PPS43 => Skill::Guns,
            Self::PPSh41 => Skill::Guns,
            Self::PKM => Skill::Guns,
            Self::PSG1 => Skill::Guns,
            Self::PGMHecateII => Skill::Guns,
            Self::PlasmaRifle => Skill::Guns,
            Self::PTRDAntiTankRifle => Skill::Guns,
            Self::Prodd => Skill::Crossbow,
            Self::PumpActionShotgun => Skill::Guns,
            Self::QBZ95 => Skill::Guns,
            Self::Remington870 => Skill::Guns,
            Self::RemingtonRollingBlockPistol => Skill::Guns,
            Self::RemingtonRollingBlockRifle => Skill::Guns,
            Self::RemingtonModel31 => Skill::Guns,
            Self::Revolver => Skill::Guns,
            Self::RepeatingCrossbow => Skill::Crossbow,
            Self::Rifle => Skill::Guns,
            Self::Rock => Skill::ThrownWeapon,
                        Self::RocketLauncher => Skill::Guns,
            Self::RPG7 => Skill::Guns,
            Self::Railgun => Skill::Guns,
            Self::Scorpion => Skill::Artillery,
            Self::RecurveBow => Skill::Bow,
            Self::Saiga12 => Skill::Guns,
            Self::SakoTRG42 => Skill::Guns,
            Self::SelfBow => Skill::Bow,
            Self::Shotgun => Skill::Guns,
                        Self::ShortBow => Skill::Bow,
            Self::SharpsRifle => Skill::Guns,
            Self::SiegeCrossbow => Skill::Crossbow,
            Self::SIGP220 => Skill::Guns,
            Self::SIGP320 => Skill::Guns,
            Self::SingleActionRevolver => Skill::Guns,
            Self::Shuriken => Skill::ThrownWeapon,
            Self::Skorpion => Skill::Guns,
            Self::Sling => Skill::Sling,
            Self::SmithWessonModel1 => Skill::Guns,
            Self::SmithWessonModel3 => Skill::Guns,
            Self::SMG => Skill::Guns,
            Self::SniperRifle => Skill::Guns,
            Self::SPAS12 => Skill::Guns,
            Self::SpencerRepeatingRifle => Skill::Guns,
            Self::SpringfieldModel1861 => Skill::Guns,
            Self::SpringfieldM1903 => Skill::Guns,
            Self::TrapdoorSpringfield => Skill::Guns,
            Self::StenGun => Skill::Guns,
            Self::Sterling => Skill::Guns,
            Self::SteyrAUG => Skill::Guns,
            Self::StingerMissile => Skill::Guns,
                        Self::StaffSling => Skill::Sling,
            Self::Stonebow => Skill::Crossbow,
            Self::Boomerang => Skill::ThrownWeapon,
                        Self::ThrowingAxe => Skill::ThrownWeapon,
            Self::ThrowingHammer => Skill::ThrownWeapon,
                        Self::ThrowingKnife => Skill::ThrownWeapon,
            Self::ThrowingSpear => Skill::ThrownWeapon,
                        Self::ThrowingStick => Skill::ThrownWeapon,
            Self::ThompsonM1928 => Skill::Guns,
            Self::Trebuchet => Skill::Artillery,
            Self::TNT => Skill::ThrownWeapon,
            Self::TOWMissile => Skill::Guns,
            Self::Uzi => Skill::Guns,
            Self::Ultimax100 => Skill::Guns,
            Self::USAS12 => Skill::Guns,
            Self::VolcanicPistol => Skill::Guns,
            Self::VickersMachineGun => Skill::Guns,
                        Self::Warbow => Skill::Bow,
            Self::WaltherPPK => Skill::Guns,
            Self::WebleyRIC => Skill::Guns,
            Self::WebleyMkVI => Skill::Guns,
            Self::Wheellock => Skill::Guns,
            Self::Winchester1866 => Skill::Guns,
            Self::Winchester1873 => Skill::Guns,
            Self::Winchester1887 => Skill::Guns,
            Self::Winchester1897 => Skill::Guns,
            Self::WinchesterRepeater => Skill::Guns,
            Self::XM8 => Skill::Guns,
            Self::Yumi => Skill::Bow,
            Self::ThrownBola => Skill::ThrownWeapon,
            Self::ThrownNet => Skill::ThrownWeapon,
        }
    }
}
