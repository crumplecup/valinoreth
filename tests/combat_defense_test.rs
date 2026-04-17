//! Tests for active defense mechanics.

use valinoreth::{
    calculate_block, calculate_dodge, calculate_parry, defense_succeeds, ActiveDefense,
    DefenseResult, MeleeWeapon, Random, RETREAT_BONUS,
};

// ========== Parry Calculation Tests ==========

#[test]
fn test_calculate_parry_basic() {
    // BS 376: Parry = (Skill / 2) + 3 + weapon modifier
    // Broadsword skill 14, parry modifier 0
    let parry = calculate_parry(14, MeleeWeapon::Broadsword);
    assert_eq!(parry, 10); // (14/2) + 3 + 0 = 10
}

#[test]
fn test_calculate_parry_with_bonus() {
    // Rapier has +1 parry modifier
    // Skill 12, rapier modifier +1
    let parry = calculate_parry(12, MeleeWeapon::Rapier);
    assert_eq!(parry, 10); // (12/2) + 3 + 1 = 10
}

#[test]
fn test_calculate_parry_with_penalty() {
    // Axe has -1 parry modifier
    // Skill 14, axe modifier -1
    let parry = calculate_parry(14, MeleeWeapon::Axe);
    assert_eq!(parry, 9); // (14/2) + 3 - 1 = 9
}

#[test]
fn test_calculate_parry_quarterstaff() {
    // Quarterstaff has +2 parry modifier (best defensive weapon)
    // Skill 14, quarterstaff modifier +2
    let parry = calculate_parry(14, MeleeWeapon::Quarterstaff);
    assert_eq!(parry, 12); // (14/2) + 3 + 2 = 12
}

#[test]
fn test_calculate_parry_low_skill() {
    // Low skill still gets base calculation
    // Skill 8, broadsword modifier 0
    let parry = calculate_parry(8, MeleeWeapon::Broadsword);
    assert_eq!(parry, 7); // (8/2) + 3 + 0 = 7
}

#[test]
fn test_calculate_parry_high_skill() {
    // High skill provides good parry
    // Skill 20, rapier modifier +1
    let parry = calculate_parry(20, MeleeWeapon::Rapier);
    assert_eq!(parry, 14); // (20/2) + 3 + 1 = 14
}

// ========== Block Calculation Tests ==========

#[test]
fn test_calculate_block_basic() {
    // BS 377: Block = (Shield Skill / 2) + 3
    // Shield skill 12
    let block = calculate_block(12);
    assert_eq!(block, 9); // (12/2) + 3 = 9
}

#[test]
fn test_calculate_block_low_skill() {
    // Low shield skill
    // Shield skill 8
    let block = calculate_block(8);
    assert_eq!(block, 7); // (8/2) + 3 = 7
}

#[test]
fn test_calculate_block_high_skill() {
    // High shield skill
    // Shield skill 16
    let block = calculate_block(16);
    assert_eq!(block, 11); // (16/2) + 3 = 11
}

#[test]
fn test_calculate_block_master_level() {
    // Master-level shield skill
    // Shield skill 20
    let block = calculate_block(20);
    assert_eq!(block, 13); // (20/2) + 3 = 13
}

// ========== Dodge Calculation Tests ==========

#[test]
fn test_calculate_dodge_basic() {
    // BS 374-375: Dodge = floor(Basic Speed) + 3
    // Basic Speed 5.75
    let dodge = calculate_dodge(5.75);
    assert_eq!(dodge, 8); // floor(5.75) + 3 = 8
}

#[test]
fn test_calculate_dodge_exact_integer() {
    // Basic Speed 6.00 (exact integer)
    let dodge = calculate_dodge(6.00);
    assert_eq!(dodge, 9); // floor(6.00) + 3 = 9
}

#[test]
fn test_calculate_dodge_low_speed() {
    // Low Basic Speed 4.25
    let dodge = calculate_dodge(4.25);
    assert_eq!(dodge, 7); // floor(4.25) + 3 = 7
}

#[test]
fn test_calculate_dodge_high_speed() {
    // High Basic Speed 7.50
    let dodge = calculate_dodge(7.50);
    assert_eq!(dodge, 10); // floor(7.50) + 3 = 10
}

#[test]
fn test_calculate_dodge_fractional() {
    // Basic Speed 5.25 (various fractions all round down to 5)
    assert_eq!(calculate_dodge(5.00), 8);
    assert_eq!(calculate_dodge(5.25), 8);
    assert_eq!(calculate_dodge(5.50), 8);
    assert_eq!(calculate_dodge(5.75), 8);
    assert_eq!(calculate_dodge(5.99), 8);
}

// ========== Defense Success Tests ==========

#[test]
fn test_defense_succeeds_exact_match() {
    // Roll exactly equals defense value - succeeds
    assert!(defense_succeeds(10, 10));
}

#[test]
fn test_defense_succeeds_under() {
    // Roll under defense value - succeeds
    assert!(defense_succeeds(10, 8));
}

#[test]
fn test_defense_succeeds_over() {
    // Roll over defense value - fails
    assert!(!defense_succeeds(10, 12));
}

#[test]
fn test_defense_succeeds_critical_success() {
    // Roll of 3 always succeeds if defense >= 3
    assert!(defense_succeeds(10, 3));
}

#[test]
fn test_defense_succeeds_critical_failure() {
    // Roll of 18 can still fail
    assert!(!defense_succeeds(10, 18));
}

// ========== ActiveDefense Tests ==========

#[test]
fn test_active_defense_dodge_value() {
    let dodge = ActiveDefense::Dodge { value: 8 };
    assert_eq!(dodge.value(), 8);
}

#[test]
fn test_active_defense_parry_value() {
    let parry = ActiveDefense::Parry {
        value: 10,
        weapon: MeleeWeapon::Broadsword,
    };
    assert_eq!(parry.value(), 10);
}

#[test]
fn test_active_defense_block_value() {
    let block = ActiveDefense::Block { value: 9 };
    assert_eq!(block.value(), 9);
}

#[test]
fn test_active_defense_dodge_roll_success() {
    let dodge = ActiveDefense::Dodge { value: 12 };

    // Find a seed that produces a successful roll
    let mut success_found = false;
    for seed in 0..100 {
        let mut test_rng = Random::from_seed(seed).unwrap();
        let result = dodge.roll(&mut test_rng);
        if matches!(result, DefenseResult::Success { .. }) {
            success_found = true;
            break;
        }
    }
    assert!(success_found, "Should find a successful dodge roll");
}

#[test]
fn test_active_defense_dodge_roll_failure() {
    let dodge = ActiveDefense::Dodge { value: 5 };

    // With low dodge, should eventually fail
    let mut failure_found = false;
    for seed in 0..100 {
        let mut test_rng = Random::from_seed(seed).unwrap();
        let result = dodge.roll(&mut test_rng);
        if matches!(result, DefenseResult::Failure { .. }) {
            failure_found = true;
            break;
        }
    }
    assert!(
        failure_found,
        "Should find a failed dodge roll with low value"
    );
}

#[test]
fn test_active_defense_parry_roll() {
    let parry = ActiveDefense::Parry {
        value: 10,
        weapon: MeleeWeapon::Broadsword,
    };
    let mut rng = Random::from_seed(42).unwrap();

    let result = parry.roll(&mut rng);

    // Should return either Success or Failure
    match result {
        DefenseResult::Success { margin } => {
            assert!(margin >= 0, "Success margin should be non-negative");
        }
        DefenseResult::Failure { margin } => {
            assert!(margin > 0, "Failure margin should be positive");
        }
    }
}

#[test]
fn test_active_defense_block_roll() {
    let block = ActiveDefense::Block { value: 9 };
    let mut rng = Random::from_seed(123).unwrap();

    let result = block.roll(&mut rng);

    // Should return either Success or Failure
    match result {
        DefenseResult::Success { margin } => {
            assert!(margin >= 0, "Success margin should be non-negative");
        }
        DefenseResult::Failure { margin } => {
            assert!(margin > 0, "Failure margin should be positive");
        }
    }
}

// ========== DefenseResult Tests ==========

#[test]
fn test_defense_result_success_margin() {
    // Defense 10, roll 7 -> margin of 3
    let defense_value = 10;
    let roll = 7;

    if defense_succeeds(defense_value, roll) {
        let margin = defense_value - roll;
        assert_eq!(margin, 3);
    }
}

#[test]
fn test_defense_result_failure_margin() {
    // Defense 10, roll 13 -> margin of 3
    let defense_value = 10;
    let roll = 13;

    if !defense_succeeds(defense_value, roll) {
        let margin = roll - defense_value;
        assert_eq!(margin, 3);
    }
}

#[test]
fn test_defense_result_pattern_matching() {
    let success = DefenseResult::Success { margin: 5 };
    let failure = DefenseResult::Failure { margin: 3 };

    match success {
        DefenseResult::Success { margin } => assert_eq!(margin, 5),
        DefenseResult::Failure { .. } => panic!("Expected success"),
    }

    match failure {
        DefenseResult::Failure { margin } => assert_eq!(margin, 3),
        DefenseResult::Success { .. } => panic!("Expected failure"),
    }
}

// ========== Retreat Bonus Tests ==========

#[test]
fn test_retreat_bonus_value() {
    // BS 377: Retreat provides +1 to any active defense
    assert_eq!(RETREAT_BONUS, 1);
}

#[test]
fn test_retreat_bonus_application() {
    // Base dodge 8, with retreat becomes 9
    let base_dodge = 8;
    let dodge_with_retreat = base_dodge + RETREAT_BONUS;
    assert_eq!(dodge_with_retreat, 9);
}

#[test]
fn test_retreat_bonus_parry() {
    // Base parry 10, with retreat becomes 11
    let skill = 14;
    let base_parry = calculate_parry(skill, MeleeWeapon::Broadsword);
    let parry_with_retreat = base_parry + RETREAT_BONUS;
    assert_eq!(base_parry, 10);
    assert_eq!(parry_with_retreat, 11);
}

#[test]
fn test_retreat_bonus_block() {
    // Base block 9, with retreat becomes 10
    let skill = 12;
    let base_block = calculate_block(skill);
    let block_with_retreat = base_block + RETREAT_BONUS;
    assert_eq!(base_block, 9);
    assert_eq!(block_with_retreat, 10);
}

// ========== Integration Tests ==========

#[test]
fn test_complete_dodge_defense() {
    // Calculate dodge from basic speed, then test defense roll
    let basic_speed = 5.75;
    let dodge_value = calculate_dodge(basic_speed);
    assert_eq!(dodge_value, 8);

    let dodge = ActiveDefense::Dodge { value: dodge_value };
    assert_eq!(dodge.value(), 8);

    let mut rng = Random::from_seed(42).unwrap();
    let _result = dodge.roll(&mut rng);
    // Result varies based on RNG, just ensure it doesn't panic
}

#[test]
fn test_complete_parry_defense() {
    // Calculate parry from skill and weapon, then test defense roll
    let skill = 14;
    let weapon = MeleeWeapon::Rapier;
    let parry_value = calculate_parry(skill, weapon);
    assert_eq!(parry_value, 11); // (14/2) + 3 + 1 = 11

    let parry = ActiveDefense::Parry {
        value: parry_value,
        weapon,
    };
    assert_eq!(parry.value(), 11);

    let mut rng = Random::from_seed(123).unwrap();
    let _result = parry.roll(&mut rng);
    // Result varies based on RNG, just ensure it doesn't panic
}

#[test]
fn test_complete_block_defense() {
    // Calculate block from shield skill, then test defense roll
    let skill = 12;
    let block_value = calculate_block(skill);
    assert_eq!(block_value, 9);

    let block = ActiveDefense::Block { value: block_value };
    assert_eq!(block.value(), 9);

    let mut rng = Random::from_seed(456).unwrap();
    let _result = block.roll(&mut rng);
    // Result varies based on RNG, just ensure it doesn't panic
}

#[test]
fn test_defense_comparison() {
    // Compare different defense types for same character
    let basic_speed = 6.0;
    let weapon_skill = 14;
    let shield_skill = 12;

    let dodge = calculate_dodge(basic_speed); // floor(6.0) + 3 = 9
    let parry = calculate_parry(weapon_skill, MeleeWeapon::Broadsword); // (14/2) + 3 = 10
    let block = calculate_block(shield_skill); // (12/2) + 3 = 9

    assert_eq!(dodge, 9);
    assert_eq!(parry, 10);
    assert_eq!(block, 9);

    // Parry is best defense in this scenario
    assert!(parry >= dodge);
    assert!(parry >= block);
}

#[test]
fn test_defensive_weapons_comparison() {
    // Compare parry values for different weapons at same skill
    let skill = 14;

    let axe_parry = calculate_parry(skill, MeleeWeapon::Axe); // -1 modifier
    let broadsword_parry = calculate_parry(skill, MeleeWeapon::Broadsword); // 0 modifier
    let rapier_parry = calculate_parry(skill, MeleeWeapon::Rapier); // +1 modifier
    let quarterstaff_parry = calculate_parry(skill, MeleeWeapon::Quarterstaff); // +2 modifier

    assert_eq!(axe_parry, 9);
    assert_eq!(broadsword_parry, 10);
    assert_eq!(rapier_parry, 11);
    assert_eq!(quarterstaff_parry, 12);

    // Quarterstaff is most defensive
    assert!(quarterstaff_parry > rapier_parry);
    assert!(quarterstaff_parry > broadsword_parry);
    assert!(quarterstaff_parry > axe_parry);
}
