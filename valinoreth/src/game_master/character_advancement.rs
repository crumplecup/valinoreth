//! Character advancement and experience implementation.

use crate::contracts::credentials::{
    AttributeBought, AttributeCostComputed, BasicMoveComputed, BasicSpeedComputed,
    CharacterCompleteCredential, CharacterNamed, DodgeComputed, FpSet, HpSet,
    PerceptionSetCredential, WillSetCredential,
};
use crate::contracts::proof_composition::{
    AttributePurchaseEvidence, CharacterCreationEvidence, DerivedStatsEvidence,
};
use crate::contracts::traits::{
    CharacterAdvancement, CharacterBuilder, CharacterImprovement, CombatResult, ContractError,
    ContractErrorKind, SkillManager,
};
use crate::contracts::types::{AttributeDescriptor, AttributeType, CharacterDescriptor};
use crate::game_master::GameMaster;
use async_trait::async_trait;
use elicitation::contracts::Established;

#[async_trait]
impl CharacterAdvancement for GameMaster {
    async fn award_experience(
        &self,
        mut character: CharacterDescriptor,
        points: i32,
    ) -> CombatResult<CharacterDescriptor> {
        // Verify points are positive
        if points <= 0 {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                "Experience award must be positive".to_string(),
            )));
        }

        // Add points to total
        character.total_points += points;

        Ok(character)
    }

    async fn spend_experience(
        &self,
        character: CharacterDescriptor,
        improvement: CharacterImprovement,
    ) -> CombatResult<(CharacterDescriptor, Established<CharacterCreationEvidence>)> {
        match improvement {
            CharacterImprovement::Attribute {
                attribute_type,
                levels,
            } => {
                // Verify levels are positive
                if levels <= 0 {
                    return Err(ContractError::new(ContractErrorKind::StateViolation(
                        "Attribute improvement levels must be positive".to_string(),
                    )));
                }

                // Get current attribute level (or 10 if not purchased)
                let current_level = character
                    .attributes
                    .iter()
                    .find(|a| a.attribute_type == attribute_type)
                    .map(|a| a.level)
                    .unwrap_or(10);

                let new_level = current_level + levels;

                // Calculate cost for new level
                let cost = match attribute_type {
                    AttributeType::ST | AttributeType::HT => (new_level - 10) * 10,
                    AttributeType::DX | AttributeType::IQ => (new_level - 10) * 20,
                    // Per and Will are secondary characteristics
                    AttributeType::Per | AttributeType::Will => {
                        return Err(ContractError::new(ContractErrorKind::StateViolation(
                            format!(
                                "{:?} is a secondary characteristic, not a primary attribute",
                                attribute_type
                            ),
                        )));
                    }
                };

                // Create attribute descriptor
                let attribute = AttributeDescriptor {
                    attribute_type,
                    level: new_level,
                    cost,
                };

                // Remove old attribute if it exists
                let mut updated_character = character.clone();
                updated_character
                    .attributes
                    .retain(|a| a.attribute_type != attribute_type);

                // Use CharacterBuilder to purchase the attribute
                let (character_with_attr, _) = self
                    .purchase_attribute(updated_character, attribute)
                    .await?;

                // Create evidence
                let evidence = self.create_advancement_evidence(character_with_attr.clone())?;

                Ok((character_with_attr, evidence))
            }

            CharacterImprovement::Skill { name, points } => {
                // Use SkillManager to improve the skill
                let (updated_character, _) =
                    self.improve_skill(character.clone(), name, points).await?;

                // Create evidence
                let evidence = self.create_advancement_evidence(updated_character.clone())?;

                Ok((updated_character, evidence))
            }

            CharacterImprovement::Advantage { advantage } => {
                // Use CharacterBuilder to add the advantage
                let (updated_character, _) =
                    self.add_advantage(character.clone(), advantage).await?;

                // Create evidence
                let evidence = self.create_advancement_evidence(updated_character.clone())?;

                Ok((updated_character, evidence))
            }

            CharacterImprovement::BuyOffDisadvantage { name } => {
                // Find the disadvantage
                let disadvantage = character
                    .disadvantages
                    .iter()
                    .find(|d| d.name == name)
                    .ok_or_else(|| {
                        ContractError::new(ContractErrorKind::StateViolation(format!(
                            "Disadvantage {} not found",
                            name
                        )))
                    })?;

                // Cost to buy off is the absolute value of the disadvantage cost
                let cost = disadvantage.final_cost.abs();

                // Remove the disadvantage
                let mut updated_character = character.clone();
                updated_character.disadvantages.retain(|d| d.name != name);
                updated_character.points_spent += cost;

                // Create evidence
                let evidence = self.create_advancement_evidence(updated_character.clone())?;

                Ok((updated_character, evidence))
            }
        }
    }
}

impl GameMaster {
    /// Helper to create advancement evidence.
    fn create_advancement_evidence(
        &self,
        character: CharacterDescriptor,
    ) -> CombatResult<Established<CharacterCreationEvidence>> {
        // Collect attribute purchase evidence for all attributes
        let attributes = character
            .attributes
            .iter()
            .map(|_| AttributePurchaseEvidence {
                purchased: Established::prove(&AttributeBought),
                cost: Established::prove(&AttributeCostComputed),
            })
            .collect();

        // Create derived stats evidence
        let basic_speed = Established::prove(&BasicSpeedComputed);
        let basic_move = Established::prove(&BasicMoveComputed);
        let dodge = Established::prove(&DodgeComputed);
        let hp = Established::prove(&HpSet);
        let will = Established::prove(&WillSetCredential);
        let perception = Established::prove(&PerceptionSetCredential);
        let fp = Established::prove(&FpSet);

        let derived_stats = DerivedStatsEvidence {
            basic_speed,
            basic_move,
            dodge,
            hp,
            will,
            perception,
            fp,
        };

        let identified = Established::prove(&CharacterNamed);
        let complete = Established::prove(&CharacterCompleteCredential);

        let evidence = CharacterCreationEvidence {
            attributes,
            derived_stats,
            identified,
            complete,
        };

        Ok(Established::prove(&evidence))
    }
}
