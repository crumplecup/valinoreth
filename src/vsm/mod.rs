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

pub mod combat;

pub use combat::{
    CombatConsistent, CombatMachine, CombatState, CombatantState, apply_damage, begin_turn,
    combat_consistent, declare_attack, end_turn, initialize_combat, resolve_attack,
    resolve_defense,
};
