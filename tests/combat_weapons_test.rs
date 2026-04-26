//! Tests for weapon definitions and statistics.

use valinoreth::{DamageType, Item, Reach, Skill, WeaponDamage};

// ========== Melee Weapon Tests ==========

#[test]
fn test_broadsword_stats() {
    let sword = Item::Broadsword;

    // BS 271: Broadsword is sw+1 cutting
    match sword.weapon_damage().unwrap() {
        WeaponDamage::Swing {
            modifier,
            damage_type,
        } => {
            assert_eq!(modifier, 1);
            assert_eq!(damage_type, DamageType::Cutting);
        }
        _ => panic!("Broadsword should use swing damage"),
    }

    assert_eq!(sword.reach().unwrap(), Reach::One);
    assert_eq!(sword.parry_modifier().unwrap(), 0);
    assert_eq!(sword.required_skill().unwrap(), Skill::Broadsword);
}

#[test]
fn test_rapier_stats() {
    let rapier = Item::Rapier;

    // BS 274: Rapier is thr+1 impaling
    match rapier.weapon_damage().unwrap() {
        WeaponDamage::Thrust {
            modifier,
            damage_type,
        } => {
            assert_eq!(modifier, 1);
            assert_eq!(damage_type, DamageType::Impaling);
        }
        _ => panic!("Rapier should use thrust damage"),
    }

    assert_eq!(rapier.reach().unwrap(), Reach::One);
    assert_eq!(rapier.parry_modifier().unwrap(), 1); // Rapier has +1 parry
    assert_eq!(rapier.required_skill().unwrap(), Skill::Rapier);
}

#[test]
fn test_spear_stats() {
    let spear = Item::Spear;

    // BS 275: Spear is thr+2 impaling
    match spear.weapon_damage().unwrap() {
        WeaponDamage::Thrust {
            modifier,
            damage_type,
        } => {
            assert_eq!(modifier, 2);
            assert_eq!(damage_type, DamageType::Impaling);
        }
        _ => panic!("Spear should use thrust damage"),
    }

    assert_eq!(spear.reach().unwrap(), Reach::OneTwo); // Can strike at 1 or 2 meters
    assert_eq!(spear.parry_modifier().unwrap(), 0);
    assert_eq!(spear.required_skill().unwrap(), Skill::Spear);
}

#[test]
fn test_axe_stats() {
    let axe = Item::Axe;

    // BS 271: Axe is sw+2 cutting
    match axe.weapon_damage().unwrap() {
        WeaponDamage::Swing {
            modifier,
            damage_type,
        } => {
            assert_eq!(modifier, 2);
            assert_eq!(damage_type, DamageType::Cutting);
        }
        _ => panic!("Axe should use swing damage"),
    }

    assert_eq!(axe.reach().unwrap(), Reach::One);
    assert_eq!(axe.parry_modifier().unwrap(), -1); // Axe has -1 parry
    assert_eq!(axe.required_skill().unwrap(), Skill::AxeMace);
}

#[test]
fn test_fist_unarmed_stats() {
    let fist = Item::Fist;

    // BS 271: Fist is thr-1 crushing
    match fist.weapon_damage().unwrap() {
        WeaponDamage::Thrust {
            modifier,
            damage_type,
        } => {
            assert_eq!(modifier, -1);
            assert_eq!(damage_type, DamageType::Crushing);
        }
        _ => panic!("Fist should use thrust damage"),
    }

    assert_eq!(fist.reach().unwrap(), Reach::Close);
    assert_eq!(fist.parry_modifier().unwrap(), 0);
    assert_eq!(fist.required_skill().unwrap(), Skill::Brawling);
}

#[test]
fn test_quarterstaff_stats() {
    let staff = Item::Quarterstaff;

    // BS 274: Quarterstaff is sw+2 crushing
    match staff.weapon_damage().unwrap() {
        WeaponDamage::Swing {
            modifier,
            damage_type,
        } => {
            assert_eq!(modifier, 2);
            assert_eq!(damage_type, DamageType::Crushing);
        }
        _ => panic!("Quarterstaff should use swing damage"),
    }

    assert_eq!(staff.reach().unwrap(), Reach::OneTwo);
    assert_eq!(staff.parry_modifier().unwrap(), 2); // Quarterstaff has +2 parry
    assert_eq!(staff.required_skill().unwrap(), Skill::Staff);
}

#[test]
fn test_knife_stats() {
    let knife = Item::Knife;

    // BS 273: Knife is thr-1 impaling
    match knife.weapon_damage().unwrap() {
        WeaponDamage::Thrust {
            modifier,
            damage_type,
        } => {
            assert_eq!(modifier, -1);
            assert_eq!(damage_type, DamageType::Impaling);
        }
        _ => panic!("Knife should use thrust damage"),
    }

    assert_eq!(knife.reach().unwrap(), Reach::Close);
    assert_eq!(knife.parry_modifier().unwrap(), -1);
    assert_eq!(knife.required_skill().unwrap(), Skill::Knife);
}

#[test]
fn test_two_handed_sword_stats() {
    let sword = Item::TwoHandedSword;

    // BS 276: Two-Handed Sword is sw+2 cutting
    match sword.weapon_damage().unwrap() {
        WeaponDamage::Swing {
            modifier,
            damage_type,
        } => {
            assert_eq!(modifier, 2);
            assert_eq!(damage_type, DamageType::Cutting);
        }
        _ => panic!("Two-Handed Sword should use swing damage"),
    }

    assert_eq!(sword.reach().unwrap(), Reach::OneTwo);
    assert_eq!(sword.parry_modifier().unwrap(), 0);
    assert_eq!(sword.required_skill().unwrap(), Skill::TwoHandedSword);
}

// ========== Ranged Weapon Tests ==========

#[test]
fn test_bow_stats() {
    let bow = Item::Bow;

    // BS 276: Bow is 1d impaling
    match bow.weapon_damage().unwrap() {
        WeaponDamage::Fixed { dice, damage_type } => {
            assert_eq!(*dice.dice(), 1);
            assert_eq!(*dice.pips(), 0);
            assert_eq!(damage_type, DamageType::Impaling);
        }
        _ => panic!("Bow should use fixed damage"),
    }

    assert_eq!(bow.accuracy().unwrap(), 2);
    assert_eq!(bow.required_skill().unwrap(), Skill::Bow);
}

#[test]
fn test_crossbow_stats() {
    let crossbow = Item::Crossbow;

    // BS 276: Crossbow is 1d+4 impaling
    match crossbow.weapon_damage().unwrap() {
        WeaponDamage::Fixed { dice, damage_type } => {
            assert_eq!(*dice.dice(), 1);
            assert_eq!(*dice.pips(), 4);
            assert_eq!(damage_type, DamageType::Impaling);
        }
        _ => panic!("Crossbow should use fixed damage"),
    }

    assert_eq!(crossbow.accuracy().unwrap(), 4);
    assert_eq!(crossbow.required_skill().unwrap(), Skill::Crossbow);
}

#[test]
fn test_pistol_stats() {
    let pistol = Item::Pistol;

    // BS 278: Pistol (9mm) is 2d+2 piercing
    match pistol.weapon_damage().unwrap() {
        WeaponDamage::Fixed { dice, damage_type } => {
            assert_eq!(*dice.dice(), 2);
            assert_eq!(*dice.pips(), 2);
            assert_eq!(damage_type, DamageType::Piercing);
        }
        _ => panic!("Pistol should use fixed damage"),
    }

    assert_eq!(pistol.accuracy().unwrap(), 2);
    assert_eq!(pistol.required_skill().unwrap(), Skill::Guns);
}

#[test]
fn test_rifle_stats() {
    let rifle = Item::Rifle;

    // BS 278: Rifle (.30) is 5d piercing
    match rifle.weapon_damage().unwrap() {
        WeaponDamage::Fixed { dice, damage_type } => {
            assert_eq!(*dice.dice(), 5);
            assert_eq!(*dice.pips(), 0);
            assert_eq!(damage_type, DamageType::Piercing);
        }
        _ => panic!("Rifle should use fixed damage"),
    }

    assert_eq!(rifle.accuracy().unwrap(), 5);
    assert_eq!(rifle.required_skill().unwrap(), Skill::Guns);
}

#[test]
fn test_sling_st_based_damage() {
    let sling = Item::Sling;

    // BS 277: Sling uses swing piercing (ST-based)
    match sling.weapon_damage().unwrap() {
        WeaponDamage::Swing {
            modifier,
            damage_type,
        } => {
            assert_eq!(modifier, 0);
            assert_eq!(damage_type, DamageType::Piercing);
        }
        _ => panic!("Sling should use swing damage"),
    }

    assert_eq!(sling.accuracy().unwrap(), 0);
    assert_eq!(sling.required_skill().unwrap(), Skill::Sling);
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
    assert_eq!(Item::Lance.reach().unwrap(), Reach::Three);
    assert_eq!(Item::LongSpear.reach().unwrap(), Reach::TwoThree);
    assert_eq!(Item::Halberd.reach().unwrap(), Reach::TwoThree);
}

#[test]
fn test_close_combat_weapons() {
    // Unarmed and small weapons are close combat
    assert_eq!(Item::Fist.reach().unwrap(), Reach::Close);
    assert_eq!(Item::BrassKnuckles.reach().unwrap(), Reach::Close);
    assert_eq!(Item::Knife.reach().unwrap(), Reach::Close);
    assert_eq!(Item::Dagger.reach().unwrap(), Reach::Close);
}

// ========== Parry Modifier Tests ==========

#[test]
fn test_defensive_weapons() {
    // Some weapons are better for parrying
    assert_eq!(Item::Rapier.parry_modifier().unwrap(), 1);
    assert_eq!(Item::MainGauche.parry_modifier().unwrap(), 1);
    assert_eq!(Item::Smallsword.parry_modifier().unwrap(), 1);
    assert_eq!(Item::Quarterstaff.parry_modifier().unwrap(), 2);
    assert_eq!(Item::Staff.parry_modifier().unwrap(), 2);
}

#[test]
fn test_poor_parry_weapons() {
    // Some weapons are poor for parrying
    assert_eq!(Item::Axe.parry_modifier().unwrap(), -1);
    assert_eq!(Item::Flail.parry_modifier().unwrap(), -2);
    assert_eq!(Item::GreatAxe.parry_modifier().unwrap(), -2);
    assert_eq!(Item::Lance.parry_modifier().unwrap(), -2);
}

// ========== Skill Linkage Tests ==========

#[test]
fn test_weapon_skill_linkage() {
    // Verify weapons correctly link to their skills
    assert_eq!(
        Item::Broadsword.required_skill().unwrap(),
        Skill::Broadsword
    );
    assert_eq!(Item::Rapier.required_skill().unwrap(), Skill::Rapier);
    assert_eq!(Item::Spear.required_skill().unwrap(), Skill::Spear);
    assert_eq!(Item::Axe.required_skill().unwrap(), Skill::AxeMace);
    assert_eq!(Item::Mace.required_skill().unwrap(), Skill::AxeMace);
}

#[test]
fn test_ranged_weapon_skills() {
    assert_eq!(Item::Bow.required_skill().unwrap(), Skill::Bow);
    assert_eq!(Item::Crossbow.required_skill().unwrap(), Skill::Crossbow);
    assert_eq!(Item::Pistol.required_skill().unwrap(), Skill::Guns);
    assert_eq!(Item::Rifle.required_skill().unwrap(), Skill::Guns);
    assert_eq!(Item::Sling.required_skill().unwrap(), Skill::Sling);
}

// ========== Weapon Damage Comparison Tests ==========

#[test]
fn test_heavy_vs_light_weapons() {
    // Heavy weapons should have higher damage modifiers
    let great_axe_mod = match Item::GreatAxe.weapon_damage().unwrap() {
        WeaponDamage::Swing { modifier, .. } => modifier,
        _ => panic!("GreatAxe should use swing"),
    };

    let hatchet_mod = match Item::Hatchet.weapon_damage().unwrap() {
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
    use valinoreth::ItemCategory;

    for weapon in Item::iter() {
        if weapon.category() == ItemCategory::MeleeWeapon {
            // Should not panic
            let _ = weapon.weapon_damage().unwrap();
            let _ = weapon.reach().unwrap();
            let _ = weapon.parry_modifier().unwrap();
            let _ = weapon.required_skill().unwrap();
        }
    }
}

#[test]
fn test_all_ranged_weapons_have_stats() {
    // Verify all ranged weapons return valid stats
    use strum::IntoEnumIterator;
    use valinoreth::ItemCategory;

    for weapon in Item::iter() {
        if weapon.category() == ItemCategory::RangedWeapon {
            // Should not panic
            let _ = weapon.weapon_damage().unwrap();
            let _ = weapon.accuracy().unwrap();
            let _ = weapon.required_skill().unwrap();
        }
    }
}
