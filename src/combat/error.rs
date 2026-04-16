//! Combat error types.
//!
//! Provides error handling for combat operations with location tracking.

use derive_more::{Display, Error};

/// Specific combat error conditions.
///
/// # GURPS Rules
///
/// Combat errors occur when rules are violated or invalid states are encountered.
///
/// # Examples
///
/// ```
/// use valinoreth::CombatErrorKind;
///
/// let error = CombatErrorKind::InvalidSkillLevel { skill: 25 };
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Display)]
pub enum CombatErrorKind {
    /// Skill level out of valid range (typically 3-20+)
    #[display("Invalid skill level: {}", skill)]
    InvalidSkillLevel {
        /// The invalid skill level
        skill: i32,
    },

    /// Attack roll value out of range (must be 3-18 for 3d6)
    #[display("Invalid attack roll: {} (must be 3-18)", roll)]
    InvalidAttackRoll {
        /// The invalid roll value
        roll: i32,
    },

    /// Defense value is negative or invalid
    #[display("Invalid defense value: {}", defense)]
    InvalidDefense {
        /// The invalid defense value
        defense: i32,
    },

    /// Damage resistance is negative
    #[display("Invalid DR: {} (cannot be negative)", dr)]
    InvalidDR {
        /// The invalid DR value
        dr: i32,
    },

    /// Weapon not ready for use
    #[display("Weapon not ready")]
    WeaponNotReady,

    /// Target out of reach
    #[display("Target out of reach: distance {} exceeds reach {}", distance, reach)]
    TargetOutOfReach {
        /// Distance to target
        distance: i32,
        /// Weapon reach
        reach: i32,
    },

    /// No valid target
    #[display("No valid target")]
    NoTarget,
}

/// Combat error with location tracking.
///
/// # Examples
///
/// ```
/// use valinoreth::{CombatError, CombatErrorKind};
///
/// let error = CombatError::new(CombatErrorKind::WeaponNotReady);
/// ```
#[derive(Debug, Clone, Display, Error)]
#[display("Combat error: {} at {}:{}", kind, file, line)]
pub struct CombatError {
    /// The specific error kind
    pub kind: CombatErrorKind,
    /// Line number where error occurred
    pub line: u32,
    /// Source file where error occurred
    pub file: &'static str,
}

impl CombatError {
    /// Creates a new combat error with caller location tracking.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{CombatError, CombatErrorKind};
    ///
    /// let error = CombatError::new(CombatErrorKind::NoTarget);
    /// ```
    #[track_caller]
    pub fn new(kind: CombatErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        tracing::error!(error_kind = %kind, "Combat error occurred");
        Self {
            kind,
            line: loc.line(),
            file: loc.file(),
        }
    }
}
