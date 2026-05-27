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
mod skills;
mod special_features;
// BEGIN ELICITATION KANI REEXPORTS — DO NOT EDIT
pub use ui::begin_compose_kani_contracted;
pub use ui::cancel_compose_kani_contracted;
pub use ui::chat_consistent;
pub use ui::receive_message_kani_contracted;
pub use ui::scroll_down_kani_contracted;
pub use ui::scroll_up_kani_contracted;
pub use ui::send_message_kani_contracted;
pub use vsm::apply_damage_kani_contracted;
pub use vsm::begin_turn_kani_contracted;
pub use vsm::conclude_combat_kani_contracted;
pub use vsm::declare_attack_kani_contracted;
pub use vsm::end_turn_kani_contracted;
pub use vsm::initialize_combat_kani_contracted;
pub use vsm::resolve_attack_kani_contracted;
pub use vsm::resolve_defense_kani_contracted;
// END ELICITATION KANI REEXPORTS






mod ui;
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
    CeremonialCastingBegun,
    CeremonialMagicDescriptor,
    CeremonialMagicEvidence,
    CeremonialSpellCompleted,
    CharacterAdvancement,
    CharacterBuilder,
    CharacterComplete,
    CharacterCreationDescriptor,
    CharacterCreationEvidence,
    CharacterDescriptor,
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

pub use contracts::{
    AdvantageDescriptorBuilder, AttackDescriptorBuilder, CasterDescriptorBuilder,
    CeremonialMagicDescriptorBuilder, CharacterCreationDescriptorBuilder,
    CharacterDescriptorBuilder, CombatantDescriptorBuilder, DamageDescriptorBuilder,
    DefenseDescriptorBuilder, DisadvantageDescriptorBuilder, SkillCheckDescriptorBuilder,
    SkillDescriptorBuilder, SpellCastingDescriptorBuilder, SpellDescriptorBuilder,
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
pub use players::{DefenseChoice, ManeuverChoice, Player, Players};
pub use skills::{Family, Skill, SkillBase, SkillDefault};
pub use special_features::SpecialFeatures;
pub use ui::{
    ChatAction, ChatConsistent, ChatKeyMap, ChatMachine, ChatMessage, ChatMessageMode, ChatSender,
    ChatState, ContextualCommunicator, GameDisplay, ChatModel, KnowledgeCache,
    ObservableCommunicator, Participant, SharedKnowledge, knowledge_cache,
    begin_compose, cancel_compose, receive_message, scroll_down, scroll_up, send_message,
};
#[cfg(feature = "frontend-ratatui")]
pub use ui::run_chat;
pub use vsm::{
    apply_damage, attack_resolved_from_gm, begin_turn, combat_consistent, conclude_combat,
    damage_applied_from_gm, declare_attack, defense_resolved_from_gm, end_turn,
    initialize_combat, resolve_attack, resolve_defense, CombatConsistent, CombatMachine,
    CombatPhase, CombatSession, CombatState, CombatStateView, CombatWorkflow, CombatantState,
    CombatantView, WorkflowError,
};
pub use contracts::{
    AttackDeclared, AttackResolved, CanApplyDamage, CanTakeAction, CombatConcluded,
    CombatInitialized, DamageApplied, DefenseRequired, DefenseResolved, ManeuverSelected,
    RoundCompleted, TurnBegan, TurnEnded, TurnOrderEstablished, VictoryConditionMet,
};
