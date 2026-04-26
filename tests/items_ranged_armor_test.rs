//! Tests for ranged weapon and armor items.

use valinoreth::{Item, ItemCategory};

// Ranged Weapon Tests

#[test]
fn test_rifle_properties() {
    let rifle = Item::Rifle;

    // Category
    assert_eq!(rifle.category(), ItemCategory::RangedWeapon);

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
    let bow = Item::Bow;

    assert_eq!(bow.category(), ItemCategory::RangedWeapon);
    assert_eq!(bow.base_cost().amount(), 100.0);
    assert_eq!(bow.tech_level().level(), 0); // Stone Age
    assert_eq!(bow.accuracy(), Some(2));
}

#[test]
fn test_all_ranged_weapons_have_properties() {
    use strum::IntoEnumIterator;

    for item in Item::iter() {
        if item.category() == ItemCategory::RangedWeapon {
            assert!(
                item.base_cost().amount() >= 0.0,
                "{:?} has invalid cost",
                item
            );
            assert!(
                item.weight().amount() >= 0.0,
                "{:?} has invalid weight",
                item
            );
            assert!(
                item.tech_level().level() <= 12,
                "{:?} has invalid TL",
                item
            );
            assert!(item.weapon_damage().is_some(), "{:?} missing damage", item);
            assert!(item.accuracy().is_some(), "{:?} missing accuracy", item);
            assert!(
                item.required_skill().is_some(),
                "{:?} missing skill",
                item
            );

            // Ranged weapons don't have melee properties
            assert!(item.reach().is_none(), "{:?} should not have reach", item);
            assert!(
                item.parry_modifier().is_none(),
                "{:?} should not have parry",
                item
            );
        }
    }
}

// Armor Tests

#[test]
fn test_chainmail_properties() {
    let chainmail = Item::Chainmail;

    // Category
    assert_eq!(chainmail.category(), ItemCategory::Armor);

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
    let none = Item::NoArmor;

    assert_eq!(none.category(), ItemCategory::Armor);
    assert_eq!(none.base_cost().amount(), 0.0);
    assert_eq!(none.weight().amount(), 0.0);
    assert_eq!(none.damage_resistance(), Some(0));
}

#[test]
fn test_plate_armor() {
    let plate = Item::PlateArmor;

    assert_eq!(plate.category(), ItemCategory::Armor);
    assert_eq!(plate.base_cost().amount(), 3000.0);
    assert_eq!(plate.weight().amount(), 50.0);
    assert_eq!(plate.tech_level().level(), 3); // Age of Sail
    assert_eq!(plate.damage_resistance(), Some(6));
}

#[test]
fn test_all_armor_have_properties() {
    use strum::IntoEnumIterator;

    for item in Item::iter() {
        if item.category() == ItemCategory::Armor {
            assert!(
                item.base_cost().amount() >= 0.0,
                "{:?} has invalid cost",
                item
            );
            assert!(
                item.weight().amount() >= 0.0,
                "{:?} has invalid weight",
                item
            );
            assert!(
                item.tech_level().level() <= 12,
                "{:?} has invalid TL",
                item
            );
            assert!(
                item.damage_resistance().is_some(),
                "{:?} missing DR",
                item
            );

            // Armor doesn't have weapon properties
            assert!(
                item.weapon_damage().is_none(),
                "{:?} should not have damage",
                item
            );
            assert!(item.reach().is_none(), "{:?} should not have reach", item);
            assert!(
                item.parry_modifier().is_none(),
                "{:?} should not have parry",
                item
            );
            assert!(
                item.required_skill().is_none(),
                "{:?} should not require skill",
                item
            );
            assert!(
                item.accuracy().is_none(),
                "{:?} should not have accuracy",
                item
            );
        }
    }
}

#[test]
fn test_armor_dr_progression() {
    // DR should increase with better armor
    assert_eq!(Item::NoArmor.damage_resistance(), Some(0));
    assert_eq!(Item::LeatherArmor.damage_resistance(), Some(1));
    assert_eq!(Item::Chainmail.damage_resistance(), Some(4));
    assert_eq!(Item::PlateArmor.damage_resistance(), Some(6));
    assert_eq!(Item::HeavyPlate.damage_resistance(), Some(8));
}
