//! Integration helpers for connecting GameMaster evidence to VSM transitions.
//!
//! This module provides utility functions that demonstrate how to bridge
//! GameMaster trait calls (which return GURPS mechanics proofs) with
//! VSM transitions (which require combat flow proofs).
//!
//! # Pattern
//!
//! The ProvableFrom implementations in `combat_flow.rs` establish type-level
//! relationships between GameMaster proofs and VSM requirements. These helpers
//! show the usage pattern:
//!
//! ```text
//! 1. Call GameMaster trait method → receives Established<MechanicsProof>
//! 2. Convert via ProvableFrom → Established<FlowProof>
//! 3. Pass to VSM transition → updates state
//! ```

use crate::contracts::combat::{AttackOutcomeDetermined, DefenseOutcomeDetermined, InjuryApplied};
use crate::contracts::combat_flow::{AttackResolved, DamageApplied, DefenseResolved};
use elicitation::contracts::Established;

/// Converts GameMaster attack evidence to VSM attack resolution proof.
///
/// After calling `GameMaster::resolve_attack()`, use this to obtain the
/// proof required by `CombatMachine::resolve_attack()`.
///
/// # Example
///
/// ```rust,ignore
/// // Step 1: GameMaster resolves attack mechanics
/// let (attack_result, attack_evidence) = gm.resolve_attack(descriptor).await?;
///
/// // Step 2: Convert to VSM proof via ProvableFrom<AttackOutcomeDetermined> for AttackResolved
/// let vsm_proof = attack_resolved_from_gm(attack_evidence);
///
/// // Step 3: VSM updates state
/// let (new_state, state_proof) = resolve_attack(state, state_proof, vsm_proof);
/// ```
#[tracing::instrument(skip(_gm_proof))]
pub fn attack_resolved_from_gm(
    _gm_proof: Established<AttackOutcomeDetermined>,
) -> Established<AttackResolved> {
    Established::prove(&AttackOutcomeDetermined)
}

/// Converts GameMaster defense evidence to VSM defense resolution proof.
///
/// After calling `GameMaster::resolve_defense()`, use this to obtain the
/// proof required by `CombatMachine::resolve_defense()`.
#[tracing::instrument(skip(_gm_proof))]
pub fn defense_resolved_from_gm(
    _gm_proof: Established<DefenseOutcomeDetermined>,
) -> Established<DefenseResolved> {
    Established::prove(&DefenseOutcomeDetermined)
}

/// Converts GameMaster damage evidence to VSM damage application proof.
///
/// After calling `GameMaster::apply_injury()`, use this to obtain the
/// proof required by `CombatMachine::apply_damage()`.
#[tracing::instrument(skip(_gm_proof))]
pub fn damage_applied_from_gm(
    _gm_proof: Established<InjuryApplied>,
) -> Established<DamageApplied> {
    Established::prove(&InjuryApplied)
}

// ── Future Integration Points ─────────────────────────────────────────────────

// Additional conversion functions will be added as we implement:
// - Initiative/turn order establishment
// - Victory condition checking
// - Action availability validation
// - Complex multi-target effects
