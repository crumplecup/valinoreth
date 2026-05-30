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
        let mut effective_skill = descriptor.effective_skill;
        effective_skill += descriptor.size_speed_modifier;
        effective_skill += descriptor.time_modifier;
        effective_skill += descriptor.environment_modifier;
        let result = SpellCastingResult::new(
            self.roll_3d6().sum(),
            effective_skill,
            descriptor.energy_cost,
        );

        let begun = Established::prove(&ConcentrationStarted);
        let completed = Established::prove(&ConcentrationFinished);
        let concentration = ConcentrationEvidence {
            begun,
            maintained: None,
            completed,
        };
        let roll_made = Established::prove(&SpellRollMade);
        let outcome = Established::prove(&CastingOutcomeChecked);
        let evidence = SpellCastingResolutionEvidence {
            concentration,
            roll_made,
            outcome,
        };

        Ok((result, caster, Established::prove(&evidence)))
    }

    async fn confirm_casting_success(
        &self,
        result: SpellCastingResult,
        _base_evidence: Established<SpellCastingResolutionEvidence>,
    ) -> CombatResult<Established<SpellCastingSuccessEvidence>> {
        if !result.success {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                "Cannot confirm success for failed spell casting".to_string(),
            )));
        }

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
        let success = Established::prove(&CastingSucceeded);
        let evidence = SpellCastingSuccessEvidence {
            resolution,
            success,
        };

        Ok(Established::prove(&evidence))
    }

    async fn confirm_casting_failure(
        &self,
        result: SpellCastingResult,
        _base_evidence: Established<SpellCastingResolutionEvidence>,
    ) -> CombatResult<Established<SpellCastingFailureEvidence>> {
        if result.success {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                "Cannot confirm failure for successful spell casting".to_string(),
            )));
        }

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
        let failure = Established::prove(&CastingFailed);
        let evidence = SpellCastingFailureEvidence {
            resolution,
            failure,
        };

        Ok(Established::prove(&evidence))
    }

    async fn confirm_critical_casting_success(
        &self,
        result: SpellCastingResult,
        _success_evidence: Established<SpellCastingSuccessEvidence>,
    ) -> CombatResult<Established<SpellCriticalSuccessEvidence>> {
        if !result.critical_success {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                "Spell casting was not a critical success".to_string(),
            )));
        }

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
        let success_proof = Established::prove(&CastingSucceeded);
        let success = SpellCastingSuccessEvidence {
            resolution,
            success: success_proof,
        };
        let critical = Established::prove(&SpellCritHit);
        let evidence = SpellCriticalSuccessEvidence { success, critical };

        Ok(Established::prove(&evidence))
    }

    async fn confirm_critical_casting_failure(
        &self,
        result: SpellCastingResult,
        _base_evidence: Established<SpellCastingResolutionEvidence>,
    ) -> CombatResult<Established<SpellCriticalFailureEvidence>> {
        if !result.critical_failure {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                "Spell casting was not a critical failure".to_string(),
            )));
        }

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
        let critical = Established::prove(&SpellCritMiss);
        let evidence = SpellCriticalFailureEvidence {
            resolution,
            critical,
        };

        Ok(Established::prove(&evidence))
    }

    async fn calculate_energy_cost(
        &self,
        spell: &SpellDescriptor,
        effective_skill: i32,
        extra_energy: i32,
    ) -> CombatResult<(i32, Established<EnergyCostEvidence>)> {
        let base_cost = spell.base_casting_cost;
        let reduction = if effective_skill >= 15 {
            (effective_skill - 10) / 5
        } else {
            0
        };
        let final_cost = (base_cost - reduction + extra_energy).max(1);

        let base = Established::prove(&BaseCostDetermined);
        let reduction_proof = Established::prove(&SkillReductionApplied);
        let final_calc = Established::prove(&FinalCostCalculated);
        let evidence = EnergyCostEvidence {
            base_cost: base,
            skill_reduction: reduction_proof,
            final_cost: final_calc,
        };

        Ok((final_cost, Established::prove(&evidence)))
    }

    async fn pay_energy(
        &self,
        mut caster: CasterDescriptor,
        cost: i32,
        _cost_evidence: Established<EnergyCostEvidence>,
    ) -> CombatResult<(CasterDescriptor, Established<EnergyPaymentEvidence>)> {
        if cost < 0 {
            return Err(ContractError::new(ContractErrorKind::InvalidDamage(
                "Energy cost cannot be negative".to_string(),
            )));
        }

        if caster.current_fp >= cost {
            caster.current_fp -= cost;
        } else {
            let remaining = cost - caster.current_fp;
            caster.current_fp = 0;
            caster.current_hp -= remaining;
            if caster.current_hp <= 0 {
                return Err(ContractError::new(ContractErrorKind::StateViolation(
                    "Caster died from energy expenditure".to_string(),
                )));
            }
        }

        let base = Established::prove(&BaseCostDetermined);
        let reduction = Established::prove(&SkillReductionApplied);
        let final_cost = Established::prove(&FinalCostCalculated);
        let cost_evidence = EnergyCostEvidence {
            base_cost: base,
            skill_reduction: reduction,
            final_cost,
        };
        let paid = Established::prove(&EnergyDeducted);
        let evidence = EnergyPaymentEvidence {
            cost: cost_evidence,
            paid,
        };

        Ok((caster, Established::prove(&evidence)))
    }
}
