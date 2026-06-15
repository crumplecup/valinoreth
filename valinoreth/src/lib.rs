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
mod spatial;
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
pub use vsm::complete_movement_action_kani_contracted;
pub use vsm::conclude_combat_kani_contracted;
pub use vsm::declare_attack_kani_contracted;
pub use vsm::end_turn_kani_contracted;
pub use vsm::initialize_combat_kani_contracted;
pub use vsm::resolve_attack_kani_contracted;
pub use vsm::resolve_defense_kani_contracted;
// END ELICITATION KANI REEXPORTS

#[cfg(feature = "frontend-ratatui")]
mod lobby;
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
    LineOfEffectClear,
    LocationMultiplierApplied,
    MageryRequirementMet,
    MaintenanceEnergyPaid,
    ManeuverExecutor,
    MissReason,
    ModifierDescriptor,
    MovementCompleted,
    MovementDeclared,
    MovementPathValid,
    MovementWithinBudget,
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
    SpatialStateConsistent,
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
    TargetWithinRange,
    TargetWithinReach,
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
pub use contracts::{
    AttackDeclared, AttackResolved, CanApplyDamage, CanTakeAction, CombatConcluded,
    CombatInitialized, CombatantLocated, DamageApplied, DefenseRequired, DefenseResolved,
    LocationsHaveSameFrame, ManeuverSelected, RoundCompleted, TurnBegan, TurnEnded,
    TurnOrderEstablished, VictoryConditionMet,
};
pub use dice::{DiceGenerator, DieFace, ThreeDiceRoll};
pub use disadvantages::{Addiction, Disadvantage, Duty, Lame, Phobia, SenseOfDuty, Vow};
pub use free::trace_init;
pub use game_master::{GameMaster, GameMasterConfig, ManaLevel};
pub use items::{
    Armor as ItemArmor, Capacity, Clothing, Container, Currency, DamageType, Item, MeleeWeapon,
    Quality, RangedWeapon, Reach, SurvivalGear, TechLevel, Tool, WeaponDamage, Weight,
};
#[cfg(feature = "frontend-ratatui")]
pub use lobby::{
    default_roster, run_lobby, AgentConfig, CombatCommunicator, CombatSetupScreen, CombatSlot,
    ConfigError, LobbyController, LobbySettings, MainLobbyScreen, PlayerKind, RosterEntry, Screen,
    ScreenTransition, SettingsScreen,
};
pub use magic::{
    Duration, EnergyCost, ResistanceType, Spell, SpellCollege, SpellPrerequisite, SpellType,
};
pub use movement::{
    apply_completed_movement, complete_movement, declare_movement,
    establish_movement_within_budget, movement_budget_from_effective_move,
    step_budget_from_effective_move, validate_movement_path, AllOutMeleeAttack, AllOutRangedAttack,
    FreeAction, Manuever, MovementBudget, MovementBudgetBuilder, MovementError, MovementErrorKind,
    MovementIntent, MovementIntentBuilder, MovementPath, MovementPathBuilder, MovementResult,
    Posture, Success, GURPS_STEP_MINIMUM_METERS,
};
pub use players::{
    DefenseChoice, ManeuverChoice, MovementChoice, MovementChoiceBuilder, Player, Players,
};
pub use skills::{Family, Skill, SkillBase, SkillDefault};
pub use spatial::{
    attack_range_limit, attack_reach_from_reach, build_default_tactical_point,
    build_tactical_point, check_line_of_effect_between_combatants, check_target_within_range,
    check_target_within_reach, combatant_occupancy_from_placement, distance_between_combatants,
    distance_between_placements, establish_same_frame, establish_spatial_state,
    initial_spatial_state_for_combat, local_tactical_frame, locate_combatant, tactical_footprint,
    tactical_frame_from_source, AttackRange, AttackRangeBuilder, AttackReach, AttackReachBuilder,
    CombatSpatialState, CombatSpatialStateBuilder, CombatantOccupancy, CombatantOccupancyBuilder,
    CombatantPlacement, CombatantPlacementBuilder, SpatialError, SpatialErrorKind, SpatialResult,
    SpatiallyCheckedMeleeTarget, SpatiallyCheckedMeleeTargetBuilder, SpatiallyCheckedRangedTarget,
    SpatiallyCheckedRangedTargetBuilder, TacticalCoordinate, TacticalCoordinateBuilder,
    TacticalDistance, TacticalDistanceBuilder, TacticalDistanceUnit, TacticalFootprint,
    TacticalFootprintBuilder, TacticalFrame, TacticalFrameBuilder, TacticalObstacle,
    TacticalObstacleBuilder, TacticalPoint, TacticalPointBuilder, TacticalTerrainZone,
    TacticalTerrainZoneBuilder, VALINORETH_TACTICAL_SRID, WGS84_SRID,
};
pub use special_features::SpecialFeatures;
pub use ui::{
    begin_compose, cancel_compose, knowledge_cache, receive_message, scroll_down, scroll_up,
    send_message, ChatAction, ChatConsistent, ChatKeyMap, ChatMachine, ChatMessage,
    ChatMessageMode, ChatModel, ChatSender, ChatState, ContextualCommunicator, GameDisplay,
    KnowledgeCache, ObservableCommunicator, Participant, SharedKnowledge,
};
#[cfg(feature = "frontend-ratatui")]
pub use ui::{
    run_chat, ChatCommunicator, LlmClient, LlmConfig, LlmElicitCommunicator, LlmProvider,
    TuiCommunicator,
};
pub use vsm::{
    apply_damage, attack_resolved_from_gm, begin_turn, combat_consistent, complete_movement_action,
    conclude_combat, damage_applied_from_gm, declare_attack, declared_attack_target,
    defense_resolved_from_gm, end_turn, initialize_combat, resolve_attack, resolve_defense,
    CombatConsistent, CombatMachine, CombatPhase, CombatSession, CombatState, CombatStateView,
    CombatWorkflow, CombatantPositionView, CombatantSlot, CombatantState, CombatantView,
    DeclaredAttackTarget, WorkflowError,
};
