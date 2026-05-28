//! Complete combat execution implementation.

use crate::contracts::credentials::{
    AttackHit, AttackOutcomeChecked, BasicDamageComputed, DefenseDidNotWork, DefenseOutcomeChecked,
    DrSubtracted, InjuryComputed, InjurySubtracted, LocationDetermined, LocationMultiplierComputed,
    ValidAttackRoll, ValidDamageRoll, ValidDefenseRoll, WoundingMultiplierComputed,
};
use crate::contracts::proof_composition::{
    AttackResolutionEvidence, AttackSuccessEvidence, BasicDamageEvidence, CombatHitEvidence,
    DefenseFailureEvidence, DefenseResolutionEvidence, InjuryApplicationEvidence,
    InjuryCalculationEvidence,
};
use crate::contracts::traits::{
    AttackResolver, CombatExchangeResult, CombatExecutor, CombatResult, DamageCalculator,
    DefenseResolver, MissReason,
};
use crate::contracts::types::{
    ArmorDescriptor, AttackDescriptor, CombatantDescriptor, DamageDescriptor, DefenseDescriptor,
    HitLocation,
};
use crate::game_master::GameMaster;
use async_trait::async_trait;
use elicitation::contracts::Established;
use tracing::instrument;

#[async_trait]
impl CombatExecutor for GameMaster {
    #[instrument(skip(self, _attacker, defender, attack_descriptor, defense_descriptor, damage_descriptor, armor))]
    async fn execute_attack(
        &self,
        _attacker: CombatantDescriptor,
        defender: CombatantDescriptor,
        attack_descriptor: AttackDescriptor,
        defense_descriptor: DefenseDescriptor,
        damage_descriptor: DamageDescriptor,
        armor: ArmorDescriptor,
    ) -> CombatResult<CombatExchangeResult> {
        // Step 1: Resolve attack
        let (attack_result, attack_evidence) = self.resolve_attack(attack_descriptor).await?;

        // Check if attack failed
        if !attack_result.success {
            let reason = if attack_result.critical_failure {
                MissReason::CriticalFailure
            } else {
                MissReason::AttackFailed
            };

            return Ok(CombatExchangeResult::Miss {
                attack_result,
                defense_result: None,
                reason,
            });
        }

        // Step 2: Attack succeeded, resolve defense
        let (defense_result, defense_evidence) = self.resolve_defense(defense_descriptor).await?;

        // Check if defense succeeded
        if defense_result.success {
            return Ok(CombatExchangeResult::Miss {
                attack_result,
                defense_result: Some(defense_result),
                reason: MissReason::DefenseSucceeded,
            });
        }

        // Step 3: Attack hit and defense failed - calculate damage
        let (raw_damage, damage_evidence) = self.roll_damage(damage_descriptor.clone()).await?;

        // Step 4: Calculate injury
        let location = HitLocation::Torso; // Default to torso for now
        let (damage_result, injury_evidence) = self
            .calculate_injury(
                raw_damage,
                armor,
                location,
                damage_descriptor,
                damage_evidence,
            )
            .await?;

        // Step 5: Apply injury to defender
        let (updated_defender, _application_evidence) = self
            .apply_injury(defender, damage_result.injury, injury_evidence)
            .await?;

        // Confirm attack success
        let _attack_success = self
            .confirm_attack_success(attack_result, attack_evidence)
            .await?;

        // Confirm defense failure
        let _defense_failure = self
            .confirm_defense_failure(defense_result, defense_evidence)
            .await?;

        // Reconstruct complete evidence chain for hit
        let attack_success_evidence = reconstruct_attack_success();
        let defense_failure_evidence = reconstruct_defense_failure();
        let injury_application_evidence = reconstruct_injury_application();

        let hit_evidence = CombatHitEvidence {
            attack: attack_success_evidence,
            defense: defense_failure_evidence,
            injury: injury_application_evidence,
        };

        Ok(CombatExchangeResult::Hit {
            defender: updated_defender,
            damage: damage_result,
            evidence: Established::prove(&hit_evidence),
        })
    }
}

/// Reconstruct attack success evidence (zero-sized types).
fn reconstruct_attack_success() -> AttackSuccessEvidence {
    let roll_made = Established::prove(&ValidAttackRoll);
    let outcome = Established::prove(&AttackOutcomeChecked);
    let resolution = AttackResolutionEvidence { roll_made, outcome };
    let success = Established::prove(&AttackHit);
    AttackSuccessEvidence {
        resolution,
        success,
    }
}

/// Reconstruct defense failure evidence (zero-sized types).
fn reconstruct_defense_failure() -> DefenseFailureEvidence {
    let roll_made = Established::prove(&ValidDefenseRoll);
    let outcome = Established::prove(&DefenseOutcomeChecked);
    let resolution = DefenseResolutionEvidence { roll_made, outcome };
    let failure = Established::prove(&DefenseDidNotWork);
    DefenseFailureEvidence {
        resolution,
        failure,
    }
}

/// Reconstruct injury application evidence (zero-sized types).
fn reconstruct_injury_application() -> InjuryApplicationEvidence {
    let damage_rolled = Established::prove(&ValidDamageRoll);
    let dr_applied = Established::prove(&DrSubtracted);
    let basic_calculated = Established::prove(&BasicDamageComputed);
    let basic_damage = BasicDamageEvidence {
        damage_rolled,
        dr_applied,
        basic_calculated,
    };

    let location = Established::prove(&LocationDetermined);
    let location_mult = Established::prove(&LocationMultiplierComputed);
    let wounding_mult = Established::prove(&WoundingMultiplierComputed);
    let injury_calculated = Established::prove(&InjuryComputed);
    let calculation = InjuryCalculationEvidence {
        basic_damage,
        location,
        location_mult,
        wounding_mult,
        injury_calculated,
    };

    let applied = Established::prove(&InjurySubtracted);
    InjuryApplicationEvidence {
        calculation,
        applied,
    }
}
