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

use derive_builder::Builder;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── Attack Descriptors ────────────────────────────────────────────────────────

/// Describes an attack attempt.
///
/// Contains all information needed to resolve an attack roll:
/// skill level, modifiers, and optional target location.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(setter(into))]
pub struct AttackDescriptor {
    /// Attacker's effective skill level (base skill + modifiers)
    pub effective_skill: i32,

    /// Whether this is an All-Out Attack
    #[builder(default)]
    pub all_out_attack: bool,

    /// Deceptive Attack skill penalty (reduces opponent defense by half this)
    #[builder(default)]
    pub deceptive_penalty: i32,

    /// Aim bonus accumulated (ranged attacks only)
    #[builder(default)]
    pub aim_bonus: i32,

    /// Target hit location (if targeted attack)
    #[builder(default)]
    pub target_location: Option<HitLocation>,
}

/// Describes the result of an attack roll.
///
/// Contains the roll result and outcome determination.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(setter(into))]
pub struct DefenseDescriptor {
    /// Type of active defense (Dodge, Parry, Block)
    pub defense_type: DefenseType,

    /// Defender's defense score
    pub defense_score: i32,

    /// Penalty from Feint (if opponent feinted successfully)
    #[builder(default)]
    pub feint_penalty: i32,

    /// Penalty from Deceptive Attack
    #[builder(default)]
    pub deceptive_penalty: i32,

    /// Whether defender is retreating (+3 to defense)
    #[builder(default)]
    pub retreating: bool,
}

/// Type of active defense.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub enum DefenseType {
    /// Dodge: 3d6 ≤ Dodge score (DX + 3 + bonuses)
    Dodge,
    /// Parry: 3d6 ≤ Parry score (skill/2 + 3 + bonuses)
    Parry,
    /// Block: 3d6 ≤ Block score (skill/2 + 3 + bonuses)
    Block,
}

/// Describes the result of a defense roll.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(setter(into))]
pub struct DamageDescriptor {
    /// Number of dice to roll
    pub dice: i32,

    /// Number of sides per die (usually 6)
    pub sides: i32,

    /// Flat modifier to add to roll
    #[builder(default)]
    pub modifier: i32,

    /// Type of damage (affects wounding multiplier)
    pub damage_type: DamageTypeDescriptor,

    /// Bonus damage from critical hit
    #[builder(default)]
    pub critical_bonus: i32,
}

/// Type of damage for wounding modifier calculation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ArmorDescriptor {
    /// Damage Resistance value
    pub dr: i32,

    /// Whether armor is flexible (affects crushing damage)
    pub flexible: bool,
}

/// Hit location on the body.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
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
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct FeintDescriptor {
    /// Attacker's skill for the Quick Contest
    pub attacker_skill: i32,

    /// Defender's skill for the Quick Contest
    pub defender_skill: i32,
}

/// Describes the result of a Feint contest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Builder)]
#[builder(setter(into))]
pub struct CombatantDescriptor {
    /// Character's current Hit Points
    pub current_hp: i32,

    /// Character's maximum Hit Points
    pub max_hp: i32,

    /// Character's Dodge score
    pub dodge: i32,

    /// Character's Parry score (if applicable)
    #[builder(default)]
    pub parry: Option<i32>,

    /// Character's Block score (if applicable)
    #[builder(default)]
    pub block: Option<i32>,

    /// Whether character has acted this turn
    #[builder(default)]
    pub has_acted: bool,

    /// Whether character used All-Out Attack (no defenses until next turn)
    #[builder(default)]
    pub all_out_attack_used: bool,

    /// Accumulated penalties from wounds, stunning, etc.
    #[builder(default)]
    pub shock_penalty: i32,
}
