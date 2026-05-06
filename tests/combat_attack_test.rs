//! Tests for combat attack resolution.

use valinoreth::{AttackResult, AttackRoll, CombatModifiers, Modifier, Random};

#[test]
fn test_attack_roll_success() {
    let mut rng = Random::from_seed(42).unwrap();
    let roll = AttackRoll::execute(14, &mut rng);

    // With seed 42, verify we get a result
    assert!(roll.effective_skill() == &14);
    assert!((3..=18).contains(roll.dice().sum()));
}

#[test]
fn test_attack_roll_guaranteed_success() {
    // Skill 20 vs roll of 3-4 is always critical success
    let mut rng = Random::from_seed(1).unwrap();

    // Try multiple times to verify logic
    for _ in 0..10 {
        let roll = AttackRoll::execute(20, &mut rng);
        let dice_total = *roll.dice().sum();

        if dice_total <= 4 {
            match roll.result() {
                AttackResult::CriticalSuccess { .. } => (),
                _ => panic!("Expected critical success for roll {}", dice_total),
            }
        } else if dice_total <= 20 {
            // Should be success or critical success
            match roll.result() {
                AttackResult::Success { .. } | AttackResult::CriticalSuccess { .. } => (),
                _ => panic!("Expected success for roll {}", dice_total),
            }
        }
    }
}

#[test]
fn test_attack_roll_guaranteed_failure() {
    // Skill 5 vs roll of 18 is always critical failure
    let mut rng = Random::from_seed(2).unwrap();

    for _ in 0..10 {
        let roll = AttackRoll::execute(5, &mut rng);
        let dice_total = *roll.dice().sum();

        if dice_total >= 18 {
            match roll.result() {
                AttackResult::CriticalFailure { .. } => (),
                _ => panic!("Expected critical failure for roll {}", dice_total),
            }
        } else if dice_total > 5 {
            // Should be failure (unless critical success on 3-4)
            if dice_total >= 6 {
                match roll.result() {
                    AttackResult::Failure { .. } | AttackResult::CriticalFailure { .. } => (),
                    _ => panic!("Expected failure for roll {}", dice_total),
                }
            }
        }
    }
}

#[test]
fn test_critical_success_threshold_skill_15_plus() {
    // With skill 15+, rolls of 5-6 are also critical successes
    let mut rng = Random::from_seed(3).unwrap();

    for _ in 0..20 {
        let roll = AttackRoll::execute(16, &mut rng);
        let dice_total = *roll.dice().sum();

        // BS 357: Critical success on 3-4, or 5-6 if skill 15+
        if dice_total <= 6 {
            match roll.result() {
                AttackResult::CriticalSuccess { .. } => (),
                _ => panic!(
                    "Expected critical success for roll {} with skill 16",
                    dice_total
                ),
            }
        }
    }
}

#[test]
fn test_critical_failure_threshold_10_over() {
    // BS 358: Critical failure if roll is 10+ over skill
    let mut rng = Random::from_seed(4).unwrap();

    for _ in 0..10 {
        let roll = AttackRoll::execute(8, &mut rng);
        let dice_total = *roll.dice().sum();

        // Roll of 18 (10+ over skill 8) should be critical failure
        if dice_total >= 18 {
            match roll.result() {
                AttackResult::CriticalFailure { .. } => (),
                _ => panic!("Expected critical failure for roll {}", dice_total),
            }
        }
    }
}

#[test]
fn test_attack_result_margins() {
    let mut rng = Random::from_seed(5).unwrap();
    let roll = AttackRoll::execute(12, &mut rng);

    // Verify margin calculation
    match roll.result() {
        AttackResult::Success { margin } | AttackResult::CriticalSuccess { margin } => {
            assert_eq!(*margin, 12 - *roll.dice().sum() as i32);
        }
        AttackResult::Failure { margin } | AttackResult::CriticalFailure { margin } => {
            assert_eq!(*margin, *roll.dice().sum() as i32 - 12);
        }
    }
}

#[test]
fn test_combat_modifiers_empty() {
    let modifiers = CombatModifiers::builder()
        .modifiers(vec![])
        .build()
        .unwrap();

    assert_eq!(modifiers.total(), 0);
}

#[test]
fn test_combat_modifiers_single() {
    let mods = vec![Modifier::new("Range", -2, "BS 550")];

    let modifiers = CombatModifiers::builder().modifiers(mods).build().unwrap();

    assert_eq!(modifiers.total(), -2);
}

#[test]
fn test_combat_modifiers_multiple_additive() {
    let mods = vec![
        Modifier::new("Range", -2, "BS 550"),
        Modifier::new("Target posture", -2, "BS 399"),
        Modifier::new("Aim bonus", 3, "BS 364"),
    ];

    let modifiers = CombatModifiers::builder().modifiers(mods).build().unwrap();

    // -2 + -2 + 3 = -1
    assert_eq!(modifiers.total(), -1);
}

#[test]
fn test_combat_modifiers_positive_and_negative() {
    let mods = vec![
        Modifier::new("All-Out Attack", 4, "BS 365"),
        Modifier::new("Darkness", -5, "BS 394"),
        Modifier::new("Aiming", 2, "BS 364"),
    ];

    let modifiers = CombatModifiers::builder().modifiers(mods).build().unwrap();

    // 4 + -5 + 2 = 1
    assert_eq!(modifiers.total(), 1);
}

#[test]
fn test_modifier_in_combat_modifiers() {
    // Test that modifiers work correctly when used in CombatModifiers
    let mods = vec![
        Modifier::new("Test1", -3, "BS 999"),
        Modifier::new("Test2", 2, "BS 888"),
    ];

    let combat_mods = CombatModifiers::builder().modifiers(mods).build().unwrap();

    // Total should be -3 + 2 = -1
    assert_eq!(combat_mods.total(), -1);
}

#[test]
fn test_attack_roll_with_modifiers() {
    let mut rng = Random::from_seed(6).unwrap();

    // Base skill 12 with -2 modifiers = effective skill 10
    let base_skill = 12;
    let mods = vec![
        Modifier::new("Range", -1, "BS 550"),
        Modifier::new("Posture", -1, "BS 399"),
    ];
    let modifiers = CombatModifiers::builder().modifiers(mods).build().unwrap();

    let effective_skill = base_skill + modifiers.total();
    assert_eq!(effective_skill, 10);

    let roll = AttackRoll::execute(effective_skill, &mut rng);

    assert_eq!(roll.effective_skill(), &10);
}

#[test]
fn test_deterministic_rolls_with_seed() {
    // Same seed should produce same results
    let mut rng1 = Random::from_seed(100).unwrap();
    let mut rng2 = Random::from_seed(100).unwrap();

    let roll1 = AttackRoll::execute(14, &mut rng1);
    let roll2 = AttackRoll::execute(14, &mut rng2);

    assert_eq!(roll1.dice().sum(), roll2.dice().sum());
    assert_eq!(roll1.result(), roll2.result());
}

#[test]
fn test_different_seeds_produce_different_results() {
    // Different seeds should (very likely) produce different results
    let mut rng1 = Random::from_seed(200).unwrap();
    let mut rng2 = Random::from_seed(201).unwrap();

    let roll1 = AttackRoll::execute(14, &mut rng1);
    let roll2 = AttackRoll::execute(14, &mut rng2);

    // This could theoretically fail if both seeds happen to produce
    // the same dice result, but probability is very low
    let same_dice = roll1.dice().sum() == roll2.dice().sum();
    let same_d1 = roll1.dice().d1() == roll2.dice().d1();
    let same_d2 = roll1.dice().d2() == roll2.dice().d2();
    let same_d3 = roll1.dice().d3() == roll2.dice().d3();

    // At least one die should be different
    assert!(!(same_d1 && same_d2 && same_d3 && same_dice));
}
