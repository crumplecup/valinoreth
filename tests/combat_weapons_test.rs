//! Tests for weapon definitions and statistics.

use valinoreth::{DamageType, MeleeWeapon, RangedWeapon, Reach, Skill, WeaponDamage};

// ========== Melee Weapon Tests ==========

#[test]
fn test_broadsword_stats() {
    let sword = MeleeWeapon::Broadsword;

    // BS 271: Broadsword is sw+1 cutting
    match sword.damage() {
        WeaponDamage::Swing {
            modifier,
            damage_type,
        } => {
            assert_eq!(modifier, 1);
            assert_eq!(damage_type, DamageType::Cutting);
        }
        _ => panic!("Broadsword should use swing damage"),
    }

    assert_eq!(sword.reach(), Reach::One);
    assert_eq!(sword.parry_modifier(), 0);
    assert_eq!(sword.required_skill(), Skill::Broadsword);
}

#[test]
fn test_rapier_stats() {
    let rapier = MeleeWeapon::Rapier;

    // BS 274: Rapier is thr+1 impaling
    match rapier.damage() {
        WeaponDamage::Thrust {
            modifier,
            damage_type,
        } => {
            assert_eq!(modifier, 1);
            assert_eq!(damage_type, DamageType::Impaling);
        }
        _ => panic!("Rapier should use thrust damage"),
    }

    assert_eq!(rapier.reach(), Reach::One);
    assert_eq!(rapier.parry_modifier(), 1); // Rapier has +1 parry
    assert_eq!(rapier.required_skill(), Skill::Rapier);
}

#[test]
fn test_spear_stats() {
    let spear = MeleeWeapon::Spear;

    // BS 275: Spear is thr+2 impaling
    match spear.damage() {
        WeaponDamage::Thrust {
            modifier,
            damage_type,
        } => {
            assert_eq!(modifier, 2);
            assert_eq!(damage_type, DamageType::Impaling);
        }
        _ => panic!("Spear should use thrust damage"),
    }

    assert_eq!(spear.reach(), Reach::OneTwo); // Can strike at 1 or 2 meters
    assert_eq!(spear.parry_modifier(), 0);
    assert_eq!(spear.required_skill(), Skill::Spear);
}

#[test]
fn test_axe_stats() {
    let axe = MeleeWeapon::Axe;

    // BS 271: Axe is sw+2 cutting
    match axe.damage() {
        WeaponDamage::Swing {
            modifier,
            damage_type,
        } => {
            assert_eq!(modifier, 2);
            assert_eq!(damage_type, DamageType::Cutting);
        }
        _ => panic!("Axe should use swing damage"),
    }

    assert_eq!(axe.reach(), Reach::One);
    assert_eq!(axe.parry_modifier(), -1); // Axe has -1 parry
    assert_eq!(axe.required_skill(), Skill::AxeMace);
}

#[test]
fn test_fist_unarmed_stats() {
    let fist = MeleeWeapon::Fist;

    // BS 271: Fist is thr-1 crushing
    match fist.damage() {
        WeaponDamage::Thrust {
            modifier,
            damage_type,
        } => {
            assert_eq!(modifier, -1);
            assert_eq!(damage_type, DamageType::Crushing);
        }
        _ => panic!("Fist should use thrust damage"),
    }

    assert_eq!(fist.reach(), Reach::Close);
    assert_eq!(fist.parry_modifier(), 0);
    assert_eq!(fist.required_skill(), Skill::Brawling);
}

#[test]
fn test_quarterstaff_stats() {
    let staff = MeleeWeapon::Quarterstaff;

    // BS 274: Quarterstaff is sw+2 crushing
    match staff.damage() {
        WeaponDamage::Swing {
            modifier,
            damage_type,
        } => {
            assert_eq!(modifier, 2);
            assert_eq!(damage_type, DamageType::Crushing);
        }
        _ => panic!("Quarterstaff should use swing damage"),
    }

    assert_eq!(staff.reach(), Reach::OneTwo);
    assert_eq!(staff.parry_modifier(), 2); // Quarterstaff has +2 parry
    assert_eq!(staff.required_skill(), Skill::Staff);
}

#[test]
fn test_knife_stats() {
    let knife = MeleeWeapon::Knife;

    // BS 273: Knife is thr-1 impaling
    match knife.damage() {
        WeaponDamage::Thrust {
            modifier,
            damage_type,
        } => {
            assert_eq!(modifier, -1);
            assert_eq!(damage_type, DamageType::Impaling);
        }
        _ => panic!("Knife should use thrust damage"),
    }

    assert_eq!(knife.reach(), Reach::Close);
    assert_eq!(knife.parry_modifier(), -1);
    assert_eq!(knife.required_skill(), Skill::Knife);
}

#[test]
fn test_two_handed_sword_stats() {
    let sword = MeleeWeapon::TwoHandedSword;

    // BS 276: Two-Handed Sword is sw+2 cutting
    match sword.damage() {
        WeaponDamage::Swing {
            modifier,
            damage_type,
        } => {
            assert_eq!(modifier, 2);
            assert_eq!(damage_type, DamageType::Cutting);
        }
        _ => panic!("Two-Handed Sword should use swing damage"),
    }

    assert_eq!(sword.reach(), Reach::OneTwo);
    assert_eq!(sword.parry_modifier(), 0);
    assert_eq!(sword.required_skill(), Skill::TwoHandedSword);
}

// ========== Ranged Weapon Tests ==========

#[test]
fn test_bow_stats() {
    let bow = RangedWeapon::Bow;

    // BS 276: Bow is 1d impaling
    match bow.damage() {
        WeaponDamage::Fixed { dice, damage_type } => {
            assert_eq!(*dice.dice(), 1);
            assert_eq!(*dice.pips(), 0);
            assert_eq!(damage_type, DamageType::Impaling);
        }
        _ => panic!("Bow should use fixed damage"),
    }

    assert_eq!(bow.accuracy(), 2);
    assert_eq!(bow.required_skill(), Skill::Bow);
}

#[test]
fn test_crossbow_stats() {
    let crossbow = RangedWeapon::Crossbow;

    // BS 276: Crossbow is 1d+4 impaling
    match crossbow.damage() {
        WeaponDamage::Fixed { dice, damage_type } => {
            assert_eq!(*dice.dice(), 1);
            assert_eq!(*dice.pips(), 4);
            assert_eq!(damage_type, DamageType::Impaling);
        }
        _ => panic!("Crossbow should use fixed damage"),
    }

    assert_eq!(crossbow.accuracy(), 4);
    assert_eq!(crossbow.required_skill(), Skill::Crossbow);
}

#[test]
fn test_pistol_stats() {
    let pistol = RangedWeapon::Pistol;

    // BS 278: Pistol (9mm) is 2d+2 piercing
    match pistol.damage() {
        WeaponDamage::Fixed { dice, damage_type } => {
            assert_eq!(*dice.dice(), 2);
            assert_eq!(*dice.pips(), 2);
            assert_eq!(damage_type, DamageType::Piercing);
        }
        _ => panic!("Pistol should use fixed damage"),
    }

    assert_eq!(pistol.accuracy(), 2);
    assert_eq!(pistol.required_skill(), Skill::Guns);
}

#[test]
fn test_rifle_stats() {
    let rifle = RangedWeapon::Rifle;

    // BS 278: Rifle (.30) is 5d piercing
    match rifle.damage() {
        WeaponDamage::Fixed { dice, damage_type } => {
            assert_eq!(*dice.dice(), 5);
            assert_eq!(*dice.pips(), 0);
            assert_eq!(damage_type, DamageType::Piercing);
        }
        _ => panic!("Rifle should use fixed damage"),
    }

    assert_eq!(rifle.accuracy(), 5);
    assert_eq!(rifle.required_skill(), Skill::Guns);
}

#[test]
fn test_sling_st_based_damage() {
    let sling = RangedWeapon::Sling;

    // BS 277: Sling uses swing piercing (ST-based)
    match sling.damage() {
        WeaponDamage::Swing {
            modifier,
            damage_type,
        } => {
            assert_eq!(modifier, 0);
            assert_eq!(damage_type, DamageType::Piercing);
        }
        _ => panic!("Sling should use swing damage"),
    }

    assert_eq!(sling.accuracy(), 0);
    assert_eq!(sling.required_skill(), Skill::Sling);
}

// ========== Damage Type Tests ==========

#[test]
fn test_damage_type_multipliers() {
    // BS 269-271: Damage type wound multipliers to torso
    assert_eq!(DamageType::Crushing.torso_multiplier(), 1.0);
    assert_eq!(DamageType::Cutting.torso_multiplier(), 1.5);
    assert_eq!(DamageType::Impaling.torso_multiplier(), 2.0);
    assert_eq!(DamageType::Piercing.torso_multiplier(), 1.0);
    assert_eq!(DamageType::PiercingLarge.torso_multiplier(), 1.5);
    assert_eq!(DamageType::PiercingSmall.torso_multiplier(), 0.5);
}

#[test]
fn test_cutting_vs_crushing() {
    // Cutting does more damage than crushing to flesh
    let cutting_mult = DamageType::Cutting.torso_multiplier();
    let crushing_mult = DamageType::Crushing.torso_multiplier();

    assert!(cutting_mult > crushing_mult);
    assert_eq!(cutting_mult, 1.5);
    assert_eq!(crushing_mult, 1.0);
}

#[test]
fn test_impaling_highest_multiplier() {
    // Impaling has highest torso multiplier among common types
    let impaling = DamageType::Impaling.torso_multiplier();

    assert_eq!(impaling, 2.0);
    assert!(impaling > DamageType::Cutting.torso_multiplier());
    assert!(impaling > DamageType::Crushing.torso_multiplier());
}

// ========== Reach Tests ==========

#[test]
fn test_reach_display() {
    assert_eq!(format!("{}", Reach::Close), "C");
    assert_eq!(format!("{}", Reach::CloseOne), "C,1");
    assert_eq!(format!("{}", Reach::One), "1");
    assert_eq!(format!("{}", Reach::OneTwo), "1,2");
    assert_eq!(format!("{}", Reach::TwoThree), "2,3");
    assert_eq!(format!("{}", Reach::Three), "3");
}

#[test]
fn test_long_reach_weapons() {
    // Verify long weapons have appropriate reach
    assert_eq!(MeleeWeapon::Lance.reach(), Reach::Three);
    assert_eq!(MeleeWeapon::LongSpear.reach(), Reach::TwoThree);
    assert_eq!(MeleeWeapon::Halberd.reach(), Reach::TwoThree);
}

#[test]
fn test_close_combat_weapons() {
    // Unarmed and small weapons are close combat
    assert_eq!(MeleeWeapon::Fist.reach(), Reach::Close);
    assert_eq!(MeleeWeapon::BrassKnuckles.reach(), Reach::Close);
    assert_eq!(MeleeWeapon::Knife.reach(), Reach::Close);
    assert_eq!(MeleeWeapon::Dagger.reach(), Reach::Close);
}

// ========== Parry Modifier Tests ==========

#[test]
fn test_defensive_weapons() {
    // Some weapons are better for parrying
    assert_eq!(MeleeWeapon::Rapier.parry_modifier(), 1);
    assert_eq!(MeleeWeapon::MainGauche.parry_modifier(), 1);
    assert_eq!(MeleeWeapon::Smallsword.parry_modifier(), 1);
    assert_eq!(MeleeWeapon::Quarterstaff.parry_modifier(), 2);
    assert_eq!(MeleeWeapon::Staff.parry_modifier(), 2);
}

#[test]
fn test_poor_parry_weapons() {
    // Some weapons are poor for parrying
    assert_eq!(MeleeWeapon::Axe.parry_modifier(), -1);
    assert_eq!(MeleeWeapon::Flail.parry_modifier(), -2);
    assert_eq!(MeleeWeapon::GreatAxe.parry_modifier(), -2);
    assert_eq!(MeleeWeapon::Lance.parry_modifier(), -2);
}

// ========== Skill Linkage Tests ==========

#[test]
fn test_weapon_skill_linkage() {
    // Verify weapons correctly link to their skills
    assert_eq!(MeleeWeapon::Broadsword.required_skill(), Skill::Broadsword);
    assert_eq!(MeleeWeapon::Rapier.required_skill(), Skill::Rapier);
    assert_eq!(MeleeWeapon::Spear.required_skill(), Skill::Spear);
    assert_eq!(MeleeWeapon::Axe.required_skill(), Skill::AxeMace);
    assert_eq!(MeleeWeapon::Mace.required_skill(), Skill::AxeMace);
}

#[test]
fn test_ranged_weapon_skills() {
    assert_eq!(RangedWeapon::Bow.required_skill(), Skill::Bow);
    assert_eq!(RangedWeapon::Crossbow.required_skill(), Skill::Crossbow);
    assert_eq!(RangedWeapon::Pistol.required_skill(), Skill::Guns);
    assert_eq!(RangedWeapon::Rifle.required_skill(), Skill::Guns);
    assert_eq!(RangedWeapon::Sling.required_skill(), Skill::Sling);
}

// ========== Weapon Damage Comparison Tests ==========

#[test]
fn test_heavy_vs_light_weapons() {
    // Heavy weapons should have higher damage modifiers
    let great_axe_mod = match MeleeWeapon::GreatAxe.damage() {
        WeaponDamage::Swing { modifier, .. } => modifier,
        _ => panic!("GreatAxe should use swing"),
    };

    let hatchet_mod = match MeleeWeapon::Hatchet.damage() {
        WeaponDamage::Swing { modifier, .. } => modifier,
        _ => panic!("Hatchet should use swing"),
    };

    assert!(great_axe_mod > hatchet_mod);
    assert_eq!(great_axe_mod, 3);
    assert_eq!(hatchet_mod, 1);
}

#[test]
fn test_all_melee_weapons_have_stats() {
    // Verify all melee weapons return valid stats
    use strum::IntoEnumIterator;

    for weapon in MeleeWeapon::iter() {
        // Should not panic
        let _ = weapon.damage();
        let _ = weapon.reach();
        let _ = weapon.parry_modifier();
        let _ = weapon.required_skill();
    }
}

#[test]
fn test_all_ranged_weapons_have_stats() {
    // Verify all ranged weapons return valid stats
    use strum::IntoEnumIterator;

    for weapon in RangedWeapon::iter() {
        // Should not panic
        let _ = weapon.damage();
        let _ = weapon.accuracy();
        let _ = weapon.required_skill();
    }
}
