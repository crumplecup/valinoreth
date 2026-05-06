//! Tests for container items.

use valinoreth::{Container, Item};

#[test]
fn test_backpack_properties() {
    let backpack = Item::Container(Container::Backpack);

    // Universal properties
    assert_eq!(backpack.base_cost().amount(), 60.0);
    assert_eq!(backpack.weight().amount(), 3.0);
    assert_eq!(backpack.tech_level().level(), 1);

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
    let chest = Item::Container(Container::LargeChest);

    assert_eq!(chest.base_cost().amount(), 300.0);
    assert_eq!(chest.weight().amount(), 30.0);
    assert_eq!(chest.tech_level().level(), 1);
    assert_eq!(chest.capacity().unwrap().amount(), 200.0);
}

#[test]
fn test_all_containers_have_properties() {
    use strum::IntoEnumIterator;

    for container in Container::iter() {
        let item = Item::Container(container.clone());

        assert!(
            item.base_cost().amount() >= 0.0,
            "{:?} has invalid cost",
            container
        );
        assert!(
            item.weight().amount() >= 0.0,
            "{:?} has invalid weight",
            container
        );
        assert!(item.tech_level().level() <= 12, "{:?} has invalid TL", container);
        assert!(item.capacity().is_some(), "{:?} should have capacity", container);

        // Containers don't have weapon or armor properties
        assert!(
            item.weapon_damage().is_none(),
            "{:?} should not have damage",
            container
        );
        assert!(
            item.damage_resistance().is_none(),
            "{:?} should not have DR",
            container
        );
    }
}

#[test]
fn test_container_tech_levels() {
    // Stone Age (TL 0)
    assert_eq!(
        Item::Container(Container::SmallPouch).tech_level().level(),
        0
    );
    assert_eq!(
        Item::Container(Container::Pouch).tech_level().level(),
        0
    );
    assert_eq!(
        Item::Container(Container::LargePouch).tech_level().level(),
        0
    );
    assert_eq!(
        Item::Container(Container::SmallSack).tech_level().level(),
        0
    );
    assert_eq!(
        Item::Container(Container::LargeSack).tech_level().level(),
        0
    );

    // Bronze Age (TL 1)
    assert_eq!(
        Item::Container(Container::SmallBackpack).tech_level().level(),
        1
    );
    assert_eq!(
        Item::Container(Container::Backpack).tech_level().level(),
        1
    );
    assert_eq!(
        Item::Container(Container::SmallChest).tech_level().level(),
        1
    );
    assert_eq!(
        Item::Container(Container::LargeChest).tech_level().level(),
        1
    );

    // Medieval (TL 2)
    assert_eq!(
        Item::Container(Container::LargeBackpack).tech_level().level(),
        2
    );
}

#[test]
fn test_container_capacity_range() {
    // Small containers
    assert!(Item::Container(Container::SmallPouch).capacity().unwrap().amount() <= 10.0);

    // Medium containers
    let backpack_capacity = Item::Container(Container::Backpack).capacity().unwrap().amount();
    assert!((30.0..=50.0).contains(&backpack_capacity));

    // Large containers
    assert!(Item::Container(Container::LargeChest).capacity().unwrap().amount() >= 100.0);
}
