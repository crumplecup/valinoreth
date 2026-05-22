//! Spell learning and management implementation.

use crate::contracts::credentials::{MageryMet, PrerequisitesMet, SpellAcquired};
use crate::contracts::proof_composition::SpellLearningEvidence;
use crate::contracts::traits::{CombatResult, ContractError, ContractErrorKind, SpellManager};
use crate::contracts::types::{CasterDescriptor, SpellDescriptor};
use crate::game_master::GameMaster;
use async_trait::async_trait;
use elicitation::contracts::Established;

#[async_trait]
impl SpellManager for GameMaster {
    async fn learn_spell(
        &self,
        mut caster: CasterDescriptor,
        spell: SpellDescriptor,
    ) -> CombatResult<(CasterDescriptor, Established<SpellLearningEvidence>)> {
        // Verify prerequisites
        if !self.can_learn_spell(&caster, &spell).await? {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                format!("Cannot learn spell {}: prerequisites not met", spell.name),
            )));
        }

        // Check if spell already known
        if caster
            .known_spells
            .iter()
            .any(|s| s.name == spell.name)
        {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                format!("Spell {} already known", spell.name),
            )));
        }

        // Add spell to known spells
        caster.known_spells.push(spell);

        // Mint proof tokens
        let prerequisites = Established::prove(&PrerequisitesMet);
        let magery = Established::prove(&MageryMet);
        let learned = Established::prove(&SpellAcquired);

        let _evidence = SpellLearningEvidence {
            prerequisites,
            magery,
            learned,
        };

        Ok((caster, Established::assert()))
    }

    async fn improve_spell(
        &self,
        mut caster: CasterDescriptor,
        spell_name: String,
        points: i32,
    ) -> CombatResult<CasterDescriptor> {
        // Verify points are positive
        if points <= 0 {
            return Err(ContractError::new(ContractErrorKind::StateViolation(
                "Points must be positive".to_string(),
            )));
        }

        // Find the spell
        let spell = caster
            .known_spells
            .iter_mut()
            .find(|s| s.name == spell_name)
            .ok_or_else(|| {
                ContractError::new(ContractErrorKind::StateViolation(format!(
                    "Spell {} not known",
                    spell_name
                )))
            })?;

        // Add points
        spell.points_invested += points;

        // Recalculate skill level
        // Spells are Mental/Hard, so IQ - 2 + relative level
        let relative_level = self.points_to_relative_level(spell.points_invested);
        spell.skill_level = caster.iq - 2 + relative_level + caster.magery_level;

        Ok(caster)
    }

    async fn can_learn_spell(
        &self,
        caster: &CasterDescriptor,
        spell: &SpellDescriptor,
    ) -> CombatResult<bool> {
        // Check Magery requirement
        if caster.magery_level < spell.magery_required {
            return Ok(false);
        }

        // Check prerequisites
        for prereq in &spell.prerequisites {
            let has_prereq = caster
                .known_spells
                .iter()
                .any(|s| &s.name == prereq);

            if !has_prereq {
                return Ok(false);
            }
        }

        Ok(true)
    }
}

impl GameMaster {
    /// Calculate skill level from points spent.
    ///
    /// # GURPS Rules
    ///
    /// Point progression: 1pt = +0, 2pt = +1, 4pt = +2, 8pt = +3, etc.
    /// Each doubling adds one relative skill level.
    ///
    /// # Citations
    ///
    /// BS 170 - Skill point costs
    fn points_to_relative_level(&self, points: i32) -> i32 {
        if points < 1 {
            return 0;
        }
        // 1pt = 0, 2pt = 1, 4pt = 2, 8pt = 3, 16pt = 4, etc.
        // This is essentially log2(points)
        let mut level = 0;
        let mut threshold = 1;
        while threshold * 2 <= points {
            level += 1;
            threshold *= 2;
        }
        level
    }
}
