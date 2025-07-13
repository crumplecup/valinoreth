pub trait BodyLocation {
    type Location;
    fn from_roll(roll: usize) -> Self::Location;
    fn to_hit(&self) -> isize;
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
    strum::EnumIter,
    derive_more::Display,
)]
pub enum BodyArea {
    Head,
    Torso,
    Arms,
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
    Skull,
    Eyes,
    Face,
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
    Chest,
    Abdomen,
    Vitals,
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
    Shoulders,
    Upper,
    Forearms,
    Elbows,
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
    Thighs,
    Knees,
    Shins,
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
