//! Weather college spells.
//!
//! # GURPS Rules
//!
//! Weather spells control atmospheric conditions, create storms, summon
//! lightning, and manipulate temperature and precipitation. They allow
//! mages to influence large-scale weather patterns and climate.
//!
//! # Citations
//!
//! M 193-200 - Weather college

use crate::{
    Duration, EnergyCost, ResistanceType, Spell, SpellCollege, SpellPrerequisite, SpellType,
};
use tracing::{debug, instrument};

/// Returns base energy cost for Weather spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Weather spell energy cost");
    match spell {
        Spell::PredictWeather => EnergyCost::Fixed(1),
        Spell::Rain => EnergyCost::Fixed(3),
        Spell::Snow => EnergyCost::Fixed(3),
        Spell::Clouds => EnergyCost::Fixed(2),
        Spell::Wind => EnergyCost::Fixed(3),
        Spell::Storm => EnergyCost::Fixed(6),
        Spell::ClearWeather => EnergyCost::Fixed(2),
        Spell::StopRain => EnergyCost::Fixed(2),
        Spell::SummonWeatherElemental => EnergyCost::Fixed(8),
        Spell::ControlWeather => EnergyCost::Fixed(10),
        Spell::WeatherShield => EnergyCost::Fixed(3),
        Spell::ClimateControl => EnergyCost::Fixed(12),
        Spell::IceStorm => EnergyCost::Fixed(5),
        Spell::Tornado => EnergyCost::Fixed(8),
        Spell::StormSense => EnergyCost::Fixed(2),
        Spell::Aurora => EnergyCost::Fixed(2),
        Spell::WeatherProof => EnergyCost::Fixed(3),
        Spell::MicroClimate => EnergyCost::Fixed(4),
        Spell::DispelClouds => EnergyCost::Fixed(2),
        Spell::Drought => EnergyCost::Fixed(8),
        Spell::EndDrought => EnergyCost::Fixed(6),
        _ => panic!("Invalid spell {:?} for Weather college", spell),
    }
}

/// Returns casting time for Weather spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Weather spell casting time");
    match spell {
        Spell::PredictWeather => 3,
        Spell::Rain => 5,
        Spell::Snow => 5,
        Spell::Clouds => 3,
        Spell::Wind => 3,
        Spell::Storm => 10,
        Spell::ClearWeather => 3,
        Spell::StopRain => 2,
        Spell::SummonWeatherElemental => 10,
        Spell::ControlWeather => 10,
        Spell::WeatherShield => 3,
        Spell::ClimateControl => 60,
        Spell::IceStorm => 5,
        Spell::Tornado => 10,
        Spell::StormSense => 2,
        Spell::Aurora => 3,
        Spell::WeatherProof => 2,
        Spell::MicroClimate => 5,
        Spell::DispelClouds => 2,
        Spell::Drought => 60,
        Spell::EndDrought => 30,
        _ => panic!("Invalid spell {:?} for Weather college", spell),
    }
}

/// Returns duration for Weather spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Weather spell duration");
    match spell {
        Spell::PredictWeather => Duration::Instant,
        Spell::Rain => Duration::Hours(1),
        Spell::Snow => Duration::Hours(1),
        Spell::Clouds => Duration::Hours(1),
        Spell::Wind => Duration::Minutes(10),
        Spell::Storm => Duration::Hours(1),
        Spell::ClearWeather => Duration::Hours(1),
        Spell::StopRain => Duration::Instant,
        Spell::SummonWeatherElemental => Duration::Hours(1),
        Spell::ControlWeather => Duration::Concentration,
        Spell::WeatherShield => Duration::Hours(1),
        Spell::ClimateControl => Duration::Days(1),
        Spell::IceStorm => Duration::Minutes(10),
        Spell::Tornado => Duration::Minutes(10),
        Spell::StormSense => Duration::Instant,
        Spell::Aurora => Duration::Hours(1),
        Spell::WeatherProof => Duration::Hours(1),
        Spell::MicroClimate => Duration::Hours(1),
        Spell::DispelClouds => Duration::Hours(1),
        Spell::Drought => Duration::Days(30),
        Spell::EndDrought => Duration::Permanent,
        _ => panic!("Invalid spell {:?} for Weather college", spell),
    }
}

/// Returns prerequisites for Weather spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Weather spell prerequisites");
    match spell {
        Spell::PredictWeather => vec![SpellPrerequisite::Magery(1)],
        Spell::Rain => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Clouds),
        ],
        Spell::Snow => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Rain),
        ],
        Spell::Clouds => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::PredictWeather),
        ],
        Spell::Wind => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::PredictWeather),
        ],
        Spell::Storm => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Rain),
            SpellPrerequisite::Spell(Spell::Wind),
        ],
        Spell::ClearWeather => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Clouds),
        ],
        Spell::StopRain => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Rain),
        ],
        Spell::SummonWeatherElemental => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Storm),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Weather, 8),
        ],
        Spell::ControlWeather => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Storm),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Weather, 12),
        ],
        Spell::WeatherShield => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::PredictWeather),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Weather, 4),
        ],
        Spell::ClimateControl => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::Spell(Spell::ControlWeather),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Weather, 15),
        ],
        Spell::IceStorm => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Snow),
        ],
        Spell::Tornado => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Wind),
            SpellPrerequisite::Spell(Spell::Storm),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Weather, 8),
        ],
        Spell::StormSense => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::PredictWeather),
        ],
        Spell::Aurora => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Clouds),
        ],
        Spell::WeatherProof => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::WeatherShield),
        ],
        Spell::MicroClimate => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::Wind),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Weather, 6),
        ],
        Spell::DispelClouds => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::ClearWeather),
        ],
        Spell::Drought => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::StopRain),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Weather, 10),
        ],
        Spell::EndDrought => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Drought),
        ],
        _ => panic!("Invalid spell {:?} for Weather college", spell),
    }
}

/// Returns spell type for Weather spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Weather spell type");
    match spell {
        Spell::PredictWeather => SpellType::Information,
        Spell::Rain => SpellType::Area,
        Spell::Snow => SpellType::Area,
        Spell::Clouds => SpellType::Area,
        Spell::Wind => SpellType::Area,
        Spell::Storm => SpellType::Area,
        Spell::ClearWeather => SpellType::Area,
        Spell::StopRain => SpellType::Area,
        Spell::SummonWeatherElemental => SpellType::Regular,
        Spell::ControlWeather => SpellType::Area,
        Spell::WeatherShield => SpellType::Regular,
        Spell::ClimateControl => SpellType::Area,
        Spell::IceStorm => SpellType::Area,
        Spell::Tornado => SpellType::Area,
        Spell::StormSense => SpellType::Information,
        Spell::Aurora => SpellType::Area,
        Spell::WeatherProof => SpellType::Regular,
        Spell::MicroClimate => SpellType::Area,
        Spell::DispelClouds => SpellType::Area,
        Spell::Drought => SpellType::Area,
        Spell::EndDrought => SpellType::Area,
        _ => panic!("Invalid spell {:?} for Weather college", spell),
    }
}

/// Returns resistance type for Weather spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Weather spell resistance");
    match spell {
        Spell::PredictWeather => None,
        Spell::Rain => None,
        Spell::Snow => None,
        Spell::Clouds => None,
        Spell::Wind => Some(ResistanceType::HT),
        Spell::Storm => Some(ResistanceType::HT),
        Spell::ClearWeather => None,
        Spell::StopRain => None,
        Spell::SummonWeatherElemental => None,
        Spell::ControlWeather => None,
        Spell::WeatherShield => None,
        Spell::ClimateControl => None,
        Spell::IceStorm => Some(ResistanceType::HT),
        Spell::Tornado => Some(ResistanceType::HT),
        Spell::StormSense => None,
        Spell::Aurora => None,
        Spell::WeatherProof => None,
        Spell::MicroClimate => None,
        Spell::DispelClouds => None,
        Spell::Drought => None,
        Spell::EndDrought => None,
        _ => panic!("Invalid spell {:?} for Weather college", spell),
    }
}

/// Returns GURPS Magic page reference for Weather spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Weather spell reference");
    match spell {
        Spell::PredictWeather => "M 196",
        Spell::Rain => "M 197",
        Spell::Snow => "M 198",
        Spell::Clouds => "M 194",
        Spell::Wind => "M 200",
        Spell::Storm => "M 199",
        Spell::ClearWeather => "M 194",
        Spell::StopRain => "M 199",
        Spell::SummonWeatherElemental => "M 199",
        Spell::ControlWeather => "M 194",
        Spell::WeatherShield => "M 200",
        Spell::ClimateControl => "M 193",
        Spell::IceStorm => "M 195",
        Spell::Tornado => "M 199",
        Spell::StormSense => "M 197",
        Spell::Aurora => "M 193",
        Spell::WeatherProof => "M 198",
        Spell::MicroClimate => "M 195",
        Spell::DispelClouds => "M 195",
        Spell::Drought => "M 195",
        Spell::EndDrought => "M 195",
        _ => panic!("Invalid spell {:?} for Weather college", spell),
    }
}
