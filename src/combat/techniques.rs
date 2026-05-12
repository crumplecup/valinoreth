//! Combat techniques and martial arts maneuvers.
//!
//! # GURPS Rules
//!
//! Techniques represent specialized combat training that builds on basic skills.
//! Each technique:
//! - Has a difficulty level (Average or Hard)
//! - Requires one or more prerequisite skills
//! - Has a default penalty relative to the prerequisite
//! - Has a maximum bonus limit
//! - May be cinematic or silly
//!
//! # Citations
//!
//! - BS 230-232 - Basic techniques
//! - MA 61-85 - Martial Arts techniques

use crate::Skill;
use tracing::instrument;

/// Technique difficulty level.
///
/// # GURPS Rules
///
/// Techniques come in two difficulty levels:
/// - Average (A): Easier to learn, 1 point per level
/// - Hard (H): More difficult to learn, 2 points per level
///
/// # Citations
///
/// BS 230 - Technique difficulty
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
pub enum TechniqueDifficulty {
    /// Average difficulty technique (1 point per level)
    #[display("Average")]
    Average,
    /// Hard difficulty technique (2 points per level)
    #[display("Hard")]
    Hard,
}

/// GURPS combat technique.
///
/// # GURPS Rules
///
/// Techniques are specialized combat maneuvers that default from base skills.
/// Players can buy them up relative to their default to improve specific moves.
///
/// # Citations
///
/// - BS 230-232 - Technique rules
/// - MA 61-85 - Martial Arts techniques
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
pub enum Technique {
    /// Joint lock technique. MA 73
    ArmLock,
    /// Safe falling technique. MA 62
    Breakfall,
    /// Strangling/choking technique. MA 77
    ChokeHold,
    /// Removing opponent's weapon. MA 82
    Disarming,
    /// Fighting on the ground. BS 231, MA 73
    GroundFighting,
    /// Spinning kick attack. MA 85
    SpinningKick,
    /// Two-handed strike. MA 86
    TwoHandedPunch,
    /// Leg lock/choke. MA 86
    TriangleChoke,
    /// Multiple rapid attacks. MA 86
    WhirlwindAttack,
}

impl Technique {
    /// Returns the difficulty of this technique.
    ///
    /// # GURPS Rules
    ///
    /// Average techniques cost 1 point per level to improve.
    /// Hard techniques cost 2 points per level to improve.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Technique, TechniqueDifficulty};
    ///
    /// let arm_lock = Technique::ArmLock;
    /// assert_eq!(arm_lock.difficulty(), TechniqueDifficulty::Average);
    ///
    /// let ground_fighting = Technique::GroundFighting;
    /// assert_eq!(ground_fighting.difficulty(), TechniqueDifficulty::Hard);
    /// ```
    ///
    /// # Citations
    ///
    /// - BS 230 - Technique difficulty
    /// - MA 61-85 - Individual technique difficulties
    #[instrument]
    pub fn difficulty(&self) -> TechniqueDifficulty {
        tracing::debug!("Getting technique difficulty");
        match self {
            Self::ArmLock => TechniqueDifficulty::Average,
            Self::Breakfall => TechniqueDifficulty::Hard,
            Self::ChokeHold => TechniqueDifficulty::Hard,
            Self::Disarming => TechniqueDifficulty::Hard,
            Self::GroundFighting => TechniqueDifficulty::Hard,
            Self::SpinningKick => TechniqueDifficulty::Hard,
            Self::TwoHandedPunch => TechniqueDifficulty::Average,
            Self::TriangleChoke => TechniqueDifficulty::Hard,
            Self::WhirlwindAttack => TechniqueDifficulty::Hard,
        }
    }

    /// Returns the prerequisite skills for this technique.
    ///
    /// # GURPS Rules
    ///
    /// Techniques require one or more base skills as prerequisites.
    /// The technique defaults from the listed prerequisite skills.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Technique, Skill};
    ///
    /// let arm_lock = Technique::ArmLock;
    /// let prereqs = arm_lock.prerequisites();
    /// assert!(prereqs.contains(&Skill::Judo));
    /// assert!(prereqs.contains(&Skill::Wrestling));
    /// ```
    ///
    /// # Citations
    ///
    /// - BS 230 - Technique prerequisites
    /// - MA 61-85 - Individual technique prerequisites
    #[instrument]
    pub fn prerequisites(&self) -> Vec<Skill> {
        tracing::debug!("Getting technique prerequisites");
        match self {
            Self::ArmLock => vec![Skill::Judo, Skill::Wrestling],
            Self::Breakfall => vec![Skill::Acrobatics, Skill::Judo, Skill::Wrestling],
            Self::ChokeHold => vec![Skill::Judo, Skill::Wrestling],
            Self::Disarming => vec![
                Skill::Brawling,
                Skill::Boxing,
                Skill::Karate,
                Skill::Judo,
                Skill::Wrestling,
            ],
            Self::GroundFighting => vec![
                Skill::Brawling,
                Skill::Boxing,
                Skill::Karate,
                Skill::Judo,
                Skill::Wrestling,
            ],
            Self::SpinningKick => vec![Skill::Karate],
            Self::TwoHandedPunch => vec![Skill::Brawling],
            Self::TriangleChoke => vec![Skill::Judo, Skill::Wrestling],
            Self::WhirlwindAttack => vec![Skill::Boxing, Skill::Karate],
        }
    }

    /// Returns the default penalty for this technique.
    ///
    /// # GURPS Rules
    ///
    /// Techniques default at a penalty relative to their prerequisite skill.
    /// For techniques with multiple prerequisites, this returns the best default.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Technique;
    ///
    /// let arm_lock = Technique::ArmLock;
    /// assert_eq!(arm_lock.default_penalty(), 0); // No penalty
    ///
    /// let ground_fighting = Technique::GroundFighting;
    /// assert_eq!(ground_fighting.default_penalty(), -4);
    /// ```
    ///
    /// # Citations
    ///
    /// - BS 230 - Technique defaults
    /// - MA 61-85 - Individual technique defaults
    #[instrument]
    pub fn default_penalty(&self) -> i32 {
        tracing::debug!("Getting technique default penalty");
        match self {
            Self::ArmLock => 0,
            Self::Breakfall => 0,
            Self::ChokeHold => -2, // Best default (Judo-2)
            Self::Disarming => 0,
            Self::GroundFighting => -4,
            Self::SpinningKick => -5,
            Self::TwoHandedPunch => -2,
            Self::TriangleChoke => -4, // Best default (Judo-4)
            Self::WhirlwindAttack => -5,
        }
    }

    /// Returns the maximum bonus this technique can be trained to.
    ///
    /// # GURPS Rules
    ///
    /// Without Technique Mastery, techniques can only be improved to a
    /// certain level relative to their prerequisite skill.
    ///
    /// Returns None for techniques that cannot exceed prerequisite skill.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Technique;
    ///
    /// let arm_lock = Technique::ArmLock;
    /// assert_eq!(arm_lock.maximum_bonus(), Some(4));
    ///
    /// let ground_fighting = Technique::GroundFighting;
    /// assert_eq!(ground_fighting.maximum_bonus(), None); // Cannot exceed skill
    /// ```
    ///
    /// # Citations
    ///
    /// - BS 230 - Technique limits
    /// - MA 61-85 - Individual technique maximums
    #[instrument]
    pub fn maximum_bonus(&self) -> Option<i32> {
        tracing::debug!("Getting technique maximum bonus");
        match self {
            Self::ArmLock => Some(4),
            Self::Breakfall => Some(5),
            Self::ChokeHold => None, // Cannot exceed prerequisite
            Self::Disarming => Some(5),
            Self::GroundFighting => None, // Cannot exceed prerequisite
            Self::SpinningKick => Some(0), // Max is prerequisite (Karate-5 +5 = Karate+0)
            Self::TwoHandedPunch => Some(0), // Max is prerequisite
            Self::TriangleChoke => Some(0), // Max is prerequisite
            Self::WhirlwindAttack => Some(0), // Max is prerequisite
        }
    }

    /// Returns whether this technique is cinematic.
    ///
    /// # GURPS Rules
    ///
    /// Cinematic techniques are marked with an asterisk (*) and are usually
    /// restricted to cinematic campaigns with superhuman martial arts.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Technique;
    ///
    /// let arm_lock = Technique::ArmLock;
    /// assert!(!arm_lock.is_cinematic());
    /// ```
    ///
    /// # Citations
    ///
    /// MA 61 - Cinematic techniques
    #[instrument]
    pub fn is_cinematic(&self) -> bool {
        tracing::debug!("Checking if technique is cinematic");
        // None of our current base techniques are cinematic
        false
    }

    /// Returns whether this technique is silly.
    ///
    /// # GURPS Rules
    ///
    /// Silly techniques are marked with a dagger (†) and are usually
    /// only available in silly campaigns.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Technique;
    ///
    /// let arm_lock = Technique::ArmLock;
    /// assert!(!arm_lock.is_silly());
    /// ```
    ///
    /// # Citations
    ///
    /// MA 61 - Silly techniques
    #[instrument]
    pub fn is_silly(&self) -> bool {
        tracing::debug!("Checking if technique is silly");
        // None of our current base techniques are silly
        false
    }
}
