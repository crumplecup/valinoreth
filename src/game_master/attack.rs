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
        // Roll 3d6 for attack
        let dice_roll = self.roll_3d6();
        let roll = dice_roll.sum();

        // Calculate effective skill with modifiers
        let effective_skill = descriptor.effective_skill
            + descriptor.aim_bonus
            - descriptor.deceptive_penalty;

        // Determine outcome
        let (success, margin) = Self::calculate_margin(roll, effective_skill);

        // Check for critical outcomes
        let critical_success = Self::is_critical_success(roll, effective_skill);
        let critical_failure = Self::is_critical_failure(roll, effective_skill, success, margin);

        // Build result
        let result = AttackRollResult {
            roll,
            effective_skill,
            success,
            margin,
            critical_success,
            critical_failure,
        };

        // Mint proof tokens using credentials
        let roll_made = Established::prove(&ValidAttackRoll);
        let outcome = Established::prove(&AttackOutcomeChecked);

        // Compose evidence bundle
        let _evidence = AttackResolutionEvidence {
            roll_made,
            outcome,
        };

        Ok((result, Established::assert()))
    }

    async fn confirm_attack_success(
        &self,
        result: AttackRollResult,
        _base_evidence: Established<AttackResolutionEvidence>,
    ) -> CombatResult<Established<AttackSuccessEvidence>> {
        // Verify the attack actually succeeded
        if !result.success {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Cannot confirm success for failed attack".to_string(),
                ),
            ));
        }

        // Reconstruct resolution evidence (zero-sized, exists only for type system)
        let roll_made = Established::prove(&ValidAttackRoll);
        let outcome = Established::prove(&AttackOutcomeChecked);
        let resolution = AttackResolutionEvidence {
            roll_made,
            outcome,
        };

        // Mint success proof
        let success = Established::prove(&AttackHit);

        // Compose success evidence
        let _evidence = AttackSuccessEvidence {
            resolution,
            success,
        };

        Ok(Established::assert())
    }

    async fn confirm_attack_failure(
        &self,
        result: AttackRollResult,
        _base_evidence: Established<AttackResolutionEvidence>,
    ) -> CombatResult<Established<AttackFailureEvidence>> {
        // Verify the attack actually failed
        if result.success {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Cannot confirm failure for successful attack".to_string(),
                ),
            ));
        }

        // Reconstruct resolution evidence
        let roll_made = Established::prove(&ValidAttackRoll);
        let outcome = Established::prove(&AttackOutcomeChecked);
        let resolution = AttackResolutionEvidence {
            roll_made,
            outcome,
        };

        // Mint failure proof
        let failure = Established::prove(&AttackMiss);

        // Compose failure evidence
        let _evidence = AttackFailureEvidence {
            resolution,
            failure,
        };

        Ok(Established::assert())
    }

    async fn confirm_critical_success(
        &self,
        result: AttackRollResult,
        _success_evidence: Established<AttackSuccessEvidence>,
    ) -> CombatResult<Established<AttackCriticalSuccessEvidence>> {
        // Verify this was actually a critical success
        if !result.critical_success {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Attack was not a critical success".to_string(),
                ),
            ));
        }

        // Reconstruct resolution evidence
        let roll_made = Established::prove(&ValidAttackRoll);
        let outcome = Established::prove(&AttackOutcomeChecked);
        let resolution = AttackResolutionEvidence {
            roll_made,
            outcome,
        };

        // Reconstruct success evidence
        let success_proof = Established::prove(&AttackHit);
        let success = AttackSuccessEvidence {
            resolution,
            success: success_proof,
        };

        // Mint critical success proof
        let critical = Established::prove(&AttackCriticalHit);

        // Compose critical success evidence
        let _evidence = AttackCriticalSuccessEvidence {
            success,
            critical,
        };

        Ok(Established::assert())
    }

    async fn confirm_critical_failure(
        &self,
        result: AttackRollResult,
        _base_evidence: Established<AttackResolutionEvidence>,
    ) -> CombatResult<Established<AttackCriticalFailureEvidence>> {
        // Verify this was actually a critical failure
        if !result.critical_failure {
            return Err(crate::contracts::traits::ContractError::new(
                crate::contracts::traits::ContractErrorKind::StateViolation(
                    "Attack was not a critical failure".to_string(),
                ),
            ));
        }

        // Reconstruct resolution evidence
        let roll_made = Established::prove(&ValidAttackRoll);
        let outcome = Established::prove(&AttackOutcomeChecked);
        let resolution = AttackResolutionEvidence {
            roll_made,
            outcome,
        };

        // Mint critical failure proof
        let critical = Established::prove(&AttackCriticalMiss);

        // Compose critical failure evidence
        let _evidence = AttackCriticalFailureEvidence {
            resolution,
            critical,
        };

        Ok(Established::assert())
    }
}
