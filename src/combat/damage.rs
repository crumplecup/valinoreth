//! Damage resolution and injury mechanics.
//!
//! # GURPS Rules
//!
//! Damage resolution follows this sequence:
//! 1. Roll weapon damage dice
//! 2. Add ST-based modifier (swing/thrust for melee)
//! 3. Apply hit location multiplier
//! 4. Subtract Damage Resistance (DR)
//! 5. Calculate HP loss
//!
//! # Citations
//!
//! - BS 378 - Damage and injury
//! - BS 379 - Damage resistance
//! - BS 398-399 - Hit location
//! - BS 552-553 - Hit location table

use crate::{BaseDamage, BodyLocation, DamageKind, DieLevel, Random, WeaponDamage};
use derive_getters::Getters;
use tracing::{debug, info, instrument};

/// Complete damage resolution result.
///
/// # GURPS Rules
///
/// Tracks the full damage calculation pipeline from initial roll
/// through location multipliers and DR to final HP loss.
///
/// # Citations
///
/// BS 378-380 - Damage and injury
///
/// # Examples
///
/// ```
/// use valinoreth::{DamageResolution, BaseDamage, DamageKind, WeaponDamage, DamageType, Torso, Random, DieLevel};
///
/// let mut rng = Random::from_seed(42).unwrap();
/// // ST 10: 1d-2 thrust, 1d swing
/// let base_damage = BaseDamage::new(
///     DamageKind::Thrust(DieLevel::new(1, -2)),
///     DamageKind::Swing(DieLevel::new(1, 0))
/// );
/// let weapon_damage = WeaponDamage::Thrust {
///     modifier: 1,
///     damage_type: DamageType::Impaling,
/// };
/// let torso = Torso::Chest;
/// let dr = 3;
///
/// let resolution = DamageResolution::resolve(
///     &base_damage,
///     &weapon_damage,
///     &torso,
///     dr,
///     &mut rng,
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Getters, derive_new::new)]
pub struct DamageResolution {
    /// Raw damage rolled before modifiers
    raw_damage: i32,
    /// Damage after ST-based modifiers
    modified_damage: i32,
    /// Hit location multiplier applied
    location_multiplier: f64,
    /// Damage after location multiplier
    location_damage: i32,
    /// Damage Resistance value
    dr: i32,
    /// Damage that penetrated DR
    penetrating_damage: i32,
    /// Final HP loss after all calculations
    hp_lost: i32,
}

impl DamageResolution {
    /// Resolves complete damage calculation.
    ///
    /// # GURPS Rules
    ///
    /// Resolution sequence:
    /// 1. Roll base dice for weapon
    /// 2. Add ST-based modifiers (swing/thrust)
    /// 3. Apply hit location multiplier (torso x2 for impaling, etc.)
    /// 4. Subtract DR (armor protection)
    /// 5. Calculate final HP loss (minimum 0)
    ///
    /// # Citations
    ///
    /// BS 378 - Damage calculation
    /// BS 269-271 - Damage types and multipliers
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{DamageResolution, BaseDamage, DamageKind, WeaponDamage, DamageType, Torso, Random, DieLevel};
    ///
    /// let mut rng = Random::from_seed(42).unwrap();
    /// let base_damage = BaseDamage::new(
    ///     DamageKind::Thrust(DieLevel::new(1, -2)),
    ///     DamageKind::Swing(DieLevel::new(1, 0))
    /// );
    /// let weapon = WeaponDamage::Thrust {
    ///     modifier: 1,
    ///     damage_type: DamageType::Impaling,
    /// };
    ///
    /// let result = DamageResolution::resolve(
    ///     &base_damage,
    ///     &weapon,
    ///     &Torso::Chest,
    ///     3,
    ///     &mut rng,
    /// );
    /// ```
    #[instrument(skip(base_damage, weapon_damage, location, random), fields(dr))]
    pub fn resolve(
        base_damage: &BaseDamage,
        weapon_damage: &WeaponDamage,
        location: &impl BodyLocation,
        dr: i32,
        random: &mut Random,
    ) -> Self {
        debug!("Starting damage resolution");

        // Step 1: Determine base dice to roll
        let dice = Self::calculate_damage_dice(base_damage, weapon_damage);
        debug!(dice = ?dice, "Calculated damage dice");

        // Step 2: Roll the dice
        let raw_damage = Self::roll_damage(&dice, random);
        info!(raw_damage, "Rolled raw damage");

        // Step 3: Apply weapon modifier
        let modified_damage = Self::apply_weapon_modifier(raw_damage, weapon_damage);
        debug!(modified_damage, "Applied weapon modifier");

        // Step 4: Get location multiplier
        let location_multiplier = Self::get_location_multiplier(weapon_damage, location);
        debug!(location_multiplier, "Got location multiplier");

        // Step 5: Apply location multiplier
        let location_damage = Self::apply_location_multiplier(modified_damage, location_multiplier);
        info!(location_damage, "Applied location multiplier");

        // Step 6: Subtract DR
        let penetrating_damage = Self::apply_dr(location_damage, dr);
        debug!(penetrating_damage, "Calculated penetrating damage");

        // Step 7: Calculate HP loss (minimum 0)
        let hp_lost = penetrating_damage.max(0);
        info!(hp_lost, "Final HP loss calculated");

        Self::new(
            raw_damage,
            modified_damage,
            location_multiplier,
            location_damage,
            dr,
            penetrating_damage,
            hp_lost,
        )
    }

    /// Calculates which damage dice to roll based on weapon type.
    ///
    /// # Citations
    ///
    /// BS 16 - Damage table
    /// BS 269 - Weapon damage
    #[instrument(skip(base_damage, weapon_damage))]
    fn calculate_damage_dice(base_damage: &BaseDamage, weapon_damage: &WeaponDamage) -> DieLevel {
        debug!("Calculating damage dice");

        let dice = match weapon_damage {
            WeaponDamage::Thrust { .. } => match base_damage.thrust() {
                DamageKind::Thrust(die_level) => *die_level,
                DamageKind::Swing(die_level) => *die_level,
            },
            WeaponDamage::Swing { .. } => match base_damage.swing() {
                DamageKind::Thrust(die_level) => *die_level,
                DamageKind::Swing(die_level) => *die_level,
            },
            WeaponDamage::Fixed { dice, .. } => *dice,
        };

        debug!(dice = ?dice, "Damage dice determined");
        dice
    }

    /// Rolls damage dice and returns total (including pips).
    ///
    /// # Citations
    ///
    /// BS 269 - Rolling damage
    #[instrument(skip(dice, random))]
    fn roll_damage(dice: &DieLevel, random: &mut Random) -> i32 {
        debug!(
            dice_count = dice.dice(),
            pips = dice.pips(),
            "Rolling damage"
        );

        let mut total = 0i32;
        for _ in 0..*dice.dice() {
            total += random.roll_die() as i32;
        }

        // Add the pips modifier from DieLevel (e.g., +2 in 1d+2)
        total += *dice.pips() as i32;

        debug!(total, "Dice rolled with pips");
        total
    }

    /// Applies weapon damage modifier.
    ///
    /// # Citations
    ///
    /// BS 269 - Damage modifiers
    #[instrument]
    fn apply_weapon_modifier(raw_damage: i32, weapon_damage: &WeaponDamage) -> i32 {
        debug!(raw_damage, "Applying weapon modifier");

        let modifier = match weapon_damage {
            WeaponDamage::Thrust { modifier, .. } => *modifier,
            WeaponDamage::Swing { modifier, .. } => *modifier,
            WeaponDamage::Fixed { dice, .. } => *dice.pips() as i32,
        };

        let modified = raw_damage + modifier;
        debug!(modifier, modified, "Weapon modifier applied");
        modified
    }

    /// Gets location multiplier for damage type and hit location.
    ///
    /// # GURPS Rules
    ///
    /// For now, uses torso multipliers as baseline. Full location-specific
    /// multipliers (head ×4, vitals ×3, etc.) will be added later.
    ///
    /// # Citations
    ///
    /// BS 269-271 - Damage type multipliers
    /// BS 398-399 - Hit locations
    #[instrument(skip(weapon_damage, _location))]
    fn get_location_multiplier(weapon_damage: &WeaponDamage, _location: &impl BodyLocation) -> f64 {
        debug!("Getting location multiplier");

        let damage_type = match weapon_damage {
            WeaponDamage::Thrust { damage_type, .. } => damage_type,
            WeaponDamage::Swing { damage_type, .. } => damage_type,
            WeaponDamage::Fixed { damage_type, .. } => damage_type,
        };

        // Use torso multiplier as baseline for now
        let multiplier = damage_type.torso_multiplier();
        debug!(multiplier, "Location multiplier retrieved");
        multiplier
    }

    /// Applies location multiplier to damage.
    ///
    /// # Citations
    ///
    /// BS 269-271 - Wounding modifiers
    #[instrument]
    fn apply_location_multiplier(damage: i32, multiplier: f64) -> i32 {
        debug!(damage, multiplier, "Applying location multiplier");

        let result = (damage as f64 * multiplier).round() as i32;
        debug!(result, "Location multiplier applied");
        result
    }

    /// Subtracts DR from damage.
    ///
    /// # GURPS Rules
    ///
    /// DR (Damage Resistance) from armor reduces penetrating damage.
    /// Can result in negative values (no penetration).
    ///
    /// # Citations
    ///
    /// BS 379 - Damage resistance
    #[instrument]
    fn apply_dr(damage: i32, dr: i32) -> i32 {
        debug!(damage, dr, "Applying DR");

        let penetrating = damage - dr;
        info!(penetrating, "DR applied");
        penetrating
    }
}

/// Armor providing damage resistance.
///
/// # GURPS Rules
///
/// Armor provides DR that reduces incoming damage before HP loss.
/// Different armor types have different DR values.
///
/// # Citations
///
/// BS 379 - Armor and DR
///
/// # Examples
///
/// ```
/// use valinoreth::Armor;
///
/// let chainmail = Armor::builder()
///     .name("Chainmail")
///     .dr(4)
///     .build()
///     .unwrap();
///
/// assert_eq!(*chainmail.dr(), 4);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Getters, derive_builder::Builder)]
#[builder(setter(into))]
pub struct Armor {
    /// Armor name
    name: String,
    /// Damage Resistance value
    dr: i32,
}

impl Armor {
    /// Creates a new armor builder.
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::Armor;
    ///
    /// let plate = Armor::builder()
    ///     .name("Plate Armor")
    ///     .dr(6)
    ///     .build()
    ///     .unwrap();
    /// ```
    pub fn builder() -> ArmorBuilder {
        ArmorBuilder::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_armor_builder() {
        let armor = Armor::builder().name("Leather").dr(1).build().unwrap();

        assert_eq!(armor.name(), "Leather");
        assert_eq!(*armor.dr(), 1);
    }
}
