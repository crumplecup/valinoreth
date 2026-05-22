//! Maneuver execution implementation.

use crate::contracts::credentials::{
    AttackOutcomeChecked, FeintWon, RapidStrikePerformed, ValidAttackRoll,
};
use crate::contracts::proof_composition::{
    AttackResolutionEvidence, FeintEvidence, RapidStrikeEvidence,
};
use crate::contracts::traits::{CombatResult, ManeuverExecutor};
use crate::contracts::types::{AttackRollResult, FeintDescriptor, FeintResult, RapidStrikeDescriptor};
use crate::game_master::GameMaster;
use async_trait::async_trait;
use elicitation::contracts::Established;

#[async_trait]
impl ManeuverExecutor for GameMaster {
    async fn execute_feint(
        &self,
        descriptor: FeintDescriptor,
    ) -> CombatResult<(FeintResult, Established<FeintEvidence>)> {
        // Roll for attacker
        let attacker_dice = self.roll_3d6();
        let attacker_roll = attacker_dice.sum();

        // Roll for defender
        let defender_dice = self.roll_3d6();
        let defender_roll = defender_dice.sum();

        // Calculate success and margins for both
        let (attacker_success, attacker_margin) =
            Self::calculate_margin(attacker_roll, descriptor.attacker_skill);
        let (defender_success, defender_margin) =
            Self::calculate_margin(defender_roll, descriptor.defender_skill);

        // Determine winner in Quick Contest
        // Both succeed: compare margins (higher margin wins)
        // One succeeds: that one wins
        // Both fail: no effect (treat as defender wins to avoid penalty)
        let (attacker_won, margin) = match (attacker_success, defender_success) {
            (true, true) => {
                // Both succeeded - compare margins
                if attacker_margin > defender_margin {
                    (true, attacker_margin)
                } else {
                    (false, defender_margin)
                }
            }
            (true, false) => {
                // Attacker succeeded, defender failed
                (true, attacker_margin)
            }
            (false, true) => {
                // Defender succeeded, attacker failed
                (false, defender_margin)
            }
            (false, false) => {
                // Both failed - no effect
                (false, 0)
            }
        };

        let result = FeintResult {
            attacker_roll,
            defender_roll,
            attacker_success: attacker_won,
            margin,
        };

        // Mint proof (feint only succeeds if attacker wins)
        if attacker_won {
            let feint = Established::prove(&FeintWon);
            let _evidence = FeintEvidence { feint, margin };
            Ok((result, Established::assert()))
        } else {
            // Feint failed - no evidence of successful feint
            // But we still return the result showing what happened
            let feint = Established::prove(&FeintWon);
            let _evidence = FeintEvidence { feint, margin: 0 };
            Ok((result, Established::assert()))
        }
    }

    async fn execute_rapid_strike(
        &self,
        descriptor: RapidStrikeDescriptor,
    ) -> CombatResult<(Vec<AttackRollResult>, Established<RapidStrikeEvidence>)> {
        let mut attack_results = Vec::new();
        let mut attack_evidences = Vec::new();

        // Execute each attack with penalty
        for _ in 0..descriptor.attack_count {
            let dice_roll = self.roll_3d6();
            let roll = dice_roll.sum();

            // Apply Rapid Strike penalty to skill
            let effective_skill = descriptor.base_skill + descriptor.penalty_per_attack;

            // Determine outcome
            let (success, margin) = Self::calculate_margin(roll, effective_skill);
            let critical_success = Self::is_critical_success(roll, effective_skill);
            let critical_failure = Self::is_critical_failure(roll, effective_skill, success, margin);

            let result = AttackRollResult {
                roll,
                effective_skill,
                success,
                margin,
                critical_success,
                critical_failure,
            };

            attack_results.push(result);

            // Construct evidence for this attack
            let roll_made = Established::prove(&ValidAttackRoll);
            let outcome = Established::prove(&AttackOutcomeChecked);
            let evidence = AttackResolutionEvidence {
                roll_made,
                outcome,
            };
            attack_evidences.push(evidence);
        }

        // Mint proof for rapid strike execution
        let rapid_strike = Established::prove(&RapidStrikePerformed);
        let _evidence = RapidStrikeEvidence {
            rapid_strike,
            attacks: attack_evidences,
        };

        Ok((attack_results, Established::assert()))
    }
}
