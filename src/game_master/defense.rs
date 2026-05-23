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
        // Roll 3d6 for defense
        let dice_roll = self.roll_3d6();
        let roll = dice_roll.sum();

        // Calculate effective defense score with modifiers
        let retreat_bonus = if descriptor.retreating {
            RETREAT_BONUS
        } else {
            0
        };
        let defense_score = descriptor.defense_score + retreat_bonus
            - descriptor.feint_penalty
            - descriptor.deceptive_penalty;

        // Determine outcome
        let (success, margin) = Self::calculate_margin(roll, defense_score);

        // Check for critical outcomes
        let critical_success = Self::is_critical_success(roll, defense_score);
        let critical_failure = Self::is_critical_failure(roll, defense_score, success, margin);

        // Build result
        let result = DefenseRollResult {
            roll,
            defense_score,
            success,
            margin,
            critical_success,
            critical_failure,
        };

        // Mint proof tokens using credentials
        let roll_made = Established::prove(&ValidDefenseRoll);
        let outcome = Established::prove(&DefenseOutcomeChecked);

        // Compose evidence bundle
        let _evidence = DefenseResolutionEvidence { roll_made, outcome };

        Ok((result, Established::assert()))
    }

    async fn confirm_defense_success(
        &self,
        result: DefenseRollResult,
        _base_evidence: Established<DefenseResolutionEvidence>,
    ) -> CombatResult<Established<DefenseSuccessEvidence>> {
        // Verify the defense actually succeeded
        if !result.success {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Cannot confirm success for failed defense".to_string(),
                ),
            ));
        }

        // Reconstruct resolution evidence (zero-sized, exists only for type system)
        let roll_made = Established::prove(&ValidDefenseRoll);
        let outcome = Established::prove(&DefenseOutcomeChecked);
        let resolution = DefenseResolutionEvidence { roll_made, outcome };

        // Mint success proof
        let success = Established::prove(&DefenseWorked);

        // Compose success evidence
        let _evidence = DefenseSuccessEvidence {
            resolution,
            success,
        };

        Ok(Established::assert())
    }

    async fn confirm_defense_failure(
        &self,
        result: DefenseRollResult,
        _base_evidence: Established<DefenseResolutionEvidence>,
    ) -> CombatResult<Established<DefenseFailureEvidence>> {
        // Verify the defense actually failed
        if result.success {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Cannot confirm failure for successful defense".to_string(),
                ),
            ));
        }

        // Reconstruct resolution evidence
        let roll_made = Established::prove(&ValidDefenseRoll);
        let outcome = Established::prove(&DefenseOutcomeChecked);
        let resolution = DefenseResolutionEvidence { roll_made, outcome };

        // Mint failure proof
        let failure = Established::prove(&DefenseDidNotWork);

        // Compose failure evidence
        let _evidence = DefenseFailureEvidence {
            resolution,
            failure,
        };

        Ok(Established::assert())
    }

    async fn confirm_defense_critical_success(
        &self,
        result: DefenseRollResult,
        _success_evidence: Established<DefenseSuccessEvidence>,
    ) -> CombatResult<Established<DefenseCriticalSuccessEvidence>> {
        // Verify this was actually a critical success
        if !result.critical_success {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Defense was not a critical success".to_string(),
                ),
            ));
        }

        // Reconstruct resolution evidence
        let roll_made = Established::prove(&ValidDefenseRoll);
        let outcome = Established::prove(&DefenseOutcomeChecked);
        let resolution = DefenseResolutionEvidence { roll_made, outcome };

        // Reconstruct success evidence
        let success_proof = Established::prove(&DefenseWorked);
        let success = DefenseSuccessEvidence {
            resolution,
            success: success_proof,
        };

        // Mint critical success proof
        let critical = Established::prove(&DefenseCriticalWin);

        // Compose critical success evidence
        let _evidence = DefenseCriticalSuccessEvidence { success, critical };

        Ok(Established::assert())
    }

    async fn confirm_defense_critical_failure(
        &self,
        result: DefenseRollResult,
        _base_evidence: Established<DefenseResolutionEvidence>,
    ) -> CombatResult<Established<DefenseCriticalFailureEvidence>> {
        // Verify this was actually a critical failure
        if !result.critical_failure {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Defense was not a critical failure".to_string(),
                ),
            ));
        }

        // Reconstruct resolution evidence
        let roll_made = Established::prove(&ValidDefenseRoll);
        let outcome = Established::prove(&DefenseOutcomeChecked);
        let resolution = DefenseResolutionEvidence { roll_made, outcome };

        // Mint critical failure proof
        let critical = Established::prove(&DefenseCriticalMiss);

        // Compose critical failure evidence
        let _evidence = DefenseCriticalFailureEvidence {
            resolution,
            critical,
        };

        Ok(Established::assert())
    }
}
