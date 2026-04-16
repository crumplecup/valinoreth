//! Body hit locations and targeting modifiers.
//!
//! # GURPS Rules
//!
//! Combat targeting allows attacking specific body locations for
//! tactical advantages. Each location has:
//! - Hit penalty modifier (to-hit penalty)
//! - Wound multiplier (damage effects)
//! - Special effects (blinding, crippling, etc.)
//!
//! # Citations
//!
//! - BS 398-400 - Hit Location rules
//! - BS 399 - Hit Location Table
//!
//! # Examples
//!
//! ```
//! use valinoreth::{BodyArea, BodyLocation};
//!
//! let target = BodyArea::Head;
//! let penalty = target.to_hit(); // -5
//! ```

/// Trait for body hit locations.
///
/// # GURPS Rules
///
/// Provides common interface for all hit location systems:
/// - Random location from 3d6 roll
/// - To-hit penalty for targeting
///
/// # Citations
///
/// BS 399 - Hit location mechanics
pub trait BodyLocation {
    /// Associated type for specific location enum
    type Location;

    /// Determines hit location from 3d6 roll (3-18).
    ///
    /// # Citations
    ///
    /// BS 399 - Random hit location table
    fn from_roll(roll: usize) -> Self::Location;

    /// Returns to-hit penalty for targeting this location.
    ///
    /// # Citations
    ///
    /// BS 399 - Hit location modifiers
    fn to_hit(&self) -> isize;
}

/// Major body areas for hit location.
///
/// # GURPS Rules
///
/// The four major body areas provide general targeting zones.
/// Each area can be further subdivided into specific locations
/// for more detailed combat.
///
/// # Citations
///
/// BS 399 - Hit Location Table
///
/// # Examples
///
/// ```
/// use valinoreth::{BodyArea, BodyLocation};
///
/// let area = BodyArea::from_roll(10); // Torso
/// assert_eq!(area.to_hit(), 0);
/// ```
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    strum::EnumIter,
    derive_more::Display,
)]
pub enum BodyArea {
    /// Head area, -5 to hit. BS 399
    Head,
    /// Torso area, no penalty. BS 399
    Torso,
    /// Arms area, -2 to hit. BS 399
    Arms,
    /// Legs area, -2 to hit. BS 399
    Legs,
}

impl BodyLocation for BodyArea {
    type Location = Self;

    fn from_roll(roll: usize) -> Self {
        match roll {
            0..5 => Self::Head,
            5..8 => Self::Arms,
            8..14 => Self::Torso,
            14..17 => Self::Legs,
            17.. => Self::Head,
        }
    }

    fn to_hit(&self) -> isize {
        match self {
            Self::Head => -5,
            Self::Torso => 0, // BS 399
            Self::Arms => -2, // BS 399
            Self::Legs => -2, // BS 399
        }
    }
}

/// Specific head hit locations.
///
/// # GURPS Rules
///
/// Head shots deal extra damage and can cause special effects:
/// - Skull: ×4 damage multiplier on brain
/// - Eyes: Blinding, ×4 damage
/// - Face: ×1.5 damage
/// - Neck: ×2 damage, can sever arteries
///
/// # Citations
///
/// BS 399 - Head hit locations
/// BS 400 - Head wound effects
///
/// # Examples
///
/// ```
/// use valinoreth::{Head, BodyLocation};
///
/// let target = Head::Eyes;
/// assert_eq!(target.to_hit(), -9); // Very difficult shot
/// ```
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    strum::EnumIter,
    derive_more::Display,
)]
pub enum Head {
    /// Top/back of head, -7 to hit. BS 399
    Skull,
    /// Eyes, -9 to hit, can blind. BS 399
    Eyes,
    /// Face, -5 to hit. BS 399
    Face,
    /// Neck, -5 to hit, can sever arteries. BS 399
    Neck,
}

impl BodyLocation for Head {
    type Location = Self;

    fn from_roll(roll: usize) -> Self {
        match roll {
            0..5 => Self::Eyes,
            5..8 => Self::Face,
            8..14 => Self::Skull,
            14..17 => Self::Neck,
            17.. => Self::Eyes,
        }
    }

    fn to_hit(&self) -> isize {
        match self {
            Self::Skull => -7, // BS 399
            Self::Eyes => -9,  // BS 399
            Self::Face => -5,  // BS 399
            Self::Neck => -5,  // BS 399
        }
    }
}

/// Specific torso hit locations.
///
/// # GURPS Rules
///
/// Torso contains vital organs and major arteries:
/// - Chest: ×1 damage
/// - Abdomen: ×1 damage
/// - Vitals: ×3 damage, instant incapacitation risk
/// - Groin: ×1 damage, stunning effects
///
/// # Citations
///
/// BS 399 - Torso hit locations
/// BS 400 - Torso wound effects
///
/// # Examples
///
/// ```
/// use valinoreth::{Torso, BodyLocation};
///
/// let target = Torso::Vitals;
/// assert_eq!(target.to_hit(), -3); // Heart/lungs
/// ```
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    strum::EnumIter,
    derive_more::Display,
)]
pub enum Torso {
    /// Chest, -2 to hit. BS 399
    Chest,
    /// Abdomen, -2 to hit. BS 399
    Abdomen,
    /// Heart/lungs, -3 to hit, ×3 damage. BS 399
    Vitals,
    /// Groin, -3 to hit, stunning. BS 399
    Groin,
}

impl BodyLocation for Torso {
    type Location = Self;

    fn from_roll(roll: usize) -> Self {
        match roll {
            0..5 => Self::Groin,
            5..8 => Self::Abdomen,
            8..14 => Self::Chest,
            14..17 => Self::Vitals,
            17.. => Self::Groin,
        }
    }

    fn to_hit(&self) -> isize {
        match self {
            Self::Chest => -2,
            Self::Abdomen => -2,
            Self::Vitals => -3, // BS 399
            Self::Groin => -3,  // BS 399
        }
    }
}

/// Specific arm hit locations.
///
/// # GURPS Rules
///
/// Arm hits can cripple limbs and disable weapon use:
/// - Sufficient damage cripples the arm
/// - Crippled arms drop held items
/// - Reduced damage (×1 for most damage types)
///
/// # Citations
///
/// BS 399 - Arm hit locations
/// BS 400 - Limb crippling
///
/// # Examples
///
/// ```
/// use valinoreth::{Arms, BodyLocation};
///
/// let target = Arms::Hands;
/// assert_eq!(target.to_hit(), -4); // Small, precise target
/// ```
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    strum::EnumIter,
    derive_more::Display,
)]
pub enum Arms {
    /// Shoulders, -2 to hit. BS 399
    Shoulders,
    /// Upper arm, -2 to hit. BS 399
    Upper,
    /// Forearms, -2 to hit. BS 399
    Forearms,
    /// Elbows, -3 to hit
    Elbows,
    /// Hands, -4 to hit. BS 399
    Hands,
}

impl BodyLocation for Arms {
    type Location = Self;

    fn from_roll(roll: usize) -> Self {
        match roll {
            0..5 => Self::Hands,
            5..8 => Self::Shoulders,
            8..14 => Self::Forearms,
            14..17 => Self::Upper,
            17.. => Self::Elbows,
        }
    }

    fn to_hit(&self) -> isize {
        match self {
            Self::Shoulders => -2, // BS 399
            Self::Upper => -2,     // BS 399
            Self::Forearms => -2,  // BS 399
            Self::Elbows => -3,
            Self::Hands => -4, // BS 399
        }
    }
}

/// Specific leg hit locations.
///
/// # GURPS Rules
///
/// Leg hits can cripple limbs and reduce mobility:
/// - Sufficient damage cripples the leg
/// - Crippled legs reduce Move and Dodge
/// - One crippled leg: Move and Dodge halved
/// - Both legs crippled: Cannot stand
///
/// # Citations
///
/// BS 399 - Leg hit locations
/// BS 400 - Limb crippling effects
///
/// # Examples
///
/// ```
/// use valinoreth::{Legs, BodyLocation};
///
/// let target = Legs::Knees;
/// assert_eq!(target.to_hit(), -3); // Joint targeting
/// ```
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    strum::EnumIter,
    derive_more::Display,
)]
pub enum Legs {
    /// Thighs, -2 to hit. BS 399
    Thighs,
    /// Knees, -3 to hit
    Knees,
    /// Shins, -2 to hit. BS 399
    Shins,
    /// Feet, -4 to hit. BS 399
    Feet,
}

impl BodyLocation for Legs {
    type Location = Self;

    fn from_roll(roll: usize) -> Self {
        match roll {
            0..5 => Self::Feet,
            5..8 => Self::Knees,
            8..14 => Self::Thighs,
            14..17 => Self::Shins,
            17.. => Self::Feet,
        }
    }

    fn to_hit(&self) -> isize {
        match self {
            Self::Thighs => -2, // BS 399
            Self::Knees => -3,
            Self::Shins => -2, // BS 399
            Self::Feet => -4,  // BS 399
        }
    }
}
