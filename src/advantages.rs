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
    AbsoluteDirection(AbsoluteDirection),
    AcuteHearing(usize),
    AcuteVision(usize),
    Ambidexterity,
    AnimalEmpathy,
    Attractive,
    // Thaumatology pg. 204
    BardicTalent(usize),
    Charisma(usize),
    CombatReflexes,
    // Thaumatology pg. 28
    EasyCasting(usize),
    // Basic Set pg. 51
    EiditicMemory(EiditicMemory),
    Fearless(usize),
    Flexible(Flexible),
    HardToKill(usize),
    HardToSubdue(usize),
    // Basic Set pg. 59
    HighManualDexterity(usize),
    IndependentIncome(usize),
    Indomidable,
    // Basic Set pg. 65
    LessSleep(usize),
    Luck(Luck),
    Magery(usize),
    MusicalAbility(usize),
    PerfectBalance,
    PlantEmpathy,
    Recovery,
    // Basic Set pg. 80
    ReducedConsumption(usize),
    Silence(usize),
    SpeakWithAnimals,
    SpiritEmpathy,
    // Thaumatology pg. 28
    StableCasting,
    Status(usize),
    Striking(usize),
    Unaging,
    VeryFit,
    Voice,
    Wealth(Wealth),
}

impl Advantage {
    pub fn cost(&self) -> i64 {
        match self {
            Self::AbsoluteDirection(level) => level.cost(),
            Self::Ambidexterity => 5,
            Self::AnimalEmpathy => 5,
            // 5 points per level [Thaumatology - 204]
            Self::BardicTalent(level) => *level as i64 * 5,
            Self::CombatReflexes => 15,
            // 40% the magery cost [Thaumatology - 28]
            // not applied to Bardic Talent
            Self::EasyCasting(level) => *level as i64 * 14,
            Self::EiditicMemory(level) => level.cost(),
            Self::Flexible(level) => level.cost(),
            // 5 points per level [BS - 59]
            Self::HighManualDexterity(level) => *level as i64 * 5,
            Self::IndependentIncome(level) => *level as i64,
            // 2 points per level [BS - 64]
            Self::LessSleep(level) => *level as i64 * 2,
            Self::Luck(level) => level.cost(),
            // 5 points for Magery 0, 10 pts per level [BS - 66]
            Self::Magery(level) => (*level as i64 * 5) + 5,
            Self::PlantEmpathy => 5,
            // 2 points per level [BS - 80]
            Self::ReducedConsumption(level) => *level as i64 * 2,
            Self::SpeakWithAnimals => 25,
            // 40% the magery cost [Thaumatology - 28]
            Self::StableCasting => 20,
            Self::Status(level) => *level as i64 * 5,
            Self::Voice => 10,
            Self::Wealth(level) => level.cost(),
            _ => 0,
        }
    }
}

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
    DeadBroke,
    Poor,
    Struggling,
    #[default]
    Average,
    Comfortable,
    Wealthy,
    VeryWealthy,
    FilthyRich,
    Multimillionaire,
}

impl Wealth {
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
    #[default]
    Normal,
    Spatial,
}

impl AbsoluteDirection {
    pub fn cost(&self) -> i64 {
        match self {
            Self::Normal => 5,
            Self::Spatial => 10,
        }
    }
}

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
    #[default]
    Normal,
    Extraordinary,
    Ridiculous,
}

impl Luck {
    pub fn cost(&self) -> i64 {
        match self {
            Self::Normal => 15,
            Self::Extraordinary => 30,
            Self::Ridiculous => 60,
        }
    }
}

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
    #[default]
    Normal,
    DoubleJointed,
}

impl Flexible {
    pub fn cost(&self) -> i64 {
        match self {
            Self::Normal => 5,
            Self::DoubleJointed => 15,
        }
    }
}

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
    #[default]
    Normal,
    Photographic,
}

impl EiditicMemory {
    pub fn cost(&self) -> i64 {
        // Basic Set pg. 51
        match self {
            Self::Normal => 5,
            Self::Photographic => 10,
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
    derive_more::Display,
)]
pub enum Perk {
    DeepSleeper,
    HonestFace,
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
    derive_more::Display,
)]
pub enum Disadvantage {
    CodeOfHonor(usize),
    Destiny(usize),
    Duty(Duty),
    Honesty,
    Selfless,
    SenseOfDuty(SenseOfDuty),
    // Basic Set pg. 155
    SocialStigma(usize),
    Status(usize),
    Stubborn,
}

impl Disadvantage {
    pub fn cost(&self) -> i64 {
        match self {
            Self::Destiny(level) => -(*level as i64),
            Self::Duty(level) => level.cost(),
            Self::Selfless => -5,
            Self::SenseOfDuty(level) => level.cost(),
            // -5 points per level, max level 4 [BS - 155]
            Self::SocialStigma(level) => -(*level as i64 * 5),
            Self::Status(level) => -(*level as i64 * 5),
            _ => 0,
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
    derive_more::Display,
)]
pub enum SenseOfDuty {
    Individual,
    SmallGroup,
    LargeGroup,
    EntireRace,
    EveryLivingBeing,
}

impl SenseOfDuty {
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
    AlmostAlways,
    QuiteOften,
    FairlyOften,
    QuiteRarely,
}

impl Duty {
    pub fn cost(&self) -> i64 {
        match self {
            Self::AlmostAlways => -15,
            Self::QuiteOften => -10,
            Self::FairlyOften => -5,
            Self::QuiteRarely => -2,
        }
    }
}
