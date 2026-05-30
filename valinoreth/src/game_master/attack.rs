//! Attack resolution implementation.

use crate::contracts::credentials::{
    AttackCriticalHit, AttackCriticalMiss, AttackHit, AttackMiss, AttackOutcomeChecked,
    ValidAttackRoll,
};
use crate::contracts::proof_composition::{
    AttackCriticalFailureEvidence, AttackCriticalSuccessEvidence, AttackFailureEvidence,
    AttackResolutionEvidence, AttackSuccessEvidence,
};
use crate::contracts::traits::{AttackResolver, CombatResult};
use crate::contracts::types::{AttackDescriptor, AttackRollResult};
use crate::game_master::GameMaster;
use async_trait::async_trait;
use elicitation::contracts::Established;

#[async_trait]
impl AttackResolver for GameMaster {
    async fn resolve_attack(
        &self,
        descriptor: AttackDescriptor,
    ) -> CombatResult<(AttackRollResult, Established<AttackResolutionEvidence>)> {
        let effective_skill =
            descriptor.effective_skill + descriptor.aim_bonus - descriptor.deceptive_penalty;
        let result = AttackRollResult::new(self.roll_3d6().sum(), effective_skill);

        let roll_made = Established::prove(&ValidAttackRoll);
        let outcome = Established::prove(&AttackOutcomeChecked);
        let evidence = AttackResolutionEvidence { roll_made, outcome };

        Ok((result, Established::prove(&evidence)))
    }

    async fn confirm_attack_success(
        &self,
        result: AttackRollResult,
        _base_evidence: Established<AttackResolutionEvidence>,
    ) -> CombatResult<Established<AttackSuccessEvidence>> {
        if !result.success {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Cannot confirm success for failed attack".to_string(),
                ),
            ));
        }

        let roll_made = Established::prove(&ValidAttackRoll);
        let outcome = Established::prove(&AttackOutcomeChecked);
        let resolution = AttackResolutionEvidence { roll_made, outcome };
        let success = Established::prove(&AttackHit);
        let evidence = AttackSuccessEvidence {
            resolution,
            success,
        };

        Ok(Established::prove(&evidence))
    }

    async fn confirm_attack_failure(
        &self,
        result: AttackRollResult,
        _base_evidence: Established<AttackResolutionEvidence>,
    ) -> CombatResult<Established<AttackFailureEvidence>> {
        if result.success {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Cannot confirm failure for successful attack".to_string(),
                ),
            ));
        }

        let roll_made = Established::prove(&ValidAttackRoll);
        let outcome = Established::prove(&AttackOutcomeChecked);
        let resolution = AttackResolutionEvidence { roll_made, outcome };
        let failure = Established::prove(&AttackMiss);
        let evidence = AttackFailureEvidence {
            resolution,
            failure,
        };

        Ok(Established::prove(&evidence))
    }

    async fn confirm_critical_success(
        &self,
        result: AttackRollResult,
        _success_evidence: Established<AttackSuccessEvidence>,
    ) -> CombatResult<Established<AttackCriticalSuccessEvidence>> {
        if !result.critical_success {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Attack was not a critical success".to_string(),
                ),
            ));
        }

        let roll_made = Established::prove(&ValidAttackRoll);
        let outcome = Established::prove(&AttackOutcomeChecked);
        let resolution = AttackResolutionEvidence { roll_made, outcome };
        let success_proof = Established::prove(&AttackHit);
        let success = AttackSuccessEvidence {
            resolution,
            success: success_proof,
        };
        let critical = Established::prove(&AttackCriticalHit);
        let evidence = AttackCriticalSuccessEvidence { success, critical };

        Ok(Established::prove(&evidence))
    }

    async fn confirm_critical_failure(
        &self,
        result: AttackRollResult,
        _base_evidence: Established<AttackResolutionEvidence>,
    ) -> CombatResult<Established<AttackCriticalFailureEvidence>> {
        if !result.critical_failure {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Attack was not a critical failure".to_string(),
                ),
            ));
        }

        let roll_made = Established::prove(&ValidAttackRoll);
        let outcome = Established::prove(&AttackOutcomeChecked);
        let resolution = AttackResolutionEvidence { roll_made, outcome };
        let critical = Established::prove(&AttackCriticalMiss);
        let evidence = AttackCriticalFailureEvidence {
            resolution,
            critical,
        };

        Ok(Established::prove(&evidence))
    }
}
