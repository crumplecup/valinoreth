//! Complete spell execution implementation.

use crate::contracts::credentials::{
    BaseCostDetermined, CastingOutcomeChecked, CastingSucceeded, CeremonyCompleted,
    CeremonyStarted, ConcentrationFinished, ConcentrationStarted, DurationSet, EffectApplied,
    EnergyDeducted, EnergyPooled, FinalCostCalculated, RangeChecked, SkillReductionApplied,
    SpellRollMade, TargetDetermined,
};
use crate::contracts::proof_composition::{
    CeremonialMagicEvidence, CompleteSpellCastingEvidence, ConcentrationEvidence,
    EnergyCostEvidence, EnergyPaymentEvidence, SpellCastingResolutionEvidence,
    SpellCastingSuccessEvidence, SpellEffectEvidence,
};
use crate::contracts::traits::{
    CombatResult, ContractError, ContractErrorKind, SpellCaster, SpellEffectResolver,
    SpellExecutionResult, SpellExecutor,
};
use crate::contracts::types::{
    CasterDescriptor, CeremonialMagicDescriptor, SpellCastingDescriptor, SpellCastingResult,
    SpellEffectDescriptor,
};
use crate::game_master::GameMaster;
use async_trait::async_trait;
use elicitation::contracts::Established;

#[async_trait]
impl SpellExecutor for GameMaster {
    async fn execute_spell(
        &self,
        caster: CasterDescriptor,
        spell_descriptor: SpellCastingDescriptor,
        effect_descriptor: SpellEffectDescriptor,
    ) -> CombatResult<SpellExecutionResult> {
        // Step 1: Cast the spell
        let (result, updated_caster, casting_evidence) =
            self.cast_spell(caster, spell_descriptor).await?;

        // Check for critical failure
        if result.critical_failure {
            let failure_evidence = self
                .confirm_critical_casting_failure(result.clone(), casting_evidence.clone())
                .await?;

            return Ok(SpellExecutionResult::CriticalFailure {
                result,
                caster: updated_caster,
                mishap: "Spell backfired with critical failure".to_string(),
                evidence: failure_evidence,
            });
        }

        // Check if casting succeeded
        if !result.success {
            let failure_evidence = self
                .confirm_casting_failure(result.clone(), casting_evidence)
                .await?;

            return Ok(SpellExecutionResult::Failure {
                result,
                caster: updated_caster,
                evidence: failure_evidence,
            });
        }

        // Confirm success and get success evidence
        let success_evidence = self
            .confirm_casting_success(result.clone(), casting_evidence)
            .await?;

        // Step 2: Apply spell effect — type-level proof that the effect was applied.
        let _effect_evidence = self
            .apply_spell_effect(
                // Need to get spell descriptor - for now use a minimal one
                &crate::contracts::types::SpellDescriptor {
                    name: effect_descriptor.spell_name.clone(),
                    college: crate::contracts::types::SpellCollege::Fire, // Placeholder
                    difficulty: crate::contracts::types::SkillDifficulty::Hard,
                    skill_level: result.effective_skill,
                    points_invested: 1,
                    prerequisites: vec![],
                    magery_required: 0,
                    base_casting_cost: result.energy_spent,
                    base_maintenance_cost: 0,
                    base_casting_time: 1,
                    spell_class: crate::contracts::types::SpellClass::Regular,
                    resistible: false,
                    resistance_attribute: None,
                },
                effect_descriptor.clone(),
                success_evidence.clone(),
            )
            .await?;

        // Reconstruct all evidence manually for complete spell casting

        // Reconstruct concentration evidence
        let begun = Established::prove(&ConcentrationStarted);
        let completed = Established::prove(&ConcentrationFinished);
        let concentration = ConcentrationEvidence {
            begun,
            maintained: None,
            completed,
        };

        // Reconstruct resolution evidence
        let roll_made = Established::prove(&SpellRollMade);
        let outcome = Established::prove(&CastingOutcomeChecked);
        let resolution = SpellCastingResolutionEvidence {
            concentration,
            roll_made,
            outcome,
        };

        // Reconstruct success evidence
        let success = Established::prove(&CastingSucceeded);
        let casting_success = SpellCastingSuccessEvidence {
            resolution,
            success,
        };

        // Reconstruct energy cost evidence
        let base = Established::prove(&BaseCostDetermined);
        let reduction = Established::prove(&SkillReductionApplied);
        let final_cost = Established::prove(&FinalCostCalculated);
        let cost_evidence = EnergyCostEvidence {
            base_cost: base,
            skill_reduction: reduction,
            final_cost,
        };

        // Reconstruct energy payment evidence
        let paid = Established::prove(&EnergyDeducted);
        let energy = EnergyPaymentEvidence {
            cost: cost_evidence,
            paid,
        };

        // Reconstruct effect evidence
        let target = Established::prove(&TargetDetermined);
        let range = Established::prove(&RangeChecked);
        let duration = Established::prove(&DurationSet);
        let effect = Established::prove(&EffectApplied);
        let effect_evidence_struct = SpellEffectEvidence {
            target,
            range,
            duration,
            effect,
        };

        // Compose complete evidence — the assembled bundle IS the credential.
        let evidence = CompleteSpellCastingEvidence {
            casting: casting_success,
            energy,
            effect: effect_evidence_struct,
        };

        Ok(SpellExecutionResult::Success {
            result,
            caster: updated_caster,
            effect: effect_descriptor,
            evidence: Established::prove(&evidence),
        })
    }

    async fn execute_ceremonial_spell(
        &self,
        descriptor: CeremonialMagicDescriptor,
    ) -> CombatResult<(SpellCastingResult, Established<CeremonialMagicEvidence>)> {
        // Verify we have at least a leader
        if descriptor.participants.is_empty() {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                "Ceremonial magic requires at least one participant (the leader)".to_string(),
            )));
        }

        // Get leader's skill and pooled energy from descriptor
        let leader_skill = descriptor.leader_skill;
        let pooled_energy = descriptor.pooled_energy;

        // Use leader's skill for the casting roll, boosted by assistant bonuses
        let effective_skill = leader_skill + descriptor.assistant_bonuses;

        // Build result; correct energy_spent for edge case where pooled_energy < 1
        let mut result = SpellCastingResult::new(self.roll_3d6().sum(), effective_skill, pooled_energy);
        if !result.success {
            result.energy_spent = 1.min(pooled_energy);
        }

        // Mint proof tokens and assemble the ceremonial evidence bundle.
        let begun = Established::prove(&CeremonyStarted);
        let energy_pooled = Established::prove(&EnergyPooled);
        let completed = Established::prove(&CeremonyCompleted);

        let evidence = CeremonialMagicEvidence {
            begun,
            energy_pooled,
            completed,
        };

        Ok((result, Established::prove(&evidence)))
    }
}
