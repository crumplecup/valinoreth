//! Test GameMaster damage calculation implementation.

use valinoreth::{
    ArmorDescriptor, CombatantDescriptorBuilder, DamageCalculator, DamageDescriptorBuilder,
    DamageTypeDescriptor, GameMaster, HitLocation,
};

#[tokio::test]
async fn test_roll_damage_basic() {
    let gm = GameMaster::new();

    let descriptor = DamageDescriptorBuilder::default()
        .dice(2)
        .sides(6)
        .modifier(1)
        .damage_type(DamageTypeDescriptor::Crushing)
        .build()
        .expect("Valid descriptor");

    let (damage, _evidence) = gm
        .roll_damage(descriptor)
        .await
        .expect("Damage roll succeeded");

    // 2d6+1 should be between 3 and 13
    assert!(
        (3..=13).contains(&damage),
        "2d6+1 should be 3-13, got {damage}"
    );
}

#[tokio::test]
async fn test_roll_damage_with_critical_bonus() {
    let gm = GameMaster::new();

    let descriptor = DamageDescriptorBuilder::default()
        .dice(1)
        .sides(6)
        .modifier(0)
        .critical_bonus(5)
        .damage_type(DamageTypeDescriptor::Cutting)
        .build()
        .expect("Valid descriptor");

    let (damage, _evidence) = gm
        .roll_damage(descriptor)
        .await
        .expect("Damage roll succeeded");

    // 1d6+0+5 should be between 6 and 11
    assert!(
        (6..=11).contains(&damage),
        "1d6+5 should be 6-11, got {damage}"
    );
}

#[tokio::test]
async fn test_seeded_damage_rolls_replay_advancing_stream() {
    let descriptor = DamageDescriptorBuilder::default()
        .dice(2)
        .sides(6)
        .modifier(0)
        .damage_type(DamageTypeDescriptor::Crushing)
        .build()
        .expect("Valid descriptor");

    let gm = GameMaster::with_seed(42);
    let mut rolls = Vec::new();
    for _ in 0..8 {
        let (damage, _evidence) = gm
            .roll_damage(descriptor.clone())
            .await
            .expect("Damage roll succeeded");
        rolls.push(damage);
    }

    let replay = GameMaster::with_seed(42);
    let mut replayed_rolls = Vec::new();
    for _ in 0..8 {
        let (damage, _evidence) = replay
            .roll_damage(descriptor.clone())
            .await
            .expect("Damage roll succeeded");
        replayed_rolls.push(damage);
    }

    assert_eq!(rolls, replayed_rolls);
    assert!(
        rolls.windows(2).any(|pair| pair[0] != pair[1]),
        "seeded damage rolls must consume one dice stream instead of recreating it per roll"
    );
}

#[tokio::test]
async fn test_calculate_injury_no_armor() {
    let gm = GameMaster::new();

    let damage_descriptor = DamageDescriptorBuilder::default()
        .dice(2)
        .sides(6)
        .modifier(0)
        .damage_type(DamageTypeDescriptor::Crushing)
        .build()
        .expect("Valid descriptor");

    let (raw_damage, damage_evidence) = gm
        .roll_damage(damage_descriptor.clone())
        .await
        .expect("Damage roll succeeded");

    let armor = ArmorDescriptor {
        dr: 0,
        flexible: false,
    };

    let (result, _evidence) = gm
        .calculate_injury(
            raw_damage,
            armor,
            HitLocation::Torso,
            damage_descriptor,
            damage_evidence,
        )
        .await
        .expect("Injury calculation succeeded");

    // No armor, torso location (×1), crushing (×1)
    // Injury should equal raw damage
    assert_eq!(result.raw_damage, raw_damage);
    assert_eq!(result.dr, 0);
    assert_eq!(result.penetrating_damage, raw_damage);
    assert_eq!(result.location_multiplier, 1.0);
    assert_eq!(result.wounding_multiplier, 1.0);
    assert_eq!(result.injury, raw_damage);
}

#[tokio::test]
async fn test_calculate_injury_with_dr() {
    let gm = GameMaster::new();

    let damage_descriptor = DamageDescriptorBuilder::default()
        .dice(2)
        .sides(6)
        .modifier(0)
        .damage_type(DamageTypeDescriptor::Cutting)
        .build()
        .expect("Valid descriptor");

    let (raw_damage, damage_evidence) = gm
        .roll_damage(damage_descriptor.clone())
        .await
        .expect("Damage roll succeeded");

    let armor = ArmorDescriptor {
        dr: 3,
        flexible: false,
    };

    let (result, _evidence) = gm
        .calculate_injury(
            raw_damage,
            armor,
            HitLocation::Torso,
            damage_descriptor,
            damage_evidence,
        )
        .await
        .expect("Injury calculation succeeded");

    // DR 3 subtracted, torso (×1), cutting (×1.5)
    let expected_penetrating = (raw_damage - 3).max(0);
    let expected_injury = ((expected_penetrating as f32) * 1.0 * 1.5).round() as i32;

    assert_eq!(result.raw_damage, raw_damage);
    assert_eq!(result.dr, 3);
    assert_eq!(result.penetrating_damage, expected_penetrating);
    assert_eq!(result.location_multiplier, 1.0);
    assert_eq!(result.wounding_multiplier, 1.5);
    assert_eq!(result.injury, expected_injury);
}

#[tokio::test]
async fn test_calculate_injury_skull_location() {
    let gm = GameMaster::new();

    let damage_descriptor = DamageDescriptorBuilder::default()
        .dice(1)
        .sides(6)
        .modifier(0)
        .damage_type(DamageTypeDescriptor::Crushing)
        .build()
        .expect("Valid descriptor");

    let (raw_damage, damage_evidence) = gm
        .roll_damage(damage_descriptor.clone())
        .await
        .expect("Damage roll succeeded");

    let armor = ArmorDescriptor {
        dr: 0,
        flexible: false,
    };

    let (result, _evidence) = gm
        .calculate_injury(
            raw_damage,
            armor,
            HitLocation::Skull,
            damage_descriptor,
            damage_evidence,
        )
        .await
        .expect("Injury calculation succeeded");

    // Skull (×4), crushing (×1)
    let expected_injury = ((raw_damage as f32) * 4.0 * 1.0).round() as i32;

    assert_eq!(result.location_multiplier, 4.0);
    assert_eq!(result.wounding_multiplier, 1.0);
    assert_eq!(result.injury, expected_injury);
}

#[tokio::test]
async fn test_calculate_injury_vitals_impaling() {
    let gm = GameMaster::new();

    let damage_descriptor = DamageDescriptorBuilder::default()
        .dice(1)
        .sides(6)
        .modifier(0)
        .damage_type(DamageTypeDescriptor::Impaling)
        .build()
        .expect("Valid descriptor");

    let (raw_damage, damage_evidence) = gm
        .roll_damage(damage_descriptor.clone())
        .await
        .expect("Damage roll succeeded");

    let armor = ArmorDescriptor {
        dr: 0,
        flexible: false,
    };

    let (result, _evidence) = gm
        .calculate_injury(
            raw_damage,
            armor,
            HitLocation::Vitals,
            damage_descriptor,
            damage_evidence,
        )
        .await
        .expect("Injury calculation succeeded");

    // Vitals with impaling (×3), impaling base (×2)
    // Note: location multiplier already accounts for damage type
    let expected_injury = ((raw_damage as f32) * 3.0 * 2.0).round() as i32;

    assert_eq!(result.location_multiplier, 3.0);
    assert_eq!(result.wounding_multiplier, 2.0);
    assert_eq!(result.injury, expected_injury);
}

#[tokio::test]
async fn test_apply_injury() {
    let gm = GameMaster::new();

    let combatant = CombatantDescriptorBuilder::default()
        .current_hp(15)
        .max_hp(15)
        .dodge(10)
        .build()
        .expect("Valid combatant");

    let damage_descriptor = DamageDescriptorBuilder::default()
        .dice(1)
        .sides(6)
        .modifier(0)
        .damage_type(DamageTypeDescriptor::Crushing)
        .build()
        .expect("Valid descriptor");

    let (raw_damage, damage_evidence) = gm
        .roll_damage(damage_descriptor.clone())
        .await
        .expect("Damage roll succeeded");

    let armor = ArmorDescriptor {
        dr: 0,
        flexible: false,
    };

    let (result, injury_evidence) = gm
        .calculate_injury(
            raw_damage,
            armor,
            HitLocation::Torso,
            damage_descriptor,
            damage_evidence,
        )
        .await
        .expect("Injury calculation succeeded");

    let (updated_combatant, _evidence) = gm
        .apply_injury(combatant.clone(), result.injury, injury_evidence)
        .await
        .expect("Injury application succeeded");

    // HP should be reduced by injury amount
    assert_eq!(
        updated_combatant.current_hp,
        combatant.current_hp - result.injury
    );
    assert_eq!(updated_combatant.max_hp, combatant.max_hp);
}

#[tokio::test]
async fn test_apply_injury_reduces_to_negative() {
    let gm = GameMaster::new();

    let combatant = CombatantDescriptorBuilder::default()
        .current_hp(5)
        .max_hp(15)
        .dodge(10)
        .build()
        .expect("Valid combatant");

    let damage_descriptor = DamageDescriptorBuilder::default()
        .dice(3)
        .sides(6)
        .modifier(5)
        .damage_type(DamageTypeDescriptor::Crushing)
        .build()
        .expect("Valid descriptor");

    let (raw_damage, damage_evidence) = gm
        .roll_damage(damage_descriptor.clone())
        .await
        .expect("Damage roll succeeded");

    let armor = ArmorDescriptor {
        dr: 0,
        flexible: false,
    };

    let (result, injury_evidence) = gm
        .calculate_injury(
            raw_damage,
            armor,
            HitLocation::Torso,
            damage_descriptor,
            damage_evidence,
        )
        .await
        .expect("Injury calculation succeeded");

    let (updated_combatant, _evidence) = gm
        .apply_injury(combatant.clone(), result.injury, injury_evidence)
        .await
        .expect("Injury application succeeded");

    // HP can go negative
    assert!(updated_combatant.current_hp <= 0);
}
