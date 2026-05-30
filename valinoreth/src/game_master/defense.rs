//! Defense resolution implementation.

use crate::contracts::credentials::{
    DefenseCriticalMiss, DefenseCriticalWin, DefenseDidNotWork, DefenseOutcomeChecked,
    DefenseWorked, ValidDefenseRoll,
};
use crate::contracts::proof_composition::{
    DefenseCriticalFailureEvidence, DefenseCriticalSuccessEvidence, DefenseFailureEvidence,
    DefenseResolutionEvidence, DefenseSuccessEvidence,
};
use crate::contracts::traits::{CombatResult, DefenseResolver};
use crate::contracts::types::{DefenseDescriptor, DefenseRollResult};
use crate::game_master::GameMaster;
use async_trait::async_trait;
use elicitation::contracts::Established;

/// Retreat bonus to active defenses.
///
/// # GURPS Rules
///
/// When retreating, add +3 to Dodge, +1 to Parry/Block.
/// For simplicity, GameMaster treats all retreats as +3.
///
/// # Citations
///
/// BS 377 - Retreat
const RETREAT_BONUS: i32 = 3;

#[async_trait]
impl DefenseResolver for GameMaster {
    async fn resolve_defense(
        &self,
        descriptor: DefenseDescriptor,
    ) -> CombatResult<(DefenseRollResult, Established<DefenseResolutionEvidence>)> {
        let retreat_bonus = if descriptor.retreating {
            RETREAT_BONUS
        } else {
            0
        };
        let defense_score = descriptor.defense_score + retreat_bonus
            - descriptor.feint_penalty
            - descriptor.deceptive_penalty;
        let result = DefenseRollResult::new(self.roll_3d6().sum(), defense_score);

        let roll_made = Established::prove(&ValidDefenseRoll);
        let outcome = Established::prove(&DefenseOutcomeChecked);
        let evidence = DefenseResolutionEvidence { roll_made, outcome };

        Ok((result, Established::prove(&evidence)))
    }

    async fn confirm_defense_success(
        &self,
        result: DefenseRollResult,
        _base_evidence: Established<DefenseResolutionEvidence>,
    ) -> CombatResult<Established<DefenseSuccessEvidence>> {
        if !result.success {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Cannot confirm success for failed defense".to_string(),
                ),
            ));
        }

        let roll_made = Established::prove(&ValidDefenseRoll);
        let outcome = Established::prove(&DefenseOutcomeChecked);
        let resolution = DefenseResolutionEvidence { roll_made, outcome };
        let success = Established::prove(&DefenseWorked);
        let evidence = DefenseSuccessEvidence {
            resolution,
            success,
        };

        Ok(Established::prove(&evidence))
    }

    async fn confirm_defense_failure(
        &self,
        result: DefenseRollResult,
        _base_evidence: Established<DefenseResolutionEvidence>,
    ) -> CombatResult<Established<DefenseFailureEvidence>> {
        if result.success {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Cannot confirm failure for successful defense".to_string(),
                ),
            ));
        }

        let roll_made = Established::prove(&ValidDefenseRoll);
        let outcome = Established::prove(&DefenseOutcomeChecked);
        let resolution = DefenseResolutionEvidence { roll_made, outcome };
        let failure = Established::prove(&DefenseDidNotWork);
        let evidence = DefenseFailureEvidence {
            resolution,
            failure,
        };

        Ok(Established::prove(&evidence))
    }

    async fn confirm_defense_critical_success(
        &self,
        result: DefenseRollResult,
        _success_evidence: Established<DefenseSuccessEvidence>,
    ) -> CombatResult<Established<DefenseCriticalSuccessEvidence>> {
        if !result.critical_success {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Defense was not a critical success".to_string(),
                ),
            ));
        }

        let roll_made = Established::prove(&ValidDefenseRoll);
        let outcome = Established::prove(&DefenseOutcomeChecked);
        let resolution = DefenseResolutionEvidence { roll_made, outcome };
        let success_proof = Established::prove(&DefenseWorked);
        let success = DefenseSuccessEvidence {
            resolution,
            success: success_proof,
        };
        let critical = Established::prove(&DefenseCriticalWin);
        let evidence = DefenseCriticalSuccessEvidence { success, critical };

        Ok(Established::prove(&evidence))
    }

    async fn confirm_defense_critical_failure(
        &self,
        result: DefenseRollResult,
        _base_evidence: Established<DefenseResolutionEvidence>,
    ) -> CombatResult<Established<DefenseCriticalFailureEvidence>> {
        if !result.critical_failure {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Defense was not a critical failure".to_string(),
                ),
            ));
        }

        let roll_made = Established::prove(&ValidDefenseRoll);
        let outcome = Established::prove(&DefenseOutcomeChecked);
        let resolution = DefenseResolutionEvidence { roll_made, outcome };
        let critical = Established::prove(&DefenseCriticalMiss);
        let evidence = DefenseCriticalFailureEvidence {
            resolution,
            critical,
        };

        Ok(Established::prove(&evidence))
    }
}
