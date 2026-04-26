//! Core types for the items system.
//!
//! # GURPS Rules
//!
//! Items in GURPS have universal properties (cost, weight, tech level) and
//! category-specific properties (weapon damage, armor DR, container capacity).
//! Quality modifiers affect both cost and performance.
//!
//! # Citations
//!
//! BS 266-289 - Equipment chapter
//! BS 274 - Quality rules

use derive_more::{Display, From};
use tracing::{debug, instrument};

/// Item category for delegation (similar to SpellCollege).
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ItemCategory {
    /// Melee weapons (~80 items): swords, axes, polearms, unarmed
    MeleeWeapon,
    /// Ranged weapons (~60 items): bows, guns, thrown weapons
    RangedWeapon,
    /// Armor (~50 items): leather, mail, plate, shields
    Armor,
    /// Clothing (~40 items): boots, gloves, cloaks, robes
    Clothing,
    /// Tools (~50 items): lockpicks, medical, crafting
    Tools,
    /// Containers (~30 items): backpacks, pouches, chests
    Containers,
    /// Survival gear (~40 items): rope, torches, tents, rations
    Survival,
    /// Magic items (~50 items): enchanted items (future expansion)
    MagicItems,
}

/// Tech level (TL 0-12 in GURPS).
///
/// Represents the technology era an item comes from:
/// - TL 0: Stone Age
/// - TL 1: Bronze Age
/// - TL 2: Iron Age/Medieval
/// - TL 3: Age of Sail
/// - TL 4: Industrial Revolution
/// - TL 5: Mechanized Age
/// - TL 6: Atomic Age
/// - TL 7: Digital Age
/// - TL 8: Microtech Age (2000s)
/// - TL 9: Nanotech Age
/// - TL 10: Robotic Age
/// - TL 11: Age of Exotic Matter
/// - TL 12: Whatever Comes Next
///
/// # Citations
///
/// BS 22 - Tech Levels
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Display)]
#[display("TL{}", _0)]
pub struct TechLevel(u8);

impl TechLevel {
    /// Creates a new tech level.
    ///
    /// Valid range is 0-12 per GURPS rules.
    pub const fn new(level: u8) -> Self {
        Self(level)
    }

    /// Returns the numeric tech level.
    pub const fn level(&self) -> u8 {
        self.0
    }
}

/// Item quality affecting cost and performance.
///
/// # GURPS Rules
///
/// Quality modifiers represent the craftsmanship and materials used.
/// Better quality items cost more but provide bonuses to use.
///
/// # Citations
///
/// BS 274 - Quality
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum Quality {
    /// Cheap quality: ×0.4 cost, no bonus, BS 274
    Cheap,
    /// Good quality: ×1.0 cost, no bonus (default)
    #[default]
    Good,
    /// Fine quality: ×4 cost, +1 bonus, BS 274
    Fine,
    /// Very Fine quality: ×20 cost, +2 bonus, BS 274
    VeryFine,
}

impl Quality {
    /// Returns the cost multiplier for this quality.
    #[instrument]
    pub fn cost_multiplier(&self) -> f64 {
        debug!("Getting cost multiplier for {:?}", self);
        match self {
            Self::Cheap => 0.4,
            Self::Good => 1.0,
            Self::Fine => 4.0,
            Self::VeryFine => 20.0,
        }
    }

    /// Returns the skill bonus for this quality (weapons/tools).
    ///
    /// Applies to weapon skill rolls and tool use.
    #[instrument]
    pub fn skill_bonus(&self) -> i32 {
        debug!("Getting skill bonus for {:?}", self);
        match self {
            Self::Cheap => 0,
            Self::Good => 0,
            Self::Fine => 1,
            Self::VeryFine => 2,
        }
    }
}

/// Currency in GURPS $ (generic currency).
///
/// GURPS uses $ as an abstract currency unit, roughly equivalent to
/// 2000s USD for modern TL8 items.
///
/// # Citations
///
/// BS 27 - Starting Wealth
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Display, From)]
#[display("${:.2}", _0)]
pub struct Currency(f64);

impl Currency {
    /// Creates a currency value from dollars.
    pub const fn dollars(amount: f64) -> Self {
        Self(amount)
    }

    /// Returns the amount in dollars.
    pub const fn amount(&self) -> f64 {
        self.0
    }
}

/// Weight in pounds.
///
/// GURPS uses pounds as the standard weight unit.
///
/// # Citations
///
/// BS 18 - Encumbrance
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Display, From)]
#[display("{:.2} lbs", _0)]
pub struct Weight(f64);

impl Weight {
    /// Creates a weight value from pounds.
    pub const fn pounds(amount: f64) -> Self {
        Self(amount)
    }

    /// Returns the amount in pounds.
    pub const fn amount(&self) -> f64 {
        self.0
    }
}

/// Container capacity in pounds.
///
/// Represents how much weight a container can hold.
///
/// # Citations
///
/// BS 288 - Containers
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Display, From)]
#[display("{:.2} lbs capacity", _0)]
pub struct Capacity(f64);

impl Capacity {
    /// Creates a capacity value from pounds.
    pub const fn pounds(amount: f64) -> Self {
        Self(amount)
    }

    /// Returns the amount in pounds.
    pub const fn amount(&self) -> f64 {
        self.0
    }
}
