//! Formal verification proof harnesses for Valinoreth VSMs.
//!
//! Contains generated and manual proof harnesses for [Kani], [Verus], and [Creusot],
//! verifying the state-machine transitions in Combat VSM.
//!
//! # What's in this module
//!
//! | Module | Contents |
//! |---|---|
//! | `kani/generated/` | Auto-generated `proof_for_contract` harnesses — one per VSM transition |
//! | `creusot/generated/` | Auto-generated Creusot companion proofs |
//! | `verus/generated/` | Auto-generated Verus companion proofs |
//!
//! # Generating proofs
//!
//! After changing VSM source files, regenerate all three backends at once:
//!
//! ```sh
//! elicitation generate all \
//!     --crate-path src/vsm \
//!     --out src/proofs
//! ```
//!
//! Or individually:
//!
//! ```sh
//! elicitation generate kani    --crate-path src/vsm --out src/proofs/kani/generated
//! elicitation generate creusot --crate-path src/vsm --out src/proofs/creusot/generated
//! elicitation generate verus   --crate-path src/vsm --out src/proofs/verus/generated
//! ```
//!
//! # Running proofs
//!
//! ```sh
//! # All backends
//! elicitation prove --kani --verus --creusot
//!
//! # Single backend
//! elicitation prove --kani
//! ```
//!
//! [Kani]: https://model-checking.github.io/kani/
//! [Verus]: https://verus-lang.github.io/verus/
//! [Creusot]: https://github.com/creusot-rs/creusot

#[cfg(kani)]
pub mod kani;

#[cfg(creusot)]
pub mod creusot;

#[cfg(verus)]
pub mod verus;
