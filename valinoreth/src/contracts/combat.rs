//! Combat system propositions — GURPS Basic Set combat rules.
//!
//! Covers attack rolls, active defenses, damage resolution, critical hits/misses,
//! and hit location effects following GURPS 4th Edition Basic Set.
//!
//! # GURPS Rules
//!
//! Combat in GURPS is resolved through:
//! - Attack rolls: 3d6 vs. effective skill (skill + modifiers)
//! - Active defenses: Dodge/Parry/Block rolls vs. defense value
//! - Damage resolution: roll weapon damage, apply DR, calculate injury
//! - Critical outcomes: special results on rolls of 3-4 (success) or 17-18 (failure)
//! - Hit location: targeted attacks at penalties, with multipliers for injury
//!
//! # Citations
//!
//! - BS 356-358 - Attack rolls and modifiers
//! - BS 374-377 - Active defense (Dodge, Parry, Block)
//! - BS 378-380 - Damage and injury calculation
//! - BS 399-400 - Critical hits and misses
//! - BS 552 - Hit location table

use elicitation_derive::Prop;

// ── Attack Roll Propositions ──────────────────────────────────────────────────

/// 3d6 attack roll was executed.
///
/// Establishes that the attacker has rolled 3d6 to attempt an attack.
/// Does not indicate success or failure - only that the roll occurred.
///
/// # GURPS Rules
///
/// Attack rolls use 3d6 against effective skill (base skill + modifiers).
/// Roll ≤ effective skill succeeds, roll > effective skill fails.
///
/// # Citations
///
/// BS 356 - Attack rolls
#[derive(Prop)]
pub struct AttackRollMade;

/// Attack roll result compared against effective skill.
///
/// Establishes that the attack roll has been evaluated against the
/// attacker's effective skill to determine success or failure.
///
/// # GURPS Rules
///
/// - Success: roll ≤ effective skill
/// - Failure: roll > effective skill
/// - Margin of success: effective skill - roll
/// - Margin of failure: roll - effective skill
///
/// # Citations
///
/// BS 356 - Success rolls
#[derive(Prop)]
pub struct AttackOutcomeDetermined;

/// Attack succeeded (roll ≤ effective skill).
///
/// Establishes that the attack roll was less than or equal to the
/// attacker's effective skill, meaning the attack hits unless actively defended.
///
/// # GURPS Rules
///
/// A successful attack may still be defended against with Dodge, Parry, or Block.
/// The defender must make an active defense roll to avoid the hit.
///
/// # Citations
///
/// BS 356 - Attack success
#[derive(Prop)]
pub struct AttackSuccessful;

/// Attack failed (roll > effective skill).
///
/// Establishes that the attack roll exceeded the attacker's effective skill,
/// meaning the attack automatically misses regardless of defenses.
///
/// # GURPS Rules
///
/// A failed attack misses completely. No defense roll is needed.
///
/// # Citations
///
/// BS 356 - Attack failure
#[derive(Prop)]
pub struct AttackFailed;

/// Critical success on attack roll.
///
/// Establishes that the attack achieved a critical success.
///
/// # GURPS Rules
///
/// Critical success occurs when:
/// - Roll is 3 or 4, OR
/// - Roll is 5 or 6 AND effective skill is 15+
///
/// Effects:
/// - Melee: +1 or +2 to damage, OR hit vital location, OR weapon unready
/// - Ranged: +1 or +2 to damage, OR hit exact weak spot
///
/// # Citations
///
/// BS 356 - Critical success
/// BS 357 - Critical hit effects (table)
#[derive(Prop)]
pub struct AttackCriticalSuccess;

/// Critical failure on attack roll.
///
/// Establishes that the attack suffered a critical failure.
///
/// # GURPS Rules
///
/// Critical failure occurs when:
/// - Roll is 18, OR
/// - Roll is 17 AND effective skill < 16, OR
/// - Roll exceeds effective skill by 10+
///
/// Effects: Roll on Critical Miss Table (BS 557)
/// - Drop weapon, hit self, hit friend, weapon breaks, etc.
///
/// # Citations
///
/// BS 356 - Critical failure
/// BS 557 - Critical miss table
#[derive(Prop)]
pub struct AttackCriticalFailure;

// ── Active Defense Propositions ───────────────────────────────────────────────

/// Active defense roll was executed.
///
/// Establishes that the defender has rolled 3d6 to attempt an active defense
/// (Dodge, Parry, or Block) against a successful attack.
///
/// # GURPS Rules
///
/// Defenders may choose one active defense per attack:
/// - Dodge: 3d6 ≤ Dodge score (DX + 3 + bonuses)
/// - Parry: 3d6 ≤ Parry score (skill/2 + 3 + bonuses)
/// - Block: 3d6 ≤ Block score (skill/2 + 3 + bonuses)
///
/// # Citations
///
/// BS 374 - Active defenses
#[derive(Prop)]
pub struct DefenseRollMade;

/// Defense roll result compared against defense value.
///
/// Establishes that the defense roll has been evaluated against the
/// defender's defense score to determine success or failure.
///
/// # GURPS Rules
///
/// - Success: roll ≤ defense score → attack dodged/parried/blocked
/// - Failure: roll > defense score → attack hits, proceed to damage
///
/// # Citations
///
/// BS 374 - Defense rolls
#[derive(Prop)]
pub struct DefenseOutcomeDetermined;

/// Defense succeeded (attack avoided).
///
/// Establishes that the defender successfully dodged, parried, or blocked
/// the attack, meaning the attack does not hit and no damage is applied.
///
/// # GURPS Rules
///
/// A successful defense completely avoids the attack.
/// Combat continues without damage resolution.
///
/// # Citations
///
/// BS 374 - Successful defenses
#[derive(Prop)]
pub struct DefenseSuccessful;

/// Defense failed (attack hits).
///
/// Establishes that the defender failed to dodge, parry, or block,
/// meaning the attack hits and damage is applied.
///
/// # GURPS Rules
///
/// When defense fails, proceed to damage resolution.
/// Roll weapon damage and apply DR.
///
/// # Citations
///
/// BS 374 - Failed defenses
#[derive(Prop)]
pub struct DefenseFailed;

/// Critical success on defense roll.
///
/// Establishes that the defense achieved a critical success.
///
/// # GURPS Rules
///
/// Critical defense success occurs when:
/// - Roll is 3 or 4, OR
/// - Roll is 5 or 6 AND defense is 15+
///
/// Effects vary by defense type:
/// - Dodge: +1 to all active defenses until next turn
/// - Parry: may attempt immediate counter-attack
/// - Block: shield undamaged by crushing attacks
///
/// # Citations
///
/// BS 375 - Critical defense success
#[derive(Prop)]
pub struct DefenseCriticalSuccess;

/// Critical failure on defense roll.
///
/// Establishes that the defense suffered a critical failure.
///
/// # GURPS Rules
///
/// Critical defense failure occurs when:
/// - Roll is 18, OR
/// - Roll is 17 AND defense < 16, OR
/// - Roll exceeds defense by 10+
///
/// Effects: Defender falls down, drops shield/weapon, or similar mishap.
///
/// # Citations
///
/// BS 375 - Critical defense failure
#[derive(Prop)]
pub struct DefenseCriticalFailure;

// ── Damage Calculation Propositions ───────────────────────────────────────────

/// Weapon damage dice were rolled.
///
/// Establishes that the attacker has rolled damage dice for their weapon
/// (e.g., 1d6+2 for a shortsword, 2d6 for a greatsword).
///
/// # GURPS Rules
///
/// Each weapon has a damage rating expressed in dice notation:
/// - XdY = roll X dice with Y sides
/// - +Z = add Z to the result
/// - Thrust/Swing damage based on ST for melee weapons
///
/// # Citations
///
/// BS 269 - Damage rolls
/// BS 378 - Damage types
#[derive(Prop)]
pub struct WeaponDamageRolled;

/// Damage Resistance (DR) applied to damage.
///
/// Establishes that the defender's DR has been subtracted from the
/// raw damage to determine penetrating damage.
///
/// # GURPS Rules
///
/// DR is provided by armor and reduces damage:
/// - penetrating_damage = max(0, raw_damage - DR)
/// - Flexible armor has limitations vs. crushing damage
///
/// # Citations
///
/// BS 378 - Damage Resistance
/// BS 379 - Armor and DR
#[derive(Prop)]
pub struct DamageResistanceApplied;

/// Basic damage (injury before multipliers) calculated.
///
/// Establishes that the penetrating damage (damage - DR) has been
/// determined as the basic injury before hit location multipliers.
///
/// # GURPS Rules
///
/// Basic damage = raw damage - DR
/// This is the injury dealt to hit locations with ×1 multiplier.
///
/// # Citations
///
/// BS 378 - Penetrating damage
#[derive(Prop)]
pub struct BasicDamageCalculated;

/// Hit location determined.
///
/// Establishes which body location was struck, either randomly
/// (normal attack) or by deliberate targeting.
///
/// # GURPS Rules
///
/// - Random hit: Roll 3d6 on hit location table (BS 552)
/// - Targeted hit: Attacker specifies location at penalty
/// - Each location has a damage multiplier (e.g., skull ×4, vitals ×3)
///
/// # Citations
///
/// BS 398 - Hit location
/// BS 552 - Hit location table
#[derive(Prop)]
pub struct HitLocationDetermined;

/// Hit location damage multiplier applied.
///
/// Establishes that the basic damage has been multiplied by the
/// hit location multiplier to determine final injury.
///
/// # GURPS Rules
///
/// Each location has a multiplier:
/// - Skull: ×4
/// - Vitals: ×3 (impaling/piercing only)
/// - Torso: ×1
/// - Arms/Legs: ×1 (limb crippling rules apply)
///
/// # Citations
///
/// BS 398-400 - Hit location effects
#[derive(Prop)]
pub struct LocationMultiplierApplied;

/// Wounding modifier for damage type applied.
///
/// Establishes that the damage type's wounding modifier has been
/// applied to determine final injury.
///
/// # GURPS Rules
///
/// Damage type modifiers:
/// - Impaling: ×2 (×3 to vitals)
/// - Piercing: ×1.5 (×3 to vitals)
/// - Cutting: ×1.5
/// - Crushing: ×1
/// - Burning: ×1
///
/// # Citations
///
/// BS 378-379 - Damage types and wounding modifiers
#[derive(Prop)]
pub struct WoundingModifierApplied;

/// Final injury to Hit Points calculated.
///
/// Establishes that all modifiers (DR, location, wounding) have been
/// applied to determine the total HP loss from this attack.
///
/// # GURPS Rules
///
/// Final injury = (raw_damage - DR) × location_multiplier × wounding_modifier
/// This injury is subtracted from the defender's current HP.
///
/// # Citations
///
/// BS 378-380 - Injury calculation
#[derive(Prop)]
pub struct InjuryCalculated;

/// Injury applied to character's HP.
///
/// Establishes that the calculated injury has been subtracted from
/// the defender's Hit Points, updating their current HP total.
///
/// # GURPS Rules
///
/// Character HP reduces by injury amount:
/// - HP > 0: Conscious and functional
/// - HP ≤ 0: Roll to remain conscious
/// - HP ≤ -1×HP: Roll vs. HT or die
/// - HP ≤ -5×HP: Instant death
///
/// # Citations
///
/// BS 419-420 - Hit Points and injury
#[derive(Prop)]
pub struct InjuryApplied;

// ── Special Attack Propositions ───────────────────────────────────────────────

/// All-Out Attack maneuver declared.
///
/// Establishes that the attacker has chosen All-Out Attack, gaining
/// offensive bonuses but losing all active defenses until next turn.
///
/// # GURPS Rules
///
/// All-Out Attack options:
/// - Determined: +4 to hit
/// - Strong: +2 to damage (melee), +1 to damage (ranged)
/// - Double: Attack twice at no penalty
/// - Feint: Feint and attack in same turn
///
/// Penalty: No active defenses until next turn.
///
/// # Citations
///
/// BS 365 - All-Out Attack
#[derive(Prop)]
pub struct AllOutAttackDeclared;

/// Aim maneuver accumulated (ranged attacks).
///
/// Establishes that the attacker has spent a turn aiming a ranged
/// weapon, gaining accuracy bonuses.
///
/// # GURPS Rules
///
/// Aim bonuses:
/// - 1 turn: +Accuracy of weapon
/// - 2 turns: +Accuracy + 1
/// - 3+ turns: +Accuracy + 2 (max)
///
/// # Citations
///
/// BS 364 - Aim
#[derive(Prop)]
pub struct AimBonusApplied;

/// Rapid Strike executed (multiple attacks at penalty).
///
/// Establishes that the attacker has made multiple attacks in one turn
/// using the Rapid Strike maneuver, each at -6 to skill (or -3 with training).
///
/// # GURPS Rules
///
/// Rapid Strike allows multiple attacks per turn:
/// - Each attack at -6 to skill
/// - -3 if attacker has Weapon Master or similar
/// - May attack different targets
///
/// # Citations
///
/// BS 370 - Rapid Strike
#[derive(Prop)]
pub struct RapidStrikeExecuted;

/// Feint successful (defense penalty applied to opponent).
///
/// Establishes that a Feint maneuver succeeded in faking out the
/// opponent, reducing their active defense.
///
/// # GURPS Rules
///
/// Feint: Quick Contest of attacker's skill vs. defender's skill
/// - Margin of victory = penalty to defender's next active defense
/// - Defender may choose to ignore feint and attack instead
///
/// # Citations
///
/// BS 365 - Feint
#[derive(Prop)]
pub struct FeintSuccessful;

/// Deceptive Attack penalty applied (harder to defend).
///
/// Establishes that the attacker traded attack skill for defense penalties,
/// making the attack harder to defend against.
///
/// # GURPS Rules
///
/// Deceptive Attack:
/// - Attacker reduces their skill by X (before roll)
/// - Defender's defense reduced by X/2 (round down)
/// - Maximum penalty: attacker's skill - 10
///
/// # Citations
///
/// BS 369 - Deceptive Attack
#[derive(Prop)]
pub struct DeceptiveAttackApplied;
