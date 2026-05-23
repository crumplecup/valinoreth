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
    /// Angon, thrust+2 impaling, reach 1. LT (barbed javelin)
    Angon,
    /// Arming sword, swing+1 cutting, reach 1. LT (one-handed knight's sword)
    ArmingSword,
    /// Awl pike, thrust+3 impaling, reach 4-5. LT (heavy pike)
    AwlPike,
    /// Assegai, thrust+2 impaling, reach 1. LT (African throwing spear)
    Assegai,
    /// ASP baton, swing+1 crushing, reach 1. HT (Expandable collapsible baton, TL7+)
    AspBaton,
    /// Baton, swing crushing, reach 1. BS 271
    Baton,
    /// Battle axe, swing+3 cutting, reach 1. BS 271
    BattleAxe,
    /// Bayonet, thrust+1 impaling, reach 1. HT (Rifle-mounted blade, TL5+)
    Bayonet,
    /// Bearded axe, swing+2 cutting, reach 1. LT (Viking axe)
    BeardedAxe,
    /// Bec de corbin, swing+2 impaling, reach 2-3. LT (crow's beak)
    BecDeCorbin,
    /// Blackjack (sap), swing crushing, reach C. BS 271
    Blackjack,
    /// Bastard sword, swing+2 cutting, reach 1-2. BS 271
    BastardSword,
    /// Bagh nakh, thrust-1 impaling, reach C. LT (Indian claw weapon)
    BaghNakh,
    /// Bardiche, swing+3 cutting, reach 2-3. LT
    Bardiche,
    /// Bill, swing+3 cutting, reach 2-3. LT
    Bill,
    /// Billhook, swing+2 cutting, reach 2. LT
    Billhook,
    /// Bola, swing crushing, reach 1. BS 271
    Bola,
    /// Bo, swing+2 crushing, reach 1-2. LT (Japanese staff)
    Bo,
    /// Bowie knife, thrust impaling, reach C-1. HT (Large hunting/fighting knife, TL5+)
    BowieKnife,
    /// Brass knuckles, thrust crushing, reach C. BS 271
    BrassKnuckles,
    /// Bullwhip, swing-1 cutting, reach 1-7. HT (Long reach whip weapon, TL5+)
    Bullwhip,
    /// Broadsword, swing+1 cutting, reach 1. BS 271
    Broadsword,
    /// Chakram, swing cutting, reach 1. BS 271
    Chakram,
    /// Cestus, thrust crushing, reach C. BS 271
    Cestus,
    /// Cattle prod, thrust+1 crushing, reach 1. HT (Electric livestock prod, TL6+)
    CattleProd,
    /// Ceramic knife, thrust impaling, reach C. HT (Non-metallic blade, TL8)
    CeramicKnife,
    /// Chain, swing+1 crushing, reach 1-3. BS 271
    Chain,
    /// Chainsaw, swing+4 cutting, reach 1. HT (Improvised cutting tool, TL6+)
    Chainsaw,
    /// Cinquedea, thrust impaling, reach C. LT (Italian thick dagger)
    Cinquedea,
    /// Claymore, swing+2 cutting, reach 1-2. BS 271
    Claymore,
    /// Club, swing+1 crushing, reach 1. BS 271
    Club,
    /// Combat net, swing-2 crushing, reach 1. BS 271
    CombatNet,
    /// Combat knife, thrust impaling, reach C. HT (Military fighting knife, TL6+)
    CombatKnife,
    /// Crowbar, swing+2 crushing, reach 1. BS 289
    Crowbar,
    /// Cutlass, swing+1 cutting, reach 1. BS 271
    Cutlass,
    /// Dagger, thrust-1 impaling, reach C. BS 271
    Dagger,
    /// Dao, swing+1 cutting, reach 1. LT (Chinese broadsword)
    Dao,
    /// Dirk, thrust impaling, reach C. LT (Scottish long dagger)
    Dirk,
    /// Estoc, thrust+2 impaling, reach 1. BS 271
    Estoc,
    /// Electric knuckles, thrust+1 crushing, reach C. HT (Shock weapon, TL8)
    ElectricKnuckles,
    /// Eku, swing+2 crushing, reach 1-2. LT (Okinawan oar weapon)
    Eku,
    /// Entrenching tool, swing+2 cutting, reach 1. HT (Military shovel as weapon, TL5+)
    EntrenchingTool,
    /// Falchion, swing+2 cutting, reach 1. BS 271
    Falchion,
    /// Falx, swing+3 cutting, reach 1. LT (Dacian curved blade)
    Falx,
    /// Fairbairn-Sykes fighting knife, thrust+1 impaling, reach C. HT (WWII British commando dagger, TL6)
    FairbairnSykes,
    /// Fauchard, swing+3 cutting, reach 2-3. LT
    Fauchard,
    /// Fist, thrust-1 crushing, reach C. BS 271
    Fist,
    /// Francisca, swing+2 cutting, reach 1. LT (Frankish throwing axe)
    Francisca,
    /// Flail, swing+2 crushing, reach 1-2. BS 271
    Flail,
    /// Gladius, swing cutting, reach 1. BS 271
    Gladius,
    /// Grain flail, swing+2 crushing, reach 1-2. LT (agricultural flail as weapon)
    GrainFlail,
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
    /// Horseman's mace, swing+2 crushing, reach 1. LT (cavalry weapon)
    HorsemansMace,
    /// Horseman's pick, swing+1 impaling, reach 1. LT (cavalry weapon)
    HorsemansPick,
    /// Hunga munga, swing+1 cutting, reach 1. LT (African multi-blade throwing weapon)
    HungaMunga,
    /// Iklwa, thrust+1 impaling, reach 1. LT (Zulu short stabbing spear)
    Iklwa,
    /// Ice pick, thrust+1 impaling, reach C. HT (Climbing tool/improvised weapon, TL5+)
    IcePick,
    /// Javelin, thrust+1 impaling, reach 1. BS 271
    Javelin,
    /// Jitte, thrust-1 impaling, reach 1. BS 271
    Jitte,
    /// Jian, swing cutting, reach 1. LT (Chinese straight sword)
    Jian,
    /// Jo, swing+1 crushing, reach 1. LT (Japanese short staff)
    Jo,
    /// Ka-Bar, thrust+1 impaling, reach C-1. HT (USMC combat knife, TL6+)
    KaBar,
    /// Kama, swing cutting, reach 1. LT (Japanese sickle weapon)
    Kama,
    /// Katana, swing+1 cutting, reach 1. BS 271
    Katana,
    /// Katar, thrust+1 impaling, reach C. BS 271
    Katar,
    /// Khanda, swing+1 cutting, reach 1. LT (Indian straight sword)
    Khanda,
    /// Kilij, swing+1 cutting, reach 1. LT (Turkish saber)
    Kilij,
    /// Kick, thrust crushing, reach C-1. BS 271
    Kick,
    /// Knife, thrust-1 impaling, reach C. BS 271
    Knife,
    /// Kopesh, swing+1 cutting, reach 1. BS 271
    Kopesh,
    /// Kopis, swing+1 cutting, reach 1. LT (Greek chopping sword)
    Kopis,
    /// Konda, swing cutting, reach 1. LT (African throwing knife)
    Konda,
    /// Kris, thrust impaling, reach C. LT (Indonesian wavy dagger)
    Kris,
    /// Kukri, swing cutting, reach C-1. HT (Gurkha curved knife, TL5+)
    Kukri,
    /// Kusarigama, swing+2 cutting, reach 1-3. LT (chain-sickle)
    Kusarigama,
    /// Kubotan, thrust crushing, reach C. HT (Self-defense keychain weapon, TL7+)
    Kubotan,
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
    /// Lucerne hammer, swing+2 impaling, reach 2-3. LT (pole hammer)
    LucerneHammer,
    /// Mace, swing+2 crushing, reach 1. BS 271
    Mace,
    /// Macuahuitl, swing+3 cutting, reach 1. LT (Aztec obsidian-edged club)
    Macuahuitl,
    /// Machete, swing+1 cutting, reach 1. HT (Utility blade/tool as weapon, TL5+)
    Machete,
    /// Main gauche, thrust-1 impaling, reach 1. BS 271
    MainGauche,
    /// Maul, swing+4 crushing, reach 1-2. BS 271
    Maul,
    /// Messer, swing+1 cutting, reach 1. LT (German knife-sword)
    Messer,
    /// Mattock, swing+2 impaling, reach 1. BS 289
    Mattock,
    /// Military flail, swing+3 crushing, reach 1-2. LT (heavy war flail)
    MilitaryFlail,
    /// Morningstar, swing+3 crushing, reach 1. BS 271
    Morningstar,
    /// Monowire, swing+2 cutting, reach C. HT (Ultra-thin molecular cutting wire, TL8)
    Monowire,
    /// Naginata, swing+2 cutting, reach 2-3. BS 271
    Naginata,
    /// Nunchaku, swing+1 crushing, reach 1. BS 271
    Nunchaku,
    /// Nightstick, swing+1 crushing, reach 1. HT (Police baton/billy club, TL6+)
    Nightstick,
    /// Nunti bo, thrust+2 impaling, reach 1-2. LT (Japanese spear-staff)
    NuntiBo,
    /// Pick, swing+2 impaling, reach 1. BS 271
    Pick,
    /// Pitchfork, thrust+1 impaling, reach 1. LT (agricultural weapon)
    Pitchfork,
    /// Pike, thrust+2 impaling, reach 4-5. BS 271
    Pike,
    /// Pilum, thrust+2 impaling, reach 1. LT (Roman javelin)
    Pilum,
    /// Poleax, swing+3 cutting, reach 2-3. BS 271
    Poleax,
    /// Partisan, thrust+2 impaling, reach 2-3. LT
    Partisan,
    /// Push dagger, thrust impaling, reach C. HT (T-grip concealment dagger, TL5+)
    PushDagger,
    /// Quarterstaff, swing+2 crushing, reach 1-2. BS 271
    Quarterstaff,
    /// Rapier, thrust+1 impaling, reach 1. BS 271
    Rapier,
    /// Ranseur, thrust+2 impaling, reach 2-3. LT (pole weapon with side blades)
    Ranseur,
    /// Saber, swing+1 cutting, reach 1. BS 271
    Saber,
    /// Sasumata, swing+1 crushing, reach 2-3. LT (Japanese man-catcher pole)
    Sasumata,
    /// Seax, swing cutting, reach 1. LT (Germanic short sword)
    Seax,
    /// Shamshir, swing+1 cutting, reach 1. LT (Persian curved sword)
    Shamshir,
    /// Sai, thrust-1 impaling, reach C. BS 271
    Sai,
    /// Sap gloves, swing crushing, reach C. HT (Weighted striking gloves, TL6+)
    SapGloves,
    /// Scimitar, swing+1 cutting, reach 1. BS 271
    Scimitar,
    /// Shortsword, swing cutting, reach 1. BS 271
    Shortsword,
    /// Shovel, swing+2 crushing, reach 1. BS 289
    Shovel,
    /// Scythe, swing+2 cutting, reach 1. LT (agricultural weapon)
    Scythe,
    /// Sickle, swing cutting, reach C-1. LT (agricultural weapon)
    Sickle,
    /// Smallsword, thrust impaling, reach 1. BS 271
    Smallsword,
    /// Sodegarami, swing+1 crushing, reach 2-3. LT (Japanese man-catcher pole)
    Sodegarami,
    /// Spear, thrust+2 impaling, reach 1-2. BS 271
    Spear,
    /// Spetum, thrust+2 impaling, reach 2-3. LT (Medieval pole weapon)
    Spetum,
    /// Spiked club, swing+2 crushing, reach 1. LT (war club)
    SpikedClub,
    /// Staff, swing+1 crushing, reach 1-2. BS 271
    Staff,
    /// Stick (walking stick/cane), swing crushing, reach 1. BS 271
    Stick,
    /// Stiletto, thrust+1 impaling, reach C. LT (Italian piercing dagger)
    Stiletto,
    /// Stun baton, swing crushing, reach 1. HT (Electric shock weapon, TL8)
    StunBaton,
    /// Survival knife, thrust+1 impaling, reach C-1. HT (Large fixed-blade survival knife, TL6+)
    SurvivalKnife,
    /// Tanto, thrust impaling, reach C. BS 271
    Tanto,
    /// Tactical folder, thrust impaling, reach C. HT (Modern folding knife, TL7+)
    TacticalFolder,
    /// Tactical pen, thrust impaling, reach C. HT (Concealed defensive tool, TL8)
    TacticalPen,
    /// Tactical tomahawk, swing+2 cutting, reach 1. HT (Modern combat axe, TL8)
    TacticalTomahawk,
    /// Tekko, thrust-1 crushing, reach C. LT (Okinawan knuckle weapon)
    Tekko,
    /// Talwar, swing+1 cutting, reach 1. LT (Indian curved sword)
    Talwar,
    /// Telescoping baton, swing+1 crushing, reach 1. HT (Collapsible baton, TL7+)
    TelescopingBaton,
    /// Tetsubo, swing+3 crushing, reach 1-2. LT (Japanese war club)
    Tetsubo,
    /// Tessen, swing crushing, reach C. LT (Japanese iron fan)
    Tessen,
    /// Three-section staff, swing+2 crushing, reach 1-2. LT (Chinese weapon)
    ThreeSectionStaff,
    /// Throwing knife, swing cutting, reach 1. HT (Balanced knife for throwing, TL5+)
    ThrowingKnife,
    /// Tonfa, swing+1 crushing, reach 1. BS 271
    Tonfa,
    /// Trench knife, thrust impaling, reach C. HT (WWI knuckle-duster knife, TL6)
    TrenchKnife,
    /// Trident, thrust+2 impaling, reach 1-2. BS 271
    Trident,
    /// Two-handed sword, swing+2 cutting, reach 1-2. BS 271
    TwoHandedSword,
    /// Wakizashi, swing cutting, reach 1. BS 271
    Wakizashi,
    /// Voulge, swing+3 cutting, reach 2-3. LT
    Voulge,
    /// Urumi, swing+1 cutting, reach 1-3. LT (Indian whip sword)
    Urumi,
    /// Vibro-knife, swing+2 cutting, reach 1. HT (Vibrating blade for enhanced cutting, TL8)
    VibroKnife,
    /// War fan, swing crushing, reach C. BS 271
    WarFan,
    /// War scythe, swing+3 cutting, reach 2-3. LT (Pole weapon with straight blade)
    WarScythe,
    /// Warhammer, swing+3 impaling, reach 1-2. BS 271
    Warhammer,
    /// Whip, swing-1 cutting, reach 1-2. BS 271
    Whip,
    /// Wire saw, swing+1 cutting, reach C. HT (Survival/combat garrote tool, TL6+)
    WireSaw,
    /// Xiphos, swing cutting, reach 1. LT (Greek short sword)
    Xiphos,
}

impl MeleeWeapon {
    /// Returns base cost for this melee weapon.
    #[instrument]
    pub fn base_cost(&self) -> Currency {
        debug!("Getting melee weapon base cost");
        match self {
            Self::Axe => Currency::dollars(50.0),
            Self::Angon => Currency::dollars(40.0),
            Self::ArmingSword => Currency::dollars(400.0),
            Self::AwlPike => Currency::dollars(100.0),
            Self::Assegai => Currency::dollars(30.0),
            Self::AspBaton => Currency::dollars(60.0),
            Self::Baton => Currency::dollars(20.0),
            Self::BattleAxe => Currency::dollars(50.0),
            Self::Bayonet => Currency::dollars(20.0),
            Self::BeardedAxe => Currency::dollars(60.0),
            Self::BecDeCorbin => Currency::dollars(150.0),
            Self::BastardSword => Currency::dollars(650.0),
            Self::BaghNakh => Currency::dollars(25.0),
            Self::Bardiche => Currency::dollars(120.0),
            Self::Bill => Currency::dollars(100.0),
            Self::Billhook => Currency::dollars(40.0),
            Self::Blackjack => Currency::dollars(20.0),
            Self::Bola => Currency::dollars(20.0),
            Self::Bo => Currency::dollars(10.0),
            Self::BowieKnife => Currency::dollars(40.0),
            Self::BrassKnuckles => Currency::dollars(10.0),
            Self::Bullwhip => Currency::dollars(20.0),
            Self::Broadsword => Currency::dollars(500.0),
            Self::Chakram => Currency::dollars(15.0),
            Self::Cestus => Currency::dollars(15.0),
            Self::CattleProd => Currency::dollars(100.0),
            Self::CeramicKnife => Currency::dollars(90.0),
            Self::Chain => Currency::dollars(30.0),
            Self::Chainsaw => Currency::dollars(200.0),
            Self::Cinquedea => Currency::dollars(30.0),
            Self::Claymore => Currency::dollars(500.0),
            Self::Club => Currency::dollars(10.0),
            Self::CombatNet => Currency::dollars(40.0),
            Self::CombatKnife => Currency::dollars(50.0),
            Self::Crowbar => Currency::dollars(20.0),
            Self::Cutlass => Currency::dollars(400.0),
            Self::Dagger => Currency::dollars(20.0),
            Self::Dao => Currency::dollars(400.0),
            Self::Dirk => Currency::dollars(30.0),
            Self::Estoc => Currency::dollars(600.0),
            Self::ElectricKnuckles => Currency::dollars(150.0),
            Self::Eku => Currency::dollars(50.0),
            Self::EntrenchingTool => Currency::dollars(25.0),
            Self::Falchion => Currency::dollars(400.0),
            Self::Falx => Currency::dollars(350.0),
            Self::FairbairnSykes => Currency::dollars(75.0),
            Self::Fauchard => Currency::dollars(90.0),
            Self::Fist => Currency::dollars(0.0),
            Self::Francisca => Currency::dollars(50.0),
            Self::Flail => Currency::dollars(60.0),
            Self::Gladius => Currency::dollars(200.0),
            Self::GrainFlail => Currency::dollars(25.0),
            Self::Garrote => Currency::dollars(15.0),
            Self::Glaive => Currency::dollars(100.0),
            Self::Guisarme => Currency::dollars(100.0),
            Self::GreatAxe => Currency::dollars(100.0),
            Self::Halberd => Currency::dollars(150.0),
            Self::Hatchet => Currency::dollars(40.0),
            Self::HorsemansMace => Currency::dollars(60.0),
            Self::HorsemansPick => Currency::dollars(70.0),
            Self::HungaMunga => Currency::dollars(60.0),
            Self::Iklwa => Currency::dollars(40.0),
            Self::IcePick => Currency::dollars(15.0),
            Self::Javelin => Currency::dollars(30.0),
            Self::Jitte => Currency::dollars(50.0),
            Self::Jian => Currency::dollars(500.0),
            Self::Jo => Currency::dollars(5.0),
            Self::KaBar => Currency::dollars(60.0),
            Self::Kama => Currency::dollars(40.0),
            Self::Katana => Currency::dollars(650.0),
            Self::Katar => Currency::dollars(40.0),
            Self::Khanda => Currency::dollars(500.0),
            Self::Kilij => Currency::dollars(500.0),
            Self::Kick => Currency::dollars(0.0),
            Self::Knife => Currency::dollars(40.0),
            Self::Kopesh => Currency::dollars(200.0),
            Self::Kopis => Currency::dollars(300.0),
            Self::Konda => Currency::dollars(50.0),
            Self::Kris => Currency::dollars(40.0),
            Self::Kukri => Currency::dollars(35.0),
            Self::Kusarigama => Currency::dollars(250.0),
            Self::Kubotan => Currency::dollars(10.0),
            Self::Kusari => Currency::dollars(70.0),
            Self::LargeKnife => Currency::dollars(60.0),
            Self::Lasso => Currency::dollars(20.0),
            Self::Lance => Currency::dollars(60.0),
            Self::LongSpear => Currency::dollars(60.0),
            Self::Longsword => Currency::dollars(500.0),
            Self::LucerneHammer => Currency::dollars(150.0),
            Self::Mace => Currency::dollars(50.0),
            Self::Macuahuitl => Currency::dollars(40.0),
            Self::Machete => Currency::dollars(30.0),
            Self::MainGauche => Currency::dollars(50.0),
            Self::Maul => Currency::dollars(80.0),
            Self::Messer => Currency::dollars(350.0),
            Self::Mattock => Currency::dollars(25.0),
            Self::MilitaryFlail => Currency::dollars(100.0),
            Self::Morningstar => Currency::dollars(80.0),
            Self::Monowire => Currency::dollars(500.0),
            Self::Naginata => Currency::dollars(100.0),
            Self::Nightstick => Currency::dollars(30.0),
            Self::Nunchaku => Currency::dollars(20.0),
            Self::NuntiBo => Currency::dollars(70.0),
            Self::Pick => Currency::dollars(70.0),
            Self::Pitchfork => Currency::dollars(15.0),
            Self::Pike => Currency::dollars(80.0),
            Self::Pilum => Currency::dollars(40.0),
            Self::Poleax => Currency::dollars(120.0),
            Self::Partisan => Currency::dollars(100.0),
            Self::PushDagger => Currency::dollars(45.0),
            Self::Quarterstaff => Currency::dollars(10.0),
            Self::Rapier => Currency::dollars(500.0),
            Self::Ranseur => Currency::dollars(90.0),
            Self::Saber => Currency::dollars(500.0),
            Self::Sasumata => Currency::dollars(80.0),
            Self::Seax => Currency::dollars(300.0),
            Self::Shamshir => Currency::dollars(500.0),
            Self::Sai => Currency::dollars(20.0),
            Self::SapGloves => Currency::dollars(50.0),
            Self::Scimitar => Currency::dollars(500.0),
            Self::Shortsword => Currency::dollars(400.0),
            Self::Shovel => Currency::dollars(15.0),
            Self::Scythe => Currency::dollars(15.0),
            Self::Sickle => Currency::dollars(15.0),
            Self::Smallsword => Currency::dollars(400.0),
            Self::Sodegarami => Currency::dollars(80.0),
            Self::Spear => Currency::dollars(40.0),
            Self::Spetum => Currency::dollars(90.0),
            Self::SpikedClub => Currency::dollars(20.0),
            Self::Staff => Currency::dollars(5.0),
            Self::Stick => Currency::dollars(5.0),
            Self::Stiletto => Currency::dollars(50.0),
            Self::StunBaton => Currency::dollars(150.0),
            Self::SurvivalKnife => Currency::dollars(70.0),
            Self::Tanto => Currency::dollars(30.0),
            Self::TacticalFolder => Currency::dollars(55.0),
            Self::TacticalPen => Currency::dollars(30.0),
            Self::TacticalTomahawk => Currency::dollars(80.0),
            Self::Tekko => Currency::dollars(15.0),
            Self::Talwar => Currency::dollars(500.0),
            Self::TelescopingBaton => Currency::dollars(75.0),
            Self::Tetsubo => Currency::dollars(80.0),
            Self::Tessen => Currency::dollars(60.0),
            Self::ThreeSectionStaff => Currency::dollars(35.0),
            Self::ThrowingKnife => Currency::dollars(25.0),
            Self::Tonfa => Currency::dollars(20.0),
            Self::TrenchKnife => Currency::dollars(30.0),
            Self::Trident => Currency::dollars(100.0),
            Self::TwoHandedSword => Currency::dollars(900.0),
            Self::Wakizashi => Currency::dollars(400.0),
            Self::Voulge => Currency::dollars(110.0),
            Self::Urumi => Currency::dollars(200.0),
            Self::VibroKnife => Currency::dollars(200.0),
            Self::WarFan => Currency::dollars(50.0),
            Self::WarScythe => Currency::dollars(150.0),
            Self::Warhammer => Currency::dollars(100.0),
            Self::Whip => Currency::dollars(20.0),
            Self::WireSaw => Currency::dollars(15.0),
            Self::Xiphos => Currency::dollars(250.0),
        }
    }

    /// Returns weight for this melee weapon.
    #[instrument]
    pub fn weight(&self) -> Weight {
        debug!("Getting melee weapon weight");
        match self {
            Self::Axe => Weight::pounds(4.0),
            Self::Angon => Weight::pounds(3.0),
            Self::ArmingSword => Weight::pounds(2.5),
            Self::AwlPike => Weight::pounds(15.0),
            Self::Assegai => Weight::pounds(2.0),
            Self::AspBaton => Weight::pounds(1.0),
            Self::Baton => Weight::pounds(1.0),
            Self::BattleAxe => Weight::pounds(6.0),
            Self::Bayonet => Weight::pounds(1.0),
            Self::BeardedAxe => Weight::pounds(5.0),
            Self::BecDeCorbin => Weight::pounds(9.0),
            Self::Blackjack => Weight::pounds(1.0),
            Self::BastardSword => Weight::pounds(5.0),
            Self::BaghNakh => Weight::pounds(0.5),
            Self::Bardiche => Weight::pounds(10.0),
            Self::Bill => Weight::pounds(9.0),
            Self::Billhook => Weight::pounds(5.0),
            Self::Bola => Weight::pounds(1.0),
            Self::Bo => Weight::pounds(5.0),
            Self::BowieKnife => Weight::pounds(1.5),
            Self::BrassKnuckles => Weight::pounds(0.25),
            Self::Bullwhip => Weight::pounds(2.0),
            Self::Broadsword => Weight::pounds(3.0),
            Self::Chakram => Weight::pounds(0.5),
            Self::Cestus => Weight::pounds(0.5),
            Self::CattleProd => Weight::pounds(2.0),
            Self::CeramicKnife => Weight::pounds(0.5),
            Self::Chain => Weight::pounds(3.0),
            Self::Chainsaw => Weight::pounds(10.0),
            Self::Cinquedea => Weight::pounds(1.0),
            Self::Claymore => Weight::pounds(7.0),
            Self::Club => Weight::pounds(3.0),
            Self::CombatNet => Weight::pounds(5.0),
            Self::CombatKnife => Weight::pounds(1.0),
            Self::Crowbar => Weight::pounds(3.0),
            Self::Cutlass => Weight::pounds(2.0),
            Self::Dagger => Weight::pounds(0.25),
            Self::Dao => Weight::pounds(3.0),
            Self::Dirk => Weight::pounds(0.75),
            Self::Estoc => Weight::pounds(3.5),
            Self::ElectricKnuckles => Weight::pounds(0.5),
            Self::Eku => Weight::pounds(6.0),
            Self::EntrenchingTool => Weight::pounds(2.5),
            Self::Falchion => Weight::pounds(3.5),
            Self::Falx => Weight::pounds(4.0),
            Self::FairbairnSykes => Weight::pounds(0.75),
            Self::Fauchard => Weight::pounds(8.0),
            Self::Fist => Weight::pounds(0.0),
            Self::Francisca => Weight::pounds(3.0),
            Self::Flail => Weight::pounds(8.0),
            Self::Gladius => Weight::pounds(2.0),
            Self::GrainFlail => Weight::pounds(8.0),
            Self::Garrote => Weight::pounds(0.25),
            Self::Glaive => Weight::pounds(8.0),
            Self::Guisarme => Weight::pounds(9.0),
            Self::GreatAxe => Weight::pounds(8.0),
            Self::Halberd => Weight::pounds(12.0),
            Self::Hatchet => Weight::pounds(2.0),
            Self::HorsemansMace => Weight::pounds(3.0),
            Self::HorsemansPick => Weight::pounds(2.5),
            Self::HungaMunga => Weight::pounds(3.0),
            Self::Iklwa => Weight::pounds(2.0),
            Self::IcePick => Weight::pounds(0.5),
            Self::Javelin => Weight::pounds(2.0),
            Self::Jitte => Weight::pounds(1.5),
            Self::Jian => Weight::pounds(2.5),
            Self::Jo => Weight::pounds(2.0),
            Self::KaBar => Weight::pounds(1.0),
            Self::Kama => Weight::pounds(1.5),
            Self::Katana => Weight::pounds(2.5),
            Self::Katar => Weight::pounds(1.0),
            Self::Khanda => Weight::pounds(3.0),
            Self::Kilij => Weight::pounds(2.5),
            Self::Kick => Weight::pounds(0.0),
            Self::Knife => Weight::pounds(1.0),
            Self::Kopesh => Weight::pounds(3.5),
            Self::Kopis => Weight::pounds(2.5),
            Self::Konda => Weight::pounds(1.5),
            Self::Kris => Weight::pounds(1.0),
            Self::Kukri => Weight::pounds(1.5),
            Self::Kusarigama => Weight::pounds(3.0),
            Self::Kubotan => Weight::pounds(0.1),
            Self::Kusari => Weight::pounds(5.0),
            Self::LargeKnife => Weight::pounds(1.5),
            Self::Lasso => Weight::pounds(3.0),
            Self::Lance => Weight::pounds(6.0),
            Self::LongSpear => Weight::pounds(5.0),
            Self::Longsword => Weight::pounds(3.0),
            Self::LucerneHammer => Weight::pounds(8.0),
            Self::Mace => Weight::pounds(5.0),
            Self::Macuahuitl => Weight::pounds(4.0),
            Self::Machete => Weight::pounds(2.0),
            Self::MainGauche => Weight::pounds(1.25),
            Self::Maul => Weight::pounds(12.0),
            Self::Messer => Weight::pounds(2.5),
            Self::Mattock => Weight::pounds(6.0),
            Self::MilitaryFlail => Weight::pounds(10.0),
            Self::Morningstar => Weight::pounds(6.0),
            Self::Monowire => Weight::pounds(0.25),
            Self::Naginata => Weight::pounds(9.0),
            Self::Nightstick => Weight::pounds(1.5),
            Self::Nunchaku => Weight::pounds(1.5),
            Self::NuntiBo => Weight::pounds(5.0),
            Self::Pick => Weight::pounds(3.0),
            Self::Pitchfork => Weight::pounds(4.0),
            Self::Pike => Weight::pounds(13.0),
            Self::Pilum => Weight::pounds(4.0),
            Self::Poleax => Weight::pounds(10.0),
            Self::Partisan => Weight::pounds(7.0),
            Self::PushDagger => Weight::pounds(0.75),
            Self::Quarterstaff => Weight::pounds(4.0),
            Self::Rapier => Weight::pounds(2.75),
            Self::Ranseur => Weight::pounds(7.0),
            Self::Saber => Weight::pounds(2.0),
            Self::Sasumata => Weight::pounds(6.0),
            Self::Seax => Weight::pounds(2.0),
            Self::Shamshir => Weight::pounds(2.5),
            Self::Sai => Weight::pounds(1.0),
            Self::SapGloves => Weight::pounds(0.5),
            Self::Scimitar => Weight::pounds(3.0),
            Self::Shortsword => Weight::pounds(2.0),
            Self::Shovel => Weight::pounds(6.0),
            Self::Scythe => Weight::pounds(5.0),
            Self::Sickle => Weight::pounds(2.0),
            Self::Smallsword => Weight::pounds(1.5),
            Self::Sodegarami => Weight::pounds(6.0),
            Self::Spear => Weight::pounds(4.0),
            Self::Spetum => Weight::pounds(7.0),
            Self::SpikedClub => Weight::pounds(4.0),
            Self::Staff => Weight::pounds(4.0),
            Self::Stick => Weight::pounds(2.0),
            Self::Stiletto => Weight::pounds(0.5),
            Self::StunBaton => Weight::pounds(1.5),
            Self::SurvivalKnife => Weight::pounds(1.5),
            Self::Tanto => Weight::pounds(0.5),
            Self::TacticalFolder => Weight::pounds(0.5),
            Self::TacticalPen => Weight::pounds(0.1),
            Self::TacticalTomahawk => Weight::pounds(2.5),
            Self::Tekko => Weight::pounds(0.5),
            Self::Talwar => Weight::pounds(3.0),
            Self::TelescopingBaton => Weight::pounds(1.5),
            Self::Tetsubo => Weight::pounds(10.0),
            Self::Tessen => Weight::pounds(1.0),
            Self::ThreeSectionStaff => Weight::pounds(4.0),
            Self::ThrowingKnife => Weight::pounds(0.5),
            Self::Tonfa => Weight::pounds(1.5),
            Self::TrenchKnife => Weight::pounds(1.0),
            Self::Trident => Weight::pounds(4.0),
            Self::TwoHandedSword => Weight::pounds(7.0),
            Self::Wakizashi => Weight::pounds(1.5),
            Self::Voulge => Weight::pounds(10.0),
            Self::Urumi => Weight::pounds(2.0),
            Self::VibroKnife => Weight::pounds(1.0),
            Self::WarFan => Weight::pounds(1.0),
            Self::WarScythe => Weight::pounds(8.0),
            Self::Warhammer => Weight::pounds(7.0),
            Self::Whip => Weight::pounds(2.0),
            Self::WireSaw => Weight::pounds(0.1),
            Self::Xiphos => Weight::pounds(2.0),
        }
    }

    /// Returns tech level for this melee weapon.
    #[instrument]
    pub fn tech_level(&self) -> TechLevel {
        debug!("Getting melee weapon tech level");
        match self {
            Self::Axe => TechLevel::new(0),         // Stone Age
            Self::Angon => TechLevel::new(1),       // Iron Age
            Self::ArmingSword => TechLevel::new(3), // Medieval
            Self::AwlPike => TechLevel::new(3),     // Medieval
            Self::Assegai => TechLevel::new(0),     // Stone Age
            Self::AspBaton => TechLevel::new(7),    // Nuclear Age
            Self::Baton => TechLevel::new(5),       // Modern (police baton)
            Self::BattleAxe => TechLevel::new(1),   // Bronze/Iron Age
            Self::Bayonet => TechLevel::new(5),     // Industrial Revolution
            Self::BeardedAxe => TechLevel::new(2),  // Iron Age
            Self::BecDeCorbin => TechLevel::new(3), // Medieval
            Self::Blackjack => TechLevel::new(1),
            Self::BastardSword => TechLevel::new(2), // Medieval
            Self::BaghNakh => TechLevel::new(1),     // Ancient India
            Self::Bardiche => TechLevel::new(3),     // Medieval
            Self::Bill => TechLevel::new(3),         // Medieval
            Self::Billhook => TechLevel::new(2),     // Medieval
            Self::Bola => TechLevel::new(0),         // Stone Age
            Self::Bo => TechLevel::new(0),           // Stone Age
            Self::BowieKnife => TechLevel::new(5),   // Industrial Revolution
            Self::BrassKnuckles => TechLevel::new(3), // Industrial
            Self::Bullwhip => TechLevel::new(5),     // Industrial Revolution
            Self::Broadsword => TechLevel::new(2),   // Medieval
            Self::Chakram => TechLevel::new(2),      // Medieval Indian
            Self::Cestus => TechLevel::new(1),       // Roman
            Self::CattleProd => TechLevel::new(6),   // WWII/Mechanized
            Self::CeramicKnife => TechLevel::new(8), // Modern/Digital
            Self::Chain => TechLevel::new(1),        // Ancient
            Self::Chainsaw => TechLevel::new(6),     // WWII/Mechanized
            Self::Cinquedea => TechLevel::new(3),    // Late Medieval
            Self::Claymore => TechLevel::new(3),     // Scottish Renaissance
            Self::Club => TechLevel::new(0),
            Self::CombatNet => TechLevel::new(1), // Bronze Age
            Self::CombatKnife => TechLevel::new(6), // Atomic Age
            Self::Crowbar => TechLevel::new(3),   // Industrial
            Self::Cutlass => TechLevel::new(4),   // Age of Sail
            Self::Dagger => TechLevel::new(1),    // Bronze Age
            Self::Dao => TechLevel::new(2),       // Iron Age
            Self::Dirk => TechLevel::new(3),      // Scottish Medieval
            Self::Estoc => TechLevel::new(3),     // Late Medieval
            Self::ElectricKnuckles => TechLevel::new(8), // Digital/High-Tech
            Self::Eku => TechLevel::new(0),       // Stone Age
            Self::EntrenchingTool => TechLevel::new(5), // Industrial Revolution
            Self::Falchion => TechLevel::new(2),  // Medieval
            Self::Falx => TechLevel::new(1),      // Dacian/Iron Age
            Self::FairbairnSykes => TechLevel::new(6), // WWII Commando
            Self::Fauchard => TechLevel::new(3),  // Medieval
            Self::Fist => TechLevel::new(0),      // Stone Age
            Self::Francisca => TechLevel::new(2), // Iron Age
            Self::Flail => TechLevel::new(2),     // Medieval
            Self::Gladius => TechLevel::new(1),   // Roman/Iron Age
            Self::GrainFlail => TechLevel::new(0), // Stone Age
            Self::Garrote => TechLevel::new(1),   // Ancient
            Self::Glaive => TechLevel::new(2),
            Self::Guisarme => TechLevel::new(3),      // Medieval
            Self::GreatAxe => TechLevel::new(1),      // Bronze/Iron Age
            Self::Halberd => TechLevel::new(2),       // Medieval
            Self::Hatchet => TechLevel::new(0),       // Stone Age
            Self::HorsemansMace => TechLevel::new(2), // Medieval
            Self::HorsemansPick => TechLevel::new(2), // Medieval
            Self::HungaMunga => TechLevel::new(0),    // African/Stone Age
            Self::Iklwa => TechLevel::new(0),         // Stone Age
            Self::IcePick => TechLevel::new(5),       // Industrial Revolution
            Self::Javelin => TechLevel::new(0),       // Stone Age
            Self::Jitte => TechLevel::new(3),
            Self::Jian => TechLevel::new(2), // Iron Age          // Japanese feudal
            Self::Jo => TechLevel::new(0),   // Stone Age
            Self::KaBar => TechLevel::new(6), // WWII USMC
            Self::Kama => TechLevel::new(2), // Medieval Japan
            Self::Katana => TechLevel::new(3), // Medieval Japan
            Self::Katar => TechLevel::new(2),
            Self::Khanda => TechLevel::new(2),        // Iron Age
            Self::Kilij => TechLevel::new(3),         // Medieval          // Medieval India
            Self::Kick => TechLevel::new(0),          // Stone Age
            Self::Knife => TechLevel::new(0),         // Stone Age
            Self::Kopesh => TechLevel::new(1),        // Egyptian Bronze Age
            Self::Kopis => TechLevel::new(1),         // Greek/Iron Age
            Self::Konda => TechLevel::new(0),         // African/Stone Age
            Self::Kris => TechLevel::new(3),          // Indonesian
            Self::Kukri => TechLevel::new(5),         // Gurkha/Industrial
            Self::Kusarigama => TechLevel::new(3),    // Medieval Japan
            Self::Kubotan => TechLevel::new(7),       // Nuclear Age
            Self::Kusari => TechLevel::new(2),        // Medieval Japan
            Self::LargeKnife => TechLevel::new(1),    // Bronze Age
            Self::Lasso => TechLevel::new(0),         // Stone Age
            Self::Lance => TechLevel::new(2),         // Medieval
            Self::LongSpear => TechLevel::new(1),     // Bronze Age
            Self::Longsword => TechLevel::new(2),     // Medieval
            Self::LucerneHammer => TechLevel::new(3), // Late Medieval
            Self::Mace => TechLevel::new(1),          // Bronze Age
            Self::Macuahuitl => TechLevel::new(0),    // Aztec/Stone Age
            Self::Machete => TechLevel::new(5),       // Industrial/Utility
            Self::MainGauche => TechLevel::new(4),    // Renaissance
            Self::Maul => TechLevel::new(1),          // Bronze/Iron Age
            Self::Messer => TechLevel::new(3),        // Late Medieval
            Self::Mattock => TechLevel::new(1),       // Bronze/Iron Age
            Self::MilitaryFlail => TechLevel::new(2), // Medieval
            Self::Morningstar => TechLevel::new(2),   // Medieval
            Self::Monowire => TechLevel::new(8),      // Digital/High-Tech
            Self::Naginata => TechLevel::new(3),
            Self::Nightstick => TechLevel::new(6), // WWII era
            Self::Nunchaku => TechLevel::new(2),   // Medieval
            Self::NuntiBo => TechLevel::new(2),    // Medieval Japan
            Self::Pick => TechLevel::new(2),       // Medieval
            Self::Pitchfork => TechLevel::new(0),  // Stone Age
            Self::Pike => TechLevel::new(2),
            Self::Pilum => TechLevel::new(1), // Roman
            Self::Poleax => TechLevel::new(2),
            Self::Partisan => TechLevel::new(3),     // Medieval
            Self::PushDagger => TechLevel::new(5),   // Industrial
            Self::Quarterstaff => TechLevel::new(0), // Stone Age
            Self::Rapier => TechLevel::new(4),       // Renaissance
            Self::Ranseur => TechLevel::new(3),      // Medieval
            Self::Saber => TechLevel::new(4),
            Self::Sasumata => TechLevel::new(3), // Medieval Japan
            Self::Seax => TechLevel::new(2),     // Iron Age
            Self::Shamshir => TechLevel::new(3), // Medieval          // Age of Sail
            Self::Sai => TechLevel::new(2),      // Medieval
            Self::SapGloves => TechLevel::new(6), // WWII/Mechanized
            Self::Scimitar => TechLevel::new(2), // Medieval
            Self::Shortsword => TechLevel::new(1), // Bronze Age
            Self::Shovel => TechLevel::new(1),   // Ancient tool
            Self::Scythe => TechLevel::new(0),   // Stone Age
            Self::Sickle => TechLevel::new(0),   // Stone Age
            Self::Smallsword => TechLevel::new(4), // Renaissance
            Self::Sodegarami => TechLevel::new(3), // Medieval Japan
            Self::Spear => TechLevel::new(0),    // Stone Age
            Self::Spetum => TechLevel::new(3),   // Medieval
            Self::SpikedClub => TechLevel::new(0), // Stone Age
            Self::Staff => TechLevel::new(0),    // Stone Age
            Self::Stick => TechLevel::new(0),
            Self::Stiletto => TechLevel::new(3), // Italian Medieval
            Self::StunBaton => TechLevel::new(8), // Digital/High-Tech
            Self::SurvivalKnife => TechLevel::new(6), // WWII era
            Self::Tanto => TechLevel::new(3),    // Japanese
            Self::TacticalFolder => TechLevel::new(7), // Nuclear Age
            Self::TacticalPen => TechLevel::new(8), // Digital/High-Tech
            Self::TacticalTomahawk => TechLevel::new(8), // Modern Combat
            Self::Tekko => TechLevel::new(0),    // Okinawan/Stone Age
            Self::Talwar => TechLevel::new(2),   // Iron Age
            Self::TelescopingBaton => TechLevel::new(7), // Nuclear Age
            Self::Tetsubo => TechLevel::new(2),  // Medieval Japan
            Self::Tessen => TechLevel::new(3),   // Japanese Renaissance
            Self::ThreeSectionStaff => TechLevel::new(2), // Medieval China
            Self::ThrowingKnife => TechLevel::new(5), // Industrial
            Self::Tonfa => TechLevel::new(0),    // Stone Age
            Self::TrenchKnife => TechLevel::new(6), // Atomic Age
            Self::Trident => TechLevel::new(0),  // Stone Age
            Self::TwoHandedSword => TechLevel::new(2), // Medieval
            Self::Wakizashi => TechLevel::new(3), // Medieval Japan
            Self::Voulge => TechLevel::new(3),   // Medieval
            Self::Urumi => TechLevel::new(2),    // Iron Age
            Self::VibroKnife => TechLevel::new(8), // Digital/High-Tech
            Self::WarFan => TechLevel::new(3),   // Japanese Renaissance
            Self::WarScythe => TechLevel::new(2), // Iron Age/Medieval
            Self::Warhammer => TechLevel::new(2), // Medieval
            Self::Whip => TechLevel::new(1),     // Bronze Age
            Self::WireSaw => TechLevel::new(6),  // WWII/Mechanized
            Self::Xiphos => TechLevel::new(1),   // Greek/Bronze Age
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
            Self::Angon => WeaponDamage::Thrust {
                modifier: 2,
                damage_type: DamageType::Impaling,
            },
            Self::ArmingSword => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::AwlPike => WeaponDamage::Thrust {
                modifier: 3,
                damage_type: DamageType::Impaling,
            },
            Self::Assegai => WeaponDamage::Thrust {
                modifier: 2,
                damage_type: DamageType::Impaling,
            },
            Self::AspBaton => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::Baton => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
            Self::BattleAxe => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Cutting,
            },
            Self::Bayonet => WeaponDamage::Thrust {
                modifier: 1,
                damage_type: DamageType::Impaling,
            },
            Self::BeardedAxe => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
            Self::BecDeCorbin => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Impaling,
            },
            Self::Blackjack => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
            Self::BastardSword => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
            Self::BaghNakh => WeaponDamage::Thrust {
                modifier: -1,
                damage_type: DamageType::Impaling,
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
            Self::Bo => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Crushing,
            },
            Self::BowieKnife => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Impaling,
            },
            Self::BrassKnuckles => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
            Self::Bullwhip => WeaponDamage::Swing {
                modifier: -1,
                damage_type: DamageType::Cutting,
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
            Self::CattleProd => WeaponDamage::Thrust {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::CeramicKnife => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Impaling,
            },
            Self::Chain => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::Chainsaw => WeaponDamage::Swing {
                modifier: 4,
                damage_type: DamageType::Cutting,
            },
            Self::Cinquedea => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Impaling,
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
            Self::CombatKnife => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Impaling,
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
            Self::Dao => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::Dirk => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Impaling,
            },
            Self::Estoc => WeaponDamage::Thrust {
                modifier: 2,
                damage_type: DamageType::Impaling,
            },
            Self::ElectricKnuckles => WeaponDamage::Thrust {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::Eku => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Crushing,
            },
            Self::EntrenchingTool => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
            Self::Falchion => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
            Self::Falx => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Cutting,
            },
            Self::FairbairnSykes => WeaponDamage::Thrust {
                modifier: 1,
                damage_type: DamageType::Impaling,
            },
            Self::Fauchard => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Cutting,
            },
            Self::Fist => WeaponDamage::Thrust {
                modifier: -1,
                damage_type: DamageType::Crushing,
            },
            Self::Francisca => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
            Self::Flail => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Crushing,
            },
            Self::Gladius => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Cutting,
            },
            Self::GrainFlail => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Crushing,
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
            Self::HorsemansMace => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Crushing,
            },
            Self::HorsemansPick => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Impaling,
            },
            Self::HungaMunga => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::Iklwa => WeaponDamage::Thrust {
                modifier: 1,
                damage_type: DamageType::Impaling,
            },
            Self::IcePick => WeaponDamage::Thrust {
                modifier: 1,
                damage_type: DamageType::Impaling,
            },
            Self::Javelin => WeaponDamage::Thrust {
                modifier: 1,
                damage_type: DamageType::Impaling,
            },
            Self::Jitte => WeaponDamage::Thrust {
                modifier: -1,
                damage_type: DamageType::Impaling,
            },
            Self::Jian => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Cutting,
            },
            Self::Jo => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::KaBar => WeaponDamage::Thrust {
                modifier: 1,
                damage_type: DamageType::Impaling,
            },
            Self::Kama => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Cutting,
            },
            Self::Katana => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::Katar => WeaponDamage::Thrust {
                modifier: 1,
                damage_type: DamageType::Impaling,
            },
            Self::Khanda => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::Kilij => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
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
            Self::Kopis => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::Konda => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Cutting,
            },
            Self::Kris => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Impaling,
            },
            Self::Kukri => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Cutting,
            },
            Self::Kusarigama => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
            Self::Kubotan => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Crushing,
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
            Self::LucerneHammer => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Impaling,
            },
            Self::Mace => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Crushing,
            },
            Self::Macuahuitl => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Cutting,
            },
            Self::Machete => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::MainGauche => WeaponDamage::Thrust {
                modifier: -1,
                damage_type: DamageType::Impaling,
            },
            Self::Maul => WeaponDamage::Swing {
                modifier: 4,
                damage_type: DamageType::Crushing,
            },
            Self::Messer => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::Mattock => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Impaling,
            },
            Self::MilitaryFlail => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Crushing,
            },
            Self::Morningstar => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Crushing,
            },
            Self::Monowire => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Cutting,
            },
            Self::Naginata => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
            Self::Nightstick => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::Nunchaku => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::NuntiBo => WeaponDamage::Thrust {
                modifier: 2,
                damage_type: DamageType::Impaling,
            },
            Self::Pick => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Impaling,
            },
            Self::Pitchfork => WeaponDamage::Thrust {
                modifier: 1,
                damage_type: DamageType::Impaling,
            },
            Self::Pike => WeaponDamage::Thrust {
                modifier: 2,
                damage_type: DamageType::Impaling,
            },
            Self::Pilum => WeaponDamage::Thrust {
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
            Self::PushDagger => WeaponDamage::Thrust {
                modifier: 0,
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
            Self::Ranseur => WeaponDamage::Thrust {
                modifier: 2,
                damage_type: DamageType::Impaling,
            },
            Self::Saber => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::Sasumata => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::Seax => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Cutting,
            },
            Self::Shamshir => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::Sai => WeaponDamage::Thrust {
                modifier: -1,
                damage_type: DamageType::Impaling,
            },
            Self::SapGloves => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Crushing,
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
            Self::Scythe => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
            Self::Sickle => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Cutting,
            },
            Self::Smallsword => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Impaling,
            },
            Self::Sodegarami => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::Spear => WeaponDamage::Thrust {
                modifier: 2,
                damage_type: DamageType::Impaling,
            },
            Self::Spetum => WeaponDamage::Thrust {
                modifier: 2,
                damage_type: DamageType::Impaling,
            },
            Self::SpikedClub => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Crushing,
            },
            Self::Staff => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::Stick => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
            Self::Stiletto => WeaponDamage::Thrust {
                modifier: 1,
                damage_type: DamageType::Impaling,
            },
            Self::StunBaton => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
            Self::SurvivalKnife => WeaponDamage::Thrust {
                modifier: 1,
                damage_type: DamageType::Impaling,
            },
            Self::Tanto => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Impaling,
            },
            Self::TacticalFolder => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Impaling,
            },
            Self::TacticalPen => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Impaling,
            },
            Self::TacticalTomahawk => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
            Self::Tekko => WeaponDamage::Thrust {
                modifier: -1,
                damage_type: DamageType::Crushing,
            },
            Self::Talwar => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::TelescopingBaton => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::Tetsubo => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Crushing,
            },
            Self::Tessen => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
            Self::ThreeSectionStaff => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Crushing,
            },
            Self::ThrowingKnife => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Cutting,
            },
            Self::Tonfa => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::TrenchKnife => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Impaling,
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
            Self::Urumi => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::VibroKnife => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
            Self::WarFan => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
            Self::WarScythe => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Cutting,
            },
            Self::Warhammer => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Impaling,
            },
            Self::Whip => WeaponDamage::Swing {
                modifier: -1,
                damage_type: DamageType::Cutting,
            },
            Self::WireSaw => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::Xiphos => WeaponDamage::Swing {
                modifier: 0,
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
            Self::Angon => Reach::One,
            Self::ArmingSword => Reach::One,
            Self::AwlPike => Reach::TwoThree,
            Self::Assegai => Reach::One,
            Self::AspBaton => Reach::One,
            Self::Baton => Reach::One,
            Self::BattleAxe => Reach::One,
            Self::Bayonet => Reach::One,
            Self::BeardedAxe => Reach::One,
            Self::BecDeCorbin => Reach::TwoThree,
            Self::Blackjack => Reach::Close,
            Self::BastardSword => Reach::OneTwo,
            Self::BaghNakh => Reach::Close,
            Self::Bardiche => Reach::TwoThree,
            Self::Bill => Reach::TwoThree,
            Self::Billhook => Reach::OneTwo,
            Self::Bola => Reach::One,
            Self::Bo => Reach::OneTwo,
            Self::BowieKnife => Reach::CloseOne,
            Self::BrassKnuckles => Reach::Close,
            Self::Bullwhip => Reach::OneThree,
            Self::Broadsword => Reach::One,
            Self::Chakram => Reach::One,
            Self::Cestus => Reach::Close,
            Self::CattleProd => Reach::One,
            Self::CeramicKnife => Reach::Close,
            Self::Chain => Reach::OneThree,
            Self::Chainsaw => Reach::One,
            Self::Cinquedea => Reach::Close,
            Self::Claymore => Reach::OneTwo,
            Self::Club => Reach::One,
            Self::CombatNet => Reach::One,
            Self::CombatKnife => Reach::Close,
            Self::Crowbar => Reach::One,
            Self::Cutlass => Reach::One,
            Self::Dagger => Reach::Close,
            Self::Dao => Reach::One,
            Self::Dirk => Reach::Close,
            Self::Estoc => Reach::One,
            Self::ElectricKnuckles => Reach::Close,
            Self::Eku => Reach::OneTwo,
            Self::EntrenchingTool => Reach::One,
            Self::Falchion => Reach::One,
            Self::Falx => Reach::One,
            Self::FairbairnSykes => Reach::Close,
            Self::Fauchard => Reach::TwoThree,
            Self::Fist => Reach::Close,
            Self::Francisca => Reach::One,
            Self::Flail => Reach::OneTwo,
            Self::Gladius => Reach::One,
            Self::GrainFlail => Reach::OneTwo,
            Self::Garrote => Reach::Close,
            Self::Glaive => Reach::TwoThree,
            Self::Guisarme => Reach::TwoThree,
            Self::GreatAxe => Reach::OneTwo,
            Self::Halberd => Reach::TwoThree,
            Self::Hatchet => Reach::One,
            Self::HorsemansMace => Reach::One,
            Self::HorsemansPick => Reach::One,
            Self::HungaMunga => Reach::One,
            Self::Iklwa => Reach::One,
            Self::IcePick => Reach::Close,
            Self::Javelin => Reach::One,
            Self::Jitte => Reach::One,
            Self::Jian => Reach::One,
            Self::Jo => Reach::One,
            Self::KaBar => Reach::CloseOne,
            Self::Kama => Reach::One,
            Self::Katana => Reach::One,
            Self::Katar => Reach::Close,
            Self::Khanda => Reach::One,
            Self::Kilij => Reach::One,
            Self::Kick => Reach::CloseOne,
            Self::Knife => Reach::Close,
            Self::Kopesh => Reach::One,
            Self::Kopis => Reach::One,
            Self::Konda => Reach::One,
            Self::Kris => Reach::Close,
            Self::Kukri => Reach::CloseOne,
            Self::Kusarigama => Reach::OneThree,
            Self::Kubotan => Reach::Close,
            Self::Kusari => Reach::OneThree,
            Self::LargeKnife => Reach::CloseOne,
            Self::Lasso => Reach::OneTwo,
            Self::Lance => Reach::Three,
            Self::LongSpear => Reach::TwoThree,
            Self::Longsword => Reach::One,
            Self::LucerneHammer => Reach::TwoThree,
            Self::Mace => Reach::One,
            Self::Macuahuitl => Reach::One,
            Self::Machete => Reach::One,
            Self::MainGauche => Reach::One,
            Self::Maul => Reach::OneTwo,
            Self::Messer => Reach::One,
            Self::Mattock => Reach::One,
            Self::MilitaryFlail => Reach::OneTwo,
            Self::Morningstar => Reach::One,
            Self::Monowire => Reach::Close,
            Self::Naginata => Reach::TwoThree,
            Self::Nightstick => Reach::One,
            Self::Nunchaku => Reach::One,
            Self::NuntiBo => Reach::OneTwo,
            Self::Pick => Reach::One,
            Self::Pitchfork => Reach::One,
            Self::Pike => Reach::TwoThree,
            Self::Pilum => Reach::One,
            Self::Poleax => Reach::TwoThree,
            Self::Partisan => Reach::TwoThree,
            Self::PushDagger => Reach::Close,
            Self::Quarterstaff => Reach::OneTwo,
            Self::Rapier => Reach::One,
            Self::Ranseur => Reach::TwoThree,
            Self::Saber => Reach::One,
            Self::Sasumata => Reach::TwoThree,
            Self::Seax => Reach::One,
            Self::Shamshir => Reach::One,
            Self::Sai => Reach::Close,
            Self::SapGloves => Reach::Close,
            Self::Scimitar => Reach::One,
            Self::Shortsword => Reach::One,
            Self::Shovel => Reach::One,
            Self::Scythe => Reach::One,
            Self::Sickle => Reach::CloseOne,
            Self::Smallsword => Reach::One,
            Self::Sodegarami => Reach::TwoThree,
            Self::Spear => Reach::OneTwo,
            Self::Spetum => Reach::TwoThree,
            Self::SpikedClub => Reach::One,
            Self::Staff => Reach::OneTwo,
            Self::Stick => Reach::One,
            Self::Stiletto => Reach::Close,
            Self::StunBaton => Reach::One,
            Self::SurvivalKnife => Reach::CloseOne,
            Self::Tanto => Reach::Close,
            Self::TacticalFolder => Reach::Close,
            Self::TacticalPen => Reach::Close,
            Self::TacticalTomahawk => Reach::One,
            Self::Tekko => Reach::Close,
            Self::Talwar => Reach::One,
            Self::TelescopingBaton => Reach::One,
            Self::Tetsubo => Reach::OneTwo,
            Self::Tessen => Reach::Close,
            Self::ThreeSectionStaff => Reach::OneTwo,
            Self::ThrowingKnife => Reach::One,
            Self::Tonfa => Reach::One,
            Self::TrenchKnife => Reach::Close,
            Self::Trident => Reach::OneTwo,
            Self::TwoHandedSword => Reach::OneTwo,
            Self::Wakizashi => Reach::One,
            Self::Voulge => Reach::TwoThree,
            Self::Urumi => Reach::OneThree,
            Self::VibroKnife => Reach::One,
            Self::WarFan => Reach::Close,
            Self::WarScythe => Reach::TwoThree,
            Self::Warhammer => Reach::OneTwo,
            Self::Whip => Reach::OneTwo,
            Self::WireSaw => Reach::Close,
            Self::Xiphos => Reach::One,
        }
    }

    /// Returns parry modifier for this melee weapon.
    #[instrument]
    pub fn parry_modifier(&self) -> i32 {
        debug!("Getting melee weapon parry modifier");
        match self {
            Self::Axe => -1,
            Self::Angon => 0,
            Self::ArmingSword => 0,
            Self::AwlPike => 0,
            Self::Assegai => 0,
            Self::AspBaton => 0,
            Self::Baton => 0,
            Self::BattleAxe => -1,
            Self::BeardedAxe => -1,
            Self::BecDeCorbin => 0,
            Self::Blackjack => 0,
            Self::BastardSword => 0,
            Self::Bayonet => 0,
            Self::BaghNakh => -1,
            Self::Bardiche => 0,
            Self::Bill => 0,
            Self::Billhook => 0,
            Self::Bola => -2,
            Self::Bo => 2,
            Self::BowieKnife => -1,
            Self::BrassKnuckles => 0,
            Self::Bullwhip => -2,
            Self::Broadsword => 0,
            Self::Chakram => -2,
            Self::Cestus => 0,
            Self::CattleProd => -2,
            Self::CeramicKnife => -1,
            Self::Chain => -2,
            Self::Chainsaw => -4,
            Self::Cinquedea => -1,
            Self::Claymore => 0,
            Self::Club => 0,
            Self::CombatNet => -2,
            Self::CombatKnife => -1,
            Self::Crowbar => 0,
            Self::Cutlass => 0,
            Self::Dagger => -1,
            Self::Dao => 0,
            Self::Dirk => -1,
            Self::Estoc => 1,
            Self::ElectricKnuckles => 0,
            Self::Eku => 2,
            Self::EntrenchingTool => -1,
            Self::Falchion => 0,
            Self::Falx => -1,
            Self::FairbairnSykes => -1,
            Self::Fauchard => 0,
            Self::Fist => 0,
            Self::Francisca => -1,
            Self::Flail => -2,
            Self::Gladius => 0,
            Self::GrainFlail => -2,
            Self::Garrote => -4,
            Self::Glaive => 0,
            Self::Guisarme => 0,
            Self::GreatAxe => -2,
            Self::Halberd => 0,
            Self::Hatchet => -1,
            Self::HorsemansMace => 0,
            Self::HorsemansPick => -1,
            Self::HungaMunga => -2,
            Self::Iklwa => 0,
            Self::IcePick => -1,
            Self::Javelin => 0,
            Self::Jitte => 1,
            Self::Jian => 0,
            Self::Jo => 1,
            Self::KaBar => -1,
            Self::Kama => -1,
            Self::Katana => 0,
            Self::Katar => 0,
            Self::Khanda => 0,
            Self::Kilij => 0,
            Self::Kick => -2,
            Self::Knife => -1,
            Self::Kopesh => 0,
            Self::Kopis => 0,
            Self::Konda => -2,
            Self::Kris => -1,
            Self::Kukri => -1,
            Self::Kusarigama => -2,
            Self::Kubotan => -2,
            Self::Kusari => -2,
            Self::LargeKnife => -1,
            Self::Lasso => -2,
            Self::Lance => -2,
            Self::LongSpear => 0,
            Self::Longsword => 0,
            Self::LucerneHammer => 0,
            Self::Mace => 0,
            Self::Macuahuitl => -1,
            Self::Machete => 0,
            Self::MainGauche => 1,
            Self::Maul => -2,
            Self::Messer => 0,
            Self::Mattock => -1,
            Self::MilitaryFlail => -2,
            Self::Morningstar => 0,
            Self::Monowire => -2,
            Self::Naginata => 1,
            Self::Nightstick => 0,
            Self::Nunchaku => -2,
            Self::NuntiBo => 1,
            Self::Pick => -1,
            Self::Pitchfork => 0,
            Self::Pike => 0,
            Self::Pilum => 0,
            Self::Poleax => 0,
            Self::Partisan => 0,
            Self::PushDagger => -1,
            Self::Quarterstaff => 2,
            Self::Rapier => 1,
            Self::Ranseur => 0,
            Self::Saber => 0,
            Self::Sasumata => 0,
            Self::Seax => 0,
            Self::Shamshir => 0,
            Self::Sai => 1,
            Self::SapGloves => 0,
            Self::Scimitar => 0,
            Self::Shortsword => 0,
            Self::Shovel => -1,
            Self::Scythe => -2,
            Self::Sickle => -2,
            Self::Smallsword => 1,
            Self::Sodegarami => 0,
            Self::Spear => 0,
            Self::Spetum => 0,
            Self::SpikedClub => 0,
            Self::Staff => 2,
            Self::Stick => 1,
            Self::Stiletto => -1,
            Self::StunBaton => 0,
            Self::SurvivalKnife => -1,
            Self::Tanto => -1,
            Self::TacticalFolder => -1,
            Self::TacticalPen => -2,
            Self::TacticalTomahawk => -1,
            Self::Tekko => 0,
            Self::Talwar => 0,
            Self::TelescopingBaton => 0,
            Self::Tetsubo => -2,
            Self::Tessen => 1,
            Self::ThreeSectionStaff => -2,
            Self::ThrowingKnife => -2,
            Self::Tonfa => 1,
            Self::TrenchKnife => -1,
            Self::Trident => 0,
            Self::TwoHandedSword => 0,
            Self::Wakizashi => 0,
            Self::Voulge => 0,
            Self::Urumi => -2,
            Self::VibroKnife => -1,
            Self::WarFan => 1,
            Self::WarScythe => -2,
            Self::Warhammer => -1,
            Self::Whip => -1,
            Self::WireSaw => -2,
            Self::Xiphos => 0,
        }
    }

    /// Returns required skill for this melee weapon.
    #[instrument]
    pub fn required_skill(&self) -> Skill {
        debug!("Getting melee weapon required skill");
        match self {
            Self::Axe => Skill::AxeMace,
            Self::Angon => Skill::Spear,
            Self::ArmingSword => Skill::Broadsword,
            Self::AwlPike => Skill::Spear,
            Self::Assegai => Skill::Spear,
            Self::AspBaton => Skill::AxeMace,
            Self::Baton => Skill::Shortsword,
            Self::BattleAxe => Skill::AxeMace,
            Self::BeardedAxe => Skill::AxeMace,
            Self::BecDeCorbin => Skill::Polearm,
            Self::Blackjack => Skill::Brawling,
            Self::BastardSword => Skill::Broadsword,
            Self::Bayonet => Skill::Spear,
            Self::BaghNakh => Skill::Brawling,
            Self::Bardiche => Skill::Polearm,
            Self::Bill => Skill::Polearm,
            Self::Billhook => Skill::Polearm,
            Self::Bola => Skill::ThrownWeapon,
            Self::Bo => Skill::Staff,
            Self::BowieKnife => Skill::Knife,
            Self::BrassKnuckles => Skill::Brawling,
            Self::Bullwhip => Skill::Whip,
            Self::Broadsword => Skill::Broadsword,
            Self::Chakram => Skill::ThrownWeapon,
            Self::Cestus => Skill::Brawling,
            Self::CattleProd => Skill::AxeMace,
            Self::CeramicKnife => Skill::Knife,
            Self::Chain => Skill::Flail,
            Self::Chainsaw => Skill::AxeMace,
            Self::Cinquedea => Skill::Knife,
            Self::Claymore => Skill::TwoHandedSword,
            Self::Club => Skill::AxeMace,
            Self::CombatNet => Skill::ThrownWeapon,
            Self::CombatKnife => Skill::Knife,
            Self::Crowbar => Skill::AxeMace,
            Self::Cutlass => Skill::Broadsword,
            Self::Dagger => Skill::Knife,
            Self::Dao => Skill::Broadsword,
            Self::Dirk => Skill::Knife,
            Self::Estoc => Skill::Rapier,
            Self::ElectricKnuckles => Skill::Brawling,
            Self::Eku => Skill::Staff,
            Self::EntrenchingTool => Skill::AxeMace,
            Self::Falchion => Skill::Broadsword,
            Self::Falx => Skill::TwoHandedAxeMace,
            Self::FairbairnSykes => Skill::Knife,
            Self::Fauchard => Skill::Polearm,
            Self::Fist => Skill::Brawling,
            Self::Francisca => Skill::AxeMace,
            Self::Flail => Skill::Flail,
            Self::Gladius => Skill::Shortsword,
            Self::GrainFlail => Skill::Flail,
            Self::Garrote => Skill::Brawling,
            Self::Glaive => Skill::Polearm,
            Self::Guisarme => Skill::Polearm,
            Self::GreatAxe => Skill::TwoHandedAxeMace,
            Self::Halberd => Skill::Polearm,
            Self::Hatchet => Skill::AxeMace,
            Self::HorsemansMace => Skill::AxeMace,
            Self::HorsemansPick => Skill::AxeMace,
            Self::HungaMunga => Skill::ThrownWeapon,
            Self::Iklwa => Skill::Spear,
            Self::IcePick => Skill::Knife,
            Self::Javelin => Skill::Spear,
            Self::Jitte => Skill::MainGauche,
            Self::Jian => Skill::Shortsword,
            Self::Jo => Skill::Staff,
            Self::KaBar => Skill::Knife,
            Self::Kama => Skill::Knife,
            Self::Katana => Skill::TwoHandedSword,
            Self::Katar => Skill::Knife,
            Self::Khanda => Skill::Broadsword,
            Self::Kilij => Skill::Saber,
            Self::Kick => Skill::Brawling,
            Self::Knife => Skill::Knife,
            Self::Kopesh => Skill::Broadsword,
            Self::Kopis => Skill::Broadsword,
            Self::Konda => Skill::ThrownWeapon,
            Self::Kris => Skill::Knife,
            Self::Kukri => Skill::Knife,
            Self::Kusarigama => Skill::Flail,
            Self::Kubotan => Skill::Brawling,
            Self::Kusari => Skill::Kusari,
            Self::LargeKnife => Skill::Knife,
            Self::Lasso => Skill::ThrownWeapon,
            Self::Lance => Skill::Lance,
            Self::LongSpear => Skill::Spear,
            Self::Longsword => Skill::Broadsword,
            Self::LucerneHammer => Skill::Polearm,
            Self::Mace => Skill::AxeMace,
            Self::Macuahuitl => Skill::Broadsword,
            Self::Machete => Skill::Broadsword,
            Self::MainGauche => Skill::MainGauche,
            Self::Maul => Skill::TwoHandedAxeMace,
            Self::Messer => Skill::Broadsword,
            Self::Mattock => Skill::AxeMace,
            Self::MilitaryFlail => Skill::Flail,
            Self::Morningstar => Skill::Flail,
            Self::Monowire => Skill::Whip,
            Self::Naginata => Skill::Polearm,
            Self::Nightstick => Skill::AxeMace,
            Self::Nunchaku => Skill::Flail,
            Self::NuntiBo => Skill::Spear,
            Self::Pick => Skill::AxeMace,
            Self::Pitchfork => Skill::Spear,
            Self::Pike => Skill::Spear,
            Self::Pilum => Skill::Spear,
            Self::Poleax => Skill::Polearm,
            Self::Partisan => Skill::Polearm,
            Self::PushDagger => Skill::Knife,
            Self::Quarterstaff => Skill::Staff,
            Self::Rapier => Skill::Rapier,
            Self::Ranseur => Skill::Polearm,
            Self::Saber => Skill::Saber,
            Self::Sasumata => Skill::Polearm,
            Self::Seax => Skill::Shortsword,
            Self::Shamshir => Skill::Saber,
            Self::Sai => Skill::Knife,
            Self::SapGloves => Skill::Brawling,
            Self::Scimitar => Skill::Broadsword,
            Self::Shortsword => Skill::Shortsword,
            Self::Shovel => Skill::AxeMace,
            Self::Scythe => Skill::TwoHandedAxeMace,
            Self::Sickle => Skill::Knife,
            Self::Smallsword => Skill::Smallsword,
            Self::Sodegarami => Skill::Polearm,
            Self::Spear => Skill::Spear,
            Self::Spetum => Skill::Polearm,
            Self::SpikedClub => Skill::AxeMace,
            Self::Staff => Skill::Staff,
            Self::Stick => Skill::Staff,
            Self::Stiletto => Skill::Knife,
            Self::StunBaton => Skill::AxeMace,
            Self::SurvivalKnife => Skill::Knife,
            Self::Tanto => Skill::Knife,
            Self::TacticalFolder => Skill::Knife,
            Self::TacticalPen => Skill::Knife,
            Self::TacticalTomahawk => Skill::AxeMace,
            Self::Tekko => Skill::Brawling,
            Self::Talwar => Skill::Broadsword,
            Self::TelescopingBaton => Skill::AxeMace,
            Self::Tetsubo => Skill::TwoHandedAxeMace,
            Self::Tessen => Skill::Shortsword,
            Self::ThreeSectionStaff => Skill::Flail,
            Self::ThrowingKnife => Skill::ThrownWeapon,
            Self::Tonfa => Skill::Shortsword,
            Self::TrenchKnife => Skill::Knife,
            Self::Trident => Skill::Spear,
            Self::TwoHandedSword => Skill::TwoHandedSword,
            Self::Wakizashi => Skill::Shortsword,
            Self::Voulge => Skill::Polearm,
            Self::Urumi => Skill::Shortsword,
            Self::VibroKnife => Skill::Knife,
            Self::WarFan => Skill::Shortsword,
            Self::WarScythe => Skill::Polearm,
            Self::Warhammer => Skill::TwoHandedAxeMace,
            Self::Whip => Skill::Shortsword,
            Self::WireSaw => Skill::Knife,
            Self::Xiphos => Skill::Shortsword,
        }
    }
}
