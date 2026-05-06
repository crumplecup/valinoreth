//! Tests for tool items.

use valinoreth::{Item, Tool};

#[test]
fn test_lockpicks_properties() {
    let lockpicks = Item::Tool(Tool::Lockpicks);

    // Universal properties
    assert_eq!(lockpicks.base_cost().amount(), 50.0);
    assert_eq!(lockpicks.weight().amount(), 0.1);
    assert_eq!(lockpicks.tech_level().level(), 3);

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
    let kit = Item::Tool(Tool::FirstAidKit);

    assert_eq!(kit.base_cost().amount(), 50.0);
    assert_eq!(kit.weight().amount(), 2.0);
    assert_eq!(kit.tech_level().level(), 5);
}

#[test]
fn test_all_tools_have_properties() {
    use strum::IntoEnumIterator;

    for tool in Tool::iter() {
        let item = Item::Tool(tool.clone());

        assert!(
            item.base_cost().amount() >= 0.0,
            "{:?} has invalid cost",
            tool
        );
        assert!(
            item.weight().amount() >= 0.0,
            "{:?} has invalid weight",
            tool
        );
        assert!(item.tech_level().level() <= 12, "{:?} has invalid TL", tool);

        // Tools don't have weapon, armor, or container properties
        assert!(
            item.weapon_damage().is_none(),
            "{:?} should not have damage",
            tool
        );
        assert!(
            item.damage_resistance().is_none(),
            "{:?} should not have DR",
            tool
        );
        assert!(
            item.capacity().is_none(),
            "{:?} should not have capacity",
            tool
        );
    }
}

#[test]
fn test_tool_tech_levels() {
    // Stone Age (TL 0)
    assert_eq!(Item::Tool(Tool::Rope).tech_level().level(), 0);
    assert_eq!(Item::Tool(Tool::Hammer).tech_level().level(), 0);

    // Bronze Age (TL 1)
    assert_eq!(Item::Tool(Tool::Toolkit).tech_level().level(), 1);
    assert_eq!(Item::Tool(Tool::Crowbar).tech_level().level(), 1);
    assert_eq!(Item::Tool(Tool::Shovel).tech_level().level(), 1);

    // Medieval (TL 2)
    assert_eq!(Item::Tool(Tool::Grapnel).tech_level().level(), 2);
    assert_eq!(Item::Tool(Tool::Saw).tech_level().level(), 2);

    // Medieval/Renaissance (TL 3)
    assert_eq!(Item::Tool(Tool::Lockpicks).tech_level().level(), 3);

    // Renaissance (TL 4)
    assert_eq!(
        Item::Tool(Tool::MagnifyingGlass).tech_level().level(),
        4
    );

    // Industrial (TL 5)
    assert_eq!(Item::Tool(Tool::FirstAidKit).tech_level().level(), 5);
}

#[test]
fn test_tool_weight_range() {
    // Very light tools
    assert!(Item::Tool(Tool::Lockpicks).weight().amount() < 0.5);
    assert!(Item::Tool(Tool::MagnifyingGlass).weight().amount() < 0.5);

    // Medium tools
    let rope_weight = Item::Tool(Tool::Rope).weight().amount();
    assert!((1.0..=3.0).contains(&rope_weight));

    // Heavy tools
    assert!(Item::Tool(Tool::Shovel).weight().amount() >= 5.0);
    assert!(Item::Tool(Tool::Toolkit).weight().amount() >= 10.0);
}
