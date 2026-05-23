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
mod contracts;
mod dice;
mod disadvantages;
mod free;
mod game_master;
mod items;
mod magic;
mod movement;
mod players;
mod proofs;
mod skills;
mod special_features;
mod vsm;

pub use advantages::{
    AbsoluteDirection, Advantage, Appearance, Claws, EiditicMemory, Flexible, Flight,
    InjuryTolerance, Luck, Perk, ProtectedSense, Regeneration, Resistant, SocialRegard, Striker,
    Teeth, Wealth,
};
pub use body::{Arms, BodyArea, BodyLocation, Head, Legs, Torso};
pub use character::{
    AttributeColumns, Attributes, BaseDamage, CombatStats, DamageDice, DamageKind,
    Encumbrance, EncumbranceDodge, EncumbranceLevel, EncumbranceMove, EncumbranceWeight, Stats,
};
pub use cli::Cli;
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
    // Skill propositions
    CharacterPointsSpentOnSkill, ComplementarySkillBonusApplied, ContestWinnerDetermined,
    DefaultPenaltyApplied, FamiliarityPenaltyApplied, SituationalModifierApplied,
    SkillCheckCriticalFailure, SkillCheckCriticalSuccess, SkillCheckFailed,
    SkillCheckOutcomeDetermined, SkillCheckRollMade, SkillCheckSuccessful, SkillContestResolved,
    SkillDefaultedToAttribute, SkillDefaultedToRelatedSkill, SkillLevelIncreased,
    SkillPointBudgetValid, SkillPrerequisiteMet, TaskDifficultyModifierApplied, TechniqueUsed,
    TimeSpentModifierApplied, WildcardSkillUsed,
    // Character propositions
    AdvantageLevelValid, AdvantageModifiersCostCalculated, AdvantagePurchased,
    AdvantagePrerequisiteMet, AttributeCostCalculated, AttributePurchased,
    AttributesMeetCampaignMinimums, BasicMoveCalculated, BasicSpeedCalculated, CharacterComplete,
    CharacterIdentified, CharacterPointValueSet, CharacterValid, DisadvantageLevelValid,
    DisadvantageTaken, DisadvantagePointLimitRespected, DisadvantagesNotConflicting,
    DodgeCalculated, FatiguePointsSet, HitPointsSet, PerceptionSet, PointBudgetBalanced,
    QuirkLimitRespected, QuirkTaken, RacialTemplateRequirementsMet, SecondaryCharacteristicPurchased,
    SelfControlRollSpecified, WillSet,
    // Magic propositions
    BaseEnergyCostDetermined, CeremonialCastingBegun, CeremonialSpellCompleted, ConcentrationBegun,
    ConcentrationCompleted, ConcentrationMaintained, EnergyPaidFromCaster,
    EnergyPooledFromParticipants, EnvironmentModifierApplied, FinalEnergyCostCalculated,
    MageryRequirementMet, MaintenanceEnergyPaid, ResistanceOvercome, ResistanceRollMade,
    ResistanceRollRequired, SizeSpeedModifierApplied, SkillBasedCostReductionApplied,
    SpellCastingFailed, SpellCastingOutcomeDetermined, SpellCastingSucceeded, SpellCriticalFailure,
    SpellCriticalSuccess, SpellDurationDetermined, SpellEffectApplied, SpellLearned,
    SpellMaintained, SpellPrerequisitesMet, SpellRangeChecked, SpellResistedSuccessfully,
    SpellSkillLevelSet, SpellSkillRollMade, SpellTargetDetermined, TimeModifierApplied,
    // Evidence bundles
    AdvantagePurchaseEvidence, AllOutAttackEvidence, AttributePurchaseEvidence,
    AttackCriticalFailureEvidence, AttackCriticalSuccessEvidence, AttackFailureEvidence,
    AttackResolutionEvidence, AttackSuccessEvidence, BasicDamageEvidence, CeremonialMagicEvidence,
    CharacterCreationEvidence, CharacterValidationEvidence, CombatHitEvidence, CombatMissEvidence,
    ComplementarySkillEvidence, CompleteSpellCastingEvidence, ConcentrationEvidence,
    DeceptiveAttackEvidence, DefenseCriticalFailureEvidence, DefenseCriticalSuccessEvidence,
    DefenseFailureEvidence, DefenseResolutionEvidence, DefenseSuccessEvidence, DerivedStatsEvidence,
    DisadvantageTakenEvidence, EnergyCostEvidence, EnergyPaymentEvidence, FeintEvidence,
    InjuryApplicationEvidence, InjuryCalculationEvidence, RapidStrikeEvidence,
    ResistanceOvercomeEvidence, SecondaryCharacteristicEvidence, SkillAttributeDefaultEvidence,
    SkillCheckCriticalFailureEvidence, SkillCheckCriticalSuccessEvidence, SkillCheckFailureEvidence,
    SkillCheckResolutionEvidence, SkillCheckSuccessEvidence, SkillImprovementEvidence,
    SkillModifiersEvidence, SkillRelatedDefaultEvidence, SpellCastingFailureEvidence,
    SpellCastingResolutionEvidence, SpellCastingSuccessEvidence, SpellCriticalFailureEvidence,
    SpellCriticalSuccessEvidence, SpellEffectEvidence, SpellLearningEvidence,
    SpellMaintenanceEvidence, SpellResistanceEvidence, TechniqueUsageEvidence, WildcardSkillEvidence,
    // Trait interfaces
    AttackMeta, AttackResolver, CharacterAdvancement, CharacterBuilder, CharacterImprovement,
    CombatExchangeResult, CombatExecutor, CombatResult, ContractError, DamageCalculator,
    DamageMeta, DefenseMeta, DefenseResolver, ManeuverExecutor, MissReason, SkillCheckExecutor,
    SkillManager, SpellCaster, SpellEffectResolver, SpellExecutionResult, SpellExecutor,
    SpellManager,
    // Descriptor types
    AdvantageDescriptor, AdvantageDescriptorBuilder, ArmorDescriptor, AttributeDescriptor,
    AttributeMinimums, AttributeType, AttackDescriptor, AttackDescriptorBuilder, AttackRollResult,
    CasterDescriptor, CasterDescriptorBuilder, CeremonialMagicDescriptor,
    CeremonialMagicDescriptorBuilder, CharacterCreationDescriptor, CharacterCreationDescriptorBuilder,
    CharacterDescriptor, CharacterDescriptorBuilder, CombatantDescriptor, CombatantDescriptorBuilder,
    DamageDescriptor, DamageDescriptorBuilder, DamageResult, DamageTypeDescriptor,
    DefenseDescriptor, DefenseDescriptorBuilder, DefenseRollResult, DefenseType, HitLocation,
    DerivedStatsDescriptor, DisadvantageDescriptor, DisadvantageDescriptorBuilder,
    FeintDescriptor, FeintResult, ModifierDescriptor, RapidStrikeDescriptor, ResistanceResult,
    SecondaryCharacteristicDescriptor, SecondaryCharacteristicType, SkillCheckDescriptor,
    SkillCheckDescriptorBuilder, SkillCheckResult, SkillDefaultDescriptor, SkillDefaultType,
    SkillDescriptor, SkillDescriptorBuilder, SkillDifficulty, SpellCastingDescriptor,
    SpellCastingDescriptorBuilder, SpellCastingResult, SpellClass, SpellDescriptor,
    SpellDescriptorBuilder, SpellEffectDescriptor, SpellResistanceDescriptor,
};
pub use dice::{DieFace, ThreeDiceRoll};
pub use disadvantages::{Addiction, Disadvantage, Duty, Lame, Phobia, SenseOfDuty, Vow};
pub use free::trace_init;
pub use game_master::{GameMaster, GameMasterConfig, ManaLevel};
pub use vsm::{
    CombatConsistent, CombatMachine, CombatState, CombatantState, apply_damage, begin_turn,
    combat_consistent, declare_attack, end_turn, initialize_combat, resolve_attack,
    resolve_defense,
};
pub use items::{
    Armor as ItemArmor, Capacity, Clothing, Container, Currency, DamageType, Item, MeleeWeapon,
    Quality, RangedWeapon, Reach, SurvivalGear, TechLevel, Tool, WeaponDamage, Weight,
};
pub use magic::{
    Duration, EnergyCost, ResistanceType, Spell, SpellCollege, SpellPrerequisite, SpellType,
};
pub use movement::{AllOutMeleeAttack, AllOutRangedAttack, FreeAction, Manuever, Posture, Success};
pub use players::Players;
pub use skills::{Family, Skill, SkillBase, SkillDefault};
pub use special_features::SpecialFeatures;
