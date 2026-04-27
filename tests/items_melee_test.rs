//! Tests for melee weapon items.

use valinoreth::{Item, MeleeWeapon, Quality, Reach, Skill, WeaponDamage};

#[test]
fn test_broadsword_properties() {
    let sword = Item::MeleeWeapon(MeleeWeapon::Broadsword);

    // Universal properties
    assert_eq!(sword.base_cost().amount(), 500.0);
    assert_eq!(sword.weight().amount(), 3.0);
    assert_eq!(sword.tech_level().level(), 2); // Medieval

    // Weapon-specific properties exist
    assert!(sword.weapon_damage().is_some());
    assert!(sword.reach().is_some());
    assert!(sword.parry_modifier().is_some());
    assert!(sword.required_skill().is_some());

    // Specific weapon stats
    assert!(matches!(
        sword.weapon_damage(),
        Some(WeaponDamage::Swing { modifier: 1, .. })
    ));
    assert_eq!(sword.reach(), Some(Reach::One));
    assert_eq!(sword.parry_modifier(), Some(0));
    assert_eq!(sword.required_skill(), Some(Skill::Broadsword));
}

#[test]
fn test_quality_modifiers() {
    let sword = Item::MeleeWeapon(MeleeWeapon::Broadsword); // Base $500

    // Quality affects cost
    assert_eq!(sword.cost(Quality::Cheap).amount(), 200.0); // ×0.4
    assert_eq!(sword.cost(Quality::Good).amount(), 500.0); // ×1.0
    assert_eq!(sword.cost(Quality::Fine).amount(), 2000.0); // ×4
    assert_eq!(sword.cost(Quality::VeryFine).amount(), 10000.0); // ×20

    // Quality skill bonuses
    assert_eq!(Quality::Cheap.skill_bonus(), 0);
    assert_eq!(Quality::Good.skill_bonus(), 0);
    assert_eq!(Quality::Fine.skill_bonus(), 1);
    assert_eq!(Quality::VeryFine.skill_bonus(), 2);
}

#[test]
fn test_fist_zero_cost() {
    let fist = Item::MeleeWeapon(MeleeWeapon::Fist);

    assert_eq!(fist.base_cost().amount(), 0.0);
    assert_eq!(fist.weight().amount(), 0.0);
    assert_eq!(fist.tech_level().level(), 0); // Stone Age
    assert_eq!(fist.required_skill(), Some(Skill::Brawling));
}

#[test]
fn test_rapier_parry_bonus() {
    let rapier = Item::MeleeWeapon(MeleeWeapon::Rapier);

    // Rapier has +1 parry modifier
    assert_eq!(rapier.parry_modifier(), Some(1));
    assert_eq!(rapier.required_skill(), Some(Skill::Rapier));
}

#[test]
fn test_spear_reach() {
    let spear = Item::MeleeWeapon(MeleeWeapon::Spear);

    // Spear has 1-2 reach
    assert_eq!(spear.reach(), Some(Reach::OneTwo));
    assert!(matches!(
        spear.weapon_damage(),
        Some(WeaponDamage::Thrust { modifier: 2, .. })
    ));
}

#[test]
fn test_all_melee_weapons_have_properties() {
    use strum::IntoEnumIterator;

    // Iterate over category enum instead of filtering top-level Item enum
    for weapon in MeleeWeapon::iter() {
        let item = Item::MeleeWeapon(weapon.clone());

        // All melee weapons must have:
        assert!(
            item.base_cost().amount() >= 0.0,
            "{:?} has invalid cost",
            weapon
        );
        assert!(
            item.weight().amount() >= 0.0,
            "{:?} has invalid weight",
            weapon
        );
        assert!(item.tech_level().level() <= 12, "{:?} has invalid TL", weapon);

        // All melee weapons must have weapon-specific properties
        assert!(item.weapon_damage().is_some(), "{:?} missing damage", weapon);
        assert!(item.reach().is_some(), "{:?} missing reach", weapon);
        assert!(
            item.parry_modifier().is_some(),
            "{:?} missing parry modifier",
            weapon
        );
        assert!(item.required_skill().is_some(), "{:?} missing skill", weapon);
    }
}

#[test]
fn test_tech_levels() {
    // Stone Age weapons (TL 0)
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::Axe)
            .tech_level()
            .level(),
        0
    );
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::Spear)
            .tech_level()
            .level(),
        0
    );
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::Staff)
            .tech_level()
            .level(),
        0
    );

    // Bronze Age (TL 1)
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::Dagger)
            .tech_level()
            .level(),
        1
    );
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::Shortsword)
            .tech_level()
            .level(),
        1
    );

    // Medieval (TL 2)
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::Broadsword)
            .tech_level()
            .level(),
        2
    );
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::TwoHandedSword)
            .tech_level()
            .level(),
        2
    );
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::Halberd)
            .tech_level()
            .level(),
        2
    );

    // Renaissance (TL 4)
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::Rapier)
            .tech_level()
            .level(),
        4
    );
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::Smallsword)
            .tech_level()
            .level(),
        4
    );

    // Modern (TL 5)
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::Baton)
            .tech_level()
            .level(),
        5
    );
}

#[test]
fn test_weapon_damage_types() {
    use strum::IntoEnumIterator;

    for weapon in MeleeWeapon::iter() {
        let item = Item::MeleeWeapon(weapon);

        if let Some(damage) = item.weapon_damage() {
            // Verify damage is either Thrust, Swing, or Fixed
            match damage {
                WeaponDamage::Thrust { .. } => {
                    // Thrust weapons: Dagger, Rapier, Spear, etc.
                }
                WeaponDamage::Swing { .. } => {
                    // Swing weapons: Broadsword, Axe, Mace, etc.
                }
                WeaponDamage::Fixed { .. } => {
                    // Fixed damage (ranged weapons, not common for melee)
                }
            }
        }
    }
}
