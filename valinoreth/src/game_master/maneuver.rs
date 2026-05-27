//! Maneuver execution implementation.

use crate::contracts::credentials::{
    AttackOutcomeChecked, FeintWon, RapidStrikePerformed, ValidAttackRoll,
};
use crate::contracts::proof_composition::{
    AttackResolutionEvidence, FeintEvidence, RapidStrikeEvidence,
};
use crate::contracts::traits::{CombatResult, ManeuverExecutor};
use crate::contracts::types::{
    AttackRollResult, FeintDescriptor, FeintResult, RapidStrikeDescriptor, SkillCheckResult,
};
use crate::game_master::GameMaster;
use async_trait::async_trait;
use elicitation::contracts::Established;

#[async_trait]
impl ManeuverExecutor for GameMaster {
    async fn execute_feint(
        &self,
        descriptor: FeintDescriptor,
    ) -> CombatResult<(FeintResult, Established<FeintEvidence>)> {
        let attacker = SkillCheckResult::new(self.roll_3d6().sum(), descriptor.attacker_skill);
        let defender = SkillCheckResult::new(self.roll_3d6().sum(), descriptor.defender_skill);

        let (attacker_won, margin) = match (attacker.success, defender.success) {
            (true, true) => {
                if attacker.margin > defender.margin {
                    (true, attacker.margin)
                } else {
                    (false, defender.margin)
                }
            }
            (true, false) => (true, attacker.margin),
            (false, true) => (false, defender.margin),
            (false, false) => (false, 0),
        };

        let result = FeintResult {
            attacker_roll: attacker.roll,
            defender_roll: defender.roll,
            attacker_success: attacker_won,
            margin,
        };

        let feint = Established::prove(&FeintWon);
        let evidence = FeintEvidence { feint, margin: if attacker_won { margin } else { 0 } };

        Ok((result, Established::prove(&evidence)))
    }

    async fn execute_rapid_strike(
        &self,
        descriptor: RapidStrikeDescriptor,
    ) -> CombatResult<(Vec<AttackRollResult>, Established<RapidStrikeEvidence>)> {
        let mut attack_results = Vec::new();
        let mut attack_evidences = Vec::new();

        for _ in 0..descriptor.attack_count {
            let effective_skill = descriptor.base_skill + descriptor.penalty_per_attack;
            let result = AttackRollResult::new(self.roll_3d6().sum(), effective_skill);
            attack_results.push(result);

            let roll_made = Established::prove(&ValidAttackRoll);
            let outcome = Established::prove(&AttackOutcomeChecked);
            attack_evidences.push(AttackResolutionEvidence { roll_made, outcome });
        }

        let rapid_strike = Established::prove(&RapidStrikePerformed);
        let evidence = RapidStrikeEvidence { rapid_strike, attacks: attack_evidences };

        Ok((attack_results, Established::prove(&evidence)))
    }
}
