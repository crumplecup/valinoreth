//! Skill check resolution implementation.

use crate::contracts::credentials::{
    SkillCheckCriticalHit, SkillCheckCriticalMiss, SkillCheckHit, SkillCheckMiss,
    SkillCheckOutcomeChecked, ValidSkillCheckRoll,
};
use crate::contracts::proof_composition::{
    SkillCheckCriticalFailureEvidence, SkillCheckCriticalSuccessEvidence,
    SkillCheckFailureEvidence, SkillCheckResolutionEvidence, SkillCheckSuccessEvidence,
};
use crate::contracts::traits::{CombatResult, SkillCheckExecutor};
use crate::contracts::types::{SkillCheckDescriptor, SkillCheckResult};
use crate::game_master::GameMaster;
use async_trait::async_trait;
use elicitation::contracts::Established;

#[async_trait]
impl SkillCheckExecutor for GameMaster {
    async fn resolve_skill_check(
        &self,
        descriptor: SkillCheckDescriptor,
    ) -> CombatResult<(SkillCheckResult, Established<SkillCheckResolutionEvidence>)> {
        // Roll 3d6 for skill check
        let dice_roll = self.roll_3d6();
        let roll = dice_roll.sum();

        // Calculate effective skill with all modifiers
        let mut effective_skill = descriptor.effective_skill;
        effective_skill += descriptor.situational_modifier;
        effective_skill += descriptor.task_difficulty;
        effective_skill += descriptor.time_modifier;
        if let Some(bonus) = descriptor.complementary_bonus {
            effective_skill += bonus;
        }

        // Determine outcome
        let (success, margin) = Self::calculate_margin(roll, effective_skill);

        // Check for critical outcomes
        let critical_success = Self::is_critical_success(roll, effective_skill);
        let critical_failure = Self::is_critical_failure(roll, effective_skill, success, margin);

        // Build result
        let result = SkillCheckResult {
            roll,
            effective_skill,
            success,
            margin,
            critical_success,
            critical_failure,
        };

        // Mint proof tokens using credentials
        let roll_made = Established::prove(&ValidSkillCheckRoll);
        let outcome = Established::prove(&SkillCheckOutcomeChecked);

        // Compose evidence bundle
        let _evidence = SkillCheckResolutionEvidence { roll_made, outcome };

        Ok((result, Established::assert()))
    }

    async fn confirm_skill_check_success(
        &self,
        result: SkillCheckResult,
        _base_evidence: Established<SkillCheckResolutionEvidence>,
    ) -> CombatResult<Established<SkillCheckSuccessEvidence>> {
        // Verify the skill check actually succeeded
        if !result.success {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Cannot confirm success for failed skill check".to_string(),
                ),
            ));
        }

        // Reconstruct resolution evidence (zero-sized, exists only for type system)
        let roll_made = Established::prove(&ValidSkillCheckRoll);
        let outcome = Established::prove(&SkillCheckOutcomeChecked);
        let resolution = SkillCheckResolutionEvidence { roll_made, outcome };

        // Mint success proof
        let success = Established::prove(&SkillCheckHit);

        // Compose success evidence
        let _evidence = SkillCheckSuccessEvidence {
            resolution,
            success,
        };

        Ok(Established::assert())
    }

    async fn confirm_skill_check_failure(
        &self,
        result: SkillCheckResult,
        _base_evidence: Established<SkillCheckResolutionEvidence>,
    ) -> CombatResult<Established<SkillCheckFailureEvidence>> {
        // Verify the skill check actually failed
        if result.success {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Cannot confirm failure for successful skill check".to_string(),
                ),
            ));
        }

        // Reconstruct resolution evidence
        let roll_made = Established::prove(&ValidSkillCheckRoll);
        let outcome = Established::prove(&SkillCheckOutcomeChecked);
        let resolution = SkillCheckResolutionEvidence { roll_made, outcome };

        // Mint failure proof
        let failure = Established::prove(&SkillCheckMiss);

        // Compose failure evidence
        let _evidence = SkillCheckFailureEvidence {
            resolution,
            failure,
        };

        Ok(Established::assert())
    }

    async fn confirm_critical_skill_success(
        &self,
        result: SkillCheckResult,
        _success_evidence: Established<SkillCheckSuccessEvidence>,
    ) -> CombatResult<Established<SkillCheckCriticalSuccessEvidence>> {
        // Verify this was actually a critical success
        if !result.critical_success {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Skill check was not a critical success".to_string(),
                ),
            ));
        }

        // Reconstruct resolution evidence
        let roll_made = Established::prove(&ValidSkillCheckRoll);
        let outcome = Established::prove(&SkillCheckOutcomeChecked);
        let resolution = SkillCheckResolutionEvidence { roll_made, outcome };

        // Reconstruct success evidence
        let success_proof = Established::prove(&SkillCheckHit);
        let success = SkillCheckSuccessEvidence {
            resolution,
            success: success_proof,
        };

        // Mint critical success proof
        let critical = Established::prove(&SkillCheckCriticalHit);

        // Compose critical success evidence
        let _evidence = SkillCheckCriticalSuccessEvidence { success, critical };

        Ok(Established::assert())
    }

    async fn confirm_critical_skill_failure(
        &self,
        result: SkillCheckResult,
        _base_evidence: Established<SkillCheckResolutionEvidence>,
    ) -> CombatResult<Established<SkillCheckCriticalFailureEvidence>> {
        // Verify this was actually a critical failure
        if !result.critical_failure {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Skill check was not a critical failure".to_string(),
                ),
            ));
        }

        // Reconstruct resolution evidence
        let roll_made = Established::prove(&ValidSkillCheckRoll);
        let outcome = Established::prove(&SkillCheckOutcomeChecked);
        let resolution = SkillCheckResolutionEvidence { roll_made, outcome };

        // Mint critical failure proof
        let critical = Established::prove(&SkillCheckCriticalMiss);

        // Compose critical failure evidence
        let _evidence = SkillCheckCriticalFailureEvidence {
            resolution,
            critical,
        };

        Ok(Established::assert())
    }
}
