//! Tests for tool items.

use valinoreth::{Item, ItemCategory};

#[test]
fn test_lockpicks_properties() {
    let lockpicks = Item::Lockpicks;

    // Category
    assert_eq!(lockpicks.category(), ItemCategory::Tools);

    // Universal properties
    assert_eq!(lockpicks.base_cost().amount(), 50.0);
    assert_eq!(lockpicks.weight().amount(), 0.1);
    assert_eq!(lockpicks.tech_level().level(), 3); // Medieval/Renaissance

    // Tools don't have weapon, armor, or container properties
    assert!(lockpicks.weapon_damage().is_none());
    assert!(lockpicks.reach().is_none());
    assert!(lockpicks.parry_modifier().is_none());
    assert!(lockpicks.required_skill().is_none());
    assert!(lockpicks.accuracy().is_none());
    assert!(lockpicks.damage_resistance().is_none());
    assert!(lockpicks.capacity().is_none());
}

#[test]
fn test_first_aid_kit_properties() {
    let kit = Item::FirstAidKit;

    assert_eq!(kit.category(), ItemCategory::Tools);
    assert_eq!(kit.base_cost().amount(), 50.0);
    assert_eq!(kit.weight().amount(), 2.0);
    assert_eq!(kit.tech_level().level(), 5); // Industrial
}

#[test]
fn test_all_tools_have_properties() {
    use strum::IntoEnumIterator;

    for item in Item::iter() {
        if item.category() == ItemCategory::Tools {
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

            // Tools don't have weapon, armor, or container properties
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
            assert!(
                item.capacity().is_none(),
                "{:?} should not have capacity",
                item
            );
        }
    }
}

#[test]
fn test_tool_tech_levels() {
    // Stone Age (TL 0)
    assert_eq!(Item::Rope.tech_level().level(), 0);
    assert_eq!(Item::Hammer.tech_level().level(), 0);

    // Bronze Age (TL 1)
    assert_eq!(Item::Toolkit.tech_level().level(), 1);
    assert_eq!(Item::Crowbar.tech_level().level(), 1);
    assert_eq!(Item::Shovel.tech_level().level(), 1);

    // Medieval (TL 2)
    assert_eq!(Item::Grapnel.tech_level().level(), 2);
    assert_eq!(Item::Saw.tech_level().level(), 2);

    // Medieval/Renaissance (TL 3)
    assert_eq!(Item::Lockpicks.tech_level().level(), 3);

    // Renaissance (TL 4)
    assert_eq!(Item::MagnifyingGlass.tech_level().level(), 4);

    // Industrial (TL 5)
    assert_eq!(Item::FirstAidKit.tech_level().level(), 5);
}

#[test]
fn test_tool_weight_range() {
    // Very light tools
    assert!(Item::Lockpicks.weight().amount() < 0.5);
    assert!(Item::MagnifyingGlass.weight().amount() < 0.5);

    // Medium tools
    let rope_weight = Item::Rope.weight().amount();
    assert!(rope_weight >= 1.0 && rope_weight <= 3.0);

    // Heavy tools
    assert!(Item::Shovel.weight().amount() >= 5.0);
    assert!(Item::Toolkit.weight().amount() >= 10.0);
}
