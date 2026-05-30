//! Character disadvantages providing points but imposing limitations.
//!
//! # GURPS Rules
//!
//! Disadvantages grant character points back (negative cost)
//! but impose behavioral restrictions, social penalties, or weaknesses.
//! Standard campaigns limit total disadvantages to -50 points.
//!
//! # Citations
//!
//! - BS 133-169 - Disadvantages
//! - BS 11 - Disadvantage limits
//!
//! # Examples
//!
//! ```
//! use valinoreth::{Disadvantage, SenseOfDuty};
//!
//! let disadvantage = Disadvantage::SenseOfDuty(SenseOfDuty::SmallGroup);
//! assert_eq!(disadvantage.cost(), -5);
//! ```

/// Character disadvantages providing points but imposing limitations.
///
/// # GURPS Rules
///
/// Disadvantages grant character points back (negative cost)
/// but impose behavioral restrictions, social penalties, or weaknesses.
/// Standard campaigns limit total disadvantages to -50 points.
///
/// # Citations
///
/// BS 133-169 - Disadvantage descriptions
/// BS 11 - Disadvantage limits
///
/// # Examples
///
/// ```
/// use valinoreth::{Disadvantage, SenseOfDuty};
///
/// let disadvantage = Disadvantage::SenseOfDuty(SenseOfDuty::SmallGroup);
/// assert_eq!(disadvantage.cost(), -5);
/// ```
#[derive(
    Debug,
    Copy,
    Clone,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Display,
)]
pub enum Disadvantage {
    /// Forgetful, distracted easily. BS 122
    AbsentMinded,
    /// Compulsive substance use. BS 122
    Addiction(Addiction),
    /// Back problems causing penalties. BS 123
    BadBack,
    /// Poor grip, drop things easily. BS 123
    BadGrip,
    /// Vision problems. BS 123
    BadSight(usize),
    /// Unpleasant body odor. BS 124
    BadSmell,
    /// Cannot see. BS 124
    Blind,
    /// Frenzy in combat. BS 124
    Berserk,
    /// Known for negative trait. BS 26
    BadReputation(usize),
    /// Compelled to kill in combat. BS 125
    Bloodlust,
    /// Indifferent to others' suffering. BS 125
    Callous,
    /// Cannot distinguish colors. BS 127
    Colorblindness,
    /// Must perform specific behavior. BS 128
    Compulsive(usize),
    /// Personal code of behavior, variable cost. BS 127
    CodeOfHonor(usize),
    /// Avoid danger at all costs. BS 129
    Cowardice,
    /// Compelled to be curious. BS 129
    Curiosity,
    /// Cannot hear. BS 129
    Deafness,
    /// False belief. BS 130
    Delusion(usize),
    /// Need specific substance to survive. BS 130
    Dependency(usize),
    /// Fated for great deeds, -5 to -15 points per level. BS 131
    Destiny(usize),
    /// Obligation to organization or cause. BS 133
    Duty(Duty),
    /// Very short stature. BS 134
    Dwarfism,
    /// Someone actively opposes you. BS 135
    Enemy(usize),
    /// Overweight, movement penalties. BS 135
    Fat,
    /// Extra damage from attacks. BS 136
    Fragile,
    /// Compelled to eat excessively. BS 137
    Gluttony,
    /// Compelled to acquire wealth. BS 137
    Greed,
    /// Poor manual dexterity. BS 138
    HamFisted,
    /// Hearing penalty. BS 138
    HardOfHearing,
    /// Blood doesn't clot. BS 138
    Hemophilia,
    /// Must obey the law and tell the truth, -10 points. BS 138
    Honesty,
    /// Spinal deformity. BS 139
    Hunchback,
    /// Act without thinking. BS 139
    Impulsiveness,
    /// Cause bad luck around you. BS 140
    Jinxed,
    /// Prejudice against specific group. BS 140
    Intolerance,
    /// Avoid work. BS 142
    Laziness,
    /// Compelled to seduce. BS 142
    Lecherousness,
    /// Movement impairment. BS 141
    Lame(Lame),
    /// Poor self-esteem. BS 143
    LowSelfImage,
    /// Lost finger or toe. BS 142
    MissingDigit,
    /// Nausea from movement. BS 144
    MotionSickness,
    /// Cannot speak. BS 125
    Mute,
    /// Nightmares disturb sleep. BS 144
    Nightmares,
    /// Can't judge distances. BS 145
    NoDepthPerception,
    /// Missing sense of smell and taste. BS 146
    NoSenseOfSmellTaste,
    /// Annoying personal habits. BS 146
    OdiousPersonalHabits(usize),
    /// Missing an arm. BS 147
    OneArm,
    /// Missing an eye. BS 147
    OneEye,
    /// Missing a hand. BS 147
    OneHand,
    /// Fixation on goal or subject. BS 146
    Obsession,
    /// Overestimate abilities. BS 148
    Overconfidence,
    /// Missing a leg. BS 142
    OneLeg,
    /// Paralyzed lower body. BS 141
    Paraplegic,
    /// Paralyzed all limbs. BS 141
    Quadriplegic,
    /// Distrust everyone. BS 148
    Paranoia,
    /// Irrational fear. BS 148
    Phobia(Phobia),
    /// Shaken after combat. BS 150
    PostCombatShakes,
    /// Compulsion to start fires. BS 150
    Pyromania,
    /// Fewer hit points than normal. BS 16
    ReducedHitPoints(usize),
    /// Dangerous information if revealed. BS 152
    Secret(usize),
    /// Uncontrollable appetite. BS 153
    UncontrollableAppetite,
    /// Put others first, -5 points. BS 153
    Selfless,
    /// Enjoy causing pain. BS 152
    Sadism,
    /// Feel compulsion to protect group. BS 153
    SenseOfDuty(SenseOfDuty),
    /// Disturbed by blood and violence. BS 156
    Squeamish,
    /// Social penalty, -5 per level (max -20). BS 155
    SocialStigma(usize),
    /// Low social standing, -5 per level. BS 28
    Status(usize),
    /// -1 to reactions, must make Will roll to change mind, -5 points. BS 157
    Stubborn,
    /// Speech impediment. BS 157
    Stuttering,
    /// Compelled to play pranks. BS 159
    Trickster,
    /// Underweight, reduced HP. BS 19
    Skinny,
    /// Personal pledge or promise. BS 160
    Vow(Vow),
    /// Extra damage from specific attack. BS 161
    Vulnerability(usize),
    /// Low strength. BS 14
    Weak(usize),
    /// Harmed by specific substance. BS 161
    WeaknessTo(usize),
}

impl Disadvantage {
    /// Calculates the character point cost (negative) of this disadvantage.
    ///
    /// # GURPS Rules
    ///
    /// Disadvantages provide points back (negative cost).
    /// The magnitude represents the severity of the limitation.
    ///
    /// # Returns
    ///
    /// Negative character points (or zero for unimplemented variants).
    ///
    /// # Citations
    ///
    /// BS 133-169 - Individual disadvantage costs
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Disadvantage, Duty};
    ///
    /// let duty = Disadvantage::Duty(Duty::QuiteOften);
    /// assert_eq!(duty.cost(), -10);
    /// ```
    pub fn cost(&self) -> i64 {
        match self {
            // BS 122
            Self::AbsentMinded => -15,
            Self::Addiction(level) => level.cost(),
            // BS 123
            Self::BadBack => -15,
            // BS 123
            Self::BadGrip => -5,
            // BS 123 - varies by severity
            Self::BadSight(level) => -(*level as i64 * 10),
            // BS 124
            Self::BadSmell => -10,
            // BS 26 - varies by scope and frequency
            Self::BadReputation(level) => -(*level as i64),
            // BS 124
            Self::Berserk => -10,
            // BS 124
            Self::Blind => -50,
            // BS 125
            Self::Bloodlust => -10,
            // BS 125
            Self::Callous => -5,
            // BS 127
            Self::CodeOfHonor(level) => -(*level as i64),
            // BS 127
            Self::Colorblindness => -10,
            // BS 128
            Self::Compulsive(level) => -(*level as i64),
            // BS 129
            Self::Cowardice => -10,
            // BS 129
            Self::Curiosity => -5,
            // BS 129
            Self::Deafness => -20,
            // BS 130
            Self::Delusion(level) => -(*level as i64),
            // BS 130 - varies greatly by frequency and rarity
            Self::Dependency(level) => -(*level as i64),
            // BS 131
            Self::Destiny(level) => -(*level as i64),
            Self::Duty(level) => level.cost(),
            // BS 134
            Self::Dwarfism => -15,
            // BS 135 - varies greatly by power and frequency
            Self::Enemy(level) => -(*level as i64),
            // BS 135
            Self::Fat => -3,
            // BS 136
            Self::Fragile => -20,
            // BS 137
            Self::Gluttony => -5,
            // BS 137
            Self::Greed => -15,
            // BS 138
            Self::HamFisted => -5,
            // BS 138
            Self::HardOfHearing => -10,
            // BS 138
            Self::Hemophilia => -30,
            // BS 138
            Self::Honesty => -10,
            // BS 139
            Self::Hunchback => -10,
            // BS 139
            Self::Impulsiveness => -10,
            // BS 140
            Self::Intolerance => -5,
            // BS 140
            Self::Jinxed => -60,
            Self::Lame(level) => level.cost(),
            // BS 142
            Self::Laziness => -10,
            // BS 142
            Self::Lecherousness => -15,
            // BS 143
            Self::LowSelfImage => -10,
            // BS 142
            Self::MissingDigit => -2,
            // BS 144
            Self::MotionSickness => -10,
            // BS 125
            Self::Mute => -25,
            // BS 144
            Self::Nightmares => -5,
            // BS 145
            Self::NoDepthPerception => -15,
            // BS 146
            Self::NoSenseOfSmellTaste => -5,
            // BS 146 - varies by severity
            Self::OdiousPersonalHabits(level) => -(*level as i64),
            // BS 147
            Self::OneArm => -20,
            // BS 147
            Self::OneEye => -15,
            // BS 147
            Self::OneHand => -15,
            // BS 142
            Self::OneLeg => -20,
            // BS 146
            Self::Obsession => -10,
            // BS 148
            Self::Overconfidence => -5,
            // BS 141
            Self::Paraplegic => -30,
            // BS 141
            Self::Quadriplegic => -80,
            // BS 148
            Self::Paranoia => -10,
            Self::Phobia(level) => level.cost(),
            // BS 150
            Self::PostCombatShakes => -5,
            // BS 150
            Self::Pyromania => -5,
            // BS 16
            Self::ReducedHitPoints(level) => -(*level as i64 * 2),
            // BS 152 - varies by severity and frequency
            Self::Secret(level) => -(*level as i64),
            // BS 152
            Self::Sadism => -15,
            // BS 153
            Self::UncontrollableAppetite => -15,
            // BS 153
            Self::Selfless => -5,
            Self::SenseOfDuty(level) => level.cost(),
            // BS 156
            Self::Squeamish => -10,
            // -5 points per level, max level 4 BS 155
            Self::SocialStigma(level) => -(*level as i64 * 5),
            // BS 28
            Self::Status(level) => -(*level as i64 * 5),
            // BS 19
            Self::Skinny => -5,
            // BS 157
            Self::Stubborn => -5,
            // BS 157
            Self::Stuttering => -10,
            // BS 159
            Self::Trickster => -15,
            Self::Vow(level) => level.cost(),
            // BS 161 - varies by commonality and damage multiplier
            Self::Vulnerability(level) => -(*level as i64),
            // BS 14 - 10 points per level of ST reduction
            Self::Weak(level) => -(*level as i64 * 10),
            // BS 161 - varies by rarity and severity
            Self::WeaknessTo(level) => -(*level as i64),
        }
    }
}

/// Compulsion to protect or aid a group.
///
/// # GURPS Rules
///
/// You feel compelled to protect members of the group,
/// even at risk to yourself. Scope determines point value.
///
/// # Citations
///
/// BS 153 - Sense of Duty
///
/// # Examples
///
/// ```
/// use valinoreth::SenseOfDuty;
///
/// let duty = SenseOfDuty::LargeGroup;
/// assert_eq!(duty.cost(), -10);
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
pub enum SenseOfDuty {
    /// One person, -2 points. BS 153
    Individual,
    /// Small group (adventuring party), -5 points. BS 153
    SmallGroup,
    /// Large group (nation, religion), -10 points. BS 153
    LargeGroup,
    /// Entire intelligent race, -15 points. BS 153
    EntireRace,
    /// All living beings, -20 points. BS 153
    EveryLivingBeing,
}

impl SenseOfDuty {
    /// Returns the character point cost for this sense of duty.
    ///
    /// # Citations
    ///
    /// BS 153 - Sense of Duty costs
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

/// Frequency of duty to an organization or cause.
///
/// # GURPS Rules
///
/// Duty represents an obligation that takes precedence over
/// personal desires. Frequency determines point value.
///
/// # Citations
///
/// BS 133 - Duty disadvantage
///
/// # Examples
///
/// ```
/// use valinoreth::Duty;
///
/// let duty = Duty::QuiteOften;
/// assert_eq!(duty.cost(), -10);
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
pub enum Duty {
    /// 15 or less on 3d6, -15 points. BS 133
    AlmostAlways,
    /// 12 or less on 3d6, -10 points. BS 133
    QuiteOften,
    /// 9 or less on 3d6, -5 points. BS 133
    FairlyOften,
    /// 6 or less on 3d6, -2 points. BS 133
    QuiteRarely,
}

impl Duty {
    /// Returns the character point cost for this duty frequency.
    ///
    /// # Citations
    ///
    /// BS 133 - Duty costs by frequency
    pub fn cost(&self) -> i64 {
        match self {
            Self::AlmostAlways => -15,
            Self::QuiteOften => -10,
            Self::FairlyOften => -5,
            Self::QuiteRarely => -2,
        }
    }
}

/// Movement impairment severity.
///
/// # GURPS Rules
///
/// Lame reduces your Move score and may require crutches or wheelchair.
/// Severity determines point cost and mobility reduction.
///
/// # Citations
///
/// BS 141-142 - Lame disadvantage
///
/// # Examples
///
/// ```
/// use valinoreth::Lame;
///
/// let lame = Lame::Crippled;
/// assert_eq!(lame.cost(), -10);
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
pub enum Lame {
    /// Move reduced to 60%, -5 points. BS 141
    #[default]
    Limp,
    /// Move reduced by 40%, needs cane, -10 points. BS 141
    Crippled,
    /// Cannot walk, needs wheelchair, -15 points. BS 142
    Legless,
}

impl Lame {
    /// Returns the character point cost for this lame severity.
    ///
    /// # Citations
    ///
    /// BS 141-142 - Lame costs
    pub fn cost(&self) -> i64 {
        match self {
            Self::Limp => -5,
            Self::Crippled => -10,
            Self::Legless => -15,
        }
    }
}

/// Addiction severity and frequency.
///
/// # GURPS Rules
///
/// Addiction represents compulsive substance use.
/// More addictive substances and frequent use cost more points.
///
/// # Citations
///
/// BS 122 - Addiction disadvantage
///
/// # Examples
///
/// ```
/// use valinoreth::Addiction;
///
/// let addiction = Addiction::Highly;
/// assert_eq!(addiction.cost(), -20);
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
pub enum Addiction {
    /// Mildly addictive, -5 points. BS 122
    Mild,
    /// Addictive, -10 points. BS 122
    #[default]
    Moderate,
    /// Highly addictive, -20 points. BS 122
    Highly,
}

impl Addiction {
    /// Returns the character point cost for this addiction severity.
    ///
    /// # Citations
    ///
    /// BS 122 - Addiction costs
    pub fn cost(&self) -> i64 {
        match self {
            Self::Mild => -5,
            Self::Moderate => -10,
            Self::Highly => -20,
        }
    }
}

/// Phobia severity.
///
/// # GURPS Rules
///
/// Phobias are irrational fears requiring self-control rolls.
/// More severe phobias are harder to resist and cost more points.
///
/// # Citations
///
/// BS 148-150 - Phobia disadvantage
///
/// # Examples
///
/// ```
/// use valinoreth::Phobia;
///
/// let phobia = Phobia::Severe;
/// assert_eq!(phobia.cost(), -15);
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
pub enum Phobia {
    /// Mild phobia, self-control at +6, -5 points. BS 148
    Mild,
    /// Moderate phobia, normal self-control, -10 points. BS 148
    #[default]
    Moderate,
    /// Severe phobia, self-control at -3, -15 points. BS 149
    Severe,
    /// Extreme phobia, self-control at -6, -20 points. BS 149
    Extreme,
}

impl Phobia {
    /// Returns the character point cost for this phobia severity.
    ///
    /// # Citations
    ///
    /// BS 148-150 - Phobia costs
    pub fn cost(&self) -> i64 {
        match self {
            Self::Mild => -5,
            Self::Moderate => -10,
            Self::Severe => -15,
            Self::Extreme => -20,
        }
    }
}

/// Vow severity and restrictiveness.
///
/// # GURPS Rules
///
/// A vow is a personal pledge that restricts your behavior.
/// More restrictive vows provide more character points.
///
/// # Citations
///
/// BS 160-161 - Vow disadvantage
///
/// # Examples
///
/// ```
/// use valinoreth::Vow;
///
/// let vow = Vow::Major;
/// assert_eq!(vow.cost(), -10);
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
pub enum Vow {
    /// Minor behavioral restriction, -5 points. BS 160
    Minor,
    /// Significant restriction, -10 points. BS 160
    #[default]
    Major,
    /// Extreme lifestyle restriction, -15 points. BS 161
    Great,
}

impl Vow {
    /// Returns the character point cost for this vow severity.
    ///
    /// # Citations
    ///
    /// BS 160-161 - Vow costs
    pub fn cost(&self) -> i64 {
        match self {
            Self::Minor => -5,
            Self::Major => -10,
            Self::Great => -15,
        }
    }
}
