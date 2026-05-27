//! Player decision types for elicitation.
//!
//! Each type here is a choice point the GM presents to a player during combat.
//! Deriving [`Elicit`] means the same enum works as:
//!
//! - A numbered option list for a [`TuiCommunicator`] (human player), and
//! - A tool schema for an [`LlmElicitCommunicator`] (agent player).
//!
//! The GM calls `SomeChoice::elicit(&player.communicator).await` and receives
//! back a typed value regardless of who is driving.
//!
//! [`TuiCommunicator`]: crate::TuiCommunicator
//! [`LlmElicitCommunicator`]: strictly_server::tui::mcp_communicator::LlmElicitCommunicator

use derive_more::Display;
use elicitation::{Elicit, KaniCompose};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// ── ManeuverChoice ────────────────────────────────────────────────────────────

/// A player's choice of combat maneuver on their turn.
///
/// Covers the subset of GURPS maneuvers needed for a minimal combat loop.
/// Additional maneuvers (Aim, Evaluate, Feint, MoveAndAttack, Ready,
/// Concentrate) can be added as the implementation expands.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
    Elicit,
    KaniCompose,
    Display,
)]
pub enum ManeuverChoice {
    /// Armed or unarmed attack against a target in reach/range.
    ///
    /// The player may step (1 meter) before or after attacking.
    #[display("Attack")]
    Attack,

    /// Commit fully to offense.
    ///
    /// Grants a bonus to hit or damage but forfeits all active defenses until
    /// the next turn.
    #[display("All-Out Attack")]
    AllOutAttack,

    /// Commit fully to defense.
    ///
    /// Grants +2 to all active defense rolls until the next turn.
    /// The player may step but not attack.
    #[display("All-Out Defense")]
    AllOutDefense,

    /// Move up to full Move score without attacking.
    #[display("Move")]
    Move,

    /// Declare a trigger condition and wait to act on it.
    ///
    /// If the trigger occurs before the next turn, the player may interrupt
    /// with an Attack or other action.
    #[display("Wait")]
    Wait,
}

// ── DefenseChoice ─────────────────────────────────────────────────────────────

/// A player's choice of active defense when struck.
///
/// Presented to the player immediately after an attack succeeds.  The GM
/// validates that the chosen defense is available (e.g. Parry requires a
/// ready weapon; Block requires a shield) before resolving.
///
/// `None` means the player accepts the hit without rolling — valid but rarely
/// optimal.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
    Elicit,
    KaniCompose,
    Display,
)]
pub enum DefenseChoice {
    /// Roll against Dodge score (Basic Speed + 3, rounded down).
    ///
    /// Always available; not affected by weapon readiness.
    #[display("Dodge")]
    Dodge,

    /// Roll against Parry score (weapon skill ÷ 2 + 3).
    ///
    /// Requires a ready melee weapon.
    #[display("Parry")]
    Parry,

    /// Roll against Block score (shield skill ÷ 2 + 3).
    ///
    /// Requires a ready shield.
    #[display("Block")]
    Block,

    /// Accept the hit without rolling an active defense.
    #[display("Accept Hit")]
    None,
}
