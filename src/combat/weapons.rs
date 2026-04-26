//! Weapon combat mechanics.
//!
//! # GURPS Rules
//!
//! This module defines core combat mechanics for weapons: damage types,
//! reach specifications, and damage calculations. Actual weapon items are
//! enumerated in the `items` module.
//!
//! # Citations
//!
//! - BS 269-271 - Damage types and effects
//! - BS 271-276 - Melee weapons table
//! - BS 276-278 - Ranged weapons table

use crate::DieLevel;
use tracing::instrument;

/// Weapon damage specification.
///
/// # GURPS Rules
///
/// Damage can be ST-based (thrust/swing) or fixed dice.
///
/// # Citations
///
/// BS 269-271 - Damage types
///
/// # Examples
///
/// ```
/// use valinoreth::{WeaponDamage, DamageType};
///
/// let damage = WeaponDamage::Swing { modifier: 1, damage_type: DamageType::Cutting };
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WeaponDamage {
    /// Thrust-based damage with modifier
    Thrust {
        /// Damage modifier (e.g., +1, -2)
        modifier: i32,
        /// Type of damage
        damage_type: DamageType,
    },
    /// Swing-based damage with modifier
    Swing {
        /// Damage modifier (e.g., +1, -2)
        modifier: i32,
        /// Type of damage
        damage_type: DamageType,
    },
    /// Fixed dice damage (for ranged weapons)
    Fixed {
        /// Damage dice (e.g., 2d+1)
        dice: DieLevel,
        /// Type of damage
        damage_type: DamageType,
    },
}

/// Damage type and wound multiplier.
///
/// # GURPS Rules
///
/// Different damage types have different wound multipliers when
/// applied to hit locations.
///
/// # Citations
///
/// BS 269-271 - Damage types and multipliers
///
/// # Examples
///
/// ```
/// use valinoreth::DamageType;
///
/// let cutting = DamageType::Cutting;
/// assert_eq!(cutting.torso_multiplier(), 1.5);
/// ```
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    strum::EnumIter,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum DamageType {
    /// Crushing damage, ×1 to torso. BS 269
    Crushing,
    /// Cutting damage, ×1.5 to torso. BS 270
    Cutting,
    /// Impaling damage, ×2 to torso. BS 270
    Impaling,
    /// Small piercing damage, ×0.5 to torso. BS 270
    PiercingSmall,
    /// Normal piercing damage, ×1 to torso. BS 270
    Piercing,
    /// Large piercing damage, ×1.5 to torso. BS 270
    PiercingLarge,
}

impl DamageType {
    /// Returns wound multiplier for torso hits.
    ///
    /// # GURPS Rules
    ///
    /// Torso multipliers vary by damage type.
    ///
    /// # Citations
    ///
    /// BS 269-271 - Wound multipliers
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::DamageType;
    ///
    /// assert_eq!(DamageType::Cutting.torso_multiplier(), 1.5);
    /// assert_eq!(DamageType::Crushing.torso_multiplier(), 1.0);
    /// assert_eq!(DamageType::Impaling.torso_multiplier(), 2.0);
    /// ```
    #[instrument]
    pub fn torso_multiplier(&self) -> f64 {
        match self {
            Self::Crushing => 1.0,
            Self::Cutting => 1.5,
            Self::Impaling => 2.0,
            Self::PiercingSmall => 0.5,
            Self::Piercing => 1.0,
            Self::PiercingLarge => 1.5,
        }
    }
}

/// Weapon reach specification.
///
/// # GURPS Rules
///
/// Reach indicates how far you can strike with a weapon.
/// C means close combat (grappling), numbers are meters.
///
/// # Citations
///
/// BS 271 - Reach definition
///
/// # Examples
///
/// ```
/// use valinoreth::Reach;
///
/// let spear_reach = Reach::OneTwo;  // Can strike at 1 or 2 meters
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, derive_more::Display)]
pub enum Reach {
    /// Close combat only (C)
    #[display("C")]
    Close,
    /// Close or 1 meter (C,1)
    #[display("C,1")]
    CloseOne,
    /// 1 meter only (1)
    #[display("1")]
    One,
    /// 1 or 2 meters (1,2)
    #[display("1,2")]
    OneTwo,
    /// 1, 2, or 3 meters (1-3)
    #[display("1-3")]
    OneThree,
    /// 2 or 3 meters (2,3)
    #[display("2,3")]
    TwoThree,
    /// 3 meters only (3)
    #[display("3")]
    Three,
}
