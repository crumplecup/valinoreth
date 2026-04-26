//! Item enumeration and property delegation.
//!
//! # Overview
//!
//! The `Item` enum contains all GURPS equipment variants. Each item delegates
//! property lookups to category-specific modules, similar to the spell system's
//! college delegation pattern.
//!
//! # Organization
//!
//! - Universal properties (cost, weight, TL): Available on all items via delegation
//! - Category-specific properties: Return `Option<T>` for type safety
//! - Property modules handle the actual data (weapons_melee.rs, armor.rs, etc.)

use crate::{
    Capacity, Currency, ItemCategory, Quality, Reach, Skill, TechLevel, WeaponDamage, Weight,
};
use tracing::{debug, instrument};

/// All GURPS items enumerated.
///
/// Each variant represents a specific item from GURPS Basic Set and supplements.
/// Items delegate to category modules for property lookup.
///
/// # Examples
///
/// ```
/// use valinoreth::{Item, ItemCategory};
///
/// let sword = Item::Broadsword;
/// assert_eq!(sword.category(), ItemCategory::MeleeWeapon);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, strum::EnumIter)]
pub enum Item {
    // Melee Weapons (30 variants from existing MeleeWeapon enum)
    /// Axe, swing+2 cutting. BS 271. $50, 4 lbs, TL 0
    Axe,
    /// Baton/Billy Club, swing crushing. BS 271. $20, 1 lb, TL 5
    Baton,
    /// Brass Knuckles, thrust crushing. BS 271. $10, 0.25 lbs, TL 3
    BrassKnuckles,
    /// Broadsword, swing+1 cutting. BS 271. $500, 3 lbs, TL 2
    Broadsword,
    /// Dagger, thrust-1 impaling. BS 271. $20, 0.25 lbs, TL 1
    Dagger,
    /// Fist (unarmed), thrust-1 crushing. BS 271. $0, 0 lbs, TL 0
    Fist,
    /// Flail, swing+2 crushing. BS 272. $60, 8 lbs, TL 2
    Flail,
    /// Great Axe, swing+3 cutting. BS 272. $100, 8 lbs, TL 1
    GreatAxe,
    /// Halberd, swing+3 cutting. BS 272. $150, 12 lbs, TL 2
    Halberd,
    /// Hatchet, swing+1 cutting. BS 272. $40, 2 lbs, TL 0
    Hatchet,
    /// Javelin, thrust+1 impaling. BS 272. $30, 2 lbs, TL 0
    Javelin,
    /// Kick, thrust crushing. BS 272. $0, 0 lbs, TL 0
    Kick,
    /// Knife, thrust-1 impaling. BS 273. $40, 1 lb, TL 0
    Knife,
    /// Kusari (chain), swing+1 crushing. BS 273. $70, 5 lbs, TL 2
    Kusari,
    /// Lance, thrust+3 impaling. BS 273. $60, 6 lbs, TL 2
    Lance,
    /// Long Spear, thrust+2 impaling. BS 273. $60, 5 lbs, TL 1
    LongSpear,
    /// Mace, swing+2 crushing. BS 273. $50, 5 lbs, TL 1
    Mace,
    /// Main-Gauche, thrust-1 impaling. BS 273. $50, 1.25 lbs, TL 4
    MainGauche,
    /// Morningstar, swing+3 crushing. BS 274. $80, 6 lbs, TL 2
    Morningstar,
    /// Quarterstaff, swing+2 crushing. BS 274. $10, 4 lbs, TL 0
    Quarterstaff,
    /// Rapier, thrust+1 impaling. BS 274. $500, 2.75 lbs, TL 4
    Rapier,
    /// Saber, swing+1 cutting. BS 274. $500, 2 lbs, TL 4
    Saber,
    /// Shortsword, swing cutting. BS 275. $400, 2 lbs, TL 1
    Shortsword,
    /// Smallsword, thrust impaling. BS 275. $400, 1.5 lbs, TL 4
    Smallsword,
    /// Spear, thrust+2 impaling. BS 275. $40, 4 lbs, TL 0
    Spear,
    /// Staff, swing+1 crushing. BS 275. $5, 4 lbs, TL 0
    Staff,
    /// Two-Handed Sword, swing+2 cutting. BS 276. $900, 7 lbs, TL 2
    TwoHandedSword,
    /// Warhammer, swing+3 impaling. BS 276. $100, 7 lbs, TL 2
    Warhammer,
    /// Katana, swing+1 cutting. LT 66. $650, 2.5 lbs, TL 3
    Katana,
    /// Scimitar, swing+1 cutting. LT 66. $500, 3 lbs, TL 2
    Scimitar,
    /// Cutlass, swing+1 cutting. LT 67. $400, 2 lbs, TL 4
    Cutlass,
    /// Longsword, swing+1 cutting. LT 66. $500, 3 lbs, TL 2
    Longsword,
    /// Bastard Sword, swing+2 cutting. LT 66. $650, 5 lbs, TL 2
    BastardSword,
    /// Wakizashi, swing cutting. LT 66. $400, 1.5 lbs, TL 3
    Wakizashi,
    /// Nunchaku, swing+1 crushing. MA 231. $20, 1.5 lbs, TL 2
    Nunchaku,
    /// Sai, thrust-1 impaling. MA 231. $20, 1 lb, TL 2
    Sai,
    /// Katar (punch dagger), thrust+1 impaling. LT 67. $40, 1 lb, TL 2
    Katar,
    /// Tonfa, swing+1 crushing. MA 232. $20, 1.5 lbs, TL 0
    Tonfa,
    /// Estoc (thrusting sword), thrust+2 impaling. LT 66. $600, 3.5 lbs, TL 3
    Estoc,
    /// Falchion, swing+2 cutting. LT 66. $400, 3.5 lbs, TL 2
    Falchion,
    /// Gladius, swing cutting. LT 67. $200, 2 lbs, TL 1
    Gladius,
    /// Maul (great hammer), swing+4 crushing. LT 67. $80, 12 lbs, TL 1
    Maul,
    /// Pick (war pick), swing+2 impaling. LT 67. $70, 3 lbs, TL 2
    Pick,
    /// Trident, thrust+2 impaling. LT 67. $100, 4 lbs, TL 0
    Trident,
    /// Whip, swing-1 cutting (reach 1-2). BS 274. $20, 2 lbs, TL 1
    Whip,
    /// Combat Net (entangling). BS 274. $40, 5 lbs, TL 1
    CombatNet,
    /// Bola (thrown entangle), swing crushing. BS 277. $20, 1 lb, TL 0
    Bola,
    /// War Fan (tessen), swing crushing. MA 230. $50, 1 lb, TL 3
    WarFan,
    /// Kopesh (Egyptian sword), swing+1 cutting. LT 66. $200, 3.5 lbs, TL 1
    Kopesh,
    /// Claymore (Scottish greatsword), swing+2 cutting. LT 66. $500, 7 lbs, TL 3
    Claymore,
    /// Tanto (Japanese knife), thrust impaling. MA 230. $30, 0.5 lbs, TL 3
    Tanto,
    /// Chakram (throwing ring), swing cutting. LT 78. $15, 0.5 lbs, TL 2
    Chakram,
    /// Battle Axe, swing+3 cutting. BS 274. $50, 6 lbs, TL 1
    BattleAxe,

    // Ranged Weapons (7 variants from existing RangedWeapon enum)
    /// Bow, 1d impaling. BS 276. $100, 2 lbs, TL 0
    Bow,
    /// Crossbow, 1d+4 impaling. BS 276. $150, 6 lbs, TL 2
    Crossbow,
    /// Pistol (9mm), 2d+2 piercing. BS 278. $350, 1.5 lbs, TL 6
    Pistol,
    /// Rifle (.30), 5d piercing. BS 278. $500, 9 lbs, TL 6
    Rifle,
    /// Shotgun (12ga), 1d+1 piercing. BS 278. $500, 8 lbs, TL 5
    Shotgun,
    /// Sling, swing piercing. BS 277. $20, 0.5 lbs, TL 0
    Sling,
    /// Throwing Knife, thrust-1 impaling. BS 277. $30, 0.5 lbs, TL 0
    ThrowingKnife,
    /// Longbow, 1d+2 impaling. BS 276. $200, 3 lbs, TL 0
    Longbow,
    /// Composite Bow, 1d+3 impaling. BS 276. $900, 2 lbs, TL 2
    CompositeBow,
    /// Short Bow, 1d-1 impaling. LT 77. $50, 1 lb, TL 0
    ShortBow,
    /// Light Crossbow, 1d+2 impaling. LT 77. $150, 4 lbs, TL 2
    LightCrossbow,
    /// Heavy Crossbow, 1d+5 impaling. LT 77. $200, 8 lbs, TL 2
    HeavyCrossbow,
    /// Throwing Axe, swing+2 cutting. LT 78. $60, 2 lbs, TL 0
    ThrowingAxe,
    /// Shuriken (throwing star), thrust impaling. MA 229. $5, 0.1 lbs, TL 2
    Shuriken,
    /// Dart, thrust-1 impaling. BS 277. $10, 0.1 lbs, TL 0
    Dart,
    /// Revolver (.38), 2d piercing. HT 102. $300, 2 lbs, TL 6
    Revolver,
    /// SMG (9mm), 2d+2 piercing. HT 104. $450, 7 lbs, TL 6
    SMG,
    /// Assault Rifle (5.56mm), 5d piercing. HT 105. $900, 9 lbs, TL 7
    AssaultRifle,
    /// Sniper Rifle (.308), 7d piercing. HT 106. $3500, 11 lbs, TL 7
    SniperRifle,
    /// Blowgun, 1d-3 impaling. BS 277. $30, 1 lb, TL 0
    Blowgun,
    /// Atlatl (spear thrower), thrust+3 impaling. LT 78. $20, 1 lb, TL 0
    Atlatl,
    /// Musket (.75 ball), 4d piercing. HT 97. $300, 10 lbs, TL 4
    Musket,
    /// Derringer (.41), 1d+2 piercing. HT 99. $100, 0.5 lbs, TL 5
    Derringer,
    /// Hunting Rifle (.30-06), 7d piercing. HT 101. $700, 9 lbs, TL 5
    HuntingRifle,
    /// Machine Gun (.30), 7d piercing. HT 108. $4000, 30 lbs, TL 6
    MachineGun,
    /// Grenade (frag), 3d×2 crushing. HT 179. $30, 1 lb, TL 6
    Grenade,
    /// Flamethrower, spec burning. HT 123. $1000, 70 lbs, TL 6
    Flamethrower,
    /// Rocket Launcher, spec crushing. HT 180. $2000, 15 lbs, TL 7
    RocketLauncher,
    /// Staff Sling, swing+2 piercing. LT 78. $20, 1 lb, TL 1
    StaffSling,
    /// Pellet Bow (modern compound), 1d+4 impaling. HT 100. $400, 3 lbs, TL 7
    PelletBow,
    /// Hand Crossbow, 1d impaling. LT 77. $150, 3 lbs, TL 2
    HandCrossbow,

    // Armor (5 variants from existing Armor struct tests)
    /// No armor, DR 0. BS 279. $0, 0 lbs, TL 0
    NoArmor,
    /// Leather Armor, DR 1. BS 279. $100, 10 lbs, TL 1
    LeatherArmor,
    /// Chainmail, DR 4. BS 279. $550, 35 lbs, TL 2
    Chainmail,
    /// Plate Armor, DR 6. BS 279. $3000, 50 lbs, TL 3
    PlateArmor,
    /// Heavy Plate, DR 8. BS 279. $6000, 60 lbs, TL 3
    HeavyPlate,
    /// Cloth Armor (heavy fabric), DR 1. LT 103. $30, 6 lbs, TL 1
    ClothArmor,
    /// Light Leather, DR 1. LT 103. $50, 5 lbs, TL 0
    LightLeather,
    /// Heavy Leather, DR 2. LT 103. $150, 15 lbs, TL 1
    HeavyLeather,
    /// Scale Mail, DR 4. LT 104. $420, 42 lbs, TL 2
    ScaleMail,
    /// Splint Mail, DR 5. LT 104. $700, 45 lbs, TL 2
    SplintMail,
    /// Half Plate, DR 5. LT 105. $1500, 30 lbs, TL 3
    HalfPlate,
    /// Mail Hauberk (long chainmail), DR 4. LT 104. $600, 45 lbs, TL 2
    MailHauberk,
    /// Mail Shirt (short chainmail), DR 4. LT 104. $350, 25 lbs, TL 2
    MailShirt,
    /// Brigandine, DR 4. LT 105. $500, 25 lbs, TL 3
    Brigandine,
    /// Lamellar Armor, DR 4. LT 104. $500, 35 lbs, TL 1
    LamellarArmor,
    /// Bronze Plate, DR 5. LT 105. $2400, 60 lbs, TL 1
    BronzePlate,
    /// Light Scale, DR 3. LT 104. $280, 28 lbs, TL 2
    LightScale,
    /// Ballistic Vest, DR 10/4. HT 178. $400, 2 lbs, TL 7
    BallisticVest,
    /// Tactical Vest, DR 18/6. HT 179. $900, 9 lbs, TL 8
    TacticalVest,
    /// Flak Jacket, DR 7/2. HT 178. $500, 20 lbs, TL 6
    FlakJacket,
    /// Small Shield, DB 1. BS 287. $40, 8 lbs, TL 1
    SmallShield,
    /// Medium Shield, DB 2. BS 287. $60, 15 lbs, TL 1
    MediumShield,
    /// Large Shield, DB 3. BS 287. $90, 25 lbs, TL 1
    LargeShield,
    /// Buckler, DB 1. BS 287. $25, 5 lbs, TL 2
    Buckler,
    /// Tower Shield, DB 4. LT 106. $150, 45 lbs, TL 2
    TowerShield,

    // Clothing (10 variants)
    /// Ordinary Clothing. BS 266. $120, 2 lbs, TL 1
    Clothing,
    /// Boots (leather). BS 266. $80, 2 lbs, TL 1
    Boots,
    /// Gloves (leather). BS 266. $30, 0.5 lbs, TL 1
    Gloves,
    /// Cloak (heavy). BS 266. $50, 4 lbs, TL 1
    Cloak,
    /// Hat (any). BS 266. $10, 0.5 lbs, TL 1
    Hat,
    /// Belt (leather). BS 266. $15, 0.25 lbs, TL 0
    Belt,
    /// Robe. BS 266. $20, 2 lbs, TL 1
    Robe,
    /// Sandals. BS 266. $25, 0.5 lbs, TL 0
    Sandals,
    /// Heavy Boots. BS 266. $100, 3 lbs, TL 2
    HeavyBoots,
    /// Reinforced Gloves. BS 266. $50, 0.75 lbs, TL 2
    ReinforcedGloves,
    /// Tunic (simple). BS 266. $30, 1 lb, TL 0
    Tunic,
    /// Shirt (cloth). BS 266. $20, 0.5 lbs, TL 1
    Shirt,
    /// Pants (cloth). BS 266. $30, 1 lb, TL 1
    Pants,
    /// Dress (simple). BS 266. $40, 2 lbs, TL 1
    Dress,
    /// Cape (short). BS 266. $20, 1 lb, TL 0
    Cape,
    /// Hood (cloth). BS 266. $5, 0.25 lbs, TL 0
    Hood,
    /// Scarf. BS 266. $5, 0.1 lbs, TL 0
    Scarf,
    /// Vest (cloth). BS 266. $25, 0.5 lbs, TL 1
    Vest,
    /// Apron (work). BS 266. $15, 0.75 lbs, TL 1
    Apron,
    /// Tabard (heraldic). BS 266. $25, 1 lb, TL 2
    Tabard,
    /// Breeches (medieval pants). BS 266. $35, 1.5 lbs, TL 2
    Breeches,
    /// Stockings (leg wear). BS 266. $5, 0.25 lbs, TL 1
    Stockings,
    /// Jacket. BS 266. $50, 2 lbs, TL 3
    Jacket,
    /// Coat (heavy). BS 266. $75, 4 lbs, TL 4
    Coat,
    /// Mantle (formal cloak). BS 266. $60, 3 lbs, TL 2
    Mantle,
    /// Fine Gloves (silk/leather). BS 266. $40, 0.25 lbs, TL 2
    FineGloves,
    /// Riding Boots (tall). BS 266. $120, 3 lbs, TL 2
    RidingBoots,
    /// Sash (waist). BS 266. $10, 0.5 lbs, TL 1
    Sash,
    /// Wimple (head covering). BS 266. $5, 0.25 lbs, TL 2
    Wimple,
    /// Doublet (fitted jacket). BS 266. $40, 1.5 lbs, TL 3
    Doublet,

    // Containers (10 variants)
    /// Small Pouch. BS 288. $10, 0.2 lbs, 3 lbs capacity, TL 0
    SmallPouch,
    /// Pouch. BS 288. $10, 0.2 lbs, 6 lbs capacity, TL 0
    Pouch,
    /// Large Pouch. BS 288. $20, 0.5 lbs, 12 lbs capacity, TL 0
    LargePouch,
    /// Backpack (small). BS 288. $60, 3 lbs, 40 lbs capacity, TL 1
    SmallBackpack,
    /// Backpack. BS 288. $60, 3 lbs, 40 lbs capacity, TL 1
    Backpack,
    /// Large Backpack. BS 288. $100, 6 lbs, 60 lbs capacity, TL 2
    LargeBackpack,
    /// Sack (small). BS 288. $30, 3 lbs, 40 lbs capacity, TL 0
    SmallSack,
    /// Sack (large). BS 288. $50, 6 lbs, 80 lbs capacity, TL 0
    LargeSack,
    /// Chest (small). BS 288. $100, 10 lbs, 100 lbs capacity, TL 1
    SmallChest,
    /// Chest (large). BS 288. $300, 30 lbs, 200 lbs capacity, TL 1
    LargeChest,
    /// Belt Pouch. BS 288. $15, 0.25 lbs, 2 lbs capacity, TL 0
    BeltPouch,
    /// Barrel (40 gallon). BS 288. $50, 30 lbs, 150 lbs capacity, TL 0
    Barrel,
    /// Crate (wooden). BS 288. $40, 20 lbs, 120 lbs capacity, TL 1
    Crate,
    /// Trunk (travel). BS 288. $150, 25 lbs, 150 lbs capacity, TL 2
    Trunk,
    /// Money Belt. BS 288. $45, 0.5 lbs, 1 lb capacity, TL 2
    MoneyBelt,
    /// Quiver (arrow). BS 288. $10, 0.5 lbs, 3 lbs capacity, TL 0
    Quiver,
    /// Flask (hip). BS 288. $10, 0.5 lbs, 1 lb capacity, TL 1
    Flask,
    /// Basket (wicker). BS 288. $20, 2 lbs, 20 lbs capacity, TL 0
    Basket,
    /// Box (small). BS 288. $20, 2 lbs, 15 lbs capacity, TL 1
    BoxSmall,
    /// Haversack (military). BS 288. $75, 4 lbs, 50 lbs capacity, TL 3
    Haversack,

    // Tools (10 variants)
    /// Lockpicks. BS 289. $50, 0.1 lbs, TL 3
    Lockpicks,
    /// First Aid Kit. BS 289. $50, 2 lbs, TL 5
    FirstAidKit,
    /// Toolkit (basic). BS 289. $200, 10 lbs, TL 1
    Toolkit,
    /// Rope (3/8", 10 yards). BS 288. $5, 1.5 lbs, TL 0
    Rope,
    /// Grapnel. BS 288. $20, 2 lbs, TL 2
    Grapnel,
    /// Crowbar. BS 289. $20, 3 lbs, TL 1
    Crowbar,
    /// Hammer (tool). BS 289. $15, 2 lbs, TL 0
    Hammer,
    /// Saw. BS 289. $150, 3 lbs, TL 2
    Saw,
    /// Shovel. BS 289. $12, 6 lbs, TL 1
    Shovel,
    /// Magnifying Glass. BS 289. $100, 0.25 lbs, TL 4
    MagnifyingGlass,
    /// Pickaxe. BS 289. $15, 5 lbs, TL 0
    Pickaxe,
    /// Hatchet (tool). BS 289. $40, 2 lbs, TL 0
    ToolHatchet,
    /// Chisel (wood). BS 289. $5, 0.5 lbs, TL 0
    Chisel,
    /// File (metal). BS 289. $5, 0.5 lbs, TL 2
    File,
    /// Pliers. BS 289. $15, 1 lb, TL 3
    Pliers,
    /// Wrench. BS 289. $20, 2 lbs, TL 4
    Wrench,
    /// Screwdriver. BS 289. $5, 0.25 lbs, TL 3
    Screwdriver,
    /// Ladder (10 ft). BS 289. $30, 20 lbs, TL 0
    Ladder,
    /// Block and Tackle (pulley). BS 289. $50, 4 lbs, TL 1
    BlockAndTackle,
    /// Compass. BS 289. $50, 0.5 lbs, TL 4
    Compass,

    // Survival Gear (10 variants)
    /// Torch (burns 1 hour). BS 288. $3, 1 lb, TL 0
    Torch,
    /// Tent (1-person). BS 288. $50, 5 lbs, TL 0
    Tent,
    /// Large Tent (4-person). BS 288. $150, 30 lbs, TL 1
    LargeTent,
    /// Blanket. BS 288. $20, 4 lbs, TL 0
    Blanket,
    /// Rations (1 person-day). BS 288. $2, 0.5 lbs, TL 0
    Rations,
    /// Waterskin (1 gallon). BS 288. $10, 0.25 lbs, TL 0
    Waterskin,
    /// Bedroll. BS 288. $25, 3 lbs, TL 0
    Bedroll,
    /// Canteen (1 quart). BS 288. $10, 0.5 lbs, TL 3
    Canteen,
    /// Lantern (oil). BS 288. $20, 2 lbs, TL 2
    Lantern,
    /// Candle (burns 1 hour). BS 288. $0.5, 0.1 lbs, TL 1
    Candle,
    /// Tinder (fire starting). BS 288. $0.5, 0.1 lbs, TL 0
    Tinder,
    /// Flint and Steel. BS 288. $5, 0.5 lbs, TL 0
    FlintAndSteel,
    /// Sleeping Bag. BS 288. $40, 8 lbs, TL 5
    SleepingBag,
    /// Cooking Pot. BS 288. $20, 3 lbs, TL 0
    CookingPot,
    /// Matches (20). BS 288. $0.5, 0.1 lbs, TL 5
    Matches,
    /// Map (regional). BS 288. $20, 0.1 lbs, TL 3
    Map,
    /// Fishing Kit (line and hooks). BS 288. $5, 0.5 lbs, TL 0
    FishingKit,
    /// Snare Wire (10 yards). BS 288. $5, 0.5 lbs, TL 1
    SnareWire,
    /// Signal Whistle. BS 288. $5, 0.1 lbs, TL 2
    SignalWhistle,
    /// Tarp (waterproof). BS 288. $50, 5 lbs, TL 5
    Tarp,
}

impl Item {
    /// Returns item category (like `Spell::college`).
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Item, ItemCategory};
    ///
    /// let sword = Item::Broadsword;
    /// assert_eq!(sword.category(), ItemCategory::MeleeWeapon);
    /// ```
    #[instrument]
    pub fn category(&self) -> ItemCategory {
        debug!(item = ?self, "Getting item category");
        match self {
            Self::Axe
            | Self::Baton
            | Self::BrassKnuckles
            | Self::Broadsword
            | Self::Dagger
            | Self::Fist
            | Self::Flail
            | Self::GreatAxe
            | Self::Halberd
            | Self::Hatchet
            | Self::Javelin
            | Self::Kick
            | Self::Knife
            | Self::Kusari
            | Self::Lance
            | Self::LongSpear
            | Self::Mace
            | Self::MainGauche
            | Self::Morningstar
            | Self::Quarterstaff
            | Self::Rapier
            | Self::Saber
            | Self::Shortsword
            | Self::Smallsword
            | Self::Spear
            | Self::Staff
            | Self::TwoHandedSword
            | Self::Warhammer
            | Self::Katana
            | Self::Scimitar
            | Self::Cutlass
            | Self::Longsword
            | Self::BastardSword
            | Self::Wakizashi
            | Self::Nunchaku
            | Self::Sai
            | Self::Katar
            | Self::Tonfa
            | Self::Estoc
            | Self::Falchion
            | Self::Gladius
            | Self::Maul
            | Self::Pick
            | Self::Trident
            | Self::Whip
            | Self::CombatNet
            | Self::Bola
            | Self::WarFan
            | Self::Kopesh
            | Self::Claymore
            | Self::Tanto
            | Self::Chakram
            | Self::BattleAxe => ItemCategory::MeleeWeapon,

            Self::Bow
            | Self::Crossbow
            | Self::Pistol
            | Self::Rifle
            | Self::Shotgun
            | Self::Sling
            | Self::ThrowingKnife
            | Self::Longbow
            | Self::CompositeBow
            | Self::ShortBow
            | Self::LightCrossbow
            | Self::HeavyCrossbow
            | Self::ThrowingAxe
            | Self::Shuriken
            | Self::Dart
            | Self::Revolver
            | Self::SMG
            | Self::AssaultRifle
            | Self::SniperRifle
            | Self::Blowgun
            | Self::Atlatl
            | Self::Musket
            | Self::Derringer
            | Self::HuntingRifle
            | Self::MachineGun
            | Self::Grenade
            | Self::Flamethrower
            | Self::RocketLauncher
            | Self::StaffSling
            | Self::PelletBow
            | Self::HandCrossbow => ItemCategory::RangedWeapon,

            Self::NoArmor
            | Self::LeatherArmor
            | Self::Chainmail
            | Self::PlateArmor
            | Self::HeavyPlate
            | Self::ClothArmor
            | Self::LightLeather
            | Self::HeavyLeather
            | Self::ScaleMail
            | Self::SplintMail
            | Self::HalfPlate
            | Self::MailHauberk
            | Self::MailShirt
            | Self::Brigandine
            | Self::LamellarArmor
            | Self::BronzePlate
            | Self::LightScale
            | Self::BallisticVest
            | Self::TacticalVest
            | Self::FlakJacket
            | Self::SmallShield
            | Self::MediumShield
            | Self::LargeShield
            | Self::Buckler
            | Self::TowerShield => ItemCategory::Armor,

            Self::Clothing
            | Self::Boots
            | Self::Gloves
            | Self::Cloak
            | Self::Hat
            | Self::Belt
            | Self::Robe
            | Self::Sandals
            | Self::HeavyBoots
            | Self::ReinforcedGloves
            | Self::Tunic
            | Self::Shirt
            | Self::Pants
            | Self::Dress
            | Self::Cape
            | Self::Hood
            | Self::Scarf
            | Self::Vest
            | Self::Apron
            | Self::Tabard
            | Self::Breeches
            | Self::Stockings
            | Self::Jacket
            | Self::Coat
            | Self::Mantle
            | Self::FineGloves
            | Self::RidingBoots
            | Self::Sash
            | Self::Wimple
            | Self::Doublet => ItemCategory::Clothing,

            Self::SmallPouch
            | Self::Pouch
            | Self::LargePouch
            | Self::SmallBackpack
            | Self::Backpack
            | Self::LargeBackpack
            | Self::SmallSack
            | Self::LargeSack
            | Self::SmallChest
            | Self::LargeChest
            | Self::BeltPouch
            | Self::Barrel
            | Self::Crate
            | Self::Trunk
            | Self::MoneyBelt
            | Self::Quiver
            | Self::Flask
            | Self::Basket
            | Self::BoxSmall
            | Self::Haversack => ItemCategory::Containers,

            Self::Lockpicks
            | Self::FirstAidKit
            | Self::Toolkit
            | Self::Rope
            | Self::Grapnel
            | Self::Crowbar
            | Self::Hammer
            | Self::Saw
            | Self::Shovel
            | Self::MagnifyingGlass
            | Self::Pickaxe
            | Self::ToolHatchet
            | Self::Chisel
            | Self::File
            | Self::Pliers
            | Self::Wrench
            | Self::Screwdriver
            | Self::Ladder
            | Self::BlockAndTackle
            | Self::Compass => ItemCategory::Tools,

            Self::Torch
            | Self::Tent
            | Self::LargeTent
            | Self::Blanket
            | Self::Rations
            | Self::Waterskin
            | Self::Bedroll
            | Self::Canteen
            | Self::Lantern
            | Self::Candle
            | Self::Tinder
            | Self::FlintAndSteel
            | Self::SleepingBag
            | Self::CookingPot
            | Self::Matches
            | Self::Map
            | Self::FishingKit
            | Self::SnareWire
            | Self::SignalWhistle
            | Self::Tarp => ItemCategory::Survival,
        }
    }

    /// Returns base cost in GURPS $.
    ///
    /// Base cost is before quality modifiers. Use `cost()` for quality-adjusted cost.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Item;
    ///
    /// let sword = Item::Broadsword;
    /// assert_eq!(sword.base_cost().amount(), 500.0);
    /// ```
    #[instrument]
    pub fn base_cost(&self) -> Currency {
        debug!(item = ?self, "Getting item base cost");
        use super::{armor, clothing, containers, survival, tools, weapons_melee, weapons_ranged};

        match self.category() {
            ItemCategory::MeleeWeapon => weapons_melee::base_cost(self),
            ItemCategory::RangedWeapon => weapons_ranged::base_cost(self),
            ItemCategory::Armor => armor::base_cost(self),
            ItemCategory::Clothing => clothing::base_cost(self),
            ItemCategory::Containers => containers::base_cost(self),
            ItemCategory::Tools => tools::base_cost(self),
            ItemCategory::Survival => survival::base_cost(self),
            _ => {
                tracing::error!(item = ?self, category = ?self.category(), "Unimplemented category in base_cost");
                Currency::dollars(0.0)
            }
        }
    }

    /// Returns cost adjusted for quality.
    ///
    /// # GURPS Rules
    ///
    /// Quality multipliers: Cheap (×0.4), Good (×1.0), Fine (×4), VeryFine (×20).
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Item, Quality};
    ///
    /// let sword = Item::Broadsword;
    /// assert_eq!(sword.cost(Quality::Good).amount(), 500.0);
    /// assert_eq!(sword.cost(Quality::Fine).amount(), 2000.0);
    /// ```
    #[instrument]
    pub fn cost(&self, quality: Quality) -> Currency {
        debug!(item = ?self, quality = ?quality, "Getting quality-adjusted cost");
        let base = self.base_cost();
        Currency::dollars(base.amount() * quality.cost_multiplier())
    }

    /// Returns weight in lbs.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Item;
    ///
    /// let sword = Item::Broadsword;
    /// assert_eq!(sword.weight().amount(), 3.0);
    /// ```
    #[instrument]
    pub fn weight(&self) -> Weight {
        debug!(item = ?self, "Getting item weight");
        use super::{armor, clothing, containers, survival, tools, weapons_melee, weapons_ranged};

        match self.category() {
            ItemCategory::MeleeWeapon => weapons_melee::weight(self),
            ItemCategory::RangedWeapon => weapons_ranged::weight(self),
            ItemCategory::Armor => armor::weight(self),
            ItemCategory::Clothing => clothing::weight(self),
            ItemCategory::Containers => containers::weight(self),
            ItemCategory::Tools => tools::weight(self),
            ItemCategory::Survival => survival::weight(self),
            _ => {
                tracing::error!(item = ?self, category = ?self.category(), "Unimplemented category in weight");
                Weight::pounds(0.0)
            }
        }
    }

    /// Returns tech level (TL 0-12).
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Item;
    ///
    /// let sword = Item::Broadsword;
    /// assert_eq!(sword.tech_level().level(), 2);  // Medieval
    /// ```
    #[instrument]
    pub fn tech_level(&self) -> TechLevel {
        debug!(item = ?self, category = ?self.category(), "Getting item tech level");
        use super::{armor, clothing, containers, survival, tools, weapons_melee, weapons_ranged};

        match self.category() {
            ItemCategory::MeleeWeapon => weapons_melee::tech_level(self),
            ItemCategory::RangedWeapon => weapons_ranged::tech_level(self),
            ItemCategory::Armor => armor::tech_level(self),
            ItemCategory::Clothing => clothing::tech_level(self),
            ItemCategory::Containers => containers::tech_level(self),
            ItemCategory::Tools => tools::tech_level(self),
            ItemCategory::Survival => survival::tech_level(self),
            _ => {
                tracing::error!(item = ?self, category = ?self.category(), "Unimplemented category in tech_level");
                TechLevel::new(0)
            }
        }
    }

    // Category-specific methods (return Option)

    /// Returns weapon damage (melee/ranged weapons only).
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Item, WeaponDamage};
    ///
    /// let sword = Item::Broadsword;
    /// assert!(sword.weapon_damage().is_some());
    /// ```
    #[instrument]
    pub fn weapon_damage(&self) -> Option<WeaponDamage> {
        debug!(item = ?self, "Getting weapon damage");
        use super::{weapons_melee, weapons_ranged};

        match self.category() {
            ItemCategory::MeleeWeapon => Some(weapons_melee::damage(self)),
            ItemCategory::RangedWeapon => Some(weapons_ranged::damage(self)),
            _ => None,
        }
    }

    /// Returns reach (melee weapons only).
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Item;
    ///
    /// let spear = Item::Spear;
    /// assert!(spear.reach().is_some());
    /// ```
    #[instrument]
    pub fn reach(&self) -> Option<Reach> {
        debug!(item = ?self, "Getting weapon reach");
        use super::weapons_melee;

        match self.category() {
            ItemCategory::MeleeWeapon => Some(weapons_melee::reach(self)),
            _ => None,
        }
    }

    /// Returns parry modifier (melee weapons only).
    ///
    /// # GURPS Rules
    ///
    /// Parry = (Weapon Skill / 2) + 3 + parry_modifier
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Item;
    ///
    /// let rapier = Item::Rapier;
    /// assert_eq!(rapier.parry_modifier(), Some(1));
    /// ```
    #[instrument]
    pub fn parry_modifier(&self) -> Option<i32> {
        debug!(item = ?self, "Getting parry modifier");
        use super::weapons_melee;

        match self.category() {
            ItemCategory::MeleeWeapon => Some(weapons_melee::parry_modifier(self)),
            _ => None,
        }
    }

    /// Returns required skill (weapons only).
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Item, Skill};
    ///
    /// let sword = Item::Broadsword;
    /// assert_eq!(sword.required_skill(), Some(Skill::Broadsword));
    /// ```
    #[instrument]
    pub fn required_skill(&self) -> Option<Skill> {
        debug!(item = ?self, "Getting required skill");
        use super::{weapons_melee, weapons_ranged};

        match self.category() {
            ItemCategory::MeleeWeapon => Some(weapons_melee::required_skill(self)),
            ItemCategory::RangedWeapon => Some(weapons_ranged::required_skill(self)),
            _ => None,
        }
    }

    /// Returns accuracy (ranged weapons only).
    ///
    /// # GURPS Rules
    ///
    /// Accuracy adds to skill when aiming. Maximum bonus from multiple aim
    /// actions cannot exceed Acc.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Item;
    ///
    /// let rifle = Item::Rifle;
    /// assert_eq!(rifle.accuracy(), Some(5));
    /// ```
    #[instrument]
    pub fn accuracy(&self) -> Option<i32> {
        debug!(item = ?self, "Getting weapon accuracy");
        use super::weapons_ranged;

        match self.category() {
            ItemCategory::RangedWeapon => Some(weapons_ranged::accuracy(self)),
            _ => None,
        }
    }

    /// Returns damage resistance (armor only).
    ///
    /// # GURPS Rules
    ///
    /// DR reduces incoming damage before applying wound multipliers.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Item;
    ///
    /// let chainmail = Item::Chainmail;
    /// assert_eq!(chainmail.damage_resistance(), Some(4));
    /// ```
    #[instrument]
    pub fn damage_resistance(&self) -> Option<i32> {
        debug!(item = ?self, "Getting damage resistance");
        use super::armor;

        match self.category() {
            ItemCategory::Armor => Some(armor::damage_resistance(self)),
            _ => None,
        }
    }

    /// Returns capacity (containers only).
    ///
    /// # GURPS Rules
    ///
    /// Capacity indicates how much weight a container can hold in pounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Item;
    ///
    /// let backpack = Item::Backpack;
    /// assert_eq!(backpack.capacity().unwrap().amount(), 40.0);
    /// ```
    #[instrument]
    pub fn capacity(&self) -> Option<Capacity> {
        debug!(item = ?self, "Getting container capacity");
        use super::containers;

        match self.category() {
            ItemCategory::Containers => Some(containers::capacity(self)),
            _ => None,
        }
    }
}
