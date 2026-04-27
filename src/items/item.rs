//! Top-level Item enum wrapping all item categories.
//!
//! This module provides a type-safe hierarchical item system where each item
//! category is represented by its own enum, and the compiler enforces
//! exhaustive matching. This eliminates the need for runtime type checking
//! and prevents silent errors from wrong-type access.

use crate::{Capacity, Currency, Quality, Reach, Skill, TechLevel, WeaponDamage, Weight};

// Import category enums from items module (not crate root to avoid naming conflicts)
use super::{Armor, Clothing, Container, MeleeWeapon, RangedWeapon, SurvivalGear, Tool};

/// Top-level item enum wrapping all item categories.
///
/// Each variant contains a category-specific enum, providing compile-time
/// type safety and eliminating the need for runtime type checking.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    /// Melee weapon (swords, axes, etc.)
    MeleeWeapon(MeleeWeapon),
    /// Ranged weapon (bows, guns, etc.)
    RangedWeapon(RangedWeapon),
    /// Armor and shields
    Armor(Armor),
    /// Clothing
    Clothing(Clothing),
    /// Containers (bags, pouches, chests, etc.)
    Container(Container),
    /// Tools and equipment
    Tool(Tool),
    /// Survival gear (tents, rations, etc.)
    Survival(SurvivalGear),
}

impl Item {
    // ========================================================================
    // Universal Properties (all items have these)
    // ========================================================================

    /// Returns base cost for this item.
    pub fn base_cost(&self) -> Currency {
        match self {
            Self::MeleeWeapon(w) => w.base_cost(),
            Self::RangedWeapon(w) => w.base_cost(),
            Self::Armor(a) => a.base_cost(),
            Self::Clothing(c) => c.base_cost(),
            Self::Container(c) => c.base_cost(),
            Self::Tool(t) => t.base_cost(),
            Self::Survival(s) => s.base_cost(),
        }
    }

    /// Returns weight for this item.
    pub fn weight(&self) -> Weight {
        match self {
            Self::MeleeWeapon(w) => w.weight(),
            Self::RangedWeapon(w) => w.weight(),
            Self::Armor(a) => a.weight(),
            Self::Clothing(c) => c.weight(),
            Self::Container(c) => c.weight(),
            Self::Tool(t) => t.weight(),
            Self::Survival(s) => s.weight(),
        }
    }

    /// Returns tech level for this item.
    pub fn tech_level(&self) -> TechLevel {
        match self {
            Self::MeleeWeapon(w) => w.tech_level(),
            Self::RangedWeapon(w) => w.tech_level(),
            Self::Armor(a) => a.tech_level(),
            Self::Clothing(c) => c.tech_level(),
            Self::Container(c) => c.tech_level(),
            Self::Tool(t) => t.tech_level(),
            Self::Survival(s) => s.tech_level(),
        }
    }

    /// Returns quality-adjusted cost for this item.
    ///
    /// The final cost is the base cost multiplied by the quality multiplier.
    pub fn cost(&self, quality: Quality) -> Currency {
        let base = self.base_cost();
        Currency::dollars(base.amount() * quality.cost_multiplier())
    }

    // ========================================================================
    // Category-Specific Properties (return Option<T>)
    // ========================================================================

    /// Returns weapon damage if this is a weapon, None otherwise.
    pub fn weapon_damage(&self) -> Option<WeaponDamage> {
        match self {
            Self::MeleeWeapon(w) => Some(w.damage()),
            Self::RangedWeapon(w) => Some(w.damage()),
            _ => None,
        }
    }

    /// Returns weapon reach if this is a melee weapon, None otherwise.
    pub fn reach(&self) -> Option<Reach> {
        match self {
            Self::MeleeWeapon(w) => Some(w.reach()),
            _ => None,
        }
    }

    /// Returns parry modifier if this is a melee weapon, None otherwise.
    pub fn parry_modifier(&self) -> Option<i32> {
        match self {
            Self::MeleeWeapon(w) => Some(w.parry_modifier()),
            _ => None,
        }
    }

    /// Returns accuracy if this is a ranged weapon, None otherwise.
    pub fn accuracy(&self) -> Option<i32> {
        match self {
            Self::RangedWeapon(w) => Some(w.accuracy()),
            _ => None,
        }
    }

    /// Returns required skill if this is a weapon, None otherwise.
    pub fn required_skill(&self) -> Option<Skill> {
        match self {
            Self::MeleeWeapon(w) => Some(w.required_skill()),
            Self::RangedWeapon(w) => Some(w.required_skill()),
            _ => None,
        }
    }

    /// Returns damage resistance if this is armor, None otherwise.
    pub fn damage_resistance(&self) -> Option<i32> {
        match self {
            Self::Armor(a) => Some(a.damage_resistance()),
            _ => None,
        }
    }

    /// Returns capacity if this is a container, None otherwise.
    pub fn capacity(&self) -> Option<Capacity> {
        match self {
            Self::Container(c) => Some(c.capacity()),
            _ => None,
        }
    }
}
