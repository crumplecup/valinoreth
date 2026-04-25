//! Character advantages providing beneficial traits.
//!
//! This module implements GURPS character advantages,
//! including their point costs and variations. Advantages are beneficial
//! traits purchased with character points.
//!
//! # GURPS Rules
//!
//! Advantages are core to GURPS character creation.
//! Each has a point cost (positive for advantages).
//!
//! # Citations
//!
//! - BS 100-132 - Advantages
//! - BS 11 - Point budget rules
//!
//! # Examples
//!
//! ```
//! use valinoreth::{Advantage, Luck};
//!
//! let luck = Advantage::Luck(Luck::Extraordinary);
//! assert_eq!(luck.cost(), 30);
//! ```

/// Character advantages providing beneficial traits.
///
/// # GURPS Rules
///
/// Advantages are purchased with character points during creation.
/// They provide benefits like enhanced abilities, special talents,
/// or unique capabilities. Each advantage has a fixed or level-based cost.
///
/// # Citations
///
/// BS 100-132 - Advantage descriptions and costs
///
/// # Examples
///
/// ```
/// use valinoreth::{Advantage, AbsoluteDirection};
///
/// let advantage = Advantage::AbsoluteDirection(AbsoluteDirection::Normal);
/// assert_eq!(advantage.cost(), 5);
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
pub enum Advantage {
    /// Innate sense of direction. BS 34
    AbsoluteDirection(AbsoluteDirection),
    /// Enhanced hearing (+1 per level to Hearing rolls). BS 35
    AcuteHearing(usize),
    /// Enhanced vision (+1 per level to Vision rolls). BS 35
    AcuteVision(usize),
    /// No penalty for using off-hand. BS 39
    Ambidexterity,
    /// +4 to handle animals, sense emotions. BS 40
    AnimalEmpathy,
    /// +1 Reaction from attracted sex. BS 21
    Attractive,
    /// Bardic magic talent. TH 204
    BardicTalent(usize),
    /// Bonus to influence rolls (+1 per level). BS 41
    Charisma(usize),
    /// +1 to all active defenses, never freeze in combat. BS 43
    CombatReflexes,
    /// Reduced energy cost for spellcasting. TH 28
    EasyCasting(usize),
    /// Perfect recall of information. BS 51
    EiditicMemory(EiditicMemory),
    /// Bonus to Fright Checks and Intimidation resistance. BS 55
    Fearless(usize),
    /// Enhanced flexibility for escape and contortion. BS 56
    Flexible(Flexible),
    /// Bonus to death checks and survival rolls. BS 58
    HardToKill(usize),
    /// Bonus to knockdown and stunning resistance. BS 59
    HardToSubdue(usize),
    /// Manual dexterity bonus to craft and DX-based skills. BS 59
    HighManualDexterity(usize),
    /// Regular income independent of job. BS 26
    IndependentIncome(usize),
    /// Immune to supernatural fear and mind control. BS 60
    Indomidable,
    /// Need less sleep than normal. BS 65
    LessSleep(usize),
    /// Reroll and choose better result. BS 66
    Luck(Luck),
    /// Magical talent and spell aptitude. BS 66
    Magery(usize),
    /// +1 per level to musical performance. BS 69
    MusicalAbility(usize),
    /// +4 to balance and related DX rolls. BS 74
    PerfectBalance,
    /// +4 to handle plants, sense plant health. BS 75
    PlantEmpathy,
    /// Recover FP and HP faster than normal. BS 80
    Recovery,
    /// Need less food and water. BS 80
    ReducedConsumption(usize),
    /// Bonus to Stealth in quiet situations. BS 85
    Silence(usize),
    /// Communicate with animals. BS 87
    SpeakWithAnimals,
    /// +4 to detect and communicate with spirits. BS 88
    SpiritEmpathy,
    /// Reduced penalty for rapid spellcasting. TH 28
    StableCasting,
    /// Social standing and influence. BS 28
    Status(usize),
    /// Bonus to unarmed damage. BS 88
    Striking(usize),
    /// Do not age after maturity. BS 95
    Unaging,
    /// Enhanced fitness, +3 to HT rolls. BS 96
    VeryFit,
    /// +2 to influence via speaking or singing. BS 97
    Voice,
    /// Starting wealth and income level. BS 25
    Wealth(Wealth),
}

impl Advantage {
    /// Calculates the character point cost of this advantage.
    ///
    /// # GURPS Rules
    ///
    /// Each advantage has a fixed or level-based point cost.
    /// The cost is always positive (or zero for unimplemented variants).
    ///
    /// # Returns
    ///
    /// Character points required to purchase this advantage.
    ///
    /// # Citations
    ///
    /// BS 100-132 - Individual advantage costs
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Advantage, Luck};
    ///
    /// let luck = Advantage::Luck(Luck::Extraordinary);
    /// assert_eq!(luck.cost(), 30);
    /// ```
    pub fn cost(&self) -> i64 {
        match self {
            Self::AbsoluteDirection(level) => level.cost(),
            // BS 39
            Self::Ambidexterity => 5,
            // BS 40
            Self::AnimalEmpathy => 5,
            // BS 35
            Self::AcuteHearing(level) => *level as i64 * 2,
            // BS 35
            Self::AcuteVision(level) => *level as i64 * 2,
            // BS 21
            Self::Attractive => 4,
            // 5 points per level TH 204
            Self::BardicTalent(level) => *level as i64 * 5,
            // BS 41
            Self::Charisma(level) => *level as i64 * 5,
            // BS 43
            Self::CombatReflexes => 15,
            // 40% the magery cost TH 28 - not applied to Bardic Talent
            Self::EasyCasting(level) => *level as i64 * 14,
            Self::EiditicMemory(level) => level.cost(),
            // BS 55
            Self::Fearless(level) => *level as i64 * 2,
            Self::Flexible(level) => level.cost(),
            // BS 58
            Self::HardToKill(level) => *level as i64 * 2,
            // BS 59
            Self::HardToSubdue(level) => *level as i64 * 2,
            // 5 points per level BS 59
            Self::HighManualDexterity(level) => *level as i64 * 5,
            // BS 26
            Self::IndependentIncome(level) => *level as i64,
            // BS 60
            Self::Indomidable => 15,
            // 2 points per level BS 65
            Self::LessSleep(level) => *level as i64 * 2,
            Self::Luck(level) => level.cost(),
            // 10 points per level + 5 for Magery 0 BS 66
            Self::Magery(level) => (*level as i64 * 10) + 5,
            // BS 69
            Self::MusicalAbility(level) => *level as i64,
            // BS 74
            Self::PerfectBalance => 15,
            // BS 75
            Self::PlantEmpathy => 5,
            // BS 80
            Self::Recovery => 10,
            // 2 points per level BS 80
            Self::ReducedConsumption(level) => *level as i64 * 2,
            // BS 85
            Self::Silence(level) => *level as i64 * 5,
            // BS 87
            Self::SpeakWithAnimals => 25,
            // BS 88
            Self::SpiritEmpathy => 10,
            // 40% the magery cost TH 28
            Self::StableCasting => 20,
            // BS 28
            Self::Status(level) => *level as i64 * 5,
            // BS 88
            Self::Striking(level) => *level as i64 * 5,
            // BS 95
            Self::Unaging => 15,
            // BS 96
            Self::VeryFit => 15,
            // BS 97
            Self::Voice => 10,
            Self::Wealth(level) => level.cost(),
        }
    }
}

/// Wealth levels determining starting money and income.
///
/// # GURPS Rules
///
/// Wealth determines starting funds and ongoing income.
/// Below-average wealth is a disadvantage (negative cost),
/// above-average is an advantage (positive cost).
///
/// # Citations
///
/// BS 25-26 - Wealth levels and effects
///
/// # Examples
///
/// ```
/// use valinoreth::Wealth;
///
/// let wealth = Wealth::Wealthy;
/// assert_eq!(wealth.cost(), 20);
/// ```
#[derive(
    Debug,
    Default,
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
pub enum Wealth {
    /// No starting funds, -25 points. BS 25
    DeadBroke,
    /// 20% normal starting funds, -15 points. BS 25
    Poor,
    /// 50% normal starting funds, -10 points. BS 25
    Struggling,
    /// Normal starting funds (default), 0 points. BS 25
    #[default]
    Average,
    /// 2x normal starting funds, 10 points. BS 25
    Comfortable,
    /// 5x normal starting funds, 20 points. BS 25
    Wealthy,
    /// 20x normal starting funds, 30 points. BS 25
    VeryWealthy,
    /// 100x normal starting funds, 50 points. BS 25
    FilthyRich,
    /// 1000x normal starting funds, 75 points (+25 per additional level). BS 25
    Multimillionaire,
}

impl Wealth {
    /// Returns the character point cost for this wealth level.
    ///
    /// # GURPS Rules
    ///
    /// Wealth below Average is a disadvantage (negative points).
    /// Wealth above Average is an advantage (positive points).
    ///
    /// # Citations
    ///
    /// BS 25-26 - Wealth point costs
    pub fn cost(&self) -> i64 {
        match self {
            Self::DeadBroke => -25,
            Self::Poor => -15,
            Self::Struggling => -10,
            Self::Average => 0,
            Self::Comfortable => 10,
            Self::Wealthy => 20,
            Self::VeryWealthy => 30,
            Self::FilthyRich => 50,
            Self::Multimillionaire => 100,
        }
    }
}

/// Innate sense of direction and orientation.
///
/// # GURPS Rules
///
/// Always know which way is north and retrace your path.
/// Spatial version works in 3D environments.
///
/// # Citations
///
/// BS 34 - Absolute Direction
///
/// # Examples
///
/// ```
/// use valinoreth::AbsoluteDirection;
///
/// let direction = AbsoluteDirection::Spatial;
/// assert_eq!(direction.cost(), 10);
/// ```
#[derive(
    Debug,
    Default,
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
pub enum AbsoluteDirection {
    /// Ground-based direction sense, 5 points. BS 34
    #[default]
    Normal,
    /// 3D direction sense (underwater, zero-G), 10 points. BS 34
    Spatial,
}

impl AbsoluteDirection {
    /// Returns the character point cost for this level.
    ///
    /// # Citations
    ///
    /// BS 34 - Absolute Direction costs
    pub fn cost(&self) -> i64 {
        match self {
            Self::Normal => 5,
            Self::Spatial => 10,
        }
    }
}

/// Luck advantage allowing rerolls.
///
/// # GURPS Rules
///
/// Once per hour of play, reroll a bad die roll or force foe to reroll a good one.
/// Higher levels allow more frequent rerolls.
///
/// # Citations
///
/// BS 66-67 - Luck levels and usage
///
/// # Examples
///
/// ```
/// use valinoreth::Luck;
///
/// let luck = Luck::Extraordinary;
/// assert_eq!(luck.cost(), 30);
/// ```
#[derive(
    Debug,
    Default,
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
pub enum Luck {
    /// Reroll once per hour, 15 points. BS 66
    #[default]
    Normal,
    /// Reroll once per 30 minutes, 30 points. BS 67
    Extraordinary,
    /// Reroll once per 10 minutes, 60 points. BS 67
    Ridiculous,
}

impl Luck {
    /// Returns the character point cost for this luck level.
    ///
    /// # Citations
    ///
    /// BS 66-67 - Luck costs
    pub fn cost(&self) -> i64 {
        match self {
            Self::Normal => 15,
            Self::Extraordinary => 30,
            Self::Ridiculous => 60,
        }
    }
}

/// Enhanced flexibility for escapes and contortion.
///
/// # GURPS Rules
///
/// +3 to Climbing, Escape, and Erotic Art skills.
/// Double-Jointed adds +5 to Escape specifically.
///
/// # Citations
///
/// BS 56 - Flexibility rules
///
/// # Examples
///
/// ```
/// use valinoreth::Flexible;
///
/// let flex = Flexible::DoubleJointed;
/// assert_eq!(flex.cost(), 15);
/// ```
#[derive(
    Debug,
    Default,
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
pub enum Flexible {
    /// +3 bonus to flexibility skills, 5 points. BS 56
    #[default]
    Normal,
    /// +5 to Escape skill, 15 points total. BS 56
    DoubleJointed,
}

impl Flexible {
    /// Returns the character point cost for this flexibility level.
    ///
    /// # Citations
    ///
    /// BS 56 - Flexibility costs
    pub fn cost(&self) -> i64 {
        match self {
            Self::Normal => 5,
            Self::DoubleJointed => 15,
        }
    }
}

/// Perfect recall of information.
///
/// # GURPS Rules
///
/// Remember everything you see or hear with perfect accuracy.
/// Photographic version includes ability to visualize and read from memory.
///
/// # Citations
///
/// BS 51 - Eidetic Memory rules
///
/// # Examples
///
/// ```
/// use valinoreth::EiditicMemory;
///
/// let memory = EiditicMemory::Photographic;
/// assert_eq!(memory.cost(), 10);
/// ```
#[derive(
    Debug,
    Default,
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
pub enum EiditicMemory {
    /// Perfect recall, automatic success on IQ rolls to remember, 5 points. BS 51
    #[default]
    Normal,
    /// Visual recall, can "read" memorized text, 10 points. BS 51
    Photographic,
}

impl EiditicMemory {
    /// Returns the character point cost for this memory level.
    ///
    /// # Citations
    ///
    /// BS 51 - Eidetic Memory costs
    pub fn cost(&self) -> i64 {
        match self {
            Self::Normal => 5,
            Self::Photographic => 10,
        }
    }
}

/// Minor advantages costing 1 point each.
///
/// # GURPS Rules
///
/// Perks are small advantages that provide minor benefits.
/// Each costs exactly 1 character point.
///
/// # Citations
///
/// BS 100 - Perks overview
///
/// # Examples
///
/// ```
/// use valinoreth::Perk;
///
/// let perk = Perk::DeepSleeper;
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
pub enum Perk {
    /// Hard to wake, +4 to avoid being awakened. BS 101
    DeepSleeper,
    /// +1 to Reaction when truth matters. BS 101
    HonestFace,
}

