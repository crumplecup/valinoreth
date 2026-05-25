//! Sound college spells.
//!
//! # GURPS Rules
//!
//! Sound spells manipulate acoustic energy, create and control sounds,
//! and affect hearing. They include spells for creating noise, silencing
//! areas, amplifying sounds, and using sonic energy as a weapon.
//!
//! # Citations
//!
//! M 171-178 - Sound college

use crate::{
    Duration, EnergyCost, ResistanceType, Spell, SpellCollege, SpellPrerequisite, SpellType,
};
use tracing::{debug, instrument};

/// Returns base energy cost for Sound spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Sound spell energy cost");
    match spell {
        Spell::SilenceSound => EnergyCost::Fixed(2),
        Spell::GreatVoice => EnergyCost::Fixed(2),
        Spell::Voices => EnergyCost::Fixed(2),
        Spell::Echo => EnergyCost::Fixed(1),
        Spell::SoundVision => EnergyCost::Fixed(3),
        Spell::Garble => EnergyCost::Fixed(2),
        Spell::Roar => EnergyCost::Fixed(2),
        Spell::Screech => EnergyCost::Fixed(3),
        Spell::Hush => EnergyCost::Fixed(2),
        Spell::PerfectHearing => EnergyCost::Fixed(3),
        Spell::SonicBoom => EnergyCost::Fixed(4),
        Spell::SoundJet => EnergyCost::Fixed(3),
        Spell::SoundWall => EnergyCost::Fixed(4),
        Spell::Vibration => EnergyCost::Fixed(2),
        Spell::Cacophony => EnergyCost::Fixed(3),
        Spell::Harmony => EnergyCost::Fixed(2),
        Spell::Whisper => EnergyCost::Fixed(1),
        Spell::Soundproof => EnergyCost::Fixed(3),
        Spell::SonicWeapon => EnergyCost::Fixed(4),
        Spell::Deafen => EnergyCost::Fixed(3),
        Spell::Ventriloquism => EnergyCost::Fixed(2),
        Spell::Music => EnergyCost::Fixed(2),
        Spell::Rhythm => EnergyCost::Fixed(2),
        Spell::Resonance => EnergyCost::Fixed(3),
        Spell::SonicShield => EnergyCost::Fixed(3),
        _ => panic!("Invalid spell {:?} for Sound college", spell),
    }
}

/// Returns casting time for Sound spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Sound spell casting time");
    match spell {
        Spell::SilenceSound => 2,
        Spell::GreatVoice => 1,
        Spell::Voices => 2,
        Spell::Echo => 1,
        Spell::SoundVision => 3,
        Spell::Garble => 1,
        Spell::Roar => 1,
        Spell::Screech => 1,
        Spell::Hush => 2,
        Spell::PerfectHearing => 2,
        Spell::SonicBoom => 2,
        Spell::SoundJet => 1,
        Spell::SoundWall => 3,
        Spell::Vibration => 1,
        Spell::Cacophony => 2,
        Spell::Harmony => 2,
        Spell::Whisper => 1,
        Spell::Soundproof => 3,
        Spell::SonicWeapon => 2,
        Spell::Deafen => 2,
        Spell::Ventriloquism => 1,
        Spell::Music => 2,
        Spell::Rhythm => 1,
        Spell::Resonance => 2,
        Spell::SonicShield => 2,
        _ => panic!("Invalid spell {:?} for Sound college", spell),
    }
}

/// Returns duration for Sound spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Sound spell duration");
    match spell {
        Spell::SilenceSound => Duration::Minutes(1),
        Spell::GreatVoice => Duration::Minutes(1),
        Spell::Voices => Duration::Minutes(1),
        Spell::Echo => Duration::Minutes(1),
        Spell::SoundVision => Duration::Minutes(1),
        Spell::Garble => Duration::Minutes(1),
        Spell::Roar => Duration::Instant,
        Spell::Screech => Duration::Instant,
        Spell::Hush => Duration::Minutes(1),
        Spell::PerfectHearing => Duration::Minutes(10),
        Spell::SonicBoom => Duration::Instant,
        Spell::SoundJet => Duration::Instant,
        Spell::SoundWall => Duration::Minutes(1),
        Spell::Vibration => Duration::Minutes(1),
        Spell::Cacophony => Duration::Minutes(1),
        Spell::Harmony => Duration::Minutes(1),
        Spell::Whisper => Duration::Minutes(1),
        Spell::Soundproof => Duration::Minutes(10),
        Spell::SonicWeapon => Duration::Minutes(1),
        Spell::Deafen => Duration::Minutes(1),
        Spell::Ventriloquism => Duration::Minutes(1),
        Spell::Music => Duration::Minutes(1),
        Spell::Rhythm => Duration::Minutes(1),
        Spell::Resonance => Duration::Minutes(1),
        Spell::SonicShield => Duration::Minutes(1),
        _ => panic!("Invalid spell {:?} for Sound college", spell),
    }
}

/// Returns prerequisites for Sound spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Sound spell prerequisites");
    match spell {
        Spell::SilenceSound => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Sound),
        ],
        Spell::GreatVoice => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Sound),
        ],
        Spell::Voices => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Sound),
        ],
        Spell::Echo => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Sound),
        ],
        Spell::SoundVision => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Sound),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Sound, 4),
        ],
        Spell::Garble => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Sound),
        ],
        Spell::Roar => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::GreatVoice),
        ],
        Spell::Screech => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Thunderclap),
        ],
        Spell::Hush => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::SilenceSound),
        ],
        Spell::PerfectHearing => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Sound),
        ],
        Spell::SonicBoom => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Thunderclap),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Sound, 4),
        ],
        Spell::SoundJet => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Sound),
            SpellPrerequisite::Spell(Spell::GreatVoice),
        ],
        Spell::SoundWall => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Sound),
            SpellPrerequisite::Spell(Spell::SoundJet),
        ],
        Spell::Vibration => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Sound),
        ],
        Spell::Cacophony => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Sound),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Sound, 3),
        ],
        Spell::Harmony => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Sound),
        ],
        Spell::Whisper => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Sound),
        ],
        Spell::Soundproof => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::SilenceSound),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Sound, 3),
        ],
        Spell::SonicWeapon => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::SoundJet),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Sound, 5),
        ],
        Spell::Deafen => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Screech),
        ],
        Spell::Ventriloquism => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Voices),
        ],
        Spell::Music => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Sound),
        ],
        Spell::Rhythm => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Sound),
        ],
        Spell::Resonance => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Sound),
            SpellPrerequisite::Spell(Spell::Vibration),
        ],
        Spell::SonicShield => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::SoundWall),
        ],
        _ => panic!("Invalid spell {:?} for Sound college", spell),
    }
}

/// Returns spell type for Sound spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Sound spell type");
    match spell {
        Spell::SilenceSound => SpellType::Area,
        Spell::GreatVoice => SpellType::Regular,
        Spell::Voices => SpellType::Regular,
        Spell::Echo => SpellType::Regular,
        Spell::SoundVision => SpellType::Regular,
        Spell::Garble => SpellType::Regular,
        Spell::Roar => SpellType::Regular,
        Spell::Screech => SpellType::Regular,
        Spell::Hush => SpellType::Area,
        Spell::PerfectHearing => SpellType::Regular,
        Spell::SonicBoom => SpellType::Regular,
        Spell::SoundJet => SpellType::Regular,
        Spell::SoundWall => SpellType::Area,
        Spell::Vibration => SpellType::Regular,
        Spell::Cacophony => SpellType::Area,
        Spell::Harmony => SpellType::Area,
        Spell::Whisper => SpellType::Regular,
        Spell::Soundproof => SpellType::Area,
        Spell::SonicWeapon => SpellType::Regular,
        Spell::Deafen => SpellType::Regular,
        Spell::Ventriloquism => SpellType::Regular,
        Spell::Music => SpellType::Regular,
        Spell::Rhythm => SpellType::Regular,
        Spell::Resonance => SpellType::Regular,
        Spell::SonicShield => SpellType::Regular,
        _ => panic!("Invalid spell {:?} for Sound college", spell),
    }
}

/// Returns resistance type for Sound spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Sound spell resistance");
    match spell {
        Spell::SilenceSound => Some(ResistanceType::Will),
        Spell::GreatVoice => None,
        Spell::Voices => None,
        Spell::Echo => None,
        Spell::SoundVision => None,
        Spell::Garble => Some(ResistanceType::Will),
        Spell::Roar => Some(ResistanceType::HT),
        Spell::Screech => Some(ResistanceType::HT),
        Spell::Hush => Some(ResistanceType::Will),
        Spell::PerfectHearing => None,
        Spell::SonicBoom => Some(ResistanceType::HT),
        Spell::SoundJet => Some(ResistanceType::HT),
        Spell::SoundWall => None,
        Spell::Vibration => None,
        Spell::Cacophony => Some(ResistanceType::Will),
        Spell::Harmony => None,
        Spell::Whisper => None,
        Spell::Soundproof => None,
        Spell::SonicWeapon => Some(ResistanceType::HT),
        Spell::Deafen => Some(ResistanceType::HT),
        Spell::Ventriloquism => Some(ResistanceType::Will),
        Spell::Music => None,
        Spell::Rhythm => None,
        Spell::Resonance => None,
        Spell::SonicShield => None,
        _ => panic!("Invalid spell {:?} for Sound college", spell),
    }
}

/// Returns GURPS Magic page reference for Sound spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Sound spell reference");
    match spell {
        Spell::SilenceSound => "M 175",
        Spell::GreatVoice => "M 173",
        Spell::Voices => "M 177",
        Spell::Echo => "M 172",
        Spell::SoundVision => "M 176",
        Spell::Garble => "M 173",
        Spell::Roar => "M 175",
        Spell::Screech => "M 176",
        Spell::Hush => "M 174",
        Spell::PerfectHearing => "M 174",
        Spell::SonicBoom => "M 176",
        Spell::SoundJet => "M 176",
        Spell::SoundWall => "M 176",
        Spell::Vibration => "M 177",
        Spell::Cacophony => "M 172",
        Spell::Harmony => "M 173",
        Spell::Whisper => "M 177",
        Spell::Soundproof => "M 176",
        Spell::SonicWeapon => "M 176",
        Spell::Deafen => "M 172",
        Spell::Ventriloquism => "M 177",
        Spell::Music => "M 174",
        Spell::Rhythm => "M 175",
        Spell::Resonance => "M 175",
        Spell::SonicShield => "M 176",
        _ => panic!("Invalid spell {:?} for Sound college", spell),
    }
}
