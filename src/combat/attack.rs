//! Attack roll resolution and modifiers.
//!
//! # GURPS Rules
//!
//! Attacks use 3d6 roll-under mechanic. Success occurs when rolling
//! ≤ effective skill (base skill + modifiers).
//!
//! Critical success: 3-4, or 5-6 if skill 15+
//! Critical failure: 17-18, or 10+ over skill
//!
//! # Citations
//!
//! - BS 356 - Attack rolls
//! - BS 357 - Critical success
//! - BS 358 - Critical failure
//! - BS 398-399 - Combat modifiers

use crate::{Dice, Random};
use derive_getters::Getters;
use tracing::{debug, error, info, instrument};

/// Result of an attack roll in GURPS combat.
///
/// # GURPS Rules
///
/// Attacks use 3d6 roll-under mechanic. Roll against effective skill
/// (base skill + modifiers). Critical success on 3-4, or 5-6 if skill 15+.
/// Critical failure on 17-18, or 10+ over skill.
///
/// # Citations
///
/// - BS 356 - Attack rolls
/// - BS 357 - Critical hits
/// - BS 358 - Critical misses
///
/// # Examples
///
/// ```
/// use valinoreth::{AttackRoll, Random};
///
/// let mut rng = Random::from_seed(42).unwrap();
/// let roll = AttackRoll::execute(14, &mut rng);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Getters, derive_new::new)]
pub struct AttackRoll {
    /// The 3d6 dice roll
    dice: Dice,
    /// Effective skill level after modifiers
    effective_skill: i32,
    /// Outcome of the attack
    result: AttackResult,
}

impl AttackRoll {
    /// Performs an attack roll against effective skill.
    ///
    /// # GURPS Rules
    ///
    /// Rolls 3d6 and compares to effective skill. Determines if attack
    /// succeeds, fails, or achieves critical result.
    ///
    /// # Citations
    ///
    /// BS 356 - Attack procedure
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{AttackRoll, Random};
    ///
    /// let mut rng = Random::from_seed(42).unwrap();
    /// let roll = AttackRoll::execute(14, &mut rng);
    /// ```
    #[instrument(skip(random), fields(effective_skill))]
    pub fn execute(effective_skill: i32, random: &mut Random) -> Self {
        debug!("Executing attack roll");

        let dice = Dice::from_random(random);
        let roll = *dice.sum() as i32;
        let result = Self::determine_result(roll, effective_skill);

        debug!(roll, effective_skill, ?result, "Attack roll completed");

        Self::new(dice, effective_skill, result)
    }

    /// Determines attack result from roll and skill.
    ///
    /// # GURPS Rules
    ///
    /// - Critical success: Roll 3-4, or 5-6 if skill 15+
    /// - Critical failure: Roll 17-18, or 10+ over skill
    /// - Success: Roll ≤ skill
    /// - Failure: Roll > skill
    ///
    /// # Citations
    ///
    /// - BS 357 - Critical success thresholds
    /// - BS 358 - Critical failure thresholds
    #[instrument(fields(roll, skill))]
    fn determine_result(roll: i32, skill: i32) -> AttackResult {
        debug!("Determining attack result");

        // BS 357 - Critical success thresholds
        if roll <= 4 || (roll <= 6 && skill >= 15) {
            info!(roll, skill, "Critical success");
            return AttackResult::CriticalSuccess {
                margin: skill - roll,
            };
        }

        // BS 358 - Critical failure thresholds
        if roll >= 18 || (roll >= 17 && skill < 16) || roll >= skill + 10 {
            error!(roll, skill, "Critical failure");
            return AttackResult::CriticalFailure {
                margin: roll - skill,
            };
        }

        // Regular success/failure
        if roll <= skill {
            info!(roll, skill, "Attack succeeded");
            AttackResult::Success {
                margin: skill - roll,
            }
        } else {
            debug!(roll, skill, "Attack failed");
            AttackResult::Failure {
                margin: roll - skill,
            }
        }
    }
}

/// Outcome of an attack attempt.
///
/// # GURPS Rules
///
/// Attack results determine whether the attack hits and may trigger
/// special effects (critical hits/misses).
///
/// # Citations
///
/// BS 356-358 - Attack results
///
/// # Examples
///
/// ```
/// use valinoreth::AttackResult;
///
/// let result = AttackResult::Success { margin: 5 };
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AttackResult {
    /// Roll 3-4, or 5-6 if skill 15+. BS 357
    CriticalSuccess {
        /// Margin of success (skill - roll)
        margin: i32,
    },
    /// Roll ≤ effective skill
    Success {
        /// Margin of success (skill - roll)
        margin: i32,
    },
    /// Roll > effective skill
    Failure {
        /// Margin of failure (roll - skill)
        margin: i32,
    },
    /// Roll 17-18, or 10+ over skill. BS 358
    CriticalFailure {
        /// Margin of failure (roll - skill)
        margin: i32,
    },
}

/// Aggregates combat modifiers for attack rolls.
///
/// # GURPS Rules
///
/// Attack rolls are modified by hit location, range, speed, posture, etc.
/// Modifiers stack additively.
///
/// # Citations
///
/// - BS 398-399 - Combat modifiers
/// - BS 548-551 - Speed/Range table
///
/// # Examples
///
/// ```
/// use valinoreth::CombatModifiers;
///
/// let modifiers = CombatModifiers::builder()
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone, Default, Getters, derive_builder::Builder)]
#[builder(setter(into))]
pub struct CombatModifiers {
    /// List of individual modifiers
    #[builder(default)]
    modifiers: Vec<Modifier>,
}

impl CombatModifiers {
    /// Creates a new builder for combat modifiers.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::CombatModifiers;
    ///
    /// let modifiers = CombatModifiers::builder()
    ///     .modifiers(vec![])
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder() -> CombatModifiersBuilder {
        CombatModifiersBuilder::default()
    }

    /// Calculates total modifier value.
    ///
    /// # Citations
    ///
    /// BS 398 - Modifiers add together
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{CombatModifiers, Modifier};
    ///
    /// let mods = vec![
    ///     Modifier::new("Range", -2, "BS 550"),
    ///     Modifier::new("Target posture", -2, "BS 399"),
    /// ];
    /// let modifiers = CombatModifiers::builder()
    ///     .modifiers(mods)
    ///     .build()
    ///     .unwrap();
    /// assert_eq!(modifiers.total(), -4);
    /// ```
    #[instrument(skip(self))]
    pub fn total(&self) -> i32 {
        debug!("Calculating total modifiers");

        let total: i32 = self.modifiers.iter().map(|m| m.value()).sum();

        debug!(
            total,
            count = self.modifiers.len(),
            "Calculated modifier total"
        );
        total
    }
}

/// Individual combat modifier.
///
/// # GURPS Rules
///
/// Modifiers represent situational factors affecting combat:
/// range, cover, posture, speed, darkness, etc.
///
/// # Citations
///
/// BS 398-399 - Modifier sources
///
/// # Examples
///
/// ```
/// use valinoreth::Modifier;
///
/// let modifier = Modifier::new("Darkness", -3, "BS 394");
/// assert_eq!(*modifier.value(), -3);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Getters, derive_new::new)]
pub struct Modifier {
    /// Descriptive name of the modifier
    name: &'static str,
    /// Numeric modifier value (positive or negative)
    value: i32,
    /// Citation to GURPS Basic Set
    citation: &'static str,
}
