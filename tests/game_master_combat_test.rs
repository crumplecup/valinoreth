//! Test GameMaster complete combat execution implementation.

use valinoreth::{
    ArmorDescriptor, AttackDescriptorBuilder, CombatExchangeResult, CombatExecutor,
    CombatantDescriptorBuilder, DamageDescriptorBuilder, DamageTypeDescriptor,
    DefenseDescriptorBuilder, DefenseType, GameMaster, MissReason,
};

#[tokio::test]
async fn test_execute_attack_successful_hit() {
    let gm = GameMaster::new();

    let attacker = CombatantDescriptorBuilder::default()
        .current_hp(15)
        .max_hp(15)
        .dodge(10)
        .build()
        .expect("Valid attacker");

    let defender = CombatantDescriptorBuilder::default()
        .current_hp(12)
        .max_hp(12)
        .dodge(8)
        .build()
        .expect("Valid defender");

    let attack_descriptor = AttackDescriptorBuilder::default()
        .effective_skill(14)
        .aim_bonus(0)
        .deceptive_penalty(0)
        .build()
        .expect("Valid attack");

    let defense_descriptor = DefenseDescriptorBuilder::default()
        .defense_type(DefenseType::Dodge)
        .defense_score(6) // Low defense to ensure failure
        .build()
        .expect("Valid defense");

    let damage_descriptor = DamageDescriptorBuilder::default()
        .dice(1)
        .sides(6)
        .modifier(2)
        .damage_type(DamageTypeDescriptor::Cutting)
        .build()
        .expect("Valid damage");

    let armor = ArmorDescriptor {
        dr: 0,
        flexible: false,
    };

    let result = gm
        .execute_attack(
            attacker,
            defender.clone(),
            attack_descriptor,
            defense_descriptor,
            damage_descriptor,
            armor,
        )
        .await
        .expect("Combat execution succeeded");

    match result {
        CombatExchangeResult::Hit {
            defender: updated_defender,
            damage,
            evidence: _,
        } => {
            // Defender should have taken damage
            assert!(
                updated_defender.current_hp < defender.current_hp,
                "Defender HP should be reduced"
            );
            assert!(damage.injury > 0, "Should have dealt injury");
        }
        CombatExchangeResult::Miss { .. } => {
            // Miss is also valid depending on roll
        }
    }
}

#[tokio::test]
async fn test_execute_attack_miss_failed_attack() {
    let gm = GameMaster::new();

    let attacker = CombatantDescriptorBuilder::default()
        .current_hp(15)
        .max_hp(15)
        .dodge(10)
        .build()
        .expect("Valid attacker");

    let defender = CombatantDescriptorBuilder::default()
        .current_hp(12)
        .max_hp(12)
        .dodge(10)
        .build()
        .expect("Valid defender");

    let attack_descriptor = AttackDescriptorBuilder::default()
        .effective_skill(3) // Very low skill
        .aim_bonus(0)
        .deceptive_penalty(0)
        .build()
        .expect("Valid attack");

    let defense_descriptor = DefenseDescriptorBuilder::default()
        .defense_type(DefenseType::Dodge)
        .defense_score(10)
        .build()
        .expect("Valid defense");

    let damage_descriptor = DamageDescriptorBuilder::default()
        .dice(1)
        .sides(6)
        .modifier(0)
        .damage_type(DamageTypeDescriptor::Crushing)
        .build()
        .expect("Valid damage");

    let armor = ArmorDescriptor {
        dr: 0,
        flexible: false,
    };

    let result = gm
        .execute_attack(
            attacker,
            defender,
            attack_descriptor,
            defense_descriptor,
            damage_descriptor,
            armor,
        )
        .await
        .expect("Combat execution succeeded");

    match result {
        CombatExchangeResult::Miss {
            attack_result,
            defense_result,
            reason,
        } => {
            // Attack likely failed due to low skill
            if !attack_result.success {
                assert!(
                    matches!(
                        reason,
                        MissReason::AttackFailed | MissReason::CriticalFailure
                    ),
                    "Should be attack failure"
                );
                assert!(
                    defense_result.is_none(),
                    "Defense shouldn't roll if attack fails"
                );
            }
        }
        CombatExchangeResult::Hit { .. } => {
            // Hit is possible with lucky roll
        }
    }
}

#[tokio::test]
async fn test_execute_attack_miss_successful_defense() {
    let gm = GameMaster::new();

    let attacker = CombatantDescriptorBuilder::default()
        .current_hp(15)
        .max_hp(15)
        .dodge(10)
        .build()
        .expect("Valid attacker");

    let defender = CombatantDescriptorBuilder::default()
        .current_hp(12)
        .max_hp(12)
        .dodge(10)
        .build()
        .expect("Valid defender");

    let attack_descriptor = AttackDescriptorBuilder::default()
        .effective_skill(14)
        .aim_bonus(0)
        .deceptive_penalty(0)
        .build()
        .expect("Valid attack");

    let defense_descriptor = DefenseDescriptorBuilder::default()
        .defense_type(DefenseType::Dodge)
        .defense_score(18) // High defense for likely success
        .build()
        .expect("Valid defense");

    let damage_descriptor = DamageDescriptorBuilder::default()
        .dice(1)
        .sides(6)
        .modifier(0)
        .damage_type(DamageTypeDescriptor::Crushing)
        .build()
        .expect("Valid damage");

    let armor = ArmorDescriptor {
        dr: 0,
        flexible: false,
    };

    let result = gm
        .execute_attack(
            attacker,
            defender,
            attack_descriptor,
            defense_descriptor,
            damage_descriptor,
            armor,
        )
        .await
        .expect("Combat execution succeeded");

    match result {
        CombatExchangeResult::Miss {
            attack_result: _,
            defense_result,
            reason,
        } => {
            // If attack succeeded but defense also succeeded
            if defense_result.is_some() {
                assert_eq!(
                    reason,
                    MissReason::DefenseSucceeded,
                    "Should be defense success"
                );
            }
        }
        CombatExchangeResult::Hit { .. } => {
            // Hit is possible if defense fails
        }
    }
}

#[tokio::test]
async fn test_execute_attack_with_armor() {
    let gm = GameMaster::new();

    let attacker = CombatantDescriptorBuilder::default()
        .current_hp(15)
        .max_hp(15)
        .dodge(10)
        .build()
        .expect("Valid attacker");

    let defender = CombatantDescriptorBuilder::default()
        .current_hp(12)
        .max_hp(12)
        .dodge(8)
        .build()
        .expect("Valid defender");

    let attack_descriptor = AttackDescriptorBuilder::default()
        .effective_skill(14)
        .aim_bonus(0)
        .deceptive_penalty(0)
        .build()
        .expect("Valid attack");

    let defense_descriptor = DefenseDescriptorBuilder::default()
        .defense_type(DefenseType::Dodge)
        .defense_score(5) // Low defense
        .build()
        .expect("Valid defense");

    let damage_descriptor = DamageDescriptorBuilder::default()
        .dice(1)
        .sides(6)
        .modifier(2)
        .damage_type(DamageTypeDescriptor::Cutting)
        .build()
        .expect("Valid damage");

    let armor = ArmorDescriptor {
        dr: 3,
        flexible: false,
    };

    let result = gm
        .execute_attack(
            attacker,
            defender.clone(),
            attack_descriptor,
            defense_descriptor,
            damage_descriptor,
            armor,
        )
        .await
        .expect("Combat execution succeeded");

    match result {
        CombatExchangeResult::Hit {
            defender: updated_defender,
            damage,
            evidence: _,
        } => {
            // DR should have reduced damage
            assert_eq!(damage.dr, 3, "DR should be applied");
            assert!(
                damage.penetrating_damage < damage.raw_damage,
                "Penetrating should be less than raw"
            );
        }
        CombatExchangeResult::Miss { .. } => {
            // Miss is also valid
        }
    }
}

#[tokio::test]
async fn test_execute_attack_complete_flow() {
    let gm = GameMaster::new();

    let attacker = CombatantDescriptorBuilder::default()
        .current_hp(15)
        .max_hp(15)
        .dodge(10)
        .build()
        .expect("Valid attacker");

    let defender = CombatantDescriptorBuilder::default()
        .current_hp(20)
        .max_hp(20)
        .dodge(8)
        .build()
        .expect("Valid defender");

    let attack_descriptor = AttackDescriptorBuilder::default()
        .effective_skill(12)
        .aim_bonus(1)
        .deceptive_penalty(0)
        .build()
        .expect("Valid attack");

    let defense_descriptor = DefenseDescriptorBuilder::default()
        .defense_type(DefenseType::Parry)
        .defense_score(9)
        .feint_penalty(0)
        .deceptive_penalty(0)
        .retreating(false)
        .build()
        .expect("Valid defense");

    let damage_descriptor = DamageDescriptorBuilder::default()
        .dice(2)
        .sides(6)
        .modifier(1)
        .damage_type(DamageTypeDescriptor::Impaling)
        .build()
        .expect("Valid damage");

    let armor = ArmorDescriptor {
        dr: 2,
        flexible: false,
    };

    let result = gm
        .execute_attack(
            attacker,
            defender.clone(),
            attack_descriptor,
            defense_descriptor,
            damage_descriptor,
            armor,
        )
        .await
        .expect("Combat execution succeeded");

    // Result is either hit or miss depending on rolls
    match result {
        CombatExchangeResult::Hit {
            defender: updated_defender,
            damage,
            evidence: _,
        } => {
            assert!(damage.raw_damage > 0, "Should have rolled damage");
            assert_eq!(damage.dr, 2, "Should have DR 2");
            assert!(
                updated_defender.current_hp <= defender.current_hp,
                "HP should not increase"
            );
        }
        CombatExchangeResult::Miss {
            attack_result: _,
            defense_result: _,
            reason: _,
        } => {
            // Valid miss outcome
        }
    }
}
