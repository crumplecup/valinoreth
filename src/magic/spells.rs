//! GURPS magic spells.
//!
//! # GURPS Rules
//!
//! Spells are organized into colleges, have energy costs, prerequisites,
//! casting times, durations, and may allow resistance.
//!
//! # Citations
//!
//! - BS 239-253 - Magic system
//! - M 10-200 - Individual spells

use tracing::{debug, instrument};

/// Magic spell colleges.
///
/// # GURPS Rules
///
/// Spells are grouped into thematic colleges. Prerequisites often
/// require knowing other spells from the same college.
///
/// # Citations
///
/// M 10 - Spell colleges
///
/// # Examples
///
/// ```
/// use valinoreth::SpellCollege;
///
/// let college = SpellCollege::Knowledge;
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, strum::EnumIter)]
pub enum SpellCollege {
    /// Information and divination spells. M 106
    Knowledge,
}

/// GURPS magic spells (Knowledge college canary implementation).
///
/// # GURPS Rules
///
/// Spells require skill rolls (3d6 ≤ skill), expend energy (FP),
/// and have prerequisites (Magery, other spells).
///
/// # Citations
///
/// BS 239-253 - Magic system
/// M 106-113 - Knowledge college
///
/// # Examples
///
/// ```
/// use valinoreth::Spell;
///
/// let spell = Spell::DetectMagic;
/// assert_eq!(spell.college(), valinoreth::SpellCollege::Knowledge);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, strum::EnumIter)]
pub enum Spell {
    // Knowledge College - M 106-113
    /// Reveals presence of magic within radius. M 107
    DetectMagic,
    /// Analyzes magical items and effects. M 106
    AnalyzeMagic,
    /// Identifies specific spell being cast. M 109
    IdentifySpell,
    /// Locates specific object or person. M 112
    Seeker,
    /// Locates air or vacuum. M 111
    SeekAir,
    /// Locates earth or stone. M 111
    SeekEarth,
    /// Locates fire or heat. M 112
    SeekFire,
    /// Locates water or liquids. M 112
    SeekWater,
    /// Locates plants. M 112
    SeekPlant,
    /// Locates food. M 111
    SeekFood,
    /// Locates machines or technology. M 111
    SeekMachine,
    /// Reveals object's past. M 108
    History,
    /// Provides helpful insight or advice. M 113
    Wisdom,
    /// Reveals ancient historical events. M 106
    AncientHistory,
    /// Restores lost memories. M 110
    RecoverMemory,
    /// Answers yes/no questions about future. M 107
    Divination,
    /// Shows best path to destination. M 110
    Pathfinder,
    /// Makes wall transparent. M 108
    GlassWall,
    /// Tracks person or object. M 113
    Trace,
    /// Reveals character's nature and power. M 107
    Aura,
}

impl Spell {
    /// Returns the spell's college.
    ///
    /// # Citations
    ///
    /// M 106 - Knowledge college
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Spell, SpellCollege};
    ///
    /// assert_eq!(Spell::DetectMagic.college(), SpellCollege::Knowledge);
    /// assert_eq!(Spell::Seeker.college(), SpellCollege::Knowledge);
    /// ```
    #[instrument]
    pub fn college(&self) -> SpellCollege {
        debug!("Getting spell college");

        let college = match self {
            Self::DetectMagic => SpellCollege::Knowledge,
            Self::AnalyzeMagic => SpellCollege::Knowledge,
            Self::IdentifySpell => SpellCollege::Knowledge,
            Self::Seeker => SpellCollege::Knowledge,
            Self::SeekAir => SpellCollege::Knowledge,
            Self::SeekEarth => SpellCollege::Knowledge,
            Self::SeekFire => SpellCollege::Knowledge,
            Self::SeekWater => SpellCollege::Knowledge,
            Self::SeekPlant => SpellCollege::Knowledge,
            Self::SeekFood => SpellCollege::Knowledge,
            Self::SeekMachine => SpellCollege::Knowledge,
            Self::History => SpellCollege::Knowledge,
            Self::Wisdom => SpellCollege::Knowledge,
            Self::AncientHistory => SpellCollege::Knowledge,
            Self::RecoverMemory => SpellCollege::Knowledge,
            Self::Divination => SpellCollege::Knowledge,
            Self::Pathfinder => SpellCollege::Knowledge,
            Self::GlassWall => SpellCollege::Knowledge,
            Self::Trace => SpellCollege::Knowledge,
            Self::Aura => SpellCollege::Knowledge,
        };

        debug!(?college, "Spell college retrieved");
        college
    }

    /// Returns base energy cost to cast.
    ///
    /// # GURPS Rules
    ///
    /// Energy cost varies by spell and may scale with effect.
    /// Most Information spells cost 1-3 FP.
    ///
    /// # Citations
    ///
    /// BS 241 - Energy cost
    /// M 106-113 - Knowledge spell costs
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Spell, EnergyCost};
    ///
    /// assert_eq!(Spell::DetectMagic.base_energy_cost(), EnergyCost::Fixed(2));
    /// assert_eq!(Spell::Seeker.base_energy_cost(), EnergyCost::Fixed(3));
    /// ```
    #[instrument]
    pub fn base_energy_cost(&self) -> EnergyCost {
        debug!("Getting base energy cost");

        let cost = match self {
            Self::DetectMagic => EnergyCost::Fixed(2),
            Self::AnalyzeMagic => EnergyCost::Fixed(8),
            Self::IdentifySpell => EnergyCost::Fixed(2),
            Self::Seeker => EnergyCost::Fixed(3),
            Self::SeekAir => EnergyCost::Fixed(1),
            Self::SeekEarth => EnergyCost::Fixed(1),
            Self::SeekFire => EnergyCost::Fixed(1),
            Self::SeekWater => EnergyCost::Fixed(1),
            Self::SeekPlant => EnergyCost::Fixed(1),
            Self::SeekFood => EnergyCost::Fixed(1),
            Self::SeekMachine => EnergyCost::Fixed(1),
            Self::History => EnergyCost::Fixed(4),
            Self::Wisdom => EnergyCost::Fixed(8),
            Self::AncientHistory => EnergyCost::Fixed(5),
            Self::RecoverMemory => EnergyCost::Fixed(3),
            Self::Divination => EnergyCost::Fixed(4),
            Self::Pathfinder => EnergyCost::Fixed(3),
            Self::GlassWall => EnergyCost::Fixed(2),
            Self::Trace => EnergyCost::Fixed(2),
            Self::Aura => EnergyCost::Fixed(2),
        };

        debug!(?cost, "Energy cost retrieved");
        cost
    }

    /// Returns casting time in seconds.
    ///
    /// # GURPS Rules
    ///
    /// Most spells take 1-4 seconds to cast. Complex divinations
    /// may take longer.
    ///
    /// # Citations
    ///
    /// BS 241 - Casting time
    /// M 106-113 - Knowledge spell casting times
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Spell;
    ///
    /// assert_eq!(Spell::DetectMagic.casting_time(), 1);
    /// assert_eq!(Spell::Wisdom.casting_time(), 5);
    /// ```
    #[instrument]
    pub fn casting_time(&self) -> i32 {
        debug!("Getting casting time");

        let time = match self {
            Self::DetectMagic => 1,
            Self::AnalyzeMagic => 1,
            Self::IdentifySpell => 1,
            Self::Seeker => 2,
            Self::SeekAir => 1,
            Self::SeekEarth => 1,
            Self::SeekFire => 1,
            Self::SeekWater => 1,
            Self::SeekPlant => 1,
            Self::SeekFood => 1,
            Self::SeekMachine => 1,
            Self::History => 3,
            Self::Wisdom => 5,
            Self::AncientHistory => 4,
            Self::RecoverMemory => 2,
            Self::Divination => 5,
            Self::Pathfinder => 2,
            Self::GlassWall => 1,
            Self::Trace => 2,
            Self::Aura => 2,
        };

        debug!(time, "Casting time retrieved");
        time
    }

    /// Returns spell duration.
    ///
    /// # GURPS Rules
    ///
    /// Duration varies: instant (information revealed), concentration
    /// (active maintenance), or timed (minutes/hours).
    ///
    /// # Citations
    ///
    /// BS 241 - Duration
    /// M 106-113 - Knowledge spell durations
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Spell, Duration};
    ///
    /// assert_eq!(Spell::DetectMagic.duration(), Duration::Concentration);
    /// assert_eq!(Spell::History.duration(), Duration::Instant);
    /// ```
    #[instrument]
    pub fn duration(&self) -> Duration {
        debug!("Getting spell duration");

        let duration = match self {
            Self::DetectMagic => Duration::Concentration,
            Self::AnalyzeMagic => Duration::Instant,
            Self::IdentifySpell => Duration::Instant,
            Self::Seeker => Duration::Instant,
            Self::SeekAir => Duration::Concentration,
            Self::SeekEarth => Duration::Concentration,
            Self::SeekFire => Duration::Concentration,
            Self::SeekWater => Duration::Concentration,
            Self::SeekPlant => Duration::Concentration,
            Self::SeekFood => Duration::Concentration,
            Self::SeekMachine => Duration::Concentration,
            Self::History => Duration::Instant,
            Self::Wisdom => Duration::Instant,
            Self::AncientHistory => Duration::Instant,
            Self::RecoverMemory => Duration::Instant,
            Self::Divination => Duration::Instant,
            Self::Pathfinder => Duration::Instant,
            Self::GlassWall => Duration::Minutes(1),
            Self::Trace => Duration::Concentration,
            Self::Aura => Duration::Instant,
        };

        debug!(?duration, "Duration retrieved");
        duration
    }

    /// Returns spell prerequisites.
    ///
    /// # GURPS Rules
    ///
    /// Prerequisites include Magery level and other spells that must
    /// be known first.
    ///
    /// # Citations
    ///
    /// BS 241 - Prerequisites
    /// M 106-113 - Knowledge spell prerequisites
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Spell, SpellPrerequisite};
    ///
    /// let prereqs = Spell::DetectMagic.prerequisites();
    /// assert_eq!(prereqs.len(), 1);
    /// ```
    #[instrument]
    pub fn prerequisites(&self) -> Vec<SpellPrerequisite> {
        debug!("Getting spell prerequisites");

        let prereqs = match self {
            Self::DetectMagic => vec![SpellPrerequisite::Magery(0)],
            Self::AnalyzeMagic => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Knowledge, 6),
            ],
            Self::IdentifySpell => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::DetectMagic),
            ],
            Self::Seeker => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::DetectMagic),
            ],
            Self::SeekAir => vec![SpellPrerequisite::Magery(0)],
            Self::SeekEarth => vec![SpellPrerequisite::Magery(0)],
            Self::SeekFire => vec![SpellPrerequisite::Magery(0)],
            Self::SeekWater => vec![SpellPrerequisite::Magery(0)],
            Self::SeekPlant => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::SeekEarth),
            ],
            Self::SeekFood => vec![
                SpellPrerequisite::Magery(0),
                SpellPrerequisite::Spell(Self::SeekWater),
            ],
            Self::SeekMachine => vec![SpellPrerequisite::Magery(0)],
            Self::History => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Knowledge, 4),
            ],
            Self::Wisdom => vec![
                SpellPrerequisite::Magery(2),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Knowledge, 12),
                SpellPrerequisite::IQ(14),
            ],
            Self::AncientHistory => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::History),
            ],
            Self::RecoverMemory => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Knowledge, 4),
            ],
            Self::Divination => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::SpellsInCollege(SpellCollege::Knowledge, 6),
            ],
            Self::Pathfinder => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::Seeker),
            ],
            Self::GlassWall => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::DetectMagic),
            ],
            Self::Trace => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::Seeker),
            ],
            Self::Aura => vec![
                SpellPrerequisite::Magery(1),
                SpellPrerequisite::Spell(Self::DetectMagic),
            ],
        };

        debug!(count = prereqs.len(), "Prerequisites retrieved");
        prereqs
    }

    /// Returns spell type (for casting mechanics).
    ///
    /// # GURPS Rules
    ///
    /// Knowledge spells are mostly Information type - they reveal
    /// data rather than affecting targets directly.
    ///
    /// # Citations
    ///
    /// BS 239 - Spell types
    /// M 106 - Information spells
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Spell, SpellType};
    ///
    /// assert_eq!(Spell::DetectMagic.spell_type(), SpellType::Information);
    /// assert_eq!(Spell::GlassWall.spell_type(), SpellType::Area);
    /// ```
    #[instrument]
    pub fn spell_type(&self) -> SpellType {
        debug!("Getting spell type");

        let spell_type = match self {
            Self::DetectMagic => SpellType::Information,
            Self::AnalyzeMagic => SpellType::Information,
            Self::IdentifySpell => SpellType::Information,
            Self::Seeker => SpellType::Information,
            Self::SeekAir => SpellType::Information,
            Self::SeekEarth => SpellType::Information,
            Self::SeekFire => SpellType::Information,
            Self::SeekWater => SpellType::Information,
            Self::SeekPlant => SpellType::Information,
            Self::SeekFood => SpellType::Information,
            Self::SeekMachine => SpellType::Information,
            Self::History => SpellType::Information,
            Self::Wisdom => SpellType::Information,
            Self::AncientHistory => SpellType::Information,
            Self::RecoverMemory => SpellType::Regular,
            Self::Divination => SpellType::Information,
            Self::Pathfinder => SpellType::Information,
            Self::GlassWall => SpellType::Area,
            Self::Trace => SpellType::Information,
            Self::Aura => SpellType::Information,
        };

        debug!(?spell_type, "Spell type retrieved");
        spell_type
    }

    /// Returns resistance type if applicable.
    ///
    /// # GURPS Rules
    ///
    /// Most Knowledge spells don't allow resistance as they
    /// don't directly affect targets. Some may allow IQ resistance.
    ///
    /// # Citations
    ///
    /// BS 241 - Resistance
    /// M 106-113 - Knowledge spell resistance
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Spell;
    ///
    /// assert_eq!(Spell::DetectMagic.resistance(), None);
    /// assert_eq!(Spell::Aura.resistance(), None);
    /// ```
    #[instrument]
    pub fn resistance(&self) -> Option<ResistanceType> {
        debug!("Getting resistance type");

        let resistance = match self {
            Self::DetectMagic => None,
            Self::AnalyzeMagic => None,
            Self::IdentifySpell => None,
            Self::Seeker => None,
            Self::SeekAir => None,
            Self::SeekEarth => None,
            Self::SeekFire => None,
            Self::SeekWater => None,
            Self::SeekPlant => None,
            Self::SeekFood => None,
            Self::SeekMachine => None,
            Self::History => None,
            Self::Wisdom => None,
            Self::AncientHistory => None,
            Self::RecoverMemory => Some(ResistanceType::IQ),
            Self::Divination => None,
            Self::Pathfinder => None,
            Self::GlassWall => None,
            Self::Trace => None,
            Self::Aura => None,
        };

        debug!(?resistance, "Resistance retrieved");
        resistance
    }

    /// Returns spell citation reference.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Spell;
    ///
    /// assert_eq!(Spell::DetectMagic.reference(), "M107");
    /// assert_eq!(Spell::Seeker.reference(), "M112");
    /// ```
    #[instrument]
    pub fn reference(&self) -> &'static str {
        debug!("Getting spell reference");

        let reference = match self {
            Self::DetectMagic => "M107",
            Self::AnalyzeMagic => "M106",
            Self::IdentifySpell => "M109",
            Self::Seeker => "M112",
            Self::SeekAir => "M111",
            Self::SeekEarth => "M111",
            Self::SeekFire => "M112",
            Self::SeekWater => "M112",
            Self::SeekPlant => "M112",
            Self::SeekFood => "M111",
            Self::SeekMachine => "M111",
            Self::History => "M108",
            Self::Wisdom => "M113",
            Self::AncientHistory => "M106",
            Self::RecoverMemory => "M110",
            Self::Divination => "M107",
            Self::Pathfinder => "M110",
            Self::GlassWall => "M108",
            Self::Trace => "M113",
            Self::Aura => "M107",
        };

        debug!(reference, "Reference retrieved");
        reference
    }
}

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
