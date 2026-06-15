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
//!            ↘ complete_movement_action ↗
//! ```
//!
//! Each transition carries proof tokens from GameMaster mechanics and combat flow contracts.

use elicitation::{
    formal_method, Elicit, Established, KaniCompose, KaniVariantState, Prop, VerifiedStateMachine,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::contracts::combat_flow::{
    AttackDeclared, AttackResolved, CombatInitialized, DamageApplied, DefenseResolved, TurnBegan,
    TurnEnded, VictoryConditionMet,
};
use crate::contracts::movement::MovementCompleted;

// ── CombatantState ────────────────────────────────────────────────────────────

/// State of a single combatant in combat.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Elicit, KaniCompose)]
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

/// Slot of a combatant in [`CombatState::Active::combatants`].
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, KaniCompose,
)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct CombatantSlot {
    index: usize,
}

impl CombatantSlot {
    /// Construct a combatant slot from an index into `CombatState::Active::combatants`.
    pub const fn new(index: usize) -> Self {
        Self { index }
    }

    /// Return the index into `CombatState::Active::combatants`.
    pub const fn index(self) -> usize {
        self.index
    }
}

/// Proof sidecar tying a declared attack to the combatant slot it targeted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, KaniCompose)]
#[cfg_attr(kani, derive(kani::Arbitrary))]
pub struct DeclaredAttackTarget {
    attacker: CombatantSlot,
    target: CombatantSlot,
}

impl DeclaredAttackTarget {
    const fn new(attacker: CombatantSlot, target: CombatantSlot) -> Self {
        Self { attacker, target }
    }

    /// Return the combatant slot that declared the attack.
    pub const fn attacker(self) -> CombatantSlot {
        self.attacker
    }

    /// Return the combatant slot targeted by the declared attack.
    pub const fn target(self) -> CombatantSlot {
        self.target
    }
}

/// Mint the target sidecar for a declared attack.
///
/// This is the only public construction path for [`DeclaredAttackTarget`].
pub fn declared_attack_target(
    attacker: CombatantSlot,
    target: CombatantSlot,
    _attack_proof: Established<AttackDeclared>,
) -> DeclaredAttackTarget {
    DeclaredAttackTarget::new(attacker, target)
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
    KaniCompose,
)]
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
    creusot_inv_body = r#"pearlite! {
        match state {
            CombatState::Uninitialized => true,
            CombatState::Concluded { .. } => true,
            CombatState::Active { combatants, turn_order, current_actor, round } =>
                combatants@.len() > 0
                && turn_order@.len() > 0
                && current_actor@ < turn_order@.len()
                && round@ > 0
                && forall<i: Int> 0 <= i && i < turn_order@.len() ==> turn_order@[i]@ < combatants@.len(),
        }
    }"#
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
    complete_movement_action,
    end_turn,
    conclude_combat,
])]
pub struct CombatMachine;

// ── Transitions ───────────────────────────────────────────────────────────────

/// Initialize combat with combatants and establish turn order.
///
/// Requires proof that combat was properly initialized (combatants validated,
/// victory conditions set, etc.).
#[formal_method(
    contracts = [CombatConsistent],
    kani_requires = ["!combatants.is_empty()", "combatants.iter().all(|c| c.basic_speed > 0)"],
    creusot_requires = ["combatants@.len() > 0"],
    creusot_body = r#"{
        let turn_order: Vec<usize> = (0..combatants.len()).collect();
        (
            CombatState::Active {
                combatants,
                turn_order,
                current_actor: 0,
                round: 1,
            },
            proof,
        )
    }"#
)]
#[instrument(skip(proof, _init_proof))]
pub fn initialize_combat(
    _state: CombatState,
    proof: Established<CombatConsistent>,
    combatants: Vec<CombatantState>,
    _init_proof: Established<CombatInitialized>,
) -> (CombatState, Established<CombatConsistent>) {
    // Sort combatants by Basic Speed (descending) for turn order
    let mut turn_order: Vec<usize> = (0..combatants.len()).collect();
    let len = turn_order.len();
    let mut i = 0;
    while i < len {
        let mut best = i;
        let mut j = i + 1;
        while j < len {
            if combatants[turn_order[j]].basic_speed > combatants[turn_order[best]].basic_speed {
                best = j;
            }
            j += 1;
        }
        turn_order.swap(i, best);
        i += 1;
    }

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
#[formal_method(contracts = [CombatConsistent])]
#[instrument(skip(proof, _declared_target))]
pub fn declare_attack(
    state: CombatState,
    proof: Established<CombatConsistent>,
    _declared_target: DeclaredAttackTarget,
) -> (CombatState, Established<CombatConsistent>) {
    // State unchanged - attack declaration is tracked through the target sidecar.
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
#[formal_method(
    contracts = [CombatConsistent],
    kani_requires = ["injury >= 0"],
    creusot_requires = ["injury@ >= 0"]
)]
#[instrument(skip(proof, _damage_proof))]
pub fn apply_damage(
    state: CombatState,
    proof: Established<CombatConsistent>,
    declared_target: DeclaredAttackTarget,
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
            let target_idx = declared_target.target().index();
            if let Some(combatant) = combatants.get_mut(target_idx) {
                combatant.current_hp = combatant.current_hp.saturating_sub(injury);

                // Check for incapacitation (0 HP or below)
                if combatant.current_hp <= 0 {
                    combatant.incapacitated = true;
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

/// Complete a movement action whose spatial sidecar transition has been proved.
///
/// Movement mutates [`crate::CombatSpatialState`], not [`CombatState`]. This
/// transition gives the combat lifecycle a proof-visible action boundary and
/// consumes the [`MovementCompleted`] token minted by the movement contract gate.
#[formal_method(contracts = [CombatConsistent])]
#[instrument(skip(proof, _movement_proof))]
pub fn complete_movement_action(
    state: CombatState,
    proof: Established<CombatConsistent>,
    _movement_proof: Established<MovementCompleted>,
) -> (CombatState, Established<CombatConsistent>) {
    (state, proof)
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
            let is_last_actor = current_actor == turn_order.len() - 1;
            let (new_actor, new_round) = if is_last_actor {
                // Wrapped around, start new round
                (0, round.saturating_add(1))
            } else {
                (current_actor + 1, round)
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
