//! Tests for clothing items.

use valinoreth::{Item, ItemCategory};

#[test]
fn test_boots_properties() {
    let boots = Item::Boots;

    // Category
    assert_eq!(boots.category(), ItemCategory::Clothing);

    // Universal properties
    assert_eq!(boots.base_cost().amount(), 80.0);
    assert_eq!(boots.weight().amount(), 2.0);
    assert_eq!(boots.tech_level().level(), 1); // Bronze Age

    // Clothing items don't have weapon or armor properties
    assert!(boots.weapon_damage().is_none());
    assert!(boots.reach().is_none());
    assert!(boots.parry_modifier().is_none());
    assert!(boots.required_skill().is_none());
    assert!(boots.accuracy().is_none());
    assert!(boots.damage_resistance().is_none());
}

#[test]
fn test_cloak_properties() {
    let cloak = Item::Cloak;

    assert_eq!(cloak.category(), ItemCategory::Clothing);
    assert_eq!(cloak.base_cost().amount(), 50.0);
    assert_eq!(cloak.weight().amount(), 4.0);
    assert_eq!(cloak.tech_level().level(), 1);
}

#[test]
fn test_all_clothing_have_properties() {
    use strum::IntoEnumIterator;

    for item in Item::iter() {
        if item.category() == ItemCategory::Clothing {
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
            assert!(item.tech_level().level() <= 12, "{:?} has invalid TL", item);

            // Clothing doesn't have weapon or armor properties
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
fn test_clothing_tech_levels() {
    // Stone Age (TL 0)
    assert_eq!(Item::Belt.tech_level().level(), 0);
    assert_eq!(Item::Sandals.tech_level().level(), 0);

    // Bronze Age (TL 1)
    assert_eq!(Item::Clothing.tech_level().level(), 1);
    assert_eq!(Item::Boots.tech_level().level(), 1);
    assert_eq!(Item::Gloves.tech_level().level(), 1);

    // Medieval (TL 2)
    assert_eq!(Item::HeavyBoots.tech_level().level(), 2);
    assert_eq!(Item::ReinforcedGloves.tech_level().level(), 2);
}

#[test]
fn test_clothing_weight_range() {
    // Check reasonable weight ranges
    assert!(Item::Belt.weight().amount() < 1.0); // Very light
    assert!(Item::Cloak.weight().amount() > 3.0); // Heavier item
}
