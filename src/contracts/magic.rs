//! Magic system propositions — GURPS spell casting rules.
//!
//! Covers spell casting (concentration, skill rolls, energy costs), spell learning
//! (prerequisites, Magery requirements), spell effects (maintenance, resistance),
//! and critical outcomes.
//!
//! # GURPS Rules
//!
//! Magic in GURPS:
//! - Each spell is a separate Mental/Hard skill
//! - Spells require Magery advantage (except in high mana)
//! - Prerequisites must be learned before advanced spells
//! - Energy cost reduces with skill (at 15, 20, 25, etc.)
//! - Concentration time varies by skill level
//! - Critical success/failure have special effects
//! - Resisted spells use Quick Contest
//!
//! # Citations
//!
//! - Magic 7-11 - Spell casting basics
//! - Magic 12-14 - Energy costs
//! - Magic 15-16 - Critical success and failure
//! - Magic 18-20 - Resistance and modifiers
//! - BS 235-253 - Magic chapter
//!
//! # Sources
//!
//! - [GURPS FAQ: Magic](https://www.sjgames.com/gurps/faq/FAQ4-4.html)
//! - [GURPS Magic Rules](https://www.scribd.com/document/372549659/GURPS-Magic-Rules)
//! - [Casting Spells (GURPS Wiki)](https://gurps.fandom.com/wiki/Casting_Spells)
//! - [Critical Spell Failure Table](https://gurps.fandom.com/wiki/Critical_Spell_Failure_Table)

use elicitation_derive::Prop;

// ── Spell Learning ────────────────────────────────────────────────────────────

/// Spell prerequisites met.
///
/// Establishes that all prerequisite spells for learning a new spell
/// have been satisfied.
///
/// # GURPS Rules
///
/// Most spells have prerequisites (other spells that must be known first).
/// Example: Fireball requires Create Fire and Shape Fire.
///
/// # Citations
///
/// Magic 8 - Prerequisites
#[derive(Prop)]
pub struct SpellPrerequisitesMet;

/// Magery requirement met.
///
/// Establishes that character has sufficient Magery advantage level
/// to learn the spell.
///
/// # GURPS Rules
///
/// Some spells require Magery 1, 2, or 3 to learn.
/// In high mana areas, Magery may not be required.
///
/// # Citations
///
/// Magic 8 - Magery requirements
/// BS 66 - Magery advantage
#[derive(Prop)]
pub struct MageryRequirementMet;

/// Spell learned.
///
/// Establishes that character has successfully learned a spell,
/// adding it to their spell repertoire.
///
/// # GURPS Rules
///
/// Spells are Mental/Hard skills that can be learned and improved
/// with character points.
///
/// # Citations
///
/// Magic 7-8 - Learning spells
#[derive(Prop)]
pub struct SpellLearned;

/// Spell skill level set.
///
/// Establishes that a spell's effective skill level was determined
/// (base skill + Magery bonus + modifiers).
///
/// # GURPS Rules
///
/// Effective spell skill = base skill + Magery level + talent + modifiers
///
/// # Citations
///
/// Magic 8-9 - Spell skill
#[derive(Prop)]
pub struct SpellSkillLevelSet;

// ── Spell Casting ─────────────────────────────────────────────────────────────

/// Concentration begun.
///
/// Establishes that spellcaster began concentrating to cast a spell.
///
/// # GURPS Rules
///
/// Concentration time varies by skill:
/// - Skill ≤11: double standard time
/// - Skill 12-20: standard time (1-3 seconds typically)
/// - Skill ≥21: half time (round up)
/// - Skill ≥25: quarter time (round up)
///
/// # Citations
///
/// Magic 9 - Casting time
#[derive(Prop)]
pub struct ConcentrationBegun;

/// Concentration maintained.
///
/// Establishes that spellcaster successfully maintained concentration
/// despite distractions or injury.
///
/// # GURPS Rules
///
/// If distracted, roll Will-3 to maintain concentration.
/// If injured, effective skill drops by damage taken.
///
/// # Citations
///
/// Magic 10 - Concentration and distraction
#[derive(Prop)]
pub struct ConcentrationMaintained;

/// Concentration completed.
///
/// Establishes that spellcaster completed the required concentration
/// time for the spell.
///
/// # GURPS Rules
///
/// After concentrating for the required time, make a skill roll
/// to cast the spell.
///
/// # Citations
///
/// Magic 9-10 - Casting time
#[derive(Prop)]
pub struct ConcentrationCompleted;

/// Spell skill roll made.
///
/// Establishes that 3d6 was rolled against the spell's effective skill.
///
/// # GURPS Rules
///
/// After concentration, roll 3d6 vs. effective spell skill.
///
/// # Citations
///
/// Magic 10 - Spell casting
#[derive(Prop)]
pub struct SpellSkillRollMade;

/// Spell casting outcome determined.
///
/// Establishes that the spell skill roll was compared to effective skill
/// to determine success or failure.
///
/// # GURPS Rules
///
/// Roll ≤ skill: spell succeeds and energy is spent.
/// Roll > skill: spell fails, energy may be lost (half or full).
///
/// # Citations
///
/// Magic 10 - Success and failure
#[derive(Prop)]
pub struct SpellCastingOutcomeDetermined;

/// Spell casting succeeded.
///
/// Establishes that the spell skill roll succeeded (roll ≤ effective skill).
///
/// # GURPS Rules
///
/// On success, spell takes effect and full energy cost is paid.
///
/// # Citations
///
/// Magic 10 - Successful spells
#[derive(Prop)]
pub struct SpellCastingSucceeded;

/// Spell casting failed.
///
/// Establishes that the spell skill roll failed (roll > effective skill).
///
/// # GURPS Rules
///
/// On failure, spell doesn't work. Energy loss depends on margin:
/// - Margin 1: lose 1 energy
/// - Margin 2-5: lose half energy (round up)
/// - Margin 6+: lose full energy
///
/// # Citations
///
/// Magic 11 - Spell failure
#[derive(Prop)]
pub struct SpellCastingFailed;

/// Spell critical success achieved.
///
/// Establishes that the spell achieved a critical success
/// (roll 3-4, or 5-6 if skill ≥15).
///
/// # GURPS Rules
///
/// Critical success results vary by GM discretion:
/// - Spell costs no energy
/// - Spell has double effect or duration
/// - Spell affects larger area or more targets
///
/// # Citations
///
/// Magic 15 - Critical success
#[derive(Prop)]
pub struct SpellCriticalSuccess;

/// Spell critical failure occurred.
///
/// Establishes that the spell critically failed
/// (roll 17-18, or margin ≥10).
///
/// # GURPS Rules
///
/// Critical failure causes mishaps:
/// - Spell fails, caster takes 1d injury (roll 3)
/// - Spell affects caster or ally instead (roll 4-6)
/// - Spell affects wrong target (roll 7-8)
/// - Caster forgets spell temporarily (roll 9-10)
/// - Worse outcomes possible (roll 11+)
///
/// # Citations
///
/// Magic 16 - Critical failure
/// Magic 71 - Critical spell failure table
#[derive(Prop)]
pub struct SpellCriticalFailure;

// ── Energy Cost ───────────────────────────────────────────────────────────────

/// Base energy cost determined.
///
/// Establishes that the spell's base energy cost was looked up
/// from spell description.
///
/// # GURPS Rules
///
/// Each spell lists its energy cost (e.g., "2 to cast, 1 to maintain").
///
/// # Citations
///
/// Magic 12 - Energy cost
#[derive(Prop)]
pub struct BaseEnergyCostDetermined;

/// Skill-based cost reduction applied.
///
/// Establishes that energy cost was reduced based on caster's
/// effective skill level.
///
/// # GURPS Rules
///
/// Energy cost reduces by 1 at skill 15, and by 1 additional
/// for every 5 levels thereafter (20, 25, etc.).
/// Minimum cost is typically 1 (or 0 for some spells).
///
/// # Citations
///
/// Magic 12-13 - Skill-based cost reduction
#[derive(Prop)]
pub struct SkillBasedCostReductionApplied;

/// Final energy cost calculated.
///
/// Establishes that the final energy cost was calculated after
/// all modifiers and reductions.
///
/// # GURPS Rules
///
/// Final cost = base cost - skill reductions + extra effect costs
///
/// # Citations
///
/// Magic 13 - Final energy cost
#[derive(Prop)]
pub struct FinalEnergyCostCalculated;

/// Energy paid from caster.
///
/// Establishes that energy was deducted from caster's
/// Fatigue Points (or HP if FP exhausted).
///
/// # GURPS Rules
///
/// Casters spend FP to power spells. When FP reaches 0,
/// they can spend HP but risk unconsciousness.
///
/// # Citations
///
/// Magic 11-12 - Energy sources
#[derive(Prop)]
pub struct EnergyPaidFromCaster;

/// Spell maintained.
///
/// Establishes that an ongoing spell is being maintained
/// at its maintenance cost per second.
///
/// # GURPS Rules
///
/// Some spells can be maintained by paying maintenance cost
/// each second. No concentration or roll needed while maintaining.
///
/// # Citations
///
/// Magic 14 - Spell maintenance
#[derive(Prop)]
pub struct SpellMaintained;

/// Maintenance energy paid.
///
/// Establishes that maintenance cost was paid to keep
/// spell active for another second.
///
/// # GURPS Rules
///
/// Pay maintenance cost each second or spell ends.
/// Can drop spell voluntarily at any time.
///
/// # Citations
///
/// Magic 14 - Maintenance cost
#[derive(Prop)]
pub struct MaintenanceEnergyPaid;

// ── Spell Effects ─────────────────────────────────────────────────────────────

/// Spell effect applied.
///
/// Establishes that the spell's effect was applied to
/// the target(s).
///
/// # GURPS Rules
///
/// On successful casting, spell effect occurs as described
/// in spell description.
///
/// # Citations
///
/// Magic 10-11 - Spell effects
#[derive(Prop)]
pub struct SpellEffectApplied;

/// Spell target determined.
///
/// Establishes that the target(s) of the spell were identified.
///
/// # GURPS Rules
///
/// Spells may target self, touched subject, distant subject,
/// or area. Some spells allow multiple targets.
///
/// # Citations
///
/// Magic 9 - Spell targets
#[derive(Prop)]
pub struct SpellTargetDetermined;

/// Spell range checked.
///
/// Establishes that spell target is within range.
///
/// # GURPS Rules
///
/// Most spells have limited range, often tied to skill level.
/// Long-distance modifiers may apply (-1 per yard beyond base).
///
/// # Citations
///
/// Magic 18-19 - Range modifiers
#[derive(Prop)]
pub struct SpellRangeChecked;

/// Spell duration determined.
///
/// Establishes that spell duration was determined
/// (instant, minutes, maintained, etc.).
///
/// # GURPS Rules
///
/// Duration varies by spell: instant, fixed time, or maintained.
///
/// # Citations
///
/// Magic 14 - Duration
#[derive(Prop)]
pub struct SpellDurationDetermined;

// ── Spell Resistance ──────────────────────────────────────────────────────────

/// Resistance roll required.
///
/// Establishes that spell is resisted and target must
/// make a resistance roll.
///
/// # GURPS Rules
///
/// Many spells can be resisted using an attribute (Will, HT, etc.).
///
/// # Citations
///
/// Magic 18 - Resistance
#[derive(Prop)]
pub struct ResistanceRollRequired;

/// Resistance roll made.
///
/// Establishes that target rolled to resist the spell.
///
/// # GURPS Rules
///
/// Resistance is a Quick Contest: caster's skill vs. target's
/// resistance attribute.
///
/// # Citations
///
/// Magic 18-20 - Resistance contests
#[derive(Prop)]
pub struct ResistanceRollMade;

/// Spell resisted successfully.
///
/// Establishes that target successfully resisted the spell.
///
/// # GURPS Rules
///
/// If target wins or ties the Quick Contest, spell has
/// no effect (or reduced effect, spell-dependent).
///
/// # Citations
///
/// Magic 18 - Successful resistance
#[derive(Prop)]
pub struct SpellResistedSuccessfully;

/// Resistance overcome.
///
/// Establishes that caster overcame target's resistance.
///
/// # GURPS Rules
///
/// If caster wins the Quick Contest, spell affects target normally.
///
/// # Citations
///
/// Magic 18 - Overcoming resistance
#[derive(Prop)]
pub struct ResistanceOvercome;

// ── Spell Modifiers ───────────────────────────────────────────────────────────

/// Size/speed modifier applied.
///
/// Establishes that size and speed modifiers were applied
/// to spell skill roll.
///
/// # GURPS Rules
///
/// Small or fast targets impose skill penalties.
/// Large targets provide bonuses.
///
/// # Citations
///
/// Magic 19 - Size and speed modifiers
#[derive(Prop)]
pub struct SizeSpeedModifierApplied;

/// Time modifier applied.
///
/// Establishes that modifier for taking extra time or rushing
/// was applied to spell skill.
///
/// # GURPS Rules
///
/// Taking extra time: +1 per additional second (max +5).
/// Rushing: possible at higher skill levels.
///
/// # Citations
///
/// Magic 20 - Time spent modifiers
#[derive(Prop)]
pub struct TimeModifierApplied;

/// Environment modifier applied.
///
/// Establishes that environmental factors (lighting, weather, etc.)
/// were applied to spell skill.
///
/// # GURPS Rules
///
/// Poor conditions impose penalties. Ideal conditions may grant bonuses.
///
/// # Citations
///
/// Magic 20 - Environmental modifiers
#[derive(Prop)]
pub struct EnvironmentModifierApplied;

// ── Ceremonial Magic ──────────────────────────────────────────────────────────

/// Ceremonial casting begun.
///
/// Establishes that multiple casters are combining energy
/// for ceremonial magic.
///
/// # GURPS Rules
///
/// Multiple mages can contribute energy or skill to one spell.
/// Requires coordination and time.
///
/// # Citations
///
/// Magic 20-21 - Ceremonial magic
#[derive(Prop)]
pub struct CeremonialCastingBegun;

/// Energy pooled from participants.
///
/// Establishes that energy contributions from all participants
/// were collected.
///
/// # GURPS Rules
///
/// Each participant contributes energy up to their FP limit.
/// Leader uses pooled energy to cast spell.
///
/// # Citations
///
/// Magic 21 - Energy pooling
#[derive(Prop)]
pub struct EnergyPooledFromParticipants;

/// Ceremonial spell completed.
///
/// Establishes that ceremonial spell was successfully cast
/// using pooled energy and skill.
///
/// # GURPS Rules
///
/// Leader makes skill roll using their skill plus bonuses
/// from assistants.
///
/// # Citations
///
/// Magic 21 - Ceremonial completion
#[derive(Prop)]
pub struct CeremonialSpellCompleted;
