//! Composite proof structures — evidence bundles for complex GURPS operations.
//!
//! Evidence bundles aggregate multiple [`Established<P>`] proofs to represent
//! complete multi-step game operations. These bundles serve as credentials for
//! higher-level game systems.
//!
//! # Pattern
//!
//! Each evidence bundle:
//! 1. Contains multiple `Established<P>` fields
//! 2. Derives `Prop` to be usable as a proposition itself
//! 3. Can be used via `ProvableFrom` to mint higher-level proofs
//!
//! # Examples
//!
//! ```rust,ignore
//! // Attack resolution requires roll + outcome determination
//! pub struct AttackResolutionEvidence {
//!     pub roll_made: Established<AttackRollMade>,
//!     pub outcome: Established<AttackOutcomeDetermined>,
//! }
//! ```

use crate::contracts::combat::{
    AllOutAttackDeclared, AttackCriticalFailure, AttackCriticalSuccess, AttackFailed,
    AttackOutcomeDetermined, AttackRollMade, AttackSuccessful, BasicDamageCalculated,
    DamageResistanceApplied, DefenseCriticalFailure, DefenseCriticalSuccess, DefenseFailed,
    DefenseOutcomeDetermined, DefenseRollMade, DefenseSuccessful, DeceptiveAttackApplied,
    FeintSuccessful, HitLocationDetermined, InjuryApplied, InjuryCalculated,
    LocationMultiplierApplied, RapidStrikeExecuted, WeaponDamageRolled, WoundingModifierApplied,
};
use elicitation::contracts::Established;
use elicitation_derive::Prop;

// ── Attack Resolution Evidence ────────────────────────────────────────────────

/// Evidence that an attack roll was made and its outcome determined.
///
/// Composite proof establishing that:
/// 1. The attacker rolled 3d6
/// 2. The roll was compared to effective skill
///
/// This bundle does NOT indicate success/failure - only that the attack
/// was properly resolved. Use `AttackSuccessEvidence` or `AttackFailureEvidence`
/// for outcome-specific proofs.
///
/// # GURPS Rules
///
/// Attack resolution requires both rolling dice and comparing to skill.
/// This evidence bundle proves both steps occurred.
///
/// # Citations
///
/// BS 356 - Attack rolls
#[derive(Prop)]
pub struct AttackResolutionEvidence {
    /// Proof that 3d6 was rolled for the attack
    pub roll_made: Established<AttackRollMade>,
    /// Proof that the roll was compared to effective skill
    pub outcome: Established<AttackOutcomeDetermined>,
}

/// Evidence that an attack succeeded.
///
/// Composite proof establishing that:
/// 1. The attack was resolved (roll + comparison)
/// 2. The result was a success (roll ≤ effective skill)
///
/// This proves the attack hit, but does NOT prove it dealt damage -
/// the defender may still successfully defend.
///
/// # GURPS Rules
///
/// A successful attack may still be dodged, parried, or blocked.
///
/// # Citations
///
/// BS 356 - Successful attacks
#[derive(Prop)]
pub struct AttackSuccessEvidence {
    /// Proof of attack resolution
    pub resolution: AttackResolutionEvidence,
    /// Proof that the attack succeeded
    pub success: Established<AttackSuccessful>,
}

/// Evidence that an attack failed.
///
/// Composite proof establishing that:
/// 1. The attack was resolved (roll + comparison)
/// 2. The result was a failure (roll > effective skill)
///
/// This proves the attack missed automatically, no defense roll needed.
///
/// # GURPS Rules
///
/// Failed attacks miss completely.
///
/// # Citations
///
/// BS 356 - Failed attacks
#[derive(Prop)]
pub struct AttackFailureEvidence {
    /// Proof of attack resolution
    pub resolution: AttackResolutionEvidence,
    /// Proof that the attack failed
    pub failure: Established<AttackFailed>,
}

/// Evidence that an attack critically succeeded.
///
/// Composite proof establishing that:
/// 1. The attack succeeded
/// 2. The roll qualifies as a critical success (3-4, or 5-6 if skill ≥ 15)
///
/// # GURPS Rules
///
/// Critical successes grant special bonuses: +1 or +2 damage,
/// hit vital location, or other dramatic effects.
///
/// # Citations
///
/// BS 356-357 - Critical hits
#[derive(Prop)]
pub struct AttackCriticalSuccessEvidence {
    /// Proof of successful attack
    pub success: AttackSuccessEvidence,
    /// Proof of critical success
    pub critical: Established<AttackCriticalSuccess>,
}

/// Evidence that an attack critically failed.
///
/// Composite proof establishing that:
/// 1. The attack was resolved
/// 2. The roll qualifies as a critical failure (17-18, or margin ≥ 10)
///
/// # GURPS Rules
///
/// Critical failures cause mishaps: drop weapon, hit self/friend,
/// weapon breaks, etc.
///
/// # Citations
///
/// BS 356 - Critical misses
/// BS 557 - Critical miss table
#[derive(Prop)]
pub struct AttackCriticalFailureEvidence {
    /// Proof of attack resolution
    pub resolution: AttackResolutionEvidence,
    /// Proof of critical failure
    pub critical: Established<AttackCriticalFailure>,
}

// ── Defense Resolution Evidence ───────────────────────────────────────────────

/// Evidence that a defense roll was made and its outcome determined.
///
/// Composite proof establishing that:
/// 1. The defender rolled 3d6 for active defense
/// 2. The roll was compared to defense score (Dodge/Parry/Block)
///
/// This bundle does NOT indicate success/failure - only that the defense
/// was properly resolved.
///
/// # GURPS Rules
///
/// Active defenses require rolling 3d6 against the defense score.
///
/// # Citations
///
/// BS 374 - Active defenses
#[derive(Prop)]
pub struct DefenseResolutionEvidence {
    /// Proof that 3d6 was rolled for defense
    pub roll_made: Established<DefenseRollMade>,
    /// Proof that the roll was compared to defense score
    pub outcome: Established<DefenseOutcomeDetermined>,
}

/// Evidence that a defense succeeded.
///
/// Composite proof establishing that:
/// 1. The defense was resolved (roll + comparison)
/// 2. The result was a success (roll ≤ defense score)
///
/// This proves the attack was avoided - no damage occurs.
///
/// # GURPS Rules
///
/// Successful defenses completely avoid the attack.
///
/// # Citations
///
/// BS 374 - Successful defenses
#[derive(Prop)]
pub struct DefenseSuccessEvidence {
    /// Proof of defense resolution
    pub resolution: DefenseResolutionEvidence,
    /// Proof that the defense succeeded
    pub success: Established<DefenseSuccessful>,
}

/// Evidence that a defense failed.
///
/// Composite proof establishing that:
/// 1. The defense was resolved (roll + comparison)
/// 2. The result was a failure (roll > defense score)
///
/// This proves the attack hit - proceed to damage resolution.
///
/// # GURPS Rules
///
/// Failed defenses allow the attack to hit and deal damage.
///
/// # Citations
///
/// BS 374 - Failed defenses
#[derive(Prop)]
pub struct DefenseFailureEvidence {
    /// Proof of defense resolution
    pub resolution: DefenseResolutionEvidence,
    /// Proof that the defense failed
    pub failure: Established<DefenseFailed>,
}

/// Evidence that a defense critically succeeded.
///
/// Composite proof establishing that:
/// 1. The defense succeeded
/// 2. The roll qualifies as a critical success
///
/// # GURPS Rules
///
/// Critical defenses grant bonuses: +1 to future defenses,
/// immediate counter-attacks, or shield undamaged.
///
/// # Citations
///
/// BS 375 - Critical defense success
#[derive(Prop)]
pub struct DefenseCriticalSuccessEvidence {
    /// Proof of successful defense
    pub success: DefenseSuccessEvidence,
    /// Proof of critical success
    pub critical: Established<DefenseCriticalSuccess>,
}

/// Evidence that a defense critically failed.
///
/// Composite proof establishing that:
/// 1. The defense was resolved
/// 2. The roll qualifies as a critical failure
///
/// # GURPS Rules
///
/// Critical defense failures cause mishaps: fall down,
/// drop weapon/shield, etc.
///
/// # Citations
///
/// BS 375 - Critical defense failure
#[derive(Prop)]
pub struct DefenseCriticalFailureEvidence {
    /// Proof of defense resolution
    pub resolution: DefenseResolutionEvidence,
    /// Proof of critical failure
    pub critical: Established<DefenseCriticalFailure>,
}

// ── Damage Resolution Evidence ────────────────────────────────────────────────

/// Evidence that basic damage was calculated.
///
/// Composite proof establishing that:
/// 1. Weapon damage was rolled
/// 2. Defender's DR was applied
/// 3. Basic (pre-multiplier) damage was calculated
///
/// This represents penetrating damage before hit location multipliers.
///
/// # GURPS Rules
///
/// Basic damage = max(0, rolled_damage - DR)
///
/// # Citations
///
/// BS 378-379 - Damage calculation
#[derive(Prop)]
pub struct BasicDamageEvidence {
    /// Proof that weapon damage dice were rolled
    pub damage_rolled: Established<WeaponDamageRolled>,
    /// Proof that DR was subtracted
    pub dr_applied: Established<DamageResistanceApplied>,
    /// Proof that basic damage was calculated
    pub basic_calculated: Established<BasicDamageCalculated>,
}

/// Evidence that injury was fully calculated.
///
/// Composite proof establishing that:
/// 1. Basic damage was calculated
/// 2. Hit location was determined
/// 3. Location multiplier was applied
/// 4. Wounding modifier was applied
/// 5. Final injury to HP was calculated
///
/// This represents complete damage resolution ready to apply to HP.
///
/// # GURPS Rules
///
/// Final injury = basic_damage × location_mult × wounding_mult
///
/// # Citations
///
/// BS 378-380 - Complete injury calculation
#[derive(Prop)]
pub struct InjuryCalculationEvidence {
    /// Proof that basic damage was calculated
    pub basic_damage: BasicDamageEvidence,
    /// Proof that hit location was determined
    pub location: Established<HitLocationDetermined>,
    /// Proof that location multiplier was applied
    pub location_mult: Established<LocationMultiplierApplied>,
    /// Proof that wounding modifier was applied
    pub wounding_mult: Established<WoundingModifierApplied>,
    /// Proof that final injury was calculated
    pub injury_calculated: Established<InjuryCalculated>,
}

/// Evidence that injury was applied to character HP.
///
/// Composite proof establishing that:
/// 1. Injury was fully calculated
/// 2. The injury was subtracted from defender's current HP
///
/// This proves the combat sequence is complete and HP has been updated.
///
/// # GURPS Rules
///
/// Character HP reduces by injury amount. Check for consciousness,
/// knockdown, and death rolls as appropriate.
///
/// # Citations
///
/// BS 419-420 - HP and injury effects
#[derive(Prop)]
pub struct InjuryApplicationEvidence {
    /// Proof of complete injury calculation
    pub calculation: InjuryCalculationEvidence,
    /// Proof that injury was applied to HP
    pub applied: Established<InjuryApplied>,
}

// ── Complete Combat Resolution Evidence ───────────────────────────────────────

/// Evidence that a complete combat exchange was resolved.
///
/// Composite proof establishing that:
/// 1. Attack was successful
/// 2. Defense failed (attack hit)
/// 3. Injury was calculated and applied
///
/// This represents a full combat turn where an attack successfully
/// dealt damage to the defender.
///
/// # GURPS Rules
///
/// Complete combat flow:
/// 1. Attacker rolls to hit
/// 2. Defender rolls to defend
/// 3. If defense fails, calculate and apply damage
///
/// # Citations
///
/// BS 356-380 - Complete combat rules
#[derive(Prop)]
pub struct CombatHitEvidence {
    /// Proof that the attack succeeded
    pub attack: AttackSuccessEvidence,
    /// Proof that the defense failed
    pub defense: DefenseFailureEvidence,
    /// Proof that injury was applied
    pub injury: InjuryApplicationEvidence,
}

/// Evidence that a combat exchange resulted in no damage.
///
/// Composite proof establishing that:
/// 1. Attack was successful, OR
/// 2. Defense succeeded (attack avoided)
///
/// This represents a combat turn where no damage occurred, either
/// because the attack missed or was successfully defended.
///
/// # GURPS Rules
///
/// No damage occurs if:
/// - Attack roll fails, OR
/// - Defense roll succeeds
///
/// # Citations
///
/// BS 356-377 - Attack and defense rules
#[derive(Prop)]
pub struct CombatMissEvidence {
    /// Proof that attack was resolved (may be success or failure)
    pub attack: AttackResolutionEvidence,
    /// Proof that defense succeeded (if attack succeeded)
    pub defense: Option<DefenseSuccessEvidence>,
}

// ── Special Maneuver Evidence ─────────────────────────────────────────────────

/// Evidence that an All-Out Attack was executed.
///
/// Composite proof establishing that:
/// 1. All-Out Attack maneuver was declared
/// 2. Attack was resolved with All-Out Attack bonuses
/// 3. Attacker loses all defenses until next turn
///
/// # GURPS Rules
///
/// All-Out Attack provides offensive bonuses at the cost of defense.
///
/// # Citations
///
/// BS 365 - All-Out Attack
#[derive(Prop)]
pub struct AllOutAttackEvidence {
    /// Proof that All-Out Attack was declared
    pub declared: Established<AllOutAttackDeclared>,
    /// Proof of attack resolution
    pub attack: AttackResolutionEvidence,
}

/// Evidence that a Rapid Strike was executed.
///
/// Composite proof establishing that:
/// 1. Rapid Strike maneuver was used
/// 2. Multiple attacks were made at -6 penalty each
///
/// # GURPS Rules
///
/// Rapid Strike allows multiple attacks per turn at penalty.
///
/// # Citations
///
/// BS 370 - Rapid Strike
#[derive(Prop)]
pub struct RapidStrikeEvidence {
    /// Proof that Rapid Strike was executed
    pub rapid_strike: Established<RapidStrikeExecuted>,
    /// Proofs of each attack in the rapid strike
    pub attacks: Vec<AttackResolutionEvidence>,
}

/// Evidence that a Feint was successful.
///
/// Composite proof establishing that:
/// 1. Feint succeeded in Quick Contest
/// 2. Opponent's next defense is penalized by margin of victory
///
/// # GURPS Rules
///
/// Successful feints penalize opponent's next active defense.
///
/// # Citations
///
/// BS 365 - Feint
#[derive(Prop)]
pub struct FeintEvidence {
    /// Proof that feint succeeded
    pub feint: Established<FeintSuccessful>,
    /// Margin of victory (penalty to opponent's defense)
    pub margin: i32,
}

/// Evidence that a Deceptive Attack was used.
///
/// Composite proof establishing that:
/// 1. Attacker traded attack skill for defense penalties
/// 2. Attack was resolved at reduced skill
/// 3. Opponent's defense is penalized
///
/// # GURPS Rules
///
/// Deceptive Attack reduces attacker's skill to penalize opponent's defense.
///
/// # Citations
///
/// BS 369 - Deceptive Attack
#[derive(Prop)]
pub struct DeceptiveAttackEvidence {
    /// Proof that Deceptive Attack was applied
    pub deceptive: Established<DeceptiveAttackApplied>,
    /// Proof of attack resolution at reduced skill
    pub attack: AttackResolutionEvidence,
    /// Skill reduction taken by attacker
    pub skill_reduction: i32,
    /// Defense penalty imposed on opponent
    pub defense_penalty: i32,
}
