use derive_builder::Builder;
use elicitation::contracts::{Established, ProvableFrom};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::spatial::{
    movement_blocking_obstacle_for_segment, movement_blocking_occupant_for_segment,
    movement_cost_meters_for_segment, TacticalFootprintBuilder,
};
use crate::{
    distance_between_placements, establish_same_frame, establish_spatial_state, locate_combatant,
    CombatSpatialState, CombatSpatialStateBuilder, CombatantPlacement, CombatantPlacementBuilder,
    CombatantSlot, LocationsHaveSameFrame, MovementCompleted, MovementDeclared, MovementPathValid,
    MovementWithinBudget, SpatialError, SpatialStateConsistent, TacticalDistance, TacticalPoint,
    TacticalPointBuilder,
};

/// Combat time occurs second by second. BS 362
/// Multiple partipants experience combat as overlapping seconds
/// Because they take actions in a turn order. BS 363
/// Turn order is by highest basic speed.
/// Ties go to highest DX (or maybe highest effective skill) BS 363
///
/// Maximum move is the characters full Move score.
/// A "step" is 1/10 of Move, minimum one meter. BS 363
pub enum Manuever {
    /// Full-turn maneuver.
    /// Bracing a ranged weapon adds +1 to Acc, meaning resting a sandbag, low wall, etc.  A
    /// one-handed firearm is considered braced if used two-handed.  Two-handed weapons are braced
    /// when used prone, or with bi-pod.
    /// +1 for two seconds of Aim, +2 for three or more seconds.
    /// Combined bonus cannot exceed weapons base Accuracy.
    /// Cannot take a "step" with a Braced two-handed weapon.
    /// Active defense spoils an Aim bonus.
    /// If injured, make a Will roll or lose your Aim. BS 364
    Aim,
    /// No active defense.  No dodge, parry or block.
    /// Move up to half Move, only forward.
    AllOutAttack,
    /// Defensive maneuver with bonuses. BS 366
    AllOutDefense,
    /// Armed or unarmed attack against an opponent.  Weapons must be ready and targets must be in reach.
    /// May step and attack or attack then step.
    Attack,
    /// Change between postures (standing, kneeling, prone, etc.). BS 364
    ChangePosture,
    /// Focus on a mental task or maintain a spell. BS 366
    Concentrate,
    /// May attempt a HT roll to recover from physical stun or IQ roll to recover from mental stun,
    /// recovering at the end of the turn. BS 364
    DoNothing,
    /// Melee equivalent of Aim.
    /// +1 per turn (max +3) on an Attack, Feint, All-Out Attack, or Move and Attack against a
    /// specific opponent on the subsequent turn. BS 364/365
    Evaluate,
    /// Feint: BS 365
    /// Roll a Quick Contest of Melee Weaon skills.  May use an unarmed skill, Cloak, Shield or DX
    /// if the skill level is higher.
    ///
    /// * Failure - Faint fails.
    /// * Foe succeeds by as much as player - Faint fails.
    /// * When foe fails, subtract margin of success [`Success::Margin`] from ensuing attack.
    /// * When foe succeeds by [`Success::Margin`] less than player, subtract the difference in
    ///   margin from the ensuing attack.
    ///
    /// * Applies to both attacks in an All-Out Attack (Double)
    /// * After striking with your shield, you may also Feint with your shield using the Shield
    ///   skill.
    /// * Step movement.
    /// * Parrying with an unbalanced weapon makes the weapon unready on the next turn, wasting the
    ///   feint.
    Feint,
    /// Move any number of meters up to your full Move score.
    /// Mounted or vehicle movement counts for full controlled movement.
    /// Sprinting grants a bonus movement on the second and later moves.
    /// No other action but Free Actions. BS 364
    Move,
    /// Move and attack at -4 or -2 for ranged weapons. BS 365
    MoveAndAttack,
    /// Ready a weapon, reload, or draw/holster. BS 366
    Ready,
    /// Wait for a trigger condition, then act. BS 366
    Wait,
}

/// All-Out Attack options for melee attack. BS 365
pub enum AllOutMeleeAttack {
    /// Make a single attack at +4 to hit.
    Determined,
    /// Make two attacks against the same foe.
    ///
    /// * Requires two ready weapons or a weapon that does not need to be readied after use.
    /// * Off-hand weapons are still at -4 per Handedness unless ambidextrous.
    Double,
    /// Make one Feint and one attack, receiving the immediate bonus from the feint.
    Feint,
    /// +2 to damage or +1 damage per die, if greater.
    ///
    /// * Applies to ST-based thrust or swing damage.
    Strong,
}

/// All-Out Attack options for ranged attack. BS 365
pub enum AllOutRangedAttack {
    /// Make a single attack at +1 to hit.
    Determined,
    /// Full-turn maneuver.
    /// Weapon RoF 5+ required.
    SuppressionFire,
}

/// Success roll types.
///
/// # GURPS Rules
///
/// Success can be measured as a simple pass/fail or by margin of success.
/// Margin affects contest outcomes and special maneuvers like Feint.
///
/// # Citations
///
/// BS 348-349 - Success rolls and margins
pub enum Success {
    /// A skill check against a set level or threshold.
    Check,
    /// The margin by which a roll succeeds a set threshold.
    Margin,
}

/// Actions that can be performed without using a maneuver.
///
/// # GURPS Rules
///
/// Free actions don't count as maneuvers and can be done
/// during any turn without penalty.
///
/// # Citations
///
/// BS 363 - Free actions
pub enum FreeAction {
    /// Speak a sentence or two. BS 363
    Talk,
    /// Maintain an active spell. BS 363
    MaintainSpell,
    /// Drop a held item. BS 363
    DropItem,
    /// Crouch for cover. BS 363
    Crouch,
}

/// Character posture affecting combat and movement.
///
/// # GURPS Rules
///
/// Posture changes require Change Posture maneuvers:
/// - Lying → Crawling/Kneeling/Sitting: 1 maneuver
/// - Crawling/Kneeling/Sitting → Standing: 1 maneuver
/// - Standing → Lying: 1 maneuver
/// - Kneeling ↔ Standing: Free as part of a step
/// - Crouching: Free action
///
/// # Citations
///
/// BS 364 - Posture rules
pub enum Posture {
    /// Standing upright, no penalties. BS 364
    Standing,
    /// Sitting, cannot attack or defend well. BS 364
    Sitting,
    /// Kneeling, -2 to melee defense. BS 364
    Kneeling,
    /// Crawling, Move at 1/3 speed. BS 364
    Crawling,
    /// Lying face up, -4 to attack and defend. BS 364
    LyingProne,
    /// Lying face down, -4 to attack and defend. BS 364
    LyingFaceDown,
}

/// Minimum tactical step distance in meters. BS 363.
pub const GURPS_STEP_MINIMUM_METERS: f64 = 1.0;

/// Result type for tactical movement operations.
pub type MovementResult<T> = Result<T, MovementError>;

/// Error kind for tactical movement validation.
#[derive(Debug, Clone, PartialEq, derive_more::Display)]
pub enum MovementErrorKind {
    /// Movement budget was negative or not finite.
    #[display("invalid movement budget: {} meters", _0)]
    InvalidBudget(f64),

    /// Effective Move cannot produce a movement budget.
    #[display("invalid effective Move score: {}", _0)]
    InvalidEffectiveMove(usize),

    /// Declared movement exceeds the available movement budget.
    #[display(
        "movement distance {} meters exceeds budget {} meters",
        distance_meters,
        budget_meters
    )]
    MovementExceedsBudget {
        /// Movement path distance in meters.
        distance_meters: f64,
        /// Available movement budget in meters.
        budget_meters: f64,
    },

    /// The movement intent no longer matches the actor's established start location.
    #[display(
        "movement start for actor slot {} does not match the established spatial sidecar",
        actor
    )]
    MovementStartMismatch {
        /// Moving combatant slot.
        actor: usize,
    },

    /// A movement path intersects another combatant's occupied footprint.
    #[display(
        "movement path for actor slot {} is blocked by occupied footprint for slot {}",
        actor,
        blocker
    )]
    MovementPathBlockedByOccupant {
        /// Moving combatant slot.
        actor: usize,
        /// Blocking combatant slot.
        blocker: usize,
    },

    /// A movement path intersects a movement-blocking obstacle.
    #[display("movement path is blocked by obstacle {}", obstacle_id)]
    MovementPathBlockedByObstacle {
        /// Blocking obstacle identifier.
        obstacle_id: String,
    },

    /// Movement cost after terrain modifiers is invalid.
    #[display("invalid movement cost: {} meters", cost_meters)]
    InvalidMovementCost {
        /// Invalid movement cost in meters.
        cost_meters: f64,
    },

    /// Spatial validation failed.
    #[display("spatial validation failed: {}", _0)]
    Spatial(String),

    /// Builder validation failed.
    #[display("movement builder failed: {}", _0)]
    Builder(String),
}

/// Errors that can occur while establishing tactical movement facts.
#[derive(Debug, Clone, derive_more::Display, derive_more::Error)]
#[display("Movement error: {} at {}:{}", kind, file, line)]
pub struct MovementError {
    /// The specific error kind.
    pub kind: MovementErrorKind,
    /// Source file where the error was created.
    pub file: &'static str,
    /// Source line where the error was created.
    pub line: u32,
}

impl MovementError {
    /// Create a movement error with call-site tracking.
    #[track_caller]
    #[instrument]
    pub fn new(kind: MovementErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind,
            file: loc.file(),
            line: loc.line(),
        }
    }
}

impl From<SpatialError> for MovementError {
    #[track_caller]
    fn from(error: SpatialError) -> Self {
        Self::new(MovementErrorKind::Spatial(error.to_string()))
    }
}

/// Movement budget in tactical meters for one maneuver.
#[derive(
    Debug, Clone, Copy, PartialEq, PartialOrd, Builder, Serialize, Deserialize, JsonSchema,
)]
#[builder(setter(into))]
pub struct MovementBudget {
    /// Meters available for this movement declaration.
    pub meters: f64,
}

/// Declared point-to-point movement for a combatant.
#[derive(Debug, Clone, PartialEq, Builder, Serialize, Deserialize, JsonSchema)]
#[builder(setter(into))]
pub struct MovementIntent {
    /// Actor whose movement is being declared.
    pub actor: CombatantSlot,

    /// Actor's established start location.
    pub start: TacticalPoint,

    /// Declared destination location.
    pub destination: TacticalPoint,

    /// Movement budget available for this maneuver.
    pub budget: MovementBudget,
}

/// Validated tactical movement path for the first point-to-point movement slice.
#[derive(Debug, Clone, PartialEq, Builder, Serialize, Deserialize, JsonSchema)]
#[builder(setter(into))]
pub struct MovementPath {
    /// Actor whose path was checked.
    pub actor: CombatantSlot,

    /// Path start location.
    pub start: TacticalPoint,

    /// Path destination location.
    pub destination: TacticalPoint,

    /// Direct tactical distance in meters.
    pub distance: TacticalDistance,

    /// Tactical movement cost after terrain modifiers.
    pub cost_meters: f64,
}

struct MovementDeclaredChecked;

impl ProvableFrom<MovementDeclaredChecked> for MovementDeclared {}

struct MovementPathChecked;

impl ProvableFrom<MovementPathChecked> for MovementPathValid {}

struct MovementBudgetChecked;

impl ProvableFrom<MovementBudgetChecked> for MovementWithinBudget {}

struct MovementCompletedChecked;

impl ProvableFrom<MovementCompletedChecked> for MovementCompleted {}

fn validate_budget(budget: MovementBudget) -> MovementResult<()> {
    if !budget.meters.is_finite() || budget.meters < 0.0 {
        return Err(MovementError::new(MovementErrorKind::InvalidBudget(
            budget.meters,
        )));
    }

    Ok(())
}

fn destination_placement(
    actor: CombatantSlot,
    destination: TacticalPoint,
) -> MovementResult<CombatantPlacement> {
    CombatantPlacementBuilder::default()
        .slot(actor)
        .point(destination)
        .build()
        .map_err(|error| MovementError::new(MovementErrorKind::Builder(error.to_string())))
}

/// Build a full-move budget from an effective Move score.
#[instrument]
pub fn movement_budget_from_effective_move(
    effective_move: usize,
) -> MovementResult<MovementBudget> {
    if effective_move == 0 {
        return Err(MovementError::new(MovementErrorKind::InvalidEffectiveMove(
            effective_move,
        )));
    }

    MovementBudgetBuilder::default()
        .meters(effective_move as f64)
        .build()
        .map_err(|error| MovementError::new(MovementErrorKind::Builder(error.to_string())))
}

/// Build a GURPS step budget from effective Move, minimum one meter. BS 363.
#[instrument]
pub fn step_budget_from_effective_move(effective_move: usize) -> MovementResult<MovementBudget> {
    if effective_move == 0 {
        return Err(MovementError::new(MovementErrorKind::InvalidEffectiveMove(
            effective_move,
        )));
    }

    MovementBudgetBuilder::default()
        .meters(((effective_move as f64) / 10.0).max(GURPS_STEP_MINIMUM_METERS))
        .build()
        .map_err(|error| MovementError::new(MovementErrorKind::Builder(error.to_string())))
}

/// Declare movement for an actor in an established spatial sidecar.
#[instrument(skip(spatial_state, spatial_proof, destination), fields(actor = actor.index()))]
pub fn declare_movement(
    spatial_state: &CombatSpatialState,
    spatial_proof: &Established<SpatialStateConsistent>,
    actor: CombatantSlot,
    destination: TacticalPoint,
    budget: MovementBudget,
) -> MovementResult<(
    MovementIntent,
    Established<MovementDeclared>,
    Established<LocationsHaveSameFrame>,
)> {
    validate_budget(budget)?;

    let (start_placement, _located) = locate_combatant(spatial_state, spatial_proof, actor)?;
    let destination_placement = destination_placement(actor, destination.clone())?;
    let same_frame = establish_same_frame(start_placement, &destination_placement)?;
    let intent = MovementIntentBuilder::default()
        .actor(actor)
        .start(start_placement.point.clone())
        .destination(destination)
        .budget(budget)
        .build()
        .map_err(|error| MovementError::new(MovementErrorKind::Builder(error.to_string())))?;

    Ok((
        intent,
        Established::<MovementDeclared>::prove(&MovementDeclaredChecked),
        same_frame,
    ))
}

/// Validate the direct point-to-point movement path for a declared movement.
#[instrument(
    skip(intent, _movement_declared, spatial_state, spatial_proof),
    fields(actor = intent.actor.index())
)]
pub fn validate_movement_path(
    intent: &MovementIntent,
    _movement_declared: &Established<MovementDeclared>,
    spatial_state: &CombatSpatialState,
    spatial_proof: &Established<SpatialStateConsistent>,
) -> MovementResult<(MovementPath, Established<MovementPathValid>)> {
    let (established_start, _located) =
        locate_combatant(spatial_state, spatial_proof, intent.actor)?;
    if established_start.point != intent.start {
        return Err(MovementError::new(
            MovementErrorKind::MovementStartMismatch {
                actor: intent.actor.index(),
            },
        ));
    }

    let start = destination_placement(intent.actor, intent.start.clone())?;
    let destination = destination_placement(intent.actor, intent.destination.clone())?;
    let (distance, _same_frame) = distance_between_placements(&start, &destination)?;
    let start_coordinate = start.point.coordinate;
    let destination_coordinate = destination.point.coordinate;

    if let Some(blocker) = movement_blocking_occupant_for_segment(
        spatial_state,
        intent.actor,
        start_coordinate,
        destination_coordinate,
    ) {
        return Err(MovementError::new(
            MovementErrorKind::MovementPathBlockedByOccupant {
                actor: intent.actor.index(),
                blocker: blocker.index(),
            },
        ));
    }

    if let Some(obstacle) = movement_blocking_obstacle_for_segment(
        spatial_state,
        start_coordinate,
        destination_coordinate,
    ) {
        return Err(MovementError::new(
            MovementErrorKind::MovementPathBlockedByObstacle {
                obstacle_id: obstacle.id.clone(),
            },
        ));
    }

    let cost_meters = movement_cost_meters_for_segment(
        spatial_state,
        start_coordinate,
        destination_coordinate,
        distance.meters,
    );
    if !cost_meters.is_finite() || cost_meters < 0.0 {
        return Err(MovementError::new(MovementErrorKind::InvalidMovementCost {
            cost_meters,
        }));
    }

    let path = MovementPathBuilder::default()
        .actor(intent.actor)
        .start(intent.start.clone())
        .destination(intent.destination.clone())
        .distance(distance)
        .cost_meters(cost_meters)
        .build()
        .map_err(|error| MovementError::new(MovementErrorKind::Builder(error.to_string())))?;

    Ok((
        path,
        Established::<MovementPathValid>::prove(&MovementPathChecked),
    ))
}

/// Establish that a movement path fits within the declared budget.
#[instrument(skip(path, _path_valid), fields(actor = path.actor.index()))]
pub fn establish_movement_within_budget(
    path: &MovementPath,
    budget: MovementBudget,
    _path_valid: &Established<MovementPathValid>,
) -> MovementResult<Established<MovementWithinBudget>> {
    validate_budget(budget)?;

    if path.cost_meters > budget.meters {
        return Err(MovementError::new(
            MovementErrorKind::MovementExceedsBudget {
                distance_meters: path.cost_meters,
                budget_meters: budget.meters,
            },
        ));
    }

    Ok(Established::<MovementWithinBudget>::prove(
        &MovementBudgetChecked,
    ))
}

/// Complete a declared movement after path and budget proofs have been established.
#[instrument(skip(intent, _movement_declared, _path_valid, _within_budget), fields(actor = intent.actor.index()))]
pub fn complete_movement(
    intent: MovementIntent,
    _movement_declared: Established<MovementDeclared>,
    _path_valid: Established<MovementPathValid>,
    _within_budget: Established<MovementWithinBudget>,
) -> (TacticalPoint, Established<MovementCompleted>) {
    (
        intent.destination,
        Established::<MovementCompleted>::prove(&MovementCompletedChecked),
    )
}

/// Apply a completed movement to the spatial sidecar and re-establish consistency.
#[instrument(skip(combat_state, spatial_state, destination, _movement_completed), fields(actor = actor.index()))]
pub fn apply_completed_movement(
    combat_state: &crate::CombatState,
    spatial_state: &CombatSpatialState,
    actor: CombatantSlot,
    destination: TacticalPoint,
    _movement_completed: Established<MovementCompleted>,
) -> MovementResult<(CombatSpatialState, Established<SpatialStateConsistent>)> {
    let mut placements = spatial_state.placements.clone();
    let placement = placements
        .iter_mut()
        .find(|placement| placement.slot == actor)
        .ok_or_else(|| {
            MovementError::from(crate::SpatialError::new(
                crate::SpatialErrorKind::MissingPlacement(actor.index()),
            ))
        })?;

    let destination = TacticalPointBuilder::default()
        .coordinate(destination.coordinate)
        .frame(destination.frame)
        .descriptor(destination.descriptor)
        .point_valid(destination.point_valid)
        .build()
        .map_err(|error| MovementError::new(MovementErrorKind::Builder(error.to_string())))?;
    placement.point = destination.clone();

    let mut occupancies = spatial_state.occupancies.clone();
    for occupancy in &mut occupancies {
        if occupancy.slot == actor {
            occupancy.footprint = TacticalFootprintBuilder::default()
                .center(destination.clone())
                .radius_meters(occupancy.footprint.radius_meters)
                .build()
                .map_err(|error| {
                    MovementError::new(MovementErrorKind::Builder(error.to_string()))
                })?;
        }
    }

    let updated = CombatSpatialStateBuilder::default()
        .frame(spatial_state.frame.clone())
        .placements(placements)
        .occupancies(occupancies)
        .obstacles(spatial_state.obstacles.clone())
        .terrain(spatial_state.terrain.clone())
        .build()
        .map_err(|error| MovementError::new(MovementErrorKind::Builder(error.to_string())))?;

    establish_spatial_state(combat_state, updated).map_err(Into::into)
}
