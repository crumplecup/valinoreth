//! Trait interfaces for GURPS combat operations.
//!
//! These traits define the contract-based API for executing game rules.
//! All methods return `Established<P>` proofs alongside their results,
//! enabling compile-time verification that GURPS rules were followed.
//!
//! # Pattern
//!
//! Traits follow the elicitation pattern:
//! - Object-safe (usable as `dyn Trait`)
//! - Return `Established<P>` proofs, not associated types
//! - Take descriptor types as parameters
//! - Use async for future MCP integration
//!
//! # Example
//!
//! ```rust,ignore
//! async fn resolve_combat<R: AttackResolver>(resolver: &R) {
//!     let descriptor = AttackDescriptorBuilder::default()
//!         .effective_skill(14)
//!         .build()
//!         .unwrap();
//!
//!     let (result, evidence) = resolver.resolve_attack(descriptor).await?;
//!
//!     // Evidence is Established<AttackResolutionEvidence>
//!     // Compiler enforces that attack was properly resolved
//! }
//! ```

use crate::contracts::proof_composition::{
    AdvantagePurchaseEvidence, AttributePurchaseEvidence, AttackCriticalFailureEvidence,
    AttackCriticalSuccessEvidence, AttackFailureEvidence, AttackResolutionEvidence,
    AttackSuccessEvidence, BasicDamageEvidence, CharacterCreationEvidence,
    CharacterValidationEvidence, CombatHitEvidence, DefenseCriticalFailureEvidence,
    DefenseCriticalSuccessEvidence, DefenseFailureEvidence, DefenseResolutionEvidence,
    DefenseSuccessEvidence, DerivedStatsEvidence, DisadvantageTakenEvidence, FeintEvidence,
    InjuryApplicationEvidence, InjuryCalculationEvidence, RapidStrikeEvidence,
    SecondaryCharacteristicEvidence, SkillCheckCriticalFailureEvidence,
    SkillCheckCriticalSuccessEvidence, SkillCheckFailureEvidence, SkillCheckResolutionEvidence,
    SkillCheckSuccessEvidence, SkillImprovementEvidence,
};
use crate::contracts::types::{
    AdvantageDescriptor, ArmorDescriptor, AttributeDescriptor, AttackDescriptor, AttackRollResult,
    CharacterCreationDescriptor, CharacterDescriptor, CombatantDescriptor, DamageDescriptor,
    DamageResult, DefenseDescriptor, DefenseRollResult, DerivedStatsDescriptor,
    DisadvantageDescriptor, FeintDescriptor, FeintResult, HitLocation, RapidStrikeDescriptor,
    SecondaryCharacteristicDescriptor, SkillCheckDescriptor, SkillCheckResult, SkillDescriptor,
};
use async_trait::async_trait;
use elicitation::contracts::Established;

/// Error type for combat operations.
pub type CombatResult<T> = Result<T, ContractError>;

/// Error kind for combat operations.
#[derive(Debug, Clone, PartialEq, Eq, derive_more::Display)]
pub enum ContractErrorKind {
    /// Invalid skill value
    #[display("Invalid skill value: {}", _0)]
    InvalidSkill(String),

    /// Invalid roll result
    #[display("Invalid roll: {}", _0)]
    InvalidRoll(String),

    /// Invalid damage value
    #[display("Invalid damage: {}", _0)]
    InvalidDamage(String),

    /// State violation (e.g., defense when already used)
    #[display("State violation: {}", _0)]
    StateViolation(String),
}

/// Errors that can occur during combat resolution.
#[derive(Debug, Clone, derive_more::Display, derive_more::Error)]
#[display("Contract error: {} at {}:{}", kind, file, line)]
pub struct ContractError {
    /// The specific error kind
    pub kind: ContractErrorKind,
    /// Source file where error occurred
    pub file: &'static str,
    /// Line number where error occurred
    pub line: u32,
}

impl ContractError {
    /// Create a new contract error with location tracking.
    #[track_caller]
    pub fn new(kind: ContractErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind,
            file: loc.file(),
            line: loc.line(),
        }
    }
}

// ── Attack Resolution ─────────────────────────────────────────────────────────

/// Resolves attack rolls and determines outcomes.
///
/// Provides methods to execute GURPS attack resolution, returning
/// proofs that the attack was properly rolled and evaluated.
#[async_trait]
pub trait AttackResolver {
    /// Resolve an attack roll.
    ///
    /// Rolls 3d6 against effective skill and determines success/failure.
    /// Returns the roll result and proof that the attack was resolved.
    ///
    /// # GURPS Rules
    ///
    /// Roll 3d6 ≤ effective skill for success.
    /// Critical success on 3-4, or 5-6 if skill ≥ 15.
    /// Critical failure on 17-18, or if margin ≥ 10.
    ///
    /// # Citations
    ///
    /// BS 356 - Attack rolls
    async fn resolve_attack(
        &self,
        descriptor: AttackDescriptor,
    ) -> CombatResult<(AttackRollResult, Established<AttackResolutionEvidence>)>;

    /// Resolve a successful attack.
    ///
    /// Takes an already-resolved attack and returns proof that it succeeded.
    /// Used when the roll is known to have succeeded.
    async fn confirm_attack_success(
        &self,
        result: AttackRollResult,
        base_evidence: Established<AttackResolutionEvidence>,
    ) -> CombatResult<Established<AttackSuccessEvidence>>;

    /// Resolve a failed attack.
    ///
    /// Takes an already-resolved attack and returns proof that it failed.
    async fn confirm_attack_failure(
        &self,
        result: AttackRollResult,
        base_evidence: Established<AttackResolutionEvidence>,
    ) -> CombatResult<Established<AttackFailureEvidence>>;

    /// Resolve a critical success.
    ///
    /// Takes a successful attack and returns proof of critical success.
    async fn confirm_critical_success(
        &self,
        result: AttackRollResult,
        success_evidence: Established<AttackSuccessEvidence>,
    ) -> CombatResult<Established<AttackCriticalSuccessEvidence>>;

    /// Resolve a critical failure.
    ///
    /// Takes an attack and returns proof of critical failure.
    async fn confirm_critical_failure(
        &self,
        result: AttackRollResult,
        base_evidence: Established<AttackResolutionEvidence>,
    ) -> CombatResult<Established<AttackCriticalFailureEvidence>>;
}

// ── Defense Resolution ────────────────────────────────────────────────────────

/// Resolves active defense rolls.
///
/// Provides methods to execute GURPS defense resolution (Dodge/Parry/Block),
/// returning proofs that defenses were properly rolled and evaluated.
#[async_trait]
pub trait DefenseResolver {
    /// Resolve an active defense roll.
    ///
    /// Rolls 3d6 against defense score and determines success/failure.
    /// Returns the roll result and proof that defense was resolved.
    ///
    /// # GURPS Rules
    ///
    /// Roll 3d6 ≤ defense score to avoid attack.
    /// Critical success on 3-4, or 5-6 if defense ≥ 15.
    /// Critical failure on 17-18, or if margin ≥ 10.
    ///
    /// # Citations
    ///
    /// BS 374 - Active defenses
    async fn resolve_defense(
        &self,
        descriptor: DefenseDescriptor,
    ) -> CombatResult<(DefenseRollResult, Established<DefenseResolutionEvidence>)>;

    /// Confirm defense success.
    async fn confirm_defense_success(
        &self,
        result: DefenseRollResult,
        base_evidence: Established<DefenseResolutionEvidence>,
    ) -> CombatResult<Established<DefenseSuccessEvidence>>;

    /// Confirm defense failure.
    async fn confirm_defense_failure(
        &self,
        result: DefenseRollResult,
        base_evidence: Established<DefenseResolutionEvidence>,
    ) -> CombatResult<Established<DefenseFailureEvidence>>;

    /// Confirm critical defense success.
    async fn confirm_defense_critical_success(
        &self,
        result: DefenseRollResult,
        success_evidence: Established<DefenseSuccessEvidence>,
    ) -> CombatResult<Established<DefenseCriticalSuccessEvidence>>;

    /// Confirm critical defense failure.
    async fn confirm_defense_critical_failure(
        &self,
        result: DefenseRollResult,
        base_evidence: Established<DefenseResolutionEvidence>,
    ) -> CombatResult<Established<DefenseCriticalFailureEvidence>>;
}

// ── Damage Calculation ────────────────────────────────────────────────────────

/// Calculates damage and injury.
///
/// Provides methods to roll weapon damage, apply DR, calculate hit location
/// effects, and determine final injury to HP.
#[async_trait]
pub trait DamageCalculator {
    /// Roll weapon damage.
    ///
    /// Rolls damage dice for the weapon and returns the result with
    /// proof that damage was rolled.
    ///
    /// # GURPS Rules
    ///
    /// Roll damage dice as specified by weapon (e.g., 1d6+2).
    ///
    /// # Citations
    ///
    /// BS 269 - Damage rolls
    async fn roll_damage(
        &self,
        descriptor: DamageDescriptor,
    ) -> CombatResult<(i32, Established<BasicDamageEvidence>)>;

    /// Calculate injury from damage.
    ///
    /// Applies DR, hit location multiplier, and wounding modifier
    /// to determine final injury.
    ///
    /// # GURPS Rules
    ///
    /// 1. Subtract DR from raw damage
    /// 2. Apply hit location multiplier
    /// 3. Apply wounding modifier for damage type
    /// 4. Result is injury to HP
    ///
    /// # Citations
    ///
    /// BS 378-380 - Damage calculation
    async fn calculate_injury(
        &self,
        raw_damage: i32,
        armor: ArmorDescriptor,
        location: HitLocation,
        damage_descriptor: DamageDescriptor,
        damage_evidence: Established<BasicDamageEvidence>,
    ) -> CombatResult<(DamageResult, Established<InjuryCalculationEvidence>)>;

    /// Apply injury to character HP.
    ///
    /// Subtracts injury from current HP and returns updated combatant state
    /// with proof that injury was applied.
    ///
    /// # GURPS Rules
    ///
    /// HP reduces by injury amount. Check for consciousness, death rolls, etc.
    ///
    /// # Citations
    ///
    /// BS 419-420 - Hit Points and injury
    async fn apply_injury(
        &self,
        combatant: CombatantDescriptor,
        injury: i32,
        injury_evidence: Established<InjuryCalculationEvidence>,
    ) -> CombatResult<(CombatantDescriptor, Established<InjuryApplicationEvidence>)>;
}

// ── Complete Combat Resolution ────────────────────────────────────────────────

/// Resolves complete combat exchanges.
///
/// High-level interface that combines attack, defense, and damage resolution
/// into complete combat turns.
#[async_trait]
pub trait CombatExecutor {
    /// Execute a complete combat exchange.
    ///
    /// Resolves attack, defense, and damage (if applicable) in one call.
    /// Returns either a hit (with injury) or miss (defense succeeded or attack failed).
    ///
    /// # GURPS Rules
    ///
    /// Complete combat flow:
    /// 1. Attacker rolls to hit
    /// 2. If hit, defender rolls to defend
    /// 3. If defense fails, calculate and apply damage
    ///
    /// # Citations
    ///
    /// BS 356-380 - Complete combat rules
    async fn execute_attack(
        &self,
        attacker: CombatantDescriptor,
        defender: CombatantDescriptor,
        attack_descriptor: AttackDescriptor,
        defense_descriptor: DefenseDescriptor,
        damage_descriptor: DamageDescriptor,
        armor: ArmorDescriptor,
    ) -> CombatResult<CombatExchangeResult>;
}

/// Result of a complete combat exchange.
#[derive(Debug, Clone)]
pub enum CombatExchangeResult {
    /// Attack hit and dealt damage
    Hit {
        /// Updated defender state after injury
        defender: CombatantDescriptor,
        /// Damage dealt
        damage: DamageResult,
        /// Proof that combat hit was resolved correctly
        evidence: Established<CombatHitEvidence>,
    },

    /// Attack missed or was defended
    Miss {
        /// Attack roll result
        attack_result: AttackRollResult,
        /// Defense roll result (if attack succeeded)
        defense_result: Option<DefenseRollResult>,
        /// Reason for miss
        reason: MissReason,
    },
}

/// Reason an attack missed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissReason {
    /// Attack roll failed
    AttackFailed,
    /// Defense roll succeeded
    DefenseSucceeded,
    /// Critical failure on attack
    CriticalFailure,
}

// ── Special Maneuvers ─────────────────────────────────────────────────────────

/// Resolves special combat maneuvers.
///
/// Provides methods for Feint, Rapid Strike, All-Out Attack, etc.
#[async_trait]
pub trait ManeuverExecutor {
    /// Execute a Feint maneuver.
    ///
    /// Resolves Quick Contest between attacker and defender skills.
    /// If attacker wins, defender's next defense is penalized by margin.
    ///
    /// # GURPS Rules
    ///
    /// Both sides roll against their skill. Compare margins of success/failure.
    /// Winner's margin becomes penalty to loser's next action.
    ///
    /// # Citations
    ///
    /// BS 365 - Feint
    async fn execute_feint(
        &self,
        descriptor: FeintDescriptor,
    ) -> CombatResult<(FeintResult, Established<FeintEvidence>)>;

    /// Execute a Rapid Strike.
    ///
    /// Makes multiple attacks at penalty (-6 each, or -3 with Weapon Master).
    ///
    /// # GURPS Rules
    ///
    /// Each attack at -6 to skill (or -3 with training).
    /// May attack different targets.
    ///
    /// # Citations
    ///
    /// BS 370 - Rapid Strike
    async fn execute_rapid_strike(
        &self,
        descriptor: RapidStrikeDescriptor,
    ) -> CombatResult<(Vec<AttackRollResult>, Established<RapidStrikeEvidence>)>;
}

// ── Meta Information ──────────────────────────────────────────────────────────

/// Metadata about an attack roll.
pub trait AttackMeta {
    /// Returns the effective skill used for this attack.
    fn effective_skill(&self) -> i32;

    /// Returns whether this attack uses All-Out Attack.
    fn is_all_out(&self) -> bool;

    /// Returns any deceptive attack penalty applied.
    fn deceptive_penalty(&self) -> i32;
}

/// Metadata about a defense roll.
pub trait DefenseMeta {
    /// Returns the defense score used.
    fn defense_score(&self) -> i32;

    /// Returns the type of defense (Dodge/Parry/Block).
    fn defense_type(&self) -> crate::contracts::types::DefenseType;

    /// Returns whether defender was retreating.
    fn is_retreating(&self) -> bool;
}

/// Metadata about damage calculation.
pub trait DamageMeta {
    /// Returns the raw damage rolled.
    fn raw_damage(&self) -> i32;

    /// Returns the DR that was applied.
    fn damage_resistance(&self) -> i32;

    /// Returns the hit location.
    fn hit_location(&self) -> HitLocation;

    /// Returns the final injury to HP.
    fn injury(&self) -> i32;
}

// ── Skill Check Resolution ────────────────────────────────────────────────────

/// Resolves skill checks and determines outcomes.
///
/// Provides methods to execute GURPS skill checks, returning
/// proofs that the skill check was properly rolled and evaluated.
#[async_trait]
pub trait SkillCheckExecutor {
    /// Resolve a skill check roll.
    ///
    /// Rolls 3d6 against effective skill and determines success/failure.
    /// Returns the roll result and proof that the skill check was resolved.
    ///
    /// # GURPS Rules
    ///
    /// Roll 3d6 ≤ effective skill for success.
    /// Critical success on 3-4, or 5-6 if skill ≥ 15.
    /// Critical failure on 17-18, or if margin ≥ 10.
    ///
    /// # Citations
    ///
    /// BS 171 - Skill checks
    /// BS 344 - Success rolls
    async fn resolve_skill_check(
        &self,
        descriptor: SkillCheckDescriptor,
    ) -> CombatResult<(SkillCheckResult, Established<SkillCheckResolutionEvidence>)>;

    /// Confirm skill check success.
    ///
    /// Takes an already-resolved skill check and returns proof that it succeeded.
    async fn confirm_skill_check_success(
        &self,
        result: SkillCheckResult,
        base_evidence: Established<SkillCheckResolutionEvidence>,
    ) -> CombatResult<Established<SkillCheckSuccessEvidence>>;

    /// Confirm skill check failure.
    ///
    /// Takes an already-resolved skill check and returns proof that it failed.
    async fn confirm_skill_check_failure(
        &self,
        result: SkillCheckResult,
        base_evidence: Established<SkillCheckResolutionEvidence>,
    ) -> CombatResult<Established<SkillCheckFailureEvidence>>;

    /// Confirm critical success on skill check.
    ///
    /// Takes a successful skill check and returns proof of critical success.
    async fn confirm_critical_skill_success(
        &self,
        result: SkillCheckResult,
        success_evidence: Established<SkillCheckSuccessEvidence>,
    ) -> CombatResult<Established<SkillCheckCriticalSuccessEvidence>>;

    /// Confirm critical failure on skill check.
    ///
    /// Takes a skill check and returns proof of critical failure.
    async fn confirm_critical_skill_failure(
        &self,
        result: SkillCheckResult,
        base_evidence: Established<SkillCheckResolutionEvidence>,
    ) -> CombatResult<Established<SkillCheckCriticalFailureEvidence>>;
}

// ── Skill Management ──────────────────────────────────────────────────────────

/// Manages character skills and improvement.
///
/// Provides methods to add skills, improve skills, and calculate skill levels.
#[async_trait]
pub trait SkillManager {
    /// Add a new skill to a character.
    ///
    /// Creates a new skill entry with the specified base level and point cost.
    ///
    /// # GURPS Rules
    ///
    /// Skills start at attribute level - penalty (based on difficulty).
    /// Spending 1-4 points raises relative skill level.
    ///
    /// # Citations
    ///
    /// BS 170 - Skill costs
    async fn add_skill(
        &self,
        character: CharacterDescriptor,
        skill: SkillDescriptor,
    ) -> CombatResult<CharacterDescriptor>;

    /// Improve an existing skill.
    ///
    /// Spends character points to increase skill level.
    /// Returns updated character and proof that improvement was valid.
    ///
    /// # GURPS Rules
    ///
    /// Cost increases exponentially: 1/2/4/8/16 points per level.
    ///
    /// # Citations
    ///
    /// BS 170 - Improving skills
    async fn improve_skill(
        &self,
        character: CharacterDescriptor,
        skill_name: String,
        points_to_spend: i32,
    ) -> CombatResult<(CharacterDescriptor, Established<SkillImprovementEvidence>)>;

    /// Calculate effective skill level.
    ///
    /// Computes effective skill including modifiers, defaults, and bonuses.
    ///
    /// # GURPS Rules
    ///
    /// Effective skill = base skill + modifiers - penalties
    ///
    /// # Citations
    ///
    /// BS 171 - Effective skill
    async fn calculate_effective_skill(
        &self,
        character: &CharacterDescriptor,
        skill_name: String,
        modifiers: i32,
    ) -> CombatResult<i32>;
}

// ── Character Creation ────────────────────────────────────────────────────────

/// Creates and validates GURPS characters.
///
/// Provides methods to build characters step-by-step, ensuring all
/// GURPS rules and campaign restrictions are followed.
#[async_trait]
pub trait CharacterBuilder {
    /// Create a new character with basic parameters.
    ///
    /// Initializes character with name, point total, and default attributes.
    ///
    /// # GURPS Rules
    ///
    /// All characters start with 10 in each attribute (ST, DX, IQ, HT).
    ///
    /// # Citations
    ///
    /// BS 10 - Character creation
    async fn create_character(
        &self,
        descriptor: CharacterCreationDescriptor,
    ) -> CombatResult<CharacterDescriptor>;

    /// Purchase a primary attribute.
    ///
    /// Sets attribute level and calculates point cost.
    /// Returns updated character and proof of valid purchase.
    ///
    /// # GURPS Rules
    ///
    /// ST/HT cost 10 points per level, DX/IQ cost 20 points per level.
    ///
    /// # Citations
    ///
    /// BS 14-16 - Attributes
    async fn purchase_attribute(
        &self,
        character: CharacterDescriptor,
        attribute: AttributeDescriptor,
    ) -> CombatResult<(CharacterDescriptor, Established<AttributePurchaseEvidence>)>;

    /// Purchase a secondary characteristic.
    ///
    /// Buys extra HP, Will, Per, FP, Basic Speed, or Basic Move.
    /// Returns updated character and proof of valid purchase.
    ///
    /// # GURPS Rules
    ///
    /// Secondary characteristics have varying costs (2-20 points per level).
    ///
    /// # Citations
    ///
    /// BS 16-17 - Secondary characteristics
    async fn purchase_secondary_characteristic(
        &self,
        character: CharacterDescriptor,
        characteristic: SecondaryCharacteristicDescriptor,
    ) -> CombatResult<(CharacterDescriptor, Established<SecondaryCharacteristicEvidence>)>;

    /// Add an advantage.
    ///
    /// Purchases advantage with modifiers and level (if applicable).
    /// Returns updated character and proof of valid purchase.
    ///
    /// # GURPS Rules
    ///
    /// Advantages cost points based on base cost and modifiers.
    ///
    /// # Citations
    ///
    /// BS 100-132 - Advantages
    async fn add_advantage(
        &self,
        character: CharacterDescriptor,
        advantage: AdvantageDescriptor,
    ) -> CombatResult<(CharacterDescriptor, Established<AdvantagePurchaseEvidence>)>;

    /// Add a disadvantage.
    ///
    /// Takes disadvantage for bonus points.
    /// Returns updated character and proof of valid disadvantage.
    ///
    /// # GURPS Rules
    ///
    /// Disadvantages provide points up to campaign limit (typically -50).
    ///
    /// # Citations
    ///
    /// BS 133-166 - Disadvantages
    async fn add_disadvantage(
        &self,
        character: CharacterDescriptor,
        disadvantage: DisadvantageDescriptor,
    ) -> CombatResult<(CharacterDescriptor, Established<DisadvantageTakenEvidence>)>;

    /// Calculate all derived statistics.
    ///
    /// Computes Basic Speed, Basic Move, Dodge, HP, Will, Per, FP.
    /// Returns updated character and proof of correct calculation.
    ///
    /// # GURPS Rules
    ///
    /// Derived stats follow formulas based on primary attributes.
    ///
    /// # Citations
    ///
    /// BS 16-17 - Derived statistics
    async fn calculate_derived_stats(
        &self,
        character: CharacterDescriptor,
    ) -> CombatResult<(DerivedStatsDescriptor, Established<DerivedStatsEvidence>)>;

    /// Finalize character creation.
    ///
    /// Validates complete character and returns proof of validity.
    /// Ensures point budget is balanced and all requirements are met.
    ///
    /// # GURPS Rules
    ///
    /// Characters must be complete, valid, and within point budget.
    ///
    /// # Citations
    ///
    /// BS 10-15 - Character validation
    async fn finalize_character(
        &self,
        character: CharacterDescriptor,
    ) -> CombatResult<(CharacterDescriptor, Established<CharacterValidationEvidence>)>;
}

// ── Character Advancement ─────────────────────────────────────────────────────

/// Manages character advancement and experience.
///
/// Provides methods to award experience points and improve characters.
#[async_trait]
pub trait CharacterAdvancement {
    /// Award experience points to character.
    ///
    /// Adds unspent character points for future improvements.
    ///
    /// # GURPS Rules
    ///
    /// GM awards points based on game progress.
    ///
    /// # Citations
    ///
    /// BS 498 - Character advancement
    async fn award_experience(
        &self,
        character: CharacterDescriptor,
        points: i32,
    ) -> CombatResult<CharacterDescriptor>;

    /// Spend experience points on improvements.
    ///
    /// Applies points to attributes, skills, or advantages.
    /// Returns updated character with proof of valid spending.
    ///
    /// # GURPS Rules
    ///
    /// Improvements follow same costs as character creation.
    ///
    /// # Citations
    ///
    /// BS 498 - Spending points
    async fn spend_experience(
        &self,
        character: CharacterDescriptor,
        improvement: CharacterImprovement,
    ) -> CombatResult<(CharacterDescriptor, Established<CharacterCreationEvidence>)>;
}

/// Types of character improvements.
#[derive(Debug, Clone, PartialEq)]
pub enum CharacterImprovement {
    /// Raise an attribute
    Attribute {
        /// Attribute to improve
        attribute_type: crate::contracts::types::AttributeType,
        /// Levels to increase
        levels: i32,
    },
    /// Improve a skill
    Skill {
        /// Skill name
        name: String,
        /// Points to spend
        points: i32,
    },
    /// Purchase new advantage
    Advantage {
        /// Advantage descriptor
        advantage: AdvantageDescriptor,
    },
    /// Buy off disadvantage
    BuyOffDisadvantage {
        /// Disadvantage name
        name: String,
    },
}
