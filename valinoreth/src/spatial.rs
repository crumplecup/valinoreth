//! Tactical spatial sidecar state for combat movement.
//!
//! Valinoreth owns the game semantics in this module while `elicit_gis` owns
//! the GIS validity proof for the underlying point geometry.

use derive_builder::Builder;
use elicit_gis::{
    GeometryCollectionDescriptor, GeometryCollectionValid, GisError, GisErrorKind, GisResult,
    LineStringDescriptor, LineStringValid, LinearRingDescriptor, LinearRingValid,
    MultiGeometryDescriptor, MultiLineStringValid, MultiPointValid, MultiPolygonValid,
    PointDescriptor, PointValid, PolygonDescriptor, PolygonValid, SfsCoordinate, SfsCoordinate3D,
    SfsGeometryFactory, SfsGeometryMeta, SfsGeometryValid,
};
use elicitation::contracts::{Established, ProvableFrom};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::{
    CombatState, CombatantLocated, CombatantSlot, DeclaredAttackTarget, LineOfEffectClear,
    LocationsHaveSameFrame, Reach, SpatialStateConsistent, TargetWithinRange, TargetWithinReach,
};

/// EPSG SRID for WGS84 source coordinates.
pub const WGS84_SRID: i32 = 4326;

/// Local engineering SRID reserved for Valinoreth tactical encounter meters.
pub const VALINORETH_TACTICAL_SRID: i32 = 990_001;

/// Result type for tactical spatial operations.
pub type SpatialResult<T> = Result<T, SpatialError>;

/// Error kind for tactical spatial validation.
#[derive(Debug, Clone, PartialEq, derive_more::Display)]
pub enum SpatialErrorKind {
    /// The combat state is not active.
    #[display("combat state is not active")]
    CombatNotActive,

    /// A combatant slot appears more than once in the spatial sidecar.
    #[display("duplicate placement for combatant slot {}", _0)]
    DuplicatePlacement(usize),

    /// A combatant slot has no placement in the spatial sidecar.
    #[display("missing placement for combatant slot {}", _0)]
    MissingPlacement(usize),

    /// A placement references a slot outside the active combatant list.
    #[display("unknown placement for combatant slot {}", _0)]
    UnknownPlacement(usize),

    /// An occupancy references a slot outside the active combatant list.
    #[display("unknown occupancy for combatant slot {}", _0)]
    UnknownOccupancy(usize),

    /// A placement frame differs from the encounter frame.
    #[display(
        "frame mismatch for combatant slot {}: expected SRID {}, got SRID {}",
        slot,
        expected_srid,
        actual_srid
    )]
    FrameMismatch {
        /// Combatant slot whose frame differed.
        slot: usize,
        /// Expected tactical frame SRID.
        expected_srid: i32,
        /// Actual placement frame SRID.
        actual_srid: i32,
    },

    /// A GIS point descriptor differs from the Valinoreth tactical frame.
    #[display(
        "GIS point SRID mismatch for combatant slot {}: expected {:?}, got {:?}",
        slot,
        expected_srid,
        actual_srid
    )]
    PointSridMismatch {
        /// Combatant slot whose GIS descriptor differed.
        slot: usize,
        /// Expected GIS descriptor SRID.
        expected_srid: Option<i32>,
        /// Actual GIS descriptor SRID.
        actual_srid: Option<i32>,
    },

    /// A coordinate is not finite.
    #[display("non-finite tactical coordinate for combatant slot {}", _0)]
    NonFiniteCoordinate(usize),

    /// A GIS point descriptor has no 2D coordinate.
    #[display("empty GIS point descriptor for combatant slot {}", _0)]
    EmptyPoint(usize),

    /// A GIS point descriptor coordinate differs from the tactical coordinate.
    #[display("GIS point coordinate mismatch for combatant slot {}", _0)]
    PointCoordinateMismatch(usize),

    /// A tactical footprint radius is invalid.
    #[display(
        "invalid tactical footprint radius for {}: {} meters",
        label,
        radius_meters
    )]
    InvalidFootprintRadius {
        /// Footprint label used for diagnostics.
        label: String,
        /// Invalid radius in meters.
        radius_meters: f64,
    },

    /// A footprint frame differs from the encounter frame.
    #[display(
        "footprint frame mismatch for {}: expected SRID {}, got SRID {}",
        label,
        expected_srid,
        actual_srid
    )]
    FootprintFrameMismatch {
        /// Footprint label used for diagnostics.
        label: String,
        /// Expected tactical frame SRID.
        expected_srid: i32,
        /// Actual footprint frame SRID.
        actual_srid: i32,
    },

    /// A footprint GIS point descriptor differs from the Valinoreth tactical frame.
    #[display(
        "footprint GIS point SRID mismatch for {}: expected {:?}, got {:?}",
        label,
        expected_srid,
        actual_srid
    )]
    FootprintPointSridMismatch {
        /// Footprint label used for diagnostics.
        label: String,
        /// Expected GIS descriptor SRID.
        expected_srid: Option<i32>,
        /// Actual GIS descriptor SRID.
        actual_srid: Option<i32>,
    },

    /// A footprint coordinate is not finite.
    #[display("non-finite tactical footprint coordinate for {}", _0)]
    NonFiniteFootprintCoordinate(String),

    /// A footprint GIS point descriptor has no 2D coordinate.
    #[display("empty GIS point descriptor for footprint {}", _0)]
    EmptyFootprintPoint(String),

    /// A footprint GIS point descriptor coordinate differs from the tactical coordinate.
    #[display("GIS point coordinate mismatch for footprint {}", _0)]
    FootprintPointCoordinateMismatch(String),

    /// A terrain movement-cost multiplier is invalid.
    #[display(
        "invalid movement cost multiplier for terrain zone {}: {}",
        zone_id,
        multiplier
    )]
    InvalidTerrainCostMultiplier {
        /// Terrain zone identifier.
        zone_id: String,
        /// Invalid multiplier.
        multiplier: f64,
    },

    /// A tactical distance calculation produced a non-finite result.
    #[display(
        "non-finite tactical distance between combatant slots {} and {}",
        from,
        to
    )]
    NonFiniteDistance {
        /// Origin combatant slot.
        from: usize,
        /// Destination combatant slot.
        to: usize,
    },

    /// A reach limit is invalid.
    #[display(
        "invalid melee reach limit: min {} meters, max {} meters",
        min_meters,
        max_meters
    )]
    InvalidReachLimit {
        /// Minimum valid reach in meters.
        min_meters: f64,
        /// Maximum valid reach in meters.
        max_meters: f64,
    },

    /// A range limit is invalid.
    #[display("invalid ranged attack limit: {} meters", _0)]
    InvalidRangeLimit(f64),

    /// The declared target is outside the attacker's melee reach.
    #[display(
        "target slot {} is outside reach for attacker slot {}: distance {} meters, reach {}-{} meters",
        target,
        attacker,
        distance_meters,
        min_reach_meters,
        max_reach_meters
    )]
    TargetOutOfReach {
        /// Attacking combatant slot.
        attacker: usize,
        /// Declared target combatant slot.
        target: usize,
        /// Measured tactical distance.
        distance_meters: f64,
        /// Minimum reach in meters.
        min_reach_meters: f64,
        /// Maximum reach in meters.
        max_reach_meters: f64,
    },

    /// The declared target is outside the attack's range.
    #[display(
        "target slot {} is outside range for attacker slot {}: distance {} meters, range {} meters",
        target,
        attacker,
        distance_meters,
        range_meters
    )]
    TargetOutOfRange {
        /// Attacking combatant slot.
        attacker: usize,
        /// Declared target combatant slot.
        target: usize,
        /// Measured tactical distance.
        distance_meters: f64,
        /// Maximum range in meters.
        range_meters: f64,
    },

    /// A tactical line of effect is blocked by an obstacle.
    #[display("line of effect is blocked by obstacle {}", obstacle_id)]
    LineOfEffectBlockedByObstacle {
        /// Blocking obstacle identifier.
        obstacle_id: String,
    },

    /// GIS point construction failed.
    #[display("GIS point construction failed: {}", _0)]
    Gis(String),

    /// Builder validation failed.
    #[display("spatial builder failed: {}", _0)]
    Builder(String),
}

/// Errors that can occur while establishing tactical spatial facts.
#[derive(Debug, Clone, derive_more::Display, derive_more::Error)]
#[display("Spatial error: {} at {}:{}", kind, file, line)]
pub struct SpatialError {
    /// The specific error kind.
    pub kind: SpatialErrorKind,
    /// Source file where the error was created.
    pub file: &'static str,
    /// Source line where the error was created.
    pub line: u32,
}

impl SpatialError {
    /// Create a spatial error with call-site tracking.
    #[track_caller]
    #[instrument]
    pub fn new(kind: SpatialErrorKind) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            kind,
            file: loc.file(),
            line: loc.line(),
        }
    }
}

impl From<elicit_gis::GisError> for SpatialError {
    #[track_caller]
    fn from(error: elicit_gis::GisError) -> Self {
        Self::new(SpatialErrorKind::Gis(error.to_string()))
    }
}

#[derive(Debug, Clone, Copy)]
struct ValinorethTacticalSfsFactory;

impl ValinorethTacticalSfsFactory {
    fn unsupported(operation: &str) -> GisError {
        GisError::new(GisErrorKind::Unsupported(operation.to_string()))
    }
}

impl SfsGeometryFactory for ValinorethTacticalSfsFactory {
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

/// Tactical distance unit for encounter-space coordinates.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, strum::EnumIter,
)]
#[serde(rename_all = "camelCase")]
pub enum TacticalDistanceUnit {
    /// Meters, matching GURPS tactical movement distance.
    Meter,
}

/// Coordinate frame for a tactical combat encounter.
#[derive(Debug, Clone, PartialEq, Eq, Builder, Serialize, Deserialize, JsonSchema)]
#[builder(setter(into))]
pub struct TacticalFrame {
    /// Local tactical SRID used by every placement in this encounter.
    pub srid: i32,

    /// Source/world CRS from which this tactical frame is derived.
    #[builder(default = "Some(WGS84_SRID)")]
    pub source_srid: Option<i32>,

    /// Distance unit used by tactical coordinates.
    #[builder(default = "TacticalDistanceUnit::Meter")]
    pub unit: TacticalDistanceUnit,

    /// Human-readable frame label.
    pub label: String,
}

/// Build the default Valinoreth tactical encounter frame.
#[instrument]
pub fn local_tactical_frame() -> TacticalFrame {
    tactical_frame_from_source(
        WGS84_SRID,
        VALINORETH_TACTICAL_SRID,
        "Valinoreth local tactical frame".to_string(),
    )
    .expect("default tactical frame is valid")
}

/// Build an explicit tactical encounter frame from a source CRS.
#[instrument]
pub fn tactical_frame_from_source(
    source_srid: i32,
    tactical_srid: i32,
    label: String,
) -> SpatialResult<TacticalFrame> {
    TacticalFrameBuilder::default()
        .srid(tactical_srid)
        .source_srid(Some(source_srid))
        .unit(TacticalDistanceUnit::Meter)
        .label(label)
        .build()
        .map_err(|error| SpatialError::new(SpatialErrorKind::Builder(error.to_string())))
}

/// A 2D coordinate in encounter meters.
#[derive(Debug, Clone, Copy, PartialEq, Builder, Serialize, Deserialize, JsonSchema)]
#[builder(setter(into))]
pub struct TacticalCoordinate {
    /// Easting or local X ordinate in meters.
    pub x_meters: f64,

    /// Northing or local Y ordinate in meters.
    pub y_meters: f64,
}

impl TacticalCoordinate {
    /// Returns `true` when both coordinate ordinates are finite.
    #[instrument]
    pub fn is_finite(&self) -> bool {
        self.x_meters.is_finite() && self.y_meters.is_finite()
    }
}

/// A tactical point backed by an `elicit_gis` point-validity proof.
#[derive(Debug, Clone, Builder, Serialize, Deserialize, JsonSchema)]
#[builder(setter(into))]
pub struct TacticalPoint {
    /// Game-facing coordinate in encounter meters.
    pub coordinate: TacticalCoordinate,

    /// Tactical coordinate frame for this point.
    pub frame: TacticalFrame,

    /// GIS construction receipt for the validated point.
    pub descriptor: PointDescriptor,

    /// Proof that the underlying GIS point is valid.
    pub point_valid: Established<PointValid>,
}

impl PartialEq for TacticalPoint {
    fn eq(&self, other: &Self) -> bool {
        self.coordinate == other.coordinate
            && self.frame == other.frame
            && self.descriptor == other.descriptor
    }
}

/// Build a tactical point by delegating GIS point validity to `elicit_gis`.
#[instrument(skip(factory), fields(srid = frame.srid))]
pub fn build_tactical_point(
    factory: &dyn SfsGeometryFactory,
    coordinate: TacticalCoordinate,
    frame: TacticalFrame,
) -> SpatialResult<TacticalPoint> {
    let (descriptor, point_valid) = factory.build_point(
        SfsCoordinate {
            x: coordinate.x_meters,
            y: coordinate.y_meters,
        },
        Some(frame.srid),
    )?;

    TacticalPointBuilder::default()
        .coordinate(coordinate)
        .frame(frame)
        .descriptor(descriptor)
        .point_valid(point_valid)
        .build()
        .map_err(|error| SpatialError::new(SpatialErrorKind::Builder(error.to_string())))
}

/// Build a tactical point using Valinoreth's local GIS point factory.
#[instrument(fields(srid = frame.srid))]
pub fn build_default_tactical_point(
    coordinate: TacticalCoordinate,
    frame: TacticalFrame,
) -> SpatialResult<TacticalPoint> {
    build_tactical_point(&ValinorethTacticalSfsFactory, coordinate, frame)
}

/// A combatant slot paired with its tactical point.
#[derive(Debug, Clone, PartialEq, Builder, Serialize, Deserialize, JsonSchema)]
#[builder(setter(into))]
pub struct CombatantPlacement {
    /// Combatant slot in the active combat state.
    pub slot: CombatantSlot,

    /// Validated tactical point occupied by the combatant.
    pub point: TacticalPoint,
}

/// Circular occupied or blocking footprint in tactical meters.
#[derive(Debug, Clone, PartialEq, Builder, Serialize, Deserialize, JsonSchema)]
#[builder(setter(into))]
pub struct TacticalFootprint {
    /// Footprint center point.
    pub center: TacticalPoint,

    /// Footprint radius in tactical meters.
    pub radius_meters: f64,
}

/// Explicit occupied space for a combatant.
#[derive(Debug, Clone, PartialEq, Builder, Serialize, Deserialize, JsonSchema)]
#[builder(setter(into))]
pub struct CombatantOccupancy {
    /// Combatant slot whose occupied footprint is represented.
    pub slot: CombatantSlot,

    /// Occupied circular footprint.
    pub footprint: TacticalFootprint,
}

/// Tactical map feature that can block movement or line of effect.
#[derive(Debug, Clone, PartialEq, Builder, Serialize, Deserialize, JsonSchema)]
#[builder(setter(into))]
pub struct TacticalObstacle {
    /// Stable map-feature identifier.
    pub id: String,

    /// Blocking footprint.
    pub footprint: TacticalFootprint,

    /// Whether this feature blocks movement paths.
    pub blocks_movement: bool,

    /// Whether this feature blocks line of effect.
    pub blocks_line_of_effect: bool,
}

/// Tactical terrain zone that modifies movement cost.
#[derive(Debug, Clone, PartialEq, Builder, Serialize, Deserialize, JsonSchema)]
#[builder(setter(into))]
pub struct TacticalTerrainZone {
    /// Stable terrain-zone identifier.
    pub id: String,

    /// Terrain-zone footprint.
    pub footprint: TacticalFootprint,

    /// Multiplier applied to movement cost through this zone.
    pub movement_cost_multiplier: f64,
}

/// Sidecar state describing combatant placement in tactical coordinate space.
#[derive(Debug, Clone, PartialEq, Builder, Serialize, Deserialize, JsonSchema)]
#[builder(setter(into))]
pub struct CombatSpatialState {
    /// Encounter tactical frame shared by every placement.
    pub frame: TacticalFrame,

    /// One placement per active combatant.
    pub placements: Vec<CombatantPlacement>,

    /// Explicit occupied spaces keyed by combatant slot.
    #[builder(default)]
    pub occupancies: Vec<CombatantOccupancy>,

    /// Explicit tactical obstacles for the encounter.
    #[builder(default)]
    pub obstacles: Vec<TacticalObstacle>,

    /// Explicit tactical terrain zones for movement-cost calculation.
    #[builder(default)]
    pub terrain: Vec<TacticalTerrainZone>,
}

/// Game-facing tactical distance between two combatants.
#[derive(Debug, Clone, Copy, PartialEq, Builder, Serialize, Deserialize, JsonSchema)]
#[builder(setter(into))]
pub struct TacticalDistance {
    /// Origin combatant slot.
    pub from: CombatantSlot,

    /// Destination combatant slot.
    pub to: CombatantSlot,

    /// Euclidean tactical distance in encounter meters.
    pub meters: f64,
}

/// Inclusive melee reach interval in tactical meters.
#[derive(Debug, Clone, Copy, PartialEq, Builder, Serialize, Deserialize, JsonSchema)]
#[builder(setter(into))]
pub struct AttackReach {
    /// Minimum legal target distance in meters.
    pub min_meters: f64,

    /// Maximum legal target distance in meters.
    pub max_meters: f64,
}

/// Maximum ranged attack distance in tactical meters.
#[derive(Debug, Clone, Copy, PartialEq, Builder, Serialize, Deserialize, JsonSchema)]
#[builder(setter(into))]
pub struct AttackRange {
    /// Maximum legal target distance in meters.
    pub meters: f64,
}

/// Declared melee target paired with the proof-relevant spatial measurement.
#[derive(Debug, Clone, Copy, PartialEq, Builder)]
#[builder(setter(into))]
pub struct SpatiallyCheckedMeleeTarget {
    /// Original combat VSM target sidecar.
    pub declared_target: DeclaredAttackTarget,

    /// Measured distance between attacker and declared target.
    pub distance: TacticalDistance,

    /// Reach interval used to validate the target.
    pub reach: AttackReach,
}

impl SpatiallyCheckedMeleeTarget {
    /// Return the combat VSM target sidecar after spatial validation.
    pub const fn declared_target(self) -> DeclaredAttackTarget {
        self.declared_target
    }
}

/// Declared ranged target paired with the proof-relevant spatial measurement.
#[derive(Debug, Clone, Copy, PartialEq, Builder)]
#[builder(setter(into))]
pub struct SpatiallyCheckedRangedTarget {
    /// Original combat VSM target sidecar.
    pub declared_target: DeclaredAttackTarget,

    /// Measured distance between attacker and declared target.
    pub distance: TacticalDistance,

    /// Range limit used to validate the target.
    pub range: AttackRange,
}

impl SpatiallyCheckedRangedTarget {
    /// Return the combat VSM target sidecar after spatial validation.
    pub const fn declared_target(self) -> DeclaredAttackTarget {
        self.declared_target
    }
}

struct SpatialStateChecked;

impl ProvableFrom<SpatialStateChecked> for SpatialStateConsistent {}

struct CombatantLocationChecked;

impl ProvableFrom<CombatantLocationChecked> for CombatantLocated {}

struct SameFrameChecked;

impl ProvableFrom<SameFrameChecked> for LocationsHaveSameFrame {}

struct LineOfEffectChecked;

impl ProvableFrom<LineOfEffectChecked> for LineOfEffectClear {}

struct TargetWithinReachChecked;

impl ProvableFrom<TargetWithinReachChecked> for TargetWithinReach {}

struct TargetWithinRangeChecked;

impl ProvableFrom<TargetWithinRangeChecked> for TargetWithinRange {}

fn validate_reach_limit(reach: AttackReach) -> SpatialResult<AttackReach> {
    if !reach.min_meters.is_finite()
        || !reach.max_meters.is_finite()
        || reach.min_meters < 0.0
        || reach.max_meters < reach.min_meters
    {
        return Err(SpatialError::new(SpatialErrorKind::InvalidReachLimit {
            min_meters: reach.min_meters,
            max_meters: reach.max_meters,
        }));
    }

    Ok(reach)
}

fn validate_range_limit(range: AttackRange) -> SpatialResult<AttackRange> {
    if !range.meters.is_finite() || range.meters < 0.0 {
        return Err(SpatialError::new(SpatialErrorKind::InvalidRangeLimit(
            range.meters,
        )));
    }

    Ok(range)
}

fn validate_point_against_frame(
    placement: &CombatantPlacement,
    expected_srid: i32,
) -> SpatialResult<()> {
    let slot = placement.slot.index();
    if placement.point.frame.srid != expected_srid {
        return Err(SpatialError::new(SpatialErrorKind::FrameMismatch {
            slot,
            expected_srid,
            actual_srid: placement.point.frame.srid,
        }));
    }
    if placement.point.descriptor.srid != Some(expected_srid) {
        return Err(SpatialError::new(SpatialErrorKind::PointSridMismatch {
            slot,
            expected_srid: Some(expected_srid),
            actual_srid: placement.point.descriptor.srid,
        }));
    }
    if !placement.point.coordinate.is_finite() {
        return Err(SpatialError::new(SpatialErrorKind::NonFiniteCoordinate(
            slot,
        )));
    }
    if placement.point.descriptor.x.is_none() || placement.point.descriptor.y.is_none() {
        return Err(SpatialError::new(SpatialErrorKind::EmptyPoint(slot)));
    }
    if placement.point.descriptor.x != Some(placement.point.coordinate.x_meters)
        || placement.point.descriptor.y != Some(placement.point.coordinate.y_meters)
    {
        return Err(SpatialError::new(
            SpatialErrorKind::PointCoordinateMismatch(slot),
        ));
    }

    Ok(())
}

fn validate_footprint_against_frame(
    footprint: &TacticalFootprint,
    label: &str,
    expected_srid: i32,
) -> SpatialResult<()> {
    if !footprint.radius_meters.is_finite() || footprint.radius_meters < 0.0 {
        return Err(SpatialError::new(
            SpatialErrorKind::InvalidFootprintRadius {
                label: label.to_string(),
                radius_meters: footprint.radius_meters,
            },
        ));
    }
    if footprint.center.frame.srid != expected_srid {
        return Err(SpatialError::new(
            SpatialErrorKind::FootprintFrameMismatch {
                label: label.to_string(),
                expected_srid,
                actual_srid: footprint.center.frame.srid,
            },
        ));
    }
    if footprint.center.descriptor.srid != Some(expected_srid) {
        return Err(SpatialError::new(
            SpatialErrorKind::FootprintPointSridMismatch {
                label: label.to_string(),
                expected_srid: Some(expected_srid),
                actual_srid: footprint.center.descriptor.srid,
            },
        ));
    }
    if !footprint.center.coordinate.is_finite() {
        return Err(SpatialError::new(
            SpatialErrorKind::NonFiniteFootprintCoordinate(label.to_string()),
        ));
    }
    if footprint.center.descriptor.x.is_none() || footprint.center.descriptor.y.is_none() {
        return Err(SpatialError::new(SpatialErrorKind::EmptyFootprintPoint(
            label.to_string(),
        )));
    }
    if footprint.center.descriptor.x != Some(footprint.center.coordinate.x_meters)
        || footprint.center.descriptor.y != Some(footprint.center.coordinate.y_meters)
    {
        return Err(SpatialError::new(
            SpatialErrorKind::FootprintPointCoordinateMismatch(label.to_string()),
        ));
    }

    Ok(())
}

fn validate_terrain_zone(terrain: &TacticalTerrainZone, expected_srid: i32) -> SpatialResult<()> {
    validate_footprint_against_frame(
        &terrain.footprint,
        &format!("terrain {}", terrain.id),
        expected_srid,
    )?;
    if !terrain.movement_cost_multiplier.is_finite() || terrain.movement_cost_multiplier < 1.0 {
        return Err(SpatialError::new(
            SpatialErrorKind::InvalidTerrainCostMultiplier {
                zone_id: terrain.id.clone(),
                multiplier: terrain.movement_cost_multiplier,
            },
        ));
    }

    Ok(())
}

fn placement_for_slot(
    spatial_state: &CombatSpatialState,
    slot: CombatantSlot,
) -> SpatialResult<&CombatantPlacement> {
    spatial_state
        .placements
        .iter()
        .find(|placement| placement.slot == slot)
        .ok_or_else(|| SpatialError::new(SpatialErrorKind::MissingPlacement(slot.index())))
}

fn tactical_distance_between_checked(
    first: &CombatantPlacement,
    second: &CombatantPlacement,
) -> SpatialResult<TacticalDistance> {
    let dx = first.point.coordinate.x_meters - second.point.coordinate.x_meters;
    let dy = first.point.coordinate.y_meters - second.point.coordinate.y_meters;
    let meters = dx.hypot(dy);
    if !meters.is_finite() {
        return Err(SpatialError::new(SpatialErrorKind::NonFiniteDistance {
            from: first.slot.index(),
            to: second.slot.index(),
        }));
    }

    TacticalDistanceBuilder::default()
        .from(first.slot)
        .to(second.slot)
        .meters(meters)
        .build()
        .map_err(|error| SpatialError::new(SpatialErrorKind::Builder(error.to_string())))
}

fn coordinate_distance(first: TacticalCoordinate, second: TacticalCoordinate) -> f64 {
    let dx = first.x_meters - second.x_meters;
    let dy = first.y_meters - second.y_meters;
    dx.hypot(dy)
}

fn segment_intersects_footprint(
    start: TacticalCoordinate,
    end: TacticalCoordinate,
    footprint: &TacticalFootprint,
) -> bool {
    let center = footprint.center.coordinate;
    let dx = end.x_meters - start.x_meters;
    let dy = end.y_meters - start.y_meters;
    let length_squared = dx.mul_add(dx, dy * dy);

    if length_squared == 0.0 {
        return coordinate_distance(start, center) <= footprint.radius_meters;
    }

    let start_to_center_x = center.x_meters - start.x_meters;
    let start_to_center_y = center.y_meters - start.y_meters;
    let projection =
        (start_to_center_x.mul_add(dx, start_to_center_y * dy) / length_squared).clamp(0.0, 1.0);
    let closest = TacticalCoordinate {
        x_meters: start.x_meters + projection * dx,
        y_meters: start.y_meters + projection * dy,
    };

    coordinate_distance(closest, center) <= footprint.radius_meters
}

fn movement_cost_multiplier_for_segment(
    spatial_state: &CombatSpatialState,
    start: TacticalCoordinate,
    end: TacticalCoordinate,
) -> f64 {
    spatial_state
        .terrain
        .iter()
        .filter(|terrain| segment_intersects_footprint(start, end, &terrain.footprint))
        .map(|terrain| terrain.movement_cost_multiplier)
        .fold(1.0, f64::max)
}

pub(crate) fn movement_blocking_occupant_for_segment(
    spatial_state: &CombatSpatialState,
    actor: CombatantSlot,
    start: TacticalCoordinate,
    end: TacticalCoordinate,
) -> Option<CombatantSlot> {
    spatial_state
        .occupancies
        .iter()
        .find(|occupancy| {
            occupancy.slot != actor
                && segment_intersects_footprint(start, end, &occupancy.footprint)
        })
        .map(|occupancy| occupancy.slot)
}

pub(crate) fn movement_blocking_obstacle_for_segment(
    spatial_state: &CombatSpatialState,
    start: TacticalCoordinate,
    end: TacticalCoordinate,
) -> Option<&TacticalObstacle> {
    spatial_state.obstacles.iter().find(|obstacle| {
        obstacle.blocks_movement && segment_intersects_footprint(start, end, &obstacle.footprint)
    })
}

fn line_of_effect_blocking_obstacle_for_segment(
    spatial_state: &CombatSpatialState,
    start: TacticalCoordinate,
    end: TacticalCoordinate,
) -> Option<&TacticalObstacle> {
    spatial_state.obstacles.iter().find(|obstacle| {
        obstacle.blocks_line_of_effect
            && segment_intersects_footprint(start, end, &obstacle.footprint)
    })
}

pub(crate) fn movement_cost_meters_for_segment(
    spatial_state: &CombatSpatialState,
    start: TacticalCoordinate,
    end: TacticalCoordinate,
    distance_meters: f64,
) -> f64 {
    let multiplier = movement_cost_multiplier_for_segment(spatial_state, start, end);
    distance_meters * multiplier
}

/// Build a circular tactical footprint around an established tactical point.
#[instrument(skip(center), fields(radius_meters))]
pub fn tactical_footprint(
    center: TacticalPoint,
    radius_meters: f64,
) -> SpatialResult<TacticalFootprint> {
    let footprint = TacticalFootprintBuilder::default()
        .center(center)
        .radius_meters(radius_meters)
        .build()
        .map_err(|error| SpatialError::new(SpatialErrorKind::Builder(error.to_string())))?;
    validate_footprint_against_frame(&footprint, "footprint", footprint.center.frame.srid)?;

    Ok(footprint)
}

/// Build a combatant occupancy footprint from an existing placement.
#[instrument(skip(placement), fields(slot = placement.slot.index(), radius_meters))]
pub fn combatant_occupancy_from_placement(
    placement: &CombatantPlacement,
    radius_meters: f64,
) -> SpatialResult<CombatantOccupancy> {
    let footprint = tactical_footprint(placement.point.clone(), radius_meters)?;

    CombatantOccupancyBuilder::default()
        .slot(placement.slot)
        .footprint(footprint)
        .build()
        .map_err(|error| SpatialError::new(SpatialErrorKind::Builder(error.to_string())))
}

/// Establish that a spatial sidecar is coherent with the active combat state.
#[instrument(skip(combat_state, spatial_state), fields(placements = spatial_state.placements.len()))]
pub fn establish_spatial_state(
    combat_state: &CombatState,
    spatial_state: CombatSpatialState,
) -> SpatialResult<(CombatSpatialState, Established<SpatialStateConsistent>)> {
    let combatant_count = match combat_state {
        CombatState::Active { combatants, .. } => combatants.len(),
        CombatState::Uninitialized | CombatState::Concluded { .. } => {
            return Err(SpatialError::new(SpatialErrorKind::CombatNotActive));
        }
    };

    let mut seen = vec![false; combatant_count];
    for placement in &spatial_state.placements {
        let slot = placement.slot.index();
        if slot >= combatant_count {
            return Err(SpatialError::new(SpatialErrorKind::UnknownPlacement(slot)));
        }
        if seen[slot] {
            return Err(SpatialError::new(SpatialErrorKind::DuplicatePlacement(
                slot,
            )));
        }
        validate_point_against_frame(placement, spatial_state.frame.srid)?;
        seen[slot] = true;
    }

    for (slot, has_placement) in seen.into_iter().enumerate() {
        if !has_placement {
            return Err(SpatialError::new(SpatialErrorKind::MissingPlacement(slot)));
        }
    }

    for occupancy in &spatial_state.occupancies {
        let slot = occupancy.slot.index();
        if slot >= combatant_count {
            return Err(SpatialError::new(SpatialErrorKind::UnknownOccupancy(slot)));
        }
        validate_footprint_against_frame(
            &occupancy.footprint,
            &format!("occupancy {slot}"),
            spatial_state.frame.srid,
        )?;
    }

    for obstacle in &spatial_state.obstacles {
        validate_footprint_against_frame(
            &obstacle.footprint,
            &format!("obstacle {}", obstacle.id),
            spatial_state.frame.srid,
        )?;
    }

    for terrain in &spatial_state.terrain {
        validate_terrain_zone(terrain, spatial_state.frame.srid)?;
    }

    Ok((
        spatial_state,
        Established::<SpatialStateConsistent>::prove(&SpatialStateChecked),
    ))
}

/// Create deterministic initial placements for an active combat encounter.
#[instrument(skip(combat_state))]
pub fn initial_spatial_state_for_combat(
    combat_state: &CombatState,
) -> SpatialResult<(CombatSpatialState, Established<SpatialStateConsistent>)> {
    let combatant_count = match combat_state {
        CombatState::Active { combatants, .. } => combatants.len(),
        CombatState::Uninitialized | CombatState::Concluded { .. } => {
            return Err(SpatialError::new(SpatialErrorKind::CombatNotActive));
        }
    };

    let frame = local_tactical_frame();
    let mut placements = Vec::with_capacity(combatant_count);
    for slot in 0..combatant_count {
        let coordinate = TacticalCoordinateBuilder::default()
            .x_meters(slot as f64)
            .y_meters(0.0)
            .build()
            .map_err(|error| SpatialError::new(SpatialErrorKind::Builder(error.to_string())))?;
        let point = build_default_tactical_point(coordinate, frame.clone())?;
        let placement = CombatantPlacementBuilder::default()
            .slot(CombatantSlot::new(slot))
            .point(point)
            .build()
            .map_err(|error| SpatialError::new(SpatialErrorKind::Builder(error.to_string())))?;
        placements.push(placement);
    }

    let spatial_state = CombatSpatialStateBuilder::default()
        .frame(frame)
        .placements(placements)
        .build()
        .map_err(|error| SpatialError::new(SpatialErrorKind::Builder(error.to_string())))?;

    establish_spatial_state(combat_state, spatial_state)
}

/// Locate a combatant in an established spatial sidecar.
#[instrument(skip(spatial_state, _spatial_proof), fields(slot = slot.index()))]
pub fn locate_combatant<'a>(
    spatial_state: &'a CombatSpatialState,
    _spatial_proof: &Established<SpatialStateConsistent>,
    slot: CombatantSlot,
) -> SpatialResult<(&'a CombatantPlacement, Established<CombatantLocated>)> {
    let placement = placement_for_slot(spatial_state, slot)?;
    validate_point_against_frame(placement, spatial_state.frame.srid)?;

    Ok((
        placement,
        Established::<CombatantLocated>::prove(&CombatantLocationChecked),
    ))
}

/// Establish that two placements are in the same tactical coordinate frame.
#[instrument(skip(first, second), fields(from = first.slot.index(), to = second.slot.index()))]
pub fn establish_same_frame(
    first: &CombatantPlacement,
    second: &CombatantPlacement,
) -> SpatialResult<Established<LocationsHaveSameFrame>> {
    validate_point_against_frame(first, first.point.frame.srid)?;
    validate_point_against_frame(second, second.point.frame.srid)?;

    if first.point.frame.srid != second.point.frame.srid {
        return Err(SpatialError::new(SpatialErrorKind::FrameMismatch {
            slot: second.slot.index(),
            expected_srid: first.point.frame.srid,
            actual_srid: second.point.frame.srid,
        }));
    }

    Ok(Established::<LocationsHaveSameFrame>::prove(
        &SameFrameChecked,
    ))
}

/// Calculate tactical distance between two placements in encounter meters.
#[instrument(skip(first, second), fields(from = first.slot.index(), to = second.slot.index()))]
pub fn distance_between_placements(
    first: &CombatantPlacement,
    second: &CombatantPlacement,
) -> SpatialResult<(TacticalDistance, Established<LocationsHaveSameFrame>)> {
    let same_frame = establish_same_frame(first, second)?;
    let distance = tactical_distance_between_checked(first, second)?;

    Ok((distance, same_frame))
}

/// Calculate tactical distance between two combatants in an established sidecar.
#[instrument(skip(spatial_state, spatial_proof), fields(from = from.index(), to = to.index()))]
pub fn distance_between_combatants(
    spatial_state: &CombatSpatialState,
    spatial_proof: &Established<SpatialStateConsistent>,
    from: CombatantSlot,
    to: CombatantSlot,
) -> SpatialResult<(TacticalDistance, Established<LocationsHaveSameFrame>)> {
    let (from_placement, _from_located) = locate_combatant(spatial_state, spatial_proof, from)?;
    let (to_placement, _to_located) = locate_combatant(spatial_state, spatial_proof, to)?;

    distance_between_placements(from_placement, to_placement)
}

/// Establish that line of effect between two combatants is unobstructed.
#[instrument(skip(spatial_state, spatial_proof), fields(from = from.index(), to = to.index()))]
pub fn check_line_of_effect_between_combatants(
    spatial_state: &CombatSpatialState,
    spatial_proof: &Established<SpatialStateConsistent>,
    from: CombatantSlot,
    to: CombatantSlot,
) -> SpatialResult<(TacticalDistance, Established<LineOfEffectClear>)> {
    let (from_placement, _from_located) = locate_combatant(spatial_state, spatial_proof, from)?;
    let (to_placement, _to_located) = locate_combatant(spatial_state, spatial_proof, to)?;
    let (distance, _same_frame) = distance_between_placements(from_placement, to_placement)?;

    if let Some(obstacle) = line_of_effect_blocking_obstacle_for_segment(
        spatial_state,
        from_placement.point.coordinate,
        to_placement.point.coordinate,
    ) {
        return Err(SpatialError::new(
            SpatialErrorKind::LineOfEffectBlockedByObstacle {
                obstacle_id: obstacle.id.clone(),
            },
        ));
    }

    Ok((
        distance,
        Established::<LineOfEffectClear>::prove(&LineOfEffectChecked),
    ))
}

/// Convert a GURPS melee weapon reach into an inclusive tactical-meter interval.
#[instrument]
pub fn attack_reach_from_reach(reach: Reach) -> SpatialResult<AttackReach> {
    let (min_meters, max_meters) = match reach {
        Reach::Close => (0.0, 0.0),
        Reach::CloseOne => (0.0, 1.0),
        Reach::One => (1.0, 1.0),
        Reach::OneTwo => (1.0, 2.0),
        Reach::OneThree => (1.0, 3.0),
        Reach::TwoThree => (2.0, 3.0),
        Reach::Three => (3.0, 3.0),
    };

    let reach = AttackReachBuilder::default()
        .min_meters(min_meters)
        .max_meters(max_meters)
        .build()
        .map_err(|error| SpatialError::new(SpatialErrorKind::Builder(error.to_string())))?;

    validate_reach_limit(reach)
}

/// Build an explicit ranged attack limit in tactical meters.
#[instrument]
pub fn attack_range_limit(meters: f64) -> SpatialResult<AttackRange> {
    let range = AttackRangeBuilder::default()
        .meters(meters)
        .build()
        .map_err(|error| SpatialError::new(SpatialErrorKind::Builder(error.to_string())))?;

    validate_range_limit(range)
}

/// Check that a declared melee target is inside the attacker's reach.
#[instrument(skip(spatial_state, spatial_proof), fields(
    attacker = declared_target.attacker().index(),
    target = declared_target.target().index()
))]
pub fn check_target_within_reach(
    spatial_state: &CombatSpatialState,
    spatial_proof: &Established<SpatialStateConsistent>,
    declared_target: DeclaredAttackTarget,
    reach: AttackReach,
) -> SpatialResult<(SpatiallyCheckedMeleeTarget, Established<TargetWithinReach>)> {
    let reach = validate_reach_limit(reach)?;
    let (distance, _same_frame) = distance_between_combatants(
        spatial_state,
        spatial_proof,
        declared_target.attacker(),
        declared_target.target(),
    )?;

    if distance.meters < reach.min_meters || distance.meters > reach.max_meters {
        return Err(SpatialError::new(SpatialErrorKind::TargetOutOfReach {
            attacker: declared_target.attacker().index(),
            target: declared_target.target().index(),
            distance_meters: distance.meters,
            min_reach_meters: reach.min_meters,
            max_reach_meters: reach.max_meters,
        }));
    }

    let checked = SpatiallyCheckedMeleeTargetBuilder::default()
        .declared_target(declared_target)
        .distance(distance)
        .reach(reach)
        .build()
        .map_err(|error| SpatialError::new(SpatialErrorKind::Builder(error.to_string())))?;

    Ok((
        checked,
        Established::<TargetWithinReach>::prove(&TargetWithinReachChecked),
    ))
}

/// Check that a declared ranged target is inside the attack's range.
#[instrument(skip(spatial_state, spatial_proof), fields(
    attacker = declared_target.attacker().index(),
    target = declared_target.target().index()
))]
pub fn check_target_within_range(
    spatial_state: &CombatSpatialState,
    spatial_proof: &Established<SpatialStateConsistent>,
    declared_target: DeclaredAttackTarget,
    range: AttackRange,
) -> SpatialResult<(SpatiallyCheckedRangedTarget, Established<TargetWithinRange>)> {
    let range = validate_range_limit(range)?;
    let (distance, _same_frame) = distance_between_combatants(
        spatial_state,
        spatial_proof,
        declared_target.attacker(),
        declared_target.target(),
    )?;

    if distance.meters > range.meters {
        return Err(SpatialError::new(SpatialErrorKind::TargetOutOfRange {
            attacker: declared_target.attacker().index(),
            target: declared_target.target().index(),
            distance_meters: distance.meters,
            range_meters: range.meters,
        }));
    }

    let checked = SpatiallyCheckedRangedTargetBuilder::default()
        .declared_target(declared_target)
        .distance(distance)
        .range(range)
        .build()
        .map_err(|error| SpatialError::new(SpatialErrorKind::Builder(error.to_string())))?;

    Ok((
        checked,
        Established::<TargetWithinRange>::prove(&TargetWithinRangeChecked),
    ))
}
