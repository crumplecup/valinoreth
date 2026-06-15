//! Tactical spatial propositions for combat movement.
//!
//! These contracts describe Valinoreth game facts layered above GIS facts from
//! `elicit_gis`. A GIS point-validity proof says that a point is geometrically
//! valid; these propositions say how that point may participate in a combat
//! encounter.

use elicitation::Prop;

/// Proposition: the spatial sidecar matches the active combat encounter.
///
/// Requires every active combatant to have exactly one placement, every
/// placement to reference a valid combatant slot, and every placement to be in
/// the encounter's tactical frame.
#[derive(Prop)]
pub struct SpatialStateConsistent;

/// Proposition: a combatant has a validated tactical location.
///
/// Requires the combatant slot to be present in the active encounter and paired
/// with a GIS-valid tactical point.
#[derive(Prop)]
pub struct CombatantLocated;

/// Proposition: compared locations use the same tactical coordinate frame.
///
/// Requires both locations to identify the same local metric frame before game
/// movement, reach, or range rules consume their coordinates.
#[derive(Prop)]
pub struct LocationsHaveSameFrame;

/// Proposition: a straight tactical line of effect is unobstructed.
///
/// Requires the line segment between two established combatant locations to be
/// checked against map features that block line of effect.
#[derive(Prop)]
pub struct LineOfEffectClear;

/// Proposition: a declared melee attack target is inside the attacker's reach.
///
/// Requires the spatial check to be performed against the same
/// `DeclaredAttackTarget` sidecar that the combat VSM will consume.
#[derive(Prop)]
pub struct TargetWithinReach;

/// Proposition: a declared ranged attack target is inside the attack's range.
///
/// Requires the spatial check to be performed against the same
/// `DeclaredAttackTarget` sidecar that the combat VSM will consume.
#[derive(Prop)]
pub struct TargetWithinRange;
