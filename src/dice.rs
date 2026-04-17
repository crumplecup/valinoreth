//! Dice rolling and probability mechanics.
//!
//! # GURPS Rules
//!
//! GURPS uses 3d6 (three six-sided dice) as its core mechanic:
//! - Success rolls: Roll 3d6 ≤ skill level
//! - Damage rolls: Roll dice specified by weapon (e.g., 2d+1)
//! - Average roll: 10-11 (bell curve distribution)
//! - Critical success: 3-4, or 5-6 if skill 15+
//! - Critical failure: 17-18, or 10+ over skill
//!
//! # Citations
//!
//! - BS 10 - Success Rolls
//! - BS 269 - Damage Rolls
//! - BS 347 - Critical Success/Failure
//!
//! # Examples
//!
//! ```
//! use valinoreth::{Random, Dice};
//!
//! let mut rng = Random::from_seed(42).unwrap();
//! let roll = Dice::from_random(&mut rng);
//! // Roll is between 3-18
//! ```

use rand::distr::Distribution;

/// Result of rolling 3d6.
///
/// # GURPS Rules
///
/// Stores individual die results and their sum. The 3d6 bell curve
/// means rolls cluster around 10-11 (16.2% chance each).
///
/// # Citations
///
/// BS 10 - 3d6 probability distribution
///
/// # Examples
///
/// ```
/// use valinoreth::{Random, Dice};
///
/// let mut rng = Random::from_seed(42).unwrap();
/// let dice = Dice::from_random(&mut rng);
/// // dice.sum() is between 3-18
/// ```
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_new::new,
    derive_getters::Getters,
)]
pub struct Dice {
    /// First die (1-6)
    d1: usize,
    /// Second die (1-6)
    d2: usize,
    /// Third die (1-6)
    d3: usize,
    /// Sum of all three dice (3-18)
    sum: usize,
}

impl Dice {
    /// Rolls 3d6 using the provided random number generator.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{Random, Dice};
    ///
    /// let mut rng = Random::from_seed(42).unwrap();
    /// let dice = Dice::from_random(&mut rng);
    /// ```
    pub fn from_random(random: &mut Random) -> Self {
        let d1 = random.roll_die();
        let d2 = random.roll_die();
        let d3 = random.roll_die();
        let sum = d1 + d2 + d3;
        Self { d1, d2, d3, sum }
    }
}

/// Damage dice specification.
///
/// # GURPS Rules
///
/// Damage is specified as NdM where:
/// - N = number of dice
/// - M = modifier (pips) added/subtracted
///
/// Examples:
/// - 2d+1: Roll 2 dice, add 1
/// - 3d-2: Roll 3 dice, subtract 2
/// - 1d: Roll 1 die, no modifier
///
/// # Citations
///
/// BS 269 - Damage notation
/// BS 16 - Damage Table
///
/// # Examples
///
/// ```
/// use valinoreth::DieLevel;
///
/// // 2d+1 damage
/// let damage = DieLevel::new(2, 1);
///
/// // 1d-2 damage
/// let damage = DieLevel::new(1, -2);
/// ```
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    derive_new::new,
    derive_getters::Getters,
)]
pub struct DieLevel {
    /// Number of dice to roll
    dice: i64,
    /// Modifier (pips) to add/subtract
    pips: i64,
}

/// Random number generator for dice rolls.
///
/// Wraps the standard RNG with a uniform distribution for 1d6.
/// Provides reproducible results when seeded.
///
/// # Examples
///
/// ```
/// use valinoreth::Random;
///
/// // Seeded for reproducibility
/// let mut rng = Random::from_seed(42).unwrap();
/// let roll = rng.roll_die(); // 1-6
///
/// // Default uses system entropy
/// let mut rng = Random::default();
/// let total = rng.roll(); // 3d6, result 3-18
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Random {
    /// Standard RNG
    range: rand::rngs::StdRng,
    /// Uniform distribution for 1-6
    die: rand::distr::Uniform<usize>,
}

impl Random {
    /// Creates a seeded random number generator.
    ///
    /// Use this for reproducible dice rolls (e.g., testing).
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Random;
    ///
    /// let mut rng = Random::from_seed(42).unwrap();
    /// let roll = rng.roll_die();
    /// ```
    pub fn from_seed(seed: u64) -> Result<Self, rand::distr::uniform::Error> {
        let range = rand::SeedableRng::seed_from_u64(seed);
        let die = rand::distr::Uniform::new(1, 7)?;
        Ok(Self { range, die })
    }

    /// Rolls a single six-sided die (1-6).
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Random;
    ///
    /// let mut rng = Random::from_seed(42).unwrap();
    /// let result = rng.roll_die(); // 1-6
    /// ```
    pub fn roll_die(&mut self) -> usize {
        self.die.sample(&mut self.range)
    }

    /// Rolls 3d6 and returns the sum (3-18).
    ///
    /// # GURPS Rules
    ///
    /// Most GURPS rolls use 3d6, which creates a bell curve
    /// distribution centered around 10-11.
    ///
    /// # Citations
    ///
    /// BS 10 - Success rolls
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Random;
    ///
    /// let mut rng = Random::from_seed(42).unwrap();
    /// let result = rng.roll(); // 3-18
    /// ```
    pub fn roll(&mut self) -> usize {
        (0..3).fold(0, |acc, _| acc + self.die.sample(&mut self.range))
    }
}

impl Default for Random {
    /// Creates a random number generator using system entropy.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Random;
    ///
    /// let mut rng = Random::default();
    /// let roll = rng.roll(); // Unpredictable 3d6
    /// ```
    fn default() -> Self {
        let mut rng = rand::rng();
        let range = <rand::rngs::StdRng as rand::SeedableRng>::from_rng(&mut rng);
        // Quick and dirty method, may panic on ...?
        let die = rand::distr::Uniform::new(1, 7).unwrap();
        Self { range, die }
    }
}
