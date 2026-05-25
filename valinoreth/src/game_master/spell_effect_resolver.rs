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
    CasterDescriptor, ResistanceResult, SpellDescriptor, SpellEffectDescriptor,
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
        // Mint proof tokens for spell effect
        let target = Established::prove(&TargetDetermined);
        let range = Established::prove(&RangeChecked);
        let duration = Established::prove(&DurationSet);
        let effect = Established::prove(&EffectApplied);

        let _evidence = SpellEffectEvidence {
            target,
            range,
            duration,
            effect,
        };

        Ok(Established::assert())
    }

    async fn resolve_resistance(
        &self,
        descriptor: SpellResistanceDescriptor,
    ) -> CombatResult<(ResistanceResult, Established<SpellResistanceEvidence>)> {
        // Roll 3d6 for both caster and target
        let caster_roll = self.roll_3d6().sum();
        let target_roll = self.roll_3d6().sum();

        // Calculate success for both sides
        let (caster_success, caster_margin) =
            Self::calculate_margin(caster_roll, descriptor.caster_skill);
        let (target_success, target_margin) =
            Self::calculate_margin(target_roll, descriptor.target_resistance);

        // Determine winner based on GURPS Quick Contest rules:
        // - If both succeed, higher margin wins
        // - If one succeeds and one fails, succeeder wins
        // - If both fail, lower roll wins (closer to success)
        let resisted = match (caster_success, target_success) {
            (true, true) => target_margin > caster_margin, // Both succeed: higher margin wins
            (true, false) => false,                        // Caster succeeds, target fails
            (false, true) => true,                         // Target succeeds, caster fails
            (false, false) => target_roll < caster_roll,   // Both fail: lower roll wins
        };

        // Calculate margin of victory
        let margin = if resisted {
            if target_success {
                target_margin - caster_margin
            } else {
                caster_roll - target_roll
            }
        } else {
            if caster_success {
                caster_margin - target_margin
            } else {
                target_roll - caster_roll
            }
        };

        let result = ResistanceResult {
            caster_roll,
            target_roll,
            resisted,
            margin,
        };

        // Mint proof tokens
        let required = Established::prove(&ResistanceNeeded);
        let roll_made = Established::prove(&ResistanceRolled);
        let resisted_proof = Established::prove(&SpellResisted);

        let _evidence = SpellResistanceEvidence {
            required,
            roll_made,
            resisted: resisted_proof,
        };

        Ok((result, Established::assert()))
    }

    async fn confirm_resistance_overcome(
        &self,
        result: ResistanceResult,
        _base_evidence: Established<SpellResistanceEvidence>,
    ) -> CombatResult<Established<ResistanceOvercomeEvidence>> {
        // Verify resistance was actually overcome
        if result.resisted {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                "Cannot confirm resistance overcome when target resisted".to_string(),
            )));
        }

        // Reconstruct base evidence
        let required = Established::prove(&ResistanceNeeded);
        let roll_made = Established::prove(&ResistanceRolled);

        // Mint proof that resistance was overcome
        let overcome = Established::prove(&ResistanceBroken);

        let _evidence = ResistanceOvercomeEvidence {
            required,
            roll_made,
            overcome,
        };

        Ok(Established::assert())
    }

    async fn maintain_spell(
        &self,
        mut caster: CasterDescriptor,
        spell: &SpellDescriptor,
    ) -> CombatResult<(CasterDescriptor, Established<SpellMaintenanceEvidence>)> {
        // Calculate maintenance cost (typically 1 per second, may vary)
        // For now, use spell's base maintenance cost
        let maintenance_cost = spell.base_maintenance_cost;

        // Verify caster can afford maintenance
        if caster.current_fp < maintenance_cost {
            if caster.current_fp + caster.current_hp < maintenance_cost {
                return Err(ContractError::new(ContractErrorKind::StateViolation(
                    "Insufficient energy for spell maintenance".to_string(),
                )));
            }
        }

        // Deduct energy
        if caster.current_fp >= maintenance_cost {
            caster.current_fp -= maintenance_cost;
        } else {
            // Spend remaining FP, then HP
            let remaining = maintenance_cost - caster.current_fp;
            caster.current_fp = 0;
            caster.current_hp -= remaining;

            // Check if caster is still alive
            if caster.current_hp <= 0 {
                return Err(ContractError::new(ContractErrorKind::StateViolation(
                    "Caster died from spell maintenance".to_string(),
                )));
            }
        }

        // Mint proof tokens
        let maintained = Established::prove(&SpellKeptActive);
        let energy_paid = Established::prove(&MaintenancePaid);

        let _evidence = SpellMaintenanceEvidence {
            maintained,
            energy_paid,
        };

        Ok((caster, Established::assert()))
    }
}
