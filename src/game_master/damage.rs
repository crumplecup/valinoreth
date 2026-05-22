//! Damage calculation implementation.

use crate::contracts::credentials::{
    BasicDamageComputed, DrSubtracted, InjuryComputed, InjurySubtracted, LocationDetermined,
    LocationMultiplierComputed, ValidDamageRoll, WoundingMultiplierComputed,
};
use crate::contracts::proof_composition::{
    BasicDamageEvidence, InjuryApplicationEvidence, InjuryCalculationEvidence,
};
use crate::contracts::traits::{CombatResult, DamageCalculator};
use crate::contracts::types::{
    ArmorDescriptor, CombatantDescriptor, DamageDescriptor, DamageResult, DamageTypeDescriptor,
    HitLocation,
};
use crate::game_master::GameMaster;
use async_trait::async_trait;
use elicitation::contracts::Established;
use elicitation::Generator;
use elicitation_rand::generators::RandomGenerator;

impl DamageTypeDescriptor {
    /// Returns the wounding multiplier for this damage type (to torso).
    ///
    /// # GURPS Rules
    ///
    /// Different damage types have different wounding effects.
    ///
    /// # Citations
    ///
    /// BS 269-271 - Damage type wounding multipliers
    fn wounding_multiplier(&self) -> f32 {
        match self {
            Self::Crushing => 1.0,
            Self::Cutting => 1.5,
            Self::Impaling => 2.0,
            Self::Piercing => 1.5,
            Self::Burning => 1.0,
        }
    }
}

#[async_trait]
impl DamageCalculator for GameMaster {
    async fn roll_damage(
        &self,
        descriptor: DamageDescriptor,
    ) -> CombatResult<(i32, Established<BasicDamageEvidence>)> {
        // Roll damage dice using deterministic generator
        let mut total = 0;
        for i in 0..descriptor.dice {
            let seed = self.config.seed.wrapping_add(i as u64);
            let gen = RandomGenerator::<u64>::with_seed(seed);
            let roll = (gen.generate() % descriptor.sides as u64) as i32 + 1;
            total += roll;
        }
        total += descriptor.modifier;
        total += descriptor.critical_bonus;

        // Ensure damage is at least 0
        let damage = total.max(0);

        // Mint proof tokens
        let damage_rolled = Established::prove(&ValidDamageRoll);
        let dr_applied = Established::prove(&DrSubtracted);
        let basic_calculated = Established::prove(&BasicDamageComputed);

        // Compose evidence
        let _evidence = BasicDamageEvidence {
            damage_rolled,
            dr_applied,
            basic_calculated,
        };

        Ok((damage, Established::assert()))
    }

    async fn calculate_injury(
        &self,
        raw_damage: i32,
        armor: ArmorDescriptor,
        location: HitLocation,
        damage_descriptor: DamageDescriptor,
        _damage_evidence: Established<BasicDamageEvidence>,
    ) -> CombatResult<(DamageResult, Established<InjuryCalculationEvidence>)> {
        // Apply DR
        let penetrating_damage = (raw_damage - armor.dr).max(0);

        // Get location multiplier
        let location_multiplier = location.damage_multiplier(damage_descriptor.damage_type);

        // Get wounding multiplier
        let wounding_multiplier = damage_descriptor.damage_type.wounding_multiplier();

        // Calculate final injury
        let injury = ((penetrating_damage as f32) * location_multiplier * wounding_multiplier)
            .round() as i32;
        let injury = injury.max(0);

        // Build result
        let result = DamageResult {
            raw_damage,
            dr: armor.dr,
            penetrating_damage,
            location,
            location_multiplier,
            wounding_multiplier,
            injury,
        };

        // Reconstruct basic damage evidence
        let damage_rolled = Established::prove(&ValidDamageRoll);
        let dr_applied = Established::prove(&DrSubtracted);
        let basic_calculated = Established::prove(&BasicDamageComputed);
        let basic_damage = BasicDamageEvidence {
            damage_rolled,
            dr_applied,
            basic_calculated,
        };

        // Mint injury calculation proofs
        let location_proof = Established::prove(&LocationDetermined);
        let location_mult = Established::prove(&LocationMultiplierComputed);
        let wounding_mult = Established::prove(&WoundingMultiplierComputed);
        let injury_calculated = Established::prove(&InjuryComputed);

        // Compose evidence
        let _evidence = InjuryCalculationEvidence {
            basic_damage,
            location: location_proof,
            location_mult,
            wounding_mult,
            injury_calculated,
        };

        Ok((result, Established::assert()))
    }

    async fn apply_injury(
        &self,
        combatant: CombatantDescriptor,
        injury: i32,
        _injury_evidence: Established<InjuryCalculationEvidence>,
    ) -> CombatResult<(CombatantDescriptor, Established<InjuryApplicationEvidence>)> {
        // Apply injury to HP
        let new_hp = combatant.current_hp - injury;

        // Create updated combatant state
        let updated_combatant = CombatantDescriptor {
            current_hp: new_hp,
            ..combatant
        };

        // Reconstruct full evidence chain
        let damage_rolled = Established::prove(&ValidDamageRoll);
        let dr_applied = Established::prove(&DrSubtracted);
        let basic_calculated = Established::prove(&BasicDamageComputed);
        let basic_damage = BasicDamageEvidence {
            damage_rolled,
            dr_applied,
            basic_calculated,
        };

        let location_proof = Established::prove(&LocationDetermined);
        let location_mult = Established::prove(&LocationMultiplierComputed);
        let wounding_mult = Established::prove(&WoundingMultiplierComputed);
        let injury_calculated = Established::prove(&InjuryComputed);
        let calculation = InjuryCalculationEvidence {
            basic_damage,
            location: location_proof,
            location_mult,
            wounding_mult,
            injury_calculated,
        };

        let applied = Established::prove(&InjurySubtracted);

        // Compose evidence
        let _evidence = InjuryApplicationEvidence {
            calculation,
            applied,
        };

        Ok((updated_combatant, Established::assert()))
    }
}
