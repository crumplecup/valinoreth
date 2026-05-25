//! Character special features collection.
//!
//! # GURPS Rules
//!
//! Characters gain advantages and disadvantages that define
//! their unique capabilities and limitations.
//!
//! # Citations
//!
//! - BS 100-169 - Advantages and disadvantages

use crate::{Advantage, Disadvantage, Perk};

/// Collection of character advantages, disadvantages, and perks.
///
/// # GURPS Rules
///
/// Organizes a character's special features. Standard campaigns
/// limit disadvantages to -50 points.
///
/// # Citations
///
/// BS 11 - Disadvantage limits
///
/// # Examples
///
/// ```
/// use valinoreth::SpecialFeatures;
///
/// let features = SpecialFeatures::new(vec![], vec![], vec![]);
/// ```
#[derive(Debug, Default, Clone, derive_new::new)]
#[cfg_attr(not(creusot), derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(not(creusot), derive(PartialEq, Eq, PartialOrd, Ord, Hash))]
pub struct SpecialFeatures {
    /// Character advantages
    advantages: Vec<Advantage>,
    /// Character disadvantages (negative point values)
    disadvantages: Vec<Disadvantage>,
    /// Character perks (1-point advantages)
    perks: Vec<Perk>,
}
