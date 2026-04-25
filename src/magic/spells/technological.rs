//! Technological college spells.
//!
//! # GURPS Rules
//!
//! Technological spells interact with machinery, electronics, and mechanical
//! devices. They allow mages to understand, control, repair, and manipulate
//! technology through magical means.
//!
//! # Citations
//!
//! M 176-185 - Technological college

use crate::{Duration, EnergyCost, ResistanceType, Spell, SpellCollege, SpellPrerequisite, SpellType};
use tracing::{debug, instrument};

/// Returns base energy cost for Technological spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Technological spell energy cost");
    match spell {
        Spell::IdentifyMachine => EnergyCost::Fixed(1),
        Spell::AnalyzeMachine => EnergyCost::Fixed(2),
        Spell::RepairMachine => EnergyCost::Fixed(3),
        Spell::JamMachine => EnergyCost::Fixed(2),
        Spell::ControlMachine => EnergyCost::Fixed(3),
        Spell::PowerMachine => EnergyCost::Fixed(3),
        Spell::DrainPower => EnergyCost::Fixed(2),
        Spell::EnhanceMachine => EnergyCost::Fixed(4),
        Spell::CreateTool => EnergyCost::Fixed(2),
        Spell::MachineThought => EnergyCost::Fixed(3),
        Spell::MachineSpeech => EnergyCost::Fixed(2),
        Spell::TechShield => EnergyCost::Fixed(3),
        Spell::Jamming => EnergyCost::Fixed(3),
        Spell::DetectSurveillance => EnergyCost::Fixed(2),
        Spell::BypassSecurity => EnergyCost::Fixed(3),
        Spell::Hologram => EnergyCost::Fixed(3),
        Spell::Cybermeld => EnergyCost::Fixed(4),
        Spell::AnimateMachine => EnergyCost::Fixed(5),
        Spell::RadiationShield => EnergyCost::Fixed(3),
        _ => panic!("Invalid spell {:?} for Technological college", spell),
    }
}

/// Returns casting time for Technological spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Technological spell casting time");
    match spell {
        Spell::IdentifyMachine => 2,
        Spell::AnalyzeMachine => 3,
        Spell::RepairMachine => 5,
        Spell::JamMachine => 2,
        Spell::ControlMachine => 2,
        Spell::PowerMachine => 2,
        Spell::DrainPower => 1,
        Spell::EnhanceMachine => 3,
        Spell::CreateTool => 4,
        Spell::MachineThought => 3,
        Spell::MachineSpeech => 2,
        Spell::TechShield => 2,
        Spell::Jamming => 2,
        Spell::DetectSurveillance => 2,
        Spell::BypassSecurity => 3,
        Spell::Hologram => 2,
        Spell::Cybermeld => 4,
        Spell::AnimateMachine => 10,
        Spell::RadiationShield => 2,
        _ => panic!("Invalid spell {:?} for Technological college", spell),
    }
}

/// Returns duration for Technological spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Technological spell duration");
    match spell {
        Spell::IdentifyMachine => Duration::Instant,
        Spell::AnalyzeMachine => Duration::Instant,
        Spell::RepairMachine => Duration::Permanent,
        Spell::JamMachine => Duration::Minutes(1),
        Spell::ControlMachine => Duration::Minutes(1),
        Spell::PowerMachine => Duration::Minutes(10),
        Spell::DrainPower => Duration::Instant,
        Spell::EnhanceMachine => Duration::Hours(1),
        Spell::CreateTool => Duration::Permanent,
        Spell::MachineThought => Duration::Minutes(1),
        Spell::MachineSpeech => Duration::Minutes(1),
        Spell::TechShield => Duration::Minutes(10),
        Spell::Jamming => Duration::Minutes(1),
        Spell::DetectSurveillance => Duration::Instant,
        Spell::BypassSecurity => Duration::Minutes(1),
        Spell::Hologram => Duration::Minutes(1),
        Spell::Cybermeld => Duration::Minutes(10),
        Spell::AnimateMachine => Duration::Hours(1),
        Spell::RadiationShield => Duration::Hours(1),
        _ => panic!("Invalid spell {:?} for Technological college", spell),
    }
}

/// Returns prerequisites for Technological spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Technological spell prerequisites");
    match spell {
        Spell::IdentifyMachine => vec![SpellPrerequisite::Magery(1)],
        Spell::AnalyzeMachine => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::IdentifyMachine),
        ],
        Spell::RepairMachine => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::AnalyzeMachine),
        ],
        Spell::JamMachine => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::IdentifyMachine),
        ],
        Spell::ControlMachine => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::AnalyzeMachine),
            SpellPrerequisite::Spell(Spell::JamMachine),
        ],
        Spell::PowerMachine => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::AnalyzeMachine),
        ],
        Spell::DrainPower => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::PowerMachine),
        ],
        Spell::EnhanceMachine => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::RepairMachine),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Technological, 4),
        ],
        Spell::CreateTool => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::IdentifyMachine),
        ],
        Spell::MachineThought => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::AnalyzeMachine),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Technological, 3),
        ],
        Spell::MachineSpeech => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::MachineThought),
        ],
        Spell::TechShield => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::IdentifyMachine),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Technological, 3),
        ],
        Spell::Jamming => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::JamMachine),
        ],
        Spell::DetectSurveillance => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::IdentifyMachine),
        ],
        Spell::BypassSecurity => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::AnalyzeMachine),
            SpellPrerequisite::Spell(Spell::ControlMachine),
        ],
        Spell::Hologram => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::AnalyzeMachine),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Technological, 4),
        ],
        Spell::Cybermeld => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::MachineThought),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Technological, 6),
        ],
        Spell::AnimateMachine => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::ControlMachine),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Technological, 8),
        ],
        Spell::RadiationShield => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::TechShield),
        ],
        _ => panic!("Invalid spell {:?} for Technological college", spell),
    }
}

/// Returns spell type for Technological spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Technological spell type");
    match spell {
        Spell::IdentifyMachine => SpellType::Information,
        Spell::AnalyzeMachine => SpellType::Information,
        Spell::RepairMachine => SpellType::Regular,
        Spell::JamMachine => SpellType::Regular,
        Spell::ControlMachine => SpellType::Regular,
        Spell::PowerMachine => SpellType::Regular,
        Spell::DrainPower => SpellType::Regular,
        Spell::EnhanceMachine => SpellType::Regular,
        Spell::CreateTool => SpellType::Regular,
        Spell::MachineThought => SpellType::Information,
        Spell::MachineSpeech => SpellType::Regular,
        Spell::TechShield => SpellType::Regular,
        Spell::Jamming => SpellType::Area,
        Spell::DetectSurveillance => SpellType::Information,
        Spell::BypassSecurity => SpellType::Regular,
        Spell::Hologram => SpellType::Regular,
        Spell::Cybermeld => SpellType::Regular,
        Spell::AnimateMachine => SpellType::Regular,
        Spell::RadiationShield => SpellType::Regular,
        _ => panic!("Invalid spell {:?} for Technological college", spell),
    }
}

/// Returns resistance type for Technological spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Technological spell resistance");
    match spell {
        Spell::IdentifyMachine => None,
        Spell::AnalyzeMachine => None,
        Spell::RepairMachine => None,
        Spell::JamMachine => None,
        Spell::ControlMachine => None,
        Spell::PowerMachine => None,
        Spell::DrainPower => None,
        Spell::EnhanceMachine => None,
        Spell::CreateTool => None,
        Spell::MachineThought => None,
        Spell::MachineSpeech => None,
        Spell::TechShield => None,
        Spell::Jamming => None,
        Spell::DetectSurveillance => None,
        Spell::BypassSecurity => None,
        Spell::Hologram => Some(ResistanceType::Will),
        Spell::Cybermeld => Some(ResistanceType::Will),
        Spell::AnimateMachine => None,
        Spell::RadiationShield => None,
        _ => panic!("Invalid spell {:?} for Technological college", spell),
    }
}

/// Returns GURPS Magic page reference for Technological spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Technological spell reference");
    match spell {
        Spell::IdentifyMachine => "M 176",
        Spell::AnalyzeMachine => "M 176",
        Spell::RepairMachine => "M 178",
        Spell::JamMachine => "M 177",
        Spell::ControlMachine => "M 177",
        Spell::PowerMachine => "M 178",
        Spell::DrainPower => "M 177",
        Spell::EnhanceMachine => "M 177",
        Spell::CreateTool => "M 176",
        Spell::MachineThought => "M 180",
        Spell::MachineSpeech => "M 176",
        Spell::TechShield => "M 179",
        Spell::Jamming => "M 177",
        Spell::DetectSurveillance => "M 177",
        Spell::BypassSecurity => "M 178",
        Spell::Hologram => "M 178",
        Spell::Cybermeld => "M 178",
        Spell::AnimateMachine => "M 176",
        Spell::RadiationShield => "M 179",
        _ => panic!("Invalid spell {:?} for Technological college", spell),
    }
}
