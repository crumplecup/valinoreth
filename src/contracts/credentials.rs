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

use crate::contracts::combat::{
    AimBonusApplied, AllOutAttackDeclared, AttackCriticalFailure, AttackCriticalSuccess,
    AttackFailed, AttackOutcomeDetermined, AttackRollMade, AttackSuccessful,
    BasicDamageCalculated, DamageResistanceApplied, DeceptiveAttackApplied,
    DefenseCriticalFailure, DefenseCriticalSuccess, DefenseFailed, DefenseOutcomeDetermined,
    DefenseRollMade, DefenseSuccessful, FeintSuccessful, HitLocationDetermined, InjuryApplied,
    InjuryCalculated, LocationMultiplierApplied, RapidStrikeExecuted, WeaponDamageRolled,
    WoundingModifierApplied,
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
