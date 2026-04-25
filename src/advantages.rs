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
    /// Second legal identity. BS 39
    AlternateIdentity,
    /// Enhanced taste and smell (+1 per level to related rolls). BS 35
    AcuteTasteSmell(usize),
    /// Enhanced sense of touch (+1 per level to related rolls). BS 35
    AcuteTouch(usize),
    /// Enhanced vision (+1 per level to Vision rolls). BS 35
    AcuteVision(usize),
    /// No penalty for using off-hand. BS 39
    Ambidexterity,
    /// +4 to handle animals, sense emotions. BS 40
    AnimalEmpathy,
    /// Physical appearance level affecting reactions. BS 21
    Appearance(Appearance),
    /// Bardic magic talent. TH 204
    BardicTalent(usize),
    /// Bonus to influence rolls (+1 per level). BS 41
    Charisma(usize),
    /// Communicate with spirits of the dead. BS 41
    Channeling,
    /// Always welcomed by specific group. BS 43
    ClaimToHospitality,
    /// Religious authority and rank. BS 43
    ClericalInvestment,
    /// Run multiple mental processes simultaneously. BS 43
    CompartmentalizedMind(usize),
    /// Helpful contact who provides aid. BS 44
    Contact(usize),
    /// Understand customs of specific culture. BS 23
    CulturalFamiliarity,
    /// Can breathe both air and water. BS 40
    Amphibious,
    /// Swing through trees like an ape. BS 41
    Brachiator,
    /// Land safely on feet from falls, reduce falling damage. BS 41
    Catfall,
    /// Sharp claws for close combat. BS 42
    Claws(Claws),
    /// Can cling to walls and ceilings. BS 43
    Clinging,
    /// +1 to all active defenses, never freeze in combat. BS 43
    CombatReflexes,
    /// GM warns when about to do something stupid. BS 43
    CommonSense,
    /// Damage resistance protecting from physical injury. BS 47
    DamageResistance(usize),
    /// Identify sounds by timbre and pitch. BS 49
    DiscriminatoryHearing,
    /// Analyze odors precisely. BS 49
    DiscriminatorySmell,
    /// Identify specific tastes. BS 49
    DiscriminatoryTaste,
    /// Don't need to breathe. BS 49
    DoesntBreathe,
    /// Don't need food or water. BS 50
    DoesntEatOrDrink,
    /// Never need to sleep. BS 50
    DoesntSleep,
    /// Warning of imminent danger. BS 47
    DangerSense,
    /// Reduced energy cost for spellcasting. TH 28
    EasyCasting(usize),
    /// Sense emotions and detect lies. BS 51
    Empathy,
    /// Perfect recall of information. BS 51
    EiditicMemory(EiditicMemory),
    /// Bonus to Fright Checks and Intimidation resistance. BS 55
    Fearless(usize),
    /// Someone owes you a favor. BS 55
    Favor(usize),
    /// Additional arms beyond two. BS 53
    ExtraArms(usize),
    /// Additional legs beyond two. BS 54
    ExtraLegs(usize),
    /// Additional hit points beyond HT. BS 16
    ExtraHitPoints(usize),
    /// Come back from death. BS 55
    ExtraLife(usize),
    /// Healthy and in good shape, +1 to HT rolls. BS 55
    Fit,
    /// Enhanced flexibility for escape and contortion. BS 56
    Flexible(Flexible),
    /// Ability to fly through the air. BS 56
    Flight(Flight),
    /// Bonus to death checks and survival rolls. BS 58
    HardToKill(usize),
    /// Bonus to knockdown and stunning resistance. BS 59
    HardToSubdue(usize),
    /// Manual dexterity bonus to craft and DX-based skills. BS 59
    HighManualDexterity(usize),
    /// Ignore shock penalties from injury. BS 59
    HighPainThreshold,
    /// See heat and infrared radiation. BS 60
    InfraredVision,
    /// Resistance to specific types of injury. BS 60
    InjuryTolerance(InjuryTolerance),
    /// Regular income independent of job. BS 26
    IndependentIncome(usize),
    /// Immune to supernatural fear and mind control. BS 60
    Indomidable,
    /// Complete immunity to specific hazard. BS 81
    Immunity(usize),
    /// Can become immaterial. BS 62
    Insubstantiality,
    /// Cannot be seen. BS 63
    Invisibility,
    /// Ask GM for hints or clues. BS 63
    Intuition,
    /// Speak additional languages fluently. BS 24
    Language(usize),
    /// Easier to learn new languages. BS 65
    LanguageTalent,
    /// Law enforcement authority. BS 65
    LegalEnforcementPowers,
    /// Diplomatic immunity from prosecution. BS 65
    LegalImmunity,
    /// Need less sleep than normal. BS 65
    LessSleep(usize),
    /// Instant mental arithmetic. BS 66
    LightningCalculator,
    /// Age more slowly than normal. BS 66
    Longevity,
    /// Reroll and choose better result. BS 66
    Luck(Luck),
    /// Magical talent and spell aptitude. BS 66
    Magery(usize),
    /// Bonus to math-heavy skills. BS 68
    MathematicalAbility(usize),
    /// Rank in military organization. BS 29
    MilitaryRank(usize),
    /// Resist mental attacks. BS 70
    MindShield(usize),
    /// +1 per level to musical performance. BS 69
    MusicalAbility(usize),
    /// Get visions of the future. BS 71
    Oracle,
    /// Powerful person or organization helps you. BS 72
    Patron(usize),
    /// Take over other bodies. BS 75
    Possession,
    /// See future events. BS 77
    Precognition,
    /// See in darkness as if it were daylight. BS 71
    NightVision(usize),
    /// Pinpoint sound sources precisely. BS 74
    ParabolicHearing,
    /// +4 to balance and related DX rolls. BS 74
    PerfectBalance,
    /// Impossible to sneak up on, no back penalty. BS 74
    PeripheralVision,
    /// Protected sense immune to blinding/deafening. BS 78
    ProtectedSense(ProtectedSense),
    /// +4 to handle plants, sense plant health. BS 75
    PlantEmpathy,
    /// Heal 2x faster from injuries. BS 79
    RapidHealing,
    /// Recover FP and HP faster than normal. BS 80
    Recovery,
    /// Need less food and water. BS 80
    ReducedConsumption(usize),
    /// Regrow lost limbs and heal rapidly. BS 80
    Regeneration(Regeneration),
    /// Regrow severed body parts instantly. BS 80
    Regrowth,
    /// Bonus to resist specific hazards. BS 80
    Resistant(Resistant),
    /// Remember past lives. BS 80
    Reawakened,
    /// Well-known for specific trait or deed. BS 26
    Reputation(usize),
    /// Immune to low-pressure environments. BS 85
    Sealed,
    /// No waste products from metabolism. BS 101
    SanitizedMetabolism,
    /// Access to classified information. BS 82
    SecurityClearance(usize),
    /// Beneficial coincidences happen. BS 83
    Serendipity(usize),
    /// Bonus to Stealth in quiet situations. BS 85
    Silence(usize),
    /// Respected or feared by society. BS 86
    SocialRegard(SocialRegard),
    /// Natural weapon like tail, horns, or spikes. BS 88
    Striker(Striker),
    /// Hear infrasonic/subsonic sounds. BS 89
    SubsonicHearing,
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
    /// Comfortable in extreme temperatures. BS 93
    TemperatureTolerance(usize),
    /// Sharp teeth for biting attacks. BS 91
    Teeth(Teeth),
    /// Cannot be fired from job. BS 93
    Tenure,
    /// See distant objects clearly. BS 92
    TelescopicVision(usize),
    /// Resist timeline changes. BS 93
    TemporalInertia,
    /// Hear ultrasonic sounds. BS 94
    Ultrahearing,
    /// See ultraviolet light. BS 94
    Ultravision,
    /// Do not age after maturity. BS 95
    Unaging,
    /// Cannot be permanently killed. BS 95
    Unkillable(usize),
    /// Survive without breathing. BS 96
    VacuumSupport,
    /// Sense vibrations through ground or water. BS 96
    VibrationSense,
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
            // BS 35
            Self::AcuteHearing(level) => *level as i64 * 2,
            // BS 39
            Self::AlternateIdentity => 15,
            // BS 35
            Self::AcuteTasteSmell(level) => *level as i64 * 2,
            // BS 35
            Self::AcuteTouch(level) => *level as i64 * 2,
            // BS 35
            Self::AcuteVision(level) => *level as i64 * 2,
            // BS 39
            Self::Ambidexterity => 5,
            // BS 40
            Self::Amphibious => 10,
            // BS 40
            Self::AnimalEmpathy => 5,
            Self::Appearance(level) => level.cost(),
            // 5 points per level TH 204
            Self::BardicTalent(level) => *level as i64 * 5,
            // BS 41
            Self::Brachiator => 5,
            // BS 41
            Self::Catfall => 10,
            // BS 41
            Self::Charisma(level) => *level as i64 * 5,
            // BS 41
            Self::Channeling => 10,
            // BS 43
            Self::ClaimToHospitality => 5,
            // BS 43
            Self::ClericalInvestment => 5,
            // BS 43
            Self::CompartmentalizedMind(level) => *level as i64 * 50,
            // BS 44 - varies by reliability and power
            Self::Contact(level) => *level as i64,
            // BS 23
            Self::CulturalFamiliarity => 1,
            Self::Claws(level) => level.cost(),
            // BS 43
            Self::Clinging => 20,
            // BS 43
            Self::CombatReflexes => 15,
            // BS 43
            Self::CommonSense => 10,
            // BS 47
            Self::DamageResistance(level) => *level as i64 * 5,
            // BS 47
            Self::DangerSense => 15,
            // BS 49
            Self::DoesntBreathe => 20,
            // BS 50
            Self::DoesntEatOrDrink => 10,
            // BS 50
            Self::DoesntSleep => 20,
            // BS 49
            Self::DiscriminatoryHearing => 15,
            // BS 49
            Self::DiscriminatorySmell => 15,
            // BS 49
            Self::DiscriminatoryTaste => 10,
            // 40% the magery cost TH 28 - not applied to Bardic Talent
            Self::EasyCasting(level) => *level as i64 * 14,
            Self::EiditicMemory(level) => level.cost(),
            // BS 51
            Self::Empathy => 15,
            // BS 55
            Self::Fearless(level) => *level as i64 * 2,
            // BS 55 - varies greatly
            Self::Favor(level) => *level as i64,
            // BS 53 - 10 points per arm
            Self::ExtraArms(level) => *level as i64 * 10,
            // BS 54 - varies: 2 legs=5pts, 3=10pts, 4=15pts, etc.
            Self::ExtraLegs(level) => {
                if *level == 1 {
                    5
                } else {
                    (*level as i64 - 1) * 10 + 5
                }
            }
            // BS 16
            Self::ExtraHitPoints(level) => *level as i64 * 2,
            // BS 55
            Self::ExtraLife(level) => *level as i64 * 25,
            // BS 55
            Self::Fit => 5,
            Self::Flexible(level) => level.cost(),
            Self::Flight(level) => level.cost(),
            // BS 58
            Self::HardToKill(level) => *level as i64 * 2,
            // BS 59
            Self::HardToSubdue(level) => *level as i64 * 2,
            // 5 points per level BS 59
            Self::HighManualDexterity(level) => *level as i64 * 5,
            // BS 59
            Self::HighPainThreshold => 10,
            // BS 26
            Self::IndependentIncome(level) => *level as i64,
            // BS 60
            Self::Indomidable => 15,
            // BS 81 - varies greatly
            Self::Immunity(level) => *level as i64 * 10,
            // BS 60
            Self::InfraredVision => 10,
            // BS 62
            Self::Insubstantiality => 80,
            // BS 63
            Self::Intuition => 15,
            // BS 63
            Self::Invisibility => 40,
            Self::InjuryTolerance(level) => level.cost(),
            // BS 24 - 3 points per language at spoken/written
            Self::Language(level) => *level as i64 * 3,
            // BS 65
            Self::LanguageTalent => 10,
            // BS 65
            Self::LegalEnforcementPowers => 10,
            // BS 65
            Self::LegalImmunity => 15,
            // 2 points per level BS 65
            Self::LessSleep(level) => *level as i64 * 2,
            // BS 66
            Self::LightningCalculator => 2,
            // BS 66
            Self::Longevity => 2,
            Self::Luck(level) => level.cost(),
            // 10 points per level + 5 for Magery 0 BS 66
            Self::Magery(level) => (*level as i64 * 10) + 5,
            // BS 68
            Self::MathematicalAbility(level) => *level as i64 * 10,
            // BS 29 - 5 points per rank level
            Self::MilitaryRank(level) => *level as i64 * 5,
            // BS 70
            Self::MindShield(level) => *level as i64 * 4,
            // BS 69
            Self::MusicalAbility(level) => *level as i64,
            // BS 71
            Self::Oracle => 15,
            // BS 72 - varies greatly based on power
            Self::Patron(level) => *level as i64 * 5,
            // BS 75
            Self::Possession => 100,
            // BS 77
            Self::Precognition => 25,
            // BS 71
            Self::NightVision(level) => *level as i64,
            // BS 74
            Self::ParabolicHearing => 4,
            // BS 74
            Self::PerfectBalance => 15,
            // BS 74
            Self::PeripheralVision => 15,
            Self::ProtectedSense(level) => level.cost(),
            // BS 75
            Self::PlantEmpathy => 5,
            // BS 79
            Self::RapidHealing => 5,
            // BS 80
            Self::Recovery => 10,
            // 2 points per level BS 80
            Self::ReducedConsumption(level) => *level as i64 * 2,
            Self::Regeneration(level) => level.cost(),
            // BS 80
            Self::Regrowth => 40,
            Self::Resistant(level) => level.cost(),
            // BS 80
            Self::Reawakened => 10,
            // BS 26 - varies by scope and frequency
            Self::Reputation(level) => *level as i64,
            // BS 101
            Self::SanitizedMetabolism => 1,
            // BS 85
            Self::Sealed => 15,
            // BS 82 - varies by level
            Self::SecurityClearance(level) => *level as i64,
            // BS 83
            Self::Serendipity(level) => *level as i64 * 15,
            // BS 85
            Self::Silence(level) => *level as i64 * 5,
            Self::SocialRegard(level) => level.cost(),
            // BS 87
            Self::SpeakWithAnimals => 25,
            // BS 88
            Self::SpiritEmpathy => 10,
            // 40% the magery cost TH 28
            Self::StableCasting => 20,
            // BS 28
            Self::Status(level) => *level as i64 * 5,
            Self::Striker(level) => level.cost(),
            // BS 88
            Self::Striking(level) => *level as i64 * 5,
            // BS 89
            Self::SubsonicHearing => 5,
            // BS 93
            Self::TemperatureTolerance(level) => *level as i64,
            Self::Teeth(level) => level.cost(),
            // BS 93
            Self::Tenure => 5,
            // BS 92
            Self::TelescopicVision(level) => *level as i64 * 5,
            // BS 93
            Self::TemporalInertia => 15,
            // BS 94
            Self::Ultrahearing => 5,
            // BS 94
            Self::Ultravision => 10,
            // BS 95
            Self::Unaging => 15,
            // BS 95 - 50 points per level
            Self::Unkillable(level) => *level as i64 * 50,
            // BS 96
            Self::VacuumSupport => 5,
            // BS 96
            Self::VibrationSense => 10,
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

/// Physical appearance affecting reaction rolls.
///
/// # GURPS Rules
///
/// Appearance affects reaction rolls from those who can see you.
/// Above-average appearance gives bonuses, below-average gives penalties.
///
/// # Citations
///
/// BS 21 - Appearance levels
///
/// # Examples
///
/// ```
/// use valinoreth::Appearance;
///
/// let appearance = Appearance::Attractive;
/// assert_eq!(appearance.cost(), 4);
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
pub enum Appearance {
    /// -16 to reactions, -4 points. BS 21
    Horrifying,
    /// -8 to reactions, -3 points. BS 21
    Monstrous,
    /// -4 to reactions, -2 points. BS 21
    Hideous,
    /// -2 to reactions, -1 point. BS 21
    Ugly,
    /// -1 to reactions, 0 points. BS 21
    Unattractive,
    /// No reaction modifier, 0 points. BS 21
    #[default]
    Average,
    /// +1 to reactions, 4 points. BS 21
    Attractive,
    /// +2 to reactions, 12 points. BS 21
    Handsome,
    /// +4 to reactions, 16 points. BS 21
    Beautiful,
    /// +6 to reactions, 20 points. BS 21
    VeryBeautiful,
}

impl Appearance {
    /// Returns the character point cost for this appearance level.
    ///
    /// # Citations
    ///
    /// BS 21 - Appearance costs
    pub fn cost(&self) -> i64 {
        match self {
            Self::Horrifying => -16,
            Self::Monstrous => -12,
            Self::Hideous => -8,
            Self::Ugly => -4,
            Self::Unattractive => -2,
            Self::Average => 0,
            Self::Attractive => 4,
            Self::Handsome => 12,
            Self::Beautiful => 16,
            Self::VeryBeautiful => 20,
        }
    }
}

/// Sharp claws for close combat.
///
/// # GURPS Rules
///
/// Natural weapons for punching and grappling attacks.
/// Sharper claws provide better damage but may have social penalties.
///
/// # Citations
///
/// BS 42-43 - Claws rules
///
/// # Examples
///
/// ```
/// use valinoreth::Claws;
///
/// let claws = Claws::Sharp;
/// assert_eq!(claws.cost(), 5);
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
pub enum Claws {
    /// Blunt claws, thrust-1 cutting, 3 points. BS 42
    Blunt,
    /// Sharp claws, thrust-1 cutting, 5 points. BS 42
    #[default]
    Sharp,
    /// Long talons, thrust cutting, 8 points. BS 43
    Talons,
}

impl Claws {
    /// Returns the character point cost for this claw type.
    ///
    /// # Citations
    ///
    /// BS 42-43 - Claws costs
    pub fn cost(&self) -> i64 {
        match self {
            Self::Blunt => 3,
            Self::Sharp => 5,
            Self::Talons => 8,
        }
    }
}

/// Ability to fly through the air.
///
/// # GURPS Rules
///
/// Fly at your Move score. Different methods have different costs.
/// Winged flight is cheaper but vulnerable to wing damage.
///
/// # Citations
///
/// BS 56 - Flight rules
///
/// # Examples
///
/// ```
/// use valinoreth::Flight;
///
/// let flight = Flight::Winged;
/// assert_eq!(flight.cost(), 30);
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
pub enum Flight {
    /// Flight with wings, vulnerable to damage, 30 points. BS 56
    #[default]
    Winged,
    /// Magical or psionic flight, 40 points. BS 56
    Standard,
}

impl Flight {
    /// Returns the character point cost for this flight type.
    ///
    /// # Citations
    ///
    /// BS 56 - Flight costs
    pub fn cost(&self) -> i64 {
        match self {
            Self::Winged => 30,
            Self::Standard => 40,
        }
    }
}

/// Resistance to specific types of injury.
///
/// # GURPS Rules
///
/// Reduces or eliminates damage from certain attack types.
/// Different injury tolerances protect against different threats.
///
/// # Citations
///
/// BS 60 - Injury Tolerance rules
///
/// # Examples
///
/// ```
/// use valinoreth::InjuryTolerance;
///
/// let tolerance = InjuryTolerance::NoPain;
/// assert_eq!(tolerance.cost(), 20);
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
pub enum InjuryTolerance {
    /// Ignore shock penalties, 20 points. BS 60
    #[default]
    NoPain,
    /// Diffuse body, immune to crippling, 100 points. BS 60
    Diffuse,
    /// Homogeneous body, no vitals, 40 points. BS 60
    Homogeneous,
    /// No blood, immune to blood loss, 5 points. BS 60
    NoBlood,
    /// No brain, immune to head hits, 5 points. BS 60
    NoBrain,
    /// No eyes, immune to eye hits, 5 points. BS 60
    NoEyes,
    /// No neck, immune to neck hits, 5 points. BS 60
    NoNeck,
    /// No vitals, no vital organ damage, 5 points. BS 60
    NoVitals,
    /// Unliving, immune to many conditions, 20 points. BS 60
    Unliving,
}

impl InjuryTolerance {
    /// Returns the character point cost for this injury tolerance.
    ///
    /// # Citations
    ///
    /// BS 60 - Injury Tolerance costs
    pub fn cost(&self) -> i64 {
        match self {
            Self::NoPain => 20,
            Self::Diffuse => 100,
            Self::Homogeneous => 40,
            Self::NoBlood => 5,
            Self::NoBrain => 5,
            Self::NoEyes => 5,
            Self::NoNeck => 5,
            Self::NoVitals => 5,
            Self::Unliving => 20,
        }
    }
}

/// Rapid healing and limb regeneration.
///
/// # GURPS Rules
///
/// Regrow lost limbs and organs over time.
/// Faster regeneration costs more points.
///
/// # Citations
///
/// BS 80 - Regeneration rules
///
/// # Examples
///
/// ```
/// use valinoreth::Regeneration;
///
/// let regen = Regeneration::Regular;
/// assert_eq!(regen.cost(), 25);
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
pub enum Regeneration {
    /// Slow regeneration, 10 points. BS 80
    Slow,
    /// Regular regeneration, 1 HP/12 hours, 25 points. BS 80
    #[default]
    Regular,
    /// Fast regeneration, 1 HP/hour, 50 points. BS 80
    Fast,
    /// Very fast regeneration, 1 HP/minute, 100 points. BS 80
    VeryFast,
    /// Extreme regeneration, 1 HP/second, 150 points. BS 80
    Extreme,
}

impl Regeneration {
    /// Returns the character point cost for this regeneration speed.
    ///
    /// # Citations
    ///
    /// BS 80 - Regeneration costs
    pub fn cost(&self) -> i64 {
        match self {
            Self::Slow => 10,
            Self::Regular => 25,
            Self::Fast => 50,
            Self::VeryFast => 100,
            Self::Extreme => 150,
        }
    }
}

/// Bonus to resist specific hazards.
///
/// # GURPS Rules
///
/// +3 or +8 to HT rolls to resist specific conditions.
/// Immunity makes you completely immune.
///
/// # Citations
///
/// BS 80-81 - Resistant rules
///
/// # Examples
///
/// ```
/// use valinoreth::Resistant;
///
/// let resistant = Resistant::Disease;
/// assert_eq!(resistant.cost(), 3);
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
pub enum Resistant {
    /// +3 to resist disease, 3 points. BS 81
    #[default]
    Disease,
    /// +8 to resist disease, 5 points. BS 81
    DiseaseImmunity,
    /// +3 to resist poison, 5 points. BS 81
    Poison,
    /// +8 to resist poison, 8 points. BS 81
    PoisonImmunity,
    /// +3 to resist sickness, varies. BS 81
    Sickness,
}

impl Resistant {
    /// Returns the character point cost for this resistance.
    ///
    /// # Citations
    ///
    /// BS 80-81 - Resistant costs
    pub fn cost(&self) -> i64 {
        match self {
            Self::Disease => 3,
            Self::DiseaseImmunity => 5,
            Self::Poison => 5,
            Self::PoisonImmunity => 8,
            Self::Sickness => 3,
        }
    }
}

/// Sharp teeth for biting attacks.
///
/// # GURPS Rules
///
/// Natural weapon for biting in close combat.
/// Sharper teeth inflict more damage.
///
/// # Citations
///
/// BS 91 - Teeth rules
///
/// # Examples
///
/// ```
/// use valinoreth::Teeth;
///
/// let teeth = Teeth::Sharp;
/// assert_eq!(teeth.cost(), 1);
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
pub enum Teeth {
    /// Sharp teeth, thrust-1 cutting, 1 point. BS 91
    #[default]
    Sharp,
    /// Fangs, thrust-1 impaling, 2 points. BS 91
    Fangs,
    /// Long fangs, thrust impaling, 5 points. BS 91
    LongFangs,
}

impl Teeth {
    /// Returns the character point cost for this tooth type.
    ///
    /// # Citations
    ///
    /// BS 91 - Teeth costs
    pub fn cost(&self) -> i64 {
        match self {
            Self::Sharp => 1,
            Self::Fangs => 2,
            Self::LongFangs => 5,
        }
    }
}

/// Protected senses immune to attacks.
///
/// # GURPS Rules
///
/// Your sense organs are protected from being disabled or damaged.
/// This prevents blinding, deafening, or similar targeted attacks.
///
/// # Citations
///
/// BS 78 - Protected Sense
///
/// # Examples
///
/// ```
/// use valinoreth::ProtectedSense;
///
/// let protected = ProtectedSense::Vision;
/// assert_eq!(protected.cost(), 5);
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
pub enum ProtectedSense {
    /// Protected hearing, 5 points. BS 78
    Hearing,
    /// Protected vision, 5 points. BS 78
    #[default]
    Vision,
    /// Protected taste/smell, 5 points. BS 78
    TasteSmell,
    /// Protected touch, 5 points. BS 78
    Touch,
}

impl ProtectedSense {
    /// Returns the character point cost for this protected sense.
    ///
    /// # Citations
    ///
    /// BS 78 - Protected Sense costs
    pub fn cost(&self) -> i64 {
        match self {
            Self::Hearing => 5,
            Self::Vision => 5,
            Self::TasteSmell => 5,
            Self::Touch => 5,
        }
    }
}

/// Natural weapon for striking attacks.
///
/// # GURPS Rules
///
/// A body part adapted for attacking: tail, horns, hooves, etc.
/// Can be used for strikes in close combat.
///
/// # Citations
///
/// BS 88 - Striker
///
/// # Examples
///
/// ```
/// use valinoreth::Striker;
///
/// let striker = Striker::Crushing;
/// assert_eq!(striker.cost(), 5);
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
pub enum Striker {
    /// Crushing striker (tail, limb), 5 points. BS 88
    #[default]
    Crushing,
    /// Cutting striker (horns, talons), 7 points. BS 88
    Cutting,
    /// Impaling striker (horn, stinger), 8 points. BS 88
    Impaling,
}

impl Striker {
    /// Returns the character point cost for this striker type.
    ///
    /// # Citations
    ///
    /// BS 88 - Striker costs
    pub fn cost(&self) -> i64 {
        match self {
            Self::Crushing => 5,
            Self::Cutting => 7,
            Self::Impaling => 8,
        }
    }
}

/// Social regard affecting reactions.
///
/// # GURPS Rules
///
/// Society views you in a particular way, giving bonuses or penalties
/// to reactions from those who recognize your status.
///
/// # Citations
///
/// BS 86-87 - Social Regard
///
/// # Examples
///
/// ```
/// use valinoreth::SocialRegard;
///
/// let regard = SocialRegard::Respected;
/// assert_eq!(regard.cost(), 5);
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
pub enum SocialRegard {
    /// Feared by society, +1 to Intimidation. BS 86
    Feared,
    /// Respected by society, +1 to reactions. BS 87
    #[default]
    Respected,
    /// Venerated by society, +2 to reactions. BS 87
    Venerated,
}

impl SocialRegard {
    /// Returns the character point cost for this social regard.
    ///
    /// # Citations
    ///
    /// BS 86-87 - Social Regard costs
    pub fn cost(&self) -> i64 {
        match self {
            Self::Feared => 5,
            Self::Respected => 5,
            Self::Venerated => 10,
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

