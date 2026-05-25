//! Communication & Empathy college spells.
//!
//! # GURPS Rules
//!
//! Communication & Empathy spells enable mental communication, language
//! translation, emotional sensing, and social interaction enhancement.
//!
//! # Citations
//!
//! M 48-55 - Communication & Empathy college

use crate::{
    Duration, EnergyCost, ResistanceType, Spell, SpellCollege, SpellPrerequisite, SpellType,
};
use tracing::{debug, instrument};

/// Returns base energy cost for Communication & Empathy spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Communication & Empathy spell energy cost");
    match spell {
        Spell::SenseEmotion => EnergyCost::Fixed(2),
        Spell::Persuasion => EnergyCost::Fixed(3),
        Spell::EmotionControl => EnergyCost::Fixed(3),
        Spell::Truthsayer => EnergyCost::Fixed(2),
        Spell::SenseLife => EnergyCost::Fixed(2),
        Spell::SenseFoes => EnergyCost::Fixed(2),
        Spell::LendLanguage => EnergyCost::Fixed(2),
        Spell::BorrowLanguage => EnergyCost::Fixed(2),
        Spell::GiftOfTongues => EnergyCost::Fixed(4),
        Spell::GiftOfLetters => EnergyCost::Fixed(3),
        Spell::Comprehend => EnergyCost::Fixed(2),
        Spell::Translate => EnergyCost::Fixed(3),
        Spell::PermanentTranslation => EnergyCost::Fixed(5),
        Spell::ProjectVoice => EnergyCost::Fixed(1),
        Spell::SilentCommunication => EnergyCost::Fixed(2),
        Spell::LinkMind => EnergyCost::Fixed(4),
        _ => panic!(
            "Invalid spell {:?} for Communication & Empathy college",
            spell
        ),
    }
}

/// Returns casting time in seconds for Communication & Empathy spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Communication & Empathy spell casting time");
    match spell {
        Spell::SenseEmotion => 1,
        Spell::Persuasion => 2,
        Spell::EmotionControl => 2,
        Spell::Truthsayer => 1,
        Spell::SenseLife => 1,
        Spell::SenseFoes => 1,
        Spell::LendLanguage => 2,
        Spell::BorrowLanguage => 2,
        Spell::GiftOfTongues => 3,
        Spell::GiftOfLetters => 2,
        Spell::Comprehend => 2,
        Spell::Translate => 2,
        Spell::PermanentTranslation => 4,
        Spell::ProjectVoice => 1,
        Spell::SilentCommunication => 1,
        Spell::LinkMind => 3,
        _ => panic!(
            "Invalid spell {:?} for Communication & Empathy college",
            spell
        ),
    }
}

/// Returns duration for Communication & Empathy spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Communication & Empathy spell duration");
    match spell {
        Spell::SenseEmotion => Duration::Instant,
        Spell::Persuasion => Duration::Minutes(10),
        Spell::EmotionControl => Duration::Minutes(10),
        Spell::Truthsayer => Duration::Concentration,
        Spell::SenseLife => Duration::Concentration,
        Spell::SenseFoes => Duration::Concentration,
        Spell::LendLanguage => Duration::Hours(1),
        Spell::BorrowLanguage => Duration::Hours(1),
        Spell::GiftOfTongues => Duration::Permanent,
        Spell::GiftOfLetters => Duration::Permanent,
        Spell::Comprehend => Duration::Minutes(1),
        Spell::Translate => Duration::Concentration,
        Spell::PermanentTranslation => Duration::Permanent,
        Spell::ProjectVoice => Duration::Minutes(1),
        Spell::SilentCommunication => Duration::Minutes(1),
        Spell::LinkMind => Duration::Minutes(10),
        _ => panic!(
            "Invalid spell {:?} for Communication & Empathy college",
            spell
        ),
    }
}

/// Returns prerequisites for Communication & Empathy spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Communication & Empathy spell prerequisites");
    match spell {
        Spell::SenseEmotion => vec![SpellPrerequisite::Magery(1)],
        Spell::Persuasion => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::SenseEmotion),
        ],
        Spell::EmotionControl => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::CommunicationEmpathy, 6),
        ],
        Spell::Truthsayer => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::SenseEmotion),
        ],
        Spell::SenseLife => vec![SpellPrerequisite::Magery(1)],
        Spell::SenseFoes => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::SenseLife),
        ],
        Spell::LendLanguage => vec![SpellPrerequisite::Magery(1)],
        Spell::BorrowLanguage => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::LendLanguage),
        ],
        Spell::GiftOfTongues => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::LendLanguage),
        ],
        Spell::GiftOfLetters => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::GiftOfTongues),
        ],
        Spell::Comprehend => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::BorrowLanguage),
        ],
        Spell::Translate => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::LendLanguage),
        ],
        Spell::PermanentTranslation => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Translate),
        ],
        Spell::ProjectVoice => vec![SpellPrerequisite::Magery(0)],
        Spell::SilentCommunication => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::ProjectVoice),
        ],
        Spell::LinkMind => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::CommunicationEmpathy, 6),
        ],
        _ => panic!(
            "Invalid spell {:?} for Communication & Empathy college",
            spell
        ),
    }
}

/// Returns spell type for Communication & Empathy spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Communication & Empathy spell type");
    match spell {
        Spell::SenseEmotion => SpellType::Information,
        Spell::Persuasion => SpellType::Regular,
        Spell::EmotionControl => SpellType::Regular,
        Spell::Truthsayer => SpellType::Information,
        Spell::SenseLife => SpellType::Information,
        Spell::SenseFoes => SpellType::Information,
        Spell::LendLanguage => SpellType::Regular,
        Spell::BorrowLanguage => SpellType::Regular,
        Spell::GiftOfTongues => SpellType::Regular,
        Spell::GiftOfLetters => SpellType::Regular,
        Spell::Comprehend => SpellType::Regular,
        Spell::Translate => SpellType::Regular,
        Spell::PermanentTranslation => SpellType::Regular,
        Spell::ProjectVoice => SpellType::Regular,
        Spell::SilentCommunication => SpellType::Regular,
        Spell::LinkMind => SpellType::Regular,
        _ => panic!(
            "Invalid spell {:?} for Communication & Empathy college",
            spell
        ),
    }
}

/// Returns resistance type for Communication & Empathy spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Communication & Empathy spell resistance");
    match spell {
        Spell::SenseEmotion => Some(ResistanceType::Will),
        Spell::Persuasion => Some(ResistanceType::Will),
        Spell::EmotionControl => Some(ResistanceType::Will),
        Spell::Truthsayer => None,
        Spell::SenseLife => None,
        Spell::SenseFoes => None,
        Spell::LendLanguage => None,
        Spell::BorrowLanguage => None,
        Spell::GiftOfTongues => None,
        Spell::GiftOfLetters => None,
        Spell::Comprehend => None,
        Spell::Translate => None,
        Spell::PermanentTranslation => None,
        Spell::ProjectVoice => None,
        Spell::SilentCommunication => None,
        Spell::LinkMind => None,
        _ => panic!(
            "Invalid spell {:?} for Communication & Empathy college",
            spell
        ),
    }
}

/// Returns page reference for Communication & Empathy spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Communication & Empathy spell reference");
    match spell {
        Spell::SenseEmotion => "M52",
        Spell::Persuasion => "M51",
        Spell::EmotionControl => "M50",
        Spell::Truthsayer => "M54",
        Spell::SenseLife => "M52",
        Spell::SenseFoes => "M52",
        Spell::LendLanguage => "M50",
        Spell::BorrowLanguage => "M48",
        Spell::GiftOfTongues => "M49",
        Spell::GiftOfLetters => "M49",
        Spell::Comprehend => "M48",
        Spell::Translate => "M54",
        Spell::PermanentTranslation => "M51",
        Spell::ProjectVoice => "M51",
        Spell::SilentCommunication => "M53",
        Spell::LinkMind => "M50",
        _ => panic!(
            "Invalid spell {:?} for Communication & Empathy college",
            spell
        ),
    }
}
