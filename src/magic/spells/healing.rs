//! Healing college spells.
//!
//! # GURPS Rules
//!
//! Healing spells restore health, cure ailments, and manipulate life energy. They
//! heal wounds, cure diseases, neutralize poisons, and remove curses.
//!
//! # Citations
//!
//! M 76-87 - Healing college

use crate::{
    Duration, EnergyCost, ResistanceType, Spell, SpellCollege, SpellPrerequisite, SpellType,
};
use tracing::{debug, instrument};

/// Returns base energy cost for Healing spells.
#[instrument]
pub(super) fn base_energy_cost(spell: &Spell) -> EnergyCost {
    debug!("Getting Healing spell energy cost");
    match spell {
        Spell::LendEnergy => EnergyCost::PerFP(1),
        Spell::LendVitality => EnergyCost::PerHP(1),
        Spell::RecoverEnergy => EnergyCost::PerFP(2),
        Spell::MinorHealing => EnergyCost::Fixed(1),
        Spell::MajorHealing => EnergyCost::PerHP(2),
        Spell::GreatHealing => EnergyCost::PerHP(4),
        Spell::Awaken => EnergyCost::Fixed(1),
        Spell::CureDisease => EnergyCost::Fixed(4),
        Spell::NeutralizePoison => EnergyCost::Fixed(3),
        Spell::InstantNeutralizePoison => EnergyCost::Fixed(6),
        Spell::StopBleeding => EnergyCost::Fixed(2),
        Spell::Regeneration => EnergyCost::PerHP(1),
        Spell::Restoration => EnergyCost::Fixed(20),
        Spell::SuspendCurse => EnergyCost::Fixed(3),
        Spell::RemoveCurse => EnergyCost::Fixed(10),
        Spell::CurseRemoval => EnergyCost::Fixed(5),
        Spell::Healing => EnergyCost::PerHP(2),
        Spell::RestoreYouth => EnergyCost::Fixed(100),
        Spell::Resurrection => EnergyCost::Fixed(300),
        Spell::Sterilize => EnergyCost::Fixed(1),
        Spell::ShareVitality => EnergyCost::PerHP(1),
        Spell::RegrowLimb => EnergyCost::Fixed(50),
        Spell::PurifyFood => EnergyCost::Fixed(2),
        Spell::InstantRecoverEnergy => EnergyCost::PerFP(3),
        Spell::StopAging => EnergyCost::Fixed(20),
        _ => panic!("Invalid spell {:?} for Healing college", spell),
    }
}

/// Returns casting time in seconds for Healing spells.
#[instrument]
pub(super) fn casting_time(spell: &Spell) -> i32 {
    debug!("Getting Healing spell casting time");
    match spell {
        Spell::LendEnergy => 1,
        Spell::LendVitality => 1,
        Spell::RecoverEnergy => 1,
        Spell::MinorHealing => 1,
        Spell::MajorHealing => 2,
        Spell::GreatHealing => 1,
        Spell::Awaken => 1,
        Spell::CureDisease => 1,
        Spell::NeutralizePoison => 1,
        Spell::InstantNeutralizePoison => 1,
        Spell::StopBleeding => 1,
        Spell::Regeneration => 2,
        Spell::Restoration => 10,
        Spell::SuspendCurse => 1,
        Spell::RemoveCurse => 5,
        Spell::CurseRemoval => 2,
        Spell::Healing => 2,
        Spell::RestoreYouth => 60,
        Spell::Resurrection => 30,
        Spell::Sterilize => 1,
        Spell::ShareVitality => 2,
        Spell::RegrowLimb => 10,
        Spell::PurifyFood => 1,
        Spell::InstantRecoverEnergy => 1,
        Spell::StopAging => 10,
        _ => panic!("Invalid spell {:?} for Healing college", spell),
    }
}

/// Returns duration for Healing spells.
#[instrument]
pub(super) fn duration(spell: &Spell) -> Duration {
    debug!("Getting Healing spell duration");
    match spell {
        Spell::LendEnergy => Duration::Concentration,
        Spell::LendVitality => Duration::Concentration,
        Spell::RecoverEnergy => Duration::Instant,
        Spell::MinorHealing => Duration::Instant,
        Spell::MajorHealing => Duration::Instant,
        Spell::GreatHealing => Duration::Instant,
        Spell::Awaken => Duration::Instant,
        Spell::CureDisease => Duration::Instant,
        Spell::NeutralizePoison => Duration::Instant,
        Spell::InstantNeutralizePoison => Duration::Instant,
        Spell::StopBleeding => Duration::Instant,
        Spell::Regeneration => Duration::Minutes(1),
        Spell::Restoration => Duration::Permanent,
        Spell::SuspendCurse => Duration::Minutes(10),
        Spell::RemoveCurse => Duration::Permanent,
        Spell::CurseRemoval => Duration::Instant,
        Spell::Healing => Duration::Instant,
        Spell::RestoreYouth => Duration::Permanent,
        Spell::Resurrection => Duration::Permanent,
        Spell::Sterilize => Duration::Hours(1),
        Spell::ShareVitality => Duration::Instant,
        Spell::RegrowLimb => Duration::Permanent,
        Spell::PurifyFood => Duration::Instant,
        Spell::InstantRecoverEnergy => Duration::Instant,
        Spell::StopAging => Duration::Days(30),
        _ => panic!("Invalid spell {:?} for Healing college", spell),
    }
}

/// Returns prerequisites for Healing spells.
#[instrument]
pub(super) fn prerequisites(spell: &Spell) -> Vec<SpellPrerequisite> {
    debug!("Getting Healing spell prerequisites");
    match spell {
        Spell::LendEnergy => vec![SpellPrerequisite::Magery(0)],
        Spell::LendVitality => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::LendEnergy),
        ],
        Spell::RecoverEnergy => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::LendEnergy),
        ],
        Spell::MinorHealing => vec![SpellPrerequisite::Magery(0)],
        Spell::MajorHealing => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::MinorHealing),
        ],
        Spell::GreatHealing => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::MajorHealing),
        ],
        Spell::Awaken => vec![SpellPrerequisite::Magery(0)],
        Spell::CureDisease => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Healing, 4),
        ],
        Spell::NeutralizePoison => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::MinorHealing),
        ],
        Spell::InstantNeutralizePoison => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::NeutralizePoison),
        ],
        Spell::StopBleeding => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::MinorHealing),
        ],
        Spell::Regeneration => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::MajorHealing),
        ],
        Spell::Restoration => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Healing, 8),
        ],
        Spell::SuspendCurse => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Healing, 4),
        ],
        Spell::RemoveCurse => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::SuspendCurse),
        ],
        Spell::CurseRemoval => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Healing, 4),
        ],
        Spell::Healing => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::MajorHealing),
        ],
        Spell::RestoreYouth => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Healing, 12),
        ],
        Spell::Resurrection => vec![
            SpellPrerequisite::Magery(3),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Healing, 12),
        ],
        Spell::Sterilize => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::MinorHealing),
        ],
        Spell::ShareVitality => vec![
            SpellPrerequisite::Magery(1),
            SpellPrerequisite::Spell(Spell::LendVitality),
        ],
        Spell::RegrowLimb => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::Regeneration),
        ],
        Spell::PurifyFood => vec![
            SpellPrerequisite::Magery(0),
            SpellPrerequisite::Spell(Spell::NeutralizePoison),
        ],
        Spell::InstantRecoverEnergy => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::Spell(Spell::RecoverEnergy),
        ],
        Spell::StopAging => vec![
            SpellPrerequisite::Magery(2),
            SpellPrerequisite::SpellsInCollege(SpellCollege::Healing, 10),
        ],
        _ => panic!("Invalid spell {:?} for Healing college", spell),
    }
}

/// Returns spell type for Healing spells.
#[instrument]
pub(super) fn spell_type(spell: &Spell) -> SpellType {
    debug!("Getting Healing spell type");
    match spell {
        Spell::LendEnergy => SpellType::Regular,
        Spell::LendVitality => SpellType::Regular,
        Spell::RecoverEnergy => SpellType::Regular,
        Spell::MinorHealing => SpellType::Regular,
        Spell::MajorHealing => SpellType::Regular,
        Spell::GreatHealing => SpellType::Regular,
        Spell::Awaken => SpellType::Regular,
        Spell::CureDisease => SpellType::Regular,
        Spell::NeutralizePoison => SpellType::Regular,
        Spell::InstantNeutralizePoison => SpellType::Regular,
        Spell::StopBleeding => SpellType::Regular,
        Spell::Regeneration => SpellType::Regular,
        Spell::Restoration => SpellType::Regular,
        Spell::SuspendCurse => SpellType::Regular,
        Spell::RemoveCurse => SpellType::Regular,
        Spell::CurseRemoval => SpellType::Regular,
        Spell::Healing => SpellType::Regular,
        Spell::RestoreYouth => SpellType::Regular,
        Spell::Resurrection => SpellType::Regular,
        Spell::Sterilize => SpellType::Area,
        Spell::ShareVitality => SpellType::Regular,
        Spell::RegrowLimb => SpellType::Regular,
        Spell::PurifyFood => SpellType::Regular,
        Spell::InstantRecoverEnergy => SpellType::Regular,
        Spell::StopAging => SpellType::Regular,
        _ => panic!("Invalid spell {:?} for Healing college", spell),
    }
}

/// Returns resistance type for Healing spells.
#[instrument]
pub(super) fn resistance(spell: &Spell) -> Option<ResistanceType> {
    debug!("Getting Healing spell resistance");
    match spell {
        Spell::LendEnergy => None,
        Spell::LendVitality => None,
        Spell::RecoverEnergy => None,
        Spell::MinorHealing => None,
        Spell::MajorHealing => None,
        Spell::GreatHealing => None,
        Spell::Awaken => None,
        Spell::CureDisease => None,
        Spell::NeutralizePoison => None,
        Spell::InstantNeutralizePoison => None,
        Spell::StopBleeding => None,
        Spell::Regeneration => None,
        Spell::Restoration => None,
        Spell::SuspendCurse => None,
        Spell::RemoveCurse => None,
        Spell::CurseRemoval => None,
        Spell::Healing => None,
        Spell::RestoreYouth => None,
        Spell::Resurrection => None,
        Spell::Sterilize => None,
        Spell::ShareVitality => None,
        Spell::RegrowLimb => None,
        Spell::PurifyFood => None,
        Spell::InstantRecoverEnergy => None,
        Spell::StopAging => None,
        _ => panic!("Invalid spell {:?} for Healing college", spell),
    }
}

/// Returns spell reference for Healing spells.
#[instrument]
pub(super) fn reference(spell: &Spell) -> &'static str {
    debug!("Getting Healing spell reference");
    match spell {
        Spell::LendEnergy => "M93",
        Spell::LendVitality => "M94",
        Spell::RecoverEnergy => "M101",
        Spell::MinorHealing => "M96",
        Spell::MajorHealing => "M94",
        Spell::GreatHealing => "M92",
        Spell::Awaken => "M90",
        Spell::CureDisease => "M91",
        Spell::NeutralizePoison => "M98",
        Spell::InstantNeutralizePoison => "M92",
        Spell::StopBleeding => "M103",
        Spell::Regeneration => "M101",
        Spell::Restoration => "M102",
        Spell::SuspendCurse => "M103",
        Spell::RemoveCurse => "M101",
        Spell::CurseRemoval => "M91",
        Spell::Healing => "M92",
        Spell::RestoreYouth => "M102",
        Spell::Resurrection => "M102",
        Spell::Sterilize => "M103",
        Spell::ShareVitality => "M103",
        Spell::RegrowLimb => "M101",
        Spell::PurifyFood => "M98",
        Spell::InstantRecoverEnergy => "M101",
        Spell::StopAging => "M103",
        _ => panic!("Invalid spell {:?} for Healing college", spell),
    }
}
