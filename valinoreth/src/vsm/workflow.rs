//! [`CombatWorkflow`] — orchestrates a GURPS combat encounter.
//!
//! # Architecture
//!
//! The workflow follows the "server holds the logic, players get a view" pattern:
//!
//! - **[`CombatSession`]** — `Arc<Mutex<CombatPhase>>` holding the authoritative
//!   VSM state.  The workflow is the only writer; UI observers read
//!   [`CombatStateView`] snapshots via [`CombatSession`] without accessing proof
//!   tokens.
//!
//! - **[`ContextualCommunicator`]** — wraps each player's communicator.  Before
//!   every maneuver or defense elicitation, the workflow pushes the current
//!   [`CombatStateView`] into the player's [`KnowledgeCache`]; the communicator
//!   then prepends it to the prompt so the player always sees fresh state.
//!
//! - **[`CombatMachine`]** (VSM) — formally verified transitions advance the
//!   encounter.  Proof tokens live in local variables inside `run()`; they are
//!   never stored in the shared session.
//!
//! - **[`GameMaster`]** — GURPS mechanics (attack rolls, damage, etc.).
//!
//! # GM narration
//!
//! [`CombatWorkflow`] posts [`ChatMessage`]s via the `chat_tx` channel.  The
//! UI event loop drains this channel each frame.
//!
//! [`ContextualCommunicator`]: crate::ContextualCommunicator
//! [`KnowledgeCache`]: crate::KnowledgeCache
//! [`CombatMachine`]: crate::CombatMachine

use elicitation::{ChoiceSet, ElicitCommunicator, Established};
use tokio::sync::mpsc;
use tracing::{debug, info, instrument};

use crate::contracts::traits::{CombatExchangeResult, CombatExecutor};
use crate::contracts::types::{
    ArmorDescriptor, AttackDescriptorBuilder, CombatantDescriptorBuilder, DamageDescriptorBuilder,
    DamageTypeDescriptor, DefenseDescriptorBuilder, DefenseType,
};
use crate::vsm::combat::{
    CombatState, CombatantState, apply_damage, begin_turn, conclude_combat, declare_attack,
    end_turn, initialize_combat, resolve_attack, resolve_defense,
};
use crate::vsm::integration::{attack_resolved_from_gm, damage_applied_from_gm};
use crate::vsm::session::{CombatPhase, CombatSession, CombatStateView};
use crate::{
    AttributeType, CharacterDescriptor, ChatMessage, ChatSender, ContextualCommunicator,
    DefenseChoice, GameMaster, ManeuverChoice, Player, SharedKnowledge, knowledge_cache,
};

// ── WorkflowError ─────────────────────────────────────────────────────────────

/// Error produced by [`CombatWorkflow::run`].
#[derive(Debug, derive_more::Display, derive_more::Error)]
pub enum WorkflowError {
    /// Elicitation failure (I/O error, Ctrl-C, LLM error, etc.).
    #[display("Elicitation error: {}", _0)]
    Elicit(elicitation::ElicitError),
    /// GameMaster contract violation (invalid dice, bad state, etc.).
    #[display("Contract error: {}", _0)]
    Contract(crate::ContractError),
}

impl From<elicitation::ElicitError> for WorkflowError {
    fn from(e: elicitation::ElicitError) -> Self {
        Self::Elicit(e)
    }
}

impl From<crate::ContractError> for WorkflowError {
    fn from(e: crate::ContractError) -> Self {
        Self::Contract(e)
    }
}

// ── CombatantEntry ────────────────────────────────────────────────────────────

/// Per-combatant bundle: player with contextual communicator, knowledge cache,
/// and team assignment.
struct CombatantEntry<C: ElicitCommunicator + Clone> {
    player: Player<ContextualCommunicator<C>>,
    knowledge: SharedKnowledge,
    team: String,
}

// ── CombatWorkflow ────────────────────────────────────────────────────────────

/// Orchestrates a GURPS combat encounter.
///
/// Generic over `C: ElicitCommunicator + Clone` so the same workflow drives
/// human players ([`TuiCommunicator`]), agent players, or any middleware
/// wrapping them ([`ObservableCommunicator`]).
///
/// Pass [`session`] to a TUI observer before calling [`run`] so the UI can
/// poll [`CombatStateView`] snapshots while combat progresses.
///
/// [`TuiCommunicator`]: crate::TuiCommunicator
/// [`ObservableCommunicator`]: crate::ObservableCommunicator
/// [`session`]: CombatWorkflow::session
/// [`run`]: CombatWorkflow::run
pub struct CombatWorkflow<C: ElicitCommunicator + Clone> {
    session: CombatSession,
    combatants: Vec<CombatantEntry<C>>,
    gm: GameMaster,
    chat_tx: mpsc::UnboundedSender<ChatMessage>,
}

impl<C: ElicitCommunicator + Clone> CombatWorkflow<C> {
    /// Create a new workflow.
    ///
    /// Each `(Player<C>, team_name)` pair is wrapped so the player's
    /// communicator receives a [`CombatStateView`] preamble before every
    /// elicitation.  Teams determine victory conditions: combat ends when
    /// only one team has non-incapacitated combatants.
    #[instrument(skip(gm, players, chat_tx))]
    pub fn new(
        gm: GameMaster,
        players: Vec<(Player<C>, String)>,
        chat_tx: mpsc::UnboundedSender<ChatMessage>,
    ) -> Self {
        let session = std::sync::Arc::new(tokio::sync::Mutex::new(CombatPhase::Unstarted));
        let combatants = players
            .into_iter()
            .map(|(player, team)| {
                let knowledge = knowledge_cache();
                let comm = ContextualCommunicator::new(player.communicator, knowledge.clone());
                CombatantEntry {
                    player: Player::new(player.character, comm),
                    knowledge,
                    team,
                }
            })
            .collect();
        Self {
            session,
            combatants,
            gm,
            chat_tx,
        }
    }

    /// Clone the session handle so a TUI observer can poll state independently.
    ///
    /// Call this before [`run`] and pass the handle to the UI render loop.
    ///
    /// [`run`]: CombatWorkflow::run
    pub fn session(&self) -> CombatSession {
        self.session.clone()
    }

    /// Run the combat encounter to completion.
    ///
    /// Returns the winning team name, or `None` if all combatants were
    /// simultaneously incapacitated (draw).
    ///
    /// # Errors
    ///
    /// Returns [`WorkflowError`] if a player communicator fails or a
    /// GameMaster contract is violated.
    #[instrument(skip(self))]
    pub async fn run(&mut self) -> Result<Option<String>, WorkflowError> {
        // ── Initialize VSM ────────────────────────────────────────────────────
        let states: Vec<CombatantState> = self
            .combatants
            .iter()
            .map(|e| build_combatant_state(&e.player.character, &e.team))
            .collect();

        let (mut vsm_state, mut vsm_proof) = initialize_combat(
            CombatState::Uninitialized,
            Established::assert(),
            states,
            Established::assert(),
        );

        push_phase(&self.session, CombatPhase::Active(vsm_state.clone())).await;
        self.narrate("⚔ Combat begins!");

        // ── Turn loop ─────────────────────────────────────────────────────────
        loop {
            // Extract what we need, then release borrows before any VSM move.
            let (actor_slot, actor_state, round) = {
                let CombatState::Active {
                    combatants,
                    turn_order,
                    current_actor,
                    round,
                } = &vsm_state
                else {
                    break;
                };
                let slot = turn_order[*current_actor];
                (slot, combatants[slot].clone(), *round)
            };

            // Inject current state into the actor's knowledge cache.
            self.refresh_knowledge(actor_slot, &vsm_state);

            (vsm_state, vsm_proof) = begin_turn(vsm_state, vsm_proof, Established::assert());

            if actor_state.incapacitated {
                self.narrate(format!("{} is incapacitated — skipping.", actor_state.id));
                (vsm_state, vsm_proof) = end_turn(vsm_state, vsm_proof, Established::assert());
                push_phase(&self.session, CombatPhase::Active(vsm_state.clone())).await;
                if let Some(victor) = winning_team(&vsm_state) {
                    conclude_combat(
                        vsm_state,
                        vsm_proof,
                        Some(victor.clone()),
                        Established::assert(),
                    );
                    push_phase(
                        &self.session,
                        CombatPhase::Concluded {
                            winner: Some(victor.clone()),
                        },
                    )
                    .await;
                    self.narrate(format!("⚔ {} wins!", victor));
                    return Ok(Some(victor));
                }
                continue;
            }

            self.narrate(format!("--- Round {} — {} ---", round, actor_state.id));

            // ── Elicit maneuver ───────────────────────────────────────────────
            let maneuver = self.combatants[actor_slot].player.choose_maneuver().await?;

            self.narrate(format!("{} → {}", actor_state.id, maneuver));

            match maneuver {
                ManeuverChoice::Attack | ManeuverChoice::AllOutAttack => {
                    let all_out = matches!(maneuver, ManeuverChoice::AllOutAttack);

                    let Some(target_slot) = find_enemy_target(&vsm_state, actor_slot) else {
                        self.narrate(format!("{} has no valid target.", actor_state.id));
                        (vsm_state, vsm_proof) =
                            end_turn(vsm_state, vsm_proof, Established::assert());
                        push_phase(&self.session, CombatPhase::Active(vsm_state.clone())).await;
                        continue;
                    };

                    let target_state = match &vsm_state {
                        CombatState::Active { combatants, .. } => combatants[target_slot].clone(),
                        _ => break,
                    };

                    // Inject state into the defender's knowledge before eliciting.
                    self.refresh_knowledge(target_slot, &vsm_state);

                    (vsm_state, vsm_proof) = declare_attack(
                        vsm_state,
                        vsm_proof,
                        actor_slot,
                        target_slot,
                        Established::assert(),
                    );

                    // ── Elicit defense ────────────────────────────────────────
                    let defenses =
                        available_defenses(&self.combatants[target_slot].player.character);
                    debug!(defender = %target_state.id, "eliciting defense choice");
                    let defense_choice = self.combatants[target_slot]
                        .player
                        .choose_defense(defenses)
                        .await?;
                    debug!(defender = %target_state.id, choice = ?defense_choice, "defense chosen");

                    // ── Build descriptors ─────────────────────────────────────
                    let attacker_char = &self.combatants[actor_slot].player.character;
                    let defender_char = &self.combatants[target_slot].player.character;

                    let attack_skill = melee_skill(attacker_char);
                    let attack_desc = AttackDescriptorBuilder::default()
                        .effective_skill(if all_out {
                            attack_skill + 4
                        } else {
                            attack_skill
                        })
                        .all_out_attack(all_out)
                        .build()
                        .expect("valid attack descriptor");

                    let attacker_desc = CombatantDescriptorBuilder::default()
                        .current_hp(actor_state.current_hp)
                        .max_hp(actor_state.max_hp)
                        .dodge(attacker_char.derived_stats.dodge)
                        .build()
                        .expect("valid attacker descriptor");

                    let defender_desc = CombatantDescriptorBuilder::default()
                        .current_hp(target_state.current_hp)
                        .max_hp(target_state.max_hp)
                        .dodge(defender_char.derived_stats.dodge)
                        .parry(Some(parry_score(defender_char)))
                        .build()
                        .expect("valid defender descriptor");

                    let defense_desc = build_defense_descriptor(defense_choice, defender_char);

                    let damage_desc = DamageDescriptorBuilder::default()
                        .dice(1_i32)
                        .sides(6_i32)
                        .modifier(0_i32)
                        .damage_type(DamageTypeDescriptor::Crushing)
                        .build()
                        .expect("valid damage descriptor");

                    let armor_desc = ArmorDescriptor {
                        dr: 0,
                        flexible: false,
                    };

                    // ── Execute combat exchange ───────────────────────────────
                    debug!(attacker = %actor_state.id, defender = %target_state.id, "executing attack");
                    match self
                        .gm
                        .execute_attack(
                            attacker_desc,
                            defender_desc,
                            attack_desc,
                            defense_desc,
                            damage_desc,
                            armor_desc,
                        )
                        .await?
                    {
                        CombatExchangeResult::Miss { reason, .. } => {
                            info!(attacker = %actor_state.id, defender = %target_state.id, ?reason, "attack missed");
                            self.narrate(format!(
                                "{} attacks {} — miss ({:?}).",
                                actor_state.id, target_state.id, reason
                            ));
                            (vsm_state, vsm_proof) = resolve_attack(
                                vsm_state,
                                vsm_proof,
                                attack_resolved_from_gm(Established::assert()),
                            );
                            (vsm_state, vsm_proof) =
                                resolve_defense(vsm_state, vsm_proof, Established::assert());
                        }

                        CombatExchangeResult::Hit { damage, .. } => {
                            let injury = damage.injury;
                            info!(attacker = %actor_state.id, defender = %target_state.id, injury, "attack hit");
                            self.narrate(format!(
                                "{} hits {} for {} injury!",
                                actor_state.id, target_state.id, injury
                            ));
                            (vsm_state, vsm_proof) = resolve_attack(
                                vsm_state,
                                vsm_proof,
                                attack_resolved_from_gm(Established::assert()),
                            );
                            (vsm_state, vsm_proof) =
                                resolve_defense(vsm_state, vsm_proof, Established::assert());
                            (vsm_state, vsm_proof) = apply_damage(
                                vsm_state,
                                vsm_proof,
                                target_slot,
                                injury,
                                damage_applied_from_gm(Established::assert()),
                            );
                        }
                    }
                }

                ManeuverChoice::AllOutDefense => {
                    self.narrate(format!("{} goes all-out defensive.", actor_state.id));
                }
                ManeuverChoice::Move => {
                    self.narrate(format!("{} moves.", actor_state.id));
                }
                ManeuverChoice::Wait => {
                    self.narrate(format!("{} waits.", actor_state.id));
                }
            }

            (vsm_state, vsm_proof) = end_turn(vsm_state, vsm_proof, Established::assert());
            push_phase(&self.session, CombatPhase::Active(vsm_state.clone())).await;

            if let Some(victor) = winning_team(&vsm_state) {
                conclude_combat(
                    vsm_state,
                    vsm_proof,
                    Some(victor.clone()),
                    Established::assert(),
                );
                push_phase(
                    &self.session,
                    CombatPhase::Concluded {
                        winner: Some(victor.clone()),
                    },
                )
                .await;
                self.narrate(format!("⚔ {} wins!", victor));
                return Ok(Some(victor));
            }
        }

        Ok(None)
    }

    /// Push the current combat state into `combatant_slot`'s knowledge cache,
    /// replacing any stale entry from a previous turn.
    #[instrument(skip(self, vsm_state), fields(combatant_slot))]
    fn refresh_knowledge(&self, combatant_slot: usize, vsm_state: &CombatState) {
        let phase = CombatPhase::Active(vsm_state.clone());
        let viewer_name = &self.combatants[combatant_slot].player.character.name;
        let view = CombatStateView::from_phase(&phase, viewer_name);
        let mut cache = self.combatants[combatant_slot].knowledge.lock().unwrap();
        cache.clear();
        cache.push(view.to_preamble());
    }

    #[instrument(skip(self, text))]
    fn narrate(&self, text: impl Into<String>) {
        let text = text.into();
        info!(narration = %text, "GM narration");
        self.chat_tx
            .send(ChatMessage::new(ChatSender::GameMaster, text))
            .ok();
    }
}

// ── Helper functions ──────────────────────────────────────────────────────────

/// Write a new phase into the session for observer reads.
#[instrument(skip(session, phase))]
async fn push_phase(session: &CombatSession, phase: CombatPhase) {
    *session.lock().await = phase;
}

#[instrument(skip(ch), fields(name = %ch.name, team))]
fn build_combatant_state(ch: &CharacterDescriptor, team: &str) -> CombatantState {
    let hp = ch.derived_stats.hp;
    CombatantState {
        id: ch.name.clone(),
        team: team.to_string(),
        current_hp: hp,
        max_hp: hp,
        current_fp: hp,
        max_fp: hp,
        basic_speed: (ch.derived_stats.basic_speed * 100.0) as i32,
        incapacitated: false,
    }
}

/// Returns the index of the first non-incapacitated combatant on an opposing team.
#[instrument(skip(state), fields(actor_slot))]
fn find_enemy_target(state: &CombatState, actor_slot: usize) -> Option<usize> {
    let CombatState::Active { ref combatants, .. } = state else {
        return None;
    };
    let actor_team = &combatants[actor_slot].team;
    combatants
        .iter()
        .enumerate()
        .find(|(_, c)| &c.team != actor_team && !c.incapacitated)
        .map(|(i, _)| i)
}

/// Returns the winning team name if exactly one team has living combatants.
#[instrument(skip(state))]
fn winning_team(state: &CombatState) -> Option<String> {
    let CombatState::Active { ref combatants, .. } = state else {
        return None;
    };
    let mut alive: std::collections::HashSet<&str> = std::collections::HashSet::new();
    for c in combatants {
        if !c.incapacitated {
            alive.insert(&c.team);
        }
    }
    if alive.len() == 1 {
        alive.into_iter().next().map(String::from)
    } else {
        None
    }
}

/// Build the [`ChoiceSet`] of defenses available to a character.
fn available_defenses(ch: &CharacterDescriptor) -> ChoiceSet<DefenseChoice> {
    ChoiceSet::new(vec![
        DefenseChoice::Dodge,
        DefenseChoice::Parry,
        DefenseChoice::Block,
        DefenseChoice::None,
    ])
    .with_prompt(format!("Choose a defense, {}:", ch.name))
}

/// Map a [`DefenseChoice`] to a [`crate::contracts::types::DefenseDescriptor`].
#[instrument(skip(ch), fields(choice = ?choice))]
fn build_defense_descriptor(
    choice: DefenseChoice,
    ch: &CharacterDescriptor,
) -> crate::contracts::types::DefenseDescriptor {
    let (defense_type, score) = match choice {
        DefenseChoice::Dodge => (DefenseType::Dodge, ch.derived_stats.dodge),
        DefenseChoice::Parry => (DefenseType::Parry, parry_score(ch)),
        DefenseChoice::Block => (DefenseType::Block, block_score()),
        DefenseChoice::None => (DefenseType::Dodge, 0),
    };
    DefenseDescriptorBuilder::default()
        .defense_type(defense_type)
        .defense_score(score)
        .build()
        .expect("valid defense descriptor")
}

/// Effective melee attack skill: first DX- or ST-based skill found, else 10.
#[instrument(skip(ch), fields(name = %ch.name))]
fn melee_skill(ch: &CharacterDescriptor) -> i32 {
    ch.skills
        .iter()
        .find(|s| matches!(s.base_attribute, AttributeType::DX | AttributeType::ST))
        .map(|s| s.level)
        .unwrap_or(10)
}

/// Parry score: weapon_skill / 2 + 3.
#[instrument(skip(ch), fields(name = %ch.name))]
fn parry_score(ch: &CharacterDescriptor) -> i32 {
    melee_skill(ch) / 2 + 3
}

/// Block score: 7 (shield_skill / 2 + 3, defaulting to shield skill 8).
fn block_score() -> i32 {
    7
}
