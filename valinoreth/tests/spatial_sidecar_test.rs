use elicit_gis::{
    GeometryCollectionDescriptor, GeometryCollectionValid, GisError, GisErrorKind, GisResult,
    LineStringDescriptor, LineStringValid, LinearRingDescriptor, LinearRingValid,
    MultiGeometryDescriptor, MultiLineStringValid, MultiPointValid, MultiPolygonValid,
    PointDescriptor, PointValid, PolygonDescriptor, PolygonValid, SfsCoordinate, SfsCoordinate3D,
    SfsGeometryFactory, SfsGeometryMeta, SfsGeometryValid,
};
use elicitation::Established;
use valinoreth::{
    apply_completed_movement, attack_range_limit, attack_reach_from_reach, build_tactical_point,
    check_line_of_effect_between_combatants, check_target_within_range, check_target_within_reach,
    combatant_occupancy_from_placement, complete_movement, declare_movement,
    declared_attack_target, distance_between_combatants, distance_between_placements,
    establish_movement_within_budget, establish_spatial_state, initial_spatial_state_for_combat,
    local_tactical_frame, movement_budget_from_effective_move, step_budget_from_effective_move,
    tactical_footprint, tactical_frame_from_source, validate_movement_path, CombatPhase,
    CombatSpatialStateBuilder, CombatState, CombatStateView, CombatantPlacementBuilder,
    CombatantSlot, CombatantState, MovementErrorKind, Reach, SpatialErrorKind,
    TacticalCoordinateBuilder, TacticalFrame, TacticalObstacleBuilder, TacticalTerrainZoneBuilder,
    GURPS_STEP_MINIMUM_METERS,
};

#[derive(Debug)]
struct TestSfsFactory;

impl TestSfsFactory {
    fn unsupported(operation: &str) -> GisError {
        GisError::new(GisErrorKind::Unsupported(operation.to_string()))
    }
}

impl SfsGeometryFactory for TestSfsFactory {
    fn build_point(
        &self,
        coord: SfsCoordinate,
        srid: Option<i32>,
    ) -> GisResult<(PointDescriptor, Established<PointValid>)> {
        if !coord.x.is_finite() || !coord.y.is_finite() {
            return Err(GisError::new(GisErrorKind::InvalidGeometry(
                "point coordinates must be finite".to_string(),
            )));
        }

        Ok((
            PointDescriptor {
                x: Some(coord.x),
                y: Some(coord.y),
                z: None,
                m: None,
                srid,
            },
            Established::assert(),
        ))
    }

    fn build_point_3d(
        &self,
        coord: SfsCoordinate3D,
        srid: Option<i32>,
    ) -> GisResult<(PointDescriptor, Established<PointValid>)> {
        if !coord.x.is_finite() || !coord.y.is_finite() || !coord.z.is_finite() {
            return Err(GisError::new(GisErrorKind::InvalidGeometry(
                "point coordinates must be finite".to_string(),
            )));
        }

        Ok((
            PointDescriptor {
                x: Some(coord.x),
                y: Some(coord.y),
                z: Some(coord.z),
                m: None,
                srid,
            },
            Established::assert(),
        ))
    }

    fn build_point_empty(&self, srid: Option<i32>) -> (PointDescriptor, Established<PointValid>) {
        (
            PointDescriptor {
                x: None,
                y: None,
                z: None,
                m: None,
                srid,
            },
            Established::assert(),
        )
    }

    fn build_line_string(
        &self,
        _coords: Vec<SfsCoordinate>,
        _srid: Option<i32>,
    ) -> GisResult<(LineStringDescriptor, Established<LineStringValid>)> {
        Err(Self::unsupported("build_line_string"))
    }

    fn build_linear_ring(
        &self,
        _coords: Vec<SfsCoordinate>,
        _srid: Option<i32>,
    ) -> GisResult<(LinearRingDescriptor, Established<LinearRingValid>)> {
        Err(Self::unsupported("build_linear_ring"))
    }

    fn build_polygon(
        &self,
        _exterior: Established<LinearRingValid>,
        _holes: Vec<Established<LinearRingValid>>,
    ) -> GisResult<(PolygonDescriptor, Established<PolygonValid>)> {
        Err(Self::unsupported("build_polygon"))
    }

    fn build_multi_point(
        &self,
        _points: Vec<Established<PointValid>>,
    ) -> GisResult<(MultiGeometryDescriptor, Established<MultiPointValid>)> {
        Err(Self::unsupported("build_multi_point"))
    }

    fn build_multi_line_string(
        &self,
        _lines: Vec<Established<LineStringValid>>,
    ) -> GisResult<(MultiGeometryDescriptor, Established<MultiLineStringValid>)> {
        Err(Self::unsupported("build_multi_line_string"))
    }

    fn build_multi_polygon(
        &self,
        _polygons: Vec<Established<PolygonValid>>,
    ) -> GisResult<(MultiGeometryDescriptor, Established<MultiPolygonValid>)> {
        Err(Self::unsupported("build_multi_polygon"))
    }

    fn build_geometry_collection(
        &self,
        _geoms: Vec<Established<SfsGeometryValid>>,
    ) -> GisResult<(
        GeometryCollectionDescriptor,
        Established<GeometryCollectionValid>,
    )> {
        Err(Self::unsupported("build_geometry_collection"))
    }

    fn geometry_from_wkt(
        &self,
        _wkt: &str,
    ) -> GisResult<(Box<dyn SfsGeometryMeta>, Established<SfsGeometryValid>)> {
        Err(Self::unsupported("geometry_from_wkt"))
    }

    fn geometry_from_wkb(
        &self,
        _wkb: &[u8],
    ) -> GisResult<(Box<dyn SfsGeometryMeta>, Established<SfsGeometryValid>)> {
        Err(Self::unsupported("geometry_from_wkb"))
    }
}

fn active_combat_state(count: usize) -> CombatState {
    let combatants = (0..count)
        .map(|index| CombatantState {
            id: format!("combatant-{index}"),
            team: format!("team-{}", index % 2),
            current_hp: 10,
            max_hp: 10,
            current_fp: 10,
            max_fp: 10,
            basic_speed: 5,
            incapacitated: false,
        })
        .collect();

    CombatState::Active {
        combatants,
        turn_order: (0..count).collect(),
        current_actor: 0,
        round: 1,
    }
}

fn coordinate(x_meters: f64, y_meters: f64) -> valinoreth::TacticalCoordinate {
    TacticalCoordinateBuilder::default()
        .x_meters(x_meters)
        .y_meters(y_meters)
        .build()
        .expect("coordinate builds")
}

fn placement(
    factory: &TestSfsFactory,
    frame: &TacticalFrame,
    slot: usize,
    x_meters: f64,
    y_meters: f64,
) -> valinoreth::CombatantPlacement {
    let point = build_tactical_point(factory, coordinate(x_meters, y_meters), frame.clone())
        .expect("point builds");

    CombatantPlacementBuilder::default()
        .slot(CombatantSlot::new(slot))
        .point(point)
        .build()
        .expect("placement builds")
}

#[test]
fn tactical_point_construction_uses_gis_factory() {
    let factory = TestSfsFactory;
    let frame = local_tactical_frame();

    let point = build_tactical_point(&factory, coordinate(3.0, 4.0), frame.clone())
        .expect("GIS-backed tactical point builds");

    assert_eq!(point.coordinate.x_meters, 3.0);
    assert_eq!(point.coordinate.y_meters, 4.0);
    assert_eq!(point.frame.srid, frame.srid);
    assert_eq!(point.descriptor.x, Some(3.0));
    assert_eq!(point.descriptor.y, Some(4.0));
    assert_eq!(point.descriptor.srid, Some(frame.srid));
}

#[test]
fn valid_spatial_state_establishes_consistency() {
    let factory = TestSfsFactory;
    let frame = local_tactical_frame();
    let combat_state = active_combat_state(2);
    let spatial_state = CombatSpatialStateBuilder::default()
        .frame(frame.clone())
        .placements(vec![
            placement(&factory, &frame, 0, 0.0, 0.0),
            placement(&factory, &frame, 1, 1.0, 0.0),
        ])
        .build()
        .expect("spatial state builds");

    let (established_state, _proof) =
        establish_spatial_state(&combat_state, spatial_state).expect("spatial state is valid");

    assert_eq!(established_state.placements.len(), 2);
}

#[test]
fn missing_spatial_placement_is_rejected() {
    let factory = TestSfsFactory;
    let frame = local_tactical_frame();
    let combat_state = active_combat_state(2);
    let spatial_state = CombatSpatialStateBuilder::default()
        .frame(frame.clone())
        .placements(vec![placement(&factory, &frame, 0, 0.0, 0.0)])
        .build()
        .expect("spatial state builds");

    let error =
        establish_spatial_state(&combat_state, spatial_state).expect_err("slot 1 is missing");

    assert_eq!(error.kind, SpatialErrorKind::MissingPlacement(1));
}

#[test]
fn duplicate_spatial_placement_is_rejected() {
    let factory = TestSfsFactory;
    let frame = local_tactical_frame();
    let combat_state = active_combat_state(2);
    let spatial_state = CombatSpatialStateBuilder::default()
        .frame(frame.clone())
        .placements(vec![
            placement(&factory, &frame, 0, 0.0, 0.0),
            placement(&factory, &frame, 0, 1.0, 0.0),
        ])
        .build()
        .expect("spatial state builds");

    let error =
        establish_spatial_state(&combat_state, spatial_state).expect_err("slot 0 is duplicated");

    assert_eq!(error.kind, SpatialErrorKind::DuplicatePlacement(0));
}

#[test]
fn distance_between_combatants_reports_meters() {
    let factory = TestSfsFactory;
    let frame = local_tactical_frame();
    let combat_state = active_combat_state(2);
    let spatial_state = CombatSpatialStateBuilder::default()
        .frame(frame.clone())
        .placements(vec![
            placement(&factory, &frame, 0, 0.0, 0.0),
            placement(&factory, &frame, 1, 3.0, 4.0),
        ])
        .build()
        .expect("spatial state builds");
    let (spatial_state, spatial_proof) =
        establish_spatial_state(&combat_state, spatial_state).expect("spatial state is valid");

    let (distance, _same_frame) = distance_between_combatants(
        &spatial_state,
        &spatial_proof,
        CombatantSlot::new(0),
        CombatantSlot::new(1),
    )
    .expect("distance is available");

    assert_eq!(distance.from, CombatantSlot::new(0));
    assert_eq!(distance.to, CombatantSlot::new(1));
    assert_eq!(distance.meters, 5.0);
}

#[test]
fn distance_between_same_location_is_zero() {
    let factory = TestSfsFactory;
    let frame = local_tactical_frame();
    let combat_state = active_combat_state(2);
    let spatial_state = CombatSpatialStateBuilder::default()
        .frame(frame.clone())
        .placements(vec![
            placement(&factory, &frame, 0, 2.0, 2.0),
            placement(&factory, &frame, 1, 2.0, 2.0),
        ])
        .build()
        .expect("spatial state builds");
    let (spatial_state, spatial_proof) =
        establish_spatial_state(&combat_state, spatial_state).expect("spatial state is valid");

    let (distance, _same_frame) = distance_between_combatants(
        &spatial_state,
        &spatial_proof,
        CombatantSlot::new(0),
        CombatantSlot::new(1),
    )
    .expect("distance is available");

    assert_eq!(distance.meters, 0.0);
}

#[test]
fn melee_target_within_reach_establishes_spatial_attack_proof() {
    let factory = TestSfsFactory;
    let frame = local_tactical_frame();
    let combat_state = active_combat_state(2);
    let spatial_state = CombatSpatialStateBuilder::default()
        .frame(frame.clone())
        .placements(vec![
            placement(&factory, &frame, 0, 0.0, 0.0),
            placement(&factory, &frame, 1, 1.0, 0.0),
        ])
        .build()
        .expect("spatial state builds");
    let (spatial_state, spatial_proof) =
        establish_spatial_state(&combat_state, spatial_state).expect("spatial state is valid");
    let declared_target = declared_attack_target(
        CombatantSlot::new(0),
        CombatantSlot::new(1),
        Established::assert(),
    );
    let reach = attack_reach_from_reach(Reach::One).expect("reach maps to tactical meters");

    let (checked_target, _within_reach) =
        check_target_within_reach(&spatial_state, &spatial_proof, declared_target, reach)
            .expect("target is within reach");

    assert_eq!(checked_target.declared_target(), declared_target);
    assert_eq!(checked_target.distance.meters, 1.0);
}

#[test]
fn melee_reach_check_uses_the_declared_target_identity() {
    let factory = TestSfsFactory;
    let frame = local_tactical_frame();
    let combat_state = active_combat_state(3);
    let spatial_state = CombatSpatialStateBuilder::default()
        .frame(frame.clone())
        .placements(vec![
            placement(&factory, &frame, 0, 0.0, 0.0),
            placement(&factory, &frame, 1, 3.0, 0.0),
            placement(&factory, &frame, 2, 1.0, 0.0),
        ])
        .build()
        .expect("spatial state builds");
    let (spatial_state, spatial_proof) =
        establish_spatial_state(&combat_state, spatial_state).expect("spatial state is valid");
    let declared_far_target = declared_attack_target(
        CombatantSlot::new(0),
        CombatantSlot::new(1),
        Established::assert(),
    );
    let reach = attack_reach_from_reach(Reach::One).expect("reach maps to tactical meters");

    let error =
        check_target_within_reach(&spatial_state, &spatial_proof, declared_far_target, reach)
            .expect_err("near bystander cannot satisfy the declared target's reach check");

    assert_eq!(
        error.kind,
        SpatialErrorKind::TargetOutOfReach {
            attacker: 0,
            target: 1,
            distance_meters: 3.0,
            min_reach_meters: 1.0,
            max_reach_meters: 1.0,
        }
    );
}

#[test]
fn ranged_target_within_range_establishes_spatial_attack_proof() {
    let factory = TestSfsFactory;
    let frame = local_tactical_frame();
    let combat_state = active_combat_state(2);
    let spatial_state = CombatSpatialStateBuilder::default()
        .frame(frame.clone())
        .placements(vec![
            placement(&factory, &frame, 0, 0.0, 0.0),
            placement(&factory, &frame, 1, 3.0, 4.0),
        ])
        .build()
        .expect("spatial state builds");
    let (spatial_state, spatial_proof) =
        establish_spatial_state(&combat_state, spatial_state).expect("spatial state is valid");
    let declared_target = declared_attack_target(
        CombatantSlot::new(0),
        CombatantSlot::new(1),
        Established::assert(),
    );
    let range = attack_range_limit(5.0).expect("range limit builds");

    let (checked_target, _within_range) =
        check_target_within_range(&spatial_state, &spatial_proof, declared_target, range)
            .expect("target is within range");

    assert_eq!(checked_target.declared_target(), declared_target);
    assert_eq!(checked_target.distance.meters, 5.0);
}

#[test]
fn ranged_target_beyond_range_is_rejected() {
    let factory = TestSfsFactory;
    let frame = local_tactical_frame();
    let combat_state = active_combat_state(2);
    let spatial_state = CombatSpatialStateBuilder::default()
        .frame(frame.clone())
        .placements(vec![
            placement(&factory, &frame, 0, 0.0, 0.0),
            placement(&factory, &frame, 1, 3.0, 4.0),
        ])
        .build()
        .expect("spatial state builds");
    let (spatial_state, spatial_proof) =
        establish_spatial_state(&combat_state, spatial_state).expect("spatial state is valid");
    let declared_target = declared_attack_target(
        CombatantSlot::new(0),
        CombatantSlot::new(1),
        Established::assert(),
    );
    let range = attack_range_limit(4.0).expect("range limit builds");

    let error = check_target_within_range(&spatial_state, &spatial_proof, declared_target, range)
        .expect_err("target is outside range");

    assert_eq!(
        error.kind,
        SpatialErrorKind::TargetOutOfRange {
            attacker: 0,
            target: 1,
            distance_meters: 5.0,
            range_meters: 4.0,
        }
    );
}

#[test]
fn distance_between_placements_rejects_frame_mismatch() {
    let factory = TestSfsFactory;
    let frame = local_tactical_frame();
    let alternate_frame =
        tactical_frame_from_source(4326, 990_002, "alternate tactical frame".to_string())
            .expect("alternate frame builds");

    let first = placement(&factory, &frame, 0, 0.0, 0.0);
    let second = placement(&factory, &alternate_frame, 1, 1.0, 1.0);

    let error = distance_between_placements(&first, &second)
        .expect_err("placements in different frames cannot be compared");

    assert_eq!(
        error.kind,
        SpatialErrorKind::FrameMismatch {
            slot: 1,
            expected_srid: frame.srid,
            actual_srid: alternate_frame.srid,
        }
    );
}

#[test]
fn movement_budget_from_effective_move_uses_full_move() {
    let budget = movement_budget_from_effective_move(5).expect("effective Move creates budget");

    assert_eq!(budget.meters, 5.0);
}

#[test]
fn step_budget_has_one_meter_minimum() {
    let normal_step = step_budget_from_effective_move(5).expect("effective Move creates step");
    let fast_step = step_budget_from_effective_move(20).expect("effective Move creates step");

    assert_eq!(normal_step.meters, GURPS_STEP_MINIMUM_METERS);
    assert_eq!(fast_step.meters, 2.0);
}

#[test]
fn declared_movement_within_budget_completes() {
    let factory = TestSfsFactory;
    let frame = local_tactical_frame();
    let combat_state = active_combat_state(1);
    let spatial_state = CombatSpatialStateBuilder::default()
        .frame(frame.clone())
        .placements(vec![placement(&factory, &frame, 0, 0.0, 0.0)])
        .build()
        .expect("spatial state builds");
    let (spatial_state, spatial_proof) =
        establish_spatial_state(&combat_state, spatial_state).expect("spatial state is valid");
    let destination = build_tactical_point(&factory, coordinate(3.0, 4.0), frame)
        .expect("destination point builds");
    let budget = movement_budget_from_effective_move(5).expect("effective Move creates budget");

    let (intent, movement_declared, _same_frame) = declare_movement(
        &spatial_state,
        &spatial_proof,
        CombatantSlot::new(0),
        destination.clone(),
        budget,
    )
    .expect("movement can be declared");
    let (path, path_valid) =
        validate_movement_path(&intent, &movement_declared, &spatial_state, &spatial_proof)
            .expect("path is valid");
    let within_budget = establish_movement_within_budget(&path, intent.budget, &path_valid)
        .expect("movement is within budget");
    let (completed_destination, _movement_completed) =
        complete_movement(intent, movement_declared, path_valid, within_budget);

    assert_eq!(path.distance.meters, 5.0);
    assert_eq!(completed_destination, destination);
}

#[test]
fn movement_beyond_budget_is_rejected() {
    let factory = TestSfsFactory;
    let frame = local_tactical_frame();
    let combat_state = active_combat_state(1);
    let spatial_state = CombatSpatialStateBuilder::default()
        .frame(frame.clone())
        .placements(vec![placement(&factory, &frame, 0, 0.0, 0.0)])
        .build()
        .expect("spatial state builds");
    let (spatial_state, spatial_proof) =
        establish_spatial_state(&combat_state, spatial_state).expect("spatial state is valid");
    let destination = build_tactical_point(&factory, coordinate(6.0, 8.0), frame)
        .expect("destination point builds");
    let budget = movement_budget_from_effective_move(5).expect("effective Move creates budget");

    let (intent, movement_declared, _same_frame) = declare_movement(
        &spatial_state,
        &spatial_proof,
        CombatantSlot::new(0),
        destination,
        budget,
    )
    .expect("movement can be declared");
    let (path, path_valid) =
        validate_movement_path(&intent, &movement_declared, &spatial_state, &spatial_proof)
            .expect("path is valid");
    let error = establish_movement_within_budget(&path, intent.budget, &path_valid)
        .expect_err("movement exceeds budget");

    assert_eq!(
        error.kind,
        MovementErrorKind::MovementExceedsBudget {
            distance_meters: 10.0,
            budget_meters: 5.0,
        }
    );
}

#[test]
fn explicit_occupancy_is_part_of_established_spatial_state() {
    let factory = TestSfsFactory;
    let frame = local_tactical_frame();
    let combat_state = active_combat_state(2);
    let placements = vec![
        placement(&factory, &frame, 0, 0.0, 0.0),
        placement(&factory, &frame, 1, 1.0, 0.0),
    ];
    let occupancy =
        combatant_occupancy_from_placement(&placements[1], 0.25).expect("occupancy builds");
    let spatial_state = CombatSpatialStateBuilder::default()
        .frame(frame)
        .placements(placements)
        .occupancies(vec![occupancy])
        .build()
        .expect("spatial state builds");

    let (spatial_state, _spatial_proof) =
        establish_spatial_state(&combat_state, spatial_state).expect("spatial state is valid");

    assert_eq!(spatial_state.occupancies.len(), 1);
    assert_eq!(spatial_state.occupancies[0].slot, CombatantSlot::new(1));
}

#[test]
fn movement_path_blocked_by_occupancy_cannot_be_validated() {
    let factory = TestSfsFactory;
    let frame = local_tactical_frame();
    let combat_state = active_combat_state(2);
    let placements = vec![
        placement(&factory, &frame, 0, 0.0, 0.0),
        placement(&factory, &frame, 1, 1.5, 0.0),
    ];
    let occupancy =
        combatant_occupancy_from_placement(&placements[1], 0.25).expect("occupancy builds");
    let spatial_state = CombatSpatialStateBuilder::default()
        .frame(frame.clone())
        .placements(placements)
        .occupancies(vec![occupancy])
        .build()
        .expect("spatial state builds");
    let (spatial_state, spatial_proof) =
        establish_spatial_state(&combat_state, spatial_state).expect("spatial state is valid");
    let destination = build_tactical_point(&factory, coordinate(3.0, 0.0), frame)
        .expect("destination point builds");
    let budget = movement_budget_from_effective_move(5).expect("effective Move creates budget");
    let (intent, movement_declared, _same_frame) = declare_movement(
        &spatial_state,
        &spatial_proof,
        CombatantSlot::new(0),
        destination,
        budget,
    )
    .expect("movement can be declared");

    let error = validate_movement_path(&intent, &movement_declared, &spatial_state, &spatial_proof)
        .expect_err("occupied footprint blocks movement proof");

    assert_eq!(
        error.kind,
        MovementErrorKind::MovementPathBlockedByOccupant {
            actor: 0,
            blocker: 1,
        }
    );
}

#[test]
fn movement_path_blocked_by_obstacle_cannot_be_validated() {
    let factory = TestSfsFactory;
    let frame = local_tactical_frame();
    let combat_state = active_combat_state(1);
    let obstacle_center = build_tactical_point(&factory, coordinate(1.5, 0.0), frame.clone())
        .expect("obstacle point builds");
    let obstacle = TacticalObstacleBuilder::default()
        .id("stone-wall")
        .footprint(tactical_footprint(obstacle_center, 0.25).expect("footprint builds"))
        .blocks_movement(true)
        .blocks_line_of_effect(false)
        .build()
        .expect("obstacle builds");
    let spatial_state = CombatSpatialStateBuilder::default()
        .frame(frame.clone())
        .placements(vec![placement(&factory, &frame, 0, 0.0, 0.0)])
        .obstacles(vec![obstacle])
        .build()
        .expect("spatial state builds");
    let (spatial_state, spatial_proof) =
        establish_spatial_state(&combat_state, spatial_state).expect("spatial state is valid");
    let destination = build_tactical_point(&factory, coordinate(3.0, 0.0), frame)
        .expect("destination point builds");
    let budget = movement_budget_from_effective_move(5).expect("effective Move creates budget");
    let (intent, movement_declared, _same_frame) = declare_movement(
        &spatial_state,
        &spatial_proof,
        CombatantSlot::new(0),
        destination,
        budget,
    )
    .expect("movement can be declared");

    let error = validate_movement_path(&intent, &movement_declared, &spatial_state, &spatial_proof)
        .expect_err("obstacle blocks movement proof");

    assert_eq!(
        error.kind,
        MovementErrorKind::MovementPathBlockedByObstacle {
            obstacle_id: "stone-wall".to_string(),
        }
    );
}

#[test]
fn terrain_modifier_affects_budget_through_movement_path() {
    let factory = TestSfsFactory;
    let frame = local_tactical_frame();
    let combat_state = active_combat_state(1);
    let terrain_center = build_tactical_point(&factory, coordinate(1.0, 0.0), frame.clone())
        .expect("terrain point builds");
    let terrain = TacticalTerrainZoneBuilder::default()
        .id("deep-mud")
        .footprint(tactical_footprint(terrain_center, 0.5).expect("footprint builds"))
        .movement_cost_multiplier(2.0)
        .build()
        .expect("terrain builds");
    let spatial_state = CombatSpatialStateBuilder::default()
        .frame(frame.clone())
        .placements(vec![placement(&factory, &frame, 0, 0.0, 0.0)])
        .terrain(vec![terrain])
        .build()
        .expect("spatial state builds");
    let (spatial_state, spatial_proof) =
        establish_spatial_state(&combat_state, spatial_state).expect("spatial state is valid");
    let destination = build_tactical_point(&factory, coordinate(2.0, 0.0), frame)
        .expect("destination point builds");
    let budget = movement_budget_from_effective_move(2).expect("effective Move creates budget");
    let (intent, movement_declared, _same_frame) = declare_movement(
        &spatial_state,
        &spatial_proof,
        CombatantSlot::new(0),
        destination,
        budget,
    )
    .expect("movement can be declared");
    let (path, path_valid) =
        validate_movement_path(&intent, &movement_declared, &spatial_state, &spatial_proof)
            .expect("terrain does not block path proof");

    let error = establish_movement_within_budget(&path, intent.budget, &path_valid)
        .expect_err("terrain-adjusted cost exceeds budget");

    assert_eq!(path.distance.meters, 2.0);
    assert_eq!(path.cost_meters, 4.0);
    assert_eq!(
        error.kind,
        MovementErrorKind::MovementExceedsBudget {
            distance_meters: 4.0,
            budget_meters: 2.0,
        }
    );
}

#[test]
fn line_of_effect_blocked_by_obstacle_cannot_be_established() {
    let factory = TestSfsFactory;
    let frame = local_tactical_frame();
    let combat_state = active_combat_state(2);
    let obstacle_center = build_tactical_point(&factory, coordinate(1.0, 0.0), frame.clone())
        .expect("obstacle point builds");
    let obstacle = TacticalObstacleBuilder::default()
        .id("closed-door")
        .footprint(tactical_footprint(obstacle_center, 0.25).expect("footprint builds"))
        .blocks_movement(false)
        .blocks_line_of_effect(true)
        .build()
        .expect("obstacle builds");
    let spatial_state = CombatSpatialStateBuilder::default()
        .frame(frame.clone())
        .placements(vec![
            placement(&factory, &frame, 0, 0.0, 0.0),
            placement(&factory, &frame, 1, 2.0, 0.0),
        ])
        .obstacles(vec![obstacle])
        .build()
        .expect("spatial state builds");
    let (spatial_state, spatial_proof) =
        establish_spatial_state(&combat_state, spatial_state).expect("spatial state is valid");

    let error = check_line_of_effect_between_combatants(
        &spatial_state,
        &spatial_proof,
        CombatantSlot::new(0),
        CombatantSlot::new(1),
    )
    .expect_err("line of effect is blocked");

    assert_eq!(
        error.kind,
        SpatialErrorKind::LineOfEffectBlockedByObstacle {
            obstacle_id: "closed-door".to_string(),
        }
    );
}

#[test]
fn default_spatial_state_places_each_active_combatant() {
    let combat_state = active_combat_state(3);

    let (spatial_state, _spatial_proof) =
        initial_spatial_state_for_combat(&combat_state).expect("default spatial state builds");

    assert_eq!(spatial_state.placements.len(), 3);
    assert_eq!(spatial_state.placements[0].point.coordinate.x_meters, 0.0);
    assert_eq!(spatial_state.placements[1].point.coordinate.x_meters, 1.0);
    assert_eq!(spatial_state.placements[2].point.coordinate.x_meters, 2.0);
}

#[test]
fn completed_movement_updates_spatial_state() {
    let factory = TestSfsFactory;
    let frame = local_tactical_frame();
    let combat_state = active_combat_state(1);
    let spatial_state = CombatSpatialStateBuilder::default()
        .frame(frame.clone())
        .placements(vec![placement(&factory, &frame, 0, 0.0, 0.0)])
        .build()
        .expect("spatial state builds");
    let (spatial_state, spatial_proof) =
        establish_spatial_state(&combat_state, spatial_state).expect("spatial state is valid");
    let destination = build_tactical_point(&factory, coordinate(2.0, 0.0), frame)
        .expect("destination point builds");
    let budget = movement_budget_from_effective_move(5).expect("effective Move creates budget");
    let (intent, movement_declared, _same_frame) = declare_movement(
        &spatial_state,
        &spatial_proof,
        CombatantSlot::new(0),
        destination,
        budget,
    )
    .expect("movement can be declared");
    let (path, path_valid) =
        validate_movement_path(&intent, &movement_declared, &spatial_state, &spatial_proof)
            .expect("path is valid");
    let within_budget = establish_movement_within_budget(&path, intent.budget, &path_valid)
        .expect("movement is within budget");
    let (completed_destination, movement_completed) =
        complete_movement(intent, movement_declared, path_valid, within_budget);

    let (updated_state, _updated_proof) = apply_completed_movement(
        &combat_state,
        &spatial_state,
        CombatantSlot::new(0),
        completed_destination,
        movement_completed,
    )
    .expect("completed movement updates sidecar");

    assert_eq!(updated_state.placements[0].point.coordinate.x_meters, 2.0);
}

#[test]
fn combat_state_view_exposes_visible_positions() {
    let combat_state = active_combat_state(2);
    let (spatial_state, _spatial_proof) =
        initial_spatial_state_for_combat(&combat_state).expect("default spatial state builds");
    let phase = CombatPhase::Active {
        combat: combat_state,
        spatial: Some(spatial_state),
    };

    let view = CombatStateView::from_phase(&phase, "combatant-0");

    assert_eq!(view.combatants.len(), 2);
    assert_eq!(
        view.combatants[0]
            .position
            .as_ref()
            .expect("combatant 0 has position")
            .x_meters,
        0.0
    );
    assert_eq!(
        view.combatants[1]
            .position
            .as_ref()
            .expect("combatant 1 has position")
            .x_meters,
        1.0
    );
    assert_eq!(view.combatants[0].distance_from_viewer_meters, Some(0.0));
    assert_eq!(view.combatants[1].distance_from_viewer_meters, Some(1.0));
    assert!(view.to_preamble().contains("[1.0 m from you]"));
}
