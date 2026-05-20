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

use crate::contracts::character::{
    AdvantageLevelValid, AdvantageModifiersCostCalculated, AdvantagePurchased,
    AttributeCostCalculated, AttributePurchased, AttributesMeetCampaignMinimums, BasicMoveCalculated,
    BasicSpeedCalculated, CharacterComplete, CharacterIdentified, CharacterValid,
    DisadvantageLevelValid, DisadvantageTaken, DisadvantagesNotConflicting, DodgeCalculated,
    FatiguePointsSet, HitPointsSet, PerceptionSet, PointBudgetBalanced,
    SecondaryCharacteristicPurchased, WillSet,
};
use crate::contracts::combat::{
    AllOutAttackDeclared, AttackCriticalFailure, AttackCriticalSuccess, AttackFailed,
    AttackOutcomeDetermined, AttackRollMade, AttackSuccessful, BasicDamageCalculated,
    DamageResistanceApplied, DefenseCriticalFailure, DefenseCriticalSuccess, DefenseFailed,
    DefenseOutcomeDetermined, DefenseRollMade, DefenseSuccessful, DeceptiveAttackApplied,
    FeintSuccessful, HitLocationDetermined, InjuryApplied, InjuryCalculated,
    LocationMultiplierApplied, RapidStrikeExecuted, WeaponDamageRolled, WoundingModifierApplied,
};
use crate::contracts::skills::{
    CharacterPointsSpentOnSkill, ComplementarySkillBonusApplied, DefaultPenaltyApplied,
    SituationalModifierApplied, SkillCheckCriticalFailure, SkillCheckCriticalSuccess,
    SkillCheckFailed, SkillCheckOutcomeDetermined, SkillCheckRollMade, SkillCheckSuccessful,
    SkillDefaultedToAttribute, SkillDefaultedToRelatedSkill, SkillLevelIncreased,
    SkillPrerequisiteMet, TaskDifficultyModifierApplied, TechniqueUsed, TimeSpentModifierApplied,
    WildcardSkillUsed,
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

// ── Skill Check Evidence ──────────────────────────────────────────────────────

/// Evidence that a skill check was made and its outcome determined.
///
/// Composite proof establishing that:
/// 1. The character rolled 3d6
/// 2. The roll was compared to effective skill
///
/// This bundle does NOT indicate success/failure - only that the skill check
/// was properly resolved.
///
/// # GURPS Rules
///
/// Skill checks require rolling 3d6 against effective skill.
///
/// # Citations
///
/// BS 171 - Skill checks
/// BS 344 - Success rolls
#[derive(Prop)]
pub struct SkillCheckResolutionEvidence {
    /// Proof that 3d6 was rolled for the skill check
    pub roll_made: Established<SkillCheckRollMade>,
    /// Proof that the roll was compared to effective skill
    pub outcome: Established<SkillCheckOutcomeDetermined>,
}

/// Evidence that a skill check succeeded.
///
/// Composite proof establishing that:
/// 1. The skill check was resolved (roll + comparison)
/// 2. The result was a success (roll ≤ effective skill)
///
/// # GURPS Rules
///
/// Successful skill checks accomplish the attempted task.
///
/// # Citations
///
/// BS 344 - Successful rolls
#[derive(Prop)]
pub struct SkillCheckSuccessEvidence {
    /// Proof of skill check resolution
    pub resolution: SkillCheckResolutionEvidence,
    /// Proof that the skill check succeeded
    pub success: Established<SkillCheckSuccessful>,
}

/// Evidence that a skill check failed.
///
/// Composite proof establishing that:
/// 1. The skill check was resolved (roll + comparison)
/// 2. The result was a failure (roll > effective skill)
///
/// # GURPS Rules
///
/// Failed skill checks do not accomplish the attempted task.
///
/// # Citations
///
/// BS 344 - Failed rolls
#[derive(Prop)]
pub struct SkillCheckFailureEvidence {
    /// Proof of skill check resolution
    pub resolution: SkillCheckResolutionEvidence,
    /// Proof that the skill check failed
    pub failure: Established<SkillCheckFailed>,
}

/// Evidence that a skill check critically succeeded.
///
/// Composite proof establishing that:
/// 1. The skill check succeeded
/// 2. The roll qualifies as a critical success (3-4, or 5-6 if skill ≥ 15)
///
/// # GURPS Rules
///
/// Critical successes grant exceptional results with additional benefits.
///
/// # Citations
///
/// BS 346 - Critical success
#[derive(Prop)]
pub struct SkillCheckCriticalSuccessEvidence {
    /// Proof of successful skill check
    pub success: SkillCheckSuccessEvidence,
    /// Proof of critical success
    pub critical: Established<SkillCheckCriticalSuccess>,
}

/// Evidence that a skill check critically failed.
///
/// Composite proof establishing that:
/// 1. The skill check was resolved
/// 2. The roll qualifies as a critical failure (17-18, or margin ≥ 10)
///
/// # GURPS Rules
///
/// Critical failures cause catastrophic mishaps with serious consequences.
///
/// # Citations
///
/// BS 346 - Critical failure
#[derive(Prop)]
pub struct SkillCheckCriticalFailureEvidence {
    /// Proof of skill check resolution
    pub resolution: SkillCheckResolutionEvidence,
    /// Proof of critical failure
    pub critical: Established<SkillCheckCriticalFailure>,
}

// ── Skill Default Evidence ────────────────────────────────────────────────────

/// Evidence that a skill was defaulted to an attribute.
///
/// Composite proof establishing that:
/// 1. Character does not have the skill trained
/// 2. Skill defaults to an attribute (DX, IQ, etc.)
/// 3. Default penalty was applied
///
/// # GURPS Rules
///
/// Most skills can default to an attribute at a penalty.
///
/// # Citations
///
/// BS 170 - Defaults
#[derive(Prop)]
pub struct SkillAttributeDefaultEvidence {
    /// Proof that skill defaulted to attribute
    pub defaulted: Established<SkillDefaultedToAttribute>,
    /// Proof that default penalty was applied
    pub penalty: Established<DefaultPenaltyApplied>,
}

/// Evidence that a skill was defaulted to a related skill.
///
/// Composite proof establishing that:
/// 1. Character does not have the skill trained
/// 2. Skill defaults to a related skill
/// 3. Default penalty was applied
///
/// # GURPS Rules
///
/// Some skills can default to related skills at a penalty.
///
/// # Citations
///
/// BS 170 - Defaults
#[derive(Prop)]
pub struct SkillRelatedDefaultEvidence {
    /// Proof that skill defaulted to related skill
    pub defaulted: Established<SkillDefaultedToRelatedSkill>,
    /// Proof that default penalty was applied
    pub penalty: Established<DefaultPenaltyApplied>,
}

// ── Skill Modifiers Evidence ──────────────────────────────────────────────────

/// Evidence that modifiers were applied to a skill check.
///
/// Composite proof establishing that various modifiers (situational,
/// difficulty, time spent) were correctly applied to the effective skill.
///
/// # GURPS Rules
///
/// Effective skill = base skill + modifiers - penalties
///
/// # Citations
///
/// BS 345 - Modifiers
#[derive(Prop)]
pub struct SkillModifiersEvidence {
    /// Proof that situational modifiers were applied
    pub situational: Option<Established<SituationalModifierApplied>>,
    /// Proof that task difficulty was applied
    pub difficulty: Option<Established<TaskDifficultyModifierApplied>>,
    /// Proof that time spent modifier was applied
    pub time_spent: Option<Established<TimeSpentModifierApplied>>,
}

// ── Skill Improvement Evidence ────────────────────────────────────────────────

/// Evidence that a skill was improved.
///
/// Composite proof establishing that:
/// 1. Character points were spent on the skill
/// 2. The skill level was increased
/// 3. Prerequisites were met (if any)
///
/// # GURPS Rules
///
/// Skills improve by spending character points based on difficulty.
///
/// # Citations
///
/// BS 170 - Improving skills
#[derive(Prop)]
pub struct SkillImprovementEvidence {
    /// Proof that character points were spent
    pub points_spent: Established<CharacterPointsSpentOnSkill>,
    /// Proof that skill level increased
    pub level_increased: Established<SkillLevelIncreased>,
    /// Proof that prerequisites were met (if applicable)
    pub prerequisite: Option<Established<SkillPrerequisiteMet>>,
}

// ── Special Skill Usage Evidence ──────────────────────────────────────────────

/// Evidence that a complementary skill provided a bonus.
///
/// Composite proof establishing that:
/// 1. Character has a related complementary skill
/// 2. Complementary bonus was applied to main skill check
///
/// # GURPS Rules
///
/// Related skills can provide +1 to +4 bonuses.
///
/// # Citations
///
/// BS 346 - Complementary skills
#[derive(Prop)]
pub struct ComplementarySkillEvidence {
    /// Proof that complementary bonus was applied
    pub bonus: Established<ComplementarySkillBonusApplied>,
    /// Bonus amount (+1 to +4)
    pub bonus_amount: i32,
}

/// Evidence that a wildcard skill was used.
///
/// Composite proof establishing that:
/// 1. Character has wildcard skill (Detective!, Soldier!, etc.)
/// 2. Wildcard covers the attempted task at full level
///
/// # GURPS Rules
///
/// Wildcard skills cover multiple related skills at full level.
///
/// # Citations
///
/// BS 175 - Wildcard skills
#[derive(Prop)]
pub struct WildcardSkillEvidence {
    /// Proof that wildcard skill was used
    pub wildcard: Established<WildcardSkillUsed>,
    /// Name of the wildcard skill
    pub skill_name: String,
}

/// Evidence that a technique was used.
///
/// Composite proof establishing that:
/// 1. Character attempted a technique (specialized maneuver)
/// 2. Technique default or training bonus was applied
///
/// # GURPS Rules
///
/// Techniques default from base skills and can be trained.
///
/// # Citations
///
/// BS 230-232 - Techniques
#[derive(Prop)]
pub struct TechniqueUsageEvidence {
    /// Proof that technique was used
    pub technique: Established<TechniqueUsed>,
    /// Technique name
    pub technique_name: String,
    /// Default penalty or training bonus
    pub modifier: i32,
}

// ── Character Creation Evidence ───────────────────────────────────────────────

/// Evidence that an attribute was purchased.
///
/// Composite proof establishing that:
/// 1. Attribute (ST, DX, IQ, HT) was set to a valid level
/// 2. Point cost was calculated correctly
///
/// # GURPS Rules
///
/// Attributes cost 10 or 20 points per level based on attribute type.
///
/// # Citations
///
/// BS 14-16 - Attributes
#[derive(Prop)]
pub struct AttributePurchaseEvidence {
    /// Proof that attribute was purchased
    pub purchased: Established<AttributePurchased>,
    /// Proof that cost was calculated correctly
    pub cost: Established<AttributeCostCalculated>,
}

/// Evidence that a secondary characteristic was purchased.
///
/// Composite proof establishing that a secondary characteristic
/// (HP, Will, Per, FP, Basic Speed, Basic Move) was purchased
/// independently from its base attribute.
///
/// # GURPS Rules
///
/// Secondary characteristics default to attributes but can be purchased.
///
/// # Citations
///
/// BS 16-17 - Secondary characteristics
#[derive(Prop)]
pub struct SecondaryCharacteristicEvidence {
    /// Proof that secondary characteristic was purchased
    pub purchased: Established<SecondaryCharacteristicPurchased>,
}

/// Evidence that an advantage was purchased.
///
/// Composite proof establishing that:
/// 1. Advantage was purchased at its listed cost
/// 2. Modifiers were correctly applied (if any)
/// 3. Level is valid (if leveled advantage)
///
/// # GURPS Rules
///
/// Advantages provide benefits at fixed or variable point costs.
///
/// # Citations
///
/// BS 100-132 - Advantages
#[derive(Prop)]
pub struct AdvantagePurchaseEvidence {
    /// Proof that advantage was purchased
    pub purchased: Established<AdvantagePurchased>,
    /// Proof that modifiers were calculated (if applicable)
    pub modifiers: Option<Established<AdvantageModifiersCostCalculated>>,
    /// Proof that level is valid (if applicable)
    pub level: Option<Established<AdvantageLevelValid>>,
}

/// Evidence that a disadvantage was taken.
///
/// Composite proof establishing that:
/// 1. Disadvantage was taken for bonus points
/// 2. Level is valid (if leveled disadvantage)
/// 3. Disadvantage does not conflict with others
///
/// # GURPS Rules
///
/// Disadvantages provide extra points up to campaign limit.
///
/// # Citations
///
/// BS 133-166 - Disadvantages
#[derive(Prop)]
pub struct DisadvantageTakenEvidence {
    /// Proof that disadvantage was taken
    pub taken: Established<DisadvantageTaken>,
    /// Proof that level is valid (if applicable)
    pub level: Option<Established<DisadvantageLevelValid>>,
    /// Proof that no conflicts exist
    pub no_conflicts: Established<DisadvantagesNotConflicting>,
}

/// Evidence that derived statistics were calculated.
///
/// Composite proof establishing that all derived stats
/// (Basic Speed, Basic Move, Dodge, HP, Will, Per, FP) were
/// correctly calculated from attributes.
///
/// # GURPS Rules
///
/// Derived stats follow formulas based on primary attributes.
///
/// # Citations
///
/// BS 16-17 - Derived statistics
#[derive(Prop)]
pub struct DerivedStatsEvidence {
    /// Proof that Basic Speed was calculated
    pub basic_speed: Established<BasicSpeedCalculated>,
    /// Proof that Basic Move was calculated
    pub basic_move: Established<BasicMoveCalculated>,
    /// Proof that Dodge was calculated
    pub dodge: Established<DodgeCalculated>,
    /// Proof that HP was set
    pub hp: Established<HitPointsSet>,
    /// Proof that Will was set
    pub will: Established<WillSet>,
    /// Proof that Perception was set
    pub perception: Established<PerceptionSet>,
    /// Proof that FP was set
    pub fp: Established<FatiguePointsSet>,
}

/// Evidence that character creation was completed.
///
/// Composite proof establishing that:
/// 1. All attributes were purchased
/// 2. Advantages and disadvantages were selected
/// 3. Derived statistics were calculated
/// 4. Character has name and description
///
/// # GURPS Rules
///
/// Complete characters have all required elements.
///
/// # Citations
///
/// BS 10-15 - Character creation
#[derive(Prop)]
pub struct CharacterCreationEvidence {
    /// Evidence of all four primary attributes
    pub attributes: Vec<AttributePurchaseEvidence>,
    /// Evidence of derived statistics
    pub derived_stats: DerivedStatsEvidence,
    /// Proof that character is identified
    pub identified: Established<CharacterIdentified>,
    /// Proof that character is complete
    pub complete: Established<CharacterComplete>,
}

/// Evidence that character is valid and playable.
///
/// Composite proof establishing that:
/// 1. Point budget is balanced
/// 2. Campaign minimums are met
/// 3. Character creation is complete
/// 4. Character passes all validation checks
///
/// # GURPS Rules
///
/// Valid characters meet all GURPS rules and campaign restrictions.
///
/// # Citations
///
/// BS 10-15 - Character validation
#[derive(Prop)]
pub struct CharacterValidationEvidence {
    /// Proof that point budget is balanced
    pub budget_balanced: Established<PointBudgetBalanced>,
    /// Proof that campaign minimums are met
    pub minimums_met: Established<AttributesMeetCampaignMinimums>,
    /// Evidence of complete character creation
    pub creation: CharacterCreationEvidence,
    /// Proof that character is valid
    pub valid: Established<CharacterValid>,
}
