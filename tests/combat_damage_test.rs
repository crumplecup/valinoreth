//! Tests for damage resolution mechanics.

use valinoreth::{
    Armor, BaseDamage, DamageKind, DamageResolution, DamageType, DieLevel, Random, Torso,
    WeaponDamage,
};

// ========== DamageResolution Tests ==========

#[test]
fn test_damage_resolution_thrust_attack() {
    // ST 10: Thrust 1d-2
    let base_damage = BaseDamage::new(
        DamageKind::Thrust(DieLevel::new(1, -2)),
        DamageKind::Swing(DieLevel::new(1, 0)),
    ); // Using constructor since we have derive_new
    let weapon = WeaponDamage::Thrust {
        modifier: 1, // Rapier: thrust+1
        damage_type: DamageType::Impaling,
    };
    let torso = Torso::Chest;
    let dr = 0;

    let mut rng = Random::from_seed(42).unwrap();
    let result = DamageResolution::resolve(&base_damage, &weapon, &torso, dr, &mut rng);

    // ST 10 thrust is 1d-2, so raw is -1 to 4, modified is 0 to 5 (thrust+1 from rapier)
    assert!((-1..=4).contains(result.raw_damage()));
    assert!((0..=5).contains(result.modified_damage()));
    assert_eq!(*result.location_multiplier(), 2.0); // Impaling to torso
    assert_eq!(*result.dr(), 0);
    assert_eq!(*result.penetrating_damage(), *result.hp_lost()); // No DR, so equal
}

#[test]
fn test_damage_resolution_swing_attack() {
    // ST 10: Swing 1d
    let base_damage = BaseDamage::new(
        DamageKind::Thrust(DieLevel::new(1, -2)),
        DamageKind::Swing(DieLevel::new(1, 0)),
    );
    let weapon = WeaponDamage::Swing {
        modifier: 1, // Broadsword: swing+1
        damage_type: DamageType::Cutting,
    };
    let torso = Torso::Chest;
    let dr = 0;

    let mut rng = Random::from_seed(123).unwrap();
    let result = DamageResolution::resolve(&base_damage, &weapon, &torso, dr, &mut rng);

    // ST 10 swing is 1d+0, so raw is 1-6, modified is 2-7 (swing+1 from broadsword)
    assert!((1..=6).contains(result.raw_damage()));
    assert!((2..=7).contains(result.modified_damage()));
    assert_eq!(*result.location_multiplier(), 1.5); // Cutting to torso
    assert_eq!(*result.dr(), 0);
}

#[test]
fn test_damage_resolution_with_dr() {
    // Test damage reduction from armor
    let base_damage = BaseDamage::new(
        DamageKind::Thrust(DieLevel::new(1, -2)),
        DamageKind::Swing(DieLevel::new(1, 0)),
    );
    let weapon = WeaponDamage::Swing {
        modifier: 2,
        damage_type: DamageType::Cutting,
    };
    let torso = Torso::Chest;
    let dr = 4; // Chainmail armor

    let mut rng = Random::from_seed(42).unwrap();
    let result = DamageResolution::resolve(&base_damage, &weapon, &torso, dr, &mut rng);

    assert_eq!(*result.dr(), 4);
    // Penetrating damage should be location_damage - DR
    let expected_pen = *result.location_damage() - 4;
    assert_eq!(*result.penetrating_damage(), expected_pen);
    // HP lost should be max(0, penetrating)
    assert_eq!(*result.hp_lost(), expected_pen.max(0));
}

#[test]
fn test_damage_resolution_dr_blocks_all() {
    // High DR blocks all damage
    let base_damage = BaseDamage::new(
        DamageKind::Thrust(DieLevel::new(1, -2)),
        DamageKind::Swing(DieLevel::new(1, 0)),
    );
    let weapon = WeaponDamage::Thrust {
        modifier: -1, // Knife: thrust-1
        damage_type: DamageType::Impaling,
    };
    let torso = Torso::Chest;
    let dr = 10; // Heavy armor

    let mut rng = Random::from_seed(42).unwrap();
    let result = DamageResolution::resolve(&base_damage, &weapon, &torso, dr, &mut rng);

    // Even with multiplier, low damage vs high DR = 0 HP loss
    assert_eq!(*result.hp_lost(), 0);
    // Penetrating can be negative
    assert!(*result.penetrating_damage() <= 0);
}

#[test]
fn test_damage_resolution_fixed_damage() {
    // Ranged weapon with fixed damage (e.g., Pistol 2d+2)
    let base_damage = BaseDamage::new(
        DamageKind::Thrust(DieLevel::new(1, -2)),
        DamageKind::Swing(DieLevel::new(1, 0)),
    ); // Irrelevant for fixed
    let weapon = WeaponDamage::Fixed {
        dice: DieLevel::new(2, 2),
        damage_type: DamageType::Piercing,
    };
    let torso = Torso::Chest;
    let dr = 2;

    let mut rng = Random::from_seed(42).unwrap();
    let result = DamageResolution::resolve(&base_damage, &weapon, &torso, dr, &mut rng);

    // Raw damage is 2d (2-12), modified is +2 (4-14)
    assert!((2..=12).contains(result.raw_damage()));
    assert!((4..=14).contains(result.modified_damage()));
    assert_eq!(*result.location_multiplier(), 1.0); // Piercing to torso
    assert_eq!(*result.dr(), 2);
}

#[test]
fn test_damage_crushing_multiplier() {
    let base_damage = BaseDamage::new(
        DamageKind::Thrust(DieLevel::new(1, -2)),
        DamageKind::Swing(DieLevel::new(1, 0)),
    );
    let weapon = WeaponDamage::Swing {
        modifier: 2,
        damage_type: DamageType::Crushing,
    };
    let torso = Torso::Chest;
    let dr = 0;

    let mut rng = Random::from_seed(42).unwrap();
    let result = DamageResolution::resolve(&base_damage, &weapon, &torso, dr, &mut rng);

    // Crushing has x1.0 multiplier to torso
    assert_eq!(*result.location_multiplier(), 1.0);
    // Location damage should equal modified damage for crushing
    assert_eq!(*result.location_damage(), *result.modified_damage());
}

#[test]
fn test_damage_cutting_multiplier() {
    let base_damage = BaseDamage::new(
        DamageKind::Thrust(DieLevel::new(1, -2)),
        DamageKind::Swing(DieLevel::new(1, 0)),
    );
    let weapon = WeaponDamage::Swing {
        modifier: 1,
        damage_type: DamageType::Cutting,
    };
    let torso = Torso::Chest;
    let dr = 0;

    let mut rng = Random::from_seed(42).unwrap();
    let result = DamageResolution::resolve(&base_damage, &weapon, &torso, dr, &mut rng);

    // Cutting has x1.5 multiplier to torso
    assert_eq!(*result.location_multiplier(), 1.5);
    // Location damage should be modified * 1.5 (rounded)
    let expected = (*result.modified_damage() as f64 * 1.5).round() as i32;
    assert_eq!(*result.location_damage(), expected);
}

#[test]
fn test_damage_impaling_multiplier() {
    let base_damage = BaseDamage::new(
        DamageKind::Thrust(DieLevel::new(1, -2)),
        DamageKind::Swing(DieLevel::new(1, 0)),
    );
    let weapon = WeaponDamage::Thrust {
        modifier: 2,
        damage_type: DamageType::Impaling,
    };
    let torso = Torso::Chest;
    let dr = 0;

    let mut rng = Random::from_seed(42).unwrap();
    let result = DamageResolution::resolve(&base_damage, &weapon, &torso, dr, &mut rng);

    // Impaling has x2.0 multiplier to torso
    assert_eq!(*result.location_multiplier(), 2.0);
    // Location damage should be modified * 2
    assert_eq!(*result.location_damage(), *result.modified_damage() * 2);
}

#[test]
fn test_damage_piercing_small_multiplier() {
    let base_damage = BaseDamage::new(
        DamageKind::Thrust(DieLevel::new(1, -2)),
        DamageKind::Swing(DieLevel::new(1, 0)),
    );
    let weapon = WeaponDamage::Fixed {
        dice: DieLevel::new(1, 0),
        damage_type: DamageType::PiercingSmall,
    };
    let torso = Torso::Chest;
    let dr = 0;

    let mut rng = Random::from_seed(42).unwrap();
    let result = DamageResolution::resolve(&base_damage, &weapon, &torso, dr, &mut rng);

    // PiercingSmall has x0.5 multiplier to torso
    assert_eq!(*result.location_multiplier(), 0.5);
}

#[test]
fn test_damage_piercing_large_multiplier() {
    let base_damage = BaseDamage::new(
        DamageKind::Thrust(DieLevel::new(1, -2)),
        DamageKind::Swing(DieLevel::new(1, 0)),
    );
    let weapon = WeaponDamage::Fixed {
        dice: DieLevel::new(2, 0),
        damage_type: DamageType::PiercingLarge,
    };
    let torso = Torso::Chest;
    let dr = 0;

    let mut rng = Random::from_seed(42).unwrap();
    let result = DamageResolution::resolve(&base_damage, &weapon, &torso, dr, &mut rng);

    // PiercingLarge has x1.5 multiplier to torso
    assert_eq!(*result.location_multiplier(), 1.5);
}

#[test]
fn test_damage_negative_modifier() {
    // Small weapon with negative modifier
    let base_damage = BaseDamage::new(
        DamageKind::Thrust(DieLevel::new(1, -2)),
        DamageKind::Swing(DieLevel::new(1, 0)),
    );
    let weapon = WeaponDamage::Thrust {
        modifier: -1, // Fist: thrust-1
        damage_type: DamageType::Crushing,
    };
    let torso = Torso::Chest;
    let dr = 0;

    let mut rng = Random::from_seed(42).unwrap();
    let result = DamageResolution::resolve(&base_damage, &weapon, &torso, dr, &mut rng);

    // Modified damage should be raw - 1
    assert_eq!(*result.modified_damage(), *result.raw_damage() - 1);
}

#[test]
fn test_damage_large_modifier() {
    // Heavy weapon with large modifier
    let base_damage = BaseDamage::new(
        DamageKind::Thrust(DieLevel::new(1, -2)),
        DamageKind::Swing(DieLevel::new(1, 0)),
    );
    let weapon = WeaponDamage::Swing {
        modifier: 3, // Great Axe: swing+3
        damage_type: DamageType::Cutting,
    };
    let torso = Torso::Chest;
    let dr = 0;

    let mut rng = Random::from_seed(42).unwrap();
    let result = DamageResolution::resolve(&base_damage, &weapon, &torso, dr, &mut rng);

    // Modified damage should be raw + 3
    assert_eq!(*result.modified_damage(), *result.raw_damage() + 3);
}

#[test]
fn test_damage_resolution_deterministic() {
    // Same seed should give same result
    let base_damage = BaseDamage::new(
        DamageKind::Thrust(DieLevel::new(1, -2)),
        DamageKind::Swing(DieLevel::new(1, 0)),
    );
    let weapon = WeaponDamage::Thrust {
        modifier: 1,
        damage_type: DamageType::Impaling,
    };
    let torso = Torso::Chest;
    let dr = 3;

    let mut rng1 = Random::from_seed(12345).unwrap();
    let result1 = DamageResolution::resolve(&base_damage, &weapon, &torso, dr, &mut rng1);

    let mut rng2 = Random::from_seed(12345).unwrap();
    let result2 = DamageResolution::resolve(&base_damage, &weapon, &torso, dr, &mut rng2);

    assert_eq!(result1, result2);
}

#[test]
fn test_damage_resolution_different_seeds() {
    // Different seeds should (likely) give different results
    let base_damage = BaseDamage::new(
        DamageKind::Thrust(DieLevel::new(1, -2)),
        DamageKind::Swing(DieLevel::new(1, 0)),
    );
    let weapon = WeaponDamage::Swing {
        modifier: 1,
        damage_type: DamageType::Cutting,
    };
    let torso = Torso::Chest;
    let dr = 0;

    let mut different_results = false;
    for seed in 0..20 {
        let mut rng1 = Random::from_seed(seed).unwrap();
        let result1 = DamageResolution::resolve(&base_damage, &weapon, &torso, dr, &mut rng1);

        let mut rng2 = Random::from_seed(seed + 1000).unwrap();
        let result2 = DamageResolution::resolve(&base_damage, &weapon, &torso, dr, &mut rng2);

        if result1 != result2 {
            different_results = true;
            break;
        }
    }
    assert!(
        different_results,
        "Different seeds should produce different results"
    );
}

// ========== Armor Tests ==========

#[test]
fn test_armor_builder() {
    let armor = Armor::builder()
        .name("Leather Armor")
        .dr(1)
        .build()
        .unwrap();

    assert_eq!(armor.name(), "Leather Armor");
    assert_eq!(*armor.dr(), 1);
}

#[test]
fn test_armor_chainmail() {
    let chainmail = Armor::builder().name("Chainmail").dr(4).build().unwrap();

    assert_eq!(chainmail.name(), "Chainmail");
    assert_eq!(*chainmail.dr(), 4);
}

#[test]
fn test_armor_plate() {
    let plate = Armor::builder().name("Plate Armor").dr(6).build().unwrap();

    assert_eq!(plate.name(), "Plate Armor");
    assert_eq!(*plate.dr(), 6);
}

#[test]
fn test_armor_no_armor() {
    let none = Armor::builder().name("No Armor").dr(0).build().unwrap();

    assert_eq!(none.name(), "No Armor");
    assert_eq!(*none.dr(), 0);
}

#[test]
fn test_armor_heavy() {
    let heavy = Armor::builder().name("Heavy Plate").dr(8).build().unwrap();

    assert_eq!(heavy.name(), "Heavy Plate");
    assert_eq!(*heavy.dr(), 8);
}

// ========== Integration Tests ==========

#[test]
fn test_combat_scenario_unarmored_target() {
    // ST 12 character with broadsword vs unarmored opponent
    let base_damage = BaseDamage::new(
        DamageKind::Thrust(DieLevel::new(1, -1)),
        DamageKind::Swing(DieLevel::new(1, 2)),
    ); // ST 12: 1d-1 thrust, 1d+2 swing
    let broadsword = WeaponDamage::Swing {
        modifier: 1, // swing+1
        damage_type: DamageType::Cutting,
    };
    let torso = Torso::Chest;
    let dr = 0;

    let mut rng = Random::from_seed(42).unwrap();
    let result = DamageResolution::resolve(&base_damage, &broadsword, &torso, dr, &mut rng);

    // ST 12 swing is 1d+2, broadsword is swing+1, so total is 1d+3 = 4-9
    assert!((4..=9).contains(result.modified_damage()));
    assert_eq!(*result.location_multiplier(), 1.5);
    assert_eq!(*result.hp_lost(), *result.location_damage()); // No DR
}

#[test]
fn test_combat_scenario_armored_target() {
    // Rapier vs chainmail
    let base_damage = BaseDamage::new(
        DamageKind::Thrust(DieLevel::new(1, -2)),
        DamageKind::Swing(DieLevel::new(1, 0)),
    ); // ST 10
    let rapier = WeaponDamage::Thrust {
        modifier: 1, // thrust+1
        damage_type: DamageType::Impaling,
    };
    let torso = Torso::Chest;
    let dr = 4; // Chainmail

    let mut rng = Random::from_seed(42).unwrap();
    let result = DamageResolution::resolve(&base_damage, &rapier, &torso, dr, &mut rng);

    // Impaling x2, but chainmail DR 4 reduces damage significantly
    assert_eq!(*result.dr(), 4);
    assert_eq!(*result.location_multiplier(), 2.0);
    assert!(*result.hp_lost() <= *result.location_damage());
}

#[test]
fn test_combat_scenario_heavy_weapon_vs_armor() {
    // Great axe (swing+3 cutting) vs plate armor (DR 6)
    let base_damage = BaseDamage::new(
        DamageKind::Thrust(DieLevel::new(1, 0)),
        DamageKind::Swing(DieLevel::new(2, 0)),
    ); // ST 14: 1d thrust, 2d swing
    let great_axe = WeaponDamage::Swing {
        modifier: 3,
        damage_type: DamageType::Cutting,
    };
    let torso = Torso::Chest;
    let dr = 6;

    let mut rng = Random::from_seed(42).unwrap();
    let result = DamageResolution::resolve(&base_damage, &great_axe, &torso, dr, &mut rng);

    // Even heavy weapon, high DR significantly reduces damage
    assert_eq!(*result.dr(), 6);
    assert_eq!(*result.location_multiplier(), 1.5);
    let pen = *result.location_damage() - 6;
    assert_eq!(*result.penetrating_damage(), pen);
    assert_eq!(*result.hp_lost(), pen.max(0));
}

#[test]
fn test_combat_scenario_crushing_vs_cutting() {
    // Compare crushing vs cutting damage
    let base_damage = BaseDamage::new(
        DamageKind::Thrust(DieLevel::new(1, -2)),
        DamageKind::Swing(DieLevel::new(1, 0)),
    );
    let torso = Torso::Chest;
    let dr = 0;

    let mace = WeaponDamage::Swing {
        modifier: 2,
        damage_type: DamageType::Crushing,
    };

    let sword = WeaponDamage::Swing {
        modifier: 2,
        damage_type: DamageType::Cutting,
    };

    let mut rng = Random::from_seed(42).unwrap();
    let crush_result = DamageResolution::resolve(&base_damage, &mace, &torso, dr, &mut rng);

    let mut rng = Random::from_seed(42).unwrap();
    let cut_result = DamageResolution::resolve(&base_damage, &sword, &torso, dr, &mut rng);

    // Same raw and modified damage (same seed)
    assert_eq!(*crush_result.raw_damage(), *cut_result.raw_damage());
    assert_eq!(
        *crush_result.modified_damage(),
        *cut_result.modified_damage()
    );

    // Different multipliers
    assert_eq!(*crush_result.location_multiplier(), 1.0);
    assert_eq!(*cut_result.location_multiplier(), 1.5);

    // Cutting should do more HP damage
    assert!(*cut_result.hp_lost() > *crush_result.hp_lost());
}
