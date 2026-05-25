//! Mind Control college spells.
//!
//! # GURPS Rules
//!
//! Mind Control spells manipulate thoughts, emotions, memories, and will.
//! They can daze, command, read minds, plant suggestions, create telepathic
//! links, and even take over another's body. Most require Will resistance.
//!
//! # Citations
//!
//! M 126-145 - Mind Control college

use crate::{
    Duration, EnergyCost, ResistanceType, Spell, SpellCollege, SpellPrerequisite, SpellType,
};
use tracing::{debug, instrument};

/// Returns base energy cost for Mind Control spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Mind Control spell energy cost");
    match spell {
        Spell::Daze => EnergyCost::Fixed(2),
        Spell::Sleep => EnergyCost::Fixed(4),
        Spell::Command => EnergyCost::Fixed(2),
        Spell::MassDaze => EnergyCost::Fixed(3),
        Spell::Fear => EnergyCost::Fixed(3),
        Spell::Terror => EnergyCost::Fixed(6),
        Spell::Loyalty => EnergyCost::Fixed(10),
        Spell::Charm => EnergyCost::Fixed(3),
        Spell::Enslave => EnergyCost::Fixed(20),
        Spell::MindReading => EnergyCost::Fixed(4),
        Spell::MindSending => EnergyCost::Fixed(2),
        Spell::Telepathy => EnergyCost::Fixed(4),
        Spell::Forgetfulness => EnergyCost::Fixed(3),
        Spell::FalseMemory => EnergyCost::Fixed(5),
        Spell::Suggestion => EnergyCost::Fixed(3),
        Spell::CompelTruth => EnergyCost::Fixed(2),
        Spell::HideThoughts => EnergyCost::Fixed(3),
        Spell::ShieldMind => EnergyCost::Fixed(4),
        Spell::Possession => EnergyCost::Fixed(10),
        Spell::SoulJar => EnergyCost::Fixed(20),
        Spell::Berserk => EnergyCost::Fixed(4),
        Spell::Bravery => EnergyCost::Fixed(2),
        Spell::EmotionControl => EnergyCost::Fixed(3),
        Spell::SenseEmotion => EnergyCost::Fixed(2),
        Spell::Persuasion => EnergyCost::Fixed(3),
        Spell::MentalStun => EnergyCost::Fixed(3),
        Spell::MassSleep => EnergyCost::Fixed(5),
        Spell::MindWhip => EnergyCost::PerDie(2),
        _ => panic!("Invalid spell {:?} for Mind Control college", spell),
    }
}

/// Returns casting time in seconds for Mind Control spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Mind Control spell casting time");
    match spell {
        Spell::Daze => 1,
        Spell::Sleep => 2,
        Spell::Command => 1,
        Spell::MassDaze => 2,
        Spell::Fear => 1,
        Spell::Terror => 2,
        Spell::Loyalty => 5,
        Spell::Charm => 2,
        Spell::Enslave => 10,
        Spell::MindReading => 2,
        Spell::MindSending => 1,
        Spell::Telepathy => 2,
        Spell::Forgetfulness => 2,
        Spell::FalseMemory => 3,
        Spell::Suggestion => 2,
        Spell::CompelTruth => 1,
        Spell::HideThoughts => 1,
        Spell::ShieldMind => 1,
        Spell::Possession => 5,
        Spell::SoulJar => 10,
        Spell::Berserk => 2,
        Spell::Bravery => 1,
        Spell::EmotionControl => 2,
        Spell::SenseEmotion => 1,
        Spell::Persuasion => 2,
        Spell::MentalStun => 1,
        Spell::MassSleep => 3,
        Spell::MindWhip => 1,
        _ => panic!("Invalid spell {:?} for Mind Control college", spell),
    }
}

/// Returns duration for Mind Control spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Mind Control spell duration");
    match spell {
        Spell::Daze => Duration::Minutes(1),
        Spell::Sleep => Duration::Minutes(1),
        Spell::Command => Duration::Minutes(1),
        Spell::MassDaze => Duration::Minutes(1),
        Spell::Fear => Duration::Minutes(1),
        Spell::Terror => Duration::Minutes(10),
        Spell::Loyalty => Duration::Hours(1),
        Spell::Charm => Duration::Hours(1),
        Spell::Enslave => Duration::Permanent,
        Spell::MindReading => Duration::Concentration,
        Spell::MindSending => Duration::Instant,
        Spell::Telepathy => Duration::Concentration,
        Spell::Forgetfulness => Duration::Permanent,
        Spell::FalseMemory => Duration::Permanent,
        Spell::Suggestion => Duration::Permanent,
        Spell::CompelTruth => Duration::Minutes(1),
        Spell::HideThoughts => Duration::Minutes(10),
        Spell::ShieldMind => Duration::Minutes(1),
        Spell::Possession => Duration::Concentration,
        Spell::SoulJar => Duration::Permanent,
        Spell::Berserk => Duration::Minutes(1),
        Spell::Bravery => Duration::Minutes(10),
        Spell::EmotionControl => Duration::Minutes(10),
        Spell::SenseEmotion => Duration::Instant,
        Spell::Persuasion => Duration::Minutes(10),
        Spell::MentalStun => Duration::Minutes(1),
        Spell::MassSleep => Duration::Minutes(1),
        Spell::MindWhip => Duration::Instant,
        _ => panic!("Invalid spell {:?} for Mind Control college", spell),
    }
}

/// Returns prerequisites for Mind Control spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Mind Control spell prerequisites");
    match spell {
        Spell::Daze => vec![SpellPrerequisite::Magery(0)],
        Spell::Sleep => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Daze),
        ],
        Spell::Command => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Daze),
        ],
        Spell::MassDaze => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Daze),
            SpellPrerequisite::SpellsInCollege(SpellCollege::MindControl, 4),
        ],
        Spell::Fear => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Daze),
        ],
        Spell::Terror => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Fear),
        ],
        Spell::Loyalty => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::MindControl, 8),
        ],
        Spell::Charm => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::MindControl, 4),
        ],
        Spell::Enslave => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::SpellsInCollege(SpellCollege::MindControl, 12),
        ],
        Spell::MindReading => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::MindControl, 4),
        ],
        Spell::MindSending => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::MindReading),
        ],
        Spell::Telepathy => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::MindReading),
            SpellPrerequisite::Spell(Spell::MindSending),
        ],
        Spell::Forgetfulness => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::MindControl, 4),
        ],
        Spell::FalseMemory => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Forgetfulness),
        ],
        Spell::Suggestion => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::MindControl, 6),
        ],
        Spell::CompelTruth => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::MindControl, 4),
        ],
        Spell::HideThoughts => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::MindReading),
        ],
        Spell::ShieldMind => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::MindControl, 6),
        ],
        Spell::Possession => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::SpellsInCollege(SpellCollege::MindControl, 10),
        ],
        Spell::SoulJar => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::SpellsInCollege(SpellCollege::MindControl, 12),
        ],
        Spell::Berserk => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Bravery),
        ],
        Spell::Bravery => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Fear),
        ],
        Spell::EmotionControl => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::MindControl, 6),
        ],
        Spell::SenseEmotion => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::MindReading),
        ],
        Spell::Persuasion => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Charm),
        ],
        Spell::MentalStun => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Daze),
        ],
        Spell::MassSleep => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Sleep),
        ],
        Spell::MindWhip => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::MindReading),
        ],
        _ => panic!("Invalid spell {:?} for Mind Control college", spell),
    }
}

/// Returns spell type for Mind Control spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Mind Control spell type");
    match spell {
        Spell::Daze => SpellType::Regular,
        Spell::Sleep => SpellType::Regular,
        Spell::Command => SpellType::Regular,
        Spell::MassDaze => SpellType::Area,
        Spell::Fear => SpellType::Regular,
        Spell::Terror => SpellType::Regular,
        Spell::Loyalty => SpellType::Regular,
        Spell::Charm => SpellType::Regular,
        Spell::Enslave => SpellType::Regular,
        Spell::MindReading => SpellType::Information,
        Spell::MindSending => SpellType::Regular,
        Spell::Telepathy => SpellType::Regular,
        Spell::Forgetfulness => SpellType::Regular,
        Spell::FalseMemory => SpellType::Regular,
        Spell::Suggestion => SpellType::Regular,
        Spell::CompelTruth => SpellType::Regular,
        Spell::HideThoughts => SpellType::Blocking,
        Spell::ShieldMind => SpellType::Blocking,
        Spell::Possession => SpellType::Regular,
        Spell::SoulJar => SpellType::Regular,
        Spell::Berserk => SpellType::Regular,
        Spell::Bravery => SpellType::Regular,
        Spell::EmotionControl => SpellType::Regular,
        Spell::SenseEmotion => SpellType::Information,
        Spell::Persuasion => SpellType::Regular,
        Spell::MentalStun => SpellType::Regular,
        Spell::MassSleep => SpellType::Area,
        Spell::MindWhip => SpellType::Regular,
        _ => panic!("Invalid spell {:?} for Mind Control college", spell),
    }
}

/// Returns resistance type for Mind Control spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Mind Control spell resistance");
    match spell {
        Spell::Daze => Some(ResistanceType::Will),
        Spell::Sleep => Some(ResistanceType::Will),
        Spell::Command => Some(ResistanceType::Will),
        Spell::MassDaze => Some(ResistanceType::Will),
        Spell::Fear => Some(ResistanceType::Will),
        Spell::Terror => Some(ResistanceType::Will),
        Spell::Loyalty => Some(ResistanceType::Will),
        Spell::Charm => Some(ResistanceType::Will),
        Spell::Enslave => Some(ResistanceType::Will),
        Spell::MindReading => Some(ResistanceType::Will),
        Spell::MindSending => None,
        Spell::Telepathy => None,
        Spell::Forgetfulness => Some(ResistanceType::Will),
        Spell::FalseMemory => Some(ResistanceType::Will),
        Spell::Suggestion => Some(ResistanceType::Will),
        Spell::CompelTruth => Some(ResistanceType::Will),
        Spell::HideThoughts => None,
        Spell::ShieldMind => None,
        Spell::Possession => Some(ResistanceType::Will),
        Spell::SoulJar => Some(ResistanceType::Will),
        Spell::Berserk => Some(ResistanceType::Will),
        Spell::Bravery => None,
        Spell::EmotionControl => Some(ResistanceType::Will),
        Spell::SenseEmotion => Some(ResistanceType::Will),
        Spell::Persuasion => Some(ResistanceType::Will),
        Spell::MentalStun => Some(ResistanceType::Will),
        Spell::MassSleep => Some(ResistanceType::Will),
        Spell::MindWhip => Some(ResistanceType::Will),
        _ => panic!("Invalid spell {:?} for Mind Control college", spell),
    }
}

/// Returns spell reference for Mind Control spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Mind Control spell reference");
    match spell {
        Spell::Daze => "M122",
        Spell::Sleep => "M139",
        Spell::Command => "M121",
        Spell::MassDaze => "M130",
        Spell::Fear => "M124",
        Spell::Terror => "M141",
        Spell::Loyalty => "M130",
        Spell::Charm => "M121",
        Spell::Enslave => "M123",
        Spell::MindReading => "M132",
        Spell::MindSending => "M133",
        Spell::Telepathy => "M141",
        Spell::Forgetfulness => "M125",
        Spell::FalseMemory => "M124",
        Spell::Suggestion => "M140",
        Spell::CompelTruth => "M121",
        Spell::HideThoughts => "M127",
        Spell::ShieldMind => "M138",
        Spell::Possession => "M136",
        Spell::SoulJar => "M140",
        Spell::Berserk => "M119",
        Spell::Bravery => "M120",
        Spell::EmotionControl => "M123",
        Spell::SenseEmotion => "M134",
        Spell::Persuasion => "M135",
        Spell::MentalStun => "M131",
        Spell::MassSleep => "M130",
        Spell::MindWhip => "M132",
        _ => panic!("Invalid spell {:?} for Mind Control college", spell),
    }
}
