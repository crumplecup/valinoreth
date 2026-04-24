//! Tests for GURPS magic spells (12 colleges implemented, 24 total).

use valinoreth::{
    Duration, EnergyCost, ResistanceType, Spell, SpellCollege, SpellPrerequisite, SpellType,
};

// ========== College Tests ==========

#[test]
fn test_all_spells_have_colleges() {
    // All spells should have valid colleges
    use strum::IntoEnumIterator;

    for spell in Spell::iter() {
        let college = spell.college();
        assert!(
            college == SpellCollege::Air
                || college == SpellCollege::Animal
                || college == SpellCollege::BodyControl
                || college == SpellCollege::CommunicationEmpathy
                || college == SpellCollege::Earth
                || college == SpellCollege::Fire
                || college == SpellCollege::Healing
                || college == SpellCollege::IllusionCreation
                || college == SpellCollege::Knowledge
                || college == SpellCollege::MindControl
                || college == SpellCollege::Movement
                || college == SpellCollege::ProtectionWarning
                || college == SpellCollege::Water,
            "Spell {:?} has unexpected college: {:?}",
            spell,
            college
        );
    }
}

// ========== Basic Spell Tests ==========

#[test]
fn test_detect_magic_stats() {
    let spell = Spell::DetectMagic;

    assert_eq!(spell.college(), SpellCollege::Knowledge);
    assert_eq!(spell.base_energy_cost(), EnergyCost::Fixed(2));
    assert_eq!(spell.casting_time(), 1);
    assert_eq!(spell.duration(), Duration::Concentration);
    assert_eq!(spell.spell_type(), SpellType::Information);
    assert_eq!(spell.resistance(), None);
    assert_eq!(spell.reference(), "M107");
}

#[test]
fn test_detect_magic_prerequisites() {
    let prereqs = Spell::DetectMagic.prerequisites();

    assert_eq!(prereqs.len(), 1);
    assert_eq!(prereqs[0], SpellPrerequisite::Magery(0));
}

#[test]
fn test_seeker_stats() {
    let spell = Spell::Seeker;

    assert_eq!(spell.college(), SpellCollege::Knowledge);
    assert_eq!(spell.base_energy_cost(), EnergyCost::Fixed(3));
    assert_eq!(spell.casting_time(), 2);
    assert_eq!(spell.duration(), Duration::Instant);
    assert_eq!(spell.spell_type(), SpellType::Information);
    assert_eq!(spell.resistance(), None);
    assert_eq!(spell.reference(), "M112");
}

#[test]
fn test_seeker_prerequisites() {
    let prereqs = Spell::Seeker.prerequisites();

    assert_eq!(prereqs.len(), 2);
    assert!(prereqs.contains(&SpellPrerequisite::Magery(1)));
    assert!(prereqs.contains(&SpellPrerequisite::Spell(Spell::DetectMagic)));
}

#[test]
fn test_wisdom_stats() {
    let spell = Spell::Wisdom;

    assert_eq!(spell.college(), SpellCollege::Knowledge);
    assert_eq!(spell.base_energy_cost(), EnergyCost::Fixed(8));
    assert_eq!(spell.casting_time(), 5);
    assert_eq!(spell.duration(), Duration::Instant);
    assert_eq!(spell.spell_type(), SpellType::Information);
    assert_eq!(spell.resistance(), None);
    assert_eq!(spell.reference(), "M113");
}

#[test]
fn test_wisdom_prerequisites() {
    let prereqs = Spell::Wisdom.prerequisites();

    assert_eq!(prereqs.len(), 3);
    assert!(prereqs.contains(&SpellPrerequisite::Magery(2)));
    assert!(prereqs.contains(&SpellPrerequisite::SpellsInCollege(
        SpellCollege::Knowledge,
        12
    )));
    assert!(prereqs.contains(&SpellPrerequisite::IQ(14)));
}

// ========== Seek Spells Tests ==========

#[test]
fn test_seek_air_stats() {
    let spell = Spell::SeekAir;

    assert_eq!(spell.base_energy_cost(), EnergyCost::Fixed(1));
    assert_eq!(spell.casting_time(), 1);
    assert_eq!(spell.duration(), Duration::Concentration);
    assert_eq!(spell.spell_type(), SpellType::Information);
}

#[test]
fn test_seek_earth_stats() {
    let spell = Spell::SeekEarth;

    assert_eq!(spell.base_energy_cost(), EnergyCost::Fixed(1));
    assert_eq!(spell.casting_time(), 1);
    assert_eq!(spell.duration(), Duration::Concentration);
}

#[test]
fn test_seek_fire_stats() {
    let spell = Spell::SeekFire;

    assert_eq!(spell.base_energy_cost(), EnergyCost::Fixed(1));
    assert_eq!(spell.casting_time(), 1);
    assert_eq!(spell.duration(), Duration::Concentration);
}

#[test]
fn test_seek_water_stats() {
    let spell = Spell::SeekWater;

    assert_eq!(spell.base_energy_cost(), EnergyCost::Fixed(1));
    assert_eq!(spell.casting_time(), 1);
    assert_eq!(spell.duration(), Duration::Concentration);
}

#[test]
fn test_seek_plant_prerequisites() {
    let prereqs = Spell::SeekPlant.prerequisites();

    assert_eq!(prereqs.len(), 2);
    assert!(prereqs.contains(&SpellPrerequisite::Magery(0)));
    assert!(prereqs.contains(&SpellPrerequisite::Spell(Spell::SeekEarth)));
}

#[test]
fn test_seek_food_prerequisites() {
    let prereqs = Spell::SeekFood.prerequisites();

    assert_eq!(prereqs.len(), 2);
    assert!(prereqs.contains(&SpellPrerequisite::Magery(0)));
    assert!(prereqs.contains(&SpellPrerequisite::Spell(Spell::SeekWater)));
}

// ========== Analysis Spells Tests ==========

#[test]
fn test_analyze_magic_stats() {
    let spell = Spell::AnalyzeMagic;

    assert_eq!(spell.base_energy_cost(), EnergyCost::Fixed(8));
    assert_eq!(spell.casting_time(), 1);
    assert_eq!(spell.duration(), Duration::Instant);
    assert_eq!(spell.spell_type(), SpellType::Information);
    assert_eq!(spell.reference(), "M106");
}

#[test]
fn test_analyze_magic_prerequisites() {
    let prereqs = Spell::AnalyzeMagic.prerequisites();

    assert_eq!(prereqs.len(), 2);
    assert!(prereqs.contains(&SpellPrerequisite::Magery(1)));
    assert!(prereqs.contains(&SpellPrerequisite::SpellsInCollege(
        SpellCollege::Knowledge,
        6
    )));
}

#[test]
fn test_identify_spell_stats() {
    let spell = Spell::IdentifySpell;

    assert_eq!(spell.base_energy_cost(), EnergyCost::Fixed(2));
    assert_eq!(spell.casting_time(), 1);
    assert_eq!(spell.duration(), Duration::Instant);
    assert_eq!(spell.reference(), "M109");
}

#[test]
fn test_identify_spell_prerequisites() {
    let prereqs = Spell::IdentifySpell.prerequisites();

    assert_eq!(prereqs.len(), 2);
    assert!(prereqs.contains(&SpellPrerequisite::Magery(1)));
    assert!(prereqs.contains(&SpellPrerequisite::Spell(Spell::DetectMagic)));
}

// ========== History Spells Tests ==========

#[test]
fn test_history_stats() {
    let spell = Spell::History;

    assert_eq!(spell.base_energy_cost(), EnergyCost::Fixed(4));
    assert_eq!(spell.casting_time(), 3);
    assert_eq!(spell.duration(), Duration::Instant);
    assert_eq!(spell.spell_type(), SpellType::Information);
    assert_eq!(spell.reference(), "M108");
}

#[test]
fn test_history_prerequisites() {
    let prereqs = Spell::History.prerequisites();

    assert_eq!(prereqs.len(), 2);
    assert!(prereqs.contains(&SpellPrerequisite::Magery(1)));
    assert!(prereqs.contains(&SpellPrerequisite::SpellsInCollege(
        SpellCollege::Knowledge,
        4
    )));
}

#[test]
fn test_ancient_history_stats() {
    let spell = Spell::AncientHistory;

    assert_eq!(spell.base_energy_cost(), EnergyCost::Fixed(5));
    assert_eq!(spell.casting_time(), 4);
    assert_eq!(spell.duration(), Duration::Instant);
    assert_eq!(spell.reference(), "M106");
}

#[test]
fn test_ancient_history_prerequisites() {
    let prereqs = Spell::AncientHistory.prerequisites();

    assert_eq!(prereqs.len(), 2);
    assert!(prereqs.contains(&SpellPrerequisite::Magery(1)));
    assert!(prereqs.contains(&SpellPrerequisite::Spell(Spell::History)));
}

// ========== Divination Spells Tests ==========

#[test]
fn test_divination_stats() {
    let spell = Spell::Divination;

    assert_eq!(spell.base_energy_cost(), EnergyCost::Fixed(4));
    assert_eq!(spell.casting_time(), 5);
    assert_eq!(spell.duration(), Duration::Instant);
    assert_eq!(spell.spell_type(), SpellType::Information);
    assert_eq!(spell.reference(), "M107");
}

#[test]
fn test_divination_prerequisites() {
    let prereqs = Spell::Divination.prerequisites();

    assert_eq!(prereqs.len(), 2);
    assert!(prereqs.contains(&SpellPrerequisite::Magery(1)));
    assert!(prereqs.contains(&SpellPrerequisite::SpellsInCollege(
        SpellCollege::Knowledge,
        6
    )));
}

// ========== Utility Spells Tests ==========

#[test]
fn test_pathfinder_stats() {
    let spell = Spell::Pathfinder;

    assert_eq!(spell.base_energy_cost(), EnergyCost::Fixed(3));
    assert_eq!(spell.casting_time(), 2);
    assert_eq!(spell.duration(), Duration::Instant);
    assert_eq!(spell.spell_type(), SpellType::Information);
    assert_eq!(spell.reference(), "M110");
}

#[test]
fn test_pathfinder_prerequisites() {
    let prereqs = Spell::Pathfinder.prerequisites();

    assert_eq!(prereqs.len(), 2);
    assert!(prereqs.contains(&SpellPrerequisite::Magery(1)));
    assert!(prereqs.contains(&SpellPrerequisite::Spell(Spell::Seeker)));
}

#[test]
fn test_glass_wall_stats() {
    let spell = Spell::GlassWall;

    assert_eq!(spell.base_energy_cost(), EnergyCost::Fixed(2));
    assert_eq!(spell.casting_time(), 1);
    assert_eq!(spell.duration(), Duration::Minutes(1));
    assert_eq!(spell.spell_type(), SpellType::Area);
    assert_eq!(spell.reference(), "M108");
}

#[test]
fn test_trace_stats() {
    let spell = Spell::Trace;

    assert_eq!(spell.base_energy_cost(), EnergyCost::Fixed(2));
    assert_eq!(spell.casting_time(), 2);
    assert_eq!(spell.duration(), Duration::Concentration);
    assert_eq!(spell.spell_type(), SpellType::Information);
    assert_eq!(spell.reference(), "M113");
}

#[test]
fn test_aura_stats() {
    let spell = Spell::Aura;

    assert_eq!(spell.base_energy_cost(), EnergyCost::Fixed(2));
    assert_eq!(spell.casting_time(), 2);
    assert_eq!(spell.duration(), Duration::Instant);
    assert_eq!(spell.spell_type(), SpellType::Information);
    assert_eq!(spell.reference(), "M107");
}

// ========== Memory Spells Tests ==========

#[test]
fn test_recover_memory_stats() {
    let spell = Spell::RecoverMemory;

    assert_eq!(spell.base_energy_cost(), EnergyCost::Fixed(3));
    assert_eq!(spell.casting_time(), 2);
    assert_eq!(spell.duration(), Duration::Instant);
    assert_eq!(spell.spell_type(), SpellType::Regular);
    assert_eq!(spell.resistance(), Some(ResistanceType::IQ));
    assert_eq!(spell.reference(), "M110");
}

#[test]
fn test_recover_memory_prerequisites() {
    let prereqs = Spell::RecoverMemory.prerequisites();

    assert_eq!(prereqs.len(), 2);
    assert!(prereqs.contains(&SpellPrerequisite::Magery(1)));
    assert!(prereqs.contains(&SpellPrerequisite::SpellsInCollege(
        SpellCollege::Knowledge,
        4
    )));
}

// ========== Duration Tests ==========

#[test]
fn test_instant_duration_spells() {
    let instant_spells = vec![
        Spell::AnalyzeMagic,
        Spell::IdentifySpell,
        Spell::Seeker,
        Spell::History,
        Spell::Wisdom,
        Spell::AncientHistory,
        Spell::RecoverMemory,
        Spell::Divination,
        Spell::Pathfinder,
        Spell::Aura,
    ];

    for spell in instant_spells {
        assert_eq!(spell.duration(), Duration::Instant);
    }
}

#[test]
fn test_concentration_duration_spells() {
    let concentration_spells = vec![
        Spell::DetectMagic,
        Spell::SeekAir,
        Spell::SeekEarth,
        Spell::SeekFire,
        Spell::SeekWater,
        Spell::SeekPlant,
        Spell::SeekFood,
        Spell::SeekMachine,
        Spell::Trace,
    ];

    for spell in concentration_spells {
        assert_eq!(spell.duration(), Duration::Concentration);
    }
}

// ========== Energy Cost Tests ==========

#[test]
fn test_low_cost_spells() {
    // Seek spells are cheapest at 1 FP
    let seek_spells = vec![
        Spell::SeekAir,
        Spell::SeekEarth,
        Spell::SeekFire,
        Spell::SeekWater,
        Spell::SeekPlant,
        Spell::SeekFood,
        Spell::SeekMachine,
    ];

    for spell in seek_spells {
        assert_eq!(spell.base_energy_cost(), EnergyCost::Fixed(1));
    }
}

#[test]
fn test_high_cost_spells() {
    // Most expensive spells in Knowledge college
    assert_eq!(Spell::AnalyzeMagic.base_energy_cost(), EnergyCost::Fixed(8));
    assert_eq!(Spell::Wisdom.base_energy_cost(), EnergyCost::Fixed(8));
}

// ========== Prerequisite Tests ==========

#[test]
fn test_no_magery_required_spells() {
    // Only Seek spells don't require Magery 1+
    let no_magery_spells = vec![
        Spell::DetectMagic,
        Spell::SeekAir,
        Spell::SeekEarth,
        Spell::SeekFire,
        Spell::SeekWater,
        Spell::SeekPlant,
        Spell::SeekFood,
        Spell::SeekMachine,
    ];

    for spell in no_magery_spells {
        let prereqs = spell.prerequisites();
        // Either Magery(0) or doesn't require Magery at all
        if let Some(SpellPrerequisite::Magery(level)) = prereqs.first() {
            assert_eq!(*level, 0);
        }
    }
}

#[test]
fn test_magery_2_required() {
    // Wisdom is the only spell requiring Magery 2
    let prereqs = Spell::Wisdom.prerequisites();
    assert!(prereqs.contains(&SpellPrerequisite::Magery(2)));
}

#[test]
fn test_college_count_prerequisites() {
    // Spells that require knowing N other spells in college
    assert!(Spell::AnalyzeMagic
        .prerequisites()
        .contains(&SpellPrerequisite::SpellsInCollege(
            SpellCollege::Knowledge,
            6
        )));

    assert!(Spell::History
        .prerequisites()
        .contains(&SpellPrerequisite::SpellsInCollege(
            SpellCollege::Knowledge,
            4
        )));

    assert!(Spell::Wisdom
        .prerequisites()
        .contains(&SpellPrerequisite::SpellsInCollege(
            SpellCollege::Knowledge,
            12
        )));
}

// ========== Spell Type Tests ==========

#[test]
fn test_information_spell_type() {
    // Most Knowledge spells are Information type
    let info_spells = vec![
        Spell::DetectMagic,
        Spell::AnalyzeMagic,
        Spell::IdentifySpell,
        Spell::Seeker,
        Spell::History,
        Spell::Wisdom,
        Spell::Aura,
    ];

    for spell in info_spells {
        assert_eq!(spell.spell_type(), SpellType::Information);
    }
}

#[test]
fn test_area_spell_type() {
    // Glass Wall is Area type
    assert_eq!(Spell::GlassWall.spell_type(), SpellType::Area);
}

#[test]
fn test_regular_spell_type() {
    // Recover Memory is Regular type (affects target)
    assert_eq!(Spell::RecoverMemory.spell_type(), SpellType::Regular);
}

// ========== Reference Tests ==========

#[test]
fn test_all_spells_have_references() {
    use strum::IntoEnumIterator;

    for spell in Spell::iter() {
        let reference = spell.reference();
        assert!(reference.starts_with('M'));
        assert!(reference.len() >= 3); // M + page number
    }
}

// ========== Integration Tests ==========

#[test]
fn test_spell_prerequisite_chain() {
    // Test a chain: DetectMagic -> Seeker -> Trace
    let detect = Spell::DetectMagic;
    let seeker = Spell::Seeker;
    let trace = Spell::Trace;

    // DetectMagic requires only Magery 0
    assert_eq!(detect.prerequisites().len(), 1);

    // Seeker requires DetectMagic
    assert!(seeker
        .prerequisites()
        .contains(&SpellPrerequisite::Spell(Spell::DetectMagic)));

    // Trace requires Seeker
    assert!(trace
        .prerequisites()
        .contains(&SpellPrerequisite::Spell(Spell::Seeker)));
}

#[test]
fn test_spell_count() {
    use strum::IntoEnumIterator;

    let count = Spell::iter().count();
    assert_eq!(count, 314); // 13 colleges: Air (23) + Animal (22) + Body Control (29) + Communication & Empathy (16) + Earth (24) + Fire (25) + Healing (25) + Illusion (23) + Knowledge (26) + Mind Control (25) + Movement (20) + Protection (27) + Water (29)
}
