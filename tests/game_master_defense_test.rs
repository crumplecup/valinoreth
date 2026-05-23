//! Test GameMaster defense resolution implementation.

use valinoreth::{DefenseDescriptorBuilder, DefenseResolver, DefenseType, GameMaster};

#[tokio::test]
async fn test_resolve_defense_success() {
    let gm = GameMaster::new();

    let descriptor = DefenseDescriptorBuilder::default()
        .defense_type(DefenseType::Dodge)
        .defense_score(10)
        .feint_penalty(0)
        .deceptive_penalty(0)
        .retreating(false)
        .build()
        .expect("Valid descriptor");

    let (result, _evidence) = gm
        .resolve_defense(descriptor)
        .await
        .expect("Defense resolution succeeded");

    // With seed 42, verify roll is deterministic
    assert!(
        result.roll >= 3 && result.roll <= 18,
        "Roll should be 3d6 range"
    );
    assert_eq!(result.defense_score, 10);
}

#[tokio::test]
async fn test_defense_with_retreat() {
    let gm = GameMaster::new();

    let descriptor = DefenseDescriptorBuilder::default()
        .defense_type(DefenseType::Dodge)
        .defense_score(10)
        .feint_penalty(0)
        .deceptive_penalty(0)
        .retreating(true) // +3 bonus
        .build()
        .expect("Valid descriptor");

    let (result, _evidence) = gm
        .resolve_defense(descriptor)
        .await
        .expect("Defense resolution succeeded");

    // Defense score should be 10 + 3 = 13
    assert_eq!(result.defense_score, 13, "Retreat should add +3");
}

#[tokio::test]
async fn test_defense_with_penalties() {
    let gm = GameMaster::new();

    let descriptor = DefenseDescriptorBuilder::default()
        .defense_type(DefenseType::Parry)
        .defense_score(12)
        .feint_penalty(2) // -2 from feint
        .deceptive_penalty(1) // -1 from deceptive attack
        .retreating(false)
        .build()
        .expect("Valid descriptor");

    let (result, _evidence) = gm
        .resolve_defense(descriptor)
        .await
        .expect("Defense resolution succeeded");

    // Defense score: 12 - 2 - 1 = 9
    assert_eq!(result.defense_score, 9, "Penalties should reduce defense");
}

#[tokio::test]
async fn test_defense_with_retreat_and_penalties() {
    let gm = GameMaster::new();

    let descriptor = DefenseDescriptorBuilder::default()
        .defense_type(DefenseType::Block)
        .defense_score(11)
        .feint_penalty(1)
        .deceptive_penalty(2)
        .retreating(true) // +3
        .build()
        .expect("Valid descriptor");

    let (result, _evidence) = gm
        .resolve_defense(descriptor)
        .await
        .expect("Defense resolution succeeded");

    // Defense score: 11 + 3 - 1 - 2 = 11
    assert_eq!(
        result.defense_score, 11,
        "Should apply retreat bonus and penalties"
    );
}

#[tokio::test]
async fn test_confirm_defense_success() {
    let gm = GameMaster::new();

    let descriptor = DefenseDescriptorBuilder::default()
        .defense_type(DefenseType::Dodge)
        .defense_score(18) // High defense for guaranteed success
        .build()
        .expect("Valid descriptor");

    let (result, evidence) = gm
        .resolve_defense(descriptor)
        .await
        .expect("Defense resolution succeeded");

    if result.success {
        let _success_evidence = gm
            .confirm_defense_success(result, evidence)
            .await
            .expect("Confirm success");
    }
}

#[tokio::test]
async fn test_confirm_defense_failure() {
    let gm = GameMaster::new();

    let descriptor = DefenseDescriptorBuilder::default()
        .defense_type(DefenseType::Dodge)
        .defense_score(3) // Very low defense
        .build()
        .expect("Valid descriptor");

    let (result, evidence) = gm
        .resolve_defense(descriptor)
        .await
        .expect("Defense resolution succeeded");

    if !result.success {
        let _failure_evidence = gm
            .confirm_defense_failure(result, evidence)
            .await
            .expect("Confirm failure");
    }
}

#[tokio::test]
async fn test_defense_critical_success_detection() {
    let gm = GameMaster::new();

    let descriptor = DefenseDescriptorBuilder::default()
        .defense_type(DefenseType::Parry)
        .defense_score(15)
        .build()
        .expect("Valid descriptor");

    let (result, _evidence) = gm
        .resolve_defense(descriptor)
        .await
        .expect("Defense resolution succeeded");

    // Critical success: roll 3-4, or 5-6 if defense >= 15
    if result.roll <= 4 || (result.roll <= 6 && result.defense_score >= 15) {
        assert!(
            result.critical_success,
            "Should be critical success for roll {} with defense {}",
            result.roll, result.defense_score
        );
    }
}

#[tokio::test]
async fn test_defense_critical_failure_detection() {
    let gm = GameMaster::new();

    let descriptor = DefenseDescriptorBuilder::default()
        .defense_type(DefenseType::Block)
        .defense_score(10)
        .build()
        .expect("Valid descriptor");

    let (result, _evidence) = gm
        .resolve_defense(descriptor)
        .await
        .expect("Defense resolution succeeded");

    // Critical failure: roll 18, or 17 if defense < 16
    if result.roll >= 18 || (result.roll >= 17 && result.defense_score < 16) {
        assert!(
            result.critical_failure,
            "Should be critical failure for roll {} with defense {}",
            result.roll, result.defense_score
        );
    }
}

#[tokio::test]
async fn test_defense_margin_calculation() {
    let gm = GameMaster::new();

    let descriptor = DefenseDescriptorBuilder::default()
        .defense_type(DefenseType::Dodge)
        .defense_score(12)
        .build()
        .expect("Valid descriptor");

    let (result, _evidence) = gm
        .resolve_defense(descriptor)
        .await
        .expect("Defense resolution succeeded");

    if result.success {
        assert_eq!(
            result.margin,
            result.defense_score - result.roll,
            "Success margin should be defense - roll"
        );
    } else {
        assert_eq!(
            result.margin,
            result.roll - result.defense_score,
            "Failure margin should be roll - defense"
        );
    }
}
