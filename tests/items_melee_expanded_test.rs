//! Tests for expanded melee weapon items.

use valinoreth::{Item, MeleeWeapon};

#[test]
fn test_katana_properties() {
    let katana = Item::MeleeWeapon(MeleeWeapon::Katana);

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
    let sword = Item::MeleeWeapon(MeleeWeapon::BastardSword);

    assert_eq!(sword.base_cost().amount(), 650.0);
    assert_eq!(sword.weight().amount(), 5.0);
    assert_eq!(sword.tech_level().level(), 2); // Medieval
}

#[test]
fn test_nunchaku_properties() {
    let nunchaku = Item::MeleeWeapon(MeleeWeapon::Nunchaku);

    assert_eq!(nunchaku.base_cost().amount(), 20.0);
    assert_eq!(nunchaku.weight().amount(), 1.5);
    assert_eq!(nunchaku.tech_level().level(), 2);
}

#[test]
fn test_maul_properties() {
    let maul = Item::MeleeWeapon(MeleeWeapon::Maul);

    assert_eq!(maul.base_cost().amount(), 80.0);
    assert_eq!(maul.weight().amount(), 12.0);
    assert_eq!(maul.tech_level().level(), 1); // Bronze/Iron Age
}

#[test]
fn test_all_expanded_melee_have_properties() {
    let expanded_weapons = vec![
        MeleeWeapon::Katana,
        MeleeWeapon::Scimitar,
        MeleeWeapon::Cutlass,
        MeleeWeapon::Longsword,
        MeleeWeapon::BastardSword,
        MeleeWeapon::Wakizashi,
        MeleeWeapon::Nunchaku,
        MeleeWeapon::Sai,
        MeleeWeapon::Katar,
        MeleeWeapon::Tonfa,
        MeleeWeapon::Estoc,
        MeleeWeapon::Falchion,
        MeleeWeapon::Gladius,
        MeleeWeapon::Maul,
        MeleeWeapon::Pick,
    ];

    for weapon in expanded_weapons {
        let item = Item::MeleeWeapon(weapon.clone());

        assert!(
            item.base_cost().amount() > 0.0,
            "{:?} has invalid cost",
            weapon
        );
        assert!(
            item.weight().amount() >= 0.0,
            "{:?} has invalid weight",
            weapon
        );
        assert!(item.tech_level().level() <= 12, "{:?} has invalid TL", weapon);
        assert!(item.weapon_damage().is_some(), "{:?} missing damage", weapon);
        assert!(item.reach().is_some(), "{:?} missing reach", weapon);
        assert!(item.parry_modifier().is_some(), "{:?} missing parry", weapon);
        assert!(item.required_skill().is_some(), "{:?} missing skill", weapon);
    }
}

#[test]
fn test_expanded_weapon_tech_level_range() {
    // Bronze/Iron Age (TL 1)
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::Gladius)
            .tech_level()
            .level(),
        1
    );
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::Maul)
            .tech_level()
            .level(),
        1
    );

    // Medieval (TL 2)
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::Scimitar)
            .tech_level()
            .level(),
        2
    );
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::Longsword)
            .tech_level()
            .level(),
        2
    );
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::BastardSword)
            .tech_level()
            .level(),
        2
    );
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::Nunchaku)
            .tech_level()
            .level(),
        2
    );
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::Sai)
            .tech_level()
            .level(),
        2
    );
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::Katar)
            .tech_level()
            .level(),
        2
    );
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::Falchion)
            .tech_level()
            .level(),
        2
    );
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::Pick)
            .tech_level()
            .level(),
        2
    );

    // Late Medieval/Japan (TL 3)
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::Katana)
            .tech_level()
            .level(),
        3
    );
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::Wakizashi)
            .tech_level()
            .level(),
        3
    );
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::Estoc)
            .tech_level()
            .level(),
        3
    );

    // Age of Sail (TL 4)
    assert_eq!(
        Item::MeleeWeapon(MeleeWeapon::Cutlass)
            .tech_level()
            .level(),
        4
    );
}

#[test]
fn test_expanded_weapon_weight_range() {
    // Light weapons (< 2 lbs)
    assert!(Item::MeleeWeapon(MeleeWeapon::Wakizashi)
        .weight()
        .amount()
        < 2.0);
    assert!(Item::MeleeWeapon(MeleeWeapon::Sai)
        .weight()
        .amount()
        < 2.0);
    assert!(Item::MeleeWeapon(MeleeWeapon::Katar)
        .weight()
        .amount()
        < 2.0);
    assert!(Item::MeleeWeapon(MeleeWeapon::Nunchaku)
        .weight()
        .amount()
        < 2.0);

    // Medium weapons (2-5 lbs)
    let katana_weight = Item::MeleeWeapon(MeleeWeapon::Katana).weight().amount();
    assert!(katana_weight >= 2.0 && katana_weight <= 5.0);

    // Heavy weapons (> 10 lbs)
    assert!(Item::MeleeWeapon(MeleeWeapon::Maul)
        .weight()
        .amount()
        >= 10.0);
}
