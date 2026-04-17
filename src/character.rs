//! Character attributes, statistics, and derived values.
//!
//! This module implements GURPS character attributes and their derived
//! statistics including Basic Speed, Basic Move, Dodge, damage calculations,
//! and encumbrance levels.
//!
//! # GURPS Rules
//!
//! Characters have four primary attributes (ST, DX, IQ, HT) and four
//! secondary attributes (HP, Will, Per, FP). Secondary attributes default
//! to their associated primary attributes but can be purchased separately.
//!
//! # Citations
//!
//! - BS 13-17 - Character attributes
//! - BS 15-17 - Derived statistics
//!
//! # Examples
//!
//! ```
//! use valinoreth::Attributes;
//!
//! // Create character with ST 12, DX 11, IQ 10, HT 10
//! let attrs = Attributes::from_base(12, 11, 10, 10);
//! ```

use derive_more::Display;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

use crate::DieLevel;

/// Character attribute types.
///
/// # GURPS Rules
///
/// Four primary attributes (Strength, Dexterity, Intelligence, Health)
/// form the basis of a character. Four secondary attributes (Hit Points,
/// Willpower, Perception, Fatigue) default to primary values but can be
/// purchased separately.
///
/// # Citations
///
/// BS 13-17 - Attribute definitions and costs
///
/// # Examples
///
/// ```
/// use valinoreth::AttributeType;
///
/// let attr = AttributeType::Strength;
/// assert_eq!(attr.to_string(), "Strength");
/// ```
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    derive_more::Display,
    derive_more::FromStr,
    strum::EnumIter,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum AttributeType {
    /// Physical strength, 10 points per ±1 from 10. BS 14
    #[default]
    Strength,
    /// Manual dexterity and agility, 10 points per ±1 from 10. BS 15
    Dexterity,
    /// Reasoning and memory, 10 points per ±1 from 10. BS 15
    Intelligence,
    /// Endurance and vitality, 10 points per ±1 from 10. BS 14
    Health,
    /// Injury capacity, defaults to ST, 2 points per ±1. BS 16
    HitPoints,
    /// Mental strength, defaults to IQ, 5 points per ±1. BS 16
    Willpower,
    /// Awareness and alertness, defaults to IQ, 5 points per ±1. BS 16
    Perception,
    /// Energy for physical exertion, defaults to HT, 3 points per ±1. BS 16
    Fatigue,
}

impl AttributeType {
    /// Parse attribute from abbreviation (case-insensitive).
    ///
    /// # Arguments
    ///
    /// * `abbr` - Abbreviation like "st", "dx", "iq", "ht", "hp", "will", "per", "fp"
    ///
    /// # Returns
    ///
    /// Some(AttributeType) if abbreviation is valid, None otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::AttributeType;
    ///
    /// assert_eq!(AttributeType::from_abbr("st"), Some(AttributeType::Strength));
    /// assert_eq!(AttributeType::from_abbr("DX"), Some(AttributeType::Dexterity));
    /// assert_eq!(AttributeType::from_abbr("invalid"), None);
    /// ```
    pub fn from_abbr(abbr: &str) -> Option<Self> {
        let lwr = abbr.to_lowercase();
        let value = match lwr.as_str() {
            "st" => Self::Strength,
            "dx" => Self::Dexterity,
            "iq" => Self::Intelligence,
            "ht" => Self::Health,
            "hp" => Self::HitPoints,
            "will" => Self::Willpower,
            "per" => Self::Perception,
            "fp" => Self::Fatigue,
            _ => return None,
        };
        Some(value)
    }
}

/// Character attributes container.
///
/// # GURPS Rules
///
/// Stores all eight character attributes: four primary (ST, DX, IQ, HT)
/// and four secondary (HP, Will, Per, FP). Secondary attributes default
/// to their associated primary attributes.
///
/// # Citations
///
/// BS 13-17 - Attribute system
///
/// # Examples
///
/// ```
/// use valinoreth::Attributes;
///
/// // Using defaults for secondary attributes
/// let attrs = Attributes::from_base(10, 11, 12, 10);
///
/// // Specifying all eight attributes
/// let attrs = Attributes::from_vec(vec![10, 11, 12, 10, 10, 12, 13, 10]);
/// ```
#[derive(
    Debug, Default, Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Serialize, Deserialize,
)]
pub struct Attributes {
    st: usize,
    dx: usize,
    iq: usize,
    ht: usize,
    hp: usize,
    will: usize,
    per: usize,
    fp: usize,
}

impl Attributes {
    /// Creates attributes with default secondary values.
    ///
    /// # GURPS Rules
    ///
    /// Secondary attributes default to:
    /// - HP = ST
    /// - Will = IQ
    /// - Per = IQ
    /// - FP = HT
    ///
    /// # Arguments
    ///
    /// * `st` - Strength (primary)
    /// * `dx` - Dexterity (primary)
    /// * `iq` - Intelligence (primary)
    /// * `ht` - Health (primary)
    ///
    /// # Citations
    ///
    /// BS 16 - Secondary attribute defaults
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Attributes;
    ///
    /// let attrs = Attributes::from_base(12, 11, 10, 10);
    /// // HP=12, Will=10, Per=10, FP=10
    /// ```
    pub fn from_base(st: usize, dx: usize, iq: usize, ht: usize) -> Self {
        Self {
            st,
            dx,
            iq,
            ht,
            // Base hit points is equal to Strength BS 16
            hp: st,
            // Base will is equal to IQ BS 16
            will: iq,
            // Base perception is equal to IQ BS 16
            per: iq,
            // Base fatique is equal to health BS 16
            fp: ht,
        }
    }

    /// Creates attributes from a vector of eight values.
    ///
    /// # Arguments
    ///
    /// * `vec` - Vector with [ST, DX, IQ, HT, HP, Will, Per, FP]
    ///
    /// # Panics
    ///
    /// Panics if vec.len() < 8
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Attributes;
    ///
    /// let attrs = Attributes::from_vec(vec![12, 11, 10, 10, 14, 10, 11, 10]);
    /// // ST=12, DX=11, IQ=10, HT=10, HP=14, Will=10, Per=11, FP=10
    /// ```
    pub fn from_vec(vec: Vec<usize>) -> Self {
        Self {
            st: vec[0],
            dx: vec[1],
            iq: vec[2],
            ht: vec[3],
            hp: vec[4],
            will: vec[5],
            per: vec[6],
            fp: vec[7],
        }
    }

    /// Returns the full name of an attribute.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Attributes, AttributeType};
    ///
    /// let attrs = Attributes::from_base(10, 10, 10, 10);
    /// assert_eq!(attrs.name(&AttributeType::Strength), "Strength");
    /// ```
    pub fn name(&self, attribute: &AttributeType) -> String {
        match *attribute {
            AttributeType::Strength => "Strength".to_string(),
            AttributeType::Dexterity => "Dexterity".to_string(),
            AttributeType::Intelligence => "Intelligence".to_string(),
            AttributeType::Health => "Health".to_string(),
            AttributeType::HitPoints => "Hit Points".to_string(),
            AttributeType::Willpower => "Willpower".to_string(),
            AttributeType::Perception => "Perception".to_string(),
            AttributeType::Fatigue => "Fatigue".to_string(),
        }
    }

    /// Returns the numeric value of an attribute.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Attributes, AttributeType};
    ///
    /// let attrs = Attributes::from_base(12, 10, 10, 10);
    /// assert_eq!(attrs.value(&AttributeType::Strength), 12);
    /// assert_eq!(attrs.value(&AttributeType::HitPoints), 12); // HP defaults to ST
    /// ```
    pub fn value(&self, attribute: &AttributeType) -> usize {
        match *attribute {
            AttributeType::Strength => self.st,
            AttributeType::Dexterity => self.dx,
            AttributeType::Intelligence => self.iq,
            AttributeType::Health => self.ht,
            AttributeType::HitPoints => self.hp,
            AttributeType::Willpower => self.will,
            AttributeType::Perception => self.per,
            AttributeType::Fatigue => self.fp,
        }
    }

    /// Generates the display value for a given column.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Attributes, AttributeType, AttributeColumns};
    ///
    /// let attrs = Attributes::from_base(12, 10, 10, 10);
    /// assert_eq!(attrs.column(&AttributeType::Strength, &AttributeColumns::Name), "Strength");
    /// assert_eq!(attrs.column(&AttributeType::Strength, &AttributeColumns::Value), "12");
    /// ```
    pub fn column(&self, attribute: &AttributeType, column: &AttributeColumns) -> String {
        match *column {
            AttributeColumns::Name => self.name(attribute),
            AttributeColumns::Value => self.value(attribute).to_string(),
        }
    }

    /// Returns all column values for an attribute.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Attributes, AttributeType};
    ///
    /// let attrs = Attributes::from_base(12, 10, 10, 10);
    /// let cols = attrs.columns(&AttributeType::Strength);
    /// assert_eq!(cols, vec!["Strength", "12"]);
    /// ```
    pub fn columns(&self, attribute: &AttributeType) -> Vec<String> {
        AttributeColumns::iter()
            .map(|c| self.column(attribute, &c))
            .collect::<Vec<String>>()
    }
}

/// Display columns for attribute output.
///
/// Used for formatting attribute data in tabular form.
///
/// # Examples
///
/// ```
/// use valinoreth::AttributeColumns;
///
/// let names = AttributeColumns::names();
/// assert_eq!(names, vec!["Name", "Value"]);
/// ```
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Display,
    EnumIter,
    Serialize,
    Deserialize,
)]
pub enum AttributeColumns {
    /// Attribute name column
    Name,
    /// Attribute value column
    #[default]
    Value,
}

impl AttributeColumns {
    /// Returns all column names as strings.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::AttributeColumns;
    ///
    /// let names = AttributeColumns::names();
    /// assert_eq!(names, vec!["Name", "Value"]);
    /// ```
    pub fn names() -> Vec<String> {
        let mut values = Vec::new();
        for column in Self::iter() {
            values.push(format!("{column}"));
        }
        values
    }
}

/// Derived character statistics.
///
/// # GURPS Rules
///
/// Basic statistics calculated from primary attributes:
/// - Basic Lift: Maximum weight liftable overhead with one hand in one second
/// - Basic Speed: Measure of reaction time and physical coordination
/// - Basic Move: Distance in meters movable per second
///
/// # Citations
///
/// - BS 15 - Basic Lift
/// - BS 17 - Basic Speed and Move
///
/// # Examples
///
/// ```
/// use valinoreth::{Attributes, Stats};
///
/// let attrs = Attributes::from_base(12, 11, 10, 10);
/// let stats = Stats::from(attrs);
/// // Basic Lift = (12 * 12) / 5 = 28 lbs
/// // Basic Speed = (10 + 11) / 4 = 5.25
/// // Basic Move = floor(5.25) = 5
/// ```
#[derive(
    Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, derive_new::new,
)]
pub struct Stats {
    /// The maximum weight you can lift over your head with one hand in one second.
    ///
    /// Formula: (ST × ST) ÷ 5
    ///
    /// BS 15
    basic_lift: usize,
    /// Basic Move is Basic Speed less any fractions.
    ///
    /// Formula: floor(Basic Speed)
    ///
    /// BS 17
    basic_move: usize,
    /// Basic speed is (HT + DX) ÷ 4
    ///
    /// Dodge is basic speed plus 3, dropping fractions.
    ///
    /// BS 17
    basic_speed: f64,
}

impl From<Attributes> for Stats {
    /// Calculates derived statistics from attributes.
    ///
    /// # GURPS Rules
    ///
    /// - Basic Lift = (ST × ST) ÷ 5, rounded down
    /// - Basic Speed = (HT + DX) ÷ 4
    /// - Basic Move = Basic Speed, rounded down
    ///
    /// # Citations
    ///
    /// - BS 15 - Basic Lift formula
    /// - BS 17 - Basic Speed and Move formulas
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Attributes, Stats};
    ///
    /// let attrs = Attributes::from_base(10, 10, 10, 10);
    /// let stats = Stats::from(attrs);
    /// // ST 10: Basic Lift = 100/5 = 20 lbs
    /// // DX 10, HT 10: Basic Speed = 20/4 = 5.0
    /// // Basic Move = 5
    /// ```
    fn from(attr: Attributes) -> Self {
        let st = attr.st as f64;
        let basic_lift = (st * st) / 5.0;
        let basic_lift = basic_lift.floor() as usize;
        let ht = attr.ht as f64;
        let dx = attr.dx as f64;
        let basic_speed = (ht + dx) / 4.0;
        let basic_move = basic_speed.floor() as usize;
        Self {
            basic_lift,
            basic_speed,
            basic_move,
        }
    }
}

/// Combat-related character statistics.
///
/// # GURPS Rules
///
/// Combat statistics track damage capability and defensive values:
/// - Thrust/Swing damage based on ST
/// - Damage Resistance (DR) from armor
/// - Parry and Block for active defenses
///
/// # Citations
///
/// - BS 269-271 - Damage
/// - BS 374-377 - Active defenses
#[derive(Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct CombatStats {
    /// Thrust damage from ST-based attacks. BS 269
    damage_thrust: DamageKind,
    /// Swing damage from ST-based attacks. BS 269
    damage_swing: DamageKind,
    /// Damage Resistance from armor. BS 269
    dr: usize,
    /// Parry defense value. BS 376
    parry: usize,
    /// Block defense value. BS 377
    block: usize,
}

/// Type of melee damage based on attack style.
///
/// # GURPS Rules
///
/// Melee weapons use either thrust or swing damage, based on the
/// character's ST. Each has different damage dice.
///
/// # Citations
///
/// BS 269-271 - Damage types
///
/// # Examples
///
/// ```
/// use valinoreth::{DamageKind, DieLevel};
///
/// let thrust = DamageKind::Thrust(DieLevel::new(1, -1));
/// let swing = DamageKind::Swing(DieLevel::new(1, 2));
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
    derive_new::new,
)]
pub enum DamageKind {
    /// Thrust attack using stabbing motion. BS 269
    Thrust(DieLevel),
    /// Swing attack using sweeping motion. BS 269
    Swing(DieLevel),
}

/// Base damage values for a character.
///
/// # GURPS Rules
///
/// Base damage is determined entirely by ST. Characters with higher
/// ST deal more damage with melee weapons and thrown weapons.
///
/// The damage table maps ST to thrust and swing damage dice.
///
/// # Citations
///
/// BS 16 - Damage table
/// BS 269 - Using damage in combat
///
/// # Examples
///
/// ```
/// use valinoreth::{Attributes, BaseDamage};
///
/// let attrs = Attributes::from_base(10, 10, 10, 10);
/// let damage = BaseDamage::from(attrs);
/// // ST 10: Thrust 1d-2, Swing 1d
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
    derive_new::new,
    derive_getters::Getters,
)]
pub struct BaseDamage {
    /// Thrust damage for stabbing attacks. BS 269
    thrust: DamageKind,
    /// Swing damage for sweeping attacks. BS 269
    swing: DamageKind,
}

impl From<Attributes> for BaseDamage {
    /// Calculates base damage from ST using the damage table.
    ///
    /// # GURPS Rules
    ///
    /// The damage table in GURPS Basic Set maps ST values to
    /// thrust and swing damage dice. This implementation covers
    /// ST 0-100+ with the complete official table.
    ///
    /// # Citations
    ///
    /// BS 16 - Damage table
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Attributes, BaseDamage};
    ///
    /// // ST 10: 1d-2 thrust, 1d swing
    /// let attrs = Attributes::from_base(10, 10, 10, 10);
    /// let damage = BaseDamage::from(attrs);
    ///
    /// // ST 20: 2d-1 thrust, 3d+2 swing
    /// let attrs = Attributes::from_base(20, 10, 10, 10);
    /// let damage = BaseDamage::from(attrs);
    /// ```
    fn from(value: Attributes) -> Self {
        let (thrust, swing) = match value.st {
            0 => (DieLevel::new(0, 0), DieLevel::new(0, 1)),
            1 => (DieLevel::new(1, -6), DieLevel::new(1, -5)),
            2 => (DieLevel::new(1, -6), DieLevel::new(1, -5)),
            3 => (DieLevel::new(1, -5), DieLevel::new(1, -4)),
            4 => (DieLevel::new(1, -5), DieLevel::new(1, -4)),
            5 => (DieLevel::new(1, -4), DieLevel::new(1, -3)),
            6 => (DieLevel::new(1, -4), DieLevel::new(1, -3)),
            7 => (DieLevel::new(1, -3), DieLevel::new(1, -2)),
            8 => (DieLevel::new(1, -3), DieLevel::new(1, -2)),
            9 => (DieLevel::new(1, -2), DieLevel::new(1, -1)),
            10 => (DieLevel::new(1, -2), DieLevel::new(1, 0)),
            11 => (DieLevel::new(1, -1), DieLevel::new(1, 1)),
            12 => (DieLevel::new(1, -1), DieLevel::new(1, 2)),
            13 => (DieLevel::new(1, 0), DieLevel::new(2, -1)),
            14 => (DieLevel::new(1, 0), DieLevel::new(2, 0)),
            15 => (DieLevel::new(1, 1), DieLevel::new(2, 1)),
            16 => (DieLevel::new(1, 1), DieLevel::new(2, 2)),
            17 => (DieLevel::new(1, 2), DieLevel::new(3, -1)),
            18 => (DieLevel::new(1, 2), DieLevel::new(3, 0)),
            19 => (DieLevel::new(2, -1), DieLevel::new(3, 1)),
            20 => (DieLevel::new(2, -1), DieLevel::new(3, 2)),
            21 => (DieLevel::new(2, 0), DieLevel::new(4, -1)),
            22 => (DieLevel::new(2, 0), DieLevel::new(4, 0)),
            23 => (DieLevel::new(2, 1), DieLevel::new(4, 1)),
            24 => (DieLevel::new(2, 1), DieLevel::new(4, 2)),
            25 => (DieLevel::new(2, 2), DieLevel::new(5, -1)),
            26 => (DieLevel::new(2, 2), DieLevel::new(5, 0)),
            27 => (DieLevel::new(3, -1), DieLevel::new(5, 1)),
            28 => (DieLevel::new(3, -1), DieLevel::new(5, 1)),
            29 => (DieLevel::new(3, 0), DieLevel::new(5, 2)),
            30 => (DieLevel::new(3, 0), DieLevel::new(5, 2)),
            31 => (DieLevel::new(3, 1), DieLevel::new(6, -1)),
            32 => (DieLevel::new(3, 1), DieLevel::new(6, -1)),
            33 => (DieLevel::new(3, 2), DieLevel::new(6, 0)),
            34 => (DieLevel::new(3, 2), DieLevel::new(6, 0)),
            35 => (DieLevel::new(4, -1), DieLevel::new(6, 1)),
            36 => (DieLevel::new(4, -1), DieLevel::new(6, 1)),
            37 => (DieLevel::new(4, 0), DieLevel::new(6, 2)),
            38 => (DieLevel::new(4, 0), DieLevel::new(6, 2)),
            39 => (DieLevel::new(4, 1), DieLevel::new(7, -1)),
            40..45 => (DieLevel::new(4, 1), DieLevel::new(7, -1)),
            45..50 => (DieLevel::new(5, 0), DieLevel::new(7, 1)),
            50..55 => (DieLevel::new(5, 2), DieLevel::new(8, -1)),
            55..60 => (DieLevel::new(6, 0), DieLevel::new(8, 1)),
            60..65 => (DieLevel::new(7, -1), DieLevel::new(9, 0)),
            65..70 => (DieLevel::new(7, 1), DieLevel::new(9, 2)),
            70..75 => (DieLevel::new(8, 0), DieLevel::new(10, 0)),
            75..80 => (DieLevel::new(8, 2), DieLevel::new(10, 2)),
            80..85 => (DieLevel::new(9, 0), DieLevel::new(11, 0)),
            85..90 => (DieLevel::new(9, 2), DieLevel::new(11, 2)),
            90..95 => (DieLevel::new(10, 0), DieLevel::new(12, 0)),
            95..100 => (DieLevel::new(10, 2), DieLevel::new(12, 2)),
            100 => (DieLevel::new(11, 0), DieLevel::new(13, 0)),
            above => {
                let buff = (above - 100) % 10;
                (
                    DieLevel::new(11 + buff as i64, 0),
                    DieLevel::new(13 + buff as i64, 0),
                )
            }
        };
        Self::new(DamageKind::Thrust(thrust), DamageKind::Swing(swing))
    }
}

/// Character encumbrance levels and effects.
///
/// # GURPS Rules
///
/// Encumbrance represents the burden of carried equipment. Higher
/// encumbrance reduces movement speed and dodge.
///
/// There are five levels: None, Light, Medium, Heavy, and Extra-Heavy.
///
/// # Citations
///
/// BS 17 - Encumbrance rules
///
/// # Examples
///
/// ```
/// use valinoreth::{Attributes, Stats, Encumbrance};
///
/// let attrs = Attributes::from_base(10, 10, 10, 10);
/// let stats = Stats::from(attrs);
/// let enc = Encumbrance::from(&stats);
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
    derive_new::new,
)]
pub struct Encumbrance {
    /// Weight thresholds for each encumbrance level. BS 17
    weight: EncumbranceWeight,
    /// Move penalties for each encumbrance level. BS 17
    enc_move: EncumbranceMove,
    /// Dodge penalties for each encumbrance level. BS 17
    dodge: EncumbranceDodge,
}

impl From<&Stats> for Encumbrance {
    /// Calculates encumbrance tables from character stats.
    ///
    /// # Citations
    ///
    /// BS 17 - Encumbrance calculation
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Attributes, Stats, Encumbrance};
    ///
    /// let attrs = Attributes::from_base(10, 10, 10, 10);
    /// let stats = Stats::from(attrs);
    /// let enc = Encumbrance::from(&stats);
    /// ```
    fn from(stats: &Stats) -> Self {
        let weight = EncumbranceWeight::from(stats);
        let enc_move = EncumbranceMove::from(stats);
        let dodge = EncumbranceDodge::from(stats);
        Self {
            weight,
            enc_move,
            dodge,
        }
    }
}

/// Weight thresholds for encumbrance levels.
///
/// # GURPS Rules
///
/// Encumbrance levels are based on multiples of Basic Lift:
/// - None: 0 to BL
/// - Light: BL+1 to 2×BL
/// - Medium: 2×BL+1 to 3×BL
/// - Heavy: 3×BL+1 to 6×BL
/// - Extra-Heavy: 6×BL+1 to 10×BL
///
/// # Citations
///
/// BS 17 - Encumbrance weight thresholds
///
/// # Examples
///
/// ```
/// use valinoreth::{Attributes, Stats, EncumbranceWeight};
///
/// let attrs = Attributes::from_base(10, 10, 10, 10);
/// let stats = Stats::from(attrs);
/// let weights = EncumbranceWeight::from(&stats);
/// // BL = 20 lbs
/// // None: 0-20, Light: 21-40, Medium: 41-60, Heavy: 61-120, X-Heavy: 121-200
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
    derive_new::new,
)]
pub struct EncumbranceWeight {
    /// Weight threshold for no encumbrance (0 to BL). BS 17
    none: usize,
    /// Weight threshold for light encumbrance (BL+1 to 2×BL). BS 17
    light: usize,
    /// Weight threshold for medium encumbrance (2×BL+1 to 3×BL). BS 17
    medium: usize,
    /// Weight threshold for heavy encumbrance (3×BL+1 to 6×BL). BS 17
    heavy: usize,
    /// Weight threshold for extra-heavy encumbrance (6×BL+1 to 10×BL). BS 17
    extra_heavy: usize,
}

impl From<&Stats> for EncumbranceWeight {
    /// Calculates weight thresholds from Basic Lift.
    ///
    /// # Citations
    ///
    /// BS 17 - Encumbrance weight multiples
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Attributes, Stats, EncumbranceWeight};
    ///
    /// let attrs = Attributes::from_base(10, 10, 10, 10);
    /// let stats = Stats::from(attrs);
    /// let weights = EncumbranceWeight::from(&stats);
    /// ```
    fn from(stats: &Stats) -> Self {
        let basic_lift = stats.basic_lift;
        let none = basic_lift;
        let light = basic_lift * 2;
        let medium = basic_lift * 3;
        let heavy = basic_lift * 6;
        let extra_heavy = basic_lift * 10;
        Self {
            none,
            light,
            medium,
            heavy,
            extra_heavy,
        }
    }
}

/// Movement penalties for encumbrance levels.
///
/// # GURPS Rules
///
/// Encumbrance reduces movement speed by a percentage of Basic Move:
/// - None: 100% (×1.0)
/// - Light: 80% (×0.8)
/// - Medium: 60% (×0.6)
/// - Heavy: 40% (×0.4)
/// - Extra-Heavy: 20% (×0.2)
///
/// # Citations
///
/// BS 17 - Encumbrance move penalties
///
/// # Examples
///
/// ```
/// use valinoreth::{Attributes, Stats, EncumbranceMove};
///
/// let attrs = Attributes::from_base(10, 10, 10, 10);
/// let stats = Stats::from(attrs);
/// let move_rates = EncumbranceMove::from(&stats);
/// // Basic Move 5: None=5, Light=4, Medium=3, Heavy=2, X-Heavy=1
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
    derive_new::new,
)]
pub struct EncumbranceMove {
    /// Move at no encumbrance (×1.0). BS 17
    none: usize,
    /// Move at light encumbrance (×0.8). BS 17
    light: usize,
    /// Move at medium encumbrance (×0.6). BS 17
    medium: usize,
    /// Move at heavy encumbrance (×0.4). BS 17
    heavy: usize,
    /// Move at extra-heavy encumbrance (×0.2). BS 17
    extra_heavy: usize,
}

impl From<&Stats> for EncumbranceMove {
    /// Calculates move rates for each encumbrance level.
    ///
    /// # Citations
    ///
    /// BS 17 - Encumbrance move multipliers
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Attributes, Stats, EncumbranceMove};
    ///
    /// let attrs = Attributes::from_base(10, 10, 10, 10);
    /// let stats = Stats::from(attrs);
    /// let move_rates = EncumbranceMove::from(&stats);
    /// ```
    fn from(stats: &Stats) -> Self {
        let basic_move = stats.basic_move;
        let none = basic_move;
        let flt = basic_move as f64 * 0.8;
        let light = flt.floor() as usize;
        let flt = basic_move as f64 * 0.6;
        let medium = flt.floor() as usize;
        let flt = basic_move as f64 * 0.4;
        let heavy = flt.floor() as usize;
        let flt = basic_move as f64 * 0.2;
        let extra_heavy = flt.floor() as usize;
        Self {
            none,
            light,
            medium,
            heavy,
            extra_heavy,
        }
    }
}

/// Dodge penalties for encumbrance levels.
///
/// # GURPS Rules
///
/// Encumbrance reduces dodge defense:
/// - None: Dodge (no penalty)
/// - Light: Dodge -1
/// - Medium: Dodge -2
/// - Heavy: Dodge -3
/// - Extra-Heavy: Dodge -4
///
/// Base Dodge = floor(Basic Speed) + 3
///
/// # Citations
///
/// BS 17 - Dodge and encumbrance
/// BS 374 - Dodge defense
///
/// # Examples
///
/// ```
/// use valinoreth::{Attributes, Stats, EncumbranceDodge};
///
/// let attrs = Attributes::from_base(10, 10, 10, 10);
/// let stats = Stats::from(attrs);
/// let dodge_values = EncumbranceDodge::from(&stats);
/// // Basic Speed 5.0: Base Dodge = 8
/// // None=8, Light=7, Medium=6, Heavy=5, X-Heavy=4
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
    derive_new::new,
)]
pub struct EncumbranceDodge {
    /// Dodge at no encumbrance. BS 17
    none: usize,
    /// Dodge at light encumbrance (-1). BS 17
    light: usize,
    /// Dodge at medium encumbrance (-2). BS 17
    medium: usize,
    /// Dodge at heavy encumbrance (-3). BS 17
    heavy: usize,
    /// Dodge at extra-heavy encumbrance (-4). BS 17
    extra_heavy: usize,
}

impl From<&Stats> for EncumbranceDodge {
    /// Calculates dodge values for each encumbrance level.
    ///
    /// # GURPS Rules
    ///
    /// Base Dodge = floor(Basic Speed) + 3, then subtract
    /// encumbrance penalty (0 to -4).
    ///
    /// # Citations
    ///
    /// BS 17 - Dodge calculation and encumbrance penalties
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Attributes, Stats, EncumbranceDodge};
    ///
    /// let attrs = Attributes::from_base(10, 10, 10, 10);
    /// let stats = Stats::from(attrs);
    /// let dodge_values = EncumbranceDodge::from(&stats);
    /// ```
    fn from(stats: &Stats) -> Self {
        // Since basic speed has a minimum of one, at extra heavy usize will not drop below zero.
        // Dodge is basic speed plus 3, dropping fractions BS 17
        let dodge = stats.basic_speed.floor() as usize + 3;
        let none = dodge;
        let light = dodge - 1;
        let medium = dodge - 2;
        let heavy = dodge - 3;
        let extra_heavy = dodge - 4;
        Self {
            none,
            light,
            medium,
            heavy,
            extra_heavy,
        }
    }
}

/// Encumbrance level categories.
///
/// # GURPS Rules
///
/// Five levels of encumbrance based on carried weight:
/// - None (0): 0 to BL
/// - Light (1): BL+1 to 2×BL, -1 dodge, ×0.8 move
/// - Medium (2): 2×BL+1 to 3×BL, -2 dodge, ×0.6 move
/// - Heavy (3): 3×BL+1 to 6×BL, -3 dodge, ×0.4 move
/// - Extra-Heavy (4): 6×BL+1 to 10×BL, -4 dodge, ×0.2 move
///
/// # Citations
///
/// BS 17 - Encumbrance levels and effects
///
/// # Examples
///
/// ```
/// use valinoreth::EncumbranceLevel;
///
/// let level = EncumbranceLevel::Light;
/// ```
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum EncumbranceLevel {
    /// No encumbrance (0 to BL). BS 17
    None,
    /// Light encumbrance (BL+1 to 2×BL). BS 17
    Light,
    /// Medium encumbrance (2×BL+1 to 3×BL). BS 17
    Medium,
    /// Heavy encumbrance (3×BL+1 to 6×BL). BS 17
    Heavy,
    /// Extra-heavy encumbrance (6×BL+1 to 10×BL). BS 17
    XHeavy,
}
