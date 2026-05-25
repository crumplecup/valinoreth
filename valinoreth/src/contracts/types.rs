//! Type descriptors for GURPS combat operations.
//!
//! These types describe game operations without containing proof tokens.
//! They serve as parameters to trait methods that return evidence bundles.
//!
//! # Pattern
//!
//! Descriptor types:
//! - Are plain data structures (no proof tokens)
//! - Use builders for construction
//! - Derive `Serialize`, `Deserialize`, `JsonSchema` for MCP integration
//! - Serve as inputs to trait methods that produce `Established<P>` proofs

#[cfg(not(creusot))]
use derive_builder::Builder;
#[cfg(not(creusot))]
use schemars::JsonSchema;
#[cfg(not(creusot))]
use serde::{Deserialize, Serialize};

// ── Attack Descriptors ────────────────────────────────────────────────────────

/// Describes an attack attempt.
///
/// Contains all information needed to resolve an attack roll:
/// skill level, modifiers, and optional target location.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(creusot), derive(Builder))]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
#[cfg_attr(not(creusot), builder(setter(into)))]
pub struct AttackDescriptor {
    /// Attacker's effective skill level (base skill + modifiers)
    pub effective_skill: i32,

    /// Whether this is an All-Out Attack
    #[cfg_attr(not(creusot), builder(default))]
    pub all_out_attack: bool,

    /// Deceptive Attack skill penalty (reduces opponent defense by half this)
    #[cfg_attr(not(creusot), builder(default))]
    pub deceptive_penalty: i32,

    /// Aim bonus accumulated (ranged attacks only)
    #[cfg_attr(not(creusot), builder(default))]
    pub aim_bonus: i32,

    /// Target hit location (if targeted attack)
    #[cfg_attr(not(creusot), builder(default))]
    pub target_location: Option<HitLocation>,
}

/// Describes the result of an attack roll.
///
/// Contains the roll result and outcome determination.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub struct AttackRollResult {
    /// The 3d6 roll result
    pub roll: i32,

    /// The effective skill that was rolled against
    pub effective_skill: i32,

    /// Whether the attack succeeded (roll ≤ effective_skill)
    pub success: bool,

    /// Margin of success (if success) or failure (if failure)
    pub margin: i32,

    /// Whether this was a critical success
    pub critical_success: bool,

    /// Whether this was a critical failure
    pub critical_failure: bool,
}

// ── Defense Descriptors ───────────────────────────────────────────────────────

/// Describes a defense attempt.
///
/// Contains all information needed to resolve an active defense roll.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(creusot), derive(Builder))]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
#[cfg_attr(not(creusot), builder(setter(into)))]
pub struct DefenseDescriptor {
    /// Type of active defense (Dodge, Parry, Block)
    pub defense_type: DefenseType,

    /// Defender's defense score
    pub defense_score: i32,

    /// Penalty from Feint (if opponent feinted successfully)
    #[cfg_attr(not(creusot), builder(default))]
    pub feint_penalty: i32,

    /// Penalty from Deceptive Attack
    #[cfg_attr(not(creusot), builder(default))]
    pub deceptive_penalty: i32,

    /// Whether defender is retreating (+3 to defense)
    #[cfg_attr(not(creusot), builder(default))]
    pub retreating: bool,
}

/// Type of active defense.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub enum DefenseType {
    /// Dodge: 3d6 ≤ Dodge score (DX + 3 + bonuses)
    Dodge,
    /// Parry: 3d6 ≤ Parry score (skill/2 + 3 + bonuses)
    Parry,
    /// Block: 3d6 ≤ Block score (skill/2 + 3 + bonuses)
    Block,
}

/// Describes the result of a defense roll.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub struct DefenseRollResult {
    /// The 3d6 roll result
    pub roll: i32,

    /// The defense score that was rolled against
    pub defense_score: i32,

    /// Whether the defense succeeded (roll ≤ defense_score)
    pub success: bool,

    /// Margin of success (if success) or failure (if failure)
    pub margin: i32,

    /// Whether this was a critical success
    pub critical_success: bool,

    /// Whether this was a critical failure
    pub critical_failure: bool,
}

// ── Damage Descriptors ────────────────────────────────────────────────────────

/// Describes weapon damage to be rolled.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(creusot), derive(Builder))]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
#[cfg_attr(not(creusot), builder(setter(into)))]
pub struct DamageDescriptor {
    /// Number of dice to roll
    pub dice: i32,

    /// Number of sides per die (usually 6)
    pub sides: i32,

    /// Flat modifier to add to roll
    #[cfg_attr(not(creusot), builder(default))]
    pub modifier: i32,

    /// Type of damage (affects wounding multiplier)
    pub damage_type: DamageTypeDescriptor,

    /// Bonus damage from critical hit
    #[cfg_attr(not(creusot), builder(default))]
    pub critical_bonus: i32,
}

/// Type of damage for wounding modifier calculation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub enum DamageTypeDescriptor {
    /// Crushing damage: ×1 wounding
    Crushing,
    /// Cutting damage: ×1.5 wounding
    Cutting,
    /// Impaling damage: ×2 wounding (×3 to vitals)
    Impaling,
    /// Piercing damage: ×1.5 wounding (×3 to vitals)
    Piercing,
    /// Burning damage: ×1 wounding
    Burning,
}

/// Describes armor protection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub struct ArmorDescriptor {
    /// Damage Resistance value
    pub dr: i32,

    /// Whether armor is flexible (affects crushing damage)
    pub flexible: bool,
}

/// Hit location on the body.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub enum HitLocation {
    /// Skull: ×4 damage multiplier, -7 to hit
    Skull,
    /// Face: ×1.5 damage multiplier, -5 to hit
    Face,
    /// Eye: special effects, -9 to hit
    Eye,
    /// Neck: ×1.5 damage multiplier, -5 to hit (crushing ×1)
    Neck,
    /// Torso: ×1 damage multiplier, no penalty
    Torso,
    /// Vitals: ×3 damage multiplier for impaling/piercing, -3 to hit
    Vitals,
    /// Groin: ×1 damage multiplier, -3 to hit (special effects)
    Groin,
    /// Arm: ×1 damage multiplier, -2 to hit (limb crippling)
    Arm,
    /// Hand: ×1 damage multiplier, -4 to hit (limb crippling)
    Hand,
    /// Leg: ×1 damage multiplier, -2 to hit (limb crippling)
    Leg,
    /// Foot: ×1 damage multiplier, -4 to hit (limb crippling)
    Foot,
}

impl HitLocation {
    /// Returns the damage multiplier for this hit location.
    ///
    /// # GURPS Rules
    ///
    /// Different body locations multiply damage differently.
    /// Some locations have damage type-specific multipliers.
    ///
    /// # Citations
    ///
    /// BS 398-400 - Hit location effects
    /// BS 552 - Hit location table
    pub fn damage_multiplier(&self, damage_type: DamageTypeDescriptor) -> f32 {
        match (self, damage_type) {
            (Self::Skull, _) => 4.0,
            (Self::Face, _) => 1.5,
            (Self::Eye, _) => 1.0, // Special effects, not multiplier
            (Self::Neck, DamageTypeDescriptor::Crushing) => 1.0,
            (Self::Neck, _) => 1.5,
            (Self::Torso, _) => 1.0,
            (Self::Vitals, DamageTypeDescriptor::Impaling | DamageTypeDescriptor::Piercing) => 3.0,
            (Self::Vitals, _) => 1.0,
            (Self::Groin, _) => 1.0,
            (Self::Arm, _) => 1.0,
            (Self::Hand, _) => 1.0,
            (Self::Leg, _) => 1.0,
            (Self::Foot, _) => 1.0,
        }
    }

    /// Returns the to-hit penalty for targeting this location.
    ///
    /// # GURPS Rules
    ///
    /// Targeted attacks suffer penalties based on the difficulty
    /// of hitting the specific location.
    ///
    /// # Citations
    ///
    /// BS 398 - Targeted attacks
    pub fn to_hit_penalty(&self) -> i32 {
        match self {
            Self::Skull => -7,
            Self::Face => -5,
            Self::Eye => -9,
            Self::Neck => -5,
            Self::Torso => 0,
            Self::Vitals => -3,
            Self::Groin => -3,
            Self::Arm => -2,
            Self::Hand => -4,
            Self::Leg => -2,
            Self::Foot => -4,
        }
    }
}

/// Describes the result of damage calculation.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub struct DamageResult {
    /// Raw damage rolled
    pub raw_damage: i32,

    /// Damage Resistance applied
    pub dr: i32,

    /// Penetrating damage (raw - DR)
    pub penetrating_damage: i32,

    /// Hit location struck
    pub location: HitLocation,

    /// Location damage multiplier applied
    pub location_multiplier: f32,

    /// Wounding modifier for damage type
    pub wounding_multiplier: f32,

    /// Final injury to HP
    pub injury: i32,
}

// ── Special Maneuver Descriptors ──────────────────────────────────────────────

/// Describes a Feint maneuver.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub struct FeintDescriptor {
    /// Attacker's skill for the Quick Contest
    pub attacker_skill: i32,

    /// Defender's skill for the Quick Contest
    pub defender_skill: i32,
}

/// Describes the result of a Feint contest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub struct FeintResult {
    /// Attacker's roll
    pub attacker_roll: i32,

    /// Defender's roll
    pub defender_roll: i32,

    /// Whether attacker won the contest
    pub attacker_success: bool,

    /// Margin of victory (penalty to opponent's next defense)
    pub margin: i32,
}

/// Describes a Rapid Strike maneuver.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub struct RapidStrikeDescriptor {
    /// Number of attacks in the rapid strike
    pub attack_count: usize,

    /// Base skill before Rapid Strike penalty
    pub base_skill: i32,

    /// Penalty per attack (-6 normal, -3 with Weapon Master)
    pub penalty_per_attack: i32,
}

// ── Combat State Descriptors ──────────────────────────────────────────────────

/// Describes complete combat state for a character.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(creusot), derive(Builder))]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
#[cfg_attr(not(creusot), builder(setter(into)))]
pub struct CombatantDescriptor {
    /// Character's current Hit Points
    pub current_hp: i32,

    /// Character's maximum Hit Points
    pub max_hp: i32,

    /// Character's Dodge score
    pub dodge: i32,

    /// Character's Parry score (if applicable)
    #[cfg_attr(not(creusot), builder(default))]
    pub parry: Option<i32>,

    /// Character's Block score (if applicable)
    #[cfg_attr(not(creusot), builder(default))]
    pub block: Option<i32>,

    /// Whether character has acted this turn
    #[cfg_attr(not(creusot), builder(default))]
    pub has_acted: bool,

    /// Whether character used All-Out Attack (no defenses until next turn)
    #[cfg_attr(not(creusot), builder(default))]
    pub all_out_attack_used: bool,

    /// Accumulated penalties from wounds, stunning, etc.
    #[cfg_attr(not(creusot), builder(default))]
    pub shock_penalty: i32,
}

// ── Skill Check Descriptors ───────────────────────────────────────────────────

/// Describes a skill check attempt.
///
/// Contains all information needed to resolve a skill check:
/// skill level, modifiers, and task difficulty.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(creusot), derive(Builder))]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
#[cfg_attr(not(creusot), builder(setter(into)))]
pub struct SkillCheckDescriptor {
    /// Character's effective skill level (base skill + modifiers)
    pub effective_skill: i32,

    /// Situational modifiers (equipment, lighting, etc.)
    #[cfg_attr(not(creusot), builder(default))]
    pub situational_modifier: i32,

    /// Task difficulty modifier (-10 to +10)
    #[cfg_attr(not(creusot), builder(default))]
    pub task_difficulty: i32,

    /// Time spent modifier (rushing or taking extra time)
    #[cfg_attr(not(creusot), builder(default))]
    pub time_modifier: i32,

    /// Whether using complementary skill bonus
    #[cfg_attr(not(creusot), builder(default))]
    pub complementary_bonus: Option<i32>,
}

/// Describes the result of a skill check roll.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub struct SkillCheckResult {
    /// The 3d6 roll result
    pub roll: i32,

    /// The effective skill that was rolled against
    pub effective_skill: i32,

    /// Whether the skill check succeeded (roll ≤ effective_skill)
    pub success: bool,

    /// Margin of success (if success) or failure (if failure)
    pub margin: i32,

    /// Whether this was a critical success
    pub critical_success: bool,

    /// Whether this was a critical failure
    pub critical_failure: bool,
}

/// Describes a skill for a character.
#[derive(Debug, Clone)]
#[cfg_attr(not(creusot), derive(Eq))]
#[cfg_attr(not(creusot), derive(PartialEq))]
#[cfg_attr(not(creusot), derive(Builder))]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
#[cfg_attr(not(creusot), builder(setter(into)))]
pub struct SkillDescriptor {
    /// Skill name
    pub name: String,

    /// Skill specialization (if any)
    #[cfg_attr(not(creusot), builder(default))]
    pub specialization: Option<String>,

    /// Skill difficulty (Easy, Average, Hard, Very Hard)
    pub difficulty: SkillDifficulty,

    /// Base attribute (DX, IQ, HT, Per, Will)
    pub base_attribute: AttributeType,

    /// Points invested in the skill
    pub points: i32,

    /// Current skill level
    pub level: i32,

    /// Whether this is a wildcard skill
    #[cfg_attr(not(creusot), builder(default))]
    pub wildcard: bool,
}

/// Skill difficulty levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub enum SkillDifficulty {
    /// Easy skills (E)
    Easy,
    /// Average skills (A)
    Average,
    /// Hard skills (H)
    Hard,
    /// Very Hard skills (VH)
    VeryHard,
}

/// Describes a skill default (fallback when skill not trained).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub struct SkillDefaultDescriptor {
    /// What the skill defaults to
    pub default_type: SkillDefaultType,

    /// Penalty when using default
    pub penalty: i32,
}

/// Types of skill defaults.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub enum SkillDefaultType {
    /// Defaults to an attribute (DX, IQ, etc.)
    Attribute(AttributeType),
    /// Defaults to a related skill
    Skill {
        /// Name of the skill to default to
        name: String,
        /// Specialization (if any)
        specialization: Option<String>,
    },
    /// No default (skill must be learned)
    None,
}

// ── Character Creation Descriptors ────────────────────────────────────────────

/// Describes a character's primary attribute.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(not(creusot), derive(Eq))]
#[cfg_attr(not(creusot), derive(PartialEq))]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub struct AttributeDescriptor {
    /// Attribute type (ST, DX, IQ, HT)
    pub attribute_type: AttributeType,

    /// Attribute level (typically 1-20)
    pub level: i32,

    /// Point cost (10 or 20 per level based on type)
    pub cost: i32,
}

/// Primary attribute types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub enum AttributeType {
    /// Strength
    ST,
    /// Dexterity
    DX,
    /// Intelligence
    IQ,
    /// Health
    HT,
    /// Perception (secondary, defaults to IQ)
    Per,
    /// Will (secondary, defaults to IQ)
    Will,
}

/// Describes a secondary characteristic purchase.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(not(creusot), derive(Eq))]
#[cfg_attr(not(creusot), derive(PartialEq))]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub struct SecondaryCharacteristicDescriptor {
    /// Type of secondary characteristic
    pub characteristic_type: SecondaryCharacteristicType,

    /// Levels bought above/below default
    pub levels: i32,

    /// Point cost
    pub cost: i32,
}

/// Secondary characteristic types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub enum SecondaryCharacteristicType {
    /// Hit Points (defaults to ST)
    HP,
    /// Will (defaults to IQ)
    Will,
    /// Perception (defaults to IQ)
    Per,
    /// Fatigue Points (defaults to HT)
    FP,
    /// Basic Speed (defaults to (DX+HT)/4)
    BasicSpeed,
    /// Basic Move (defaults to Basic Speed)
    BasicMove,
}

/// Describes an advantage for a character.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(creusot), derive(Builder))]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
#[cfg_attr(not(creusot), builder(setter(into)))]
pub struct AdvantageDescriptor {
    /// Advantage name
    pub name: String,

    /// Base point cost
    pub base_cost: i32,

    /// Level (if leveled advantage)
    #[cfg_attr(not(creusot), builder(default))]
    pub level: Option<i32>,

    /// Enhancement modifiers (percentage bonuses)
    #[cfg_attr(not(creusot), builder(default))]
    pub enhancements: Vec<ModifierDescriptor>,

    /// Limitation modifiers (percentage penalties)
    #[cfg_attr(not(creusot), builder(default))]
    pub limitations: Vec<ModifierDescriptor>,

    /// Final point cost after modifiers
    pub final_cost: i32,
}

/// Describes a disadvantage for a character.
#[derive(Debug, Clone)]
#[cfg_attr(not(creusot), derive(PartialEq))]
#[cfg_attr(not(creusot), derive(Builder))]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
#[cfg_attr(not(creusot), builder(setter(into)))]
pub struct DisadvantageDescriptor {
    /// Disadvantage name
    pub name: String,

    /// Base point value (negative)
    pub base_cost: i32,

    /// Level (if leveled disadvantage)
    #[cfg_attr(not(creusot), builder(default))]
    pub level: Option<i32>,

    /// Self-control roll (6, 9, 12, 15) if applicable
    #[cfg_attr(not(creusot), builder(default))]
    pub self_control: Option<i32>,

    /// Final point value (negative)
    pub final_cost: i32,
}

/// Describes an enhancement or limitation modifier.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub struct ModifierDescriptor {
    /// Modifier name
    pub name: String,

    /// Percentage modifier (+/-10, +/-20, etc.)
    pub percentage: i32,
}

/// Describes complete character creation parameters.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(creusot), derive(Builder))]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
#[cfg_attr(not(creusot), builder(setter(into)))]
pub struct CharacterCreationDescriptor {
    /// Character name
    pub name: String,

    /// Character description
    #[cfg_attr(not(creusot), builder(default))]
    pub description: String,

    /// Total character point value (100, 150, 200, etc.)
    pub total_points: i32,

    /// Disadvantage point limit (typically -50)
    #[cfg_attr(not(creusot), builder(default = "-50"))]
    pub disadvantage_limit: i32,

    /// Campaign attribute minimums
    #[cfg_attr(not(creusot), builder(default))]
    pub attribute_minimums: Option<AttributeMinimums>,
}

/// Campaign-specific attribute minimums.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub struct AttributeMinimums {
    /// Minimum ST
    pub st: i32,
    /// Minimum DX
    pub dx: i32,
    /// Minimum IQ
    pub iq: i32,
    /// Minimum HT
    pub ht: i32,
}

/// Describes a complete character state.
#[derive(Debug, Clone)]
#[cfg_attr(not(creusot), derive(PartialEq))]
#[cfg_attr(not(creusot), derive(Builder))]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
#[cfg_attr(not(creusot), builder(setter(into)))]
pub struct CharacterDescriptor {
    /// Character name
    pub name: String,

    /// Character description
    #[cfg_attr(not(creusot), builder(default))]
    pub description: String,

    /// Total character point value
    pub total_points: i32,

    /// Points spent so far
    pub points_spent: i32,

    /// Primary attributes
    pub attributes: Vec<AttributeDescriptor>,

    /// Secondary characteristics (if purchased)
    #[cfg_attr(not(creusot), builder(default))]
    pub secondary_characteristics: Vec<SecondaryCharacteristicDescriptor>,

    /// Advantages
    #[cfg_attr(not(creusot), builder(default))]
    pub advantages: Vec<AdvantageDescriptor>,

    /// Disadvantages
    #[cfg_attr(not(creusot), builder(default))]
    pub disadvantages: Vec<DisadvantageDescriptor>,

    /// Skills
    #[cfg_attr(not(creusot), builder(default))]
    pub skills: Vec<SkillDescriptor>,

    /// Derived statistics
    pub derived_stats: DerivedStatsDescriptor,
}

/// Describes derived statistics.
#[derive(Debug, Clone, Copy)]
#[cfg_attr(not(creusot), derive(PartialEq))]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub struct DerivedStatsDescriptor {
    /// Basic Speed
    pub basic_speed: f32,

    /// Basic Move
    pub basic_move: i32,

    /// Dodge score
    pub dodge: i32,

    /// Hit Points
    pub hp: i32,

    /// Will
    pub will: i32,

    /// Perception
    pub perception: i32,

    /// Fatigue Points
    pub fp: i32,
}

// ── Spell Descriptors ─────────────────────────────────────────────────────────

/// Describes a spell for learning.
#[derive(Debug, Clone)]
#[cfg_attr(not(creusot), derive(PartialEq, Eq))]
#[cfg_attr(not(creusot), derive(Builder))]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
#[cfg_attr(not(creusot), builder(setter(into)))]
pub struct SpellDescriptor {
    /// Spell name
    pub name: String,

    /// Spell college (Fire, Air, Enchantment, etc.)
    pub college: SpellCollege,

    /// Difficulty level (Hard for all spells)
    #[cfg_attr(not(creusot), builder(default = "SkillDifficulty::Hard"))]
    pub difficulty: SkillDifficulty,

    /// Current skill level with this spell
    pub skill_level: i32,

    /// Character points invested in this spell
    pub points_invested: i32,

    /// Prerequisite spell names
    #[cfg_attr(not(creusot), builder(default))]
    pub prerequisites: Vec<String>,

    /// Minimum Magery level required (0-3)
    #[cfg_attr(not(creusot), builder(default))]
    pub magery_required: i32,

    /// Base energy cost to cast
    pub base_casting_cost: i32,

    /// Base energy cost to maintain (0 if not maintained)
    #[cfg_attr(not(creusot), builder(default))]
    pub base_maintenance_cost: i32,

    /// Standard casting time in seconds
    pub base_casting_time: i32,

    /// Spell class (Area, Missile, Regular, etc.)
    pub spell_class: SpellClass,

    /// Whether spell can be resisted
    #[cfg_attr(not(creusot), builder(default))]
    pub resistible: bool,

    /// Resistance attribute if resistible (Will, HT, etc.)
    #[cfg_attr(not(creusot), builder(default))]
    pub resistance_attribute: Option<String>,
}

/// Spell colleges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub enum SpellCollege {
    /// Air spells
    Air,
    /// Animal spells
    Animal,
    /// Body Control spells
    BodyControl,
    /// Communication and Empathy spells
    CommunicationEmpathy,
    /// Earth spells
    Earth,
    /// Enchantment spells
    Enchantment,
    /// Fire spells
    Fire,
    /// Food spells
    Food,
    /// Gate spells (teleportation)
    Gate,
    /// Healing spells
    Healing,
    /// Illusion and Creation spells
    IllusionCreation,
    /// Knowledge spells
    Knowledge,
    /// Light and Darkness spells
    LightDarkness,
    /// Making and Breaking spells
    MakingBreaking,
    /// Meta-Spells (affect other spells)
    MetaSpells,
    /// Mind Control spells
    MindControl,
    /// Movement spells
    Movement,
    /// Necromantic spells
    Necromantic,
    /// Plant spells
    Plant,
    /// Protection and Warning spells
    ProtectionWarning,
    /// Sound spells
    Sound,
    /// Technological spells
    Technological,
    /// Water spells
    Water,
    /// Weather spells
    Weather,
}

/// Spell classes (casting types).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub enum SpellClass {
    /// Regular spell affecting single target
    Regular,
    /// Area spell affecting region
    Area,
    /// Missile spell (ranged projectile)
    Missile,
    /// Melee spell (touch or close range)
    Melee,
    /// Blocking spell (intercepts other spells)
    Blocking,
    /// Information spell (divination, sensing)
    Information,
    /// Special (unique mechanics)
    Special,
}

/// Describes a spell casting attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(creusot), derive(Builder))]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
#[cfg_attr(not(creusot), builder(setter(into)))]
pub struct SpellCastingDescriptor {
    /// Spell being cast
    pub spell_name: String,

    /// Caster's effective skill with spell
    pub effective_skill: i32,

    /// Energy cost for this casting
    pub energy_cost: i32,

    /// Casting time in seconds
    pub casting_time: i32,

    /// Target identifier (if applicable)
    #[cfg_attr(not(creusot), builder(default))]
    pub target: Option<String>,

    /// Distance to target in yards
    #[cfg_attr(not(creusot), builder(default))]
    pub range_yards: i32,

    /// Size/speed modifier
    #[cfg_attr(not(creusot), builder(default))]
    pub size_speed_modifier: i32,

    /// Time modifier (extra time or rushing)
    #[cfg_attr(not(creusot), builder(default))]
    pub time_modifier: i32,

    /// Environmental modifier
    #[cfg_attr(not(creusot), builder(default))]
    pub environment_modifier: i32,

    /// Whether taking extra energy for greater effect
    #[cfg_attr(not(creusot), builder(default))]
    pub extra_energy: i32,

    /// Whether spell is being maintained (not initial cast)
    #[cfg_attr(not(creusot), builder(default))]
    pub is_maintenance: bool,
}

/// Describes the result of a spell casting attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub struct SpellCastingResult {
    /// The 3d6 roll result
    pub roll: i32,

    /// The effective skill that was rolled against
    pub effective_skill: i32,

    /// Whether the spell casting succeeded
    pub success: bool,

    /// Margin of success (if success) or failure (if failure)
    pub margin: i32,

    /// Whether this was a critical success
    pub critical_success: bool,

    /// Whether this was a critical failure
    pub critical_failure: bool,

    /// Energy actually spent (may differ from cost on failure)
    pub energy_spent: i32,
}

/// Describes caster's state for spell casting.
#[derive(Debug, Clone)]
#[cfg_attr(not(creusot), derive(PartialEq, Eq))]
#[cfg_attr(not(creusot), derive(Builder))]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
#[cfg_attr(not(creusot), builder(setter(into)))]
pub struct CasterDescriptor {
    /// Caster's current Fatigue Points
    pub current_fp: i32,

    /// Caster's maximum Fatigue Points
    pub max_fp: i32,

    /// Caster's current Hit Points
    pub current_hp: i32,

    /// Caster's maximum Hit Points
    pub max_hp: i32,

    /// Caster's Magery level (0-3)
    pub magery_level: i32,

    /// Caster's IQ attribute
    pub iq: i32,

    /// Spells known by caster
    #[cfg_attr(not(creusot), builder(default))]
    pub known_spells: Vec<SpellDescriptor>,

    /// Whether caster is concentrating on a spell
    #[cfg_attr(not(creusot), builder(default))]
    pub concentrating: bool,

    /// Penalties from wounds, stunning, etc.
    #[cfg_attr(not(creusot), builder(default))]
    pub penalties: i32,
}

/// Describes spell effect details.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub struct SpellEffectDescriptor {
    /// Spell name
    pub spell_name: String,

    /// Effect description
    pub effect_description: String,

    /// Duration in seconds (0 for instant)
    pub duration_seconds: i32,

    /// Whether spell is currently maintained
    pub is_maintained: bool,

    /// Target identifier(s)
    pub targets: Vec<String>,

    /// Numerical effect value (damage, healing, etc.)
    pub effect_value: Option<i32>,

    /// Area radius in yards (0 for single target)
    pub area_radius_yards: i32,
}

/// Describes spell resistance contest.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub struct SpellResistanceDescriptor {
    /// Caster's effective spell skill
    pub caster_skill: i32,

    /// Target's resistance attribute value
    pub target_resistance: i32,

    /// Type of resistance (Will, HT, etc.)
    pub resistance_type: String,
}

/// Describes result of resistance contest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
pub struct ResistanceResult {
    /// Caster's roll
    pub caster_roll: i32,

    /// Target's roll
    pub target_roll: i32,

    /// Whether target successfully resisted
    pub resisted: bool,

    /// Margin of victory (positive if caster won, negative if target won)
    pub margin: i32,
}

/// Describes ceremonial magic attempt.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(creusot), derive(Builder))]
#[cfg_attr(not(creusot), derive(Serialize, Deserialize, JsonSchema))]
#[cfg_attr(not(creusot), builder(setter(into)))]
pub struct CeremonialMagicDescriptor {
    /// Lead caster
    pub leader: String,

    /// Participating casters
    pub participants: Vec<String>,

    /// Spell being cast
    pub spell_name: String,

    /// Total energy pooled from all participants
    pub pooled_energy: i32,

    /// Leader's effective skill
    pub leader_skill: i32,

    /// Skill bonuses from assistants
    #[cfg_attr(not(creusot), builder(default))]
    pub assistant_bonuses: i32,

    /// Ceremony duration in seconds
    pub ceremony_time: i32,
}

// ── Metadata Trait Implementations ───────────────────────────────────────────

impl crate::contracts::traits::AttackMeta for AttackDescriptor {
    fn effective_skill(&self) -> i32 {
        self.effective_skill
    }

    fn is_all_out(&self) -> bool {
        self.all_out_attack
    }

    fn deceptive_penalty(&self) -> i32 {
        self.deceptive_penalty
    }
}

impl crate::contracts::traits::DefenseMeta for DefenseDescriptor {
    fn defense_score(&self) -> i32 {
        self.defense_score
    }

    fn defense_type(&self) -> DefenseType {
        self.defense_type
    }

    fn is_retreating(&self) -> bool {
        self.retreating
    }
}

impl crate::contracts::traits::DamageMeta for DamageResult {
    fn raw_damage(&self) -> i32 {
        self.raw_damage
    }

    fn damage_resistance(&self) -> i32 {
        self.dr
    }

    fn hit_location(&self) -> HitLocation {
        self.location
    }

    fn injury(&self) -> i32 {
        self.injury
    }
}
