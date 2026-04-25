//! Character disadvantages providing points but imposing limitations.
//!
//! # GURPS Rules
//!
//! Disadvantages grant character points back (negative cost)
//! but impose behavioral restrictions, social penalties, or weaknesses.
//! Standard campaigns limit total disadvantages to -50 points.
//!
//! # Citations
//!
//! - BS 133-169 - Disadvantages
//! - BS 11 - Disadvantage limits
//!
//! # Examples
//!
//! ```
//! use valinoreth::{Disadvantage, SenseOfDuty};
//!
//! let disadvantage = Disadvantage::SenseOfDuty(SenseOfDuty::SmallGroup);
//! assert_eq!(disadvantage.cost(), -5);
//! ```

/// Character disadvantages providing points but imposing limitations.
///
/// # GURPS Rules
///
/// Disadvantages grant character points back (negative cost)
/// but impose behavioral restrictions, social penalties, or weaknesses.
/// Standard campaigns limit total disadvantages to -50 points.
///
/// # Citations
///
/// BS 133-169 - Disadvantage descriptions
/// BS 11 - Disadvantage limits
///
/// # Examples
///
/// ```
/// use valinoreth::{Disadvantage, SenseOfDuty};
///
/// let disadvantage = Disadvantage::SenseOfDuty(SenseOfDuty::SmallGroup);
/// assert_eq!(disadvantage.cost(), -5);
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
    derive_more::Display,
)]
pub enum Disadvantage {
    /// Personal code of behavior, variable cost. BS 127
    CodeOfHonor(usize),
    /// Fated for great deeds, -5 to -15 points per level. BS 131
    Destiny(usize),
    /// Obligation to organization or cause. BS 133
    Duty(Duty),
    /// Must obey the law and tell the truth, -10 points. BS 138
    Honesty,
    /// Put others first, -5 points. BS 153
    Selfless,
    /// Feel compulsion to protect group. BS 153
    SenseOfDuty(SenseOfDuty),
    /// Social penalty, -5 per level (max -20). BS 155
    SocialStigma(usize),
    /// Low social standing, -5 per level. BS 28
    Status(usize),
    /// -1 to reactions, must make Will roll to change mind, -5 points. BS 157
    Stubborn,
}

impl Disadvantage {
    /// Calculates the character point cost (negative) of this disadvantage.
    ///
    /// # GURPS Rules
    ///
    /// Disadvantages provide points back (negative cost).
    /// The magnitude represents the severity of the limitation.
    ///
    /// # Returns
    ///
    /// Negative character points (or zero for unimplemented variants).
    ///
    /// # Citations
    ///
    /// BS 133-169 - Individual disadvantage costs
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Disadvantage, Duty};
    ///
    /// let duty = Disadvantage::Duty(Duty::QuiteOften);
    /// assert_eq!(duty.cost(), -10);
    /// ```
    pub fn cost(&self) -> i64 {
        match self {
            // BS 127
            Self::CodeOfHonor(level) => -(*level as i64),
            // BS 131
            Self::Destiny(level) => -(*level as i64),
            Self::Duty(level) => level.cost(),
            // BS 138
            Self::Honesty => -10,
            // BS 153
            Self::Selfless => -5,
            Self::SenseOfDuty(level) => level.cost(),
            // -5 points per level, max level 4 BS 155
            Self::SocialStigma(level) => -(*level as i64 * 5),
            // BS 28
            Self::Status(level) => -(*level as i64 * 5),
            // BS 157
            Self::Stubborn => -5,
        }
    }
}

/// Compulsion to protect or aid a group.
///
/// # GURPS Rules
///
/// You feel compelled to protect members of the group,
/// even at risk to yourself. Scope determines point value.
///
/// # Citations
///
/// BS 153 - Sense of Duty
///
/// # Examples
///
/// ```
/// use valinoreth::SenseOfDuty;
///
/// let duty = SenseOfDuty::LargeGroup;
/// assert_eq!(duty.cost(), -10);
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
    derive_more::Display,
)]
pub enum SenseOfDuty {
    /// One person, -2 points. BS 153
    Individual,
    /// Small group (adventuring party), -5 points. BS 153
    SmallGroup,
    /// Large group (nation, religion), -10 points. BS 153
    LargeGroup,
    /// Entire intelligent race, -15 points. BS 153
    EntireRace,
    /// All living beings, -20 points. BS 153
    EveryLivingBeing,
}

impl SenseOfDuty {
    /// Returns the character point cost for this sense of duty.
    ///
    /// # Citations
    ///
    /// BS 153 - Sense of Duty costs
    pub fn cost(&self) -> i64 {
        match self {
            Self::Individual => -2,
            Self::SmallGroup => -5,
            Self::LargeGroup => -10,
            Self::EntireRace => -15,
            Self::EveryLivingBeing => -20,
        }
    }
}

/// Frequency of duty to an organization or cause.
///
/// # GURPS Rules
///
/// Duty represents an obligation that takes precedence over
/// personal desires. Frequency determines point value.
///
/// # Citations
///
/// BS 133 - Duty disadvantage
///
/// # Examples
///
/// ```
/// use valinoreth::Duty;
///
/// let duty = Duty::QuiteOften;
/// assert_eq!(duty.cost(), -10);
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
    derive_more::Display,
)]
pub enum Duty {
    /// 15 or less on 3d6, -15 points. BS 133
    AlmostAlways,
    /// 12 or less on 3d6, -10 points. BS 133
    QuiteOften,
    /// 9 or less on 3d6, -5 points. BS 133
    FairlyOften,
    /// 6 or less on 3d6, -2 points. BS 133
    QuiteRarely,
}

impl Duty {
    /// Returns the character point cost for this duty frequency.
    ///
    /// # Citations
    ///
    /// BS 133 - Duty costs by frequency
    pub fn cost(&self) -> i64 {
        match self {
            Self::AlmostAlways => -15,
            Self::QuiteOften => -10,
            Self::FairlyOften => -5,
            Self::QuiteRarely => -2,
        }
    }
}
