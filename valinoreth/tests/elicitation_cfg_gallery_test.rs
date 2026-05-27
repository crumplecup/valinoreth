//! Gallery for isolating `unexpected_cfgs` warnings from elicitation macros.
//!
//! Each case uses real elicitation derives/attributes with real cfg(kani)/cfg(creusot).
//! Run `cargo check --test elicitation_cfg_gallery_test -p valinoreth` to see which
//! cases produce warnings. No crate-level allow — we want to see them all.
//!
//! | Case | Macro                                             | Warning? |
//! |------|---------------------------------------------------|----------|
//! | A    | #[formal_method] bare                             | NO       |
//! | B    | #[formal_method] + #[instrument]                  | YES      |
//! | C    | #[formal_method] + contracts                      | YES      |
//! | D    | #[formal_method] + contracts + #[instrument]      | YES      |
//! | E    | hand-written mod wrapper (cfg_attr kani/creusot)  | NO       |
//! | F    | hand-written const wrapper (cfg kani/creusot)     | NO       |
//! | G    | #[derive(Elicit)] plain struct                    | YES      |

use elicitation::{formal_method, Elicit, Prop};
use elicitation::{
    formal_method_test_v1, formal_method_test_v2, formal_method_test_v3,
    formal_method_test_v4, formal_method_test_v5, formal_method_test_v6,
    formal_method_test_v7, formal_method_test_v8, formal_method_test_v9,
};
use elicitation::{ElicitTestE1, ElicitTestE2};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use tracing::instrument;

// ── Shared state & contract for Cases C/D ────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Elicit)]
pub struct GalleryState {
    pub value: u32,
}

pub fn gallery_invariant(state: &GalleryState) -> bool {
    state.value < u32::MAX
}

#[derive(Prop)]
#[prop(
    kani_invariant_fn = "gallery_invariant",
    creusot_invariant_fn = "gallery_invariant",
    verus_inv_body = "true",
    creusot_inv_body = "true",
)]
pub struct GalleryInv;

// ── Case A: #[formal_method] bare ─────────────────────────────────────────────
#[formal_method]
pub fn gallery_a_fn(_x: u32) -> u32 {
    _x + 1
}

// ── Case B: #[formal_method] + #[instrument] ──────────────────────────────────
#[formal_method]
#[instrument]
pub fn gallery_b_fn(_x: u32) -> u32 {
    _x + 1
}

// ── Case C: #[formal_method] + contracts ──────────────────────────────────────
#[formal_method(contracts = [GalleryInv])]
pub fn gallery_c_fn(s: GalleryState) -> (GalleryState,) {
    (s,)
}

// ── Case D: #[formal_method] + contracts + #[instrument] ──────────────────────
#[formal_method(contracts = [GalleryInv])]
#[instrument(skip_all)]
pub fn gallery_d_fn(s: GalleryState) -> (GalleryState,) {
    (s,)
}

// ── Case E: hand-written mod wrapper ──────────────────────────────────────────
// Direct hand-written equivalent of what formal_method should emit.
// If this works, the fix must match this pattern exactly.
#[allow(unexpected_cfgs)]
mod _gallery_e_mod {
    #[cfg_attr(not(any(kani, creusot)), inline)]
    pub fn gallery_e_fn() {}
}
pub use _gallery_e_mod::gallery_e_fn;

// ── Case F: hand-written const wrapper ────────────────────────────────────────
#[allow(unexpected_cfgs)]
const _: () = {
    #[cfg(not(any(kani, creusot)))]
    fn _gallery_f_companion() {}
};

#[test]
fn gallery_compiles() {
    gallery_a_fn(1);
    gallery_b_fn(1);
    let s = GalleryState { value: 1 };
    gallery_c_fn(s.clone());
    gallery_d_fn(s);
    gallery_e_fn();
}

// ── Bisect Cases V1–V5: incrementally add features back ──────────────────────

// V1: passthrough — should produce NO warnings
#[formal_method_test_v1]
pub fn gallery_v1_fn(_x: u32) -> u32 { _x }

// V2: mod wrapper only — should produce NO warnings
#[formal_method_test_v2]
pub fn gallery_v2_fn(_x: u32) -> u32 { _x }

// V3: mod wrapper + instrument→cfg_attr transform — should produce NO warnings if mod wrapping works
#[formal_method_test_v3]
#[instrument]
pub fn gallery_v3_fn(_x: u32) -> u32 { _x }

// V4: V3 + empty const _: () = {} — should still be NO warnings
#[formal_method_test_v4]
#[instrument]
pub fn gallery_v4_fn(_x: u32) -> u32 { _x }

// V5: V4 + real #[cfg(kani)] harness inside const — tests whether cfg(kani) in const causes warning
#[formal_method_test_v5]
#[instrument]
pub fn gallery_v5_fn(_x: u32) -> u32 { _x }

// V6: #[allow(unexpected_cfgs)] on the function directly (no mod wrapper)
#[formal_method_test_v6]
#[instrument]
pub fn gallery_v6_fn(_x: u32) -> u32 { _x }

// V7: #[allow] on both outer mod AND inner function
#[formal_method_test_v7]
#[instrument]
pub fn gallery_v7_fn(_x: u32) -> u32 { _x }

// V8: mod wrapper, #[instrument] passed through unchanged (no cfg_attr)
#[formal_method_test_v8]
#[instrument]
pub fn gallery_v8_fn(_x: u32) -> u32 { _x }

// V9: strip #[instrument], manually inject tracing span into body — no cfg_attr at all
#[formal_method_test_v9]
#[instrument]
pub fn gallery_v9_fn(_x: u32) -> u32 { _x }

// ── Impl-block cfg tests ──────────────────────────────────────────────────────
// Tests whether #[allow(unexpected_cfgs)] on an impl block suppresses
// #[cfg(creusot)] warnings on methods within it.
// This mirrors the pattern in struct_impl.rs for #[derive(Elicit)].

pub struct ImplCfgTest;

// I1: allow on impl, cfg on method inside — does it suppress?
#[allow(unexpected_cfgs)]
impl ImplCfgTest {
    #[cfg(not(creusot))]
    pub fn i1_method(&self) -> u32 { 1 }
    #[cfg(creusot)]
    pub fn i1_method(&self) -> u32 { 0 }
}

// I2: no allow on impl — should warn
impl ImplCfgTest {
    #[cfg(not(creusot))]
    pub fn i2_method(&self) -> u32 { 1 }
    #[cfg(creusot)]
    pub fn i2_method(&self) -> u32 { 0 }
}

// I3: allow on impl, cfg in method body (block scope) — does it suppress?
#[allow(unexpected_cfgs)]
impl ImplCfgTest {
    pub fn i3_method(&self) -> u32 {
        #[cfg(creusot)]
        { return 0; }
        1
    }
}

// ── Separate impl hypothesis tests ───────────────────────────────────────────
// I4: cfg on the impl block itself (no allow, no mod) — does rustc warn?
pub struct SeparateImplTest;

#[cfg(not(creusot))]
impl SeparateImplTest {
    pub fn i4_method(&self) -> u32 { 1 }
}
#[cfg(creusot)]
impl SeparateImplTest {
    pub fn i4_method(&self) -> u32 { 0 }
}

// I5: cfg on impl block inside #[allow] mod — does allow propagate?
#[allow(unexpected_cfgs)]
mod _i5_mod {
    use super::*;
    #[cfg(not(creusot))]
    impl SeparateImplTest {
        pub fn i5_method(&self) -> u32 { 1 }
    }
    #[cfg(creusot)]
    impl SeparateImplTest {
        pub fn i5_method(&self) -> u32 { 0 }
    }
}

// ── Minimal isolation: #[derive(Elicit)] with required bounds ────────────────
// J1: minimal enum with Elicit (requires Serialize + Deserialize + JsonSchema)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[derive(Elicit)]
pub enum MinimalEnum {
    A,
    B,
}

// J2: minimal struct with Elicit
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[derive(Elicit)]
pub struct MinimalStruct {
    pub value: u32,
}

// ── Bisect: one sub-piece of expand_enum at a time ───────────────────────────
// E1/E2 remain: they generate only pieces that have no unsatisfied bounds.
// E3–E13 were removed: after the unexpected_cfgs investigation concluded, the
// macros evolved to reference cross-piece types/bounds that break isolated use.
// See elicitation/UNEXPECTED_CFGS.md and git history for the full bisection.

#[derive(ElicitTestE1)]
pub enum BisectEnum { X, Y }

#[derive(ElicitTestE2)]
pub enum BisectEnumE2 { X, Y }


#[test]
fn bisect_compiles() {
    gallery_v1_fn(1);
    gallery_v2_fn(1);
    gallery_v3_fn(1);
    gallery_v4_fn(1);
    gallery_v5_fn(1);
    gallery_v6_fn(1);
    gallery_v7_fn(1);
    gallery_v8_fn(1);
    gallery_v9_fn(1);
}
