//! Tests for expanded melee weapon items.

use valinoreth::{Item, ItemCategory};

#[test]
fn test_katana_properties() {
    let katana = Item::Katana;

    // Category
    assert_eq!(katana.category(), ItemCategory::MeleeWeapon);

    // Universal properties
    assert_eq!(katana.base_cost().amount(), 650.0);
    assert_eq!(katana.weight().amount(), 2.5);
    assert_eq!(katana.tech_level().level(), 3); // Medieval Japan

    // Weapon-specific
    assert!(katana.weapon_damage().is_some());
    assert!(katana.reach().is_some());
    assert!(katana.parry_modifier().is_some());
    assert!(katana.required_skill().is_some());

    // Not armor/container
    assert!(katana.damage_resistance().is_none());
    assert!(katana.capacity().is_none());
}

#[test]
fn test_bastard_sword_properties() {
    let sword = Item::BastardSword;

    assert_eq!(sword.category(), ItemCategory::MeleeWeapon);
    assert_eq!(sword.base_cost().amount(), 650.0);
    assert_eq!(sword.weight().amount(), 5.0);
    assert_eq!(sword.tech_level().level(), 2); // Medieval
}

#[test]
fn test_nunchaku_properties() {
    let nunchaku = Item::Nunchaku;

    assert_eq!(nunchaku.category(), ItemCategory::MeleeWeapon);
    assert_eq!(nunchaku.base_cost().amount(), 20.0);
    assert_eq!(nunchaku.weight().amount(), 1.5);
    assert_eq!(nunchaku.tech_level().level(), 2);
}

#[test]
fn test_maul_properties() {
    let maul = Item::Maul;

    assert_eq!(maul.category(), ItemCategory::MeleeWeapon);
    assert_eq!(maul.base_cost().amount(), 80.0);
    assert_eq!(maul.weight().amount(), 12.0);
    assert_eq!(maul.tech_level().level(), 1); // Bronze/Iron Age
}

#[test]
fn test_all_expanded_melee_have_properties() {
    let expanded_weapons = vec![
        Item::Katana,
        Item::Scimitar,
        Item::Cutlass,
        Item::Longsword,
        Item::BastardSword,
        Item::Wakizashi,
        Item::Nunchaku,
        Item::Sai,
        Item::Katar,
        Item::Tonfa,
        Item::Estoc,
        Item::Falchion,
        Item::Gladius,
        Item::Maul,
        Item::Pick,
    ];

    for item in expanded_weapons {
        assert_eq!(item.category(), ItemCategory::MeleeWeapon);
        assert!(
            item.base_cost().amount() > 0.0,
            "{:?} has invalid cost",
            item
        );
        assert!(
            item.weight().amount() >= 0.0,
            "{:?} has invalid weight",
            item
        );
        assert!(item.tech_level().level() <= 12, "{:?} has invalid TL", item);
        assert!(item.weapon_damage().is_some(), "{:?} missing damage", item);
        assert!(item.reach().is_some(), "{:?} missing reach", item);
        assert!(item.parry_modifier().is_some(), "{:?} missing parry", item);
        assert!(item.required_skill().is_some(), "{:?} missing skill", item);
    }
}

#[test]
fn test_expanded_weapon_tech_level_range() {
    // Bronze/Iron Age (TL 1)
    assert_eq!(Item::Gladius.tech_level().level(), 1);
    assert_eq!(Item::Maul.tech_level().level(), 1);

    // Medieval (TL 2)
    assert_eq!(Item::Scimitar.tech_level().level(), 2);
    assert_eq!(Item::Longsword.tech_level().level(), 2);
    assert_eq!(Item::BastardSword.tech_level().level(), 2);
    assert_eq!(Item::Nunchaku.tech_level().level(), 2);
    assert_eq!(Item::Sai.tech_level().level(), 2);
    assert_eq!(Item::Katar.tech_level().level(), 2);
    assert_eq!(Item::Falchion.tech_level().level(), 2);
    assert_eq!(Item::Pick.tech_level().level(), 2);

    // Late Medieval/Japan (TL 3)
    assert_eq!(Item::Katana.tech_level().level(), 3);
    assert_eq!(Item::Wakizashi.tech_level().level(), 3);
    assert_eq!(Item::Estoc.tech_level().level(), 3);

    // Age of Sail (TL 4)
    assert_eq!(Item::Cutlass.tech_level().level(), 4);
}

#[test]
fn test_expanded_weapon_weight_range() {
    // Light weapons (< 2 lbs)
    assert!(Item::Wakizashi.weight().amount() < 2.0);
    assert!(Item::Sai.weight().amount() < 2.0);
    assert!(Item::Katar.weight().amount() < 2.0);
    assert!(Item::Nunchaku.weight().amount() < 2.0);

    // Medium weapons (2-5 lbs)
    let katana_weight = Item::Katana.weight().amount();
    assert!(katana_weight >= 2.0 && katana_weight <= 5.0);

    // Heavy weapons (> 10 lbs)
    assert!(Item::Maul.weight().amount() >= 10.0);
}
