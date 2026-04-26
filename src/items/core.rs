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
    Capacity, Currency, ItemCategory, Quality, Reach, Skill, TechLevel, Weight, WeaponDamage,
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
            | Self::Warhammer => ItemCategory::MeleeWeapon,

            Self::Bow
            | Self::Crossbow
            | Self::Pistol
            | Self::Rifle
            | Self::Shotgun
            | Self::Sling
            | Self::ThrowingKnife => ItemCategory::RangedWeapon,

            Self::NoArmor
            | Self::LeatherArmor
            | Self::Chainmail
            | Self::PlateArmor
            | Self::HeavyPlate => ItemCategory::Armor,

            Self::Clothing
            | Self::Boots
            | Self::Gloves
            | Self::Cloak
            | Self::Hat
            | Self::Belt
            | Self::Robe
            | Self::Sandals
            | Self::HeavyBoots
            | Self::ReinforcedGloves => ItemCategory::Clothing,

            Self::SmallPouch
            | Self::Pouch
            | Self::LargePouch
            | Self::SmallBackpack
            | Self::Backpack
            | Self::LargeBackpack
            | Self::SmallSack
            | Self::LargeSack
            | Self::SmallChest
            | Self::LargeChest => ItemCategory::Containers,
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
        use super::{armor, clothing, containers, weapons_melee, weapons_ranged};

        match self.category() {
            ItemCategory::MeleeWeapon => weapons_melee::base_cost(self),
            ItemCategory::RangedWeapon => weapons_ranged::base_cost(self),
            ItemCategory::Armor => armor::base_cost(self),
            ItemCategory::Clothing => clothing::base_cost(self),
            ItemCategory::Containers => containers::base_cost(self),
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
        use super::{armor, clothing, containers, weapons_melee, weapons_ranged};

        match self.category() {
            ItemCategory::MeleeWeapon => weapons_melee::weight(self),
            ItemCategory::RangedWeapon => weapons_ranged::weight(self),
            ItemCategory::Armor => armor::weight(self),
            ItemCategory::Clothing => clothing::weight(self),
            ItemCategory::Containers => containers::weight(self),
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
        use super::{armor, clothing, containers, weapons_melee, weapons_ranged};

        match self.category() {
            ItemCategory::MeleeWeapon => weapons_melee::tech_level(self),
            ItemCategory::RangedWeapon => weapons_ranged::tech_level(self),
            ItemCategory::Armor => armor::tech_level(self),
            ItemCategory::Clothing => clothing::tech_level(self),
            ItemCategory::Containers => containers::tech_level(self),
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
