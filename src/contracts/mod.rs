//! Contract system for GURPS game rules.
//!
//! This module provides a type-based contract system for representing GURPS 4th Edition
//! game rules as compile-time propositions. Each game rule becomes a [`Prop`] that can
//! be proven via [`Established<P>`] tokens, enabling compile-time verification of game
//! rule adherence.
//!
//! # Design
//!
//! Following the elicitation framework pattern:
//!
//! 1. **Atomic Propositions**: Each unique GURPS rule is a zero-sized `Prop` type
//! 2. **Proof Tokens**: `Established<P>` witnesses that proposition P holds
//! 3. **Evidence Bundles**: Structs composing multiple proofs for complex operations
//! 4. **Trait Interfaces**: Object-safe traits returning `Established<P>` proofs
//!
//! # Structure
//!
//! - [`combat`] - Combat system propositions (attack, defense, damage)
//! - [`skills`] - Skill check and advancement propositions
//! - [`character`] - Character creation and validation propositions
//! - [`proof_composition`] - Evidence bundles for multi-step operations
//! - [`traits`] - Trait interfaces returning proof tokens
//! - [`types`] - Descriptor types for game operations
//!
//! # Example
//!
//! ```rust,ignore
//! use valinoreth::contracts::{AttackRollMade, AttackOutcomeDetermined, Established};
//! use valinoreth::contracts::proof_composition::AttackResolutionEvidence;
//!
//! // Atomic proofs
//! let roll_proof: Established<AttackRollMade> = Established::assert();
//! let outcome_proof: Established<AttackOutcomeDetermined> = Established::assert();
//!
//! // Composite evidence bundle
//! let evidence = AttackResolutionEvidence {
//!     roll_made: roll_proof,
//!     outcome: outcome_proof,
//! };
//! ```
//!
//! # Integration with Game Systems
//!
//! Traits in the main crate (e.g., `CombatResolver`) will return `Established<P>`
//! proofs, enabling compile-time verification that all GURPS rules are followed:
//!
//! ```rust,ignore
//! trait CombatResolver {
//!     async fn resolve_attack(
//!         &self,
//!         skill: i32,
//!         modifiers: Vec<Modifier>,
//!     ) -> Result<(AttackResult, AttackResolutionEvidence), CombatError>;
//! }
//! ```
//!
//! # Future Work
//!
//! This module will expand to cover:
//! - Magic system (if implementing magic)
//! - Equipment and encumbrance
//! - Social interactions and influence
//!
//! Eventually, the entire GURPS ruleset will form a [`VerifiedStateMachine`][elicitation::contracts::VerifiedStateMachine]
//! with formally verified state transitions.

pub mod character;
pub mod combat;
mod credentials;
pub mod proof_composition;
pub mod skills;
pub mod traits;
pub mod types;

// Re-export core contract types from elicitation
pub use elicitation::contracts::{
    And, Established, Implies, InVariant, Is, Prop, ProvableFrom, Refines,
};

// Re-export combat propositions
pub use combat::{
    AimBonusApplied, AllOutAttackDeclared, AttackCriticalFailure, AttackCriticalSuccess,
    AttackFailed, AttackOutcomeDetermined, AttackRollMade, AttackSuccessful,
    BasicDamageCalculated, DamageResistanceApplied, DeceptiveAttackApplied,
    DefenseCriticalFailure, DefenseCriticalSuccess, DefenseFailed, DefenseOutcomeDetermined,
    DefenseRollMade, DefenseSuccessful, FeintSuccessful, HitLocationDetermined, InjuryApplied,
    InjuryCalculated, LocationMultiplierApplied, RapidStrikeExecuted, WeaponDamageRolled,
    WoundingModifierApplied,
};

// Re-export skill propositions
pub use skills::{
    CharacterPointsSpentOnSkill, ComplementarySkillBonusApplied, ContestWinnerDetermined,
    DefaultPenaltyApplied, FamiliarityPenaltyApplied, SituationalModifierApplied,
    SkillCheckCriticalFailure, SkillCheckCriticalSuccess, SkillCheckFailed,
    SkillCheckOutcomeDetermined, SkillCheckRollMade, SkillCheckSuccessful, SkillContestResolved,
    SkillDefaultedToAttribute, SkillDefaultedToRelatedSkill, SkillLevelIncreased,
    SkillPointBudgetValid, SkillPrerequisiteMet, TaskDifficultyModifierApplied, TechniqueUsed,
    TimeSpentModifierApplied, WildcardSkillUsed,
};

// Re-export character propositions
pub use character::{
    AdvantageLevelValid, AdvantageModifiersCostCalculated, AdvantagePurchased,
    AdvantagePrerequisiteMet, AttributeCostCalculated, AttributePurchased,
    AttributesMeetCampaignMinimums, BasicMoveCalculated, BasicSpeedCalculated, CharacterComplete,
    CharacterIdentified, CharacterPointValueSet, CharacterValid, DisadvantageLevelValid,
    DisadvantageTaken, DisadvantagePointLimitRespected, DisadvantagesNotConflicting,
    DodgeCalculated, FatiguePointsSet, HitPointsSet, PerceptionSet, PointBudgetBalanced,
    QuirkLimitRespected, QuirkTaken, RacialTemplateRequirementsMet, SecondaryCharacteristicPurchased,
    SelfControlRollSpecified, WillSet,
};

// Re-export evidence bundles
pub use proof_composition::{
    AdvantagePurchaseEvidence, AllOutAttackEvidence, AttributePurchaseEvidence,
    AttackCriticalFailureEvidence, AttackCriticalSuccessEvidence, AttackFailureEvidence,
    AttackResolutionEvidence, AttackSuccessEvidence, BasicDamageEvidence,
    CharacterCreationEvidence, CharacterValidationEvidence, CombatHitEvidence, CombatMissEvidence,
    ComplementarySkillEvidence, DeceptiveAttackEvidence, DefenseCriticalFailureEvidence,
    DefenseCriticalSuccessEvidence, DefenseFailureEvidence, DefenseResolutionEvidence,
    DefenseSuccessEvidence, DerivedStatsEvidence, DisadvantageTakenEvidence, FeintEvidence,
    InjuryApplicationEvidence, InjuryCalculationEvidence, RapidStrikeEvidence,
    SecondaryCharacteristicEvidence, SkillAttributeDefaultEvidence, SkillCheckCriticalFailureEvidence,
    SkillCheckCriticalSuccessEvidence, SkillCheckFailureEvidence, SkillCheckResolutionEvidence,
    SkillCheckSuccessEvidence, SkillImprovementEvidence, SkillModifiersEvidence,
    SkillRelatedDefaultEvidence, TechniqueUsageEvidence, WildcardSkillEvidence,
};

// Re-export trait interfaces
pub use traits::{
    AttackMeta, AttackResolver, CharacterAdvancement, CharacterBuilder, CharacterImprovement,
    CombatExchangeResult, CombatExecutor, CombatResult, ContractError, DamageCalculator,
    DamageMeta, DefenseMeta, DefenseResolver, ManeuverExecutor, MissReason, SkillCheckExecutor,
    SkillManager,
};

// Re-export descriptor types
pub use types::{
    AdvantageDescriptor, AdvantageDescriptorBuilder, ArmorDescriptor, AttributeDescriptor,
    AttributeMinimums, AttackDescriptor, AttackDescriptorBuilder, AttackRollResult,
    CharacterCreationDescriptor, CharacterCreationDescriptorBuilder, CharacterDescriptor,
    CharacterDescriptorBuilder, CombatantDescriptor, CombatantDescriptorBuilder,
    DamageDescriptor, DamageDescriptorBuilder, DamageResult, DamageTypeDescriptor,
    DefenseDescriptor, DefenseDescriptorBuilder, DefenseRollResult, DefenseType,
    DerivedStatsDescriptor, DisadvantageDescriptor, DisadvantageDescriptorBuilder,
    FeintDescriptor, FeintResult, ModifierDescriptor, RapidStrikeDescriptor,
    SecondaryCharacteristicDescriptor, SecondaryCharacteristicType, SkillCheckDescriptor,
    SkillCheckDescriptorBuilder, SkillCheckResult, SkillDefaultDescriptor, SkillDefaultType,
    SkillDescriptor, SkillDescriptorBuilder, SkillDifficulty,
};
