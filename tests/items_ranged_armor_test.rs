//! Tests for ranged weapon and armor items.

use valinoreth::{Item, ItemArmor, RangedWeapon};

// Ranged Weapon Tests

#[test]
fn test_rifle_properties() {
    let rifle = Item::RangedWeapon(RangedWeapon::Rifle);

    // Universal properties
    assert_eq!(rifle.base_cost().amount(), 500.0);
    assert_eq!(rifle.weight().amount(), 9.0);
    assert_eq!(rifle.tech_level().level(), 6); // Atomic Age

    // Weapon-specific properties
    assert!(rifle.weapon_damage().is_some());
    assert!(rifle.required_skill().is_some());
    assert_eq!(rifle.accuracy(), Some(5));

    // Not melee or armor
    assert!(rifle.reach().is_none());
    assert!(rifle.parry_modifier().is_none());
    assert!(rifle.damage_resistance().is_none());
}

#[test]
fn test_bow_properties() {
    let bow = Item::RangedWeapon(RangedWeapon::Bow);

    assert_eq!(bow.base_cost().amount(), 100.0);
    assert_eq!(bow.tech_level().level(), 0); // Stone Age
    assert_eq!(bow.accuracy(), Some(2));
}

#[test]
fn test_all_ranged_weapons_have_properties() {
    use strum::IntoEnumIterator;

    for weapon in RangedWeapon::iter() {
        let item = Item::RangedWeapon(weapon.clone());

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
        assert!(item.weapon_damage().is_some(), "{:?} missing damage", weapon);
        assert!(item.accuracy().is_some(), "{:?} missing accuracy", weapon);
        assert!(item.required_skill().is_some(), "{:?} missing skill", weapon);

        // Ranged weapons don't have melee properties
        assert!(item.reach().is_none(), "{:?} should not have reach", weapon);
        assert!(
            item.parry_modifier().is_none(),
            "{:?} should not have parry",
            weapon
        );
    }
}

// Armor Tests

#[test]
fn test_chainmail_properties() {
    let chainmail = Item::Armor(ItemArmor::Chainmail);

    // Universal properties
    assert_eq!(chainmail.base_cost().amount(), 550.0);
    assert_eq!(chainmail.weight().amount(), 35.0);
    assert_eq!(chainmail.tech_level().level(), 2); // Medieval

    // Armor-specific
    assert_eq!(chainmail.damage_resistance(), Some(4));

    // Not a weapon
    assert!(chainmail.weapon_damage().is_none());
    assert!(chainmail.reach().is_none());
    assert!(chainmail.parry_modifier().is_none());
    assert!(chainmail.required_skill().is_none());
    assert!(chainmail.accuracy().is_none());
}

#[test]
fn test_no_armor() {
    let none = Item::Armor(ItemArmor::NoArmor);

    assert_eq!(none.base_cost().amount(), 0.0);
    assert_eq!(none.weight().amount(), 0.0);
    assert_eq!(none.damage_resistance(), Some(0));
}

#[test]
fn test_plate_armor() {
    let plate = Item::Armor(ItemArmor::PlateArmor);

    assert_eq!(plate.base_cost().amount(), 3000.0);
    assert_eq!(plate.weight().amount(), 50.0);
    assert_eq!(plate.tech_level().level(), 3); // Age of Sail
    assert_eq!(plate.damage_resistance(), Some(6));
}

#[test]
fn test_all_armor_have_properties() {
    use strum::IntoEnumIterator;

    for armor in ItemArmor::iter() {
        let item = Item::Armor(armor.clone());

        assert!(
            item.base_cost().amount() >= 0.0,
            "{:?} has invalid cost",
            armor
        );
        assert!(
            item.weight().amount() >= 0.0,
            "{:?} has invalid weight",
            armor
        );
        assert!(item.tech_level().level() <= 12, "{:?} has invalid TL", armor);
        assert!(item.damage_resistance().is_some(), "{:?} missing DR", armor);

        // Armor doesn't have weapon properties
        assert!(
            item.weapon_damage().is_none(),
            "{:?} should not have damage",
            armor
        );
        assert!(item.reach().is_none(), "{:?} should not have reach", armor);
        assert!(
            item.parry_modifier().is_none(),
            "{:?} should not have parry",
            armor
        );
        assert!(
            item.required_skill().is_none(),
            "{:?} should not require skill",
            armor
        );
        assert!(
            item.accuracy().is_none(),
            "{:?} should not have accuracy",
            armor
        );
    }
}

#[test]
fn test_armor_dr_progression() {
    // DR should increase with better armor
    assert_eq!(
        Item::Armor(ItemArmor::NoArmor).damage_resistance(),
        Some(0)
    );
    assert_eq!(
        Item::Armor(ItemArmor::LeatherArmor).damage_resistance(),
        Some(1)
    );
    assert_eq!(
        Item::Armor(ItemArmor::Chainmail).damage_resistance(),
        Some(4)
    );
    assert_eq!(
        Item::Armor(ItemArmor::PlateArmor).damage_resistance(),
        Some(6)
    );
    assert_eq!(
        Item::Armor(ItemArmor::HeavyPlate).damage_resistance(),
        Some(8)
    );
}
