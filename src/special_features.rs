use crate::{Advantage, Disadvantage, Perk};

#[derive(
    Debug,
    Default,
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
pub struct SpecialFeatures {
    advantages: Vec<Advantage>,
    disadvantages: Vec<Disadvantage>,
    perks: Vec<Perk>,
}
