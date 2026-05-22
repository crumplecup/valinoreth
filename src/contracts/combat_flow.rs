//! Combat flow contracts - propositions and credentials for game state management.
//!
//! These contracts represent the **implicit rules** of turn-based combat that
//! govern when actions can be taken, in what order, and with what preconditions.
//!
//! # Pattern
//!
//! GURPS mechanics (in `combat.rs`, `magic.rs`, etc.) provide credentials for
//! "how the rules work" (attack rolls, damage calculation, spell casting).
//!
//! Combat flow contracts provide credentials for "when you can do things"
//! (turn structure, action economy, sequencing).
//!
//! The VSM consumes both to orchestrate valid game flow.

use elicitation::Prop;

// ── Combat Lifecycle ──────────────────────────────────────────────────────────

/// Proposition: Combat has been properly initialized.
///
/// Requires:
/// - All combatants have valid descriptors
/// - Combat context is established
/// - Victory conditions are defined
#[derive(Prop)]
pub struct CombatInitialized;

/// Proposition: Victory or defeat condition has been met.
///
/// Requires:
/// - At least one combatant is incapacitated, surrendered, or fled
/// - Victory conditions checked against current state
#[derive(Prop)]
pub struct VictoryConditionMet;

/// Proposition: Combat has been properly concluded.
///
/// Requires:
/// - Victory condition met
/// - Final state recorded
/// - All pending effects resolved
#[derive(Prop)]
pub struct CombatConcluded;

// ── Turn Structure ────────────────────────────────────────────────────────────

/// Proposition: Turn order has been established for all combatants.
///
/// Requires:
/// - Initiative rolled for all participants
/// - Turn sequence determined
/// - First combatant identified
#[derive(Prop)]
pub struct TurnOrderEstablished;

/// Proposition: A combatant's turn has begun.
///
/// Requires:
/// - Previous turn ended (or this is first turn)
/// - Combatant is active (not incapacitated)
/// - Turn order followed
#[derive(Prop)]
pub struct TurnBegan;

/// Proposition: A combatant's turn has ended.
///
/// Requires:
/// - All required actions completed or waived
/// - Temporary effects cleared
/// - State consistent for next turn
#[derive(Prop)]
pub struct TurnEnded;

/// Proposition: A combat round has completed.
///
/// Requires:
/// - All combatants have taken their turn
/// - Round-duration effects processed
/// - Ready for next round
#[derive(Prop)]
pub struct RoundCompleted;

// ── Action Economy ────────────────────────────────────────────────────────────

/// Proposition: Combatant can take an action.
///
/// Requires:
/// - It is this combatant's turn
/// - Combatant has not used their action this turn
/// - Combatant is capable of acting (not stunned, etc.)
#[derive(Prop)]
pub struct CanTakeAction;

/// Proposition: A maneuver has been selected.
///
/// Requires:
/// - Valid maneuver for current state
/// - Maneuver requirements met (e.g., weapon ready)
/// - Action economy allows this maneuver
#[derive(Prop)]
pub struct ManeuverSelected;

/// Proposition: An attack has been declared.
///
/// Requires:
/// - Attack action is available
/// - Valid target selected
/// - Weapon/attack method specified
#[derive(Prop)]
pub struct AttackDeclared;

/// Proposition: A defense attempt is required.
///
/// Requires:
/// - Incoming attack succeeded
/// - Defender is able to defend (not using All-Out Attack)
/// - Defense type available (Dodge/Parry/Block)
#[derive(Prop)]
pub struct DefenseRequired;

// ── Action Resolution Sequencing ──────────────────────────────────────────────

/// Proposition: An attack has been fully resolved.
///
/// Requires:
/// - Attack roll made via GameMaster
/// - Success/failure determined
/// - Critical outcomes checked
/// - Evidence tokens minted
#[derive(Prop)]
pub struct AttackResolved;

/// Proposition: A defense has been fully resolved.
///
/// Requires:
/// - Defense roll made via GameMaster
/// - Success/failure determined
/// - Result determines next step (damage or miss)
#[derive(Prop)]
pub struct DefenseResolved;

/// Proposition: Damage can be applied to target.
///
/// Requires:
/// - Attack succeeded
/// - Defense failed or was not available
/// - Damage calculation ready
#[derive(Prop)]
pub struct CanApplyDamage;

/// Proposition: Damage has been applied to target.
///
/// Requires:
/// - Damage rolled and calculated via GameMaster
/// - HP/injury updated
/// - Incapacitation checked
#[derive(Prop)]
pub struct DamageApplied;

// ── Bridges from GameMaster Mechanics ─────────────────────────────────────────

// These ProvableFrom implementations will connect GameMaster evidence
// to combat flow propositions. They go in contracts/credentials.rs
// using proof_credential! macro.
//
// Examples:
// - AttackSuccessful -> DefenseRequired
// - DefenseFailed -> CanApplyDamage
// - InjuryApplied -> DamageApplied
// - CombatantIncapacitated -> VictoryConditionMet

// ── Future Extensions ─────────────────────────────────────────────────────────

// Additional propositions for more complex combat:
// - MovementCompleted (for tactical positioning)
// - ConcentrationMaintained (for ongoing spells)
// - StatusEffectApplied (stun, poison, etc.)
// - TeamInitiativeRolled (for group combat)
// - SurpriseRoundActive (for ambushes)
