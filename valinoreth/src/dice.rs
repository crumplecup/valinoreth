//! Dice types for GURPS.
//!
//! GURPS uses 3d6 for all checks. [`DieFace`] represents a single
//! six-sided die (1–6) and [`ThreeDiceRoll`] bundles three dice together,
//! providing the sum (3–18) used for all GURPS checks.
//!
//! # Random Generation
//!
//! Both types integrate with the elicitation framework's [`Generator`] system
//! via `#[derive(Rand)]`. This provides seeded, deterministic dice generation:
//!
//! ```rust,ignore
//! use elicitation::Generator;
//! use valinoreth::ThreeDiceRoll;
//!
//! // Create a deterministic dice generator
//! let dice = ThreeDiceRoll::random_generator(42);
//!
//! // Each call produces the next roll in the sequence
//! let roll = dice.generate();
//! println!("Rolled {}", roll.sum());
//! ```

use elicitation::Elicit;
use elicitation::Generator;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tracing::instrument;

/// A single die face (1–6).
///
/// Using an enum guarantees at the type level that a face can never be 0 or 7+.
///
/// Derives `Rand` for uniform random face selection via `DieFace::random_generator(seed)`.
#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    strum::EnumIter,
    elicitation_derive::Rand,
    Elicit,
    Serialize,
    Deserialize,
    JsonSchema,
)]
pub enum DieFace {
    /// Face showing 1.
    #[default]
    One = 1,
    /// Face showing 2.
    Two = 2,
    /// Face showing 3.
    Three = 3,
    /// Face showing 4.
    Four = 4,
    /// Face showing 5.
    Five = 5,
    /// Face showing 6.
    Six = 6,
}

impl DieFace {
    /// Returns the numeric value (1–6).
    pub fn value(self) -> u8 {
        self as u8
    }

    /// All six faces in order.
    pub const ALL: [DieFace; 6] = [
        DieFace::One,
        DieFace::Two,
        DieFace::Three,
        DieFace::Four,
        DieFace::Five,
        DieFace::Six,
    ];

    /// Creates a [`DieFace`] from a numeric value (1–6).
    ///
    /// Returns `None` for values outside 1..=6.
    #[instrument]
    pub fn from_value(v: u8) -> Option<Self> {
        match v {
            1 => Some(DieFace::One),
            2 => Some(DieFace::Two),
            3 => Some(DieFace::Three),
            4 => Some(DieFace::Four),
            5 => Some(DieFace::Five),
            6 => Some(DieFace::Six),
            _ => None,
        }
    }
}

impl std::fmt::Display for DieFace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}

/// A roll of three dice (3d6).
///
/// The sum is always in 3..=18. This is the standard check roll in GURPS.
///
/// # Random Generation
///
/// Use `ThreeDiceRoll::random_generator(seed)` to create a deterministic dice generator.
/// Each `generate()` call produces an independent roll by composing three [`DieFace`]
/// generators with split seeds:
///
/// ```rust,ignore
/// use elicitation::Generator;
/// use valinoreth::ThreeDiceRoll;
///
/// let dice = ThreeDiceRoll::random_generator(42);
/// let roll = dice.generate(); // Independent, seeded roll
/// assert!(roll.sum() >= 3 && roll.sum() <= 18);
/// ```
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Elicit, Serialize, Deserialize, JsonSchema)]
pub struct ThreeDiceRoll {
    /// First die.
    die1: DieFace,
    /// Second die.
    die2: DieFace,
    /// Third die.
    die3: DieFace,
}

impl ThreeDiceRoll {
    /// Creates a new dice roll from three faces.
    pub fn new(die1: DieFace, die2: DieFace, die3: DieFace) -> Self {
        Self { die1, die2, die3 }
    }

    /// Returns the first die.
    pub fn die1(self) -> DieFace {
        self.die1
    }

    /// Returns the second die.
    pub fn die2(self) -> DieFace {
        self.die2
    }

    /// Returns the third die.
    pub fn die3(self) -> DieFace {
        self.die3
    }

    /// Returns the sum of all three dice (3–18).
    pub fn sum(self) -> i32 {
        (self.die1.value() + self.die2.value() + self.die3.value()) as i32
    }

    /// Returns true for critical success (3 or 4).
    ///
    /// # GURPS Rules
    ///
    /// A roll of 3 or 4 is always a critical success.
    /// A roll of 5 or 6 is also a critical success if skill ≥ 15.
    pub fn is_automatic_critical_success(self) -> bool {
        let s = self.sum();
        s <= 4
    }

    /// Returns true for potential critical success with high skill (5 or 6).
    pub fn is_conditional_critical_success(self) -> bool {
        let s = self.sum();
        s == 5 || s == 6
    }

    /// Returns true for automatic critical failure (18).
    ///
    /// # GURPS Rules
    ///
    /// A roll of 18 is always a critical failure.
    /// A roll of 17 is also a critical failure if skill < 16.
    pub fn is_automatic_critical_failure(self) -> bool {
        self.sum() == 18
    }

    /// Returns true for conditional critical failure (17).
    pub fn is_conditional_critical_failure(self) -> bool {
        self.sum() == 17
    }

    /// Creates a seeded dice generator using the elicitation framework.
    ///
    /// The generator composes three independent [`DieFace`] generators with
    /// split seeds, ensuring die1, die2, and die3 are uncorrelated. Same seed
    /// always produces the same sequence — ideal for replays and testing.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use elicitation::Generator;
    /// use valinoreth::ThreeDiceRoll;
    ///
    /// let dice = ThreeDiceRoll::random_generator(42);
    /// let roll1 = dice.generate();
    /// let roll2 = dice.generate();
    /// // Deterministic: same seed → same sequence
    /// ```
    pub fn random_generator(seed: u64) -> impl elicitation::Generator<Target = Self> {
        elicitation_rand::generators::MapGenerator::new(
            elicitation_rand::generators::RandomGenerator::<u64>::with_seed(seed),
            |inner_seed: u64| {
                // Split seed for independent dice
                let gen1 = DieFace::random_generator(inner_seed);
                let gen2 = DieFace::random_generator(inner_seed.wrapping_add(1));
                let gen3 = DieFace::random_generator(inner_seed.wrapping_add(2));
                ThreeDiceRoll::new(gen1.generate(), gen2.generate(), gen3.generate())
            },
        )
    }

    /// All 216 possible dice roll combinations (6³).
    pub fn all_combinations() -> impl Iterator<Item = ThreeDiceRoll> {
        DieFace::ALL.iter().flat_map(|&d1| {
            DieFace::ALL.iter().flat_map(move |&d2| {
                DieFace::ALL
                    .iter()
                    .map(move |&d3| ThreeDiceRoll::new(d1, d2, d3))
            })
        })
    }
}

impl std::fmt::Display for ThreeDiceRoll {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}+{}+{}={}",
            self.die1,
            self.die2,
            self.die3,
            self.sum()
        )
    }
}
