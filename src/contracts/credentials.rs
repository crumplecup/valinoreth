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
    AdvantageLevelValid, AdvantageModifiersCostCalculated, AdvantagePurchased,
    AdvantagePrerequisiteMet, AttributeCostCalculated, AttributePurchased,
    AttributesMeetCampaignMinimums, BasicMoveCalculated, BasicSpeedCalculated, CharacterComplete,
    CharacterIdentified, CharacterPointValueSet, CharacterValid, DisadvantageLevelValid,
    DisadvantageTaken, DisadvantagePointLimitRespected, DisadvantagesNotConflicting,
    DodgeCalculated, FatiguePointsSet, HitPointsSet, PerceptionSet, PointBudgetBalanced,
    QuirkLimitRespected, QuirkTaken, RacialTemplateRequirementsMet, SecondaryCharacteristicPurchased,
    SelfControlRollSpecified, WillSet,
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
use crate::contracts::combat::{
    AimBonusApplied, AllOutAttackDeclared, AttackCriticalFailure, AttackCriticalSuccess,
    AttackFailed, AttackOutcomeDetermined, AttackRollMade, AttackSuccessful,
    BasicDamageCalculated, DamageResistanceApplied, DeceptiveAttackApplied,
    DefenseCriticalFailure, DefenseCriticalSuccess, DefenseFailed, DefenseOutcomeDetermined,
    DefenseRollMade, DefenseSuccessful, FeintSuccessful, HitLocationDetermined, InjuryApplied,
    InjuryCalculated, LocationMultiplierApplied, RapidStrikeExecuted, WeaponDamageRolled,
    WoundingModifierApplied,
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
    pub(crate) ValidAttackRoll => AttackRollMade;

    /// Witness that attack roll was compared to effective skill.
    ///
    /// Only constructible by code that performed the comparison.
    pub(crate) AttackOutcomeChecked => AttackOutcomeDetermined;

    /// Witness that attack succeeded (roll ≤ skill).
    ///
    /// Only constructible when roll result confirms success.
    pub(crate) AttackHit => AttackSuccessful;

    /// Witness that attack failed (roll > skill).
    ///
    /// Only constructible when roll result confirms failure.
    pub(crate) AttackMiss => AttackFailed;

    /// Witness that attack achieved critical success.
    ///
    /// Only constructible when roll meets critical success criteria.
    pub(crate) AttackCriticalHit => AttackCriticalSuccess;

    /// Witness that attack suffered critical failure.
    ///
    /// Only constructible when roll meets critical failure criteria.
    pub(crate) AttackCriticalMiss => AttackCriticalFailure;
}

// ── Defense Roll Credentials ──────────────────────────────────────────────────

proof_credential! {
    /// Witness that 3d6 was rolled for defense.
    pub(crate) ValidDefenseRoll => DefenseRollMade;

    /// Witness that defense roll was compared to defense score.
    pub(crate) DefenseOutcomeChecked => DefenseOutcomeDetermined;

    /// Witness that defense succeeded (roll ≤ defense score).
    pub(crate) DefenseWorked => DefenseSuccessful;

    /// Witness that defense failed (roll > defense score).
    pub(crate) DefenseDidNotWork => DefenseFailed;

    /// Witness that defense achieved critical success.
    pub(crate) DefenseCriticalWin => DefenseCriticalSuccess;

    /// Witness that defense suffered critical failure.
    pub(crate) DefenseCriticalMiss => DefenseCriticalFailure;
}

// ── Damage Calculation Credentials ────────────────────────────────────────────

proof_credential! {
    /// Witness that weapon damage dice were rolled.
    pub(crate) ValidDamageRoll => WeaponDamageRolled;

    /// Witness that Damage Resistance was subtracted from damage.
    pub(crate) DrSubtracted => DamageResistanceApplied;

    /// Witness that basic (pre-multiplier) damage was calculated.
    pub(crate) BasicDamageComputed => BasicDamageCalculated;

    /// Witness that hit location was determined (rolled or targeted).
    pub(crate) LocationDetermined => HitLocationDetermined;

    /// Witness that hit location damage multiplier was applied.
    pub(crate) LocationMultiplierComputed => LocationMultiplierApplied;

    /// Witness that wounding modifier for damage type was applied.
    pub(crate) WoundingMultiplierComputed => WoundingModifierApplied;

    /// Witness that final injury to HP was calculated.
    pub(crate) InjuryComputed => InjuryCalculated;

    /// Witness that injury was subtracted from character's HP.
    pub(crate) InjurySubtracted => InjuryApplied;
}

// ── Special Maneuver Credentials ──────────────────────────────────────────────

proof_credential! {
    /// Witness that All-Out Attack maneuver was declared.
    pub(crate) AllOutAttackChosen => AllOutAttackDeclared;

    /// Witness that Aim maneuver accumulated accuracy bonus.
    pub(crate) AimAccumulated => AimBonusApplied;

    /// Witness that Feint succeeded in Quick Contest.
    pub(crate) FeintWon => FeintSuccessful;

    /// Witness that Deceptive Attack penalty was applied.
    pub(crate) DeceptiveApplied => DeceptiveAttackApplied;

    /// Witness that Rapid Strike was executed.
    pub(crate) RapidStrikePerformed => RapidStrikeExecuted;
}

// ── Helper Types ──────────────────────────────────────────────────────────────

/// Validated dice roll result.
///
/// Contains the roll value and serves as evidence that dice were actually rolled.
/// Only constructible by code that performed the roll.
#[derive(Debug, Clone, Copy)]
pub(crate) struct DiceRollResult {
    /// The sum of 3d6
    pub value: i32,
}

impl DiceRollResult {
    /// Construct from a validated roll.
    ///
    /// # Safety
    ///
    /// Caller must ensure `value` is an actual 3d6 roll (3-18).
    pub(crate) fn new(value: i32) -> Self {
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
pub(crate) struct SkillCheckResult {
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
    pub(crate) fn new(roll: i32, skill: i32) -> Self {
        let success = roll <= skill;
        let margin = if success {
            skill - roll
        } else {
            roll - skill
        };
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
    pub(crate) fn is_critical_success(&self) -> bool {
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
    pub(crate) fn is_critical_failure(&self) -> bool {
        self.roll >= 18 || (self.roll >= 17 && self.skill < 16) || (!self.success && self.margin >= 10)
    }
}

/// Validated damage calculation.
///
/// Contains damage components and serves as evidence that damage was calculated.
#[derive(Debug, Clone, Copy)]
pub(crate) struct DamageCalculation {
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
    pub(crate) fn new(
        raw: i32,
        dr: i32,
        location_mult: f32,
        wounding_mult: f32,
    ) -> Self {
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
    pub(crate) ValidSkillCheckRoll => SkillCheckRollMade;

    /// Witness that skill check roll was compared to effective skill.
    pub(crate) SkillCheckOutcomeChecked => SkillCheckOutcomeDetermined;

    /// Witness that skill check succeeded (roll ≤ skill).
    pub(crate) SkillCheckHit => SkillCheckSuccessful;

    /// Witness that skill check failed (roll > skill).
    pub(crate) SkillCheckMiss => SkillCheckFailed;

    /// Witness that skill check achieved critical success.
    pub(crate) SkillCheckCriticalHit => SkillCheckCriticalSuccess;

    /// Witness that skill check suffered critical failure.
    pub(crate) SkillCheckCriticalMiss => SkillCheckCriticalFailure;
}

// ── Skill Default Credentials ─────────────────────────────────────────────────

proof_credential! {
    /// Witness that skill defaulted to an attribute.
    pub(crate) AttributeDefaultUsed => SkillDefaultedToAttribute;

    /// Witness that skill defaulted to a related skill.
    pub(crate) RelatedSkillDefaultUsed => SkillDefaultedToRelatedSkill;

    /// Witness that default penalty was applied.
    pub(crate) DefaultPenaltyAppliedCredential => DefaultPenaltyApplied;
}

// ── Skill Modifier Credentials ────────────────────────────────────────────────

proof_credential! {
    /// Witness that situational modifier was applied.
    pub(crate) SituationalModApplied => SituationalModifierApplied;

    /// Witness that task difficulty modifier was applied.
    pub(crate) TaskDifficultyApplied => TaskDifficultyModifierApplied;

    /// Witness that time spent modifier was applied.
    pub(crate) TimeSpentApplied => TimeSpentModifierApplied;

    /// Witness that complementary skill bonus was applied.
    pub(crate) ComplementaryBonusApplied => ComplementarySkillBonusApplied;

    /// Witness that familiarity penalty was applied.
    pub(crate) FamiliarityPenaltyAppliedCredential => FamiliarityPenaltyApplied;
}

// ── Skill Improvement Credentials ─────────────────────────────────────────────

proof_credential! {
    /// Witness that character points were spent on skill.
    pub(crate) PointsSpentOnSkill => CharacterPointsSpentOnSkill;

    /// Witness that skill level was increased.
    pub(crate) SkillLevelRaised => SkillLevelIncreased;

    /// Witness that skill prerequisite was met.
    pub(crate) PrerequisiteMet => SkillPrerequisiteMet;

    /// Witness that skill point budget is valid.
    pub(crate) SkillBudgetValid => SkillPointBudgetValid;
}

// ── Special Skill Usage Credentials ───────────────────────────────────────────

proof_credential! {
    /// Witness that wildcard skill was used.
    pub(crate) WildcardUsed => WildcardSkillUsed;

    /// Witness that technique was used.
    pub(crate) TechniqueApplied => TechniqueUsed;

    /// Witness that skill contest was resolved.
    pub(crate) ContestResolved => SkillContestResolved;

    /// Witness that contest winner was determined.
    pub(crate) WinnerDetermined => ContestWinnerDetermined;
}

// ── Character Point Budget Credentials ────────────────────────────────────────

proof_credential! {
    /// Witness that character point value was set.
    pub(crate) PointValueSet => CharacterPointValueSet;

    /// Witness that point budget is balanced.
    pub(crate) BudgetBalanced => PointBudgetBalanced;

    /// Witness that disadvantage point limit is respected.
    pub(crate) DisadvantageLimitRespected => DisadvantagePointLimitRespected;
}

// ── Attribute Purchase Credentials ────────────────────────────────────────────

proof_credential! {
    /// Witness that attribute was purchased.
    pub(crate) AttributeBought => AttributePurchased;

    /// Witness that attribute cost was calculated correctly.
    pub(crate) AttributeCostComputed => AttributeCostCalculated;

    /// Witness that secondary characteristic was purchased.
    pub(crate) SecondaryCharacteristicBought => SecondaryCharacteristicPurchased;
}

// ── Advantage Selection Credentials ───────────────────────────────────────────

proof_credential! {
    /// Witness that advantage was purchased.
    pub(crate) AdvantageBought => AdvantagePurchased;

    /// Witness that advantage modifiers were calculated.
    pub(crate) AdvantageModifiersComputed => AdvantageModifiersCostCalculated;

    /// Witness that advantage level is valid.
    pub(crate) AdvantageLevelValidated => AdvantageLevelValid;

    /// Witness that advantage prerequisite was met.
    pub(crate) AdvantagePrereqMet => AdvantagePrerequisiteMet;
}

// ── Disadvantage Selection Credentials ────────────────────────────────────────

proof_credential! {
    /// Witness that disadvantage was taken.
    pub(crate) DisadvantageTakenCredential => DisadvantageTaken;

    /// Witness that self-control roll was specified.
    pub(crate) SelfControlSpecified => SelfControlRollSpecified;

    /// Witness that disadvantage level is valid.
    pub(crate) DisadvantageLevelValidated => DisadvantageLevelValid;

    /// Witness that disadvantages do not conflict.
    pub(crate) NoConflicts => DisadvantagesNotConflicting;
}

// ── Quirk Credentials ─────────────────────────────────────────────────────────

proof_credential! {
    /// Witness that quirk was taken.
    pub(crate) QuirkTakenCredential => QuirkTaken;

    /// Witness that quirk limit is respected.
    pub(crate) QuirkLimitRespectedCredential => QuirkLimitRespected;
}

// ── Derived Statistics Credentials ────────────────────────────────────────────

proof_credential! {
    /// Witness that Basic Speed was calculated.
    pub(crate) BasicSpeedComputed => BasicSpeedCalculated;

    /// Witness that Basic Move was calculated.
    pub(crate) BasicMoveComputed => BasicMoveCalculated;

    /// Witness that Dodge was calculated.
    pub(crate) DodgeComputed => DodgeCalculated;

    /// Witness that HP was set.
    pub(crate) HpSet => HitPointsSet;

    /// Witness that Will was set.
    pub(crate) WillSetCredential => WillSet;

    /// Witness that Perception was set.
    pub(crate) PerceptionSetCredential => PerceptionSet;

    /// Witness that FP was set.
    pub(crate) FpSet => FatiguePointsSet;
}

// ── Character Validity Credentials ────────────────────────────────────────────

proof_credential! {
    /// Witness that attributes meet campaign minimums.
    pub(crate) MinimumsMet => AttributesMeetCampaignMinimums;

    /// Witness that racial template requirements were met.
    pub(crate) RacialRequirementsMet => RacialTemplateRequirementsMet;

    /// Witness that character is identified.
    pub(crate) CharacterNamed => CharacterIdentified;

    /// Witness that character is complete.
    pub(crate) CharacterCompleteCredential => CharacterComplete;

    /// Witness that character is valid.
    pub(crate) CharacterValidated => CharacterValid;
}

// ── Spell Learning Credentials ────────────────────────────────────────────────

proof_credential! {
    /// Witness that spell prerequisites were met.
    pub(crate) PrerequisitesMet => SpellPrerequisitesMet;

    /// Witness that Magery requirement was met.
    pub(crate) MageryMet => MageryRequirementMet;

    /// Witness that spell was learned.
    pub(crate) SpellAcquired => SpellLearned;

    /// Witness that spell skill level was set.
    pub(crate) SpellSkillSet => SpellSkillLevelSet;
}

// ── Spell Casting Credentials ─────────────────────────────────────────────────

proof_credential! {
    /// Witness that concentration began.
    pub(crate) ConcentrationStarted => ConcentrationBegun;

    /// Witness that concentration was maintained.
    pub(crate) ConcentrationKept => ConcentrationMaintained;

    /// Witness that concentration completed.
    pub(crate) ConcentrationFinished => ConcentrationCompleted;

    /// Witness that spell skill roll was made.
    pub(crate) SpellRollMade => SpellSkillRollMade;

    /// Witness that spell casting outcome was determined.
    pub(crate) CastingOutcomeChecked => SpellCastingOutcomeDetermined;

    /// Witness that spell casting succeeded.
    pub(crate) CastingSucceeded => SpellCastingSucceeded;

    /// Witness that spell casting failed.
    pub(crate) CastingFailed => SpellCastingFailed;

    /// Witness that spell critically succeeded.
    pub(crate) SpellCritHit => SpellCriticalSuccess;

    /// Witness that spell critically failed.
    pub(crate) SpellCritMiss => SpellCriticalFailure;
}

// ── Energy Cost Credentials ───────────────────────────────────────────────────

proof_credential! {
    /// Witness that base energy cost was determined.
    pub(crate) BaseCostDetermined => BaseEnergyCostDetermined;

    /// Witness that skill-based cost reduction was applied.
    pub(crate) SkillReductionApplied => SkillBasedCostReductionApplied;

    /// Witness that final energy cost was calculated.
    pub(crate) FinalCostCalculated => FinalEnergyCostCalculated;

    /// Witness that energy was paid from caster.
    pub(crate) EnergyDeducted => EnergyPaidFromCaster;

    /// Witness that spell is being maintained.
    pub(crate) SpellKeptActive => SpellMaintained;

    /// Witness that maintenance energy was paid.
    pub(crate) MaintenancePaid => MaintenanceEnergyPaid;
}

// ── Spell Effect Credentials ──────────────────────────────────────────────────

proof_credential! {
    /// Witness that spell effect was applied.
    pub(crate) EffectApplied => SpellEffectApplied;

    /// Witness that spell target was determined.
    pub(crate) TargetDetermined => SpellTargetDetermined;

    /// Witness that spell range was checked.
    pub(crate) RangeChecked => SpellRangeChecked;

    /// Witness that spell duration was determined.
    pub(crate) DurationSet => SpellDurationDetermined;
}

// ── Spell Resistance Credentials ──────────────────────────────────────────────

proof_credential! {
    /// Witness that resistance roll was required.
    pub(crate) ResistanceNeeded => ResistanceRollRequired;

    /// Witness that resistance roll was made.
    pub(crate) ResistanceRolled => ResistanceRollMade;

    /// Witness that spell was successfully resisted.
    pub(crate) SpellResisted => SpellResistedSuccessfully;

    /// Witness that resistance was overcome.
    pub(crate) ResistanceBroken => ResistanceOvercome;
}

// ── Spell Modifier Credentials ────────────────────────────────────────────────

proof_credential! {
    /// Witness that size/speed modifier was applied.
    pub(crate) SizeSpeedApplied => SizeSpeedModifierApplied;

    /// Witness that time modifier was applied.
    pub(crate) TimeModApplied => TimeModifierApplied;

    /// Witness that environment modifier was applied.
    pub(crate) EnvironmentApplied => EnvironmentModifierApplied;
}

// ── Ceremonial Magic Credentials ──────────────────────────────────────────────

proof_credential! {
    /// Witness that ceremonial casting began.
    pub(crate) CeremonyStarted => CeremonialCastingBegun;

    /// Witness that energy was pooled from participants.
    pub(crate) EnergyPooled => EnergyPooledFromParticipants;

    /// Witness that ceremonial spell completed.
    pub(crate) CeremonyCompleted => CeremonialSpellCompleted;
}
