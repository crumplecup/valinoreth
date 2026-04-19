//! Spell type definitions and helper enums.
//!
//! # GURPS Rules
//!
//! Spells have various properties: energy costs, durations, prerequisites,
//! resistance types, and casting mechanics.
//!
//! # Citations
//!
//! BS 239-253 - Magic system
//! M 10 - Spell notation conventions

use crate::{Spell, SpellCollege};

/// Energy cost structure for spells.
///
/// # GURPS Rules
///
/// Energy costs can be fixed or scale with spell effect
/// (per die of damage, per HP healed, per yard of range, etc.).
///
/// # Citations
///
/// BS 241 - Energy cost
/// M 10 - Cost notation
///
/// # Examples
///
/// ```
/// use valinoreth::EnergyCost;
///
/// let fixed = EnergyCost::Fixed(2);
/// let scaling = EnergyCost::PerDie(1);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EnergyCost {
    /// Fixed energy cost
    Fixed(i32),
    /// Cost per die of damage
    PerDie(i32),
    /// Cost per HP restored/damaged
    PerHP(i32),
    /// Cost per FP transferred
    PerFP(i32),
    /// Cost per yard of range/radius
    PerYard(i32),
}

/// Spell prerequisite.
///
/// # GURPS Rules
///
/// Spells require Magery levels, other spells, minimum attributes,
/// or a count of spells known in a college.
///
/// # Citations
///
/// BS 241 - Prerequisites
/// M 10 - Prerequisite notation
///
/// # Examples
///
/// ```
/// use valinoreth::{SpellPrerequisite, Spell, SpellCollege};
///
/// let magery = SpellPrerequisite::Magery(1);
/// let spell = SpellPrerequisite::Spell(Spell::DetectMagic);
/// let college = SpellPrerequisite::SpellsInCollege(SpellCollege::Knowledge, 4);
/// let iq = SpellPrerequisite::IQ(12);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SpellPrerequisite {
    /// Magery level required (0 = any, 1+ = specific level)
    Magery(usize),
    /// Specific spell must be known
    Spell(Spell),
    /// Number of spells in college must be known
    SpellsInCollege(SpellCollege, usize),
    /// Minimum IQ required
    IQ(i32),
}

/// Spell type for casting mechanics.
///
/// # GURPS Rules
///
/// Different spell types have different casting mechanics:
/// - Regular: affects single target
/// - Area: affects area/multiple targets
/// - Missile: ranged attack requiring hit roll
/// - Information: reveals data
///
/// # Citations
///
/// BS 239 - Spell types
///
/// # Examples
///
/// ```
/// use valinoreth::SpellType;
///
/// let info = SpellType::Information;
/// let area = SpellType::Area;
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SpellType {
    /// Standard single-target spell
    Regular,
    /// Affects area or multiple targets
    Area,
    /// Ranged attack (requires to-hit roll)
    Missile,
    /// Melee touch attack
    Melee,
    /// Blocks or resists other spells
    Blocking,
    /// Reveals information
    Information,
}

/// Spell duration.
///
/// # GURPS Rules
///
/// Durations vary from instant effects to maintained concentrations
/// to timed effects.
///
/// # Citations
///
/// BS 241 - Duration
///
/// # Examples
///
/// ```
/// use valinoreth::Duration;
///
/// let instant = Duration::Instant;
/// let maintained = Duration::Concentration;
/// let timed = Duration::Minutes(10);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Duration {
    /// Instant effect
    Instant,
    /// Requires active concentration
    Concentration,
    /// Lasts for minutes
    Minutes(i32),
    /// Lasts for hours
    Hours(i32),
    /// Lasts until dispelled
    Permanent,
}

/// Resistance type for spells.
///
/// # GURPS Rules
///
/// Some spells allow resistance rolls to avoid or reduce effects.
/// Resistance is typically vs Will, HT, or IQ.
///
/// # Citations
///
/// BS 241 - Resistance
///
/// # Examples
///
/// ```
/// use valinoreth::ResistanceType;
///
/// let will = ResistanceType::Will;
/// let iq = ResistanceType::IQ;
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ResistanceType {
    /// Resistance vs Will
    Will,
    /// Resistance vs Health
    HT,
    /// Resistance vs Intelligence
    IQ,
    /// Special resistance (varies)
    Special,
}
