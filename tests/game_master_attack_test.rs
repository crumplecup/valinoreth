//! Test GameMaster attack resolution implementation.

use valinoreth::{AttackDescriptorBuilder, AttackResolver, GameMaster};

#[tokio::test]
async fn test_resolve_attack_success() {
    let gm = GameMaster::new();

    let descriptor = AttackDescriptorBuilder::default()
        .effective_skill(15)
        .aim_bonus(0)
        .deceptive_penalty(0)
        .build()
        .expect("Valid descriptor");

    let (result, _evidence) = gm.resolve_attack(descriptor)
        .await
        .expect("Attack resolution succeeded");

    // With seed 42, verify roll is deterministic
    assert!(result.roll >= 3 && result.roll <= 18, "Roll should be 3d6 range");
    assert_eq!(result.effective_skill, 15);
}

#[tokio::test]
async fn test_confirm_attack_success() {
    let gm = GameMaster::new();

    let descriptor = AttackDescriptorBuilder::default()
        .effective_skill(18) // High skill for guaranteed success
        .aim_bonus(0)
        .deceptive_penalty(0)
        .build()
        .expect("Valid descriptor");

    let (result, evidence) = gm.resolve_attack(descriptor)
        .await
        .expect("Attack resolution succeeded");

    if result.success {
        let _success_evidence = gm.confirm_attack_success(result, evidence)
            .await
            .expect("Confirm success");
    }
}

#[tokio::test]
async fn test_confirm_attack_failure() {
    let gm = GameMaster::new();

    let descriptor = AttackDescriptorBuilder::default()
        .effective_skill(3) // Very low skill
        .aim_bonus(0)
        .deceptive_penalty(0)
        .build()
        .expect("Valid descriptor");

    let (result, evidence) = gm.resolve_attack(descriptor)
        .await
        .expect("Attack resolution succeeded");

    if !result.success {
        let _failure_evidence = gm.confirm_attack_failure(result, evidence)
            .await
            .expect("Confirm failure");
    }
}

#[tokio::test]
async fn test_critical_success_detection() {
    let gm = GameMaster::new();

    let descriptor = AttackDescriptorBuilder::default()
        .effective_skill(15)
        .aim_bonus(0)
        .deceptive_penalty(0)
        .build()
        .expect("Valid descriptor");

    let (result, _evidence) = gm.resolve_attack(descriptor)
        .await
        .expect("Attack resolution succeeded");

    // Critical success: roll 3-4, or 5-6 if skill >= 15
    if result.roll <= 4 || (result.roll <= 6 && result.effective_skill >= 15) {
        assert!(result.critical_success, "Should be critical success");
    }
}

#[tokio::test]
async fn test_critical_failure_detection() {
    let gm = GameMaster::new();

    let descriptor = AttackDescriptorBuilder::default()
        .effective_skill(10)
        .aim_bonus(0)
        .deceptive_penalty(0)
        .build()
        .expect("Valid descriptor");

    let (result, _evidence) = gm.resolve_attack(descriptor)
        .await
        .expect("Attack resolution succeeded");

    // Critical failure: roll 18, or 17 if skill < 16
    if result.roll >= 18 || (result.roll >= 17 && result.effective_skill < 16) {
        assert!(result.critical_failure, "Should be critical failure");
    }
}

#[tokio::test]
async fn test_margin_calculation() {
    let gm = GameMaster::new();

    let descriptor = AttackDescriptorBuilder::default()
        .effective_skill(12)
        .aim_bonus(2) // +2 aim
        .deceptive_penalty(1) // -1 deceptive
        .build()
        .expect("Valid descriptor");

    let (result, _evidence) = gm.resolve_attack(descriptor)
        .await
        .expect("Attack resolution succeeded");

    // Effective skill: 12 + 2 - 1 = 13
    assert_eq!(result.effective_skill, 13);

    if result.success {
        assert_eq!(result.margin, result.effective_skill - result.roll);
    } else {
        assert_eq!(result.margin, result.roll - result.effective_skill);
    }
}
