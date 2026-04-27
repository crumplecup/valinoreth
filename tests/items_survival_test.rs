//! Tests for survival gear items.

use valinoreth::{Item, SurvivalGear};

#[test]
fn test_torch_properties() {
    let torch = Item::Survival(SurvivalGear::Torch);

    // Universal properties
    assert_eq!(torch.base_cost().amount(), 3.0);
    assert_eq!(torch.weight().amount(), 1.0);
    assert_eq!(torch.tech_level().level(), 0);

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
    let tent = Item::Survival(SurvivalGear::Tent);

    assert_eq!(tent.base_cost().amount(), 50.0);
    assert_eq!(tent.weight().amount(), 5.0);
    assert_eq!(tent.tech_level().level(), 0);
}

#[test]
fn test_all_survival_have_properties() {
    use strum::IntoEnumIterator;

    for survival in SurvivalGear::iter() {
        let item = Item::Survival(survival.clone());

        assert!(
            item.base_cost().amount() >= 0.0,
            "{:?} has invalid cost",
            survival
        );
        assert!(
            item.weight().amount() >= 0.0,
            "{:?} has invalid weight",
            survival
        );
        assert!(item.tech_level().level() <= 12, "{:?} has invalid TL", survival);

        // Survival items don't have weapon, armor, or container properties
        assert!(
            item.weapon_damage().is_none(),
            "{:?} should not have damage",
            survival
        );
        assert!(
            item.damage_resistance().is_none(),
            "{:?} should not have DR",
            survival
        );
        assert!(
            item.capacity().is_none(),
            "{:?} should not have capacity",
            survival
        );
    }
}

#[test]
fn test_survival_tech_levels() {
    // Stone Age (TL 0)
    assert_eq!(
        Item::Survival(SurvivalGear::Torch).tech_level().level(),
        0
    );
    assert_eq!(
        Item::Survival(SurvivalGear::Tent).tech_level().level(),
        0
    );
    assert_eq!(
        Item::Survival(SurvivalGear::Blanket).tech_level().level(),
        0
    );
    assert_eq!(
        Item::Survival(SurvivalGear::Rations).tech_level().level(),
        0
    );
    assert_eq!(
        Item::Survival(SurvivalGear::Waterskin).tech_level().level(),
        0
    );
    assert_eq!(
        Item::Survival(SurvivalGear::Bedroll).tech_level().level(),
        0
    );

    // Bronze Age (TL 1)
    assert_eq!(
        Item::Survival(SurvivalGear::LargeTent).tech_level().level(),
        1
    );
    assert_eq!(
        Item::Survival(SurvivalGear::Candle).tech_level().level(),
        1
    );

    // Medieval (TL 2)
    assert_eq!(
        Item::Survival(SurvivalGear::Lantern).tech_level().level(),
        2
    );

    // Medieval/Renaissance (TL 3)
    assert_eq!(
        Item::Survival(SurvivalGear::Canteen).tech_level().level(),
        3
    );
}

#[test]
fn test_survival_weight_range() {
    // Very light items
    assert!(Item::Survival(SurvivalGear::Candle).weight().amount() < 0.5);
    assert!(Item::Survival(SurvivalGear::Waterskin).weight().amount() < 0.5);
    assert!(Item::Survival(SurvivalGear::Rations).weight().amount() < 1.0);

    // Medium items
    let torch_weight = Item::Survival(SurvivalGear::Torch).weight().amount();
    assert!(torch_weight >= 1.0 && torch_weight <= 5.0);

    // Heavy items
    assert!(Item::Survival(SurvivalGear::LargeTent).weight().amount() >= 20.0);
}

#[test]
fn test_survival_cost_range() {
    // Cheap consumables
    assert!(Item::Survival(SurvivalGear::Candle).base_cost().amount() < 1.0);
    assert!(Item::Survival(SurvivalGear::Rations).base_cost().amount() < 5.0);

    // Medium cost equipment
    let tent_cost = Item::Survival(SurvivalGear::Tent).base_cost().amount();
    assert!(tent_cost >= 20.0 && tent_cost <= 100.0);

    // Expensive equipment
    assert!(Item::Survival(SurvivalGear::LargeTent).base_cost().amount() >= 100.0);
}
