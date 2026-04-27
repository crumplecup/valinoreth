//! Active defense mechanics.
//!
//! # GURPS Rules
//!
//! Characters can actively defend against attacks using:
//! - Dodge: Based on Basic Speed, avoid any attack
//! - Parry: Based on weapon skill, deflect melee attacks
//! - Block: Based on shield skill, stop attacks with shield
//!
//! # Citations
//!
//! - BS 374 - Active defenses overview
//! - BS 374-375 - Dodge
//! - BS 376 - Parry
//! - BS 377 - Block

use crate::{Item, Random};
use tracing::{debug, info, instrument};

/// Active defense type and value.
///
/// # GURPS Rules
///
/// On their turn, defenders can choose one active defense against
/// each attack. Each defense type has different characteristics:
/// - Dodge: Works against any attack, no weapon needed
/// - Parry: Only melee, requires weapon, can't parry flails/whips
/// - Block: Any attack, requires shield, DB bonus applies
///
/// # Citations
///
/// BS 374 - Active defense types
///
/// # Examples
///
/// ```
/// use valinoreth::ActiveDefense;
///
/// let dodge = ActiveDefense::Dodge { value: 8 };
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActiveDefense {
    /// Dodge defense (Basic Speed + 3). BS 374-375
    Dodge {
        /// Dodge value to roll under
        value: i32,
    },
    /// Parry defense (Weapon Skill / 2 + 3). BS 376
    Parry {
        /// Parry value to roll under
        value: i32,
        /// Weapon used for parry (must be a melee weapon item)
        weapon: Item,
    },
    /// Block defense (Shield Skill / 2 + 3). BS 377
    Block {
        /// Block value to roll under
        value: i32,
    },
}

impl ActiveDefense {
    /// Returns the defense value to roll against.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::ActiveDefense;
    ///
    /// let dodge = ActiveDefense::Dodge { value: 8 };
    /// assert_eq!(dodge.value(), 8);
    /// ```
    #[instrument(skip(self))]
    pub fn value(&self) -> i32 {
        debug!("Getting defense value");

        let val = match self {
            Self::Dodge { value } => *value,
            Self::Parry { value, .. } => *value,
            Self::Block { value } => *value,
        };

        debug!(value = val, "Defense value retrieved");
        val
    }

    /// Attempts an active defense roll.
    ///
    /// # GURPS Rules
    ///
    /// Roll 3d6 ≤ defense value to successfully defend.
    /// - Success: Attack is avoided/deflected/blocked
    /// - Failure: Attack proceeds to damage resolution
    ///
    /// # Citations
    ///
    /// BS 374 - Defense rolls
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{ActiveDefense, Random};
    ///
    /// let dodge = ActiveDefense::Dodge { value: 8 };
    /// let mut rng = Random::from_seed(42).unwrap();
    /// let result = dodge.roll(&mut rng);
    /// ```
    #[instrument(skip(self, random))]
    pub fn roll(&self, random: &mut Random) -> DefenseResult {
        debug!("Rolling active defense");

        let defense_value = self.value();
        let roll = random.roll();

        let result = if roll <= defense_value as usize {
            info!(roll, defense_value, "Defense succeeded");
            DefenseResult::Success {
                margin: defense_value - roll as i32,
            }
        } else {
            debug!(roll, defense_value, "Defense failed");
            DefenseResult::Failure {
                margin: roll as i32 - defense_value,
            }
        };

        debug!(?result, "Defense roll completed");
        result
    }
}

/// Result of a defense roll.
///
/// # GURPS Rules
///
/// Defense rolls use 3d6 ≤ defense value.
///
/// # Citations
///
/// BS 374 - Defense success/failure
///
/// # Examples
///
/// ```
/// use valinoreth::DefenseResult;
///
/// let success = DefenseResult::Success { margin: 3 };
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DefenseResult {
    /// Defense succeeded
    Success {
        /// Margin of success (defense - roll)
        margin: i32,
    },
    /// Defense failed
    Failure {
        /// Margin of failure (roll - defense)
        margin: i32,
    },
}

/// Calculates parry defense value from weapon skill.
///
/// # GURPS Rules
///
/// Parry = (Weapon Skill / 2) + 3 + weapon parry modifier
///
/// Parry cannot defend against:
/// - Ranged attacks
/// - Flails or whips (unless you have the Parry skill)
///
/// # Citations
///
/// BS 376 - Parry calculation
///
/// # Examples
///
/// ```
/// use valinoreth::{calculate_parry, Item, MeleeWeapon};
///
/// // Broadsword skill 14, parry modifier 0
/// let parry = calculate_parry(14, &Item::MeleeWeapon(MeleeWeapon::Broadsword));
/// assert_eq!(parry, 10); // (14/2) + 3 + 0 = 10
/// ```
#[instrument]
pub fn calculate_parry(skill: i32, weapon: &Item) -> i32 {
    debug!(skill, ?weapon, "Calculating parry value");

    let weapon_modifier = weapon
        .parry_modifier()
        .expect("Parry calculation requires a melee weapon item");
    let base_parry = (skill / 2) + 3;
    let parry = base_parry + weapon_modifier;

    info!(skill, weapon_modifier, parry, "Parry calculated");
    parry
}

/// Calculates block defense value from shield skill.
///
/// # GURPS Rules
///
/// Block = (Shield Skill / 2) + 3
///
/// Block can defend against any attack, including ranged.
/// Shields also provide Damage Resistance (DR).
///
/// # Citations
///
/// BS 377 - Block calculation
///
/// # Examples
///
/// ```
/// use valinoreth::calculate_block;
///
/// // Shield skill 12
/// let block = calculate_block(12);
/// assert_eq!(block, 9); // (12/2) + 3 = 9
/// ```
#[instrument]
pub fn calculate_block(skill: i32) -> i32 {
    debug!(skill, "Calculating block value");

    let block = (skill / 2) + 3;

    info!(skill, block, "Block calculated");
    block
}

/// Calculates dodge defense value from basic speed.
///
/// # GURPS Rules
///
/// Dodge = floor(Basic Speed) + 3
///
/// Dodge can defend against any attack. It's reduced by
/// encumbrance and injuries.
///
/// # Citations
///
/// BS 374-375 - Dodge calculation
/// BS 17 - Dodge and encumbrance
///
/// # Examples
///
/// ```
/// use valinoreth::calculate_dodge;
///
/// // Basic Speed 5.75
/// let dodge = calculate_dodge(5.75);
/// assert_eq!(dodge, 8); // floor(5.75) + 3 = 8
/// ```
#[instrument]
pub fn calculate_dodge(basic_speed: f64) -> i32 {
    debug!(basic_speed, "Calculating dodge value");

    let dodge = basic_speed.floor() as i32 + 3;

    info!(basic_speed, dodge, "Dodge calculated");
    dodge
}

/// Retreat bonus for active defenses.
///
/// # GURPS Rules
///
/// You can retreat (move one yard away) to gain +1 to any
/// active defense. You can only retreat once per turn.
///
/// # Citations
///
/// BS 377 - Retreat
///
/// # Examples
///
/// ```
/// use valinoreth::RETREAT_BONUS;
///
/// let base_dodge = 8;
/// let dodge_with_retreat = base_dodge + RETREAT_BONUS;
/// assert_eq!(dodge_with_retreat, 9);
/// ```
pub const RETREAT_BONUS: i32 = 1;

/// Checks if a defense roll succeeds.
///
/// # GURPS Rules
///
/// Defense succeeds if 3d6 ≤ defense value.
///
/// # Citations
///
/// BS 374 - Defense success
///
/// # Examples
///
/// ```
/// use valinoreth::defense_succeeds;
///
/// assert!(defense_succeeds(10, 8));  // Roll 8 vs defense 10
/// assert!(!defense_succeeds(10, 12)); // Roll 12 vs defense 10
/// ```
#[instrument]
pub fn defense_succeeds(defense_value: i32, roll: i32) -> bool {
    debug!(defense_value, roll, "Checking defense success");

    let success = roll <= defense_value;

    if success {
        info!(defense_value, roll, "Defense succeeded");
    } else {
        debug!(defense_value, roll, "Defense failed");
    }

    success
}
