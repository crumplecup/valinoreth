//! Proof credentials for combat operations.
//!
//! Credentials establish [`ProvableFrom`] relationships between runtime checks
//! and propositions. Each credential is a zero-sized type that can only be
//! constructed by code that performed the corresponding validation.
//!
//! # Pattern
//!
//! Credentials are `pub(crate)` so only internal code can mint them:
//!
//! ```rust,ignore
//! // Inside resolver implementation (same crate)
//! let roll = roll_3d6();
//! let credential = ValidDiceRoll { roll };
//! let proof: Established<AttackRollMade> = Established::prove(&credential);
//!
//! // External code CANNOT construct ValidDiceRoll
//! // External code CANNOT call Established::prove without the credential
//! ```
//!
//! This ensures proofs can only be minted by code that actually performed
//! the validation.

use crate::contracts::character::{
    AdvantageLevelValid, AdvantageModifiersCostCalculated, AdvantagePrerequisiteMet,
    AdvantagePurchased, AttributeCostCalculated, AttributePurchased,
    AttributesMeetCampaignMinimums, BasicMoveCalculated, BasicSpeedCalculated, CharacterComplete,
    CharacterIdentified, CharacterPointValueSet, CharacterValid, DisadvantageLevelValid,
    DisadvantagePointLimitRespected, DisadvantageTaken, DisadvantagesNotConflicting,
    DodgeCalculated, FatiguePointsSet, HitPointsSet, PerceptionSet, PointBudgetBalanced,
    QuirkLimitRespected, QuirkTaken, RacialTemplateRequirementsMet,
    SecondaryCharacteristicPurchased, SelfControlRollSpecified, WillSet,
};
use crate::contracts::combat::{
    AimBonusApplied, AllOutAttackDeclared, AttackCriticalFailure, AttackCriticalSuccess,
    AttackFailed, AttackOutcomeDetermined, AttackRollMade, AttackSuccessful, BasicDamageCalculated,
    DamageResistanceApplied, DeceptiveAttackApplied, DefenseCriticalFailure,
    DefenseCriticalSuccess, DefenseFailed, DefenseOutcomeDetermined, DefenseRollMade,
    DefenseSuccessful, FeintSuccessful, HitLocationDetermined, InjuryApplied, InjuryCalculated,
    LocationMultiplierApplied, RapidStrikeExecuted, WeaponDamageRolled, WoundingModifierApplied,
};
use crate::contracts::combat_flow::{
    AttackDeclared, AttackResolved, CanApplyDamage, CanTakeAction, CombatConcluded,
    CombatInitialized, DamageApplied, DefenseRequired, DefenseResolved, ManeuverSelected,
    RoundCompleted, TurnBegan, TurnEnded, TurnOrderEstablished, VictoryConditionMet,
};
use crate::contracts::magic::{
    BaseEnergyCostDetermined, CeremonialCastingBegun, CeremonialSpellCompleted, ConcentrationBegun,
    ConcentrationCompleted, ConcentrationMaintained, EnergyPaidFromCaster,
    EnergyPooledFromParticipants, EnvironmentModifierApplied, FinalEnergyCostCalculated,
    MageryRequirementMet, MaintenanceEnergyPaid, ResistanceOvercome, ResistanceRollMade,
    ResistanceRollRequired, SizeSpeedModifierApplied, SkillBasedCostReductionApplied,
    SpellCastingFailed, SpellCastingOutcomeDetermined, SpellCastingSucceeded, SpellCriticalFailure,
    SpellCriticalSuccess, SpellDurationDetermined, SpellEffectApplied, SpellLearned,
    SpellMaintained, SpellPrerequisitesMet, SpellRangeChecked, SpellResistedSuccessfully,
    SpellSkillLevelSet, SpellSkillRollMade, SpellTargetDetermined, TimeModifierApplied,
};
use crate::contracts::skills::{
    CharacterPointsSpentOnSkill, ComplementarySkillBonusApplied, ContestWinnerDetermined,
    DefaultPenaltyApplied, FamiliarityPenaltyApplied, SituationalModifierApplied,
    SkillCheckCriticalFailure, SkillCheckCriticalSuccess, SkillCheckFailed,
    SkillCheckOutcomeDetermined, SkillCheckRollMade, SkillCheckSuccessful, SkillContestResolved,
    SkillDefaultedToAttribute, SkillDefaultedToRelatedSkill, SkillLevelIncreased,
    SkillPointBudgetValid, SkillPrerequisiteMet, TaskDifficultyModifierApplied, TechniqueUsed,
    TimeSpentModifierApplied, WildcardSkillUsed,
};
use elicitation::proof_credential;

// ── Attack Roll Credentials ───────────────────────────────────────────────────

proof_credential! {
    /// Witness that 3d6 was rolled for an attack.
    ///
    /// Only constructible by code that actually rolled the dice.
    pub ValidAttackRoll => AttackRollMade;

    /// Witness that attack roll was compared to effective skill.
    ///
    /// Only constructible by code that performed the comparison.
    pub AttackOutcomeChecked => AttackOutcomeDetermined;

    /// Witness that attack succeeded (roll ≤ skill).
    ///
    /// Only constructible when roll result confirms success.
    pub AttackHit => AttackSuccessful;

    /// Witness that attack failed (roll > skill).
    ///
    /// Only constructible when roll result confirms failure.
    pub AttackMiss => AttackFailed;

    /// Witness that attack achieved critical success.
    ///
    /// Only constructible when roll meets critical success criteria.
    pub AttackCriticalHit => AttackCriticalSuccess;

    /// Witness that attack suffered critical failure.
    ///
    /// Only constructible when roll meets critical failure criteria.
    pub AttackCriticalMiss => AttackCriticalFailure;
}

// ── Defense Roll Credentials ──────────────────────────────────────────────────

proof_credential! {
    /// Witness that 3d6 was rolled for defense.
    pub ValidDefenseRoll => DefenseRollMade;

    /// Witness that defense roll was compared to defense score.
    pub DefenseOutcomeChecked => DefenseOutcomeDetermined;

    /// Witness that defense succeeded (roll ≤ defense score).
    pub DefenseWorked => DefenseSuccessful;

    /// Witness that defense failed (roll > defense score).
    pub DefenseDidNotWork => DefenseFailed;

    /// Witness that defense achieved critical success.
    pub DefenseCriticalWin => DefenseCriticalSuccess;

    /// Witness that defense suffered critical failure.
    pub DefenseCriticalMiss => DefenseCriticalFailure;
}

// ── Damage Calculation Credentials ────────────────────────────────────────────

proof_credential! {
    /// Witness that weapon damage dice were rolled.
    pub ValidDamageRoll => WeaponDamageRolled;

    /// Witness that Damage Resistance was subtracted from damage.
    pub DrSubtracted => DamageResistanceApplied;

    /// Witness that basic (pre-multiplier) damage was calculated.
    pub BasicDamageComputed => BasicDamageCalculated;

    /// Witness that hit location was determined (rolled or targeted).
    pub LocationDetermined => HitLocationDetermined;

    /// Witness that hit location damage multiplier was applied.
    pub LocationMultiplierComputed => LocationMultiplierApplied;

    /// Witness that wounding modifier for damage type was applied.
    pub WoundingMultiplierComputed => WoundingModifierApplied;

    /// Witness that final injury to HP was calculated.
    pub InjuryComputed => InjuryCalculated;

    /// Witness that injury was subtracted from character's HP.
    pub InjurySubtracted => InjuryApplied;
}

// ── Special Maneuver Credentials ──────────────────────────────────────────────

proof_credential! {
    /// Witness that All-Out Attack maneuver was declared.
    pub AllOutAttackChosen => AllOutAttackDeclared;

    /// Witness that Aim maneuver accumulated accuracy bonus.
    pub AimAccumulated => AimBonusApplied;

    /// Witness that Feint succeeded in Quick Contest.
    pub FeintWon => FeintSuccessful;

    /// Witness that Deceptive Attack penalty was applied.
    pub DeceptiveApplied => DeceptiveAttackApplied;

    /// Witness that Rapid Strike was executed.
    pub RapidStrikePerformed => RapidStrikeExecuted;
}

// ── Helper Types ──────────────────────────────────────────────────────────────

/// Validated dice roll result.
///
/// Contains the roll value and serves as evidence that dice were actually rolled.
/// Only constructible by code that performed the roll.
#[derive(Debug, Clone, Copy)]
pub struct DiceRollResult {
    /// The sum of 3d6
    pub value: i32,
}

impl DiceRollResult {
    /// Construct from a validated roll.
    ///
    /// # Safety
    ///
    /// Caller must ensure `value` is an actual 3d6 roll (3-18).
    pub fn new(value: i32) -> Self {
        debug_assert!(
            (3..=18).contains(&value),
            "Dice roll must be 3-18, got {}",
            value
        );
        Self { value }
    }
}

/// Validated skill comparison result.
///
/// Contains the margin of success/failure and serves as evidence that
/// roll-vs-skill comparison was performed.
#[derive(Debug, Clone, Copy)]
pub struct SkillCheckResult {
    /// The roll value
    pub roll: i32,
    /// The skill level rolled against
    pub skill: i32,
    /// Whether roll ≤ skill (success)
    pub success: bool,
    /// Margin of success (positive) or failure (negative)
    pub margin: i32,
}

impl SkillCheckResult {
    /// Construct from validated comparison.
    pub fn new(roll: i32, skill: i32) -> Self {
        let success = roll <= skill;
        let margin = if success { skill - roll } else { roll - skill };
        Self {
            roll,
            skill,
            success,
            margin,
        }
    }

    /// Check if this is a critical success.
    ///
    /// # GURPS Rules
    ///
    /// Critical success when:
    /// - Roll is 3 or 4, OR
    /// - Roll is 5 or 6 AND skill ≥ 15
    pub fn is_critical_success(&self) -> bool {
        self.roll <= 4 || (self.roll <= 6 && self.skill >= 15)
    }

    /// Check if this is a critical failure.
    ///
    /// # GURPS Rules
    ///
    /// Critical failure when:
    /// - Roll is 18, OR
    /// - Roll is 17 AND skill < 16, OR
    /// - Margin of failure ≥ 10
    pub fn is_critical_failure(&self) -> bool {
        self.roll >= 18
            || (self.roll >= 17 && self.skill < 16)
            || (!self.success && self.margin >= 10)
    }
}

/// Validated damage calculation.
///
/// Contains damage components and serves as evidence that damage was calculated.
#[derive(Debug, Clone, Copy)]
pub struct DamageCalculation {
    /// Raw damage rolled
    pub raw: i32,
    /// Damage Resistance applied
    pub dr: i32,
    /// Penetrating damage (raw - DR, minimum 0)
    pub penetrating: i32,
    /// Location damage multiplier
    pub location_mult: f32,
    /// Wounding modifier
    pub wounding_mult: f32,
    /// Final injury to HP
    pub injury: i32,
}

impl DamageCalculation {
    /// Construct from validated calculation.
    pub fn new(raw: i32, dr: i32, location_mult: f32, wounding_mult: f32) -> Self {
        let penetrating = (raw - dr).max(0);
        let injury = ((penetrating as f32) * location_mult * wounding_mult).round() as i32;
        Self {
            raw,
            dr,
            penetrating,
            location_mult,
            wounding_mult,
            injury,
        }
    }
}

// ── Skill Check Credentials ───────────────────────────────────────────────────

proof_credential! {
    /// Witness that 3d6 was rolled for a skill check.
    pub ValidSkillCheckRoll => SkillCheckRollMade;

    /// Witness that skill check roll was compared to effective skill.
    pub SkillCheckOutcomeChecked => SkillCheckOutcomeDetermined;

    /// Witness that skill check succeeded (roll ≤ skill).
    pub SkillCheckHit => SkillCheckSuccessful;

    /// Witness that skill check failed (roll > skill).
    pub SkillCheckMiss => SkillCheckFailed;

    /// Witness that skill check achieved critical success.
    pub SkillCheckCriticalHit => SkillCheckCriticalSuccess;

    /// Witness that skill check suffered critical failure.
    pub SkillCheckCriticalMiss => SkillCheckCriticalFailure;
}

// ── Skill Default Credentials ─────────────────────────────────────────────────

proof_credential! {
    /// Witness that skill defaulted to an attribute.
    pub AttributeDefaultUsed => SkillDefaultedToAttribute;

    /// Witness that skill defaulted to a related skill.
    pub RelatedSkillDefaultUsed => SkillDefaultedToRelatedSkill;

    /// Witness that default penalty was applied.
    pub DefaultPenaltyAppliedCredential => DefaultPenaltyApplied;
}

// ── Skill Modifier Credentials ────────────────────────────────────────────────

proof_credential! {
    /// Witness that situational modifier was applied.
    pub SituationalModApplied => SituationalModifierApplied;

    /// Witness that task difficulty modifier was applied.
    pub TaskDifficultyApplied => TaskDifficultyModifierApplied;

    /// Witness that time spent modifier was applied.
    pub TimeSpentApplied => TimeSpentModifierApplied;

    /// Witness that complementary skill bonus was applied.
    pub ComplementaryBonusApplied => ComplementarySkillBonusApplied;

    /// Witness that familiarity penalty was applied.
    pub FamiliarityPenaltyAppliedCredential => FamiliarityPenaltyApplied;
}

// ── Skill Improvement Credentials ─────────────────────────────────────────────

proof_credential! {
    /// Witness that character points were spent on skill.
    pub PointsSpentOnSkill => CharacterPointsSpentOnSkill;

    /// Witness that skill level was increased.
    pub SkillLevelRaised => SkillLevelIncreased;

    /// Witness that skill prerequisite was met.
    pub PrerequisiteMet => SkillPrerequisiteMet;

    /// Witness that skill point budget is valid.
    pub SkillBudgetValid => SkillPointBudgetValid;
}

// ── Special Skill Usage Credentials ───────────────────────────────────────────

proof_credential! {
    /// Witness that wildcard skill was used.
    pub WildcardUsed => WildcardSkillUsed;

    /// Witness that technique was used.
    pub TechniqueApplied => TechniqueUsed;

    /// Witness that skill contest was resolved.
    pub ContestResolved => SkillContestResolved;

    /// Witness that contest winner was determined.
    pub WinnerDetermined => ContestWinnerDetermined;
}

// ── Character Point Budget Credentials ────────────────────────────────────────

proof_credential! {
    /// Witness that character point value was set.
    pub PointValueSet => CharacterPointValueSet;

    /// Witness that point budget is balanced.
    pub BudgetBalanced => PointBudgetBalanced;

    /// Witness that disadvantage point limit is respected.
    pub DisadvantageLimitRespected => DisadvantagePointLimitRespected;
}

// ── Attribute Purchase Credentials ────────────────────────────────────────────

proof_credential! {
    /// Witness that attribute was purchased.
    pub AttributeBought => AttributePurchased;

    /// Witness that attribute cost was calculated correctly.
    pub AttributeCostComputed => AttributeCostCalculated;

    /// Witness that secondary characteristic was purchased.
    pub SecondaryCharacteristicBought => SecondaryCharacteristicPurchased;
}

// ── Advantage Selection Credentials ───────────────────────────────────────────

proof_credential! {
    /// Witness that advantage was purchased.
    pub AdvantageBought => AdvantagePurchased;

    /// Witness that advantage modifiers were calculated.
    pub AdvantageModifiersComputed => AdvantageModifiersCostCalculated;

    /// Witness that advantage level is valid.
    pub AdvantageLevelValidated => AdvantageLevelValid;

    /// Witness that advantage prerequisite was met.
    pub AdvantagePrereqMet => AdvantagePrerequisiteMet;
}

// ── Disadvantage Selection Credentials ────────────────────────────────────────

proof_credential! {
    /// Witness that disadvantage was taken.
    pub DisadvantageTakenCredential => DisadvantageTaken;

    /// Witness that self-control roll was specified.
    pub SelfControlSpecified => SelfControlRollSpecified;

    /// Witness that disadvantage level is valid.
    pub DisadvantageLevelValidated => DisadvantageLevelValid;

    /// Witness that disadvantages do not conflict.
    pub NoConflicts => DisadvantagesNotConflicting;
}

// ── Quirk Credentials ─────────────────────────────────────────────────────────

proof_credential! {
    /// Witness that quirk was taken.
    pub QuirkTakenCredential => QuirkTaken;

    /// Witness that quirk limit is respected.
    pub QuirkLimitRespectedCredential => QuirkLimitRespected;
}

// ── Derived Statistics Credentials ────────────────────────────────────────────

proof_credential! {
    /// Witness that Basic Speed was calculated.
    pub BasicSpeedComputed => BasicSpeedCalculated;

    /// Witness that Basic Move was calculated.
    pub BasicMoveComputed => BasicMoveCalculated;

    /// Witness that Dodge was calculated.
    pub DodgeComputed => DodgeCalculated;

    /// Witness that HP was set.
    pub HpSet => HitPointsSet;

    /// Witness that Will was set.
    pub WillSetCredential => WillSet;

    /// Witness that Perception was set.
    pub PerceptionSetCredential => PerceptionSet;

    /// Witness that FP was set.
    pub FpSet => FatiguePointsSet;
}

// ── Character Validity Credentials ────────────────────────────────────────────

proof_credential! {
    /// Witness that attributes meet campaign minimums.
    pub MinimumsMet => AttributesMeetCampaignMinimums;

    /// Witness that racial template requirements were met.
    pub RacialRequirementsMet => RacialTemplateRequirementsMet;

    /// Witness that character is identified.
    pub CharacterNamed => CharacterIdentified;

    /// Witness that character is complete.
    pub CharacterCompleteCredential => CharacterComplete;

    /// Witness that character is valid.
    pub CharacterValidated => CharacterValid;
}

// ── Spell Learning Credentials ────────────────────────────────────────────────

proof_credential! {
    /// Witness that spell prerequisites were met.
    pub PrerequisitesMet => SpellPrerequisitesMet;

    /// Witness that Magery requirement was met.
    pub MageryMet => MageryRequirementMet;

    /// Witness that spell was learned.
    pub SpellAcquired => SpellLearned;

    /// Witness that spell skill level was set.
    pub SpellSkillSet => SpellSkillLevelSet;
}

// ── Spell Casting Credentials ─────────────────────────────────────────────────

proof_credential! {
    /// Witness that concentration began.
    pub ConcentrationStarted => ConcentrationBegun;

    /// Witness that concentration was maintained.
    pub ConcentrationKept => ConcentrationMaintained;

    /// Witness that concentration completed.
    pub ConcentrationFinished => ConcentrationCompleted;

    /// Witness that spell skill roll was made.
    pub SpellRollMade => SpellSkillRollMade;

    /// Witness that spell casting outcome was determined.
    pub CastingOutcomeChecked => SpellCastingOutcomeDetermined;

    /// Witness that spell casting succeeded.
    pub CastingSucceeded => SpellCastingSucceeded;

    /// Witness that spell casting failed.
    pub CastingFailed => SpellCastingFailed;

    /// Witness that spell critically succeeded.
    pub SpellCritHit => SpellCriticalSuccess;

    /// Witness that spell critically failed.
    pub SpellCritMiss => SpellCriticalFailure;
}

// ── Energy Cost Credentials ───────────────────────────────────────────────────

proof_credential! {
    /// Witness that base energy cost was determined.
    pub BaseCostDetermined => BaseEnergyCostDetermined;

    /// Witness that skill-based cost reduction was applied.
    pub SkillReductionApplied => SkillBasedCostReductionApplied;

    /// Witness that final energy cost was calculated.
    pub FinalCostCalculated => FinalEnergyCostCalculated;

    /// Witness that energy was paid from caster.
    pub EnergyDeducted => EnergyPaidFromCaster;

    /// Witness that spell is being maintained.
    pub SpellKeptActive => SpellMaintained;

    /// Witness that maintenance energy was paid.
    pub MaintenancePaid => MaintenanceEnergyPaid;
}

// ── Spell Effect Credentials ──────────────────────────────────────────────────

proof_credential! {
    /// Witness that spell effect was applied.
    pub EffectApplied => SpellEffectApplied;

    /// Witness that spell target was determined.
    pub TargetDetermined => SpellTargetDetermined;

    /// Witness that spell range was checked.
    pub RangeChecked => SpellRangeChecked;

    /// Witness that spell duration was determined.
    pub DurationSet => SpellDurationDetermined;
}

// ── Spell Resistance Credentials ──────────────────────────────────────────────

proof_credential! {
    /// Witness that resistance roll was required.
    pub ResistanceNeeded => ResistanceRollRequired;

    /// Witness that resistance roll was made.
    pub ResistanceRolled => ResistanceRollMade;

    /// Witness that spell was successfully resisted.
    pub SpellResisted => SpellResistedSuccessfully;

    /// Witness that resistance was overcome.
    pub ResistanceBroken => ResistanceOvercome;
}

// ── Spell Modifier Credentials ────────────────────────────────────────────────

proof_credential! {
    /// Witness that size/speed modifier was applied.
    pub SizeSpeedApplied => SizeSpeedModifierApplied;

    /// Witness that time modifier was applied.
    pub TimeModApplied => TimeModifierApplied;

    /// Witness that environment modifier was applied.
    pub EnvironmentApplied => EnvironmentModifierApplied;
}

// ── Ceremonial Magic Credentials ──────────────────────────────────────────────

proof_credential! {
    /// Witness that ceremonial casting began.
    pub CeremonyStarted => CeremonialCastingBegun;

    /// Witness that energy was pooled from participants.
    pub EnergyPooled => EnergyPooledFromParticipants;

    /// Witness that ceremonial spell completed.
    pub CeremonyCompleted => CeremonialSpellCompleted;
}

// ── Combat Flow Credentials (Game State Management) ───────────────────────────

proof_credential! {
    /// Witness that combat has been initialized.
    pub CombatStarted => CombatInitialized;

    /// Witness that victory condition has been met.
    pub VictoryAchieved => VictoryConditionMet;

    /// Witness that combat has been concluded.
    pub CombatFinished => CombatConcluded;
}

// ── Turn Structure Credentials ────────────────────────────────────────────────

proof_credential! {
    /// Witness that turn order has been established.
    pub TurnOrderSet => TurnOrderEstablished;

    /// Witness that a turn has begun.
    pub TurnStarted => TurnBegan;

    /// Witness that a turn has ended.
    pub TurnCompleted => TurnEnded;

    /// Witness that a round has completed.
    pub RoundFinished => RoundCompleted;
}

// ── Action Economy Credentials ────────────────────────────────────────────────

proof_credential! {
    /// Witness that combatant can take action.
    pub ActionAvailable => CanTakeAction;

    /// Witness that a maneuver has been selected.
    pub ManeuverChosen => ManeuverSelected;

    /// Witness that an attack has been declared.
    pub AttackAnnounced => AttackDeclared;

    /// Witness that a defense is required.
    pub MustDefend => DefenseRequired;
}

// ── Action Resolution Credentials ─────────────────────────────────────────────

proof_credential! {
    /// Witness that an attack has been resolved.
    pub AttackCompleted => AttackResolved;

    /// Witness that a defense has been resolved.
    pub DefenseCompleted => DefenseResolved;

    /// Witness that damage can be applied.
    pub DamageReady => CanApplyDamage;

    /// Witness that damage has been applied.
    pub DamageDealt => DamageApplied;
}
