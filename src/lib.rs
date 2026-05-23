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
    AttributeColumns, Attributes, BaseDamage, CombatStats, DamageDice, DamageKind, Encumbrance,
    EncumbranceDodge, EncumbranceLevel, EncumbranceMove, EncumbranceWeight, Stats,
};
pub use cli::Cli;
pub use contracts::{
    // Descriptor types
    AdvantageDescriptor,
    AdvantageDescriptorBuilder,
    // Character propositions
    AdvantageLevelValid,
    AdvantageModifiersCostCalculated,
    AdvantagePrerequisiteMet,
    // Evidence bundles
    AdvantagePurchaseEvidence,
    AdvantagePurchased,
    // Combat propositions
    AimBonusApplied,
    AllOutAttackDeclared,
    AllOutAttackEvidence,
    // Logical operators from elicitation
    And,
    ArmorDescriptor,
    AttackCriticalFailure,
    AttackCriticalFailureEvidence,
    AttackCriticalSuccess,
    AttackCriticalSuccessEvidence,
    AttackDescriptor,
    AttackDescriptorBuilder,
    AttackFailed,
    AttackFailureEvidence,
    // Trait interfaces
    AttackMeta,
    AttackOutcomeDetermined,
    AttackResolutionEvidence,
    AttackResolver,
    AttackRollMade,
    AttackRollResult,
    AttackSuccessEvidence,
    AttackSuccessful,
    AttributeCostCalculated,
    AttributeDescriptor,
    AttributeMinimums,
    AttributePurchaseEvidence,
    AttributePurchased,
    AttributeType,
    AttributesMeetCampaignMinimums,
    // Magic propositions
    BaseEnergyCostDetermined,
    BasicDamageCalculated,
    BasicDamageEvidence,
    BasicMoveCalculated,
    BasicSpeedCalculated,
    CasterDescriptor,
    CasterDescriptorBuilder,
    CeremonialCastingBegun,
    CeremonialMagicDescriptor,
    CeremonialMagicDescriptorBuilder,
    CeremonialMagicEvidence,
    CeremonialSpellCompleted,
    CharacterAdvancement,
    CharacterBuilder,
    CharacterComplete,
    CharacterCreationDescriptor,
    CharacterCreationDescriptorBuilder,
    CharacterCreationEvidence,
    CharacterDescriptor,
    CharacterDescriptorBuilder,
    CharacterIdentified,
    CharacterImprovement,
    CharacterPointValueSet,
    // Skill propositions
    CharacterPointsSpentOnSkill,
    CharacterValid,
    CharacterValidationEvidence,
    CombatExchangeResult,
    CombatExecutor,
    CombatHitEvidence,
    CombatMissEvidence,
    CombatResult,
    CombatantDescriptor,
    CombatantDescriptorBuilder,
    ComplementarySkillBonusApplied,
    ComplementarySkillEvidence,
    CompleteSpellCastingEvidence,
    ConcentrationBegun,
    ConcentrationCompleted,
    ConcentrationEvidence,
    ConcentrationMaintained,
    ContestWinnerDetermined,
    ContractError,
    DamageCalculator,
    DamageDescriptor,
    DamageDescriptorBuilder,
    DamageMeta,
    DamageResistanceApplied,
    DamageResult,
    DamageTypeDescriptor,
    DeceptiveAttackApplied,
    DeceptiveAttackEvidence,
    DefaultPenaltyApplied,
    DefenseCriticalFailure,
    DefenseCriticalFailureEvidence,
    DefenseCriticalSuccess,
    DefenseCriticalSuccessEvidence,
    DefenseDescriptor,
    DefenseDescriptorBuilder,
    DefenseFailed,
    DefenseFailureEvidence,
    DefenseMeta,
    DefenseOutcomeDetermined,
    DefenseResolutionEvidence,
    DefenseResolver,
    DefenseRollMade,
    DefenseRollResult,
    DefenseSuccessEvidence,
    DefenseSuccessful,
    DefenseType,
    DerivedStatsDescriptor,
    DerivedStatsEvidence,
    DisadvantageDescriptor,
    DisadvantageDescriptorBuilder,
    DisadvantageLevelValid,
    DisadvantagePointLimitRespected,
    DisadvantageTaken,
    DisadvantageTakenEvidence,
    DisadvantagesNotConflicting,
    DodgeCalculated,
    EnergyCostEvidence,
    EnergyPaidFromCaster,
    EnergyPaymentEvidence,
    EnergyPooledFromParticipants,
    EnvironmentModifierApplied,
    Established,
    FamiliarityPenaltyApplied,
    FatiguePointsSet,
    FeintDescriptor,
    FeintEvidence,
    FeintResult,
    FeintSuccessful,
    FinalEnergyCostCalculated,
    HitLocation,
    HitLocationDetermined,
    HitPointsSet,
    Implies,
    InVariant,
    InjuryApplicationEvidence,
    InjuryApplied,
    InjuryCalculated,
    InjuryCalculationEvidence,
    Is,
    LocationMultiplierApplied,
    MageryRequirementMet,
    MaintenanceEnergyPaid,
    ManeuverExecutor,
    MissReason,
    ModifierDescriptor,
    PerceptionSet,
    PointBudgetBalanced,
    Prop,
    ProvableFrom,
    QuirkLimitRespected,
    QuirkTaken,
    RacialTemplateRequirementsMet,
    RapidStrikeDescriptor,
    RapidStrikeEvidence,
    RapidStrikeExecuted,
    Refines,
    ResistanceOvercome,
    ResistanceOvercomeEvidence,
    ResistanceResult,
    ResistanceRollMade,
    ResistanceRollRequired,
    SecondaryCharacteristicDescriptor,
    SecondaryCharacteristicEvidence,
    SecondaryCharacteristicPurchased,
    SecondaryCharacteristicType,
    SelfControlRollSpecified,
    SituationalModifierApplied,
    SizeSpeedModifierApplied,
    SkillAttributeDefaultEvidence,
    SkillBasedCostReductionApplied,
    SkillCheckCriticalFailure,
    SkillCheckCriticalFailureEvidence,
    SkillCheckCriticalSuccess,
    SkillCheckCriticalSuccessEvidence,
    SkillCheckDescriptor,
    SkillCheckDescriptorBuilder,
    SkillCheckExecutor,
    SkillCheckFailed,
    SkillCheckFailureEvidence,
    SkillCheckOutcomeDetermined,
    SkillCheckResolutionEvidence,
    SkillCheckResult,
    SkillCheckRollMade,
    SkillCheckSuccessEvidence,
    SkillCheckSuccessful,
    SkillContestResolved,
    SkillDefaultDescriptor,
    SkillDefaultType,
    SkillDefaultedToAttribute,
    SkillDefaultedToRelatedSkill,
    SkillDescriptor,
    SkillDescriptorBuilder,
    SkillDifficulty,
    SkillImprovementEvidence,
    SkillLevelIncreased,
    SkillManager,
    SkillModifiersEvidence,
    SkillPointBudgetValid,
    SkillPrerequisiteMet,
    SkillRelatedDefaultEvidence,
    SpellCaster,
    SpellCastingDescriptor,
    SpellCastingDescriptorBuilder,
    SpellCastingFailed,
    SpellCastingFailureEvidence,
    SpellCastingOutcomeDetermined,
    SpellCastingResolutionEvidence,
    SpellCastingResult,
    SpellCastingSucceeded,
    SpellCastingSuccessEvidence,
    SpellClass,
    SpellCriticalFailure,
    SpellCriticalFailureEvidence,
    SpellCriticalSuccess,
    SpellCriticalSuccessEvidence,
    SpellDescriptor,
    SpellDescriptorBuilder,
    SpellDurationDetermined,
    SpellEffectApplied,
    SpellEffectDescriptor,
    SpellEffectEvidence,
    SpellEffectResolver,
    SpellExecutionResult,
    SpellExecutor,
    SpellLearned,
    SpellLearningEvidence,
    SpellMaintained,
    SpellMaintenanceEvidence,
    SpellManager,
    SpellPrerequisitesMet,
    SpellRangeChecked,
    SpellResistanceDescriptor,
    SpellResistanceEvidence,
    SpellResistedSuccessfully,
    SpellSkillLevelSet,
    SpellSkillRollMade,
    SpellTargetDetermined,
    TaskDifficultyModifierApplied,
    TechniqueUsageEvidence,
    TechniqueUsed,
    TimeModifierApplied,
    TimeSpentModifierApplied,
    WeaponDamageRolled,
    WildcardSkillEvidence,
    WildcardSkillUsed,
    WillSet,
    WoundingModifierApplied,
};
pub use dice::{DieFace, ThreeDiceRoll};
pub use disadvantages::{Addiction, Disadvantage, Duty, Lame, Phobia, SenseOfDuty, Vow};
pub use free::trace_init;
pub use game_master::{GameMaster, GameMasterConfig, ManaLevel};
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
pub use vsm::{
    apply_damage, begin_turn, combat_consistent, declare_attack, end_turn, initialize_combat,
    resolve_attack, resolve_defense, CombatConsistent, CombatMachine, CombatState, CombatantState,
};
