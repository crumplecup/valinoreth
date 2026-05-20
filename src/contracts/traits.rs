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
    AttackCriticalFailureEvidence, AttackCriticalSuccessEvidence, AttackFailureEvidence,
    AttackResolutionEvidence, AttackSuccessEvidence, BasicDamageEvidence, CombatHitEvidence,
    DefenseCriticalFailureEvidence, DefenseCriticalSuccessEvidence, DefenseFailureEvidence,
    DefenseResolutionEvidence, DefenseSuccessEvidence, FeintEvidence,
    InjuryApplicationEvidence, InjuryCalculationEvidence, RapidStrikeEvidence,
};
use crate::contracts::types::{
    ArmorDescriptor, AttackDescriptor, AttackRollResult, CombatantDescriptor, DamageDescriptor,
    DamageResult, DefenseDescriptor, DefenseRollResult, FeintDescriptor, FeintResult, HitLocation,
    RapidStrikeDescriptor,
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
