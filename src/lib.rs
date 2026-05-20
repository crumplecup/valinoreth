//! # Valinoreth
//!
//! A Rust implementation of GURPS (Generic Universal RolePlaying System) game mechanics.
//!
//! ## Attribution
//!
//! GURPS is a trademark of Steve Jackson Games, and its rules and contents are copyrighted
//! by Steve Jackson Games. All rights are reserved by Steve Jackson Games.
//!
//! This is an unofficial, non-commercial implementation for personal use only.
//! This material is not official and is not endorsed by Steve Jackson Games.
//!
//! For official GURPS products, visit <http://www.sjgames.com/>

#![warn(missing_docs)]

mod advantages;
mod body;
mod character;
mod cli;
mod combat;
mod contracts;
mod dice;
mod disadvantages;
mod free;
mod items;
mod magic;
mod movement;
mod players;
mod skills;
mod special_features;

pub use advantages::{
    AbsoluteDirection, Advantage, Appearance, Claws, EiditicMemory, Flexible, Flight,
    InjuryTolerance, Luck, Perk, ProtectedSense, Regeneration, Resistant, SocialRegard, Striker,
    Teeth, Wealth,
};
pub use body::{Arms, BodyArea, BodyLocation, Head, Legs, Torso};
pub use character::{
    AttributeColumns, AttributeType, Attributes, BaseDamage, CombatStats, DamageKind, Encumbrance,
    EncumbranceDodge, EncumbranceLevel, EncumbranceMove, EncumbranceWeight, Stats,
};
pub use cli::Cli;
pub use combat::{
    calculate_block, calculate_dodge, calculate_parry, defense_succeeds, ActiveDefense, Armor,
    ArmorBuilder, AttackResult, AttackRoll, CombatError, CombatErrorKind, CombatModifiers,
    CombatModifiersBuilder, DamageResolution, DamageType, DefenseResult, Modifier, Reach,
    Technique, TechniqueDifficulty, WeaponDamage, RETREAT_BONUS,
};
pub use contracts::{
    // Logical operators from elicitation
    And, Established, Implies, InVariant, Is, Prop, ProvableFrom, Refines,
    // Combat propositions
    AimBonusApplied, AllOutAttackDeclared, AttackCriticalFailure, AttackCriticalSuccess,
    AttackFailed, AttackOutcomeDetermined, AttackRollMade, AttackSuccessful,
    BasicDamageCalculated, DamageResistanceApplied, DeceptiveAttackApplied,
    DefenseCriticalFailure, DefenseCriticalSuccess, DefenseFailed, DefenseOutcomeDetermined,
    DefenseRollMade, DefenseSuccessful, FeintSuccessful, HitLocationDetermined, InjuryApplied,
    InjuryCalculated, LocationMultiplierApplied, RapidStrikeExecuted, WeaponDamageRolled,
    WoundingModifierApplied,
    // Evidence bundles
    AllOutAttackEvidence, AttackCriticalFailureEvidence, AttackCriticalSuccessEvidence,
    AttackFailureEvidence, AttackResolutionEvidence, AttackSuccessEvidence, BasicDamageEvidence,
    CombatHitEvidence, CombatMissEvidence, DeceptiveAttackEvidence,
    DefenseCriticalFailureEvidence, DefenseCriticalSuccessEvidence, DefenseFailureEvidence,
    DefenseResolutionEvidence, DefenseSuccessEvidence, FeintEvidence,
    InjuryApplicationEvidence, InjuryCalculationEvidence, RapidStrikeEvidence,
    // Trait interfaces
    AttackMeta, AttackResolver, CombatExchangeResult, CombatExecutor, CombatResult, ContractError,
    DamageCalculator, DamageMeta, DefenseMeta, DefenseResolver, ManeuverExecutor, MissReason,
    // Descriptor types
    ArmorDescriptor, AttackDescriptor, AttackDescriptorBuilder, AttackRollResult,
    CombatantDescriptor, CombatantDescriptorBuilder, DamageDescriptor, DamageDescriptorBuilder,
    DamageResult, DamageTypeDescriptor, DefenseDescriptor, DefenseDescriptorBuilder,
    DefenseRollResult, DefenseType, FeintDescriptor, FeintResult, HitLocation,
    RapidStrikeDescriptor,
};
pub use dice::{Dice, DieLevel, Random};
pub use disadvantages::{Addiction, Disadvantage, Duty, Lame, Phobia, SenseOfDuty, Vow};
pub use free::trace_init;
pub use items::{
    Armor as ItemArmor, Capacity, Clothing, Container, Currency, Item, MeleeWeapon, Quality,
    RangedWeapon, SurvivalGear, TechLevel, Tool, Weight,
};
pub use magic::{
    Duration, EnergyCost, ResistanceType, Spell, SpellCollege, SpellPrerequisite, SpellType,
};
pub use movement::{AllOutMeleeAttack, AllOutRangedAttack, FreeAction, Manuever, Posture, Success};
pub use players::Players;
pub use skills::{Family, Skill, SkillBase, SkillDefault};
pub use special_features::SpecialFeatures;
