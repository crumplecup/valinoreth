//! Weapon definitions and statistics.
//!
//! # GURPS Rules
//!
//! Weapons have standardized statistics including damage, reach, required
//! skill, parry modifier (for melee), and additional stats for ranged weapons.
//!
//! # Citations
//!
//! - BS 269-271 - Damage types and effects
//! - BS 271-276 - Melee weapons table
//! - BS 276-278 - Ranged weapons table

use crate::{DieLevel, Skill};
use tracing::instrument;

/// Weapon type (melee or ranged).
///
/// # Examples
///
/// ```
/// use valinoreth::{Weapon, MeleeWeapon};
///
/// let weapon = Weapon::Melee(MeleeWeapon::Broadsword);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Weapon {
    /// Melee weapon
    Melee(MeleeWeapon),
    /// Ranged weapon
    Ranged(RangedWeapon),
}

/// Melee weapon types.
///
/// # GURPS Rules
///
/// Melee weapons use ST-based damage (thrust or swing) with modifiers.
/// Each weapon has reach, parry modifier, and required skill.
///
/// # Citations
///
/// BS 271-276 - Melee weapons table
///
/// # Examples
///
/// ```
/// use valinoreth::MeleeWeapon;
///
/// let sword = MeleeWeapon::Broadsword;
/// let damage = sword.damage();
/// ```
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    strum::EnumIter,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum MeleeWeapon {
    /// Axe, swing+2 cutting. BS 271
    Axe,
    /// Baton/Billy Club, swing crushing. BS 271
    Baton,
    /// Brass Knuckles, thrust crushing. BS 271
    BrassKnuckles,
    /// Broadsword, swing+1 cutting or thrust+1 crushing. BS 271
    Broadsword,
    /// Dagger, thrust-1 impaling or swing-1 cutting. BS 271
    Dagger,
    /// Fist (unarmed), thrust-1 crushing. BS 271
    Fist,
    /// Flail, swing+2 crushing. BS 272
    Flail,
    /// Great Axe, swing+3 cutting. BS 272
    GreatAxe,
    /// Halberd, swing+3 cutting or thrust+3 impaling. BS 272
    Halberd,
    /// Hatchet, swing+1 cutting. BS 272
    Hatchet,
    /// Javelin, thrust+1 impaling. BS 272
    Javelin,
    /// Kick, thrust crushing. BS 272
    Kick,
    /// Knife, thrust-1 impaling or swing-2 cutting. BS 273
    Knife,
    /// Kusari (chain), swing+1 crushing. BS 273
    Kusari,
    /// Lance, thrust+3 impaling. BS 273
    Lance,
    /// Long Spear, thrust+2 impaling. BS 273
    LongSpear,
    /// Mace, swing+2 crushing. BS 273
    Mace,
    /// Main-Gauche, thrust-1 impaling. BS 273
    MainGauche,
    /// Morningstar, swing+3 crushing. BS 274
    Morningstar,
    /// Quarterstaff, swing+2 crushing or thrust+2 crushing. BS 274
    Quarterstaff,
    /// Rapier, thrust+1 impaling. BS 274
    Rapier,
    /// Saber, swing+1 cutting or thrust impaling. BS 274
    Saber,
    /// Shortsword, swing cutting or thrust impaling. BS 275
    Shortsword,
    /// Smallsword, thrust impaling. BS 275
    Smallsword,
    /// Spear, thrust+2 impaling or swing+2 cutting. BS 275
    Spear,
    /// Staff, swing+1 crushing or thrust+1 crushing. BS 275
    Staff,
    /// Two-Handed Sword, swing+2 cutting or thrust+2 impaling. BS 276
    TwoHandedSword,
    /// Warhammer, swing+3 impaling. BS 276
    Warhammer,
}

impl MeleeWeapon {
    /// Returns weapon damage specification.
    ///
    /// # GURPS Rules
    ///
    /// Melee weapons modify ST-based thrust or swing damage.
    ///
    /// # Citations
    ///
    /// BS 271-276 - Weapon damage by type
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{MeleeWeapon, WeaponDamage, DamageType};
    ///
    /// let sword = MeleeWeapon::Broadsword;
    /// let damage = sword.damage();
    /// ```
    #[instrument]
    pub fn damage(&self) -> WeaponDamage {
        match self {
            Self::Axe => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
            Self::Baton => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
            Self::BrassKnuckles => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
            Self::Broadsword => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::Dagger => WeaponDamage::Thrust {
                modifier: -1,
                damage_type: DamageType::Impaling,
            },
            Self::Fist => WeaponDamage::Thrust {
                modifier: -1,
                damage_type: DamageType::Crushing,
            },
            Self::Flail => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Crushing,
            },
            Self::GreatAxe => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Cutting,
            },
            Self::Halberd => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Cutting,
            },
            Self::Hatchet => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::Javelin => WeaponDamage::Thrust {
                modifier: 1,
                damage_type: DamageType::Impaling,
            },
            Self::Kick => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Crushing,
            },
            Self::Knife => WeaponDamage::Thrust {
                modifier: -1,
                damage_type: DamageType::Impaling,
            },
            Self::Kusari => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::Lance => WeaponDamage::Thrust {
                modifier: 3,
                damage_type: DamageType::Impaling,
            },
            Self::LongSpear => WeaponDamage::Thrust {
                modifier: 2,
                damage_type: DamageType::Impaling,
            },
            Self::Mace => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Crushing,
            },
            Self::MainGauche => WeaponDamage::Thrust {
                modifier: -1,
                damage_type: DamageType::Impaling,
            },
            Self::Morningstar => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Crushing,
            },
            Self::Quarterstaff => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Crushing,
            },
            Self::Rapier => WeaponDamage::Thrust {
                modifier: 1,
                damage_type: DamageType::Impaling,
            },
            Self::Saber => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Cutting,
            },
            Self::Shortsword => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Cutting,
            },
            Self::Smallsword => WeaponDamage::Thrust {
                modifier: 0,
                damage_type: DamageType::Impaling,
            },
            Self::Spear => WeaponDamage::Thrust {
                modifier: 2,
                damage_type: DamageType::Impaling,
            },
            Self::Staff => WeaponDamage::Swing {
                modifier: 1,
                damage_type: DamageType::Crushing,
            },
            Self::TwoHandedSword => WeaponDamage::Swing {
                modifier: 2,
                damage_type: DamageType::Cutting,
            },
            Self::Warhammer => WeaponDamage::Swing {
                modifier: 3,
                damage_type: DamageType::Impaling,
            },
        }
    }

    /// Returns weapon reach.
    ///
    /// # GURPS Rules
    ///
    /// Reach determines how far away you can strike. C means close combat
    /// (grappling range), numbers are meters.
    ///
    /// # Citations
    ///
    /// BS 271 - Reach definition
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::MeleeWeapon;
    ///
    /// let spear = MeleeWeapon::Spear;
    /// let reach = spear.reach();
    /// ```
    #[instrument]
    pub fn reach(&self) -> Reach {
        match self {
            Self::Axe => Reach::One,
            Self::Baton => Reach::One,
            Self::BrassKnuckles => Reach::Close,
            Self::Broadsword => Reach::One,
            Self::Dagger => Reach::Close,
            Self::Fist => Reach::Close,
            Self::Flail => Reach::OneTwo,
            Self::GreatAxe => Reach::OneTwo,
            Self::Halberd => Reach::TwoThree,
            Self::Hatchet => Reach::One,
            Self::Javelin => Reach::One,
            Self::Kick => Reach::CloseOne,
            Self::Knife => Reach::Close,
            Self::Kusari => Reach::OneThree,
            Self::Lance => Reach::Three,
            Self::LongSpear => Reach::TwoThree,
            Self::Mace => Reach::One,
            Self::MainGauche => Reach::One,
            Self::Morningstar => Reach::One,
            Self::Quarterstaff => Reach::OneTwo,
            Self::Rapier => Reach::One,
            Self::Saber => Reach::One,
            Self::Shortsword => Reach::One,
            Self::Smallsword => Reach::One,
            Self::Spear => Reach::OneTwo,
            Self::Staff => Reach::OneTwo,
            Self::TwoHandedSword => Reach::OneTwo,
            Self::Warhammer => Reach::OneTwo,
        }
    }

    /// Returns parry modifier.
    ///
    /// # GURPS Rules
    ///
    /// Parry = (Weapon Skill / 2) + 3 + parry_modifier
    ///
    /// # Citations
    ///
    /// BS 376 - Parry calculation
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::MeleeWeapon;
    ///
    /// let rapier = MeleeWeapon::Rapier;
    /// assert_eq!(rapier.parry_modifier(), 1);
    /// ```
    #[instrument]
    pub fn parry_modifier(&self) -> i32 {
        match self {
            Self::Axe => -1,
            Self::Baton => 0,
            Self::BrassKnuckles => 0,
            Self::Broadsword => 0,
            Self::Dagger => -1,
            Self::Fist => 0,
            Self::Flail => -2,
            Self::GreatAxe => -2,
            Self::Halberd => 0,
            Self::Hatchet => -1,
            Self::Javelin => 0,
            Self::Kick => -2,
            Self::Knife => -1,
            Self::Kusari => -2,
            Self::Lance => -2,
            Self::LongSpear => 0,
            Self::Mace => 0,
            Self::MainGauche => 1,
            Self::Morningstar => 0,
            Self::Quarterstaff => 2,
            Self::Rapier => 1,
            Self::Saber => 0,
            Self::Shortsword => 0,
            Self::Smallsword => 1,
            Self::Spear => 0,
            Self::Staff => 2,
            Self::TwoHandedSword => 0,
            Self::Warhammer => -1,
        }
    }

    /// Returns required skill for this weapon.
    ///
    /// # GURPS Rules
    ///
    /// Each weapon requires a specific skill to use effectively.
    ///
    /// # Citations
    ///
    /// BS 271-276 - Weapon skills
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{MeleeWeapon, Skill};
    ///
    /// let sword = MeleeWeapon::Broadsword;
    /// assert_eq!(sword.required_skill(), Skill::Broadsword);
    /// ```
    #[instrument]
    pub fn required_skill(&self) -> Skill {
        match self {
            Self::Axe => Skill::AxeMace,
            Self::Baton => Skill::Shortsword,
            Self::BrassKnuckles => Skill::Brawling,
            Self::Broadsword => Skill::Broadsword,
            Self::Dagger => Skill::Knife,
            Self::Fist => Skill::Brawling,
            Self::Flail => Skill::Flail,
            Self::GreatAxe => Skill::TwoHandedAxeMace,
            Self::Halberd => Skill::Polearm,
            Self::Hatchet => Skill::AxeMace,
            Self::Javelin => Skill::Spear,
            Self::Kick => Skill::Brawling,
            Self::Knife => Skill::Knife,
            Self::Kusari => Skill::Kusari,
            Self::Lance => Skill::Lance,
            Self::LongSpear => Skill::Spear,
            Self::Mace => Skill::AxeMace,
            Self::MainGauche => Skill::MainGauche,
            Self::Morningstar => Skill::Flail,
            Self::Quarterstaff => Skill::Staff,
            Self::Rapier => Skill::Rapier,
            Self::Saber => Skill::Saber,
            Self::Shortsword => Skill::Shortsword,
            Self::Smallsword => Skill::Smallsword,
            Self::Spear => Skill::Spear,
            Self::Staff => Skill::Staff,
            Self::TwoHandedSword => Skill::TwoHandedSword,
            Self::Warhammer => Skill::TwoHandedAxeMace,
        }
    }
}

/// Ranged weapon types.
///
/// # GURPS Rules
///
/// Ranged weapons have fixed damage, accuracy, rate of fire, range,
/// shots, and recoil.
///
/// # Citations
///
/// BS 276-278 - Ranged weapons table
///
/// # Examples
///
/// ```
/// use valinoreth::RangedWeapon;
///
/// let bow = RangedWeapon::Bow;
/// let damage = bow.damage();
/// ```
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    strum::EnumIter,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum RangedWeapon {
    /// Bow, 1d impaling. BS 276
    Bow,
    /// Crossbow, 1d+4 impaling. BS 276
    Crossbow,
    /// Pistol (9mm), 2d+2 piercing. BS 278
    Pistol,
    /// Rifle (.30), 5d piercing. BS 278
    Rifle,
    /// Shotgun (12ga), 1d+1 piercing. BS 278
    Shotgun,
    /// Sling, swing piercing. BS 277
    Sling,
    /// Throwing Knife, thrust-1 impaling. BS 277
    ThrowingKnife,
}

impl RangedWeapon {
    /// Returns weapon damage specification.
    ///
    /// # GURPS Rules
    ///
    /// Ranged weapons have fixed damage dice.
    ///
    /// # Citations
    ///
    /// BS 276-278 - Ranged weapon damage
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::RangedWeapon;
    ///
    /// let bow = RangedWeapon::Bow;
    /// let damage = bow.damage();
    /// ```
    #[instrument]
    pub fn damage(&self) -> WeaponDamage {
        match self {
            Self::Bow => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 0),
                damage_type: DamageType::Impaling,
            },
            Self::Crossbow => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 4),
                damage_type: DamageType::Impaling,
            },
            Self::Pistol => WeaponDamage::Fixed {
                dice: DieLevel::new(2, 2),
                damage_type: DamageType::Piercing,
            },
            Self::Rifle => WeaponDamage::Fixed {
                dice: DieLevel::new(5, 0),
                damage_type: DamageType::Piercing,
            },
            Self::Shotgun => WeaponDamage::Fixed {
                dice: DieLevel::new(1, 1),
                damage_type: DamageType::Piercing,
            },
            Self::Sling => WeaponDamage::Swing {
                modifier: 0,
                damage_type: DamageType::Piercing,
            },
            Self::ThrowingKnife => WeaponDamage::Thrust {
                modifier: -1,
                damage_type: DamageType::Impaling,
            },
        }
    }

    /// Returns weapon accuracy bonus.
    ///
    /// # GURPS Rules
    ///
    /// Accuracy adds to skill when aiming. Maximum bonus from
    /// multiple aim actions cannot exceed Acc.
    ///
    /// # Citations
    ///
    /// BS 364 - Aim maneuver and Accuracy
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::RangedWeapon;
    ///
    /// let rifle = RangedWeapon::Rifle;
    /// assert_eq!(rifle.accuracy(), 5);
    /// ```
    #[instrument]
    pub fn accuracy(&self) -> i32 {
        match self {
            Self::Bow => 2,
            Self::Crossbow => 4,
            Self::Pistol => 2,
            Self::Rifle => 5,
            Self::Shotgun => 3,
            Self::Sling => 0,
            Self::ThrowingKnife => 0,
        }
    }

    /// Returns required skill for this weapon.
    ///
    /// # Citations
    ///
    /// BS 276-278 - Ranged weapon skills
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::{RangedWeapon, Skill};
    ///
    /// let bow = RangedWeapon::Bow;
    /// assert_eq!(bow.required_skill(), Skill::Bow);
    /// ```
    #[instrument]
    pub fn required_skill(&self) -> Skill {
        match self {
            Self::Bow => Skill::Bow,
            Self::Crossbow => Skill::Crossbow,
            Self::Pistol => Skill::Guns,
            Self::Rifle => Skill::Guns,
            Self::Shotgun => Skill::Guns,
            Self::Sling => Skill::Sling,
            Self::ThrowingKnife => Skill::ThrownWeapon,
        }
    }
}

/// Weapon damage specification.
///
/// # GURPS Rules
///
/// Damage can be ST-based (thrust/swing) or fixed dice.
///
/// # Citations
///
/// BS 269-271 - Damage types
///
/// # Examples
///
/// ```
/// use valinoreth::{WeaponDamage, DamageType};
///
/// let damage = WeaponDamage::Swing { modifier: 1, damage_type: DamageType::Cutting };
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WeaponDamage {
    /// Thrust-based damage with modifier
    Thrust {
        /// Damage modifier (e.g., +1, -2)
        modifier: i32,
        /// Type of damage
        damage_type: DamageType,
    },
    /// Swing-based damage with modifier
    Swing {
        /// Damage modifier (e.g., +1, -2)
        modifier: i32,
        /// Type of damage
        damage_type: DamageType,
    },
    /// Fixed dice damage (for ranged weapons)
    Fixed {
        /// Damage dice (e.g., 2d+1)
        dice: DieLevel,
        /// Type of damage
        damage_type: DamageType,
    },
}

/// Damage type and wound multiplier.
///
/// # GURPS Rules
///
/// Different damage types have different wound multipliers when
/// applied to hit locations.
///
/// # Citations
///
/// BS 269-271 - Damage types and multipliers
///
/// # Examples
///
/// ```
/// use valinoreth::DamageType;
///
/// let cutting = DamageType::Cutting;
/// assert_eq!(cutting.torso_multiplier(), 1.5);
/// ```
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    strum::EnumIter,
    derive_more::Display,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum DamageType {
    /// Crushing damage, ×1 to torso. BS 269
    Crushing,
    /// Cutting damage, ×1.5 to torso. BS 270
    Cutting,
    /// Impaling damage, ×2 to torso. BS 270
    Impaling,
    /// Small piercing damage, ×0.5 to torso. BS 270
    PiercingSmall,
    /// Normal piercing damage, ×1 to torso. BS 270
    Piercing,
    /// Large piercing damage, ×1.5 to torso. BS 270
    PiercingLarge,
}

impl DamageType {
    /// Returns wound multiplier for torso hits.
    ///
    /// # GURPS Rules
    ///
    /// Torso multipliers vary by damage type.
    ///
    /// # Citations
    ///
    /// BS 269-271 - Wound multipliers
    ///
    /// # Examples
    ///
    /// ```
    /// use valinoreth::DamageType;
    ///
    /// assert_eq!(DamageType::Cutting.torso_multiplier(), 1.5);
    /// assert_eq!(DamageType::Crushing.torso_multiplier(), 1.0);
    /// assert_eq!(DamageType::Impaling.torso_multiplier(), 2.0);
    /// ```
    #[instrument]
    pub fn torso_multiplier(&self) -> f64 {
        match self {
            Self::Crushing => 1.0,
            Self::Cutting => 1.5,
            Self::Impaling => 2.0,
            Self::PiercingSmall => 0.5,
            Self::Piercing => 1.0,
            Self::PiercingLarge => 1.5,
        }
    }
}

/// Weapon reach specification.
///
/// # GURPS Rules
///
/// Reach indicates how far you can strike with a weapon.
/// C means close combat (grappling), numbers are meters.
///
/// # Citations
///
/// BS 271 - Reach definition
///
/// # Examples
///
/// ```
/// use valinoreth::Reach;
///
/// let spear_reach = Reach::OneTwo;  // Can strike at 1 or 2 meters
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, derive_more::Display)]
pub enum Reach {
    /// Close combat only (C)
    #[display("C")]
    Close,
    /// Close or 1 meter (C,1)
    #[display("C,1")]
    CloseOne,
    /// 1 meter only (1)
    #[display("1")]
    One,
    /// 1 or 2 meters (1,2)
    #[display("1,2")]
    OneTwo,
    /// 1, 2, or 3 meters (1-3)
    #[display("1-3")]
    OneThree,
    /// 2 or 3 meters (2,3)
    #[display("2,3")]
    TwoThree,
    /// 3 meters only (3)
    #[display("3")]
    Three,
}
