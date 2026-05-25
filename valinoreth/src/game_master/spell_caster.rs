//! Spell casting resolution implementation.

use crate::contracts::credentials::{
    BaseCostDetermined, CastingFailed, CastingOutcomeChecked, CastingSucceeded,
    ConcentrationFinished, ConcentrationStarted, EnergyDeducted, FinalCostCalculated,
    SkillReductionApplied, SpellCritHit, SpellCritMiss, SpellRollMade,
};
use crate::contracts::proof_composition::{
    ConcentrationEvidence, EnergyCostEvidence, EnergyPaymentEvidence, SpellCastingFailureEvidence,
    SpellCastingResolutionEvidence, SpellCastingSuccessEvidence, SpellCriticalFailureEvidence,
    SpellCriticalSuccessEvidence,
};
use crate::contracts::traits::{CombatResult, ContractError, ContractErrorKind, SpellCaster};
use crate::contracts::types::{
    CasterDescriptor, SpellCastingDescriptor, SpellCastingResult, SpellDescriptor,
};
use crate::game_master::GameMaster;
use async_trait::async_trait;
use elicitation::contracts::Established;

#[async_trait]
impl SpellCaster for GameMaster {
    async fn cast_spell(
        &self,
        caster: CasterDescriptor,
        descriptor: SpellCastingDescriptor,
    ) -> CombatResult<(
        SpellCastingResult,
        CasterDescriptor,
        Established<SpellCastingResolutionEvidence>,
    )> {
        // Roll 3d6 for spell casting
        let dice_roll = self.roll_3d6();
        let roll = dice_roll.sum();

        // Calculate effective skill with all modifiers
        let mut effective_skill = descriptor.effective_skill;
        effective_skill += descriptor.size_speed_modifier;
        effective_skill += descriptor.time_modifier;
        effective_skill += descriptor.environment_modifier;

        // Determine outcome
        let (success, margin) = Self::calculate_margin(roll, effective_skill);

        // Check for critical outcomes
        let critical_success = Self::is_critical_success(roll, effective_skill);
        let critical_failure = Self::is_critical_failure(roll, effective_skill, success, margin);

        // Calculate energy spent
        let energy_spent = if success {
            descriptor.energy_cost
        } else {
            // On failure, spend 1 energy
            1
        };

        // Build result
        let result = SpellCastingResult {
            roll,
            effective_skill,
            success,
            margin,
            critical_success,
            critical_failure,
            energy_spent,
        };

        // Mint proof tokens for concentration
        let begun = Established::prove(&ConcentrationStarted);
        let completed = Established::prove(&ConcentrationFinished);
        let concentration = ConcentrationEvidence {
            begun,
            maintained: None,
            completed,
        };

        // Mint proof tokens for casting
        let roll_made = Established::prove(&SpellRollMade);
        let outcome = Established::prove(&CastingOutcomeChecked);

        // Compose evidence bundle
        let _evidence = SpellCastingResolutionEvidence {
            concentration,
            roll_made,
            outcome,
        };

        Ok((result, caster, Established::assert()))
    }

    async fn confirm_casting_success(
        &self,
        result: SpellCastingResult,
        _base_evidence: Established<SpellCastingResolutionEvidence>,
    ) -> CombatResult<Established<SpellCastingSuccessEvidence>> {
        // Verify the spell casting actually succeeded
        if !result.success {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                "Cannot confirm success for failed spell casting".to_string(),
            )));
        }

        // Reconstruct resolution evidence
        let begun = Established::prove(&ConcentrationStarted);
        let completed = Established::prove(&ConcentrationFinished);
        let concentration = ConcentrationEvidence {
            begun,
            maintained: None,
            completed,
        };
        let roll_made = Established::prove(&SpellRollMade);
        let outcome = Established::prove(&CastingOutcomeChecked);
        let resolution = SpellCastingResolutionEvidence {
            concentration,
            roll_made,
            outcome,
        };

        // Mint success proof
        let success = Established::prove(&CastingSucceeded);

        // Compose success evidence
        let _evidence = SpellCastingSuccessEvidence {
            resolution,
            success,
        };

        Ok(Established::assert())
    }

    async fn confirm_casting_failure(
        &self,
        result: SpellCastingResult,
        _base_evidence: Established<SpellCastingResolutionEvidence>,
    ) -> CombatResult<Established<SpellCastingFailureEvidence>> {
        // Verify the spell casting actually failed
        if result.success {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                "Cannot confirm failure for successful spell casting".to_string(),
            )));
        }

        // Reconstruct resolution evidence
        let begun = Established::prove(&ConcentrationStarted);
        let completed = Established::prove(&ConcentrationFinished);
        let concentration = ConcentrationEvidence {
            begun,
            maintained: None,
            completed,
        };
        let roll_made = Established::prove(&SpellRollMade);
        let outcome = Established::prove(&CastingOutcomeChecked);
        let resolution = SpellCastingResolutionEvidence {
            concentration,
            roll_made,
            outcome,
        };

        // Mint failure proof
        let failure = Established::prove(&CastingFailed);

        // Compose failure evidence
        let _evidence = SpellCastingFailureEvidence {
            resolution,
            failure,
        };

        Ok(Established::assert())
    }

    async fn confirm_critical_casting_success(
        &self,
        result: SpellCastingResult,
        _success_evidence: Established<SpellCastingSuccessEvidence>,
    ) -> CombatResult<Established<SpellCriticalSuccessEvidence>> {
        // Verify this was actually a critical success
        if !result.critical_success {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                "Spell casting was not a critical success".to_string(),
            )));
        }

        // Reconstruct resolution evidence
        let begun = Established::prove(&ConcentrationStarted);
        let completed = Established::prove(&ConcentrationFinished);
        let concentration = ConcentrationEvidence {
            begun,
            maintained: None,
            completed,
        };
        let roll_made = Established::prove(&SpellRollMade);
        let outcome = Established::prove(&CastingOutcomeChecked);
        let resolution = SpellCastingResolutionEvidence {
            concentration,
            roll_made,
            outcome,
        };

        // Reconstruct success evidence
        let success_proof = Established::prove(&CastingSucceeded);
        let success = SpellCastingSuccessEvidence {
            resolution,
            success: success_proof,
        };

        // Mint critical success proof
        let critical = Established::prove(&SpellCritHit);

        // Compose critical success evidence
        let _evidence = SpellCriticalSuccessEvidence { success, critical };

        Ok(Established::assert())
    }

    async fn confirm_critical_casting_failure(
        &self,
        result: SpellCastingResult,
        _base_evidence: Established<SpellCastingResolutionEvidence>,
    ) -> CombatResult<Established<SpellCriticalFailureEvidence>> {
        // Verify this was actually a critical failure
        if !result.critical_failure {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                "Spell casting was not a critical failure".to_string(),
            )));
        }

        // Reconstruct resolution evidence
        let begun = Established::prove(&ConcentrationStarted);
        let completed = Established::prove(&ConcentrationFinished);
        let concentration = ConcentrationEvidence {
            begun,
            maintained: None,
            completed,
        };
        let roll_made = Established::prove(&SpellRollMade);
        let outcome = Established::prove(&CastingOutcomeChecked);
        let resolution = SpellCastingResolutionEvidence {
            concentration,
            roll_made,
            outcome,
        };

        // Mint critical failure proof
        let critical = Established::prove(&SpellCritMiss);

        // Compose critical failure evidence
        let _evidence = SpellCriticalFailureEvidence {
            resolution,
            critical,
        };

        Ok(Established::assert())
    }

    async fn calculate_energy_cost(
        &self,
        spell: &SpellDescriptor,
        effective_skill: i32,
        extra_energy: i32,
    ) -> CombatResult<(i32, Established<EnergyCostEvidence>)> {
        // Start with base cost
        let base_cost = spell.base_casting_cost;

        // Calculate skill-based cost reduction
        // Skill 15: -1, skill 20: -2, skill 25: -3, etc.
        let reduction = if effective_skill >= 15 {
            (effective_skill - 10) / 5
        } else {
            0
        };

        // Calculate final cost (minimum 1)
        let final_cost = (base_cost - reduction + extra_energy).max(1);

        // Mint proof tokens
        let base = Established::prove(&BaseCostDetermined);
        let reduction_proof = Established::prove(&SkillReductionApplied);
        let final_calc = Established::prove(&FinalCostCalculated);

        let _evidence = EnergyCostEvidence {
            base_cost: base,
            skill_reduction: reduction_proof,
            final_cost: final_calc,
        };

        Ok((final_cost, Established::assert()))
    }

    async fn pay_energy(
        &self,
        mut caster: CasterDescriptor,
        cost: i32,
        _cost_evidence: Established<EnergyCostEvidence>,
    ) -> CombatResult<(CasterDescriptor, Established<EnergyPaymentEvidence>)> {
        // Verify cost is valid
        if cost < 0 {
            return Err(ContractError::new(ContractErrorKind::InvalidDamage(
                "Energy cost cannot be negative".to_string(),
            )));
        }

        // Spend FP first
        if caster.current_fp >= cost {
            caster.current_fp -= cost;
        } else {
            // Spend remaining FP, then HP
            let remaining = cost - caster.current_fp;
            caster.current_fp = 0;
            caster.current_hp -= remaining;

            // Check if caster is still alive
            if caster.current_hp <= 0 {
                return Err(ContractError::new(ContractErrorKind::StateViolation(
                    "Caster died from energy expenditure".to_string(),
                )));
            }
        }

        // Reconstruct cost evidence
        let base = Established::prove(&BaseCostDetermined);
        let reduction = Established::prove(&SkillReductionApplied);
        let final_cost = Established::prove(&FinalCostCalculated);

        let cost_evidence = EnergyCostEvidence {
            base_cost: base,
            skill_reduction: reduction,
            final_cost,
        };

        // Mint payment proof
        let paid = Established::prove(&EnergyDeducted);

        let _evidence = EnergyPaymentEvidence {
            cost: cost_evidence,
            paid,
        };

        Ok((caster, Established::assert()))
    }
}
