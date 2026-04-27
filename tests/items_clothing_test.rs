//! Tests for clothing items.

use valinoreth::{Clothing, Item};

#[test]
fn test_boots_properties() {
    let boots = Item::Clothing(Clothing::Boots);

    assert_eq!(boots.base_cost().amount(), 80.0);
    assert_eq!(boots.weight().amount(), 2.0);
    assert_eq!(boots.tech_level().level(), 1);

    // Clothing items don't have weapon or armor properties
    assert!(boots.weapon_damage().is_none());
    assert!(boots.reach().is_none());
    assert!(boots.parry_modifier().is_none());
    assert!(boots.required_skill().is_none());
    assert!(boots.accuracy().is_none());
    assert!(boots.damage_resistance().is_none());
    assert!(boots.capacity().is_none());
}

#[test]
fn test_cloak_properties() {
    let cloak = Item::Clothing(Clothing::Cloak);

    assert_eq!(cloak.base_cost().amount(), 50.0);
    assert_eq!(cloak.weight().amount(), 4.0);
    assert_eq!(cloak.tech_level().level(), 1);
}

#[test]
fn test_all_clothing_have_properties() {
    use strum::IntoEnumIterator;

    for clothing in Clothing::iter() {
        let item = Item::Clothing(clothing.clone());

        assert!(
            item.base_cost().amount() >= 0.0,
            "{:?} has invalid cost",
            clothing
        );
        assert!(
            item.weight().amount() >= 0.0,
            "{:?} has invalid weight",
            clothing
        );
        assert!(item.tech_level().level() <= 12, "{:?} has invalid TL", clothing);

        // Clothing doesn't have weapon/armor properties
        assert!(item.weapon_damage().is_none());
        assert!(item.damage_resistance().is_none());
        assert!(item.capacity().is_none());
    }
}

#[test]
fn test_clothing_tech_levels() {
    // Stone Age (TL 0)
    assert_eq!(
        Item::Clothing(Clothing::Belt).tech_level().level(),
        0
    );
    assert_eq!(
        Item::Clothing(Clothing::Sandals).tech_level().level(),
        0
    );

    // Bronze Age (TL 1)
    assert_eq!(
        Item::Clothing(Clothing::Clothing).tech_level().level(),
        1
    );
    assert_eq!(
        Item::Clothing(Clothing::Boots).tech_level().level(),
        1
    );
    assert_eq!(
        Item::Clothing(Clothing::Gloves).tech_level().level(),
        1
    );

    // Medieval (TL 2)
    assert_eq!(
        Item::Clothing(Clothing::HeavyBoots).tech_level().level(),
        2
    );
    assert_eq!(
        Item::Clothing(Clothing::ReinforcedGloves).tech_level().level(),
        2
    );
}

#[test]
fn test_clothing_weight_range() {
    // Check reasonable weight ranges
    assert!(Item::Clothing(Clothing::Belt).weight().amount() < 1.0);
    assert!(Item::Clothing(Clothing::Cloak).weight().amount() > 3.0);
}
