//! Character creation propositions — GURPS character building rules.
//!
//! Covers attribute purchase, advantage/disadvantage selection, point budgets,
//! derived statistics, and character validity constraints.
//!
//! # GURPS Rules
//!
//! Character creation in GURPS:
//! - Total point value (typically 100-300 points)
//! - Attributes purchased at 10-20 points per level
//! - Advantages purchased at variable cost
//! - Disadvantages provide extra points (up to -50 or campaign limit)
//! - Skills purchased based on difficulty
//! - Derived stats calculated from attributes
//!
//! # Citations
//!
//! - BS 10-15 - Character creation
//! - BS 16-20 - Attributes
//! - BS 100-132 - Advantages
//! - BS 133-166 - Disadvantages

use elicitation_derive::Prop;

// ── Character Point Budget ────────────────────────────────────────────────────

/// Character total point value established.
///
/// Establishes that the character has a defined total point value
/// (e.g., 100-point character, 150-point character).
///
/// # GURPS Rules
///
/// Campaign determines starting point total:
/// - 0-50: Below average
/// - 100: Competent
/// - 150: Experienced
/// - 200+: Heroic/Exceptional
///
/// # Citations
///
/// BS 10 - Character points
#[derive(Prop)]
pub struct CharacterPointValueSet;

/// Total points spent does not exceed character point budget.
///
/// Establishes that the sum of all points spent on attributes,
/// advantages, disadvantages (negative), and skills equals or is
/// less than the character's total point value.
///
/// # GURPS Rules
///
/// Points must balance:
/// - attributes + advantages + skills - disadvantages ≤ total_points
///
/// # Citations
///
/// BS 10 - Point totals
#[derive(Prop)]
pub struct PointBudgetBalanced;

/// Disadvantage point limit respected.
///
/// Establishes that the total points gained from disadvantages
/// does not exceed the campaign limit (typically -50 points).
///
/// # GURPS Rules
///
/// Disadvantage limits prevent min-maxing:
/// - Default limit: -50 points
/// - Campaign may set higher or lower
///
/// # Citations
///
/// BS 11 - Disadvantage limits
#[derive(Prop)]
pub struct DisadvantagePointLimitRespected;

// ── Attribute Purchase ────────────────────────────────────────────────────────

/// Attribute purchased within valid range.
///
/// Establishes that an attribute (ST, DX, IQ, HT) was purchased
/// at a valid level (typically 1-20).
///
/// # GURPS Rules
///
/// Attribute levels:
/// - Minimum: 1 (severely impaired)
/// - Average: 10 (baseline human)
/// - Maximum: 20 (superhuman, without enhancements)
/// - Cost: 10 points per +1 (ST, HT), 20 points per +1 (DX, IQ)
///
/// # Citations
///
/// BS 14-16 - Attributes
#[derive(Prop)]
pub struct AttributePurchased;

/// Attribute cost calculated correctly.
///
/// Establishes that the point cost for an attribute was calculated
/// according to GURPS rules.
///
/// # GURPS Rules
///
/// Attribute costs:
/// - ST: 10 points per level above/below 10
/// - DX: 20 points per level above/below 10
/// - IQ: 20 points per level above/below 10
/// - HT: 10 points per level above/below 10
///
/// # Citations
///
/// BS 14-16 - Attribute costs
#[derive(Prop)]
pub struct AttributeCostCalculated;

/// Secondary characteristic purchased.
///
/// Establishes that a secondary characteristic (HP, Will, Per, FP, Basic Speed, Basic Move)
/// was purchased independently from its base attribute.
///
/// # GURPS Rules
///
/// Secondary characteristics default to attributes but can be purchased separately:
/// - HP defaults to ST, costs 2 points per level
/// - Will defaults to IQ, costs 5 points per level
/// - Per defaults to IQ, costs 5 points per level
/// - FP defaults to HT, costs 3 points per level
/// - Basic Speed defaults to (DX+HT)/4, costs 20 points per +1.00
/// - Basic Move defaults to Basic Speed, costs 5 points per level
///
/// # Citations
///
/// BS 16-17 - Secondary characteristics
#[derive(Prop)]
pub struct SecondaryCharacteristicPurchased;

// ── Advantage Selection ───────────────────────────────────────────────────────

/// Advantage purchased.
///
/// Establishes that an advantage was purchased for the character
/// at its listed point cost.
///
/// # GURPS Rules
///
/// Advantages provide benefits:
/// - Fixed cost (e.g., Combat Reflexes: 15 points)
/// - Variable cost (e.g., Wealth: 10-50 points)
/// - Leveled (e.g., Luck: 15/30/60 points for levels 1/2/3)
///
/// # Citations
///
/// BS 100-132 - Advantages
#[derive(Prop)]
pub struct AdvantagePurchased;

/// Advantage cost calculated with modifiers.
///
/// Establishes that enhancement and limitation modifiers were
/// correctly applied to an advantage's base cost.
///
/// # GURPS Rules
///
/// Advantages can have modifiers:
/// - Enhancements: increase cost (e.g., +20% = ×1.2)
/// - Limitations: decrease cost (e.g., -10% = ×0.9)
/// - Formula: base_cost × (1 + enhancements - limitations)
///
/// # Citations
///
/// BS 101-108 - Modifiers
#[derive(Prop)]
pub struct AdvantageModifiersCostCalculated;

/// Advantage level valid for leveled advantage.
///
/// Establishes that a leveled advantage (Luck, Status, Wealth, etc.)
/// was purchased at a valid level with correct cost.
///
/// # GURPS Rules
///
/// Leveled advantages:
/// - Each level has specific cost
/// - Some have minimum/maximum levels
/// - Example: Status ranges from -2 to 8
///
/// # Citations
///
/// BS 100-132 - Leveled advantages
#[derive(Prop)]
pub struct AdvantageLevelValid;

/// Advantage prerequisite met.
///
/// Establishes that prerequisites for an advantage were satisfied
/// (required attributes, other advantages, etc.).
///
/// # GURPS Rules
///
/// Some advantages require prerequisites:
/// - Example: Weapon Master requires DX 12+ and training
/// - Example: Magery 0 required before buying spells
///
/// # Citations
///
/// BS 100-132 - Advantage prerequisites
#[derive(Prop)]
pub struct AdvantagePrerequisiteMet;

// ── Disadvantage Selection ────────────────────────────────────────────────────

/// Disadvantage taken.
///
/// Establishes that a disadvantage was taken for the character,
/// providing bonus points equal to its negative cost.
///
/// # GURPS Rules
///
/// Disadvantages provide extra points:
/// - Social stigmas, physical limitations, mental quirks
/// - Must roleplay the disadvantage
/// - GM can disallow inappropriate choices
///
/// # Citations
///
/// BS 133-166 - Disadvantages
#[derive(Prop)]
pub struct DisadvantageTaken;

/// Self-control roll specified for disadvantage.
///
/// Establishes that a self-control roll value (6, 9, 12, 15) was
/// specified for a disadvantage that requires self-control.
///
/// # GURPS Rules
///
/// Many mental disadvantages require self-control rolls:
/// - Roll 3d6 ≤ self-control value to resist
/// - Lower values (harder to resist) provide more points
/// - Typical values: 6 (hard), 9 (fairly hard), 12 (fairly easy), 15 (easy)
///
/// # Citations
///
/// BS 120 - Self-control rolls
#[derive(Prop)]
pub struct SelfControlRollSpecified;

/// Disadvantage level valid for leveled disadvantage.
///
/// Establishes that a leveled disadvantage (Bad Temper, Phobia, etc.)
/// was taken at a valid level with correct point value.
///
/// # GURPS Rules
///
/// Leveled disadvantages:
/// - Each level has specific negative cost
/// - Example: Phobias range from -5 (mild) to -15 (severe)
///
/// # Citations
///
/// BS 133-166 - Leveled disadvantages
#[derive(Prop)]
pub struct DisadvantageLevelValid;

/// Mutually exclusive disadvantages avoided.
///
/// Establishes that the character does not have disadvantages
/// that conflict with each other.
///
/// # GURPS Rules
///
/// Some disadvantages cannot coexist:
/// - Example: Cannot have both Pacifism and Bloodlust
/// - Example: Deaf and Hard of Hearing conflict
///
/// # Citations
///
/// BS 133-166 - Disadvantage conflicts
#[derive(Prop)]
pub struct DisadvantagesNotConflicting;

// ── Quirks ────────────────────────────────────────────────────────────────────

/// Quirk taken (minor personality trait).
///
/// Establishes that a quirk was taken for the character,
/// providing 1 point each.
///
/// # GURPS Rules
///
/// Quirks are minor disadvantages:
/// - Worth -1 point each
/// - Maximum 5 quirks (5 points)
/// - Must be roleplayed
///
/// # Citations
///
/// BS 162-164 - Quirks
#[derive(Prop)]
pub struct QuirkTaken;

/// Quirk limit respected (maximum 5 quirks).
///
/// Establishes that the character has taken no more than
/// 5 quirks (providing at most -5 points).
///
/// # GURPS Rules
///
/// Quirk limit: 5 quirks maximum (-5 points)
///
/// # Citations
///
/// BS 162 - Quirk limits
#[derive(Prop)]
pub struct QuirkLimitRespected;

// ── Derived Statistics ────────────────────────────────────────────────────────

/// Basic Speed calculated from DX and HT.
///
/// Establishes that Basic Speed was calculated correctly
/// as (DX + HT) / 4, unless purchased separately.
///
/// # GURPS Rules
///
/// Basic Speed = (DX + HT) / 4
/// - Determines turn order in combat
/// - Affects Dodge and Move
///
/// # Citations
///
/// BS 17 - Basic Speed
#[derive(Prop)]
pub struct BasicSpeedCalculated;

/// Basic Move calculated from Basic Speed.
///
/// Establishes that Basic Move was calculated correctly
/// as floor(Basic Speed), unless purchased separately.
///
/// # GURPS Rules
///
/// Basic Move = floor(Basic Speed)
/// - Distance in yards per second
/// - Can be enhanced or reduced independently
///
/// # Citations
///
/// BS 17 - Basic Move
#[derive(Prop)]
pub struct BasicMoveCalculated;

/// Dodge calculated from Basic Speed.
///
/// Establishes that Dodge defense was calculated correctly
/// as floor(Basic Speed) + 3.
///
/// # GURPS Rules
///
/// Dodge = floor(Basic Speed) + 3
/// - Active defense against attacks
/// - Modified by encumbrance
///
/// # Citations
///
/// BS 17, 374 - Dodge
#[derive(Prop)]
pub struct DodgeCalculated;

/// Hit Points calculated or purchased.
///
/// Establishes that HP was set correctly, either defaulting to ST
/// or purchased independently at 2 points per level.
///
/// # GURPS Rules
///
/// HP = ST (default) or purchased at 2 points per level
///
/// # Citations
///
/// BS 16 - Hit Points
#[derive(Prop)]
pub struct HitPointsSet;

/// Will calculated or purchased.
///
/// Establishes that Will was set correctly, either defaulting to IQ
/// or purchased independently at 5 points per level.
///
/// # GURPS Rules
///
/// Will = IQ (default) or purchased at 5 points per level
///
/// # Citations
///
/// BS 16 - Will
#[derive(Prop)]
pub struct WillSet;

/// Perception calculated or purchased.
///
/// Establishes that Perception was set correctly, either defaulting to IQ
/// or purchased independently at 5 points per level.
///
/// # GURPS Rules
///
/// Per = IQ (default) or purchased at 5 points per level
///
/// # Citations
///
/// BS 16 - Perception
#[derive(Prop)]
pub struct PerceptionSet;

/// Fatigue Points calculated or purchased.
///
/// Establishes that FP was set correctly, either defaulting to HT
/// or purchased independently at 3 points per level.
///
/// # GURPS Rules
///
/// FP = HT (default) or purchased at 3 points per level
///
/// # Citations
///
/// BS 16 - Fatigue Points
#[derive(Prop)]
pub struct FatiguePointsSet;

// ── Character Validity ────────────────────────────────────────────────────────

/// Character attributes meet campaign minimums.
///
/// Establishes that all character attributes meet the campaign's
/// minimum requirements.
///
/// # GURPS Rules
///
/// Campaigns may set minimums:
/// - Example: "No attribute below 8"
/// - Example: "ST must be at least 10"
///
/// # Citations
///
/// BS 11 - Campaign limits
#[derive(Prop)]
pub struct AttributesMeetCampaignMinimums;

/// Character meets racial template requirements.
///
/// Establishes that a character using a racial template (Elf, Dwarf, etc.)
/// has the required advantages, disadvantages, and attribute modifiers.
///
/// # GURPS Rules
///
/// Racial templates provide:
/// - Fixed attribute modifiers (cost included in template)
/// - Required advantages/disadvantages
/// - Racial abilities
///
/// # Citations
///
/// BS 19-23 - Meta-traits and racial templates
#[derive(Prop)]
pub struct RacialTemplateRequirementsMet;

/// Character name and description provided.
///
/// Establishes that the character has a name and description
/// suitable for play.
///
/// # GURPS Rules
///
/// Characters need identification:
/// - Name
/// - Description (appearance, personality)
/// - Background (optional but recommended)
///
/// # Citations
///
/// BS 10 - Character creation
#[derive(Prop)]
pub struct CharacterIdentified;

/// All required character elements present.
///
/// Establishes that the character has all required elements:
/// attributes, advantages, disadvantages, skills, and derived stats.
///
/// # GURPS Rules
///
/// Complete characters have:
/// - 4 primary attributes (ST, DX, IQ, HT)
/// - Secondary characteristics (derived or purchased)
/// - Skills (at least one)
/// - Name and description
///
/// # Citations
///
/// BS 10-15 - Character creation checklist
#[derive(Prop)]
pub struct CharacterComplete;

/// Character is balanced and playable.
///
/// Establishes that the character meets all GURPS rules and
/// campaign restrictions, and is ready for play.
///
/// # GURPS Rules
///
/// Valid characters:
/// - Points balance correctly
/// - No illegal combinations
/// - All prerequisites met
/// - Meets campaign restrictions
///
/// # Citations
///
/// BS 10-15 - Character validation
#[derive(Prop)]
pub struct CharacterValid;
