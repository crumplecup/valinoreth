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
        let mut effective_skill = descriptor.effective_skill;
        effective_skill += descriptor.situational_modifier;
        effective_skill += descriptor.task_difficulty;
        effective_skill += descriptor.time_modifier;
        if let Some(bonus) = descriptor.complementary_bonus {
            effective_skill += bonus;
        }
        let result = SkillCheckResult::new(self.roll_3d6().sum(), effective_skill);

        let roll_made = Established::prove(&ValidSkillCheckRoll);
        let outcome = Established::prove(&SkillCheckOutcomeChecked);
        let evidence = SkillCheckResolutionEvidence { roll_made, outcome };

        Ok((result, Established::prove(&evidence)))
    }

    async fn confirm_skill_check_success(
        &self,
        result: SkillCheckResult,
        _base_evidence: Established<SkillCheckResolutionEvidence>,
    ) -> CombatResult<Established<SkillCheckSuccessEvidence>> {
        if !result.success {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Cannot confirm success for failed skill check".to_string(),
                ),
            ));
        }

        let roll_made = Established::prove(&ValidSkillCheckRoll);
        let outcome = Established::prove(&SkillCheckOutcomeChecked);
        let resolution = SkillCheckResolutionEvidence { roll_made, outcome };
        let success = Established::prove(&SkillCheckHit);
        let evidence = SkillCheckSuccessEvidence {
            resolution,
            success,
        };

        Ok(Established::prove(&evidence))
    }

    async fn confirm_skill_check_failure(
        &self,
        result: SkillCheckResult,
        _base_evidence: Established<SkillCheckResolutionEvidence>,
    ) -> CombatResult<Established<SkillCheckFailureEvidence>> {
        if result.success {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Cannot confirm failure for successful skill check".to_string(),
                ),
            ));
        }

        let roll_made = Established::prove(&ValidSkillCheckRoll);
        let outcome = Established::prove(&SkillCheckOutcomeChecked);
        let resolution = SkillCheckResolutionEvidence { roll_made, outcome };
        let failure = Established::prove(&SkillCheckMiss);
        let evidence = SkillCheckFailureEvidence {
            resolution,
            failure,
        };

        Ok(Established::prove(&evidence))
    }

    async fn confirm_critical_skill_success(
        &self,
        result: SkillCheckResult,
        _success_evidence: Established<SkillCheckSuccessEvidence>,
    ) -> CombatResult<Established<SkillCheckCriticalSuccessEvidence>> {
        if !result.critical_success {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Skill check was not a critical success".to_string(),
                ),
            ));
        }

        let roll_made = Established::prove(&ValidSkillCheckRoll);
        let outcome = Established::prove(&SkillCheckOutcomeChecked);
        let resolution = SkillCheckResolutionEvidence { roll_made, outcome };
        let success_proof = Established::prove(&SkillCheckHit);
        let success = SkillCheckSuccessEvidence {
            resolution,
            success: success_proof,
        };
        let critical = Established::prove(&SkillCheckCriticalHit);
        let evidence = SkillCheckCriticalSuccessEvidence { success, critical };

        Ok(Established::prove(&evidence))
    }

    async fn confirm_critical_skill_failure(
        &self,
        result: SkillCheckResult,
        _base_evidence: Established<SkillCheckResolutionEvidence>,
    ) -> CombatResult<Established<SkillCheckCriticalFailureEvidence>> {
        if !result.critical_failure {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Skill check was not a critical failure".to_string(),
                ),
            ));
        }

        let roll_made = Established::prove(&ValidSkillCheckRoll);
        let outcome = Established::prove(&SkillCheckOutcomeChecked);
        let resolution = SkillCheckResolutionEvidence { roll_made, outcome };
        let critical = Established::prove(&SkillCheckCriticalMiss);
        let evidence = SkillCheckCriticalFailureEvidence {
            resolution,
            critical,
        };

        Ok(Established::prove(&evidence))
    }
}
