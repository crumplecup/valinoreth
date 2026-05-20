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
//! - [`proof_composition`] - Evidence bundles for multi-step operations
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
//! - Skill checks and task resolution
//! - Character creation and advancement
//! - Magic system (if implementing magic)
//! - Equipment and encumbrance
//! - Social interactions and influence
//!
//! Eventually, the entire GURPS ruleset will form a [`VerifiedStateMachine`][elicitation::contracts::VerifiedStateMachine]
//! with formally verified state transitions.

pub mod combat;
mod credentials;
pub mod proof_composition;
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

// Re-export evidence bundles
pub use proof_composition::{
    AllOutAttackEvidence, AttackCriticalFailureEvidence, AttackCriticalSuccessEvidence,
    AttackFailureEvidence, AttackResolutionEvidence, AttackSuccessEvidence, BasicDamageEvidence,
    CombatHitEvidence, CombatMissEvidence, DeceptiveAttackEvidence,
    DefenseCriticalFailureEvidence, DefenseCriticalSuccessEvidence, DefenseFailureEvidence,
    DefenseResolutionEvidence, DefenseSuccessEvidence, FeintEvidence,
    InjuryApplicationEvidence, InjuryCalculationEvidence, RapidStrikeEvidence,
};

// Re-export trait interfaces
pub use traits::{
    AttackMeta, AttackResolver, CombatExchangeResult, CombatExecutor, CombatResult,
    ContractError, DamageCalculator, DamageMeta, DefenseMeta, DefenseResolver, ManeuverExecutor,
    MissReason,
};

// Re-export descriptor types
pub use types::{
    ArmorDescriptor, AttackDescriptor, AttackDescriptorBuilder, AttackRollResult,
    CombatantDescriptor, CombatantDescriptorBuilder, DamageDescriptor, DamageDescriptorBuilder,
    DamageResult, DamageTypeDescriptor, DefenseDescriptor, DefenseDescriptorBuilder,
    DefenseRollResult, DefenseType, FeintDescriptor, FeintResult, HitLocation,
    RapidStrikeDescriptor,
};
