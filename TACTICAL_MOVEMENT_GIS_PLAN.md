# Tactical Movement GIS Plan

## Overview

Add explicit tactical position and movement to the combat VSM by dogfooding
`elicit_gis` as the coordinate and spatial-relationship interface.

The intent is not to turn Valinoreth into a GIS application. The intent is to
use GIS-quality vocabulary for the part of combat that is genuinely spatial:
where combatants are, how far apart they are, whether a movement is legal, and
whether an attack target is reachable from the attacker's actual position.

## Goal

Represent combatant and object location in a precise coordinate space, then use
contracts and proof tokens to make movement and spatial combat rules auditable.

This should eventually let the compiler help enforce facts like:

- the combatant moved from the location they actually occupied
- the movement path is legal for the selected maneuver
- the target being attacked is the target whose position was checked
- the attack has the required reach or range proof
- the VSM state and spatial sidecar state describe the same encounter

## Design Commitments

### Sidecar Pattern

Movement should use the same `(state, proof)` sidecar pattern already used for
combat invariants. Spatial state should be explicit and paired with proof tokens
that establish the relevant program invariant.

The initial direction is a `CombatSpatialState` sidecar keyed by combatant slot
or combatant id. The existing `CombatState` should remain focused on combat
lifecycle until we have a reason to merge the representations.

### Contract Boundary

`elicit_gis` proves GIS facts such as point validity, finite coordinates, SRS
units, and geometric predicates. Valinoreth contracts prove game facts such as
combatant placement, legal movement, reach, range, path clearance, and target
identity.

Do not treat `Established<PointValid>` as proof that a combatant may stand
there. It proves the point is a valid point. A separate Valinoreth proof must
establish that the location is legal in the current encounter.

### World CRS and Tactical Frame

Anchor the campaign world in real GIS coordinates, with WGS84 as the source
CRS and Grants Pass, Oregon as the initial proxy location. This keeps the
long-term path open for LiDAR, national forest terrain data, and other real
geospatial inputs.

Combat should still operate in a local metric tactical frame derived from that
world CRS. GURPS movement is expressed in meters and relative distance; using
longitude and latitude degrees directly for tactical distance would make the
game math harder to audit. The expected model is:

- WGS84 or another real CRS for world/source data
- local projected or engineering frame for combat encounters
- meters as the tactical unit
- explicit proof or validation that compared locations are in the same frame

This gives Valinoreth real-world GIS compatibility without letting earth-scale
coordinate details leak into second-by-second combat logic.

### Generated Proof Crate

`valinoreth_proofs` is generated. Contracts and proof-facing items that must be
referenced by the proof crate need to remain public enough for generation.

Implementation work should update the generator inputs and regenerated proof
artifacts together, rather than hand-editing generated proof code as design
source.

### Repository Rules

When this moves from plan to implementation:

- use builders for new public data types
- keep `lib.rs` to module declarations and re-exports
- put tests under `tests/`, not inline `#[cfg(test)]` modules
- instrument public functions
- do not add `#[allow]` attributes

## Proposed Domain Model

### Core Types

Initial Valinoreth-facing types should wrap GIS concepts in game language:

- `TacticalCoordinate`: a 2D coordinate in encounter meters
- `TacticalPoint`: a validated point descriptor plus point proof
- `CombatantPlacement`: a combatant slot or id paired with a tactical point
- `CombatSpatialState`: all placements for an active combat encounter
- `MovementIntent`: actor, start, destination, maneuver, and movement budget
- `MovementPath`: point-to-point path for the first slice, richer path later
- `ReachEnvelope`: melee reach or ranged envelope derived from weapon/action

Names can change during implementation, but the boundary should remain:
Valinoreth owns game semantics; `elicit_gis` owns spatial primitives and
spatial predicates.

### Contract Candidates

The first movement contracts should be small and composable:

- `SpatialStateConsistent`
- `CombatantLocated`
- `LocationsHaveSameFrame`
- `MovementDeclared`
- `MovementWithinBudget`
- `MovementPathValid`
- `MovementCompleted`
- `TargetPositionChecked`
- `TargetWithinReach`
- `TargetWithinRange`
- `AttackSpatiallyValid`

The important audit point is that `AttackSpatiallyValid` must be derived from a
specific attacker placement and the same target placement that the attack later
uses. This is the spatial analogue of the hit-resolution target identity fix.

### GIS Trait Usage

Use the smallest `elicit_gis` trait surface that supports the next milestone:

- `SfsGeometryFactory` for building validated points and simple paths
- `SfsGeometryMeta` for SRID, dimensionality, and validity reporting
- `SfsTopology::distance` for range and reach checks
- CRS traits for world/source frame and tactical frame metadata

Lean on the elicitation geospatial ecosystem first: `elicit_geo`,
`elicit_geo_types`, `elicit_georaster`, `elicit_rstar`, and related crates
should be preferred over local geometry code. A narrow Valinoreth tactical
adapter is only a fallback if the existing ecosystem cannot provide a required
trait boundary.

## Milestones

### MV0: Dependency and Frame Decision

**Status:** Pending review

Decide how Valinoreth consumes `elicit_gis`.

Acceptance criteria:

- crates.io dependency strategy is explicit for the `0.11` line
- WGS84 source CRS and local metric tactical frame policy is documented
- a tiny point-construction test proves the chosen backend works
- no movement VSM behavior changes yet

Notes:

- use `elicit_gis` as published on crates.io in the `0.11` line
- use the matching elicitation geospatial implementation crates where available
- avoid local path dependencies unless we are explicitly dogfooding unpublished
  framework work

### MV1: Spatial State Sidecar

**Status:** Pending

Introduce the Valinoreth spatial data model without changing combat behavior.

Acceptance criteria:

- active combatants can be paired with validated tactical points
- spatial state can be checked for slot/id coherence
- `SpatialStateConsistent` can be established through one canonical gate
- tests cover valid placement, missing placement, and duplicate placement

### MV2: Spatial Queries

**Status:** Pending

Provide game-facing spatial query helpers backed by `elicit_gis` traits.

Acceptance criteria:

- distance between two combatants is available in meters
- same-frame/SRID mismatch is rejected before game rules use the values
- source-to-tactical frame conversion is explicit rather than implicit
- query helpers return game-specific results, not raw GIS implementation types
- tests cover distance, same location, and frame mismatch

### MV3: Movement Contracts

**Status:** Pending

Encode GURPS movement facts as contracts.

Acceptance criteria:

- a `Move` maneuver has a declared destination and budget
- movement budget uses the character's effective Move
- a GURPS step is represented as one meter minimum, based on Basic Set p. 363
- movement completion requires a proof of declared legal movement
- illegal movement has no proof-token minting path

### MV4: VSM Integration

**Status:** Pending

Thread spatial state through the combat workflow.

Acceptance criteria:

- combat initialization creates or accepts initial placements
- `ManeuverChoice::Move` updates spatial state through contract-checked
  transition functions
- `CombatSession` can expose the current visible position data
- existing attack/defense/damage flow remains behaviorally unchanged

### MV5: Spatial Attack Preconditions

**Status:** Pending

Require spatial proof before resolving attacks.

Acceptance criteria:

- melee attacks require `TargetWithinReach`
- ranged attacks require `TargetWithinRange`
- the checked target placement and declared attack target are tied by type or
  proof evidence
- tests cover the original class of target identity bugs at the spatial layer

### MV6: Obstacles and Occupancy

**Status:** Pending

Add map features only after point movement is working.

Acceptance criteria:

- occupied spaces or footprints are represented explicitly
- blocked movement paths cannot establish `MovementPathValid`
- line of effect or line of sight has a contract boundary
- terrain modifiers can affect movement budget without bypassing contracts

### MV7: Player-Facing Workflow

**Status:** Pending

Expose movement choices through the elicitation-based player interface.

Acceptance criteria:

- players can provide movement intent without hand-written UI branching
- shared knowledge can report visible position and distance information
- hidden or private spatial facts remain controllable by the GM/session model
- chat and TUI frontends receive the same domain-level movement events

### MV8: Proof Generation and Verification

**Status:** Pending

Regenerate proof artifacts after the movement contracts settle.

Acceptance criteria:

- generated Kani and Verus proof crates cover the new contract surface
- Creusot artifacts remain generated for nightly verification
- stable verification remains available for Valinoreth, Kani, and Verus work
- proof crate public API needs are documented rather than treated as leaks

## First Executable Slice

The first implementation slice should be intentionally small:

1. Add the chosen `elicit_gis` and implementation crate dependencies.
2. Add a `spatial` module with tactical coordinate and placement builders.
3. Build validated points through `SfsGeometryFactory`.
4. Add `CombatSpatialState` and `SpatialStateConsistent`.
5. Add tests for placement coherence and distance queries.
6. Leave movement VSM transitions untouched until the sidecar state is solid.

This gives us proof-carrying coordinates before we use them to change combat.

## Open Review Questions

1. Is facing part of the first movement slice, or should it wait for reach,
   shield arcs, and retreat rules?
2. Should combatant occupancy start as a point, circle, footprint, or
   size-derived shape?
3. Which projected or local tactical frame should be derived from the WGS84
   source frame for the Grants Pass proxy area?
4. What is the first concrete source data target after combat movement works:
   LiDAR terrain, forest roads, trails, hydrography, or encounter maps?

## Decisions

- continuous meters are canonical for tactical movement
- hexes and squares are not part of the invariant layer
- grid views may be UI or import/export adapters later
- use the elicitation geospatial ecosystem before considering local geometry
  code
- defer obstacles and terrain until point movement, chasing, and attack reach
  are solid

## Non-Goals for the First Slice

- no full map editor
- no LiDAR or terrain ingestion in the first movement slice
- no ISO metadata workflow in combat logic
- no terrain, cover, or obstacle modeling before point placement works
- no UI-specific movement shortcuts that bypass the contract layer

## Current Repo Touchpoints

Known places likely to change during implementation:

- `valinoreth/src/movement.rs`
- `valinoreth/src/vsm/combat.rs`
- `valinoreth/src/vsm/workflow.rs`
- `valinoreth/src/contracts/combat_flow.rs`
- `valinoreth/src/players/decisions.rs`
- `valinoreth/src/ui/vsm/mod.rs`
- `valinoreth/tests/`
- proof generator inputs and generated `valinoreth_proofs` artifacts

## Notes

Movement is a large addition because it changes what combat state means. The
safe path is to establish coordinates first, then legal movement, then spatial
attack preconditions. Each step should have a narrow proof-token gate so the
audit surface remains small.

The practical acceptance model is tag: if Valinoreth can express two
characters racing, one trying to catch the other, movement budgets changing
locations over repeated turns, and "caught" being established by relative
distance or reach, then the core movement model is useful before terrain and
full attack integration arrive.

---

**Last Updated:** 2026-06-15
