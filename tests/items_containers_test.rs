//! Tests for container items.

use valinoreth::{Item, ItemCategory};

#[test]
fn test_backpack_properties() {
    let backpack = Item::Backpack;

    // Category
    assert_eq!(backpack.category(), ItemCategory::Containers);

    // Universal properties
    assert_eq!(backpack.base_cost().amount(), 60.0);
    assert_eq!(backpack.weight().amount(), 3.0);
    assert_eq!(backpack.tech_level().level(), 1); // Bronze Age

    // Container-specific property
    assert_eq!(backpack.capacity().unwrap().amount(), 40.0);

    // Containers don't have weapon or armor properties
    assert!(backpack.weapon_damage().is_none());
    assert!(backpack.reach().is_none());
    assert!(backpack.parry_modifier().is_none());
    assert!(backpack.required_skill().is_none());
    assert!(backpack.accuracy().is_none());
    assert!(backpack.damage_resistance().is_none());
}

#[test]
fn test_chest_properties() {
    let chest = Item::LargeChest;

    assert_eq!(chest.category(), ItemCategory::Containers);
    assert_eq!(chest.base_cost().amount(), 300.0);
    assert_eq!(chest.weight().amount(), 30.0);
    assert_eq!(chest.tech_level().level(), 1);
    assert_eq!(chest.capacity().unwrap().amount(), 200.0);
}

#[test]
fn test_all_containers_have_properties() {
    use strum::IntoEnumIterator;

    for item in Item::iter() {
        if item.category() == ItemCategory::Containers {
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
                item.capacity().is_some(),
                "{:?} should have capacity",
                item
            );

            // Containers don't have weapon or armor properties
            assert!(
                item.weapon_damage().is_none(),
                "{:?} should not have damage",
                item
            );
            assert!(
                item.damage_resistance().is_none(),
                "{:?} should not have DR",
                item
            );
        }
    }
}

#[test]
fn test_container_tech_levels() {
    // Stone Age (TL 0)
    assert_eq!(Item::SmallPouch.tech_level().level(), 0);
    assert_eq!(Item::Pouch.tech_level().level(), 0);
    assert_eq!(Item::LargePouch.tech_level().level(), 0);
    assert_eq!(Item::SmallSack.tech_level().level(), 0);
    assert_eq!(Item::LargeSack.tech_level().level(), 0);

    // Bronze Age (TL 1)
    assert_eq!(Item::SmallBackpack.tech_level().level(), 1);
    assert_eq!(Item::Backpack.tech_level().level(), 1);
    assert_eq!(Item::SmallChest.tech_level().level(), 1);
    assert_eq!(Item::LargeChest.tech_level().level(), 1);

    // Medieval (TL 2)
    assert_eq!(Item::LargeBackpack.tech_level().level(), 2);
}

#[test]
fn test_container_capacity_range() {
    // Small containers
    assert!(Item::SmallPouch.capacity().unwrap().amount() <= 10.0);

    // Medium containers
    let backpack_capacity = Item::Backpack.capacity().unwrap().amount();
    assert!(backpack_capacity >= 30.0 && backpack_capacity <= 50.0);

    // Large containers
    assert!(Item::LargeChest.capacity().unwrap().amount() >= 100.0);
}
