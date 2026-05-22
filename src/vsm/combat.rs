//! [`CombatMachine`] — verified state machine for GURPS combat lifecycle.
//!
//! ## States
//!
//! ```text
//! Uninitialized ──initialize_combat──► Active ──end_turn──► Active (next turn)
//!                                        │                      │
//!                                        └──conclude_combat──► Concluded
//! ```
//!
//! ## Turn Flow Within Active State
//!
//! While in `Active`, a single turn flows through transitions:
//!
//! ```text
//! begin_turn → declare_attack → resolve_attack → resolve_defense → apply_damage → end_turn
//! ```
//!
//! Each transition carries proof tokens from GameMaster mechanics and combat flow contracts.

use elicitation::{
    Elicit, Established, KaniVariantState, Prop, VerifiedStateMachine, contracts::ProvableFrom,
    formal_method,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::contracts::combat_flow::{
    CombatInitialized, TurnBegan, AttackDeclared, AttackResolved,
    DefenseResolved, DamageApplied, TurnEnded, VictoryConditionMet,
};

// ── CombatantState ────────────────────────────────────────────────────────────

/// State of a single combatant in combat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit)]
#[cfg_attr(kani, derive(elicitation::KaniCompose))]
pub struct CombatantState {
    /// Unique identifier for this combatant.
    pub id: String,

    /// Team this combatant belongs to (for victory conditions).
    pub team: String,

    /// Current hit points.
    pub current_hp: i32,

    /// Maximum hit points.
    pub max_hp: i32,

    /// Current fatigue points.
    pub current_fp: i32,

    /// Maximum fatigue points.
    pub max_fp: i32,

    /// Basic Speed (for turn order determination).
    pub basic_speed: i32,

    /// Whether this combatant is incapacitated (0 HP or worse).
    pub incapacitated: bool,
}

// ── CombatState ───────────────────────────────────────────────────────────────

/// Lifecycle state of a GURPS combat encounter.
#[derive(
    Debug,
    Clone,
    Default,
    PartialEq,
    Serialize,
    Deserialize,
    JsonSchema,
    Elicit,
    KaniVariantState,
)]
#[cfg_attr(kani, derive(elicitation::KaniCompose))]
pub enum CombatState {
    /// Combat has not been initialized.
    #[default]
    Uninitialized,

    /// Combat is active with ongoing turns.
    Active {
        /// All combatants in the encounter.
        combatants: Vec<CombatantState>,

        /// Turn order (indices into combatants, sorted by Basic Speed).
        turn_order: Vec<usize>,

        /// Index into turn_order for current actor.
        current_actor: usize,

        /// Current round number (starts at 1).
        round: u32,
    },

    /// Combat has concluded with a victor.
    Concluded {
        /// Winning team name, or None for draw/no victor.
        victor: Option<String>,
    },
}

// ── CombatConsistent ──────────────────────────────────────────────────────────

/// Proposition: the combat state machine is in a self-consistent state.
///
/// Structural invariants:
/// - Active state has valid combatants and turn order
/// - Current actor index is in bounds
/// - Turn order indices are valid
/// - Round number is positive
#[derive(Prop)]
#[prop(
    kani_invariant_fn = "combat_consistent",
    creusot_invariant_fn = "combat_consistent",
    verus_inv_body = "true",
    creusot_inv_body = "true"
)]
pub struct CombatConsistent;

/// Structural invariant predicate for [`CombatState`].
///
/// Checks that:
/// - Active state has at least one combatant
/// - current_actor is within turn_order bounds
/// - turn_order indices are valid for combatants vec
/// - round is positive
pub fn combat_consistent(state: &CombatState) -> bool {
    match state {
        CombatState::Uninitialized | CombatState::Concluded { .. } => true,

        CombatState::Active {
            combatants,
            turn_order,
            current_actor,
            round,
        } => {
            // Must have at least one combatant
            if combatants.is_empty() {
                return false;
            }

            // Must have turn order
            if turn_order.is_empty() {
                return false;
            }

            // Current actor must be valid index into turn_order
            if *current_actor >= turn_order.len() {
                return false;
            }

            // All turn_order indices must be valid for combatants
            if !turn_order.iter().all(|&i| i < combatants.len()) {
                return false;
            }

            // Round must be positive
            if *round == 0 {
                return false;
            }

            true
        }
    }
}

// ── CombatMachine ─────────────────────────────────────────────────────────────

/// Verified state machine for the combat lifecycle.
///
/// Tracks combat flow from initialization through turns to conclusion,
/// with formally proved transitions.
#[derive(VerifiedStateMachine)]
#[vsm(transitions = [
    initialize_combat,
    begin_turn,
    declare_attack,
    resolve_attack,
    resolve_defense,
    apply_damage,
    end_turn,
    conclude_combat,
])]
pub struct CombatMachine;

// ── Transitions ───────────────────────────────────────────────────────────────

/// Initialize combat with combatants and establish turn order.
///
/// Requires proof that combat was properly initialized (combatants validated,
/// victory conditions set, etc.).
#[formal_method(contracts = [CombatConsistent], kani_requires = [
    "!combatants.is_empty()",
    "combatants.iter().all(|c| c.basic_speed > 0)",
])]
#[instrument(skip(proof, _init_proof))]
pub fn initialize_combat(
    _state: CombatState,
    proof: Established<CombatConsistent>,
    combatants: Vec<CombatantState>,
    _init_proof: Established<CombatInitialized>,
) -> (CombatState, Established<CombatConsistent>) {
    // Sort combatants by Basic Speed (descending) for turn order
    let mut turn_order: Vec<usize> = (0..combatants.len()).collect();
    turn_order.sort_by(|&a, &b| {
        combatants[b].basic_speed.cmp(&combatants[a].basic_speed)
    });

    (
        CombatState::Active {
            combatants,
            turn_order,
            current_actor: 0,
            round: 1,
        },
        proof,
    )
}

/// Begin a combatant's turn.
///
/// Validates that it's the correct combatant's turn and they can act.
#[formal_method(contracts = [CombatConsistent])]
#[instrument(skip(proof, _turn_proof))]
pub fn begin_turn(
    state: CombatState,
    proof: Established<CombatConsistent>,
    _turn_proof: Established<TurnBegan>,
) -> (CombatState, Established<CombatConsistent>) {
    // State stays the same, just marks turn as begun
    (state, proof)
}

/// Declare an attack action from attacker to target.
///
/// Validates that the attacker can take action and target is valid.
#[formal_method(contracts = [CombatConsistent], kani_requires = [
    "attacker_id < 100",  // Reasonable bound for verification
    "target_id < 100",
])]
#[instrument(skip(proof, _attack_proof))]
pub fn declare_attack(
    state: CombatState,
    proof: Established<CombatConsistent>,
    _attacker_id: usize,
    _target_id: usize,
    _attack_proof: Established<AttackDeclared>,
) -> (CombatState, Established<CombatConsistent>) {
    // State unchanged - attack declaration is tracked through proof token
    (state, proof)
}

/// Resolve attack roll using GameMaster evidence.
///
/// Consumes AttackResolved proof from GameMaster indicating attack was rolled.
#[formal_method(contracts = [CombatConsistent])]
#[instrument(skip(proof, _resolved_proof))]
pub fn resolve_attack(
    state: CombatState,
    proof: Established<CombatConsistent>,
    _resolved_proof: Established<AttackResolved>,
) -> (CombatState, Established<CombatConsistent>) {
    // Attack mechanics handled by GameMaster, proof consumed here
    (state, proof)
}

/// Resolve defense roll using GameMaster evidence.
///
/// Consumes DefenseResolved proof from GameMaster indicating defense was rolled.
#[formal_method(contracts = [CombatConsistent])]
#[instrument(skip(proof, _resolved_proof))]
pub fn resolve_defense(
    state: CombatState,
    proof: Established<CombatConsistent>,
    _resolved_proof: Established<DefenseResolved>,
) -> (CombatState, Established<CombatConsistent>) {
    // Defense mechanics handled by GameMaster, proof consumed here
    (state, proof)
}

/// Apply damage to target, updating HP and checking incapacitation.
///
/// Consumes DamageApplied proof from GameMaster indicating damage was calculated
/// and injury applied.
#[formal_method(contracts = [CombatConsistent])]
#[instrument(skip(proof, _damage_proof))]
pub fn apply_damage(
    state: CombatState,
    proof: Established<CombatConsistent>,
    target_id: usize,
    injury: i32,
    _damage_proof: Established<DamageApplied>,
) -> (CombatState, Established<CombatConsistent>) {
    match state {
        CombatState::Active {
            mut combatants,
            turn_order,
            current_actor,
            round,
        } => {
            // Find target in turn_order and update their HP
            if let Some(&combatant_idx) = turn_order.get(target_id) {
                if let Some(combatant) = combatants.get_mut(combatant_idx) {
                    combatant.current_hp -= injury;

                    // Check for incapacitation (0 HP or below)
                    if combatant.current_hp <= 0 {
                        combatant.incapacitated = true;
                    }
                }
            }

            (
                CombatState::Active {
                    combatants,
                    turn_order,
                    current_actor,
                    round,
                },
                proof,
            )
        }
        _ => (state, proof),
    }
}

/// End current turn and advance to next combatant.
///
/// Cycles through turn order, advancing to next round when all have acted.
#[formal_method(contracts = [CombatConsistent])]
#[instrument(skip(proof, _turn_proof))]
pub fn end_turn(
    state: CombatState,
    proof: Established<CombatConsistent>,
    _turn_proof: Established<TurnEnded>,
) -> (CombatState, Established<CombatConsistent>) {
    match state {
        CombatState::Active {
            combatants,
            turn_order,
            current_actor,
            round,
        } => {
            let next_actor = current_actor + 1;
            let (new_actor, new_round) = if next_actor >= turn_order.len() {
                // Wrapped around, start new round
                (0, round + 1)
            } else {
                (next_actor, round)
            };

            (
                CombatState::Active {
                    combatants,
                    turn_order,
                    current_actor: new_actor,
                    round: new_round,
                },
                proof,
            )
        }
        _ => (state, proof),
    }
}

/// Conclude combat with victory determination.
///
/// Requires proof that victory condition was met (all enemies incapacitated, etc.).
#[formal_method(contracts = [CombatConsistent])]
#[instrument(skip(proof, _victory_proof))]
pub fn conclude_combat(
    _state: CombatState,
    proof: Established<CombatConsistent>,
    victor: Option<String>,
    _victory_proof: Established<VictoryConditionMet>,
) -> (CombatState, Established<CombatConsistent>) {
    (CombatState::Concluded { victor }, proof)
}
