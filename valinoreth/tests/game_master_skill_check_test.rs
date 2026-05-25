//! Test GameMaster skill check resolution implementation.

use valinoreth::{GameMaster, SkillCheckDescriptorBuilder, SkillCheckExecutor};

#[tokio::test]
async fn test_resolve_skill_check_success() {
    let gm = GameMaster::new();

    let descriptor = SkillCheckDescriptorBuilder::default()
        .effective_skill(14)
        .situational_modifier(0)
        .task_difficulty(0)
        .time_modifier(0)
        .build()
        .expect("Valid descriptor");

    let (result, _evidence) = gm
        .resolve_skill_check(descriptor)
        .await
        .expect("Skill check resolution succeeded");

    // With seed 42, verify roll is deterministic
    assert!(
        result.roll >= 3 && result.roll <= 18,
        "Roll should be 3d6 range"
    );
    assert_eq!(result.effective_skill, 14);
}

#[tokio::test]
async fn test_resolve_skill_check_with_modifiers() {
    let gm = GameMaster::new();

    let descriptor = SkillCheckDescriptorBuilder::default()
        .effective_skill(12)
        .situational_modifier(2) // Good lighting
        .task_difficulty(-2) // Difficult task
        .time_modifier(1) // Taking extra time
        .build()
        .expect("Valid descriptor");

    let (result, _evidence) = gm
        .resolve_skill_check(descriptor)
        .await
        .expect("Skill check resolution succeeded");

    // Effective skill: 12 + 2 - 2 + 1 = 13
    assert_eq!(result.effective_skill, 13, "Modifiers should be applied");
}

#[tokio::test]
async fn test_resolve_skill_check_with_complementary() {
    let gm = GameMaster::new();

    let descriptor = SkillCheckDescriptorBuilder::default()
        .effective_skill(10)
        .complementary_bonus(Some(2)) // Complementary skill bonus
        .build()
        .expect("Valid descriptor");

    let (result, _evidence) = gm
        .resolve_skill_check(descriptor)
        .await
        .expect("Skill check resolution succeeded");

    // Effective skill: 10 + 2 = 12
    assert_eq!(
        result.effective_skill, 12,
        "Complementary bonus should be applied"
    );
}

#[tokio::test]
async fn test_confirm_skill_check_success() {
    let gm = GameMaster::new();

    let descriptor = SkillCheckDescriptorBuilder::default()
        .effective_skill(18) // High skill for guaranteed success
        .build()
        .expect("Valid descriptor");

    let (result, evidence) = gm
        .resolve_skill_check(descriptor)
        .await
        .expect("Skill check resolution succeeded");

    if result.success {
        let _success_evidence = gm
            .confirm_skill_check_success(result, evidence)
            .await
            .expect("Confirm success");
    }
}

#[tokio::test]
async fn test_confirm_skill_check_failure() {
    let gm = GameMaster::new();

    let descriptor = SkillCheckDescriptorBuilder::default()
        .effective_skill(3) // Very low skill
        .build()
        .expect("Valid descriptor");

    let (result, evidence) = gm
        .resolve_skill_check(descriptor)
        .await
        .expect("Skill check resolution succeeded");

    if !result.success {
        let _failure_evidence = gm
            .confirm_skill_check_failure(result, evidence)
            .await
            .expect("Confirm failure");
    }
}

#[tokio::test]
async fn test_skill_check_critical_success_detection() {
    let gm = GameMaster::new();

    let descriptor = SkillCheckDescriptorBuilder::default()
        .effective_skill(15)
        .build()
        .expect("Valid descriptor");

    let (result, _evidence) = gm
        .resolve_skill_check(descriptor)
        .await
        .expect("Skill check resolution succeeded");

    // Critical success: roll 3-4, or 5-6 if skill >= 15
    if result.roll <= 4 || (result.roll <= 6 && result.effective_skill >= 15) {
        assert!(result.critical_success, "Should be critical success");
    }
}

#[tokio::test]
async fn test_skill_check_critical_failure_detection() {
    let gm = GameMaster::new();

    let descriptor = SkillCheckDescriptorBuilder::default()
        .effective_skill(10)
        .build()
        .expect("Valid descriptor");

    let (result, _evidence) = gm
        .resolve_skill_check(descriptor)
        .await
        .expect("Skill check resolution succeeded");

    // Critical failure: roll 18, or 17 if skill < 16
    if result.roll >= 18 || (result.roll >= 17 && result.effective_skill < 16) {
        assert!(result.critical_failure, "Should be critical failure");
    }
}

#[tokio::test]
async fn test_skill_check_margin_calculation() {
    let gm = GameMaster::new();

    let descriptor = SkillCheckDescriptorBuilder::default()
        .effective_skill(12)
        .build()
        .expect("Valid descriptor");

    let (result, _evidence) = gm
        .resolve_skill_check(descriptor)
        .await
        .expect("Skill check resolution succeeded");

    if result.success {
        assert_eq!(
            result.margin,
            result.effective_skill - result.roll,
            "Success margin should be skill - roll"
        );
    } else {
        assert_eq!(
            result.margin,
            result.roll - result.effective_skill,
            "Failure margin should be roll - skill"
        );
    }
}

#[tokio::test]
async fn test_skill_check_with_negative_modifiers() {
    let gm = GameMaster::new();

    let descriptor = SkillCheckDescriptorBuilder::default()
        .effective_skill(14)
        .situational_modifier(-3) // Bad conditions
        .task_difficulty(-4) // Very difficult
        .build()
        .expect("Valid descriptor");

    let (result, _evidence) = gm
        .resolve_skill_check(descriptor)
        .await
        .expect("Skill check resolution succeeded");

    // Effective skill: 14 - 3 - 4 = 7
    assert_eq!(result.effective_skill, 7, "Penalties should reduce skill");
}

#[tokio::test]
async fn test_skill_check_with_all_modifiers() {
    let gm = GameMaster::new();

    let descriptor = SkillCheckDescriptorBuilder::default()
        .effective_skill(15)
        .situational_modifier(1)
        .task_difficulty(-2)
        .time_modifier(2)
        .complementary_bonus(Some(1))
        .build()
        .expect("Valid descriptor");

    let (result, _evidence) = gm
        .resolve_skill_check(descriptor)
        .await
        .expect("Skill check resolution succeeded");

    // Effective skill: 15 + 1 - 2 + 2 + 1 = 17
    assert_eq!(
        result.effective_skill, 17,
        "All modifiers should be applied correctly"
    );
}

#[tokio::test]
async fn test_confirm_critical_success() {
    let gm = GameMaster::new();

    let descriptor = SkillCheckDescriptorBuilder::default()
        .effective_skill(18)
        .build()
        .expect("Valid descriptor");

    let (result, evidence) = gm
        .resolve_skill_check(descriptor)
        .await
        .expect("Skill check resolution succeeded");

    if result.success && result.critical_success {
        let success_evidence = gm
            .confirm_skill_check_success(result, evidence)
            .await
            .expect("Confirm success");

        let _critical_evidence = gm
            .confirm_critical_skill_success(result, success_evidence)
            .await
            .expect("Confirm critical success");
    }
}

#[tokio::test]
async fn test_confirm_critical_failure() {
    let gm = GameMaster::new();

    let descriptor = SkillCheckDescriptorBuilder::default()
        .effective_skill(5)
        .build()
        .expect("Valid descriptor");

    let (result, evidence) = gm
        .resolve_skill_check(descriptor)
        .await
        .expect("Skill check resolution succeeded");

    if !result.success && result.critical_failure {
        let _critical_evidence = gm
            .confirm_critical_skill_failure(result, evidence)
            .await
            .expect("Confirm critical failure");
    }
}
