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
//! - [`magic`] - Spell casting and magic system propositions
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
//! - Equipment and encumbrance
//! - Social interactions and influence
//!
//! Eventually, the entire GURPS ruleset will form a [`VerifiedStateMachine`][elicitation::contracts::VerifiedStateMachine]
//! with formally verified state transitions.

pub mod character;
pub mod combat;
pub mod combat_flow;
pub mod credentials;
pub mod magic;
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
    AttackFailed, AttackOutcomeDetermined, AttackRollMade, AttackSuccessful, BasicDamageCalculated,
    DamageResistanceApplied, DeceptiveAttackApplied, DefenseCriticalFailure,
    DefenseCriticalSuccess, DefenseFailed, DefenseOutcomeDetermined, DefenseRollMade,
    DefenseSuccessful, FeintSuccessful, HitLocationDetermined, InjuryApplied, InjuryCalculated,
    LocationMultiplierApplied, RapidStrikeExecuted, WeaponDamageRolled, WoundingModifierApplied,
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
    AdvantageLevelValid, AdvantageModifiersCostCalculated, AdvantagePrerequisiteMet,
    AdvantagePurchased, AttributeCostCalculated, AttributePurchased,
    AttributesMeetCampaignMinimums, BasicMoveCalculated, BasicSpeedCalculated, CharacterComplete,
    CharacterIdentified, CharacterPointValueSet, CharacterValid, DisadvantageLevelValid,
    DisadvantagePointLimitRespected, DisadvantageTaken, DisadvantagesNotConflicting,
    DodgeCalculated, FatiguePointsSet, HitPointsSet, PerceptionSet, PointBudgetBalanced,
    QuirkLimitRespected, QuirkTaken, RacialTemplateRequirementsMet,
    SecondaryCharacteristicPurchased, SelfControlRollSpecified, WillSet,
};

// Re-export magic propositions
pub use magic::{
    BaseEnergyCostDetermined, CeremonialCastingBegun, CeremonialSpellCompleted, ConcentrationBegun,
    ConcentrationCompleted, ConcentrationMaintained, EnergyPaidFromCaster,
    EnergyPooledFromParticipants, EnvironmentModifierApplied, FinalEnergyCostCalculated,
    MageryRequirementMet, MaintenanceEnergyPaid, ResistanceOvercome, ResistanceRollMade,
    ResistanceRollRequired, SizeSpeedModifierApplied, SkillBasedCostReductionApplied,
    SpellCastingFailed, SpellCastingOutcomeDetermined, SpellCastingSucceeded, SpellCriticalFailure,
    SpellCriticalSuccess, SpellDurationDetermined, SpellEffectApplied, SpellLearned,
    SpellMaintained, SpellPrerequisitesMet, SpellRangeChecked, SpellResistedSuccessfully,
    SpellSkillLevelSet, SpellSkillRollMade, SpellTargetDetermined, TimeModifierApplied,
};

// Re-export combat flow propositions (game state management)
pub use combat_flow::{
    AttackDeclared, AttackResolved, CanApplyDamage, CanTakeAction, CombatConcluded,
    CombatInitialized, DamageApplied, DefenseRequired, DefenseResolved, ManeuverSelected,
    RoundCompleted, TurnBegan, TurnEnded, TurnOrderEstablished, VictoryConditionMet,
};

// Re-export evidence bundles
pub use proof_composition::{
    AdvantagePurchaseEvidence, AllOutAttackEvidence, AttackCriticalFailureEvidence,
    AttackCriticalSuccessEvidence, AttackFailureEvidence, AttackResolutionEvidence,
    AttackSuccessEvidence, AttributePurchaseEvidence, BasicDamageEvidence, CeremonialMagicEvidence,
    CharacterCreationEvidence, CharacterValidationEvidence, CombatHitEvidence, CombatMissEvidence,
    ComplementarySkillEvidence, CompleteSpellCastingEvidence, ConcentrationEvidence,
    DeceptiveAttackEvidence, DefenseCriticalFailureEvidence, DefenseCriticalSuccessEvidence,
    DefenseFailureEvidence, DefenseResolutionEvidence, DefenseSuccessEvidence,
    DerivedStatsEvidence, DisadvantageTakenEvidence, EnergyCostEvidence, EnergyPaymentEvidence,
    FeintEvidence, InjuryApplicationEvidence, InjuryCalculationEvidence, RapidStrikeEvidence,
    ResistanceOvercomeEvidence, SecondaryCharacteristicEvidence, SkillAttributeDefaultEvidence,
    SkillCheckCriticalFailureEvidence, SkillCheckCriticalSuccessEvidence,
    SkillCheckFailureEvidence, SkillCheckResolutionEvidence, SkillCheckSuccessEvidence,
    SkillImprovementEvidence, SkillModifiersEvidence, SkillRelatedDefaultEvidence,
    SpellCastingFailureEvidence, SpellCastingResolutionEvidence, SpellCastingSuccessEvidence,
    SpellCriticalFailureEvidence, SpellCriticalSuccessEvidence, SpellEffectEvidence,
    SpellLearningEvidence, SpellMaintenanceEvidence, SpellResistanceEvidence,
    TechniqueUsageEvidence, WildcardSkillEvidence,
};

// Re-export trait interfaces
pub use traits::{
    AttackMeta, AttackResolver, CharacterAdvancement, CharacterBuilder, CharacterImprovement,
    CombatExchangeResult, CombatExecutor, CombatResult, ContractError, DamageCalculator,
    DamageMeta, DefenseMeta, DefenseResolver, ManeuverExecutor, MissReason, SkillCheckExecutor,
    SkillManager, SpellCaster, SpellEffectResolver, SpellExecutionResult, SpellExecutor,
    SpellManager,
};

// Re-export descriptor types
pub use types::{
    AdvantageDescriptor, ArmorDescriptor, AttackDescriptor, AttackRollResult, AttributeDescriptor,
    AttributeMinimums, AttributeType, CasterDescriptor, CeremonialMagicDescriptor,
    CharacterCreationDescriptor, CharacterDescriptor, CombatantDescriptor, DamageDescriptor,
    DamageResult, DamageTypeDescriptor, DefenseDescriptor, DefenseRollResult, DefenseType,
    DerivedStatsDescriptor, DisadvantageDescriptor, FeintDescriptor, FeintResult, HitLocation,
    ModifierDescriptor, RapidStrikeDescriptor, ResistanceResult, SecondaryCharacteristicDescriptor,
    SecondaryCharacteristicType, SkillCheckDescriptor, SkillCheckResult, SkillDefaultDescriptor,
    SkillDefaultType, SkillDescriptor, SkillDifficulty, SpellCastingDescriptor, SpellCastingResult,
    SpellClass, SpellCollege, SpellDescriptor, SpellEffectDescriptor, SpellResistanceDescriptor,
};

#[cfg(not(creusot))]
pub use types::{
    AdvantageDescriptorBuilder, AttackDescriptorBuilder, CasterDescriptorBuilder,
    CeremonialMagicDescriptorBuilder, CharacterCreationDescriptorBuilder,
    CharacterDescriptorBuilder, CombatantDescriptorBuilder, DamageDescriptorBuilder,
    DefenseDescriptorBuilder, DisadvantageDescriptorBuilder, SkillCheckDescriptorBuilder,
    SkillDescriptorBuilder, SpellCastingDescriptorBuilder, SpellDescriptorBuilder,
};
