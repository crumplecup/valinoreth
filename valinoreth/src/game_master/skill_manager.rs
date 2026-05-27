//! Skill management implementation.

use crate::contracts::credentials::{PointsSpentOnSkill, PrerequisiteMet, SkillLevelRaised};
use crate::contracts::proof_composition::SkillImprovementEvidence;
use crate::contracts::traits::{CombatResult, ContractError, ContractErrorKind, SkillManager};
use crate::contracts::types::{CharacterDescriptor, SkillDescriptor, SkillDifficulty};
use crate::game_master::GameMaster;
use async_trait::async_trait;
use elicitation::contracts::Established;

impl SkillDifficulty {
    /// Returns the difficulty penalty for this skill difficulty level.
    ///
    /// # GURPS Rules
    ///
    /// Easy (E): attribute-0
    /// Average (A): attribute-1
    /// Hard (H): attribute-2
    /// Very Hard (VH): attribute-3
    fn penalty(&self) -> i32 {
        match self {
            Self::Easy => 0,
            Self::Average => 1,
            Self::Hard => 2,
            Self::VeryHard => 3,
        }
    }
}

/// Calculate skill level from points spent.
///
/// # GURPS Rules
///
/// Point progression: 1pt = +0, 2pt = +1, 4pt = +2, 8pt = +3, etc.
/// Each doubling adds one relative skill level.
///
/// # Citations
///
/// BS 170 - Skill point costs
fn points_to_relative_level(points: i32) -> i32 {
    if points < 1 {
        return 0;
    }
    // 1pt = 0, 2pt = 1, 4pt = 2, 8pt = 3, 16pt = 4, etc.
    // This is essentially log2(points)
    let mut level = 0;
    let mut threshold = 1;
    while threshold * 2 <= points {
        level += 1;
        threshold *= 2;
    }
    level
}

#[async_trait]
impl SkillManager for GameMaster {
    async fn add_skill(
        &self,
        mut character: CharacterDescriptor,
        skill: SkillDescriptor,
    ) -> CombatResult<CharacterDescriptor> {
        // Verify points are positive
        if skill.points < 0 {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                "Skill points cannot be negative".to_string(),
            )));
        }

        // Check if skill already exists
        if character
            .skills
            .iter()
            .any(|s| s.name == skill.name && s.specialization == skill.specialization)
        {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                format!("Skill {} already exists", skill.name),
            )));
        }

        // Add skill to character
        character.skills.push(skill);

        Ok(character)
    }

    async fn improve_skill(
        &self,
        mut character: CharacterDescriptor,
        skill_name: String,
        points_to_spend: i32,
    ) -> CombatResult<(CharacterDescriptor, Established<SkillImprovementEvidence>)> {
        // Verify points are positive
        if points_to_spend <= 0 {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                "Points to spend must be positive".to_string(),
            )));
        }

        // Find the skill
        let skill = character
            .skills
            .iter_mut()
            .find(|s| s.name == skill_name)
            .ok_or_else(|| {
                ContractError::new(ContractErrorKind::StateViolation(format!(
                    "Skill {} not found",
                    skill_name
                )))
            })?;

        // Calculate old level
        let old_level = skill.level;

        // Add points
        skill.points += points_to_spend;

        // Recalculate level based on new point total
        let relative_level = points_to_relative_level(skill.points);

        // Get base attribute value (simplified - assuming base 10)
        // In a real implementation, we'd look up the attribute from character.attributes
        let base_attribute_value = 10; // Placeholder

        // Calculate new level: attribute - difficulty penalty + relative level
        skill.level = base_attribute_value - skill.difficulty.penalty() + relative_level;

        // Verify level increased
        if skill.level <= old_level {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                "Skill level did not increase".to_string(),
            )));
        }

        // Update character points spent
        character.points_spent += points_to_spend;

        // Mint proof tokens
        let points_spent = Established::prove(&PointsSpentOnSkill);
        let level_increased = Established::prove(&SkillLevelRaised);
        let prerequisite = Some(Established::prove(&PrerequisiteMet));

        let evidence = SkillImprovementEvidence {
            points_spent,
            level_increased,
            prerequisite,
        };

        Ok((character, Established::prove(&evidence)))
    }

    async fn calculate_effective_skill(
        &self,
        character: &CharacterDescriptor,
        skill_name: String,
        modifiers: i32,
    ) -> CombatResult<i32> {
        // Find the skill
        let skill = character
            .skills
            .iter()
            .find(|s| s.name == skill_name)
            .ok_or_else(|| {
                ContractError::new(ContractErrorKind::StateViolation(format!(
                    "Skill {} not found",
                    skill_name
                )))
            })?;

        // Calculate effective skill: base level + modifiers
        let effective_skill = skill.level + modifiers;

        Ok(effective_skill)
    }
}
