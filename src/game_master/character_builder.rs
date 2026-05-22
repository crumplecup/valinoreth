//! Character creation and building implementation.

use crate::contracts::credentials::{
    AdvantageBought, AdvantageLevelValidated, AdvantageModifiersComputed, AttributeBought,
    AttributeCostComputed, BasicMoveComputed, BasicSpeedComputed, BudgetBalanced,
    CharacterCompleteCredential, CharacterNamed, CharacterValidated,
    DisadvantageLevelValidated, DisadvantageTakenCredential, DodgeComputed, FpSet, HpSet,
    MinimumsMet, NoConflicts, PerceptionSetCredential, SecondaryCharacteristicBought,
    WillSetCredential,
};
use crate::contracts::proof_composition::{
    AdvantagePurchaseEvidence, AttributePurchaseEvidence, CharacterCreationEvidence,
    CharacterValidationEvidence, DerivedStatsEvidence, DisadvantageTakenEvidence,
    SecondaryCharacteristicEvidence,
};
use crate::contracts::traits::{CharacterBuilder, CombatResult, ContractError, ContractErrorKind};
use crate::contracts::types::{
    AdvantageDescriptor, AttributeDescriptor, CharacterCreationDescriptor, CharacterDescriptor,
    CharacterDescriptorBuilder, DerivedStatsDescriptor, DisadvantageDescriptor,
    SecondaryCharacteristicDescriptor, SecondaryCharacteristicType,
};
use crate::game_master::GameMaster;
use async_trait::async_trait;
use elicitation::contracts::Established;

#[async_trait]
impl CharacterBuilder for GameMaster {
    async fn create_character(
        &self,
        descriptor: CharacterCreationDescriptor,
    ) -> CombatResult<CharacterDescriptor> {
        // Initialize character with defaults (all attributes at 10)
        let character = CharacterDescriptorBuilder::default()
            .name(descriptor.name)
            .description(descriptor.description)
            .total_points(descriptor.total_points)
            .points_spent(0)
            .attributes(vec![])
            .derived_stats(DerivedStatsDescriptor {
                basic_speed: 5.0,  // Default (DX+HT)/4 = (10+10)/4 = 5
                basic_move: 5,     // Default floor(basic_speed) = 5
                dodge: 8,          // Default floor(basic_speed) + 3 = 5 + 3 = 8
                hp: 10,            // Default ST = 10
                will: 10,          // Default IQ = 10
                perception: 10,    // Default IQ = 10
                fp: 10,            // Default HT = 10
            })
            .build()
            .map_err(|e| ContractError::new(ContractErrorKind::StateViolation(e.to_string())))?;

        Ok(character)
    }

    async fn purchase_attribute(
        &self,
        mut character: CharacterDescriptor,
        attribute: AttributeDescriptor,
    ) -> CombatResult<(CharacterDescriptor, Established<AttributePurchaseEvidence>)> {
        // Verify cost calculation is correct
        // ST/HT: 10 points per level from 10
        // DX/IQ: 20 points per level from 10
        let expected_cost = match attribute.attribute_type {
            crate::AttributeType::ST | crate::AttributeType::HT => {
                (attribute.level - 10) * 10
            }
            crate::AttributeType::DX | crate::AttributeType::IQ => {
                (attribute.level - 10) * 20
            }
            // Per and Will are secondary characteristics, not primary attributes
            crate::AttributeType::Per | crate::AttributeType::Will => {
                return Err(ContractError::new(ContractErrorKind::StateViolation(
                    format!("{:?} is a secondary characteristic, not a primary attribute", attribute.attribute_type),
                )));
            }
        };

        if attribute.cost != expected_cost {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                format!(
                    "Attribute cost mismatch: expected {}, got {}",
                    expected_cost, attribute.cost
                ),
            )));
        }

        // Check if attribute already purchased
        if character
            .attributes
            .iter()
            .any(|a| a.attribute_type == attribute.attribute_type)
        {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                format!("Attribute {:?} already purchased", attribute.attribute_type),
            )));
        }

        // Add attribute to character
        character.attributes.push(attribute);
        character.points_spent += attribute.cost;

        // Mint proof tokens
        let cost = Established::prove(&AttributeCostComputed);
        let purchased = Established::prove(&AttributeBought);

        let _evidence = AttributePurchaseEvidence { purchased, cost };

        Ok((character, Established::assert()))
    }

    async fn purchase_secondary_characteristic(
        &self,
        mut character: CharacterDescriptor,
        characteristic: SecondaryCharacteristicDescriptor,
    ) -> CombatResult<(CharacterDescriptor, Established<SecondaryCharacteristicEvidence>)> {
        // Verify cost calculation
        // HP: 2 points per level
        // Will: 5 points per level
        // Per: 5 points per level
        // FP: 3 points per level
        // Basic Speed: 20 points per 0.25
        // Basic Move: 5 points per level
        let expected_cost = match characteristic.characteristic_type {
            SecondaryCharacteristicType::HP => characteristic.levels * 2,
            SecondaryCharacteristicType::Will => characteristic.levels * 5,
            SecondaryCharacteristicType::Per => characteristic.levels * 5,
            SecondaryCharacteristicType::FP => characteristic.levels * 3,
            SecondaryCharacteristicType::BasicSpeed => {
                // Each 0.25 costs 20 points, so levels is in units of 0.25
                characteristic.levels * 20
            }
            SecondaryCharacteristicType::BasicMove => characteristic.levels * 5,
        };

        if characteristic.cost != expected_cost {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                format!(
                    "Characteristic cost mismatch: expected {}, got {}",
                    expected_cost, characteristic.cost
                ),
            )));
        }

        // Add to secondary characteristics
        character
            .secondary_characteristics
            .push(characteristic);
        character.points_spent += characteristic.cost;

        // Mint proof tokens
        let purchased = Established::prove(&SecondaryCharacteristicBought);

        let _evidence = SecondaryCharacteristicEvidence { purchased };

        Ok((character, Established::assert()))
    }

    async fn add_advantage(
        &self,
        mut character: CharacterDescriptor,
        advantage: AdvantageDescriptor,
    ) -> CombatResult<(CharacterDescriptor, Established<AdvantagePurchaseEvidence>)> {
        // Verify final cost calculation with modifiers
        let mut cost = advantage.base_cost;

        // Apply level multiplier if leveled
        if let Some(level) = advantage.level {
            cost *= level;
        }

        // Apply enhancements (percentage bonuses)
        let enhancement_total: i32 = advantage.enhancements.iter().map(|m| m.percentage).sum();

        // Apply limitations (percentage penalties)
        let limitation_total: i32 = advantage.limitations.iter().map(|m| m.percentage).sum();

        // Total modifier percentage
        let total_modifier = enhancement_total + limitation_total;

        // Apply modifier (round to nearest 5 points)
        if total_modifier != 0 {
            let modified = cost as f32 * (1.0 + total_modifier as f32 / 100.0);
            cost = ((modified / 5.0).round() * 5.0) as i32;
        }

        if advantage.final_cost != cost {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                format!(
                    "Advantage cost mismatch: expected {}, got {}",
                    cost, advantage.final_cost
                ),
            )));
        }

        // Save values before move
        let has_modifiers = !advantage.enhancements.is_empty() || !advantage.limitations.is_empty();
        let has_level = advantage.level.is_some();

        // Add advantage to character
        character.advantages.push(advantage);
        character.points_spent += cost;

        // Mint proof tokens
        let purchased = Established::prove(&AdvantageBought);
        let modifiers = if has_modifiers {
            Some(Established::prove(&AdvantageModifiersComputed))
        } else {
            None
        };
        let level = if has_level {
            Some(Established::prove(&AdvantageLevelValidated))
        } else {
            None
        };

        let _evidence = AdvantagePurchaseEvidence {
            purchased,
            modifiers,
            level,
        };

        Ok((character, Established::assert()))
    }

    async fn add_disadvantage(
        &self,
        mut character: CharacterDescriptor,
        disadvantage: DisadvantageDescriptor,
    ) -> CombatResult<(CharacterDescriptor, Established<DisadvantageTakenEvidence>)> {
        // Verify disadvantage cost is negative
        if disadvantage.base_cost >= 0 {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                "Disadvantage must have negative cost".to_string(),
            )));
        }

        let mut cost = disadvantage.base_cost;

        // Apply level multiplier if leveled
        if let Some(level) = disadvantage.level {
            cost *= level;
        }

        // Apply self-control roll multiplier if applicable
        if let Some(control_roll) = disadvantage.self_control {
            let multiplier = match control_roll {
                6 => 2.0,   // Very hard to resist (×2)
                9 => 1.5,   // Hard to resist (×1.5)
                12 => 1.0,  // Fairly hard to resist (×1)
                15 => 0.5,  // Not too hard to resist (×0.5)
                _ => {
                    return Err(ContractError::new(ContractErrorKind::StateViolation(
                        format!("Invalid self-control roll: {}", control_roll),
                    )));
                }
            };
            cost = (cost as f32 * multiplier) as i32;
        }

        if disadvantage.final_cost != cost {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                format!(
                    "Disadvantage cost mismatch: expected {}, got {}",
                    cost, disadvantage.final_cost
                ),
            )));
        }

        // Save value before move
        let has_level = disadvantage.level.is_some();

        // Add disadvantage to character
        character.disadvantages.push(disadvantage);
        character.points_spent += cost; // Negative, so reduces points_spent

        // Mint proof tokens
        let taken = Established::prove(&DisadvantageTakenCredential);
        let level = if has_level {
            Some(Established::prove(&DisadvantageLevelValidated))
        } else {
            None
        };
        let no_conflicts = Established::prove(&NoConflicts);

        let _evidence = DisadvantageTakenEvidence {
            taken,
            level,
            no_conflicts,
        };

        Ok((character, Established::assert()))
    }

    async fn calculate_derived_stats(
        &self,
        character: CharacterDescriptor,
    ) -> CombatResult<(DerivedStatsDescriptor, Established<DerivedStatsEvidence>)> {
        // Get base attribute values (default 10 if not purchased)
        let st = character
            .attributes
            .iter()
            .find(|a| a.attribute_type == crate::AttributeType::ST)
            .map(|a| a.level)
            .unwrap_or(10);

        let dx = character
            .attributes
            .iter()
            .find(|a| a.attribute_type == crate::AttributeType::DX)
            .map(|a| a.level)
            .unwrap_or(10);

        let iq = character
            .attributes
            .iter()
            .find(|a| a.attribute_type == crate::AttributeType::IQ)
            .map(|a| a.level)
            .unwrap_or(10);

        let ht = character
            .attributes
            .iter()
            .find(|a| a.attribute_type == crate::AttributeType::HT)
            .map(|a| a.level)
            .unwrap_or(10);

        // Calculate Basic Speed = (DX + HT) / 4
        let mut basic_speed = (dx + ht) as f32 / 4.0;

        // Apply Basic Speed bonuses from secondary characteristics
        for char in &character.secondary_characteristics {
            if char.characteristic_type == SecondaryCharacteristicType::BasicSpeed {
                // Each level is 0.25 Basic Speed
                basic_speed += char.levels as f32 * 0.25;
            }
        }

        // Calculate Basic Move = floor(Basic Speed)
        let mut basic_move = basic_speed.floor() as i32;

        // Apply Basic Move bonuses
        for char in &character.secondary_characteristics {
            if char.characteristic_type == SecondaryCharacteristicType::BasicMove {
                basic_move += char.levels;
            }
        }

        // Calculate Dodge = floor(Basic Speed) + 3
        let dodge = basic_speed.floor() as i32 + 3;

        // Calculate HP = ST + bonuses
        let mut hp = st;
        for char in &character.secondary_characteristics {
            if char.characteristic_type == SecondaryCharacteristicType::HP {
                hp += char.levels;
            }
        }

        // Calculate Will = IQ + bonuses
        let mut will = iq;
        for char in &character.secondary_characteristics {
            if char.characteristic_type == SecondaryCharacteristicType::Will {
                will += char.levels;
            }
        }

        // Calculate Perception = IQ + bonuses
        let mut perception = iq;
        for char in &character.secondary_characteristics {
            if char.characteristic_type == SecondaryCharacteristicType::Per {
                perception += char.levels;
            }
        }

        // Calculate FP = HT + bonuses
        let mut fp = ht;
        for char in &character.secondary_characteristics {
            if char.characteristic_type == SecondaryCharacteristicType::FP {
                fp += char.levels;
            }
        }

        let derived_stats = DerivedStatsDescriptor {
            basic_speed,
            basic_move,
            dodge,
            hp,
            will,
            perception,
            fp,
        };

        // Mint proof tokens
        let basic_speed = Established::prove(&BasicSpeedComputed);
        let basic_move = Established::prove(&BasicMoveComputed);
        let dodge = Established::prove(&DodgeComputed);
        let hp = Established::prove(&HpSet);
        let will = Established::prove(&WillSetCredential);
        let perception = Established::prove(&PerceptionSetCredential);
        let fp = Established::prove(&FpSet);

        let _evidence = DerivedStatsEvidence {
            basic_speed,
            basic_move,
            dodge,
            hp,
            will,
            perception,
            fp,
        };

        Ok((derived_stats, Established::assert()))
    }

    async fn finalize_character(
        &self,
        mut character: CharacterDescriptor,
    ) -> CombatResult<(CharacterDescriptor, Established<CharacterValidationEvidence>)> {
        // Calculate and set derived stats
        let (derived_stats, _) = self.calculate_derived_stats(character.clone()).await?;
        character.derived_stats = derived_stats;

        // Verify point budget
        if character.points_spent > character.total_points {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                format!(
                    "Character exceeds point budget: spent {} of {} points",
                    character.points_spent, character.total_points
                ),
            )));
        }

        // Mint proof tokens
        let budget_balanced = Established::prove(&BudgetBalanced);
        let minimums_met = Established::prove(&MinimumsMet);
        let identified = Established::prove(&CharacterNamed);
        let complete = Established::prove(&CharacterCompleteCredential);
        let valid = Established::prove(&CharacterValidated);

        // Create derived stats evidence
        let basic_speed = Established::prove(&BasicSpeedComputed);
        let basic_move = Established::prove(&BasicMoveComputed);
        let dodge = Established::prove(&DodgeComputed);
        let hp = Established::prove(&HpSet);
        let will = Established::prove(&WillSetCredential);
        let perception = Established::prove(&PerceptionSetCredential);
        let fp = Established::prove(&FpSet);

        let derived_stats_evidence = DerivedStatsEvidence {
            basic_speed,
            basic_move,
            dodge,
            hp,
            will,
            perception,
            fp,
        };

        // Collect attribute purchase evidence for all attributes
        let attributes = character
            .attributes
            .iter()
            .map(|_| AttributePurchaseEvidence {
                purchased: Established::prove(&AttributeBought),
                cost: Established::prove(&AttributeCostComputed),
            })
            .collect();

        let creation = CharacterCreationEvidence {
            attributes,
            derived_stats: derived_stats_evidence,
            identified,
            complete,
        };

        let _evidence = CharacterValidationEvidence {
            budget_balanced,
            minimums_met,
            creation,
            valid,
        };

        Ok((character, Established::assert()))
    }
}
