use rand::distr::Distribution;

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
)]
pub struct Dice {
    d1: usize,
    d2: usize,
    d3: usize,
    sum: usize,
}

impl Dice {
    pub fn from_random(random: &mut Random) -> Self {
        let d1 = random.roll_die();
        let d2 = random.roll_die();
        let d3 = random.roll_die();
        let sum = d1 + d2 + d3;
        Self { d1, d2, d3, sum }
    }
}

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
)]
pub struct DieLevel {
    dice: i64,
    pips: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Random {
    range: rand::rngs::StdRng,
    die: rand::distr::Uniform<usize>,
}

impl Random {
    pub fn from_seed(seed: u64) -> Result<Self, rand::distr::uniform::Error> {
        let range = rand::SeedableRng::seed_from_u64(seed);
        let die = rand::distr::Uniform::new(1, 7)?;
        Ok(Self { range, die })
    }

    pub fn roll_die(&mut self) -> usize {
        self.die.sample(&mut self.range)
    }

    pub fn roll(&mut self) -> usize {
        // let mut total = 0;
        // for _ in 0..3 {
        //     total += self.die.sample(&mut self.range)
        // }
        // total
        (0..3).fold(0, |acc, _| acc + self.die.sample(&mut self.range))
    }
}

impl Default for Random {
    fn default() -> Self {
        let mut rng = rand::rng();
        let range = <rand::rngs::StdRng as rand::SeedableRng>::from_rng(&mut rng);
        // Quick and dirty method, may panic on ...?
        let die = rand::distr::Uniform::new(1, 7).unwrap();
        Self { range, die }
    }
}
