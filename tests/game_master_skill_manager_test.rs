//! Test GameMaster skill management implementation.

use valinoreth::{
    AttributeType, CharacterDescriptor, CharacterDescriptorBuilder, DerivedStatsDescriptor,
    GameMaster, SkillDescriptorBuilder, SkillDifficulty, SkillManager,
};

fn create_test_character() -> CharacterDescriptor {
    CharacterDescriptorBuilder::default()
        .name("Test Character".to_string())
        .description("Test".to_string())
        .total_points(150)
        .points_spent(0)
        .attributes(vec![])
        .derived_stats(DerivedStatsDescriptor {
            basic_speed: 5.0,
            basic_move: 5,
            dodge: 8,
            hp: 10,
            will: 10,
            perception: 10,
            fp: 10,
        })
        .build()
        .expect("Valid character")
}

#[tokio::test]
async fn test_add_skill() {
    let gm = GameMaster::new();

    let character = create_test_character();

    let skill = SkillDescriptorBuilder::default()
        .name("Broadsword".to_string())
        .difficulty(SkillDifficulty::Average)
        .base_attribute(AttributeType::DX)
        .points(2)
        .level(10)
        .build()
        .expect("Valid skill");

    let updated_character = gm
        .add_skill(character.clone(), skill.clone())
        .await
        .expect("Add skill succeeded");

    assert_eq!(updated_character.skills.len(), 1, "Should have 1 skill");
    assert_eq!(
        updated_character.skills[0].name, "Broadsword",
        "Skill name should match"
    );
    assert_eq!(
        updated_character.skills[0].points, 2,
        "Skill points should match"
    );
}

#[tokio::test]
async fn test_add_duplicate_skill_fails() {
    let gm = GameMaster::new();

    let character = create_test_character();

    let skill = SkillDescriptorBuilder::default()
        .name("Broadsword".to_string())
        .difficulty(SkillDifficulty::Average)
        .base_attribute(AttributeType::DX)
        .points(2)
        .level(10)
        .build()
        .expect("Valid skill");

    let character_with_skill = gm
        .add_skill(character, skill.clone())
        .await
        .expect("First add succeeded");

    // Try to add same skill again
    let result = gm.add_skill(character_with_skill, skill).await;

    assert!(result.is_err(), "Adding duplicate skill should fail");
}

#[tokio::test]
async fn test_improve_skill() {
    let gm = GameMaster::new();

    let character = create_test_character();

    let skill = SkillDescriptorBuilder::default()
        .name("Acrobatics".to_string())
        .difficulty(SkillDifficulty::Hard)
        .base_attribute(AttributeType::DX)
        .points(2)
        .level(9) // 10 (DX) - 2 (Hard) + 1 (2 pts) = 9
        .build()
        .expect("Valid skill");

    let character_with_skill = gm
        .add_skill(character, skill)
        .await
        .expect("Add skill succeeded");

    let initial_points_spent = character_with_skill.points_spent;

    let (updated_character, _evidence) = gm
        .improve_skill(character_with_skill.clone(), "Acrobatics".to_string(), 2)
        .await
        .expect("Improve skill succeeded");

    // Find the improved skill
    let improved_skill = updated_character
        .skills
        .iter()
        .find(|s| s.name == "Acrobatics")
        .expect("Skill should exist");

    assert_eq!(
        improved_skill.points, 4,
        "Points should increase from 2 to 4"
    );
    assert!(
        improved_skill.level > character_with_skill.skills[0].level,
        "Level should increase"
    );
    assert_eq!(
        updated_character.points_spent,
        initial_points_spent + 2,
        "Character points spent should increase"
    );
}

#[tokio::test]
async fn test_improve_nonexistent_skill_fails() {
    let gm = GameMaster::new();

    let character = create_test_character();

    let result = gm
        .improve_skill(character, "Nonexistent".to_string(), 2)
        .await;

    assert!(result.is_err(), "Improving nonexistent skill should fail");
}

#[tokio::test]
async fn test_improve_skill_with_negative_points_fails() {
    let gm = GameMaster::new();

    let character = create_test_character();

    let skill = SkillDescriptorBuilder::default()
        .name("Climbing".to_string())
        .difficulty(SkillDifficulty::Average)
        .base_attribute(AttributeType::DX)
        .points(1)
        .level(9)
        .build()
        .expect("Valid skill");

    let character_with_skill = gm
        .add_skill(character, skill)
        .await
        .expect("Add skill succeeded");

    let result = gm
        .improve_skill(character_with_skill, "Climbing".to_string(), -1)
        .await;

    assert!(
        result.is_err(),
        "Improving skill with negative points should fail"
    );
}

#[tokio::test]
async fn test_calculate_effective_skill() {
    let gm = GameMaster::new();

    let character = create_test_character();

    let skill = SkillDescriptorBuilder::default()
        .name("Sword".to_string())
        .difficulty(SkillDifficulty::Average)
        .base_attribute(AttributeType::DX)
        .points(4)
        .level(11) // Base level
        .build()
        .expect("Valid skill");

    let character_with_skill = gm
        .add_skill(character, skill)
        .await
        .expect("Add skill succeeded");

    let effective_skill = gm
        .calculate_effective_skill(&character_with_skill, "Sword".to_string(), 0)
        .await
        .expect("Calculate succeeded");

    assert_eq!(effective_skill, 11, "Effective skill should equal base level");
}

#[tokio::test]
async fn test_calculate_effective_skill_with_modifiers() {
    let gm = GameMaster::new();

    let character = create_test_character();

    let skill = SkillDescriptorBuilder::default()
        .name("Stealth".to_string())
        .difficulty(SkillDifficulty::Average)
        .base_attribute(AttributeType::DX)
        .points(2)
        .level(10)
        .build()
        .expect("Valid skill");

    let character_with_skill = gm
        .add_skill(character, skill)
        .await
        .expect("Add skill succeeded");

    let effective_skill = gm
        .calculate_effective_skill(&character_with_skill, "Stealth".to_string(), 3)
        .await
        .expect("Calculate succeeded");

    assert_eq!(
        effective_skill, 13,
        "Effective skill should be base + modifier (10 + 3)"
    );
}

#[tokio::test]
async fn test_calculate_effective_skill_with_penalties() {
    let gm = GameMaster::new();

    let character = create_test_character();

    let skill = SkillDescriptorBuilder::default()
        .name("Lockpicking".to_string())
        .difficulty(SkillDifficulty::Average)
        .base_attribute(AttributeType::IQ)
        .points(2)
        .level(10)
        .build()
        .expect("Valid skill");

    let character_with_skill = gm
        .add_skill(character, skill)
        .await
        .expect("Add skill succeeded");

    let effective_skill = gm
        .calculate_effective_skill(&character_with_skill, "Lockpicking".to_string(), -4)
        .await
        .expect("Calculate succeeded");

    assert_eq!(
        effective_skill, 6,
        "Effective skill should be base + penalty (10 - 4)"
    );
}

#[tokio::test]
async fn test_calculate_nonexistent_skill_fails() {
    let gm = GameMaster::new();

    let character = create_test_character();

    let result = gm
        .calculate_effective_skill(&character, "Nonexistent".to_string(), 0)
        .await;

    assert!(
        result.is_err(),
        "Calculating nonexistent skill should fail"
    );
}
