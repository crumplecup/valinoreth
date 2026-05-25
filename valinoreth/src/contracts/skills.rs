//! Skill system propositions — GURPS skill checks and advancement.
//!
//! Covers skill checks (success rolls), defaults, critical outcomes,
//! margin of success/failure, and skill improvement through character advancement.
//!
//! # GURPS Rules
//!
//! Skills in GURPS:
//! - Rolled on 3d6 vs. effective skill level
//! - Critical success: 3-4, or 5-6 if skill ≥ 15
//! - Critical failure: 17-18, or 17 if skill < 16, or margin ≥ 10
//! - Defaults: Can use related skills at penalty
//! - Improvement: Spend character points to increase skill level
//!
//! # Citations
//!
//! - BS 171-173 - Skill checks
//! - BS 344-345 - Success rolls
//! - BS 346 - Critical success and failure
//! - BS 170 - Defaults

use elicitation_derive::Prop;

// ── Skill Check Propositions ──────────────────────────────────────────────────

/// 3d6 skill check roll was executed.
///
/// Establishes that a skill check has been rolled on 3d6.
/// Does not indicate success or failure - only that the roll occurred.
///
/// # GURPS Rules
///
/// Skill checks use 3d6 against effective skill (base skill + modifiers).
/// Roll ≤ effective skill succeeds, roll > effective skill fails.
///
/// # Citations
///
/// BS 171 - Skill checks
/// BS 344 - Success rolls
#[derive(Prop)]
pub struct SkillCheckRollMade;

/// Skill check result compared against effective skill.
///
/// Establishes that the skill check roll has been evaluated against the
/// character's effective skill to determine success or failure.
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
/// BS 344 - Success rolls
#[derive(Prop)]
pub struct SkillCheckOutcomeDetermined;

/// Skill check succeeded (roll ≤ effective skill).
///
/// Establishes that the skill check roll was less than or equal to the
/// character's effective skill, meaning the action succeeds.
///
/// # GURPS Rules
///
/// A successful skill check means the character accomplishes the task.
/// Margin of success may provide additional benefits (faster, better quality, etc.).
///
/// # Citations
///
/// BS 344 - Successful rolls
#[derive(Prop)]
pub struct SkillCheckSuccessful;

/// Skill check failed (roll > effective skill).
///
/// Establishes that the skill check roll exceeded the character's effective skill,
/// meaning the action fails.
///
/// # GURPS Rules
///
/// A failed skill check means the character does not accomplish the task.
/// Margin of failure may indicate how badly it went wrong.
///
/// # Citations
///
/// BS 344 - Failed rolls
#[derive(Prop)]
pub struct SkillCheckFailed;

/// Critical success on skill check.
///
/// Establishes that the skill check achieved a critical success.
///
/// # GURPS Rules
///
/// Critical success occurs when:
/// - Roll is 3 or 4, OR
/// - Roll is 5 or 6 AND effective skill is 15+
///
/// Effects vary by task but generally mean exceptional success,
/// often with additional benefits or reduced time.
///
/// # Citations
///
/// BS 346 - Critical success
#[derive(Prop)]
pub struct SkillCheckCriticalSuccess;

/// Critical failure on skill check.
///
/// Establishes that the skill check suffered a critical failure.
///
/// # GURPS Rules
///
/// Critical failure occurs when:
/// - Roll is 18, OR
/// - Roll is 17 AND effective skill < 16, OR
/// - Roll exceeds effective skill by 10+
///
/// Effects vary by task but generally mean catastrophic failure
/// with serious consequences.
///
/// # Citations
///
/// BS 346 - Critical failure
#[derive(Prop)]
pub struct SkillCheckCriticalFailure;

/// Contest of skills resolved between two characters.
///
/// Establishes that a Quick Contest or Regular Contest was resolved
/// between two characters using their skills.
///
/// # GURPS Rules
///
/// - Quick Contest: Both roll, highest margin wins
/// - Regular Contest: Both roll, compare success/failure then margins
///
/// # Citations
///
/// BS 348-349 - Contests
#[derive(Prop)]
pub struct SkillContestResolved;

/// Contest winner determined.
///
/// Establishes which character won the contest and by what margin.
///
/// # GURPS Rules
///
/// Winner determined by:
/// 1. Success beats failure
/// 2. If both succeed or fail, higher margin wins
/// 3. Ties may be rerolled or shared
///
/// # Citations
///
/// BS 348-349 - Contest resolution
#[derive(Prop)]
pub struct ContestWinnerDetermined;

// ── Skill Defaults ────────────────────────────────────────────────────────────

/// Skill default applied from base attribute.
///
/// Establishes that a character used an attribute (DX, IQ, etc.) as a
/// default for an untrained skill.
///
/// # GURPS Rules
///
/// Many skills default to attributes at a penalty:
/// - Most IQ skills default to IQ-5 or IQ-6
/// - Most DX skills default to DX-4 to DX-6
///
/// # Citations
///
/// BS 170 - Defaults
#[derive(Prop)]
pub struct SkillDefaultedToAttribute;

/// Skill default applied from related skill.
///
/// Establishes that a character used a related skill at a penalty
/// as a default for an untrained skill.
///
/// # GURPS Rules
///
/// Skills can default from related skills:
/// - Same technique family (e.g., Sword defaults to other Sword skills)
/// - Similar concepts (e.g., Savoir-Faire defaults to other SF skills)
/// - Penalty typically -2 to -6
///
/// # Citations
///
/// BS 170 - Defaults
#[derive(Prop)]
pub struct SkillDefaultedToRelatedSkill;

/// Default penalty applied to effective skill.
///
/// Establishes that the appropriate penalty for using a default
/// has been subtracted from the base skill/attribute.
///
/// # GURPS Rules
///
/// When using a default:
/// - Effective skill = base - default_penalty
/// - Some skills have no default (must be learned)
///
/// # Citations
///
/// BS 170 - Default penalties
#[derive(Prop)]
pub struct DefaultPenaltyApplied;

// ── Skill Modifiers ───────────────────────────────────────────────────────────

/// Situational modifier applied to skill check.
///
/// Establishes that situational modifiers (lighting, equipment, time pressure)
/// have been applied to the effective skill.
///
/// # GURPS Rules
///
/// Common modifiers:
/// - Equipment: +0 to +5 (quality tools/weapons)
/// - Time: -1 to -10 (rushed) or +1 to +5 (extra time taken)
/// - Lighting: -1 to -9 (darkness)
/// - Difficulty: -1 to -10 (harder than normal)
///
/// # Citations
///
/// BS 345 - Modifiers
#[derive(Prop)]
pub struct SituationalModifierApplied;

/// Task difficulty modifier applied.
///
/// Establishes that a modifier based on task difficulty
/// (Easy/Average/Hard/Very Hard) has been applied.
///
/// # GURPS Rules
///
/// Task difficulty modifiers:
/// - Very Easy: +6 to +10
/// - Easy: +2 to +4
/// - Average: +0
/// - Hard: -2 to -4
/// - Very Hard: -6 to -10
///
/// # Citations
///
/// BS 345 - Task difficulty
#[derive(Prop)]
pub struct TaskDifficultyModifierApplied;

/// Time spent modifier applied (taking extra time or rushing).
///
/// Establishes that the time modifier has been applied based on
/// how much time the character spent on the task.
///
/// # GURPS Rules
///
/// Time spent modifiers:
/// - Half time: -2
/// - Normal time: +0
/// - 1.5x time: +1
/// - 2x time: +2
/// - 4x time: +3
/// - 8x time: +4
/// - 16x time: +5 (maximum)
///
/// # Citations
///
/// BS 346 - Time spent
#[derive(Prop)]
pub struct TimeSpentModifierApplied;

// ── Skill Improvement ─────────────────────────────────────────────────────────

/// Character points spent on skill improvement.
///
/// Establishes that character points were spent to improve a skill.
///
/// # GURPS Rules
///
/// Cost to improve skill depends on difficulty:
/// - Easy: 1/2/4/8/16 points for levels
/// - Average: 1/2/4/8/16 points for levels
/// - Hard: 2/4/8/16/32 points for levels
/// - Very Hard: 4/8/16/32/64 points for levels
///
/// # Citations
///
/// BS 170 - Skill cost
#[derive(Prop)]
pub struct CharacterPointsSpentOnSkill;

/// Skill level increased.
///
/// Establishes that a skill's level has been increased by spending
/// the appropriate number of character points.
///
/// # GURPS Rules
///
/// Skill levels increase one at a time by spending points.
/// Cost increases with each level purchased.
///
/// # Citations
///
/// BS 170 - Improving skills
#[derive(Prop)]
pub struct SkillLevelIncreased;

/// Skill point budget validated.
///
/// Establishes that the total character points spent on skills
/// does not exceed the character's available points.
///
/// # GURPS Rules
///
/// Characters have a limited point budget (typically 100-200 points).
/// Points are allocated among attributes, advantages, disadvantages, and skills.
///
/// # Citations
///
/// BS 10 - Character points
#[derive(Prop)]
pub struct SkillPointBudgetValid;

/// Prerequisite skill level met for advanced skill.
///
/// Establishes that a character has the required level in a prerequisite
/// skill before learning an advanced skill.
///
/// # GURPS Rules
///
/// Some skills require other skills as prerequisites:
/// - Example: Acrobatics requires DX 12+
/// - Example: Expert Skill requires related skill at 12+
///
/// # Citations
///
/// BS 169 - Prerequisites
#[derive(Prop)]
pub struct SkillPrerequisiteMet;

// ── Special Skill Rules ───────────────────────────────────────────────────────

/// Complementary skill bonus applied.
///
/// Establishes that a bonus from a complementary skill was applied
/// to the primary skill check.
///
/// # GURPS Rules
///
/// A related skill can provide a +1 to +4 bonus:
/// - Skill at 12-15: +1
/// - Skill at 16-19: +2
/// - Skill at 20+: +3
/// - Critical success on complementary roll: +4
///
/// # Citations
///
/// BS 346 - Complementary skills
#[derive(Prop)]
pub struct ComplementarySkillBonusApplied;

/// Familiarity penalty applied (for unfamiliar specializations).
///
/// Establishes that a familiarity penalty was applied when using
/// a skill in an unfamiliar context or specialization.
///
/// # GURPS Rules
///
/// Unfamiliar specializations: -2 to -6 penalty
/// - Example: Mechanic (Cars) at -4 when working on Boats
///
/// # Citations
///
/// BS 169 - Familiarity
#[derive(Prop)]
pub struct FamiliarityPenaltyApplied;

/// Wildcard skill used (covers multiple related skills).
///
/// Establishes that a wildcard skill was used, which covers
/// multiple related skills at full level.
///
/// # GURPS Rules
///
/// Wildcard skills (e.g., Detective!, Soldier!) are expensive but
/// cover many related skills at full level with no default penalty.
///
/// # Citations
///
/// BS 175 - Wildcard skills
#[derive(Prop)]
pub struct WildcardSkillUsed;

/// Technique used to modify skill check.
///
/// Establishes that a technique (specialized combat or skill maneuver)
/// was used, applying its default or trained bonus.
///
/// # GURPS Rules
///
/// Techniques default from base skills at a penalty but can be
/// trained to reduce or eliminate the penalty.
///
/// # Citations
///
/// BS 230-232 - Techniques
#[derive(Prop)]
pub struct TechniqueUsed;
