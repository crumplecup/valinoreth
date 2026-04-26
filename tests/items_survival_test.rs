//! Tests for survival gear items.

use valinoreth::{Item, ItemCategory};

#[test]
fn test_torch_properties() {
    let torch = Item::Torch;

    // Category
    assert_eq!(torch.category(), ItemCategory::Survival);

    // Universal properties
    assert_eq!(torch.base_cost().amount(), 3.0);
    assert_eq!(torch.weight().amount(), 1.0);
    assert_eq!(torch.tech_level().level(), 0); // Stone Age

    // Survival items don't have weapon, armor, or container properties
    assert!(torch.weapon_damage().is_none());
    assert!(torch.reach().is_none());
    assert!(torch.parry_modifier().is_none());
    assert!(torch.required_skill().is_none());
    assert!(torch.accuracy().is_none());
    assert!(torch.damage_resistance().is_none());
    assert!(torch.capacity().is_none());
}

#[test]
fn test_tent_properties() {
    let tent = Item::Tent;

    assert_eq!(tent.category(), ItemCategory::Survival);
    assert_eq!(tent.base_cost().amount(), 50.0);
    assert_eq!(tent.weight().amount(), 5.0);
    assert_eq!(tent.tech_level().level(), 0);
}

#[test]
fn test_all_survival_have_properties() {
    use strum::IntoEnumIterator;

    for item in Item::iter() {
        if item.category() == ItemCategory::Survival {
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

            // Survival items don't have weapon, armor, or container properties
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
fn test_survival_tech_levels() {
    // Stone Age (TL 0)
    assert_eq!(Item::Torch.tech_level().level(), 0);
    assert_eq!(Item::Tent.tech_level().level(), 0);
    assert_eq!(Item::Blanket.tech_level().level(), 0);
    assert_eq!(Item::Rations.tech_level().level(), 0);
    assert_eq!(Item::Waterskin.tech_level().level(), 0);
    assert_eq!(Item::Bedroll.tech_level().level(), 0);

    // Bronze Age (TL 1)
    assert_eq!(Item::LargeTent.tech_level().level(), 1);
    assert_eq!(Item::Candle.tech_level().level(), 1);

    // Medieval (TL 2)
    assert_eq!(Item::Lantern.tech_level().level(), 2);

    // Medieval/Renaissance (TL 3)
    assert_eq!(Item::Canteen.tech_level().level(), 3);
}

#[test]
fn test_survival_weight_range() {
    // Very light items
    assert!(Item::Candle.weight().amount() < 0.5);
    assert!(Item::Waterskin.weight().amount() < 0.5);
    assert!(Item::Rations.weight().amount() < 1.0);

    // Medium items
    let torch_weight = Item::Torch.weight().amount();
    assert!(torch_weight >= 1.0 && torch_weight <= 5.0);

    // Heavy items
    assert!(Item::LargeTent.weight().amount() >= 20.0);
}

#[test]
fn test_survival_cost_range() {
    // Cheap consumables
    assert!(Item::Candle.base_cost().amount() < 1.0);
    assert!(Item::Rations.base_cost().amount() < 5.0);

    // Medium cost equipment
    let tent_cost = Item::Tent.base_cost().amount();
    assert!(tent_cost >= 20.0 && tent_cost <= 100.0);

    // Expensive equipment
    assert!(Item::LargeTent.base_cost().amount() >= 100.0);
}
