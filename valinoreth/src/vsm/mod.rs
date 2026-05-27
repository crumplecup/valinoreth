//! Verified State Machines for GURPS combat and game flow.
//!
//! This module contains formal VSM definitions for combat lifecycle and
//! game state transitions. Each machine tracks a specific concern and its
//! `#[formal_method]` transitions carry proof tokens that compose into
//! verifiable game state.
//!
//! # Machines
//!
//! | Machine | States | Key invariant |
//! |---------|--------|---------------|
//! | [`CombatMachine`] | Uninitialized → Active → Concluded | `CombatConsistent` |
//!
//! # Architecture
//!
//! The VSMs are the *formal model* of game flow. They orchestrate GameMaster
//! trait calls to execute GURPS rules, collecting evidence tokens and proving
//! that state transitions followed valid game rules.
//!
//! ## Separation of Concerns
//!
//! - **VSM**: "Can you do this now?" (game flow, turn order, action economy)
//! - **GameMaster**: "How do you do this?" (GURPS mechanics, dice rolls)
//! - **ProvableFrom bridges**: Connect GameMaster evidence to VSM guards
//!
//! ## Example Flow
//!
//! ```text
//! 1. VSM: begin_turn() - proves it's combatant's turn
//! 2. VSM: declare_attack() - proves action is available
//! 3. GM: resolve_attack() - mints AttackSuccessful proof
//! 4. VSM: resolve_attack() - consumes AttackSuccessful, updates state
//! 5. GM: resolve_defense() - mints DefenseFailed proof
//! 6. VSM: apply_damage() - consumes DefenseFailed, updates HP
//! 7. VSM: end_turn() - advances to next combatant
//! ```

mod combat;
mod integration;
mod session;
mod workflow;

pub use combat::{
    apply_damage, begin_turn, combat_consistent, conclude_combat, declare_attack, end_turn,
    initialize_combat, resolve_attack, resolve_defense, CombatConsistent, CombatMachine,
    CombatState, CombatantState,
};
pub use integration::{attack_resolved_from_gm, damage_applied_from_gm, defense_resolved_from_gm};
pub use session::{CombatPhase, CombatSession, CombatStateView, CombatantView};
pub use workflow::{CombatWorkflow, WorkflowError};
// BEGIN ELICITATION KANI REEXPORTS — DO NOT EDIT
pub use combat::apply_damage_kani_contracted;
pub use combat::begin_turn_kani_contracted;
pub use combat::conclude_combat_kani_contracted;
pub use combat::declare_attack_kani_contracted;
pub use combat::end_turn_kani_contracted;
pub use combat::initialize_combat_kani_contracted;
pub use combat::resolve_attack_kani_contracted;
pub use combat::resolve_defense_kani_contracted;
// END ELICITATION KANI REEXPORTS
