//! Spell effect resolution implementation.

use crate::contracts::credentials::{
    DurationSet, EffectApplied, MaintenancePaid, RangeChecked, ResistanceBroken, ResistanceNeeded,
    ResistanceRolled, SpellKeptActive, SpellResisted, TargetDetermined,
};
use crate::contracts::proof_composition::{
    ResistanceOvercomeEvidence, SpellCastingSuccessEvidence, SpellEffectEvidence,
    SpellMaintenanceEvidence, SpellResistanceEvidence,
};
use crate::contracts::traits::{
    CombatResult, ContractError, ContractErrorKind, SpellEffectResolver,
};
use crate::contracts::types::{
    CasterDescriptor, ResistanceResult, SkillCheckResult, SpellDescriptor, SpellEffectDescriptor,
    SpellResistanceDescriptor,
};
use crate::game_master::GameMaster;
use async_trait::async_trait;
use elicitation::contracts::Established;

#[async_trait]
impl SpellEffectResolver for GameMaster {
    async fn apply_spell_effect(
        &self,
        _spell: &SpellDescriptor,
        _descriptor: SpellEffectDescriptor,
        _casting_evidence: Established<SpellCastingSuccessEvidence>,
    ) -> CombatResult<Established<SpellEffectEvidence>> {
        let target = Established::prove(&TargetDetermined);
        let range = Established::prove(&RangeChecked);
        let duration = Established::prove(&DurationSet);
        let effect = Established::prove(&EffectApplied);
        let evidence = SpellEffectEvidence { target, range, duration, effect };

        Ok(Established::prove(&evidence))
    }

    async fn resolve_resistance(
        &self,
        descriptor: SpellResistanceDescriptor,
    ) -> CombatResult<(ResistanceResult, Established<SpellResistanceEvidence>)> {
        let caster = SkillCheckResult::new(self.roll_3d6().sum(), descriptor.caster_skill);
        let target = SkillCheckResult::new(self.roll_3d6().sum(), descriptor.target_resistance);

        let resisted = match (caster.success, target.success) {
            (true, true) => target.margin > caster.margin,
            (true, false) => false,
            (false, true) => true,
            (false, false) => target.roll < caster.roll,
        };

        let margin = if resisted {
            if target.success { target.margin - caster.margin } else { caster.roll - target.roll }
        } else {
            if caster.success { caster.margin - target.margin } else { target.roll - caster.roll }
        };

        let result = ResistanceResult { caster_roll: caster.roll, target_roll: target.roll, resisted, margin };

        let required = Established::prove(&ResistanceNeeded);
        let roll_made = Established::prove(&ResistanceRolled);
        let resisted_proof = Established::prove(&SpellResisted);
        let evidence = SpellResistanceEvidence { required, roll_made, resisted: resisted_proof };

        Ok((result, Established::prove(&evidence)))
    }

    async fn confirm_resistance_overcome(
        &self,
        result: ResistanceResult,
        _base_evidence: Established<SpellResistanceEvidence>,
    ) -> CombatResult<Established<ResistanceOvercomeEvidence>> {
        if result.resisted {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                "Cannot confirm resistance overcome when target resisted".to_string(),
            )));
        }

        let required = Established::prove(&ResistanceNeeded);
        let roll_made = Established::prove(&ResistanceRolled);
        let overcome = Established::prove(&ResistanceBroken);
        let evidence = ResistanceOvercomeEvidence { required, roll_made, overcome };

        Ok(Established::prove(&evidence))
    }

    async fn maintain_spell(
        &self,
        mut caster: CasterDescriptor,
        spell: &SpellDescriptor,
    ) -> CombatResult<(CasterDescriptor, Established<SpellMaintenanceEvidence>)> {
        let maintenance_cost = spell.base_maintenance_cost;

        if caster.current_fp < maintenance_cost {
            if caster.current_fp + caster.current_hp < maintenance_cost {
                return Err(ContractError::new(ContractErrorKind::StateViolation(
                    "Insufficient energy for spell maintenance".to_string(),
                )));
            }
        }

        if caster.current_fp >= maintenance_cost {
            caster.current_fp -= maintenance_cost;
        } else {
            let remaining = maintenance_cost - caster.current_fp;
            caster.current_fp = 0;
            caster.current_hp -= remaining;
            if caster.current_hp <= 0 {
                return Err(ContractError::new(ContractErrorKind::StateViolation(
                    "Caster died from spell maintenance".to_string(),
                )));
            }
        }

        let maintained = Established::prove(&SpellKeptActive);
        let energy_paid = Established::prove(&MaintenancePaid);
        let evidence = SpellMaintenanceEvidence { maintained, energy_paid };

        Ok((caster, Established::prove(&evidence)))
    }
}
