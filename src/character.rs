use derive_more::Display;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

use crate::DieLevel;

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
    #[default]
    Strength,
    Dexterity,
    Intelligence,
    Health,
    HitPoints,
    Willpower,
    Perception,
    Fatigue,
}

impl AttributeType {
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
    pub fn from_base(st: usize, dx: usize, iq: usize, ht: usize) -> Self {
        Self {
            st,
            dx,
            iq,
            ht,
            // Base hit points is equal to Strength [BS - 16]
            hp: st,
            // Base will is equal to IQ [BS - 16]
            will: iq,
            // Base perception is equal to IQ [BS - 16]
            per: iq,
            // Base fatique is equal to health [BS - 16]
            fp: ht,
        }
    }

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
    pub fn column(&self, attribute: &AttributeType, column: &AttributeColumns) -> String {
        match *column {
            AttributeColumns::Name => self.name(attribute),
            AttributeColumns::Value => self.value(attribute).to_string(),
        }
    }

    pub fn columns(&self, attribute: &AttributeType) -> Vec<String> {
        AttributeColumns::iter()
            .map(|c| self.column(attribute, &c))
            .collect::<Vec<String>>()
    }
}

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
    Name,
    #[default]
    Value,
}

impl AttributeColumns {
    pub fn names() -> Vec<String> {
        let mut values = Vec::new();
        for column in Self::iter() {
            values.push(format!("{column}"));
        }
        values
    }
}

#[derive(
    Debug, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize, derive_new::new,
)]
pub struct Stats {
    /// The maximum weight you can lift over your head with one hand one second.
    /// (ST * ST)/5
    /// BS-15
    basic_lift: usize,
    /// Basic Move is Basic Speed less any fractions [BS - 17]
    basic_move: usize,
    /// Basic speed is (HT + DX)/4
    /// Dodge is basic speed plus 3, dropping fractions [BS - 17]
    basic_speed: f64,
}

impl From<Attributes> for Stats {
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

pub struct CombatStats {
    damage_thrust: DamageKind,
    damage_swing: DamageKind,
    dr: usize,
    parry: usize,
    block: usize,
}

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
    Thrust(DieLevel),
    Swing(DieLevel),
}

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
pub struct BaseDamage {
    thrust: DamageKind,
    swing: DamageKind,
}

impl From<Attributes> for BaseDamage {
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
    weight: EncumbranceWeight,
    enc_move: EncumbranceMove,
    dodge: EncumbranceDodge,
}

impl From<&Stats> for Encumbrance {
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
    none: usize,
    light: usize,
    medium: usize,
    heavy: usize,
    extra_heavy: usize,
}

impl From<&Stats> for EncumbranceWeight {
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
    none: usize,
    light: usize,
    medium: usize,
    heavy: usize,
    extra_heavy: usize,
}

impl From<&Stats> for EncumbranceMove {
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
    none: usize,
    light: usize,
    medium: usize,
    heavy: usize,
    extra_heavy: usize,
}

impl From<&Stats> for EncumbranceDodge {
    fn from(stats: &Stats) -> Self {
        // Since basic speed has a minimum of one, at extra heavy usize will not drop below zero.
        // Dodge is basic speed plus 3, dropping fractions [BS - 17]
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

pub enum EncumbranceLevel {
    None,
    Light,
    Medium,
    Heavy,
    XHeavy,
}
