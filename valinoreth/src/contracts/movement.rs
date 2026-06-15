//! Movement propositions for tactical combat.
//!
//! These contracts describe Valinoreth movement facts layered above tactical
//! spatial facts. Spatial contracts establish where combatants are and how far
//! apart points are; movement contracts establish that a declared move is legal
//! for the selected maneuver and movement budget.

use elicitation::Prop;

/// Proposition: a movement intent has been declared.
///
/// Requires an actor, current tactical start location, destination, and movement
/// budget in the same tactical frame.
#[derive(Prop)]
pub struct MovementDeclared;

/// Proposition: a declared movement path is valid.
///
/// The first implementation treats movement as a direct point-to-point tactical
/// path. Future terrain, facing, and obstacle rules should narrow this gate
/// rather than minting this proof somewhere else.
#[derive(Prop)]
pub struct MovementPathValid;

/// Proposition: a movement path fits inside the actor's movement budget.
///
/// Requires the path distance in tactical meters to be less than or equal to
/// the budget derived from effective Move.
#[derive(Prop)]
pub struct MovementWithinBudget;

/// Proposition: a declared legal movement has completed.
///
/// Requires the movement declaration, path validity, and budget proof tokens.
#[derive(Prop)]
pub struct MovementCompleted;
